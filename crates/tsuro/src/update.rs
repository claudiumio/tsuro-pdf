//! Atualização pelo próprio app a partir das GitHub Releases.
//!
//! Usa os artefatos que a release já publica (DMG, instalador NSIS e
//! `SHA256SUMS`) e as ferramentas do SO (`curl`, `shasum`, `hdiutil`,
//! `ditto`), como `fetch_latest_tag`: nenhuma dependência nova.
//!
//! No macOS o `.app` novo é copiado ao lado do instalado e trocado por
//! `rename` no mesmo volume; o processo em execução segue com o binário
//! antigo mapeado até reiniciar. No Windows o instalador NSIS (`setup.exe`)
//! só corre depois que o app fecha — o `.exe` em execução fica travado —
//! então ele é baixado e validado para `%LOCALAPPDATA%\TsuroPDF\update\` e
//! um helper PowerShell o instala em silêncio ao sair.

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::i18n::Lang;

const REPO: &str = "claudiumio/tsuro-pdf";
#[cfg(target_os = "macos")]
const APP_NAME: &str = "TsuroPDF.app";
#[cfg(target_os = "windows")]
/// Pasta da instalação por usuário, onde o NSIS põe o `.exe`.
const INSTALL_DIR: [&str; 4] = ["LOCALAPPDATA", "Programs", "TsuroPDF", "TsuroPDF.exe"];
#[cfg(target_os = "windows")]
/// `%LOCALAPPDATA%\TsuroPDF\update\`: onde o instalador validado aguarda o app fechar.
const UPDATE_DIR: [&str; 3] = ["LOCALAPPDATA", "TsuroPDF", "update"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateError {
    /// Plataforma sem instalação pelo app (por ora, tudo fora do macOS).
    Unsupported,
    /// O executável não está dentro de um `.app` (ex.: `cargo run`).
    NotInstalled,
    /// Rodando de uma cópia isolada pelo Gatekeeper (App Translocation).
    Translocated,
    Download,
    Checksum,
    /// Montar o DMG ou achar o `.app` dentro dele.
    Unpack,
    /// Copiar ou trocar o `.app` (ex.: pasta sem permissão de escrita).
    Install,
}

impl UpdateError {
    pub fn message(self, lang: Lang) -> &'static str {
        match self {
            UpdateError::Unsupported => lang.pick(
                "Atualização automática ainda não disponível neste sistema.",
                "Automatic update is not available on this system yet.",
            ),
            UpdateError::NotInstalled => lang.pick(
                "Este TsuroPDF não está instalado como app.",
                "This TsuroPDF is not installed as an app.",
            ),
            UpdateError::Translocated => lang.pick(
                "Mova o TsuroPDF para Aplicativos e abra de lá para atualizar.",
                "Move TsuroPDF to Applications and open it from there to update.",
            ),
            UpdateError::Download => lang.pick(
                "Não foi possível baixar a atualização.",
                "Could not download the update.",
            ),
            UpdateError::Checksum => lang.pick(
                "O download veio corrompido; nada foi alterado.",
                "The download was corrupted; nothing was changed.",
            ),
            UpdateError::Unpack => lang.pick(
                "Não foi possível abrir o pacote da atualização.",
                "Could not open the update package.",
            ),
            UpdateError::Install => lang.pick(
                "Não foi possível instalar a atualização.",
                "Could not install the update.",
            ),
        }
    }
}

/// Onde buscar um artefato da release e a soma que o valida.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Source {
    pub asset_name: String,
    pub asset_url: String,
    pub sums_url: String,
}

/// Nome do artefato desta plataforma, como os scripts de bundle o geram.
pub(crate) fn asset_name(version: &str) -> Option<String> {
    let version = version.strip_prefix('v').unwrap_or(version);
    if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        Some(format!("TsuroPDF-{version}-aarch64-apple-darwin.dmg"))
    } else if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
        Some(format!(
            "TsuroPDF-{version}-x86_64-pc-windows-msvc-setup.exe"
        ))
    } else {
        None
    }
}

