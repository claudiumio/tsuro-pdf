# Installer and updater research (Tsuro, 2026-09-07)

Pergunta: qual stack de instalador + atualizador é sólida e consistente para o Tsuro?

Restrições: visor nativo iced + Pdfium (`crates/tsuro`, MIT, `dev.tsuro.reader`, 0.1.0). Sem Tauri, Electron, loja ou nuvem. Pdfium é dylib/dll por plataforma — o instalador TEM de empacotar e manter codesign/rpath válidos. O ship atual é `scripts/bundle-macos.sh`: monta `dist/Tsuro.app`, copia `libpdfium.dylib` para `Contents/Frameworks`, assina ad-hoc (`codesign -s - --force --deep`). O engine procura a lib em `Contents/Frameworks` (`crates/tsuro/src/engine.rs`). Alvo de distribuição: macOS + Windows (Linux é extra). Certificado Apple Developer / Authenticode: desconhecido — avaliar os dois caminhos.

O usuário precisa repetir a mesma história nos dois SOs: instalar uma vez, depois atualizar. Onde o SO impede (notação Apple vs SmartScreen), dizer. Não tratar “zipar o `.app`” como instalador de longo prazo.

## Comparison

| Candidato | Instala GUI nativa | Atualiza in-app | Assina payload | Pdfium no bundle | Força loja/webview | Licença | Custo operacional |
|---|---|---|---|---|---|---|---|
| `bundle-macos.sh` de hoje | Só Mac, arrastar `.app` | Não | Ad-hoc, só teste local | Sim (`Frameworks`) | Não | MIT (nosso) | Zero, mas Gatekeeper bloqueia download |
| Developer ID + `notarytool` + DMG | Mac: DMG → `/Applications` | Não (só o envelope) | Apple + ticket stapled | Sim, se assinar a dylib | Não | N/A (processo Apple) | US$ 99/ano |
| cargo-packager + packager-updater | `.app`/DMG + NSIS/MSI | Sim, minisign, um JSON | minisign + codesign/Authenticode opcionais | `resources` mapeável p/ `Frameworks` | Não (preview, não é Tauri) | Apache-2.0 OR MIT | Apple US$ 99 + Authenticode ~US$ 10/mês |
| cargo-dist + axoupdater | CLI (`curl\|sh`, MSI em Program Files) | Sidecar `tsuro-update`, não o `.app` | checksum; signing Windows via ssl.com/Azure | `include` na raiz do archive, sem `.app` | Não | MIT OR Apache-2.0 | CI GitHub; MSI pede admin |
| Sparkle + WinSparkle | Não instala; atualiza o que já está | Sim, appcast + EdDSA | EdDSA + Developer ID / Authenticode | Preserva o `.app` se o archive vier certo | Não | MIT (ambos) | Duas FFI a partir do iced |
| `self_update` 1.3 | Não instala | Sim, GitHub `latest` + `.app` inteiro | zipsign + digest GitHub | Troca o bundle inteiro | Não | MIT | Só CI + feed |
| MSIX + App Installer | Windows only | Sim, `.appinstaller` | Authenticode obrigatório fora da Store | Precisa ir no pacote | Não, mas filosofia Windows | N/A | Cert + hosting HTTPS |
| Mac App Store / Microsoft Store | Sim | Sim (a loja) | A loja re-assina | Sandbox/certificação | Sim: conta, review, listagem | N/A | Apple US$ 99; Store individual grátis, ainda exige Partner Center |
| cargo-bundle | `.app` + MSI experimental | Não | Não documenta notação | `osx_frameworks` → `Frameworks` | Não | MIT OR Apache-2.0 | README ainda se declara alpha |
| Homebrew Cask | Canal `brew install --cask` | `brew upgrade` | Gatekeeper do artefato upstream | Se o DMG já trouxer | Não | Canal, não produto | Manutenção do cask |

## Notes per candidate

