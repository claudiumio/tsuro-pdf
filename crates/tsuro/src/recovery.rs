//! Diário de recuperação de marcações (#128, `docs/roadmap/investigacao-f-marc-01.md`).
//!
//! Cada aba grava um snapshot completo das marcações em
//! `<dir>/<instância>.journal`, com o anterior em `.prev`, e segura um lock
//! exclusivo em `<instância>.lock` enquanto vive. O lock fica num arquivo
//! lateral porque o rename troca o inode do diário. Diário sem dono vivo é
//! oferecido na próxima abertura (`abandoned`).

#![allow(dead_code)]

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::page::{PageNo, Quad};
use crate::positions::{escape_field, escape_path, unescape_field, unescape_path};
use crate::session::{AnnotKind, Annotation, TextRange};

const MAGIC: &str = "tsuro-recovery 1";
const LOCK: &str = "lock";
const JOURNAL: &str = "journal";
const PREV: &str = "prev";
const TMP: &str = "tmp";

/// Identidade do conteúdo do PDF; o caminho do `Checkpoint` é só exibição.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ContentId {
    pub hash: u64,
    pub len: u64,
    pub pages: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Checkpoint {
    pub content: ContentId,
    pub path: PathBuf,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct InstanceId(String);

static NEXT_INSTANCE: AtomicU64 = AtomicU64::new(0);

impl InstanceId {
    /// O contador separa duas abas do mesmo processo no mesmo tique do
    /// relógio (no macOS o `SystemTime` anda de microssegundo em microssegundo).
    fn fresh() -> Self {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let seq = NEXT_INSTANCE.fetch_add(1, Ordering::Relaxed);
        InstanceId(format!("{:x}-{nanos:x}-{seq:x}", std::process::id()))
    }

    fn parse(raw: &str) -> Option<Self> {
        let valid = !raw.is_empty() && raw.bytes().all(|b| b.is_ascii_hexdigit() || b == b'-');
        valid.then(|| InstanceId(raw.to_string()))
    }

    fn file(&self, dir: &Path, ext: &str) -> PathBuf {
        dir.join(format!("{}.{ext}", self.0))
    }
}

/// Diário de uma aba. Soltar sem `retire` mantém os arquivos no disco (é o
/// caso do crash) e libera só o lock.
#[derive(Debug)]
pub(crate) struct Journal {
    id: InstanceId,
    dir: PathBuf,
    _lock: File,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Abandoned {
    pub id: InstanceId,
    pub checkpoint: Checkpoint,
}

/// FNV-1a 64 (decisão D-C): pega mudança acidental, não adulteração.
pub(crate) fn content_hash(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, &byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

#[cfg(test)]
use std::cell::RefCell;

#[cfg(test)]
thread_local! {
    static RECOVERY_DIR: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
}

#[cfg(test)]
pub(crate) fn with_recovery_dir<R>(dir: PathBuf, f: impl FnOnce() -> R) -> R {
    RECOVERY_DIR.with(|slot| *slot.borrow_mut() = Some(dir));
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    RECOVERY_DIR.with(|slot| *slot.borrow_mut() = None);
    match result {
        Ok(value) => value,
        Err(panic) => std::panic::resume_unwind(panic),
    }
}

pub(crate) fn recovery_dir() -> PathBuf {
    #[cfg(test)]
    {
        if let Some(dir) = RECOVERY_DIR.with(|slot| slot.borrow().clone()) {
            return dir;
        }
    }
    if let Some(dir) = std::env::var_os("TSURO_RECOVERY") {
        return PathBuf::from(dir);
    }
    if cfg!(target_os = "macos") {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home).join("Library/Application Support/Tsuro/recovery");
        }
    }
    if cfg!(target_os = "windows") {
        if let Some(local) = std::env::var_os("LOCALAPPDATA") {
            return PathBuf::from(local).join("TsuroPDF").join("recovery");
        }
    }
    if let Some(xdg) = std::env::var_os("XDG_DATA_HOME") {
        return PathBuf::from(xdg).join("tsuro/recovery");
    }
    if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
        return PathBuf::from(home).join(".local/share/tsuro/recovery");
    }
    std::env::temp_dir().join("tsuro-recovery")
}