/// URLs de download da release `tag` (sem consultar a API: os nomes são fixos).
pub(crate) fn release_source(tag: &str) -> Option<Source> {
    let asset_name = asset_name(tag)?;
    let base = format!("https://github.com/{REPO}/releases/download/{tag}");
    Some(Source {
        asset_url: format!("{base}/{asset_name}"),
        sums_url: format!("{base}/SHA256SUMS"),
        asset_name,
    })
}

/// Página da release, para quem não tem instalação pelo app.
pub fn release_page(tag: &str) -> String {
    format!("https://github.com/{REPO}/releases/tag/{tag}")
}

/// Soma de `name` num `SHA256SUMS` (`<hex>  <nome>` ou `<hex> *<nome>`).
pub(crate) fn expected_sha256(sums: &str, name: &str) -> Option<String> {
    sums.lines().find_map(|line| {
        let (hex, file) = line.trim().split_once(char::is_whitespace)?;
        let file = file.trim_start().trim_start_matches('*');
        (file == name && hex.len() == 64 && hex.bytes().all(|b| b.is_ascii_hexdigit()))
            .then(|| hex.to_ascii_lowercase())
    })
}

/// O `.app` que contém o executável, recusando a cópia translocada.
pub(crate) fn installed_bundle(exe: &Path) -> Result<PathBuf, UpdateError> {
    if exe
        .components()
        .any(|c| c.as_os_str() == "AppTranslocation")
    {
        return Err(UpdateError::Translocated);
    }
    exe.ancestors()
        .find(|p| p.extension().is_some_and(|ext| ext == "app"))
        .map(Path::to_path_buf)
        .ok_or(UpdateError::NotInstalled)
}

/// Caminho do `.exe` dentro de uma pasta de instalação por usuário.
#[cfg(target_os = "windows")]
fn installed_exe(dir: &Path) -> PathBuf {
    dir.join("TsuroPDF.exe")
}

/// `.exe` da instalação por usuário (`%LOCALAPPDATA%\Programs\TsuroPDF`):
/// o que o helper reabre depois de instalar.
#[cfg(target_os = "windows")]
pub(crate) fn installed_exe_path() -> Option<PathBuf> {
    windows_path(&INSTALL_DIR).map(|dir| installed_exe(&dir))
}

/// Caminho do instalador já validado, onde o helper PowerShell o busca.
#[cfg(target_os = "windows")]
fn staged_installer(update_dir: &Path, asset_name: &str) -> PathBuf {
    update_dir.join(asset_name)
}

/// O executável está dentro de uma pasta de instalação por usuário?
/// (Recusa `cargo run` e cópias soltas — o instalador as ignoraria.)
#[cfg(target_os = "windows")]
fn is_installed_exe(exe: &Path, install_dir: &Path) -> bool {
    exe == installed_exe(install_dir)
}

/// Aceita só o instalador do TsuroPDF com nome de asset exato.
/// (Impede o helper rodar um `.exe` qualquer plantado na pasta de update.)
#[cfg(target_os = "windows")]
fn is_safe_asset_name(name: &str) -> bool {
    let Some(rest) = name.strip_prefix("TsuroPDF-") else {
        return false;
    };
    let Some(version) = rest.strip_suffix("-x86_64-pc-windows-msvc-setup.exe") else {
        return false;
    };
    !version.is_empty()
        && version
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'+'))
}

/// Espera este processo sair, roda o instalador em silêncio e reabre o app.
/// PowerShell por ser o que já vem no Windows e conseguir esperar o PID.
#[cfg(target_os = "windows")]
fn powershell_script(pid: u32, installer: &Path, exe: &Path) -> String {
    format!(
        "$ErrorActionPreference='Stop'; \
         $ok=$false; \
         try {{ \
           Wait-Process -Id {pid} -ErrorAction SilentlyContinue; \
           Start-Process -FilePath {installer} -ArgumentList '/S' -Wait; \
           if (Test-Path -LiteralPath {exe}) {{ \
             Start-Process -FilePath {exe}; \
             $ok=$true \
           }} \
         }} catch {{}}; \
         if (-not $ok) {{ exit 1 }}",
        pid = pid,
        installer = ps_quote(installer),
        exe = ps_quote(exe),
    )
}