**Developer ID + `notarytool` + DMG (keep, caminho v0.1).** Fora da Mac App Store, o Gatekeeper só confia em software assinado com certificado Developer ID; a notação é o scan automático da Apple, distinto do App Review. A partir do macOS 10.15, software Developer ID construído depois de 1 jun 2019 TEM de ser notariado. A notação exige: assinatura válida em todo executável (incluindo `libpdfium.dylib`), certificado Developer ID (não ad-hoc, não “Apple Developer”, não Mac Distribution), Hardened Runtime, timestamp seguro, sem `com.apple.security.get-task-allow=true`, SDK ≥ 10.9. `altool` deixou de ser aceito em 1 nov 2023; o fluxo é `xcrun notarytool submit` + `xcrun stapler staple`. ZIP pode ser notariado mas não stapled — para o usuário offline, staple no `.app` ou no DMG. DMG (UDIF) e PKG flat são recipientes oficiais; o DMG com symlink `/Applications` é o envelope que a Apple e o Sparkle recomendam para site. Sem o programa (US$ 99/ano) não há certificado Developer ID nem notação. Ad-hoc (`codesign -s -`) serve só para o teste local que o script já faz: a notação recusa, e um `.app` baixado da internet fica no Gatekeeper.
Sources: https://developer.apple.com/developer-id/, https://developer.apple.com/documentation/security/notarizing-macos-software-before-distribution, https://developer.apple.com/documentation/security/customizing-the-notarization-workflow, https://developer.apple.com/programs/, `scripts/bundle-macos.sh`

**cargo-packager + cargo-packager-updater (keep, stack que continua no Windows).** Em 2026 o tool antigo “tauri-bundler disfarçado” não é o que existe: `cargo-packager` 0.11.8 (crates.io, 27 nov 2025) empacota executável nativo em `.app`, `.dmg`, NSIS `.exe` e WiX `.msi`, sem webview. `MacOsConfig` tem `signing_identity`, `notarization_credentials`, `entitlements` e `frameworks`. `Resource::Mapped { src, target }` deixa pousar `libpdfium.dylib` em `Contents/Frameworks` — o caminho que `engine.rs` já resolve. NSIS oferece `CurrentUser` / `PerMachine` / `Both`; `CurrentUser` evita UAC na primeira instalação e no update. O updater (`cargo-packager-updater` 0.2.3, 21 jul 2025) verifica um endpoint HTTPS, exige `pubkey` minisign, e instala formatos `app` / `nsis` / `wix` / `appimage`. No Windows o modo padrão é `passive` (barra, sem clique). Licença Apache-2.0 OR MIT, compatível. Não força Tauri: a doc lista Tauri como um consumidor, não como requisito. Ressalva: o packager ainda se declara “public preview”; o último release do CLI é nov 2025 e o do updater jul 2025 — vivos, mas mais lentos que `self_update`/`cargo-dist`. Se o preview estancar, o instalador permanece e o updater troca (ver `self_update` abaixo).
Sources: https://crates.io/crates/cargo-packager, https://crates.io/crates/cargo-packager-updater, https://docs.crabnebula.dev/packager/, https://docs.crabnebula.dev/packager/updater/, https://docs.rs/cargo-packager/latest/cargo_packager/config/struct.MacOsConfig.html, https://docs.rs/cargo-packager/latest/cargo_packager/config/enum.Resource.html, https://docs.rs/cargo-packager/latest/cargo_packager/config/enum.NSISInstallerMode.html

**cargo-dist + axoupdater (watch, não é o instalador do produto).** `cargo-dist` 0.32.0 (mai 2026) gera CI, GitHub Release, tarball/zip, `shell`/`powershell`, Homebrew, npm e MSI. Não gera `.app` nem DMG. `include` e `package-libraries` (experimental) metem dylibs na raiz do archive — o engine do Tsuro não procura aí. O updater (`install-updater = true`) instala um binário `tsuro-update` ao lado do CLI em `CARGO_HOME/bin` ou `~/.app/bin`; a doc chama o feature de experimental e o MSI default vai para `%ProgramFiles%\{app}` (admin). Assinatura Windows existe (`ssldotcom-windows-sign`, `azure-windows-sign` desde 0.32.0), só x86_64. Excelente para CLI e para um cask Homebrew derivado; péssimo como história de um leitor GUI. `axoupdater` 0.10.2 (12 ago 2026) lê o *install receipt* que o dist grava em `~/.config/APP` — inútil se o usuário arrastou um `.app`.
Sources: https://axodotdev.github.io/cargo-dist/book/introduction.html, https://axodotdev.github.io/cargo-dist/book/reference/config.html, https://axodotdev.github.io/cargo-dist/book/installers/msi.html, https://axodotdev.github.io/cargo-dist/book/installers/updater.html, https://crates.io/crates/axoupdater

