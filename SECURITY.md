# Security Policy

## Supported Versions

Only the latest release receives security updates. Older versions are not supported.

## Reporting a Vulnerability

Report security issues privately to **claudiumio@proton.me**. Do not open a public issue for vulnerabilities.

Include:

- Steps to reproduce
- Expected vs actual behavior
- Affected version(s)
- Any relevant PDF samples (redact sensitive content)

We aim to acknowledge reports within 48 hours and resolve confirmed issues before the next release.

## Threat Model

Tsuro PDF is a local desktop application. The threat model assumes:

- **In scope:** malicious or malformed PDF files, path traversal via opened files, unsafe FFI at platform boundaries, dependency supply chain.
- **Out of scope:** network attacks against the update checker (the app uses `curl` to check GitHub for updates and downloads release assets during installation), privilege escalation beyond the user's own account, physical access to the machine.

### What we defend against

- **Malformed PDF input.** The Pdfium engine parses untrusted PDF bytes. Signature parsing (`tsuro-sign`) validates CMS structures defensively and rejects out-of-bounds reads.
- **File-type filtering.** File paths from Apple Events (macOS) are accepted only when the extension is `.pdf`; this filters by type but does not restrict which directories can be opened. Recents and preferences files live under platform-standard directories (`~/Library/Application Support/Tsuro`, `$XDG_DATA_HOME/tsuro`).
- **Unsafe FFI.** All `unsafe` blocks are confined to platform-specific FFI boundaries (ObjC `msg_send!`, Windows Win32, Pdfium C API). Return types are explicitly annotated to prevent type confusion.
- **Dependency vulnerabilities.** `cargo audit` runs on every PR via CI. Cryptographic primitives come from well-audited crates (`rsa`, `sha2`, `x509-parser`); no hand-rolled crypto.

### What we do not defend against

- Physical access to the machine or unlocked session.
- Compromised OS or kernel-level attacks.
- Malicious PDFium shared libraries substituted after installation.

## Security Hygiene

- No secrets, tokens, or credentials in source code or git history.
- `.gitignore` excludes `libpdfium.*`, `pdfium.dll`, `*.local`, and `target/`.
- GitHub Actions workflows pin third-party actions by commit SHA.
- Recovery journal files are created with `0o600` permissions on Unix. On Windows, they inherit the folder's access rules.
- Update installer asset names are validated before execution. Commands generally receive separate arguments; the Windows update helper embeds paths in PowerShell command text using `ps_quote` to escape them.