impl Journal {
    pub(crate) fn create(dir: &Path) -> io::Result<Journal> {
        create_private_dir(dir)?;
        let id = InstanceId::fresh();
        let lock = private_options()
            .read(true)
            .write(true)
            .create_new(true)
            .open(id.file(dir, LOCK))?;
        lock.try_lock()?;
        Ok(Journal {
            id,
            dir: dir.to_path_buf(),
            _lock: lock,
        })
    }

    pub(crate) fn id(&self) -> &InstanceId {
        &self.id
    }

    /// Grava `.tmp`, gira `.journal` para `.prev` e promove o `.tmp`. Morrer
    /// entre os dois renames deixa o último checkpoint válido em `.prev`.
    pub(crate) fn write(&self, cp: &Checkpoint) -> io::Result<()> {
        let tmp = self.id.file(&self.dir, TMP);
        let journal = self.id.file(&self.dir, JOURNAL);
        let mut file = private_options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&tmp)?;
        file.write_all(encode(cp).as_bytes())?;
        file.sync_all()?;
        drop(file);
        match std::fs::rename(&journal, self.id.file(&self.dir, PREV)) {
            Err(err) if err.kind() != io::ErrorKind::NotFound => return Err(err),
            _ => {}
        }
        std::fs::rename(&tmp, &journal)?;
        sync_dir(&self.dir)
    }

    pub(crate) fn retire(self) -> io::Result<()> {
        remove_instance(&self.dir, &self.id, self._lock)
    }

    /// Apaga o diário de `old` depois de prender o lock dele. Exige um
    /// checkpoint próprio válido antes: uma falha no meio deixa os dois
    /// diários, nunca nenhum.
    pub(crate) fn adopt(&self, old: Abandoned) -> io::Result<()> {
        if load(&self.dir, &self.id).is_none() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "adotar exige um checkpoint próprio gravado",
            ));
        }
        let lock = private_options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(old.id.file(&self.dir, LOCK))?;
        lock.try_lock()?;
        remove_instance(&self.dir, &old.id, lock)
    }
}

/// Diários sem dono vivo, cada um já validado. Dono vivo = `.lock` preso.
pub(crate) fn abandoned(dir: &Path) -> Vec<Abandoned> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut ids: Vec<InstanceId> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            let ext = path.extension()?.to_str()?;
            if ext != JOURNAL && ext != PREV {
                return None;
            }
            InstanceId::parse(path.file_stem()?.to_str()?)
        })
        .collect();
    ids.sort();
    ids.dedup();
    ids.into_iter()
        .filter(|id| !owner_alive(dir, id))
        .filter_map(|id| {
            let checkpoint = load(dir, &id)?;
            Some(Abandoned { id, checkpoint })
        })
        .collect()
}

pub(crate) fn load(dir: &Path, id: &InstanceId) -> Option<Checkpoint> {
    [JOURNAL, PREV].into_iter().find_map(|ext| {
        let raw = std::fs::read_to_string(id.file(dir, ext)).ok()?;
        decode(&raw)
    })
}

pub(crate) fn encode(cp: &Checkpoint) -> String {
    let ContentId { hash, len, pages } = cp.content;
    let mut body = format!(
        "{MAGIC}\ncontent {hash:016x} {len} {pages}\npath {}\n",
        escape_path(&cp.path)
    );
    for annot in &cp.annotations {
        body.push_str(&format!(
            "annot {} {} {} {} {} {} {} {}\n",
            annot.id,
            annot.page.index(),
            kind_token(annot.kind),
            annot.range.start,
            annot.range.end,
            encode_marker(annot.marker),
            encode_quads(&annot.quads),
            escape_field(&annot.text),
        ));
    }
    let checksum = content_hash(body.as_bytes());
    body.push_str(&format!("end {} {checksum:016x}\n", cp.annotations.len()));
    body
}