**Sparkle (watch no Mac, não sozinho).** Framework MIT de update para `.app`: appcast RSS, EdDSA (ed25519) na archive, HTTPS, `generate_appcast`, delta updates. A doc manda notarizar + Developer ID e assinar o DMG/zip com a chave EdDSA; Hardened Runtime + Library Validation recusa Sparkle se o host for ad-hoc. Troca o `.app` no lugar (sem admin se o usuário for dono de `/Applications/Tsuro.app`). Formatos de update: dmg, zip, tar, aar, pkg. É o updater de desktop Mac mais maduro — e é ObjC/Swift, um `.framework` a mais para codesign. Do iced isso é FFI, não um crate. Não instala a primeira cópia.
Sources: https://sparkle-project.org/documentation/, https://github.com/sparkle-project/Sparkle/blob/2.x/LICENSE

**WinSparkle (watch no Windows, par do Sparkle).** MIT, C API, UI nativa numa thread à parte, sem dependência de toolkit — o iced consegue chamar. Um `WinSparkle.dll` no mesmo diretório do exe. Mesma filosofia de appcast. Não cria o instalador; o first-run continua sendo NSIS/MSI. História consistente com Sparkle, duas implementações.
Sources: https://winsparkle.org/

**`self_update` 1.3.0 (keep como plano B do updater).** crates.io 2 set 2026, MIT, backend GitHub (`GET /repos/{owner}/{repo}/releases/latest` por baixo). 1.3 adiciona `bundle_path_in_archive("Tsuro.app")`: troca o `.app` inteiro (não só o exe — senão a assinatura e o Pdfium ficam velhos). Verifica digest `sha256:` do asset GitHub, zipsign opcional, `verify_binary` aceita `codesign --verify --deep` no bundle staged. Nunca eleva privilégio: install em Program Files falha com `InstallPathNotWritable`. Detecta App Translocation e recusa update no mount read-only. Não é instalador. Se o packager-updater parar, este crate é o updater in-app nos dois SOs, com o mesmo feed.
Sources: https://crates.io/crates/self_update, https://docs.github.com/en/rest/releases/releases#get-the-latest-release

**MSIX + App Installer (reject como história única).** Auto-update e repair sem Store existem desde Windows 10 2004: `.appinstaller` com `HoursBetweenUpdateChecks`, `UpdateBlocksActivation`, `UpdateURI` de fallback. Fora da Store o pacote TEM de ser assinado; certificado self-signed só serve em máquina que já confia nele. SmartScreen avalia publisher + hash à parte da assinatura: OV/EV mostram aviso até acumular reputação; EV **deixou de** pular o aviso. Artifact Signing (~US$ 10/mês) é o caminho que a Microsoft recomenda para CI. Isso é uma história Windows (URI de update, Sideload). No Mac não há equivalente — notação ≠ SmartScreen. Adotar MSIX no Windows e Sparkle no Mac é duas filosofias.
Sources: https://learn.microsoft.com/en-us/windows/msix/app-installer/auto-update-and-repair--overview, https://learn.microsoft.com/en-us/windows/msix/package/signing-package-overview, https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/smartscreen-reputation

**Mac App Store e Microsoft Store (reject).** A Apple oferece *Direct Distribution* / Developer ID precisamente para Mac fora da loja; a loja pede App Store Connect, App Review, metadados travados depois do upload, e o sandbox/capabilities do programa. O Tsuro abre PDF arbitrário do disco — sandbox de loja luta com “leitor MIT pequeno, sem conta”. A Microsoft Store re-assina MSIX (SmartScreen some) e o registro individual ficou grátis, mas o pacote passa pelo Windows App Certification Kit, Partner Center, identidade do publisher e listagem. EXE/MSI na Store ainda exigem Authenticode próprio e instalador offline. Nenhuma das duas lojas é o canal de um leitor sem conta.
Sources: https://developer.apple.com/documentation/xcode/distributing-your-app-for-beta-testing-and-releases, https://developer.apple.com/programs/, https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/overview, https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/msix/app-package-requirements

**cargo-bundle (reject como stack).** 0.11.0 saiu em 30 mai 2026 (não está morto), gera `.app` e copia frameworks para `Contents/Frameworks`. Sem updater, MSI “experimental”, e o próprio README ainda escreve “Very early alpha. Expect the format to change”. Não carrega notação nem signing Windows.
Sources: https://crates.io/crates/cargo-bundle

**Homebrew Cask (watch, canal opcional).** Cask exige artefato que o desenvolvedor publica e que passa no Gatekeeper — ou seja, o DMG notariado de cima. Não substitui o instalador do produto. `brew upgrade` é o updater desse canal; quem instalou pelo DMG não é cliente do brew.
Sources: https://docs.brew.sh/Acceptable-Casks