/// Literal PowerShell entre aspas simples; `'` interno dobra (`''`).
#[cfg(target_os = "windows")]
fn ps_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "''"))
}

/// Argumentos do helper: só o script, que já leva os caminhos embutidos.
#[cfg(target_os = "windows")]
fn powershell_args(script: &str) -> Vec<String> {
    vec![
        "-NoProfile".to_string(),
        "-WindowStyle".to_string(),
        "Hidden".to_string(),
        "-Command".to_string(),
        script.to_string(),
    ]
}

/// `CFBundleShortVersionString` de um `Info.plist` XML.
pub(crate) fn plist_version(xml: &str) -> Option<&str> {
    let rest = xml.split_once("<key>CFBundleShortVersionString</key>")?.1;
    let rest = rest.split_once("<string>")?.1;
    Some(rest.split_once("</string>")?.0.trim())
}

/// Baixa, valida e instala a release `tag` no lugar do app em execução.
/// Bloqueante: o chamador roda em `spawn_blocking`. No macOS devolve o `.app`
/// novo; no Windows devolve o instalador validado, que só roda ao sair.
pub(crate) fn install_release(tag: &str) -> Result<PathBuf, UpdateError> {
    #[cfg(target_os = "windows")]
    {
        install_release_windows(tag)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let source = release_source(tag).ok_or(UpdateError::Unsupported)?;
        if !cfg!(target_os = "macos") {
            return Err(UpdateError::Unsupported);
        }
        let exe = std::env::current_exe().map_err(|_| UpdateError::NotInstalled)?;
        let bundle = installed_bundle(&exe)?;
        install_from(&source, tag, &bundle)?;
        Ok(bundle)
    }
}

/// Windows: baixa `SHA256SUMS` + `setup.exe` para `%LOCALAPPDATA%\TsuroPDF\update\`,
/// confere a soma e devolve o instalador. O `.exe` em execução fica travado,
/// então a instalação em si fica para `relaunch_if_armed`.
#[cfg(target_os = "windows")]
fn install_release_windows(tag: &str) -> Result<PathBuf, UpdateError> {
    let source = release_source(tag).ok_or(UpdateError::Unsupported)?;
    if !is_safe_asset_name(&source.asset_name) {
        return Err(UpdateError::Unsupported);
    }
    let exe = std::env::current_exe().map_err(|_| UpdateError::NotInstalled)?;
    let install_dir = windows_path(&INSTALL_DIR).ok_or(UpdateError::Install)?;
    if !is_installed_exe(&exe, &install_dir) {
        return Err(UpdateError::NotInstalled);
    }
    let update_dir = windows_path(&UPDATE_DIR).ok_or(UpdateError::Install)?;
    let installer = staged_installer(&update_dir, &source.asset_name);
    std::fs::create_dir_all(&update_dir).map_err(|_| UpdateError::Download)?;

    let sums = curl_text(&source.sums_url)?;
    let expected = expected_sha256(&sums, &source.asset_name).ok_or(UpdateError::Checksum)?;
    let _ = std::fs::remove_file(&installer);
    curl_file(&source.asset_url, &installer)?;
    if sha256_of(&installer)? != expected {
        let _ = std::fs::remove_file(&installer);
        return Err(UpdateError::Checksum);
    }
    Ok(installer)
}

/// Junta variáveis de ambiente do Windows (`LOCALAPPDATA`, …) num caminho.
#[cfg(target_os = "windows")]
fn windows_path(segments: &[&str]) -> Option<PathBuf> {
    let mut path = PathBuf::from(std::env::var_os(segments[0])?);
    path.extend(&segments[1..]);
    Some(path)
}