/// Tudo ou nada: qualquer linha ilegível, contagem ou checksum errados → `None`.
pub(crate) fn decode(raw: &str) -> Option<Checkpoint> {
    let end_start = raw.strip_suffix('\n')?.rfind('\n')? + 1;
    let (body, end_line) = raw.split_at(end_start);
    let [tag, count, checksum] = split_fields(end_line.strip_suffix('\n')?)?;
    let count: usize = count.parse().ok()?;
    let checksum = u64::from_str_radix(checksum, 16).ok()?;
    if tag != "end" || checksum != content_hash(body.as_bytes()) {
        return None;
    }
    let mut lines = body.strip_suffix('\n')?.split('\n');
    if lines.next()? != MAGIC {
        return None;
    }
    let [hash, len, pages] = split_fields(lines.next()?.strip_prefix("content ")?)?;
    let content = ContentId {
        hash: u64::from_str_radix(hash, 16).ok()?,
        len: len.parse().ok()?,
        pages: pages.parse().ok()?,
    };
    let path = unescape_path(lines.next()?.strip_prefix("path ")?);
    let annotations = lines.map(decode_annot).collect::<Option<Vec<_>>>()?;
    (annotations.len() == count).then_some(Checkpoint {
        content,
        path,
        annotations,
    })
}

fn decode_annot(line: &str) -> Option<Annotation> {
    let [tag, id, page, kind, start, end, marker, quads, text] = split_fields(line)?;
    if tag != "annot" {
        return None;
    }
    Some(Annotation {
        id: id.parse().ok()?,
        page: PageNo::from_index(page.parse().ok()?),
        range: TextRange {
            start: start.parse().ok()?,
            end: end.parse().ok()?,
        },
        quads: decode_quads(quads)?,
        kind: kind_from_token(kind)?,
        text: unescape_field(text),
        marker: decode_marker(marker)?,
    })
}

/// `N` campos separados por espaço; o último leva o resto da linha.
fn split_fields<const N: usize>(line: &str) -> Option<[&str; N]> {
    line.splitn(N, ' ').collect::<Vec<_>>().try_into().ok()
}

const KINDS: [AnnotKind; 4] = [
    AnnotKind::Highlight,
    AnnotKind::Underline,
    AnnotKind::Strikeout,
    AnnotKind::Note,
];

fn kind_token(kind: AnnotKind) -> &'static str {
    match kind {
        AnnotKind::Highlight => "highlight",
        AnnotKind::Underline => "underline",
        AnnotKind::Strikeout => "strikeout",
        AnnotKind::Note => "note",
    }
}

fn kind_from_token(token: &str) -> Option<AnnotKind> {
    KINDS.into_iter().find(|&kind| kind_token(kind) == token)
}

/// Bits do `f32` em hex: a geometria volta exata, `-0.0` incluso.
fn encode_f32(value: f32) -> String {
    format!("{:08x}", value.to_bits())
}

fn decode_f32(raw: &str) -> Option<f32> {
    u32::from_str_radix(raw, 16).ok().map(f32::from_bits)
}

fn encode_marker(marker: Option<[f32; 2]>) -> String {
    match marker {
        Some([x, y]) => format!("{},{}", encode_f32(x), encode_f32(y)),
        None => "-".to_string(),
    }
}

fn decode_marker(raw: &str) -> Option<Option<[f32; 2]>> {
    if raw == "-" {
        return Some(None);
    }
    let (x, y) = raw.split_once(',')?;
    Some(Some([decode_f32(x)?, decode_f32(y)?]))
}

fn encode_quads(quads: &[Quad]) -> String {
    if quads.is_empty() {
        return "-".to_string();
    }
    quads
        .iter()
        .flat_map(|q| [q.x0, q.y0, q.x1, q.y1, q.x2, q.y2, q.x3, q.y3])
        .map(encode_f32)
        .collect::<Vec<_>>()
        .join(",")
}