**Zip do `.app` sozinho (reject como instalador de longo prazo).** Apple aceita ZIP só como *veículo de notação*. ZIP não staple. Sparkle avisa que archive com mais do que o `.app` piora App Translocation; `self_update` recusa update em app translocated. Sem DMG + symlink `/Applications`, o usuário roda de dentro do download e o update quebra. O script de hoje entrega um `.app` para arrastar — ok no primeiro dia, não é o ship de 0.2.

## O que o SO não unifica

- **Notação ≠ SmartScreen.** Apple: sem Developer ID + ticket, o Gatekeeper bloqueia o download. Windows: mesmo com OV/EV, o primeiro hash é “unrecognized” até reputação (semanas, centenas de installs limpos). EV não compra o atalho. Store Windows some com o aviso; Store Apple some com a notação — e as duas lojas já foram rejeitadas.
- **Admin.** Mac: usuário dono de `/Applications/Tsuro.app` atualiza sem admin. Windows: MSI em Program Files pede admin sempre; NSIS `CurrentUser` e o updater `passive` não. MSIX per-user também não. Escolher MSI “porque o cargo-dist já faz” quebra a história.
- **Ad-hoc / sem cert.** Mac: não notaria, Sparkle não carrega com Library Validation, Gatekeeper recusa o arquivo baixado. Windows: SmartScreen “Windows protected your PC”; Smart App Control (Win11) bloqueia unsigned sem reputação positiva. O caminho sem certificado é só `bundle-macos.sh` na máquina do desenvolvedor.

## Verdict

Uma stack, não um menu.

**v0.1 (Mac primeiro, o `.app` de hoje).** Continuar o `scripts/bundle-macos.sh` como gerador do bundle (ele já acerta Pdfium em `Frameworks` e o `CFBundleIdentifier`). Enrolar no Apple Developer Program (US$ 99/ano). Trocar o `codesign -s -` por Developer ID Application + `--options runtime` + timestamp, assinar também `Contents/Frameworks/libpdfium.dylib`, empacotar num DMG UDIF com symlink `/Applications`, `notarytool submit --wait`, `stapler staple` no DMG. Publicar o DMG num GitHub Release. Sem updater in-app nesta versão: só existe um release. Não zipar o `.app` como canal estável.

**v0.2, quando o Windows sair — a mesma história.** `cargo-packager` gera o DMG Mac (com `signing_identity` + `notarization_credentials`, Pdfium via `Resource::Mapped` → `Contents/Frameworks`) e o NSIS `CurrentUser` no Windows (Pdfium dll ao lado do exe). `cargo-packager-updater` no crate `tsuro` consulta um JSON HTTPS (pode viver no próprio GitHub Release), verifica minisign, baixa `format = "app"` ou `"nsis"`, instala sem admin. O usuário nos dois SOs: baixa o instalador uma vez, depois o app se atualiza. Não é Tauri. Não é loja.

Plano B se o preview do packager-updater esfriar: manter o packager só para o first-install e trocar o updater por `self_update` 1.3 (`bundle_path_in_archive` + digest GitHub). cargo-dist entra depois, se quisermos Homebrew automático e CI de tag — como *fábrica de release*, não como instalador do leitor. Sparkle/WinSparkle ficam na reserva se a UI nativa de update passar a importar mais do que um crate Rust.


## Verdict OSS sem certificado (GitHub, 2026-09-07)

