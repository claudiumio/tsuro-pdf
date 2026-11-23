//! Digital signature inspection and CMS verification for PDF files.
//!
//! Recognition walks the PDF object graph (AcroForm fields and signature
//! dictionaries). Cryptographic checks decode the PKCS#7/CMS blob, compare the
//! message-digest signed attribute with the ByteRange payload, and verify the
//! signer’s RSA signature over the signed attributes.
//!
//! A matching digest and RSA check only prove integrity of the covered bytes.
//! Without a trust anchor this crate never reports [`SignatureStatus::Valid`].

mod cms;

use base64::{engine::general_purpose::STANDARD as B64, Engine};
use lopdf::{Dictionary, Document, Object};
use rsa::pkcs1v15::{Signature as RsaPkcs1Signature, VerifyingKey};
use rsa::signature::Verifier as _;
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::{Digest, Sha256};

use cms::parse_signed_data;

#[derive(Debug, thiserror::Error)]
pub enum SigError {
    #[error("não foi possível ler o PDF: {0}")]
    Pdf(String),
    #[error("CMS inválido: {0}")]
    Cms(String),
    #[error("algoritmo não suportado: {0}")]
    Unsupported(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SignatureStatus {
    Valid,
    IntactButUntrusted,
    DocumentModified,
    Invalid,
    Unsupported,
    CertificateExpired,
    CertificateNotYetValid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateInfo {
    pub subject: String,
    pub common_name: Option<String>,
    pub organization: Option<String>,
    pub issuer: String,
    pub serial: String,
    pub not_before: String,
    pub not_after: String,
    pub is_self_signed: bool,
}

/// Outcome of comparing the CMS `messageDigest` with the signed bytes.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DigestCheck {
    Matched,
    Mismatched,
    NotComputed,
}

/// Outcome of the RSA signature check, kept separate from parse and key errors.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum RsaCheck {
    Passed,
    Failed,
    KeyUnavailable,
    UnsupportedDigest,
    NotRun,
}

/// Outcome of the certificate validity check against the system clock.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ClockCheck {
    WithinValidity,
    Expired,
    NotYetValid,
    NotRun,
}

/// Trust-chain check. No anchor exists in this app, so it never runs.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ChainCheck {
    NotRun,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureInfo {
    pub field_name: Option<String>,
    pub signer_name: Option<String>,
    pub reason: Option<String>,
    pub location: Option<String>,
    pub contact_info: Option<String>,
    pub signing_time: Option<String>,
    pub filter: Option<String>,
    pub sub_filter: Option<String>,
    pub byte_range: Option<[i64; 4]>,
    pub covers_whole_document: bool,
    pub status: SignatureStatus,
    /// Language-neutral code describing how the status was reached.
    pub status_detail: String,
    pub certificate: Option<CertificateInfo>,
    pub digest_algorithm: Option<String>,
    pub signature_algorithm: Option<String>,
    pub digest: DigestCheck,
    /// `Some(true)` when the file has bytes outside a usable signed range,
    /// `None` when the range is missing or malformed and nothing was compared.
    pub bytes_after_signed_range: Option<bool>,
    pub rsa: RsaCheck,
    /// Whether the certificate used for verification is the one named by the
    /// CMS SignerIdentifier.
    pub signer_cert_matches_sid: Option<bool>,
    pub clock: ClockCheck,
    pub chain: ChainCheck,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfAnalysis {
    pub page_count_hint: Option<u32>,
    pub has_acro_form: bool,
    pub signatures: Vec<SignatureInfo>,
}

pub fn analyze_pdf(bytes: &[u8]) -> Result<PdfAnalysis, SigError> {
    let document = Document::load_mem(bytes).map_err(|e| SigError::Pdf(e.to_string()))?;
    let mut signatures = Vec::new();
    let mut visited: Vec<Vec<u8>> = Vec::new();

    let has_acro_form = document
        .catalog()
        .ok()
        .and_then(|c| c.get(b"AcroForm").ok())
        .is_some();

    for object in document.objects.values() {
        let Object::Dictionary(dict) = object else {
            continue;
        };
        if let Some(sig) = signature_from_field(&document, dict, bytes, &mut visited) {
            signatures.push(sig);
        } else if looks_like_sig_dict(dict) {
            if let Some(sig) = signature_from_dict(&document, dict, None, bytes, &mut visited) {
                signatures.push(sig);
            }
        }
    }

    let page_count_hint = Some(document.get_pages().len() as u32);

    Ok(PdfAnalysis {
        page_count_hint,
        has_acro_form,
        signatures,
    })
}

pub fn verify_cms_b64(pkcs7_b64: &str, sha256_hex: &str) -> Result<SignatureInfo, SigError> {
    let pkcs7 = B64
        .decode(pkcs7_b64)
        .map_err(|e| SigError::Cms(e.to_string()))?;
    let digest = hex_decode(sha256_hex).map_err(SigError::Cms)?;
    verify_cms(&pkcs7, Some(digest.as_slice()), None)
}

pub fn verify_cms(
    pkcs7: &[u8],
    sha256: Option<&[u8]>,
    sha1: Option<&[u8]>,
) -> Result<SignatureInfo, SigError> {
    let parsed = parse_signed_data(pkcs7)?;
    let certificate = parsed.certificate.clone();
    let mut info = SignatureInfo {
        field_name: None,
        signer_name: certificate.as_ref().and_then(|c| c.common_name.clone()),
        reason: None,
        location: None,
        contact_info: None,
        signing_time: None,
        filter: None,
        sub_filter: Some("adbe.pkcs7.detached".into()),
        byte_range: None,
        covers_whole_document: true,
        status: SignatureStatus::Unsupported,
        status_detail: String::new(),
        certificate: certificate.clone(),
        digest_algorithm: Some(parsed.digest_algorithm.to_string()),
        signature_algorithm: Some(parsed.signature_algorithm.to_string()),
        digest: DigestCheck::NotComputed,
        bytes_after_signed_range: None,
        rsa: RsaCheck::NotRun,
        signer_cert_matches_sid: parsed.signer_cert_matches_sid,
        clock: ClockCheck::NotRun,
        chain: ChainCheck::NotRun,
    };

    // A digest comparison needs signed attributes, the messageDigest attribute
    // inside them, and the payload being verified. Each missing input is reported
    // on its own instead of being folded into "document modified".
    let Some(signed_attrs) = parsed.signed_attrs_for_verify.as_ref() else {
        info.status_detail = "signed_attributes_missing".into();
        return Ok(info);
    };
    let Some(expected_digest) = parsed.message_digest.as_ref() else {
        info.status_detail = "message_digest_missing".into();
        return Ok(info);
    };
    let payload = match parsed.digest_algorithm {
        "SHA-256" => sha256,
        "SHA-1" => sha1,
        _ => {
            info.status_detail = "digest_algorithm_unsupported".into();
            return Ok(info);
        }
    };
    let Some(payload) = payload else {
        info.status_detail = "digest_payload_unavailable".into();
        return Ok(info);
    };

    if expected_digest.as_slice() != payload {
        info.digest = DigestCheck::Mismatched;
        info.status = SignatureStatus::DocumentModified;
        info.status_detail = "digest_mismatch".into();
        return Ok(info);
    }
    info.digest = DigestCheck::Matched;

    if parsed.digest_algorithm == "SHA-1" {
        info.status = SignatureStatus::Unsupported;
        info.status_detail = "sha1_unsupported".into();
        info.rsa = RsaCheck::UnsupportedDigest;
        return Ok(info);
    }

    let Some(public) = parsed.public_key else {
        info.status = SignatureStatus::Invalid;
        info.rsa = RsaCheck::KeyUnavailable;
        info.status_detail = if parsed.signer_cert_matches_sid == Some(false) {
            "signer_certificate_not_found"
        } else {
            "rsa_key_unavailable"
        }
        .into();
        return Ok(info);
    };

    let signature = RsaPkcs1Signature::try_from(parsed.signature.as_slice())
        .map_err(|e| SigError::Cms(e.to_string()))?;
    let crypto_ok = match parsed.digest_algorithm {
        "SHA-256" => VerifyingKey::<Sha256>::new(public)
            .verify(signed_attrs, &signature)
            .is_ok(),
        _ => {
            info.status = SignatureStatus::Unsupported;
            info.status_detail = "digest_algorithm_unsupported".into();
            info.rsa = RsaCheck::UnsupportedDigest;
            return Ok(info);
        }
    };

    if !crypto_ok {
        info.status = SignatureStatus::Invalid;
        info.status_detail = "signature_mismatch".into();
        info.rsa = RsaCheck::Failed;
        return Ok(info);
    }
    info.rsa = RsaCheck::Passed;

    if let (Some(not_before), Some(not_after)) = (parsed.cert_not_before, parsed.cert_not_after) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        if now < not_before {
            info.status = SignatureStatus::CertificateNotYetValid;
            info.status_detail = "certificate_not_yet_valid".into();
            info.clock = ClockCheck::NotYetValid;
            return Ok(info);
        }
        if now > not_after {
            info.status = SignatureStatus::CertificateExpired;
            info.status_detail = "certificate_expired".into();
            info.clock = ClockCheck::Expired;
            return Ok(info);
        }
        info.clock = ClockCheck::WithinValidity;
    }

    info.status = SignatureStatus::IntactButUntrusted;
    if certificate
        .as_ref()
        .map(|c| c.is_self_signed)
        .unwrap_or(true)
    {
        info.status_detail = "intact_self_signed".into();
    } else {
        info.status_detail = "intact_chain_unverified".into();
    }

    Ok(info)
}