/// Hash SHA-256 de um arquivo. PowerShell primeiro (vem em todo Windows);
/// `certutil` é o plano B das edições sem ele.
#[cfg(target_os = "windows")]
fn sha256_of(path: &Path) -> Result<String, UpdateError> {
    let hex = run_capture(
        "powershell",
        &[
            "-NoProfile".as_ref(),
            "-Command".as_ref(),
            format!(
                "(Get-FileHash -Algorithm SHA256 -LiteralPath {}).Hash",
                ps_quote(&path.to_string_lossy())
            )
            .as_ref(),
        ],
    )
    .or_else(|_| {
        run_capture(
            "certutil",
            &["-hashfile".as_ref(), path.as_os_str(), "SHA256".as_ref()],
        )
        .map(|out| {
            out.split_whitespace()
                .nth(1)
                .unwrap_or_default()
                .to_string()
        })
    })?;
    let hex = hex.trim().to_ascii_lowercase();
    (hex.len() == 64 && hex.bytes().all(|b| b.is_ascii_hexdigit()))
        .then_some(hex)
        .ok_or(UpdateError::Checksum)
}

/// Roda um comando e devolve o stdout cortado (para saídas curtas: hash).
#[cfg(target_os = "windows")]
fn run_capture(program: &str, args: &[&std::ffi::OsStr]) -> Result<String, UpdateError> {
    let out = std::process::Command::new(program)
        .args(args)
        .output()
        .map_err(|_| UpdateError::Checksum)?;
    if !out.status.success() {
        return Err(UpdateError::Checksum);
    }
    String::from_utf8(out.stdout).map_err(|_| UpdateError::Checksum)
}

#[cfg(target_os = "macos")]
pub(crate) fn install_from(source: &Source, tag: &str, bundle: &Path) -> Result<(), UpdateError> {
    let parent = bundle.parent().ok_or(UpdateError::Install)?;
    let work = TempDir::new()?;
    let asset = work.path().join(&source.asset_name);

    let sums = curl_text(&source.sums_url)?;
    let expected = expected_sha256(&sums, &source.asset_name).ok_or(UpdateError::Checksum)?;
    curl_file(&source.asset_url, &asset)?;
    if sha256_of(&asset)? != expected {
        return Err(UpdateError::Checksum);
    }

    let staged = parent.join(".TsuroPDF.app.update");
    let _ = std::fs::remove_dir_all(&staged);
    {
        let mount = work.path().join("mnt");
        let _volume = Mounted::attach(&asset, &mount)?;
        let app = mount.join(APP_NAME);
        if !app.is_dir() {
            return Err(UpdateError::Unpack);
        }
        run(
            "ditto",
            &[app.as_os_str(), staged.as_os_str()],
            UpdateError::Install,
        )?;
    }
    let version = std::fs::read_to_string(staged.join("Contents/Info.plist"))
        .ok()
        .and_then(|xml| plist_version(&xml).map(str::to_owned));
    if version.as_deref() != Some(tag.strip_prefix('v').unwrap_or(tag)) {
        let _ = std::fs::remove_dir_all(&staged);
        return Err(UpdateError::Unpack);
    }
    swap(bundle, &staged)
}

#[cfg(not(target_os = "macos"))]
pub(crate) fn install_from(_: &Source, _: &str, _: &Path) -> Result<(), UpdateError> {
    Err(UpdateError::Unsupported)
}

/// Troca `bundle` por `staged` no mesmo volume; desfaz se o segundo passo falhar.
#[cfg(target_os = "macos")]
fn swap(bundle: &Path, staged: &Path) -> Result<(), UpdateError> {
    let old = bundle.with_file_name(".TsuroPDF.app.old");
    let _ = std::fs::remove_dir_all(&old);
    std::fs::rename(bundle, &old).map_err(|_| UpdateError::Install)?;
    if std::fs::rename(staged, bundle).is_err() {
        let _ = std::fs::rename(&old, bundle);
        let _ = std::fs::remove_dir_all(staged);
        return Err(UpdateError::Install);
    }
    let _ = std::fs::remove_dir_all(&old);
    Ok(())
}