Restrição real: sem Apple Developer Program agora. Distribuição 100% open source via GitHub Releases — DMG Apple Silicon (arm64) + instalador Windows. Update in-app como o [Hermes Desktop](https://github.com/fathah/hermes-desktop): aviso sutil, um clique, download e reinício automáticos.

### O que o Hermes faz (referência)

Hermes usa `electron-updater` contra GitHub Releases: checa ~5s após abrir, `autoDownload = false`, avisa o renderer via IPC (`update-available` → botão na sidebar), o usuário clica, baixa (`download-update`), quando termina mostra “Reiniciar para atualizar” e chama `quitAndInstall()`. Ver `src/main/index.ts` e o botão `sidebar-update-btn` em `Layout.tsx`.

No Tsuro (Rust/iced nativo), o equivalente é o crate [`self_update`](https://docs.rs/self_update/1.3.0/self_update/) 1.3 com backend GitHub — sem Sparkle, sem electron-updater, sem certificado pago.

### Instalação (first run)

| Plataforma | Artefato no Release | Experiência |
|---|---|---|
| macOS arm64 | `Tsuro-{version}-aarch64-apple-darwin.dmg` | Abrir DMG, arrastar para `/Applications`. **Ad-hoc sign** (`codesign -s - --options runtime`) — não notarizado. Gatekeeper pede confirmação **uma vez** (“Abrir mesmo assim” em Ajustes → Privacidade e Segurança no macOS 15+). Depois abre normal. |
| Windows | `Tsuro-{version}-x86_64-pc-windows-msvc-setup.exe` (NSIS `CurrentUser`) | Instala em `%LOCALAPPDATA%\Programs\Tsuro` sem admin. SmartScreen pode avisar na primeira vez (mesma classe de atrito que Gatekeeper sem certificado). |

Pdfium continua em `Contents/Frameworks/libpdfium.dylib` (Mac) ou ao lado do `.exe` (Windows). O script `bundle-macos.sh` já acerta o Mac; falta empacotar DMG (`create-dmg` ou `cargo-packager`) e NSIS no CI.

**Não** zipar o `.app` como canal principal — DMG com symlink `/Applications` evita App Translocation, que quebra update in-place (`self_update` recusa com `Error::AppTranslocated`).

Documentar no README a primeira abertura no Mac (Gatekeeper esperado, não é malware). Projetos OSS fazem o mesmo: [Macget](https://github.com/Suryansh-Codes2209/Macget), [Dropshit](https://github.com/iamsumanp/Dropshit).

### Update in-app (fluxo Hermes → Tsuro)

1. **Checar** — ao abrir (delay ~5s) e no máximo 1×/24h (`self_update::UpdateCheckGuard`). `GET /repos/claudioorjunior/tsuro-pdf/releases/latest` via `is_update_available()`.
2. **Avisar** — se houver versão maior: pill/botão discreto na barra do Tsuro: “Atualização v0.2.0 disponível”. Sem modal bloqueante.
3. **Baixar** — usuário clica “Atualizar”; barra de progresso na pill ou toast. Asset escolhido por target (`aarch64-apple-darwin` / `x86_64-pc-windows-msvc`).
4. **Instalar** — Mac: `bundle_path_in_archive("Tsuro.app")` + `bundle_install_path("/Applications/Tsuro.app")` — troca o bundle inteiro (Pdfium incluído). Windows: NSIS silent ou troca do diretório de instalação. Verificar digest `sha256:` que o GitHub publica por asset (`checksums` feature).
5. **Reiniciar** — `self_update::restart::restart()` relança o app na versão nova.

Sem certificado Apple, **não** usar `verify_binary` com `codesign --verify --deep` — falharia no ad-hoc. Integridade vem do digest GitHub (+ opcional minisign depois, chave no repo, zero custo).

### CI / Releases (GitHub Actions)

Workflow em tag `v*`:

- `cargo build --release -p tsuro --target aarch64-apple-darwin`
- `bundle-macos.sh` → `.app` ad-hoc → DMG
- `cargo build --release -p tsuro --target x86_64-pc-windows-msvc` → NSIS (cargo-packager ou script)
- Anexar assets ao GitHub Release com nomes que o `self_update` reconheça pelo target triple

Sem Apple ID no CI por enquanto — build ad-hoc only. Quando houver US$ 99/ano, o mesmo pipeline ganha `notarytool` + staple no DMG; o updater **não muda**.

### O que fica de fora (por enquanto)

- Notarização Apple / Developer ID
- Authenticode pago no Windows (SmartScreen some com reputação ou certificado; até lá, aviso na primeira instalação)
- Mac App Store, Microsoft Store
- Sparkle (ObjC, briga com ad-hoc + Library Validation; `self_update` cobre o caso)
- `cargo-dist` como instalador GUI (só CLI)

### Quando pagar Apple (opcional, depois)

Só remove o atrito da **primeira** instalação no Mac. O updater, o DMG, o feed GitHub e a UI permanecem iguais — entra `Developer ID` + `notarytool` no job de release. Não é pré-requisito para o fluxo Hermes-like.

**Custo agora:** zero (GitHub Releases + ad-hoc + digest SHA-256). **Custo depois (opcional):** Apple US$ 99/ano (some o aviso do Gatekeeper na primeira instalação) + Authenticode (~US$ 10/mês no Windows, ou reputação orgânica no SmartScreen).