fn signature_from_field(
    document: &Document,
    field: &Dictionary,
    bytes: &[u8],
    visited: &mut Vec<Vec<u8>>,
) -> Option<SignatureInfo> {
    let ft = name_of(field.get(b"FT").ok()?)?;
    if ft != "Sig" {
        return None;
    }
    let field_name = string_of(field.get(b"T").ok());
    let value = field.get(b"V").ok()?;
    let dict = deref_dict(document, value)?;
    signature_from_dict(document, &dict, field_name, bytes, visited)
}

fn signature_from_dict(
    _document: &Document,
    dict: &Dictionary,
    field_name: Option<String>,
    bytes: &[u8],
    visited: &mut Vec<Vec<u8>>,
) -> Option<SignatureInfo> {
    let contents = contents_bytes(dict)?;
    if visited.iter().any(|c| c == &contents) {
        return None;
    }
    visited.push(contents.clone());

    let declared_range = read_byte_range(dict);
    let byte_range = declared_range
        .as_ref()
        .ok()
        .copied()
        .filter(|br| byte_range_slices(bytes, *br).is_some());
    let byte_range_issue = match (&declared_range, byte_range) {
        (Err(issue), _) => Some(*issue),
        (Ok(_), None) => Some(ByteRangeIssue::Malformed),
        (Ok(_), Some(_)) => None,
    };
    let covers = byte_range
        .map(|br| covers_whole_document(bytes, br, &contents))
        .unwrap_or(false);

    let sha256 = byte_range.and_then(|br| hash_byte_range::<Sha256>(bytes, br));
    let sha1 = byte_range.and_then(|br| hash_byte_range::<Sha1>(bytes, br));

    let mut cms_parsed = true;
    let mut info = verify_cms(&contents, sha256.as_deref(), sha1.as_deref()).unwrap_or_else(|_| {
        cms_parsed = false;
        SignatureInfo {
            field_name: None,
            signer_name: string_of(dict.get(b"Name").ok()),
            reason: string_of(dict.get(b"Reason").ok()),
            location: string_of(dict.get(b"Location").ok()),
            contact_info: string_of(dict.get(b"ContactInfo").ok()),
            signing_time: string_of(dict.get(b"M").ok()),
            filter: name_of(dict.get(b"Filter").ok().unwrap_or(&Object::Null)),
            sub_filter: name_of(dict.get(b"SubFilter").ok().unwrap_or(&Object::Null)),
            byte_range,
            covers_whole_document: covers,
            status: SignatureStatus::Invalid,
            status_detail: "cms_parse_error".into(),
            certificate: None,
            digest_algorithm: None,
            signature_algorithm: None,
            digest: DigestCheck::NotComputed,
            bytes_after_signed_range: None,
            rsa: RsaCheck::NotRun,
            signer_cert_matches_sid: None,
            clock: ClockCheck::NotRun,
            chain: ChainCheck::NotRun,
        }
    });

    if info.field_name.is_none() {
        info.field_name = field_name;
    }
    if info.signer_name.is_none() {
        info.signer_name = string_of(dict.get(b"Name").ok());
    }
    if info.reason.is_none() {
        info.reason = string_of(dict.get(b"Reason").ok());
    }
    if info.location.is_none() {
        info.location = string_of(dict.get(b"Location").ok());
    }
    if info.contact_info.is_none() {
        info.contact_info = string_of(dict.get(b"ContactInfo").ok());
    }
    if info.signing_time.is_none() {
        info.signing_time = string_of(dict.get(b"M").ok()).map(|m| format_pdf_date(&m));
    } else if let Some(raw) = info.signing_time.clone() {
        info.signing_time = Some(format_pdf_date(&raw));
    }
    info.filter = name_of(dict.get(b"Filter").ok().unwrap_or(&Object::Null)).or(info.filter);
    info.sub_filter =
        name_of(dict.get(b"SubFilter").ok().unwrap_or(&Object::Null)).or(info.sub_filter);
    info.byte_range = byte_range.or(info.byte_range);
    info.covers_whole_document = covers;
    info.bytes_after_signed_range = byte_range.map(|_| !covers);

    if cms_parsed {
        if let Some(issue) = byte_range_issue {
            // Nothing was hashed, so the only honest report is the broken range.
            info.digest = DigestCheck::NotComputed;
            info.bytes_after_signed_range = None;
            if info.status_detail == "digest_payload_unavailable" {
                info.status = SignatureStatus::Unsupported;
                info.status_detail = issue.detail().into();
                info.rsa = RsaCheck::NotRun;
                info.clock = ClockCheck::NotRun;
            }
        } else if !covers
            && matches!(
                info.status,
                SignatureStatus::IntactButUntrusted
                    | SignatureStatus::CertificateExpired
                    | SignatureStatus::CertificateNotYetValid
            )
        {
            // The signature verified, but the file carries bytes outside the range
            // it signed: an incremental update happened after signing.
            info.status = SignatureStatus::DocumentModified;
            info.status_detail = "bytes_after_signed_range".into();
        }
    }

    Some(info)
}

fn looks_like_sig_dict(dict: &Dictionary) -> bool {
    match dict.get(b"Type").ok().and_then(name_of) {
        Some(name) if name == "Sig" => true,
        _ => dict.get(b"ByteRange").is_ok() && dict.get(b"Contents").is_ok(),
    }
}

fn deref_dict(document: &Document, object: &Object) -> Option<Dictionary> {
    match object {
        Object::Dictionary(dict) => Some(dict.clone()),
        Object::Reference(id) => match document.get_object(*id).ok()? {
            Object::Dictionary(dict) => Some(dict.clone()),
            _ => None,
        },
        _ => None,
    }
}

fn name_of(object: &Object) -> Option<String> {
    match object {
        Object::Name(n) => Some(String::from_utf8_lossy(n).into_owned()),
        Object::Reference(_) => None,
        _ => None,
    }
}

fn string_of(object: Option<&Object>) -> Option<String> {
    match object? {
        Object::String(s, _) => Some(String::from_utf8_lossy(s).into_owned()),
        Object::Name(n) => Some(String::from_utf8_lossy(n).into_owned()),
        _ => None,
    }
}