fn curl_text(url: &str) -> Result<String, UpdateError> {
    let out = std::process::Command::new("curl")
        .args(["-fsSL", "--max-time", "30", url])
        .output()
        .map_err(|_| UpdateError::Download)?;
    if !out.status.success() {
        return Err(UpdateError::Download);
    }
    String::from_utf8(out.stdout).map_err(|_| UpdateError::Download)
}

fn curl_file(url: &str, dest: &Path) -> Result<(), UpdateError> {
    run(
        "curl",
        &[
            "-fsSL".as_ref(),
            "--max-time".as_ref(),
            "900".as_ref(),
            "-o".as_ref(),
            dest.as_os_str(),
            url.as_ref(),
        ],
        UpdateError::Download,
    )
}

#[cfg(target_os = "macos")]
fn sha256_of(path: &Path) -> Result<String, UpdateError> {
    let out = std::process::Command::new("shasum")
        .args(["-a".as_ref(), "256".as_ref(), path.as_os_str()])
        .output()
        .map_err(|_| UpdateError::Checksum)?;
    let text = String::from_utf8_lossy(&out.stdout);
    match text.split_whitespace().next() {
        Some(hex) if out.status.success() => Ok(hex.to_ascii_lowercase()),
        _ => Err(UpdateError::Checksum),
    }
}

fn run(program: &str, args: &[&std::ffi::OsStr], error: UpdateError) -> Result<(), UpdateError> {
    let status = std::process::Command::new(program)
        .args(args)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|_| error)?;
    status.success().then_some(()).ok_or(error)
}

/// DMG montado só para leitura; desmonta ao sair do escopo.
#[cfg(target_os = "macos")]
struct Mounted(PathBuf);

#[cfg(target_os = "macos")]
impl Mounted {
    fn attach(dmg: &Path, mount: &Path) -> Result<Self, UpdateError> {
        std::fs::create_dir_all(mount).map_err(|_| UpdateError::Unpack)?;
        run(
            "hdiutil",
            &[
                "attach".as_ref(),
                "-nobrowse".as_ref(),
                "-readonly".as_ref(),
                "-noautoopen".as_ref(),
                "-mountpoint".as_ref(),
                mount.as_os_str(),
                dmg.as_os_str(),
            ],
            UpdateError::Unpack,
        )?;
        Ok(Self(mount.to_path_buf()))
    }
}

#[cfg(target_os = "macos")]
impl Drop for Mounted {
    fn drop(&mut self) {
        let _ = run(
            "hdiutil",
            &["detach".as_ref(), "-force".as_ref(), self.0.as_os_str()],
            UpdateError::Unpack,
        );
    }
}

#[cfg(target_os = "macos")]
struct TempDir(PathBuf);