fn decode_quads(raw: &str) -> Option<Vec<Quad>> {
    if raw == "-" {
        return Some(Vec::new());
    }
    let values = raw.split(',').map(decode_f32).collect::<Option<Vec<_>>>()?;
    if values.len() % 8 != 0 {
        return None;
    }
    Some(
        values
            .chunks_exact(8)
            .map(|v| Quad {
                x0: v[0],
                y0: v[1],
                x1: v[2],
                y1: v[3],
                x2: v[4],
                y2: v[5],
                x3: v[6],
                y3: v[7],
            })
            .collect(),
    )
}

fn owner_alive(dir: &Path, id: &InstanceId) -> bool {
    match OpenOptions::new()
        .read(true)
        .write(true)
        .open(id.file(dir, LOCK))
    {
        Err(err) => err.kind() != io::ErrorKind::NotFound,
        // Soltar o handle solta a sonda junto.
        Ok(probe) => probe.try_lock().is_err(),
    }
}

/// O `.lock` sai por último e o handle só fecha depois: ninguém vê os dados
/// da instância sem dono enquanto eles somem.
fn remove_instance(dir: &Path, id: &InstanceId, lock: File) -> io::Result<()> {
    for ext in [JOURNAL, PREV, TMP, LOCK] {
        match std::fs::remove_file(id.file(dir, ext)) {
            Err(err) if err.kind() != io::ErrorKind::NotFound => return Err(err),
            _ => {}
        }
    }
    sync_dir(dir)?;
    drop(lock);
    Ok(())
}

/// Notas e caminhos são dados do usuário (D-H): `0700` na pasta, `0600` nos arquivos.
fn private_options() -> OpenOptions {
    let mut options = OpenOptions::new();
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    options
}

fn create_private_dir(dir: &Path) -> io::Result<()> {
    let mut builder = std::fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder.create(dir)
}