fn contents_bytes(dict: &Dictionary) -> Option<Vec<u8>> {
    match dict.get(b"Contents").ok()? {
        Object::String(s, _) => Some(s.clone()),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ByteRangeIssue {
    Missing,
    Malformed,
}

impl ByteRangeIssue {
    fn detail(self) -> &'static str {
        match self {
            Self::Missing => "byte_range_missing",
            Self::Malformed => "byte_range_malformed",
        }
    }
}

/// Reads `/ByteRange` as four non-negative integers. `Missing` means the key is
/// absent; `Malformed` means it exists but cannot be used as a byte range.
fn read_byte_range(dict: &Dictionary) -> Result<[i64; 4], ByteRangeIssue> {
    let Ok(object) = dict.get(b"ByteRange") else {
        return Err(ByteRangeIssue::Missing);
    };
    let Object::Array(items) = object else {
        return Err(ByteRangeIssue::Malformed);
    };
    if items.len() != 4 {
        return Err(ByteRangeIssue::Malformed);
    }
    let mut out = [0i64; 4];
    for (i, item) in items.iter().enumerate() {
        out[i] = match item {
            Object::Integer(n) if *n >= 0 => *n,
            Object::Real(n) if n.is_finite() && *n >= 0.0 => *n as i64,
            _ => return Err(ByteRangeIssue::Malformed),
        };
    }
    Ok(out)
}

fn is_pdf_ws(b: u8) -> bool {
    matches!(b, b' ' | b'\n' | b'\r' | b'\t' | 0 | b'\x0c')
}

fn byte_range_slices(bytes: &[u8], br: [i64; 4]) -> Option<(&[u8], &[u8])> {
    let [a, b, c, d] = br;
    let a = usize::try_from(a).ok()?;
    let b = usize::try_from(b).ok()?;
    let c = usize::try_from(c).ok()?;
    let d = usize::try_from(d).ok()?;
    let first_end = a.checked_add(b)?;
    let second_end = c.checked_add(d)?;
    if first_end > bytes.len() || second_end > bytes.len() || first_end > c {
        return None;
    }
    Some((&bytes[a..first_end], &bytes[c..second_end]))
}

fn covers_whole_document(bytes: &[u8], br: [i64; 4], contents: &[u8]) -> bool {
    let [a, b, c, d] = br;
    if a != 0 {
        return false;
    }
    if byte_range_slices(bytes, br).is_none() {
        return false;
    }
    let Ok(a) = usize::try_from(a) else {
        return false;
    };
    let Ok(b) = usize::try_from(b) else {
        return false;
    };
    let Ok(c) = usize::try_from(c) else {
        return false;
    };
    let Ok(d) = usize::try_from(d) else {
        return false;
    };
    let Some(gap_start) = a.checked_add(b) else {
        return false;
    };
    let Some(tail_start) = c.checked_add(d) else {
        return false;
    };
    if tail_start > bytes.len() || gap_start > c {
        return false;
    }
    if !bytes[tail_start..].iter().copied().all(is_pdf_ws) {
        return false;
    }
    gap_is_contents(&bytes[gap_start..c], contents)
}

fn gap_is_contents(gap: &[u8], contents: &[u8]) -> bool {
    let start = gap.iter().position(|b| !is_pdf_ws(*b)).unwrap_or(gap.len());
    let end = gap
        .iter()
        .rposition(|b| !is_pdf_ws(*b))
        .map(|i| i + 1)
        .unwrap_or(0);
    if start >= end {
        return false;
    }
    let core = &gap[start..end];
    if core.len() < 2 || core[0] != b'<' || core[core.len() - 1] != b'>' {
        return false;
    }
    let inner = &core[1..core.len() - 1];
    if !inner
        .iter()
        .copied()
        .all(|b| b.is_ascii_hexdigit() || is_pdf_ws(b))
    {
        return false;
    }
    let hex: Vec<u8> = inner
        .iter()
        .copied()
        .filter(u8::is_ascii_hexdigit)
        .collect();
    if !hex.len().is_multiple_of(2) {
        return false;
    }
    let Ok(hex_str) = std::str::from_utf8(&hex) else {
        return false;
    };
    let Ok(decoded) = hex_decode(hex_str) else {
        return false;
    };
    padded_bytes_eq(&decoded, contents)
}

fn padded_bytes_eq(a: &[u8], b: &[u8]) -> bool {
    let (short, long) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    long.starts_with(short) && long[short.len()..].iter().all(|b| *b == 0)
}

fn hash_byte_range<D: Digest>(bytes: &[u8], br: [i64; 4]) -> Option<Vec<u8>> {
    let (first, second) = byte_range_slices(bytes, br)?;
    let mut hasher = D::new();
    hasher.update(first);
    hasher.update(second);
    Some(hasher.finalize().to_vec())
}

fn format_pdf_date(raw: &str) -> String {
    let s = raw.strip_prefix("D:").unwrap_or(raw);
    // A data é sempre ASCII (D:YYYYMMDDHHmmSS…). Fatias por índice de byte em
    // uma string multibyte panicam; fora do formato, devolvemos a string crua.
    let bytes = s.as_bytes();
    if bytes.len() < 14 || !bytes[..14].iter().all(u8::is_ascii_digit) {
        return raw.to_string();
    }
    let digits =
        |range: std::ops::Range<usize>| std::str::from_utf8(&bytes[range]).unwrap_or_default();
    format!(
        "{}-{}-{} {}:{}:{}",
        digits(0..4),
        digits(4..6),
        digits(6..8),
        digits(8..10),
        digits(10..12),
        digits(12..14)
    )
}

fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err("hex inválido".into());
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name)
    }

    #[test]
    fn unsigned_guide_has_no_signatures() {
        let bytes = std::fs::read(fixture("guia-folio.pdf")).expect("guia");
        let analysis = analyze_pdf(&bytes).expect("parse");
        assert!(analysis.signatures.is_empty());
    }

    #[test]
    fn multibyte_m_date_does_not_panic() {
        let mut file = Vec::new();
        file.extend_from_slice(b"%PDF-1.4\n");
        let obj1 = file.len();
        file.extend_from_slice(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n");
        let obj2 = file.len();
        file.extend_from_slice(b"2 0 obj\n<< /Type /Pages /Kids [] /Count 0 >>\nendobj\n");
        let obj3 = file.len();
        // "é" (UTF-8, 2 bytes) atravessa o índice de byte 4 da data.
        file.extend_from_slice(
            b"3 0 obj\n<< /Type /Sig /Contents <41> /M (D:abc\xc3\xa9abcdefghij) >>\nendobj\n",
        );
        let xref_at = file.len();
        let xref = format!(
            "xref\n0 4\n0000000000 65535 f \n{obj1:010} 00000 n \n{obj2:010} 00000 n \n{obj3:010} 00000 n \ntrailer\n<< /Size 4 /Root 1 0 R >>\nstartxref\n{xref_at:010}\n%%EOF\n"
        );
        file.extend_from_slice(xref.as_bytes());

        let analysis = analyze_pdf(&file).expect("pdf parses");
        assert_eq!(analysis.signatures.len(), 1, "{analysis:?}");
        let sig = &analysis.signatures[0];
        // Data fora do formato degrada para a string crua, sem pânico (#201).
        assert_eq!(
            sig.signing_time.as_deref(),
            Some("D:abc\u{e9}abcdefghij"),
            "{:?}",
            sig.signing_time
        );
    }

    #[test]
    fn signed_contract_is_recognized() {
        let bytes = std::fs::read(fixture("contrato-assinado.pdf")).expect("contrato");
        let analysis = analyze_pdf(&bytes).expect("parse");
        assert_eq!(analysis.signatures.len(), 1, "{analysis:?}");
        let sig = &analysis.signatures[0];
        assert!(sig.covers_whole_document, "{}", sig.status_detail);
        assert_ne!(sig.status, SignatureStatus::Valid, "{}", sig.status_detail);
        assert!(
            matches!(
                sig.status,
                SignatureStatus::IntactButUntrusted
                    | SignatureStatus::CertificateExpired
                    | SignatureStatus::CertificateNotYetValid
            ),
            "{:?} {}",
            sig.status,
            sig.status_detail
        );
        let cn = sig
            .certificate
            .as_ref()
            .and_then(|c| c.common_name.as_deref())
            .or(sig.signer_name.as_deref())
            .unwrap_or("");
        assert!(cn.contains("Maria"), "{cn}");
        // The real fixture's SignerIdentifier must resolve to its certificate,
        // otherwise the RSA check would run against nothing (#161 defect 4).
        assert_eq!(sig.signer_cert_matches_sid, Some(true), "{sig:?}");
        assert_eq!(sig.rsa, RsaCheck::Passed, "{sig:?}");
        assert_eq!(sig.digest, DigestCheck::Matched, "{sig:?}");
    }

    #[test]
    fn tampering_after_signature_is_detected() {
        let mut bytes = std::fs::read(fixture("contrato-assinado.pdf")).expect("contrato");
        let last = bytes.len() - 1;
        bytes[last] ^= 0x7f;
        if bytes[last] == 0 || bytes[last].is_ascii_whitespace() {
            bytes[last] = b'X';
        }
        // Flip a byte inside the first range if the tail is ignored whitespace.
        if bytes.len() > 80 {
            bytes[40] ^= 0x11;
        }
        let analysis = analyze_pdf(&bytes).expect("parse");
        assert_eq!(analysis.signatures.len(), 1);
        assert!(
            matches!(
                analysis.signatures[0].status,
                SignatureStatus::DocumentModified | SignatureStatus::Invalid
            ),
            "{:?}",
            analysis.signatures[0]
        );
    }

    #[test]
    fn negative_byte_range_does_not_panic() {
        let bytes = vec![0u8; 32];
        assert!(!covers_whole_document(&bytes, [0, 10, -1, 1], b""));
        assert!(hash_byte_range::<Sha256>(&bytes, [0, 10, -1, 1]).is_none());
        assert!(hash_byte_range::<Sha256>(&bytes, [0, 4, 8, 4]).is_some());
    }

    #[test]
    fn gap_must_be_the_contents_hex_string() {
        let contents = b"\x01\x02";
        let mut bytes = b"AAAA".to_vec();
        bytes.extend_from_slice(b"<0102>");
        bytes.extend_from_slice(b"BBBB");
        let br = [0, 4, 10, 4];
        assert!(covers_whole_document(&bytes, br, contents));

        let mut dirty = b"AAAA".to_vec();
        dirty.extend_from_slice(b"<0102>xref>");
        dirty.extend_from_slice(b"BBBB");
        let dirty_br = [0, 4, 15, 4];
        assert!(!covers_whole_document(&dirty, dirty_br, contents));
    }

    fn pdf_with_nested_catalog_array(depth: usize) -> Vec<u8> {
        let mut array = Vec::with_capacity(depth * 2);
        array.extend(std::iter::repeat_n(b'[', depth));
        array.extend(std::iter::repeat_n(b']', depth));

        let mut body = Vec::new();
        body.extend_from_slice(b"%PDF-1.4\n");
        let obj1 = body.len();
        body.extend_from_slice(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R /X ");
        body.extend(array);
        body.extend_from_slice(b" >>\nendobj\n");
        let obj2 = body.len();
        body.extend_from_slice(b"2 0 obj\n<< /Type /Pages /Kids [] /Count 0 >>\nendobj\n");
        let xref_at = body.len();
        let xref = format!(
            "xref\n0 3\n0000000000 65535 f \n{obj1:010} 00000 n \n{obj2:010} 00000 n \ntrailer\n<< /Size 3 /Root 1 0 R >>\nstartxref\n{xref_at}\n%%EOF\n"
        );
        body.extend_from_slice(xref.as_bytes());
        body
    }

    #[test]
    fn deeply_nested_catalog_is_an_error() {
        let shallow = pdf_with_nested_catalog_array(2);
        let ok = analyze_pdf(&shallow).expect("a two-level catalog array is a normal PDF");
        assert!(ok.signatures.is_empty());

        let nested = pdf_with_nested_catalog_array(10_380);
        let analysis = analyze_pdf(&nested).expect("a 10380-deep catalog array returns");
        assert!(analysis.signatures.is_empty());
        assert_eq!(analysis.page_count_hint, Some(0));
    }

    fn der_tlv(tag: u8, value: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(value.len() + 4);
        out.push(tag);
        let len = value.len();
        if len < 0x80 {
            out.push(len as u8);
        } else if len <= 0xff {
            out.push(0x81);
            out.push(len as u8);
        } else {
            out.push(0x82);
            out.push((len >> 8) as u8);
            out.push(len as u8);
        }
        out.extend_from_slice(value);
        out
    }

    fn der_cat(parts: &[&[u8]]) -> Vec<u8> {
        let mut out = Vec::new();
        for part in parts {
            out.extend_from_slice(part);
        }
        out
    }

    fn cms_sha256_without_certificate(digest: &[u8]) -> Vec<u8> {
        let sha256 = der_tlv(
            0x30,
            &der_cat(&[
                &der_tlv(
                    0x06,
                    &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01],
                ),
                &der_tlv(0x05, &[]),
            ]),
        );
        let rsa = der_tlv(
            0x30,
            &der_cat(&[
                &der_tlv(
                    0x06,
                    &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01],
                ),
                &der_tlv(0x05, &[]),
            ]),
        );
        let attr = der_tlv(
            0x30,
            &der_cat(&[
                &der_tlv(
                    0x06,
                    &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x09, 0x04],
                ),
                &der_tlv(0x31, &der_tlv(0x04, digest)),
            ]),
        );
        let sid = der_tlv(
            0x30,
            &der_cat(&[
                &der_tlv(
                    0x30,
                    &der_tlv(
                        0x31,
                        &der_tlv(
                            0x30,
                            &der_cat(&[&der_tlv(0x06, &[0x55, 0x04, 0x06]), &der_tlv(0x13, b"BR")]),
                        ),
                    ),
                ),
                &der_tlv(0x02, &[1]),
            ]),
        );
        let signer = der_tlv(
            0x30,
            &der_cat(&[
                &der_tlv(0x02, &[1]),
                &sid,
                &sha256,
                &der_tlv(0xa0, &attr),
                &rsa,
                &der_tlv(0x04, &[0x00]),
            ]),
        );
        let signed_data = der_tlv(
            0x30,
            &der_cat(&[
                &der_tlv(0x02, &[1]),
                &der_tlv(0x31, &sha256),
                &der_tlv(
                    0x30,
                    &der_tlv(
                        0x06,
                        &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x07, 0x01],
                    ),
                ),
                &der_tlv(0x31, &signer),
            ]),
        );
        der_tlv(
            0x30,
            &der_cat(&[
                &der_tlv(
                    0x06,
                    &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x07, 0x02],
                ),
                &der_tlv(0xa0, &signed_data),
            ]),
        )
    }

    fn pdf_named_banco_with_matching_digest() -> Vec<u8> {
        let probe = cms_sha256_without_certificate(&[0u8; 32]);
        let placeholder = format!("<{}>", "0".repeat(probe.len() * 2));
        let mut file = Vec::new();
        file.extend_from_slice(b"%PDF-1.4\n");
        let obj1 = file.len();
        file.extend_from_slice(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n");
        let obj2 = file.len();
        file.extend_from_slice(b"2 0 obj\n<< /Type /Pages /Kids [] /Count 0 >>\nendobj\n");
        let obj3 = file.len();
        file.extend_from_slice(
            b"3 0 obj\n<< /Type /Sig /Filter /Adobe.PPKLite /SubFilter /adbe.pkcs7.detached /Name (Banco) /ByteRange ",
        );
        let br_at = file.len();
        file.extend_from_slice(b"[0000000000 0000000000 0000000000 0000000000]");
        file.extend_from_slice(b" /Contents ");
        let gap_at = file.len();
        file.extend_from_slice(placeholder.as_bytes());
        let after_gap = file.len();
        file.extend_from_slice(b" >>\nendobj\n");
        let xref_at = file.len();
        let xref = format!(
            "xref\n0 4\n0000000000 65535 f \n{obj1:010} 00000 n \n{obj2:010} 00000 n \n{obj3:010} 00000 n \ntrailer\n<< /Size 4 /Root 1 0 R >>\nstartxref\n{xref_at:010}\n%%EOF\n"
        );
        file.extend_from_slice(xref.as_bytes());

        let tail = file.len() - after_gap;
        let br = format!("[{:010} {:010} {:010} {:010}]", 0, gap_at, after_gap, tail);
        assert_eq!(br.len(), 45);
        file[br_at..br_at + br.len()].copy_from_slice(br.as_bytes());

        let mut hasher = Sha256::new();
        hasher.update(&file[..gap_at]);
        hasher.update(&file[after_gap..]);
        let digest = hasher.finalize().to_vec();
        let cms = cms_sha256_without_certificate(&digest);
        let hex: String = cms.iter().map(|b| format!("{b:02X}")).collect();
        let contents = format!("<{hex}>");
        assert_eq!(contents.len(), placeholder.len());
        file[gap_at..after_gap].copy_from_slice(contents.as_bytes());
        file
    }

    #[test]
    fn matching_digest_without_rsa_key_is_invalid() {
        let digest = [0x11u8; 32];
        let cms = cms_sha256_without_certificate(&digest);
        let info = verify_cms(&cms, Some(&digest), None).expect("cms parses");
        assert_eq!(info.status, SignatureStatus::Invalid);
        assert_eq!(info.digest, DigestCheck::Matched);
        assert_eq!(info.rsa, RsaCheck::KeyUnavailable);
        assert_eq!(info.status_detail, "rsa_key_unavailable");

        let pdf = pdf_named_banco_with_matching_digest();
        let analysis = analyze_pdf(&pdf).expect("pdf parses");
        assert_eq!(analysis.signatures.len(), 1, "{analysis:?}");
        let sig = &analysis.signatures[0];
        assert!(sig.covers_whole_document, "{}", sig.status_detail);
        assert_eq!(sig.signer_name.as_deref(), Some("Banco"));
        assert_eq!(sig.status, SignatureStatus::Invalid);
        assert_eq!(sig.rsa, RsaCheck::KeyUnavailable);
        assert_eq!(sig.status_detail, "rsa_key_unavailable");
    }

    fn cms_sha256_without_signed_attrs() -> Vec<u8> {
        // Same skeleton as cms_sha256_without_certificate, but SignerInfo has no
        // signedAttrs → message_digest stays None even when a document digest is provided.
        let sha256 = der_tlv(
            0x30,
            &der_cat(&[
                &der_tlv(
                    0x06,
                    &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01],
                ),
                &der_tlv(0x05, &[]),
            ]),
        );
        let rsa = der_tlv(
            0x30,
            &der_cat(&[
                &der_tlv(
                    0x06,
                    &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01],
                ),
                &der_tlv(0x05, &[]),
            ]),
        );
        let sid = der_tlv(
            0x30,
            &der_cat(&[
                &der_tlv(
                    0x30,
                    &der_tlv(
                        0x31,
                        &der_tlv(
                            0x30,
                            &der_cat(&[&der_tlv(0x06, &[0x55, 0x04, 0x06]), &der_tlv(0x13, b"BR")]),
                        ),
                    ),
                ),
                &der_tlv(0x02, &[1]),
            ]),
        );
        let signer = der_tlv(
            0x30,
            &der_cat(&[
                &der_tlv(0x02, &[1]),
                &sid,
                &sha256,
                // no signedAttrs here
                &rsa,
                &der_tlv(0x04, &[0x00]),
            ]),
        );
        let signed_data = der_tlv(
            0x30,
            &der_cat(&[
                &der_tlv(0x02, &[1]),
                &der_tlv(0x31, &sha256),
                &der_tlv(
                    0x30,
                    &der_tlv(
                        0x06,
                        &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x07, 0x01],
                    ),
                ),
                &der_tlv(0x31, &signer),
            ]),
        );
        der_tlv(
            0x30,
            &der_cat(&[
                &der_tlv(
                    0x06,
                    &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x07, 0x02],
                ),
                &der_tlv(0xa0, &signed_data),
            ]),
        )
    }

    /// CMS without messageDigest must not be reported as an altered document —
    /// no digest comparison happened (#161 defect 2).
    #[test]
    fn cms_without_message_digest_is_not_document_modified() {
        let digest = [0xAAu8; 32];
        let cms = cms_sha256_without_signed_attrs();
        let info = verify_cms(&cms, Some(&digest), None).expect("cms parses");
        assert_ne!(
            info.status,
            SignatureStatus::DocumentModified,
            "missing messageDigest must not claim the file was altered: {:?}",
            info
        );
        assert_eq!(info.digest, DigestCheck::NotComputed);
        assert_eq!(info.rsa, RsaCheck::NotRun);
        assert_eq!(info.status_detail, "signed_attributes_missing");
    }

    fn pdf_sig_without_byte_range() -> Vec<u8> {
        pdf_sig_with_contents(&cms_sha256_without_certificate(&[0x11u8; 32]))
    }

    fn pdf_sig_with_contents(cms: &[u8]) -> Vec<u8> {
        let placeholder = format!("<{}>", "0".repeat(cms.len() * 2));
        let mut file = Vec::new();
        file.extend_from_slice(
            b"%PDF-1.4
",
        );
        let obj1 = file.len();
        file.extend_from_slice(
            b"1 0 obj
<< /Type /Catalog /AcroForm 2 0 R /Pages 3 0 R >>
endobj
",
        );
        let obj2 = file.len();
        file.extend_from_slice(
            b"2 0 obj
<< /Fields [4 0 R] /SigFlags 3 >>
endobj
",
        );
        let obj3 = file.len();
        file.extend_from_slice(
            b"3 0 obj
<< /Type /Pages /Kids [] /Count 0 >>
endobj
",
        );
        let obj4 = file.len();
        file.extend_from_slice(
            b"4 0 obj
<< /FT /Sig /T (Sig1) /V 5 0 R >>
endobj
",
        );
        let obj5 = file.len();
        file.extend_from_slice(
            b"5 0 obj
<< /Type /Sig /Filter /Adobe.PPKLite /SubFilter /adbe.pkcs7.detached /Contents ",
        );
        let gap_at = file.len();
        file.extend_from_slice(placeholder.as_bytes());
        let after_gap = file.len();
        file.extend_from_slice(
            b" >>
endobj
",
        );
        let xref_at = file.len();
        let xref = format!(
            "xref
0 6
0000000000 65535 f 
{obj1:010} 00000 n 
{obj2:010} 00000 n 
{obj3:010} 00000 n 
{obj4:010} 00000 n 
{obj5:010} 00000 n 
trailer
<< /Size 6 /Root 1 0 R >>
startxref
{xref_at:010}
%%EOF
"
        );
        file.extend_from_slice(xref.as_bytes());
        let contents: String = cms.iter().map(|b| format!("{b:02X}")).collect();
        let filled = format!("<{contents}>");
        assert_eq!(filled.len(), placeholder.len());
        file[gap_at..after_gap].copy_from_slice(filled.as_bytes());
        file
    }

    /// A /Sig dictionary with Contents but no ByteRange must not claim the
    /// document was altered — nothing was compared (#161 defect 1).
    #[test]
    fn missing_byte_range_is_not_document_modified() {
        let pdf = pdf_sig_without_byte_range();
        let analysis = analyze_pdf(&pdf).expect("pdf parses");
        assert_eq!(analysis.signatures.len(), 1, "{analysis:?}");
        let sig = &analysis.signatures[0];
        assert!(sig.byte_range.is_none(), "{sig:?}");
        assert!(!sig.covers_whole_document, "{sig:?}");
        assert_ne!(
            sig.status,
            SignatureStatus::DocumentModified,
            "missing ByteRange must not claim alteration: {:?}",
            sig
        );
        assert_eq!(sig.status, SignatureStatus::Unsupported);
        assert_eq!(sig.status_detail, "byte_range_missing");
        assert_eq!(sig.digest, DigestCheck::NotComputed);
        assert_eq!(sig.bytes_after_signed_range, None);
        assert_eq!(sig.rsa, RsaCheck::NotRun);
    }

    // --- Generated fixtures for the #161 cases: every input is built in code,
    // so no binary test file is versioned for these paths.

    const OID_SHA256: &[u8] = &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01];
    const OID_RSA_ENCRYPTION: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01];
    const OID_SHA256_RSA: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x0b];
    const OID_DATA: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x07, 0x01];
    const OID_SIGNED_DATA: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x07, 0x02];
    const OID_MESSAGE_DIGEST: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x09, 0x04];
    const OID_CONTENT_TYPE: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x09, 0x03];
    const OID_EC_PUBLIC_KEY: &[u8] = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01];
    const OID_PRIME256V1: &[u8] = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07];
    const OID_COMMON_NAME: &[u8] = &[0x55, 0x04, 0x03];
    const OID_SUBJECT_KEY_ID: &[u8] = &[0x55, 0x1d, 0x0e];
    const CERT_VALID_FROM: &str = "200101000000Z";
    const CERT_VALID_TO: &str = "491231235959Z";

    fn der_alg_id(oid: &[u8]) -> Vec<u8> {
        der_tlv(0x30, &der_cat(&[&der_tlv(0x06, oid), &der_tlv(0x05, &[])]))
    }

    fn der_name(cn: &str) -> Vec<u8> {
        der_tlv(
            0x30,
            &der_tlv(
                0x31,
                &der_tlv(
                    0x30,
                    &der_cat(&[
                        &der_tlv(0x06, OID_COMMON_NAME),
                        &der_tlv(0x0c, cn.as_bytes()),
                    ]),
                ),
            ),
        )
    }

    /// DER INTEGER, adding the sign byte when the leading bit is set.
    fn der_uint(bytes: &[u8]) -> Vec<u8> {
        let mut value = bytes.to_vec();
        if value.first().is_some_and(|b| b & 0x80 != 0) {
            value.insert(0, 0);
        }
        der_tlv(0x02, &value)
    }

    fn rsa_spki(public: &rsa::RsaPublicKey) -> Vec<u8> {
        use rsa::traits::PublicKeyParts;
        let key = der_tlv(
            0x30,
            &der_cat(&[
                &der_uint(&public.n().to_bytes_be()),
                &der_uint(&public.e().to_bytes_be()),
            ]),
        );
        let bits = der_tlv(0x03, &der_cat(&[&[0u8], &key]));
        der_tlv(0x30, &der_cat(&[&der_alg_id(OID_RSA_ENCRYPTION), &bits]))
    }

    /// An EC key the crate cannot verify: only the algorithm identifier matters.
    fn ec_spki() -> Vec<u8> {
        let alg = der_tlv(
            0x30,
            &der_cat(&[
                &der_tlv(0x06, OID_EC_PUBLIC_KEY),
                &der_tlv(0x06, OID_PRIME256V1),
            ]),
        );
        let mut point = vec![0x04u8];
        point.extend([0u8; 64]);
        let bits = der_tlv(0x03, &der_cat(&[&[0u8], &point]));
        der_tlv(0x30, &der_cat(&[&alg, &bits]))
    }

    fn subject_key_id_extension(ski: &[u8]) -> Vec<u8> {
        let extension = der_tlv(
            0x30,
            &der_cat(&[
                &der_tlv(0x06, OID_SUBJECT_KEY_ID),
                &der_tlv(0x04, &der_tlv(0x04, ski)),
            ]),
        );
        der_tlv(0xa3, &der_tlv(0x30, &extension))
    }

    /// Minimal X.509 certificate. The verifier never checks the certificate's
    /// own signature, so a placeholder BIT STRING is enough.
    fn generate_cert(
        spki: &[u8],
        serial: u8,
        cn: &str,
        not_before: &str,
        not_after: &str,
        ski: Option<&[u8]>,
    ) -> Vec<u8> {
        let signature_algorithm = der_alg_id(OID_SHA256_RSA);
        let issuer = der_name("Test CA");
        let subject = der_name(cn);
        let validity = der_tlv(
            0x30,
            &der_cat(&[
                &der_tlv(0x17, not_before.as_bytes()),
                &der_tlv(0x17, not_after.as_bytes()),
            ]),
        );
        let mut tbs = der_cat(&[
            &der_tlv(0xa0, &der_tlv(0x02, &[2])),
            &der_tlv(0x02, &[serial]),
            &signature_algorithm,
            &issuer,
            &validity,
            &subject,
            spki,
        ]);
        if let Some(ski) = ski {
            tbs.extend(subject_key_id_extension(ski));
        }
        let tbs = der_tlv(0x30, &tbs);
        der_tlv(
            0x30,
            &der_cat(&[&tbs, &signature_algorithm, &der_tlv(0x03, &[0x00, 0x00])]),
        )
    }

    fn rsa_keypair() -> (rsa::RsaPrivateKey, rsa::RsaPublicKey) {
        let private =
            rsa::RsaPrivateKey::new(&mut rsa::rand_core::OsRng, 1024).expect("generate RSA key");
        let public = private.to_public_key();
        (private, public)
    }

    /// Signs the DER `SET OF Attribute` form, which is what the verifier feeds
    /// to RSA after rewriting the IMPLICIT tag.
    fn sign_attrs(private: &rsa::RsaPrivateKey, attrs_inner: &[u8]) -> Vec<u8> {
        use rsa::signature::{SignatureEncoding, Signer};
        let key = rsa::pkcs1v15::SigningKey::<Sha256>::new(private.clone());
        key.sign(&der_tlv(0x31, attrs_inner)).to_vec()
    }

    /// Concatenated signed attributes (no SET header), with messageDigest.
    fn message_digest_attrs(digest: &[u8]) -> Vec<u8> {
        let digest_attr = der_tlv(
            0x30,
            &der_cat(&[
                &der_tlv(0x06, OID_MESSAGE_DIGEST),
                &der_tlv(0x31, &der_tlv(0x04, digest)),
            ]),
        );
        let content_type = der_tlv(
            0x30,
            &der_cat(&[
                &der_tlv(0x06, OID_CONTENT_TYPE),
                &der_tlv(0x31, &der_tlv(0x06, OID_DATA)),
            ]),
        );
        der_cat(&[&digest_attr, &content_type])
    }

    fn signer_id_issuer_serial(issuer_cn: &str, serial: u8) -> Vec<u8> {
        der_tlv(
            0x30,
            &der_cat(&[&der_name(issuer_cn), &der_tlv(0x02, &[serial])]),
        )
    }

    /// `signed_attrs` is the concatenation of attributes (no SET header), as it
    /// travels inside SignerInfo.
    fn build_cms(
        certs: &[Vec<u8>],
        sid: &[u8],
        signed_attrs: Option<&[u8]>,
        signature: &[u8],
    ) -> Vec<u8> {
        let sha256 = der_alg_id(OID_SHA256);
        let mut signer = der_cat(&[&der_tlv(0x02, &[1]), sid, &sha256]);
        if let Some(attrs) = signed_attrs {
            signer.extend(der_tlv(0xa0, attrs));
        }
        signer.extend(der_alg_id(OID_RSA_ENCRYPTION));
        signer.extend(der_tlv(0x04, signature));
        let signer = der_tlv(0x30, &signer);

        let mut signed_data = der_cat(&[
            &der_tlv(0x02, &[1]),
            &der_tlv(0x31, &sha256),
            &der_tlv(0x30, &der_tlv(0x06, OID_DATA)),
        ]);
        if !certs.is_empty() {
            let mut blob = Vec::new();
            for cert in certs {
                blob.extend_from_slice(cert);
            }
            signed_data.extend(der_tlv(0xa0, &blob));
        }
        signed_data.extend(der_tlv(0x31, &signer));
        der_tlv(
            0x30,
            &der_cat(&[
                &der_tlv(0x06, OID_SIGNED_DATA),
                &der_tlv(0xa0, &der_tlv(0x30, &signed_data)),
            ]),
        )
    }

    struct SignedPdf {
        bytes: Vec<u8>,
        gap_at: usize,
        after_gap: usize,
        signed_end: usize,
    }

    impl SignedPdf {
        fn signed_digest(&self) -> Vec<u8> {
            let mut hasher = Sha256::new();
            hasher.update(&self.bytes[..self.gap_at]);
            hasher.update(&self.bytes[self.after_gap..self.signed_end]);
            hasher.finalize().to_vec()
        }
    }

    /// PDF whose /ByteRange skips only the /Contents hex string. `tail` lands
    /// after the signed range, simulating bytes written after signing.
    fn build_signed_pdf(cms: &[u8], tail: &[u8]) -> SignedPdf {
        let hex: String = cms.iter().map(|b| format!("{b:02X}")).collect();
        let contents = format!("<{hex}>");
        let mut file = Vec::new();
        file.extend_from_slice(b"%PDF-1.4\n");
        let obj1 = file.len();
        file.extend_from_slice(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n");
        let obj2 = file.len();
        file.extend_from_slice(b"2 0 obj\n<< /Type /Pages /Kids [] /Count 0 >>\nendobj\n");
        let obj3 = file.len();
        file.extend_from_slice(
            b"3 0 obj\n<< /Type /Sig /Filter /Adobe.PPKLite /SubFilter /adbe.pkcs7.detached /ByteRange ",
        );
        let br_at = file.len();
        file.extend_from_slice(b"[0000000000 0000000000 0000000000 0000000000]");
        file.extend_from_slice(b" /Contents ");
        let gap_at = file.len();
        file.extend_from_slice(contents.as_bytes());
        let after_gap = file.len();
        file.extend_from_slice(b" >>\nendobj\n");
        let xref_at = file.len();
        let xref = format!(
            "xref\n0 4\n0000000000 65535 f \n{obj1:010} 00000 n \n{obj2:010} 00000 n \n{obj3:010} 00000 n \ntrailer\n<< /Size 4 /Root 1 0 R >>\nstartxref\n{xref_at:010}\n%%EOF\n"
        );
        file.extend_from_slice(xref.as_bytes());
        let signed_end = file.len();
        file.extend_from_slice(tail);
        let br = format!(
            "[{:010} {:010} {:010} {:010}]",
            0,
            gap_at,
            after_gap,
            signed_end - after_gap
        );
        assert_eq!(br.len(), 45);
        file[br_at..br_at + br.len()].copy_from_slice(br.as_bytes());
        SignedPdf {
            bytes: file,
            gap_at,
            after_gap,
            signed_end,
        }
    }

    /// Builds a PDF whose CMS is computed over its own ranges: `make_cms`
    /// receives the digest of the covered bytes. The CMS length does not depend
    /// on the digest value, so a probe pass settles the layout first.
    fn pdf_signed_by(make_cms: impl Fn(&[u8]) -> Vec<u8>, tail: &[u8]) -> Vec<u8> {
        let probe = build_signed_pdf(&make_cms(&[0u8; 32]), tail);
        let digest = probe.signed_digest();
        let pdf = build_signed_pdf(&make_cms(&digest), tail);
        assert_eq!(pdf.bytes.len(), probe.bytes.len());
        pdf.bytes
    }

    fn signed_pdf_by(
        private: &rsa::RsaPrivateKey,
        cert: &[u8],
        sid: &[u8],
        tail: &[u8],
    ) -> Vec<u8> {
        pdf_signed_by(
            |digest| {
                let attrs = message_digest_attrs(digest);
                let signature = sign_attrs(private, &attrs);
                build_cms(&[cert.to_vec()], sid, Some(&attrs), &signature)
            },
            tail,
        )
    }

    /// ECDSA signer certificates cannot be verified by the RSA path, and that
    /// must be reported as a missing key rather than as an altered document.
    #[test]
    fn unsupported_public_key_reports_key_unavailable() {
        let digest = [0x33u8; 32];
        let cert = generate_cert(
            &ec_spki(),
            1,
            "ECDSA Signer",
            CERT_VALID_FROM,
            CERT_VALID_TO,
            None,
        );
        let attrs = message_digest_attrs(&digest);
        let cms = build_cms(
            &[cert],
            &signer_id_issuer_serial("Test CA", 1),
            Some(&attrs),
            &[0u8; 128],
        );
        let info = verify_cms(&cms, Some(&digest), None).expect("cms parses");
        assert_eq!(info.digest, DigestCheck::Matched);
        assert_eq!(info.status, SignatureStatus::Invalid);
        assert_eq!(info.rsa, RsaCheck::KeyUnavailable);
        assert_eq!(info.status_detail, "rsa_key_unavailable");
        assert_eq!(info.signer_cert_matches_sid, Some(true));
        assert!(info.certificate.is_some(), "{info:?}");
    }

    /// The certificate named by the SignerIdentifier is used, not the first one
    /// in the set (#161 defect 4).
    #[test]
    fn signer_identifier_selects_the_matching_certificate() {
        let (_decoy_private, decoy_public) = rsa_keypair();
        let (private, public) = rsa_keypair();
        let decoy = generate_cert(
            &rsa_spki(&decoy_public),
            41,
            "Outro signatário",
            CERT_VALID_FROM,
            CERT_VALID_TO,
            None,
        );
        let signer = generate_cert(
            &rsa_spki(&public),
            42,
            "Signatário",
            CERT_VALID_FROM,
            CERT_VALID_TO,
            None,
        );
        let digest = [0x44u8; 32];
        let attrs = message_digest_attrs(&digest);
        let signature = sign_attrs(&private, &attrs);
        let cms = build_cms(
            &[decoy, signer],
            &signer_id_issuer_serial("Test CA", 42),
            Some(&attrs),
            &signature,
        );
        let info = verify_cms(&cms, Some(&digest), None).expect("cms parses");
        assert_eq!(info.signer_cert_matches_sid, Some(true));
        assert_eq!(info.digest, DigestCheck::Matched);
        assert_eq!(info.rsa, RsaCheck::Passed);
        assert_eq!(info.clock, ClockCheck::WithinValidity);
        assert_eq!(info.status, SignatureStatus::IntactButUntrusted);
        assert_eq!(info.status_detail, "intact_chain_unverified");
        assert_eq!(
            info.certificate.as_ref().map(|c| c.serial.as_str()),
            Some("2a")
        );
    }

    /// Without a certificate matching the SignerIdentifier the signature is
    /// never checked against an unrelated key.
    #[test]
    fn signer_identifier_without_match_never_verifies() {
        let (private, public) = rsa_keypair();
        let cert = generate_cert(
            &rsa_spki(&public),
            43,
            "Signatário",
            CERT_VALID_FROM,
            CERT_VALID_TO,
            None,
        );
        let digest = [0x55u8; 32];
        let attrs = message_digest_attrs(&digest);
        let signature = sign_attrs(&private, &attrs);
        let cms = build_cms(
            &[cert],
            &signer_id_issuer_serial("Test CA", 99),
            Some(&attrs),
            &signature,
        );
        let info = verify_cms(&cms, Some(&digest), None).expect("cms parses");
        assert_eq!(info.signer_cert_matches_sid, Some(false));
        assert!(info.certificate.is_none(), "{info:?}");
        assert_eq!(info.status, SignatureStatus::Invalid);
        assert_eq!(info.rsa, RsaCheck::KeyUnavailable);
        assert_eq!(info.status_detail, "signer_certificate_not_found");
    }

    #[test]
    fn subject_key_identifier_selects_the_certificate() {
        let (private, public) = rsa_keypair();
        let ski = [0xAB, 0xCD, 0xEF];
        let cert = generate_cert(
            &rsa_spki(&public),
            44,
            "Signatário",
            CERT_VALID_FROM,
            CERT_VALID_TO,
            Some(&ski),
        );
        let digest = [0x66u8; 32];
        let attrs = message_digest_attrs(&digest);
        let signature = sign_attrs(&private, &attrs);
        let cms = build_cms(&[cert], &der_tlv(0x80, &ski), Some(&attrs), &signature);
        let info = verify_cms(&cms, Some(&digest), None).expect("cms parses");
        assert_eq!(info.signer_cert_matches_sid, Some(true));
        assert_eq!(info.rsa, RsaCheck::Passed);
        assert_eq!(info.status, SignatureStatus::IntactButUntrusted);
    }

    /// A wrong key is a failed RSA check, distinct from a parse error.
    #[test]
    fn rsa_mismatch_reports_a_failed_check() {
        let (private, _public) = rsa_keypair();
        let (_other_private, other_public) = rsa_keypair();
        let cert = generate_cert(
            &rsa_spki(&other_public),
            45,
            "Signatário",
            CERT_VALID_FROM,
            CERT_VALID_TO,
            None,
        );
        let digest = [0x77u8; 32];
        let attrs = message_digest_attrs(&digest);
        let signature = sign_attrs(&private, &attrs);
        let cms = build_cms(
            &[cert],
            &signer_id_issuer_serial("Test CA", 45),
            Some(&attrs),
            &signature,
        );
        let info = verify_cms(&cms, Some(&digest), None).expect("cms parses");
        assert_eq!(info.digest, DigestCheck::Matched);
        assert_eq!(info.status, SignatureStatus::Invalid);
        assert_eq!(info.rsa, RsaCheck::Failed);
        assert_eq!(info.status_detail, "signature_mismatch");
    }

    #[test]
    fn certificate_clock_states_are_reported() {
        let (private, public) = rsa_keypair();
        let spki = rsa_spki(&public);
        let digest = [0x88u8; 32];
        let attrs = message_digest_attrs(&digest);
        let signature = sign_attrs(&private, &attrs);

        let expired = generate_cert(&spki, 46, "Antigo", "200101000000Z", "200201000000Z", None);
        let cms = build_cms(
            &[expired],
            &signer_id_issuer_serial("Test CA", 46),
            Some(&attrs),
            &signature,
        );
        let info = verify_cms(&cms, Some(&digest), None).expect("cms parses");
        assert_eq!(info.status, SignatureStatus::CertificateExpired);
        assert_eq!(info.clock, ClockCheck::Expired);
        assert_eq!(info.status_detail, "certificate_expired");

        let future = generate_cert(&spki, 47, "Futuro", "491231235959Z", "500101000000Z", None);
        let cms = build_cms(
            &[future],
            &signer_id_issuer_serial("Test CA", 47),
            Some(&attrs),
            &signature,
        );
        let info = verify_cms(&cms, Some(&digest), None).expect("cms parses");
        assert_eq!(info.status, SignatureStatus::CertificateNotYetValid);
        assert_eq!(info.clock, ClockCheck::NotYetValid);
        assert_eq!(info.status_detail, "certificate_not_yet_valid");
    }

    /// Signed attributes without messageDigest keep their own explanation
    /// instead of being overwritten by the document-modified verdict.
    #[test]
    fn signed_attributes_without_message_digest_keep_their_own_detail() {
        let digest = [0x99u8; 32];
        let content_type = der_tlv(
            0x30,
            &der_cat(&[
                &der_tlv(0x06, OID_CONTENT_TYPE),
                &der_tlv(0x31, &der_tlv(0x06, OID_DATA)),
            ]),
        );
        let cms = build_cms(
            &[],
            &signer_id_issuer_serial("Test CA", 1),
            Some(&content_type),
            &[0u8; 128],
        );
        let info = verify_cms(&cms, Some(&digest), None).expect("cms parses");
        assert_eq!(info.digest, DigestCheck::NotComputed);
        assert_eq!(info.rsa, RsaCheck::NotRun);
        assert_ne!(info.status, SignatureStatus::DocumentModified);
        assert_eq!(info.status_detail, "message_digest_missing");
    }

    #[test]
    fn unparseable_cms_is_a_parse_error() {
        let pdf = pdf_sig_with_contents(&[0x30, 0x03, 0x02, 0x01, 0x00]);
        let analysis = analyze_pdf(&pdf).expect("pdf parses");
        assert_eq!(analysis.signatures.len(), 1, "{analysis:?}");
        let sig = &analysis.signatures[0];
        assert_eq!(sig.status, SignatureStatus::Invalid);
        assert_eq!(sig.status_detail, "cms_parse_error");
        assert_eq!(sig.rsa, RsaCheck::NotRun);
        assert_eq!(sig.digest, DigestCheck::NotComputed);
        assert!(sig.certificate.is_none(), "{sig:?}");
    }

    /// A corrupt /ByteRange is reported on its own, never as an alteration.
    #[test]
    fn malformed_byte_range_is_reported_on_its_own() {
        let mut pdf = pdf_named_banco_with_matching_digest();
        let key = b"/ByteRange ";
        let key_at = pdf
            .windows(key.len())
            .position(|w| w == key)
            .expect("byte range key");
        let at = key_at + key.len();
        let width = 45;
        assert_eq!(pdf[at], b'[');
        assert_eq!(pdf[at + width - 1], b']');
        // Same width keeps every byte offset — and the digest input — untouched.
        let mut replacement = b"[0 0 0 /X]".to_vec();
        replacement.resize(width, b' ');
        pdf[at..at + width].copy_from_slice(&replacement);

        let analysis = analyze_pdf(&pdf).expect("pdf parses");
        assert_eq!(analysis.signatures.len(), 1, "{analysis:?}");
        let sig = &analysis.signatures[0];
        assert_eq!(sig.status, SignatureStatus::Unsupported);
        assert_eq!(sig.status_detail, "byte_range_malformed");
        assert_eq!(sig.digest, DigestCheck::NotComputed);
        assert_eq!(sig.bytes_after_signed_range, None);
        assert_eq!(sig.rsa, RsaCheck::NotRun);
    }

    /// End-to-end: a verified signature over the whole file.
    #[test]
    fn verified_signature_over_the_whole_document_is_intact() {
        let (private, public) = rsa_keypair();
        let cert = generate_cert(
            &rsa_spki(&public),
            48,
            "Signatário",
            CERT_VALID_FROM,
            CERT_VALID_TO,
            None,
        );
        let sid = signer_id_issuer_serial("Test CA", 48);
        let pdf = signed_pdf_by(&private, &cert, &sid, b"");
        let analysis = analyze_pdf(&pdf).expect("pdf parses");
        assert_eq!(analysis.signatures.len(), 1, "{analysis:?}");
        let sig = &analysis.signatures[0];
        assert!(sig.covers_whole_document, "{}", sig.status_detail);
        assert_eq!(sig.digest, DigestCheck::Matched);
        assert_eq!(sig.rsa, RsaCheck::Passed);
        assert_eq!(sig.signer_cert_matches_sid, Some(true));
        assert_eq!(sig.bytes_after_signed_range, Some(false));
        assert_eq!(sig.status, SignatureStatus::IntactButUntrusted);
        assert_eq!(sig.status_detail, "intact_chain_unverified");
    }

    /// End-to-end: the signature verifies but the file grew after signing.
    #[test]
    fn verified_signature_with_bytes_after_the_range_is_document_modified() {
        let (private, public) = rsa_keypair();
        let cert = generate_cert(
            &rsa_spki(&public),
            49,
            "Signatário",
            CERT_VALID_FROM,
            CERT_VALID_TO,
            None,
        );
        let sid = signer_id_issuer_serial("Test CA", 49);
        let pdf = signed_pdf_by(&private, &cert, &sid, b"% incremental update\n");
        let analysis = analyze_pdf(&pdf).expect("pdf parses");
        assert_eq!(analysis.signatures.len(), 1, "{analysis:?}");
        let sig = &analysis.signatures[0];
        assert!(!sig.covers_whole_document, "{}", sig.status_detail);
        assert_eq!(sig.digest, DigestCheck::Matched);
        assert_eq!(sig.rsa, RsaCheck::Passed);
        assert_eq!(sig.bytes_after_signed_range, Some(true));
        assert_eq!(sig.status, SignatureStatus::DocumentModified);
        assert_eq!(sig.status_detail, "bytes_after_signed_range");
    }
}