#[cfg(target_os = "macos")]
impl TempDir {
    fn new() -> Result<Self, UpdateError> {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or_default();
        let dir = std::env::temp_dir().join(format!("tsuro-update-{}-{stamp}", std::process::id()));
        std::fs::create_dir_all(&dir).map_err(|_| UpdateError::Download)?;
        Ok(Self(dir))
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

#[cfg(target_os = "macos")]
impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// O que rodar quando a janela fechar: no macOS o `.app` novo; no Windows o
/// instalador validado e o `.exe` a reabrir depois.
pub(crate) enum Pending {
    #[cfg(target_os = "macos")]
    Bundle(PathBuf),
    #[cfg(target_os = "windows")]
    Installer { installer: PathBuf, exe: PathBuf },
}

/// App instalado que reabre quando esta janela fechar (Reiniciar).
static RELAUNCH: Mutex<Option<Pending>> = Mutex::new(None);

pub(crate) fn arm_relaunch(pending: Pending) {
    *RELAUNCH.lock().unwrap_or_else(|e| e.into_inner()) = Some(pending);
}

pub(crate) fn disarm_relaunch() {
    RELAUNCH.lock().unwrap_or_else(|e| e.into_inner()).take();
}

/// Chamado no fechamento da janela: se armado, um helper espera este processo
/// terminar e conclui a atualização — `open` no macOS; no Windows o instalador
/// `/S` (o `.exe` em execução ficaria travado antes disso).
pub(crate) fn relaunch_if_armed() {
    let Some(pending) = RELAUNCH.lock().unwrap_or_else(|e| e.into_inner()).take() else {
        return;
    };
    match pending {
        #[cfg(target_os = "macos")]
        Pending::Bundle(bundle) => relaunch_bundle(&bundle),
        #[cfg(target_os = "windows")]
        Pending::Installer { installer, exe } => spawn_install_helper(installer, exe),
    }
}

#[cfg(target_os = "macos")]
fn relaunch_bundle(bundle: &Path) {
    let _ = std::process::Command::new("/bin/sh")
        .args([
            "-c".as_ref(),
            "while kill -0 \"$1\" 2>/dev/null; do sleep 0.2; done; open \"$2\"".as_ref(),
            "tsuro-relaunch".as_ref(),
            std::process::id().to_string().as_ref(),
            bundle.as_os_str(),
        ])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
}

/// Helper destacado e invisível (nenhuma piscada de console): espera o PID do
/// app sair, instala em silêncio e reabre o `.exe` novo.
#[cfg(target_os = "windows")]
fn spawn_install_helper(installer: PathBuf, exe: PathBuf) {
    use std::os::windows::process::CommandExt;
    if !installer
        .file_name()
        .and_then(|n| n.to_str())
        .is_some_and(is_safe_asset_name)
    {
        return; // nome inesperado na pasta de update: não roda
    }
    let script = powershell_script(std::process::id(), &installer, &exe);
    let _ = std::process::Command::new("powershell")
        .args(powershell_args(&script))
        .creation_flags(0x0800_0000) // CREATE_NO_WINDOW
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_sha256_finds_the_asset_line() {
        let hex = "a".repeat(64);
        let other = "b".repeat(64);
        let sums = format!(
            "{other}  TsuroPDF-0.5.5-x86_64-pc-windows-msvc-setup.exe\n{hex}  TsuroPDF-0.5.5-aarch64-apple-darwin.dmg\n"
        );
        assert_eq!(
            expected_sha256(&sums, "TsuroPDF-0.5.5-aarch64-apple-darwin.dmg"),
            Some(hex.clone())
        );
        assert_eq!(
            expected_sha256(&format!("{}  x.dmg", hex.to_uppercase()), "x.dmg"),
            Some(hex.clone()),
            "hex maiúsculo normaliza"
        );
        assert_eq!(
            expected_sha256(&format!("{hex} *x.dmg"), "x.dmg"),
            Some(hex)
        );
        assert_eq!(expected_sha256("zz  x.dmg", "x.dmg"), None, "hex inválido");
        assert_eq!(expected_sha256(&sums, "outro.dmg"), None);
    }

    #[test]
    fn release_source_matches_bundle_script_names() {
        let Some(source) = release_source("v0.5.5") else {
            return;
        };
        if cfg!(target_os = "macos") {
            assert_eq!(source.asset_name, "TsuroPDF-0.5.5-aarch64-apple-darwin.dmg");
        }
        assert_eq!(
            source.asset_url,
            format!(
                "https://github.com/claudiumio/tsuro-pdf/releases/download/v0.5.5/{}",
                source.asset_name
            )
        );
        assert_eq!(
            source.sums_url,
            "https://github.com/claudiumio/tsuro-pdf/releases/download/v0.5.5/SHA256SUMS"
        );
    }

    #[test]
    fn installed_bundle_finds_app_and_refuses_translocation() {
        assert_eq!(
            installed_bundle(Path::new(
                "/Applications/TsuroPDF.app/Contents/MacOS/TsuroPDF"
            )),
            Ok(PathBuf::from("/Applications/TsuroPDF.app"))
        );
        assert_eq!(
            installed_bundle(Path::new("/Users/x/repo/target/release/TsuroPDF")),
            Err(UpdateError::NotInstalled)
        );
        assert_eq!(
            installed_bundle(Path::new(
                "/private/var/folders/ab/AppTranslocation/1234/d/TsuroPDF.app/Contents/MacOS/TsuroPDF"
            )),
            Err(UpdateError::Translocated)
        );
    }

    #[test]
    fn plist_version_reads_short_version() {
        let xml = "<dict><key>CFBundleVersion</key><string>9</string>\
                   <key>CFBundleShortVersionString</key>\n  <string>0.5.5</string></dict>";
        assert_eq!(plist_version(xml), Some("0.5.5"));
        assert_eq!(plist_version("<dict></dict>"), None);
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn is_installed_exe_accepts_only_the_per_user_install() {
        let dir = Path::new(r"C:\Users\x\AppData\Local\Programs\TsuroPDF");
        assert!(is_installed_exe(&dir.join("TsuroPDF.exe"), dir,));
        assert!(
            !is_installed_exe(Path::new(r"C:\Users\x\repo\target\debug\TsuroPDF.exe"), dir,),
            "cargo run não conta como instalado"
        );
        assert!(
            !is_installed_exe(&dir.join("tsuro.exe"), dir),
            "nome trocado não conta"
        );
        // Mesmo nome em outra pasta: instalador por usuário de outro local.
        assert!(
            !is_installed_exe(Path::new(r"C:\Program Files\TsuroPDF\TsuroPDF.exe"), dir,),
            "só a pasta por usuário do NSIS vale"
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn staged_installer_lives_under_the_update_dir() {
        let update = Path::new(r"C:\Users\x\AppData\Local\TsuroPDF\update");
        assert_eq!(
            staged_installer(update, "TsuroPDF-9.9.9-x86_64-pc-windows-msvc-setup.exe"),
            update.join("TsuroPDF-9.9.9-x86_64-pc-windows-msvc-setup.exe")
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn is_safe_asset_name_refuses_anything_else() {
        assert!(is_safe_asset_name(
            "TsuroPDF-9.9.9-x86_64-pc-windows-msvc-setup.exe"
        ));
        assert!(
            !is_safe_asset_name("malware.exe"),
            "o helper não roda .exe plantado"
        );
        assert!(
            !is_safe_asset_name("TsuroPDF-x86_64-pc-windows-msvc-setup.exe"),
            "sem versão, não"
        );
        assert!(
            !is_safe_asset_name("TsuroPDF-9.9.9;calc-x86_64-pc-windows-msvc-setup.exe"),
            "caractere estranho no nome"
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn ps_quote_escapes_single_quotes_for_powershell_literals() {
        assert_eq!(ps_quote(r"C:\update"), r"'C:\update'");
        assert_eq!(ps_quote(r"C:\it's"), r"'C:\it''s'");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn powershell_script_waits_installs_and_reopens() {
        let script = powershell_script(
            4242,
            Path::new(
                r"C:\Users\x\AppData\Local\TsuroPDF\update\TsuroPDF-9.9.9-x86_64-pc-windows-msvc-setup.exe",
            ),
            Path::new(r"C:\Users\x\AppData\Local\Programs\TsuroPDF\TsuroPDF.exe"),
        );
        assert!(script.contains("Wait-Process -Id 4242"), "{script}");
        assert!(script.contains("-ArgumentList '/S'"), "{script}");
        assert!(
            script.contains(
                r"-FilePath 'C:\Users\x\AppData\Local\TsuroPDF\update\TsuroPDF-9.9.9-x86_64-pc-windows-msvc-setup.exe'"
            ),
            "instalador entre aspas simples"
        );
        assert!(
            script.contains(r"-FilePath 'C:\Users\x\AppData\Local\Programs\TsuroPDF\TsuroPDF.exe'"),
            "reabre o .exe instalado"
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn powershell_args_hide_the_window() {
        let args = powershell_args("script");
        assert_eq!(
            args,
            vec!["-NoProfile", "-WindowStyle", "Hidden", "-Command", "script"]
        );
    }

    /// Release falsa servida por `file://`: DMG real com um `.app` mínimo.
    #[cfg(target_os = "macos")]
    struct FakeRelease {
        root: PathBuf,
        source: Source,
    }

    #[cfg(target_os = "macos")]
    impl FakeRelease {
        fn new(version: &str, corrupt_sum: bool) -> Self {
            let root = std::env::temp_dir().join(format!(
                "tsuro-update-test-{}-{version}-{corrupt_sum}",
                std::process::id()
            ));
            let _ = std::fs::remove_dir_all(&root);
            let stage = root.join("stage");
            fake_app(&stage.join(APP_NAME), version, "nova");
            let name = format!("TsuroPDF-{version}-aarch64-apple-darwin.dmg");
            let dmg = root.join(&name);
            let status = std::process::Command::new("hdiutil")
                .args([
                    "create",
                    "-volname",
                    "TsuroPDF",
                    "-format",
                    "UDZO",
                    "-srcfolder",
                ])
                .arg(&stage)
                .arg(&dmg)
                .stdout(std::process::Stdio::null())
                .status()
                .expect("hdiutil create");
            assert!(status.success(), "hdiutil create falhou");
            let sum = if corrupt_sum {
                "0".repeat(64)
            } else {
                sha256_of(&dmg).expect("shasum")
            };
            std::fs::write(root.join("SHA256SUMS"), format!("{sum}  {name}\n")).unwrap();
            let source = Source {
                asset_url: format!("file://{}", dmg.display()),
                sums_url: format!("file://{}", root.join("SHA256SUMS").display()),
                asset_name: name,
            };
            Self { root, source }
        }
    }

    #[cfg(target_os = "macos")]
    impl Drop for FakeRelease {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    #[cfg(target_os = "macos")]
    fn fake_app(app: &Path, version: &str, marker: &str) {
        std::fs::create_dir_all(app.join("Contents/MacOS")).unwrap();
        std::fs::write(
            app.join("Contents/Info.plist"),
            format!(
                "<plist><dict><key>CFBundleShortVersionString</key><string>{version}</string></dict></plist>"
            ),
        )
        .unwrap();
        std::fs::write(app.join("Contents/MacOS/TsuroPDF"), marker).unwrap();
    }

    #[cfg(target_os = "macos")]
    fn installed_marker(bundle: &Path) -> String {
        std::fs::read_to_string(bundle.join("Contents/MacOS/TsuroPDF")).unwrap()
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn install_from_replaces_the_bundle_with_the_release_app() {
        let release = FakeRelease::new("9.9.9", false);
        let bundle = release.root.join("Applications").join(APP_NAME);
        fake_app(&bundle, "0.5.0", "antiga");

        install_from(&release.source, "v9.9.9", &bundle).expect("instala a release");

        assert_eq!(installed_marker(&bundle), "nova");
        let plist = std::fs::read_to_string(bundle.join("Contents/Info.plist")).unwrap();
        assert_eq!(plist_version(&plist), Some("9.9.9"));
        let leftovers: Vec<_> = std::fs::read_dir(bundle.parent().unwrap())
            .unwrap()
            .map(|e| e.unwrap().file_name())
            .collect();
        assert_eq!(
            leftovers,
            vec![std::ffi::OsString::from(APP_NAME)],
            "sem .update/.old"
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn install_from_keeps_the_bundle_when_the_checksum_fails() {
        let release = FakeRelease::new("9.9.8", true);
        let bundle = release.root.join("Applications").join(APP_NAME);
        fake_app(&bundle, "0.5.0", "antiga");

        assert_eq!(
            install_from(&release.source, "v9.9.8", &bundle),
            Err(UpdateError::Checksum)
        );
        assert_eq!(installed_marker(&bundle), "antiga");
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn install_from_refuses_a_dmg_with_another_version() {
        let release = FakeRelease::new("9.9.7", false);
        let bundle = release.root.join("Applications").join(APP_NAME);
        fake_app(&bundle, "0.5.0", "antiga");

        assert_eq!(
            install_from(&release.source, "v9.9.6", &bundle),
            Err(UpdateError::Unpack)
        );
        assert_eq!(installed_marker(&bundle), "antiga");
    }
}