/// Sem fsync da pasta, o rename pode não sobreviver a uma queda de energia.
/// No Windows não dá para abrir pasta como arquivo; o NTFS registra o rename.
fn sync_dir(dir: &Path) -> io::Result<()> {
    #[cfg(unix)]
    File::open(dir)?.sync_all()?;
    #[cfg(not(unix))]
    let _ = dir;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn temp_dir(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "tsuro-recovery-unit-{}-{}-{name}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0),
        ))
    }

    fn names(dir: &Path) -> BTreeSet<String> {
        std::fs::read_dir(dir)
            .unwrap()
            .map(|e| e.unwrap().file_name().into_string().unwrap())
            .collect()
    }

    fn instance_files(id: &InstanceId, exts: &[&str]) -> BTreeSet<String> {
        exts.iter().map(|ext| format!("{}.{ext}", id.0)).collect()
    }

    fn annot(id: u64, kind: AnnotKind, text: &str) -> Annotation {
        Annotation {
            id,
            page: PageNo::from_index(id as u32),
            range: TextRange { start: 3, end: 17 },
            quads: vec![Quad::from_rect(10.0, 20.0, 110.0, 32.5)],
            kind,
            text: text.to_string(),
            marker: None,
        }
    }

    fn checkpoint(texts: &[&str]) -> Checkpoint {
        Checkpoint {
            content: ContentId {
                hash: 0xdead_beef_0123_4567,
                len: 98_765,
                pages: 12,
            },
            path: PathBuf::from("/tmp/a pasta/contrato.pdf"),
            annotations: texts
                .iter()
                .enumerate()
                .map(|(i, text)| annot(i as u64, AnnotKind::Note, text))
                .collect(),
        }
    }

    fn tricky_checkpoint() -> Checkpoint {
        let mut highlight = annot(4, AnnotKind::Highlight, "");
        highlight.quads = vec![Quad {
            x0: 0.1,
            y0: -0.0,
            x1: f32::MIN_POSITIVE,
            y1: f32::MAX,
            x2: -1.5e-30,
            y2: 595.276,
            x3: 0.0,
            y3: 841.89,
        }];
        let mut underline = annot(7, AnnotKind::Underline, "");
        underline.quads.push(Quad::from_rect(-0.0, 0.3, 0.7, 1.1));
        let mut strikeout = annot(9, AnnotKind::Strikeout, "");
        strikeout.quads.clear();
        let mut note = annot(
            12,
            AnnotKind::Note,
            "linha 1\n\tcom tab \\ barra e \\n literal\nação  ",
        );
        note.marker = Some([-0.0, 123.456]);
        Checkpoint {
            content: ContentId {
                hash: u64::MAX,
                len: 0,
                pages: 1,
            },
            path: PathBuf::from("/tmp/x\ty\\z\nw  dir/doc.pdf"),
            annotations: vec![highlight, underline, strikeout, note],
        }
    }

    fn float_bits(cp: &Checkpoint) -> Vec<u32> {
        cp.annotations
            .iter()
            .flat_map(|a| {
                let quads = a
                    .quads
                    .iter()
                    .flat_map(|q| [q.x0, q.y0, q.x1, q.y1, q.x2, q.y2, q.x3, q.y3]);
                quads.chain(a.marker.into_iter().flatten())
            })
            .map(f32::to_bits)
            .collect()
    }

    #[test]
    fn encode_decode_round_trips_bit_exact() {
        let cp = tricky_checkpoint();
        let back = decode(&encode(&cp)).expect("checkpoint íntegro decodifica");
        assert_eq!(back, cp);
        assert_eq!(
            float_bits(&back),
            float_bits(&cp),
            "floats bit a bit, -0.0 incluso"
        );
    }

    #[test]
    fn every_truncation_is_rejected() {
        let encoded = encode(&tricky_checkpoint());
        assert!(decode(&encoded).is_some(), "arquivo inteiro é válido");
        for cut in (0..encoded.len()).filter(|&cut| encoded.is_char_boundary(cut)) {
            assert!(
                decode(&encoded[..cut]).is_none(),
                "prefixo de {cut}/{} bytes foi aceito",
                encoded.len()
            );
        }
    }

    #[test]
    fn every_flipped_byte_is_rejected() {
        let encoded = encode(&tricky_checkpoint()).into_bytes();
        for at in 0..encoded.len() {
            let mut bytes = encoded.clone();
            bytes[at] ^= 0x01;
            if let Ok(raw) = String::from_utf8(bytes) {
                assert!(decode(&raw).is_none(), "byte {at} trocado foi aceito");
            }
        }
    }

    #[test]
    fn unknown_version_and_count_mismatch_are_rejected() {
        let encoded = encode(&checkpoint(&["a", "b"]));
        let v2 = encoded.replacen(MAGIC, "tsuro-recovery 2", 1);
        let v2 = reseal(&v2);
        assert!(decode(&v2).is_none(), "versão desconhecida");
        let body = &encoded[..encoded.rfind("end ").unwrap()];
        let wrong_count = format!("{body}end 3 {:016x}\n", content_hash(body.as_bytes()));
        assert!(
            decode(&wrong_count).is_none(),
            "contagem diferente do corpo"
        );
        assert!(
            decode(&reseal(&encoded)).is_some(),
            "reseal preserva o válido"
        );
    }

    /// Recalcula a linha `end` para testar só a validação do corpo.
    fn reseal(encoded: &str) -> String {
        let body = &encoded[..encoded.rfind("end ").unwrap()];
        let count = body.lines().filter(|l| l.starts_with("annot ")).count();
        format!("{body}end {count} {:016x}\n", content_hash(body.as_bytes()))
    }

    #[test]
    fn corrupt_journal_falls_back_to_prev() {
        let dir = temp_dir("prev");
        let journal = Journal::create(&dir).unwrap();
        let first = checkpoint(&["primeira"]);
        let second = checkpoint(&["primeira", "segunda"]);
        journal.write(&first).unwrap();
        journal.write(&second).unwrap();
        assert_eq!(load(&dir, journal.id()), Some(second));
        let current = journal.id().file(&dir, JOURNAL);
        let raw = std::fs::read(&current).unwrap();
        std::fs::write(&current, &raw[..raw.len() / 2]).unwrap();
        assert_eq!(load(&dir, journal.id()), Some(first), "cai para o .prev");
        std::fs::write(journal.id().file(&dir, PREV), b"lixo").unwrap();
        assert_eq!(load(&dir, journal.id()), None, "nada válido, nada aplicado");
        journal.retire().unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn abandoned_skips_live_owners() {
        let dir = temp_dir("live");
        let a = Journal::create(&dir).unwrap();
        let b = Journal::create(&dir).unwrap();
        a.write(&checkpoint(&["a"])).unwrap();
        b.write(&checkpoint(&["b"])).unwrap();
        let second_handle = OpenOptions::new()
            .read(true)
            .write(true)
            .open(b.id().file(&dir, LOCK))
            .unwrap();
        assert!(
            second_handle.try_lock().is_err(),
            "outro handle do mesmo processo não pega o lock de uma aba viva"
        );
        drop(second_handle);
        assert!(abandoned(&dir).is_empty(), "as duas abas estão vivas");
        let a_id = a.id().clone();
        drop(a);
        assert_eq!(
            abandoned(&dir),
            vec![Abandoned {
                id: a_id,
                checkpoint: checkpoint(&["a"]),
            }],
            "só a aba que soltou o lock aparece"
        );
        b.retire().unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn retire_removes_only_its_own_files() {
        let dir = temp_dir("retire");
        let gone = Journal::create(&dir).unwrap();
        let kept = Journal::create(&dir).unwrap();
        gone.write(&checkpoint(&["1"])).unwrap();
        gone.write(&checkpoint(&["1", "2"])).unwrap();
        kept.write(&checkpoint(&["k"])).unwrap();
        let kept_id = kept.id().clone();
        gone.retire().unwrap();
        assert_eq!(names(&dir), instance_files(&kept_id, &[LOCK, JOURNAL]));
        kept.retire().unwrap();
        assert!(names(&dir).is_empty());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn adopt_removes_old_instance_after_own_checkpoint() {
        let dir = temp_dir("adopt");
        let crashed = Journal::create(&dir).unwrap();
        crashed.write(&checkpoint(&["velha"])).unwrap();
        crashed.write(&checkpoint(&["velha", "mais"])).unwrap();
        let old_id = crashed.id().clone();
        drop(crashed);
        let fresh = Journal::create(&dir).unwrap();
        let old = abandoned(&dir).pop().expect("diário sem dono é oferecido");
        assert_eq!(old.id, old_id);
        assert!(
            fresh.adopt(old.clone()).is_err(),
            "adotar antes do primeiro checkpoint próprio"
        );
        assert_eq!(
            load(&dir, &old_id),
            Some(old.checkpoint.clone()),
            "antigo intacto"
        );
        fresh.write(&old.checkpoint).unwrap();
        fresh.adopt(old.clone()).unwrap();
        assert_eq!(
            names(&dir),
            instance_files(fresh.id(), &[LOCK, JOURNAL]),
            "só os arquivos da aba nova sobram"
        );
        assert_eq!(load(&dir, fresh.id()), Some(old.checkpoint));
        assert!(abandoned(&dir).is_empty());
        fresh.retire().unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn journal_files_are_private() {
        use std::os::unix::fs::PermissionsExt;
        let dir = temp_dir("private");
        let journal = Journal::create(&dir).unwrap();
        journal.write(&checkpoint(&["segredo"])).unwrap();
        let mode = |path: &Path| std::fs::metadata(path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode(&dir), 0o700, "pasta");
        assert_eq!(mode(&journal.id().file(&dir, LOCK)), 0o600, "lock");
        assert_eq!(mode(&journal.id().file(&dir, JOURNAL)), 0o600, "diário");
        journal.retire().unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn content_hash_matches_fnv1a_vectors() {
        assert_eq!(content_hash(b""), 0xcbf2_9ce4_8422_2325);
        assert_eq!(content_hash(b"a"), 0xaf63_dc4c_8601_ec8c);
        assert_eq!(content_hash(b"foobar"), 0x8594_4171_f739_67e8);
    }

    #[test]
    fn recovery_dir_honors_test_override() {
        let dir = temp_dir("override");
        assert_eq!(with_recovery_dir(dir.clone(), recovery_dir), dir);
        assert_ne!(recovery_dir(), dir, "override some depois do escopo");
    }
}
