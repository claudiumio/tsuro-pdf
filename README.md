<p align="right">🇧🇷 <a href="README.pt-BR.md">Português</a></p>

<p align="center">
  <img src="docs/hero.png" alt="TsuroPDF hero — origami crane over misty mountains" width="100%">
</p>

<h1 align="center">TsuroPDF</h1>

<p align="center">
  <a href="https://github.com/claudiumio/tsuro-pdf/releases"><img src="https://img.shields.io/github/v/release/claudiumio/tsuro-pdf?sort=semver&display_name=release" alt="Latest release"></a>
  <a href="LICENSE"><img src="https://img.shields.io/github/license/claudiumio/tsuro-pdf" alt="License: MIT"></a>
  <img src="https://img.shields.io/badge/macOS-Apple_Silicon-000?logo=apple&logoColor=white" alt="macOS Apple Silicon">
  <img src="https://img.shields.io/badge/Windows-x64-0078D4?logo=windows&logoColor=white" alt="Windows x64">
  <img src="https://img.shields.io/badge/built_with-Rust-CE422B?logo=rust&logoColor=white" alt="Built with Rust">
</p>

<p align="center"><strong>A fast, lightweight, open-source PDF reader for the desktop. Read, mark, print. Nothing else.</strong></p>

TsuroPDF is a native viewer written in Rust ([iced](https://iced.rs/) + [Pdfium](https://pdfium.googlesource.com/pdfium/)). It targets any PDF: large files, demanding typography, digitally signed documents. If a feature does not make reading, marking, or printing faster, lighter, or clearer, it does not belong here. No forms, no cloud, no collaboration, no suite.

## Features

- **Open** from the toolbar, by drag and drop, or from the empty-state browser (folders and recent files)
- **Tabs** — open in a new tab with Ctrl+T, cycle with Ctrl+Tab
- **Pages** — thumbnail panel with document outline tab, previous/next, page N / total counter, go-to-page with keyboard navigation
- **View modes** — single page or continuous scroll
- **Resume reading** — reopens each file where you left off (page + zoom); back/forward history
- **Zoom** — fit to width, fit to page, `+` / `-`
- **Search** the extracted text, with match count and `F3` / `Shift+F3` to step through hits
- **Annotations** — highlight, underline, strikeout and notes anchored to a passage (`H` / `U` / `S` / `N`)
- **Export marks** — copy as Markdown (`M`) or write a Markdown file (`Cmd/Ctrl+E`)
- **Save a copy** with the annotations drawn in (`Cmd/Ctrl+S`)
- **Recovery journal** — unsaved annotations survive a crash and are offered on the next launch
- **Print** — built-in dialog with direct spool (renders a 200 DPI print PDF, no native dialog)
- **Signatures** — on-demand panel with signer and cryptographic status
- **Command palette** — `Cmd/Ctrl+K` for actions and document hits
- **Settings** — theme (system, dark, light), language (Português, English), default reader, macOS permissions
- **In-app update** — checks the latest GitHub Release and installs from inside the app
- **View rotation** — 90° per session, plus a light/dark theme with embedded Inter

Annotations are session state and are not written back to the file you opened. What persists are your reading position, the saved copies you make, and the recovery journal. What stays out, on purpose, is everything else.

## Install

The short way, with [Bun](https://bun.sh/):

```bash
bun run install:tsuro -- --install
```

It queries the latest [GitHub Release](https://github.com/claudiumio/tsuro-pdf/releases), downloads the artifact for your OS, and verifies the SHA-256. Without `--install` it only downloads. `--check` reports whether a newer version exists; `--version v0.6.0` pins a tag. If there is no release for your OS yet, the script prints the clone path and exits with code 1.

The app also checks on launch (Settings → About shows the state) and can install from there: on macOS the bundle is swapped in place and reopened; on Windows the NSIS installer runs silently after the app closes.

Or grab the file from the release page:

- **Mac Apple Silicon** — `TsuroPDF-{version}-aarch64-apple-darwin.dmg`. Open the DMG and drag TsuroPDF to Applications.
- **Windows x64** — `TsuroPDF-{version}-x86_64-pc-windows-msvc-setup.exe`. Installs to `%LOCALAPPDATA%\Programs\TsuroPDF`, no admin needed. `/S` is the silent install.

No builds for Intel Macs or Linux yet.

**First launch on macOS.** Signing is ad-hoc (no Apple Developer Program). Gatekeeper will warn: right-click TsuroPDF → Open.

**First launch on Windows.** If SmartScreen appears: More info → Run anyway.

## Shortcuts

`Cmd` on macOS, `Ctrl` elsewhere unless noted. Plain keys are ignored while a text field has focus.

| Keys | Action |
| --- | --- |
| `Cmd/Ctrl+O` | Open PDF |
| `Cmd/Ctrl+T` / `Cmd/Ctrl+W` | Open in new tab / close tab |
| `Ctrl+Tab` / `Ctrl+Shift+Tab` | Cycle tabs (both platforms; macOS keeps `Cmd+Tab` for the system) |
| `Cmd/Ctrl+P` | Print |
| `Cmd/Ctrl+S` | Save a copy with marks |
| `Cmd/Ctrl+E` | Export annotations as Markdown |
| `Cmd/Ctrl+,` | Settings |
| `Cmd/Ctrl+K` | Command palette |
| `Cmd/Ctrl+F` | Focus search; `Enter` next hit, `Shift+Enter` previous |
| `F3` / `Shift+F3` | Next / previous hit |
| `Cmd/Ctrl+C` | Copy selected text |
| `M` | Copy annotations as Markdown |
| `Cmd/Ctrl+Z` / `Cmd/Ctrl+Shift+Z` | Undo / redo annotation |
| `H` / `U` / `S` / `N` | Highlight / underline / strikeout / note |
| `Delete` | Delete selected annotation |
| `R` | Rotate view 90° |
| `+` / `-` | Zoom in / out |
| `PageUp` / `PageDown`, `←` / `→` | Previous / next page |
| `Home` / `End` | First / last page |
| `↑` / `↓` / `Enter` | Walk the open panel (outline, marks) |
| `Cmd/Ctrl+Enter` | Save the note draft |
| `Esc` | Close the dialog / palette |

Controls also live in the toolbar and the `⋯` menu; the palette lists every action with its shortcut.

## Build from source

```bash
git clone https://github.com/claudiumio/tsuro-pdf.git
cd tsuro-pdf
./scripts/bundle-macos.sh        # macOS .app in dist/
```

```powershell
powershell -File scripts/bundle-windows.ps1   # Windows NSIS installer
```

Developing the viewer needs stable [Rust](https://rustup.rs/) (`rustup default stable`) plus the [Pdfium](https://github.com/bblanchon/pdfium-binaries) library in the project dir or on the system — `bundle-macos.sh` handles that on macOS. Node.js is only needed for the legacy viewer (Tauri + PDF.js), which is not the product.

Run from code:

```bash
cargo test -p tsuro
cargo test -p tsuro-sign
cargo run -p tsuro -- public/samples/guia-folio.pdf
```

With no argument the window opens empty.

## Samples

Three PDFs in `public/samples/`:

| File | What it exercises |
| --- | --- |
| `guia-folio.pdf` | Typography, tables, accents, search |
| `sumario-folio.pdf` | Bookmarks (outline) driving the Sumário tab |
| `contrato-assinado.pdf` | `/Sig` field with a demo self-signed certificate |

The contract certificate is **self-signed**. TsuroPDF treats it as cryptographically intact but with no public trust chain. The expected state is "intact (no public trust)".

## Digital signatures

Detection and verification live in the Rust crate `tsuro-sign`:

- walks AcroForm and `/Sig` dictionaries
- reads PKCS#7/CMS (`adbe.pkcs7.detached` profile)
- checks `ByteRange` and `messageDigest`
- verifies RSA + SHA-256 over the signed attributes

Panel states: valid, intact (no public trust), document modified, invalid, unsupported, certificate expired or not yet valid.

## Project structure

```
crates/tsuro            native viewer (iced + Pdfium) — the product
crates/tsuro-sign       signature engine (PDF + CMS)
public/samples          sample PDFs
docs/hero.png           README hero
src-tauri, src          legacy viewer (Tauri + React + PDF.js)
```

The legacy tree stays in the repo for reference. It is not what ships as TsuroPDF.

```bash
npm install
npm run test:sign
npm run dev          # legacy browser viewer (http://127.0.0.1:43177)
npm run tauri dev
```

## Contributing

Read, verify, and packaging contributions are welcome. TsuroPDF intends to stay small: every proposal is judged against the mission gate (faster, lighter, or clearer reading, marking, or printing). Issues, PRs, and commits in English or PT-BR, PRs and issues with a bilingual body, PRs small and one-topic. See [CONTRIBUTING.md](CONTRIBUTING.md) for the full guide and the order we want contributions in.

## License

[MIT](LICENSE). Copyright (c) 2026 Tsuro contributors.
