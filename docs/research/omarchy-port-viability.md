# Port do TsuroPDF para Omarchy Linux: estudo de viabilidade

Data: 2026-11-14. Base: `52960c6` (main). Escopo: decidir **se**, **com que custo** e **em que ordem** o visor nativo (iced + Pdfium) roda num Omarchy de verdade, e o que a distribuição exige além de "o binário abre".

## 1. Veredito curto

Viável, e mais barato do que a história do `AGENTS.md` sugere. O crate já foi escrito com uma plataforma Linux de primeira classe em dois lugares que costumam doer: as métricas de chrome (`kiri::Metrics::for_platform`, `kiri.rs:467`, com valores próprios para `Platform::Linux`) e o roteamento de atalhos (`Ctrl` em vez de `⌘`, `session.rs:5702`). O que falta é concentrado e nomeável: empacotamento, escolha de decoração de janela sob Hyprland, Pdfium como `libpdfium.so`, spool de impressão via CUPS e o caminho de atualização.

O custo real não está no Rust. Está em três decisões de produto que o código ainda não tomou: (a) que decoração de janela o app usa no Wayland, (b) se o app adota o tema do Omarchy, (c) quem atualiza o binário depois de instalado. As três têm resposta barata se escolhidas antes do primeiro PR, e caras se escolhidas durante.

Recomendo um PR de integração (fase 0 + fase 1 abaixo), medido em um Omarchy real, antes de qualquer compromisso público de suporte a Linux.

## 2. O alvo, concretamente

> Fontes: manual e repositório do Omarchy (`omarchy.org/manual/*`, `github.com/basecamp/omarchy`), AUR.

- Omarchy é Arch Linux + Hyprland, opinativo, entregue como pacotes `pacman` do repositório próprio do Omarchy (`/usr/share/omarchy/...`), atualizado por `Update > Omarchy`. Não é uma distro separada com API de plugin.
- Instalação de terceiros: `pacman` e AUR. O menu (`Super + Space` → Install > Package / AUR) chama `omarchy pkg add` e `yay`. Não há canal oficial de AppImage, Flatpak ou Snap; há integrações comunitárias.
- Tema: cada tema é um diretório com `colors.toml` em `/usr/share/omarchy/themes/<nome>` (ou `~/.config/omarchy/themes/<nome>`), expandido para os apps via templates em `default/themed/*.tpl` quando `omarchy-theme-set` roda. Um app de terceiro se integra colocando seu próprio `.tpl` em `~/.config/omarchy/themed/` (ou via `omarchy-theme-hook`), não por API.
- Wayland nativo, com `XWayland` de fallback. Hyprland **não desenha decoração de servidor**: sem SSR, as janelas são sem moldura; o app desenha o que quiser, ou o `libdecor` desenha uma barra Adwaita por cima.
- O PDF que o Omarchy abre com duplo clique vai para o Document Viewer. Um port do Tsuro precisa de `.desktop` e de `MimeType=application/pdf` para competir por esse duplo clique.

## 3. O que o Tsuro é hoje, por camada

Legenda: **P** portátil como está, **C** precisa de `cfg` novo, **M** macOS-only, **V** vazio hoje (retorna erro "não suportado").

| Módulo / área | LOC | Estado no Linux | Nota |
|---|---|---|---|
| `page.rs`, `search.rs`, `i18n.rs`, `font.rs`, `positions.rs`, `prefs.rs`, `browse.rs`, `recovery.rs`, `session.rs`, `view.rs` | ~29k | **P** | Sem `objc`. Caminhos de dados já usam `XDG_DATA_HOME` com fallback `~/.local/share/tsuro` (`prefs.rs:49-63`, `positions.rs:60-75`, `browse.rs:88-102`). |
| `kiri.rs` (tokens + métricas) | 1.7k | **P** | `Platform::Linux` já existe com `control/button = 34`, `bar_height = 50`, `tab_strip = 36`, `unified_title() = false` (`kiri.rs:436-497`). |
| `view.rs` (chrome) | 5.1k | **P** | O layout da barra já ramifica em `unified_title()` e cai num caminho "barra nativa acima" (`view.rs:467-535`). |
| `session.rs` (atalhos) | 17k | **P** | `#[cfg(not(target_os = "macos"))]` já dá `Ctrl` para undo/save/etc. e `Alt+←/→` para histórico (`session.rs:5702-5790`). |
| `engine.rs` (Pdfium) | 1.7k | **P** | `bind()` procura `../Frameworks`, ao lado do binário e depois o sistema (`engine.rs:313`). `Pdfium::pdfium_platform_library_name()` resolve `libpdfium.so` no Linux. |
| `main.rs` | 61 | **C** | O caminho não-macOS já existe e é o app iced genérico (`main.rs:19-27`). Só precisa compilar de verdade. |
| `print.rs` + `spool.rs` | 750 | **C** | Fallback de visualizador externo já usa `xdg-open` (`print.rs:332`). Spool passa por `printers 2.3.0`, que faz `#[link(name = "cups")]` no unix. |
| `update.rs` | 982 | **V** | `asset_name()` devolve `None` fora de macOS aarch64 e Windows x64 (`update.rs:89-100`); `install_from` não-macOS é `Err(Unsupported)` (`update.rs:430-433`). |
| `macos_menu.rs` | 418 | **M** | Barra de menu do app no macOS. Não tem equivalente; no Wayland o menu é do app. |
| `macos_window.rs` | 136 | **M** | Semáforos nativos e `mouseDownCanMoveWindow`. Irrelevante no Linux. |
| `native_open.rs` | 414 | **M** | Handler de Apple Events (`odoc`) para abrir PDF do Finder. O análogo Linux é `.desktop` + `%f` na linha `Exec`, que já é o que `main.rs` consome (`std::env::args().nth(1)`). |
| `src-tauri/` | n/a | **M** | Legado. `AGENTS.md` proíbe estender. |

Total macOS-only: 968 LOC, todos atrás de `#[cfg(target_os = "macos")]`, zero entrelaçados em `session`/`view`.

## 4. Os cinco riscos concretos

### 4.1 Decoração de janela sob Hyprland

O iced não expõe "desenhe sua própria titlebar" (`iced-rs/iced#759`, aberto). O `iced_winit 0.13` liga por default as features `x11`, `wayland`, `wayland-dlopen` e `wayland-csd-adwaita`: o winit desenha uma barra Adwaita de cliente. Hyprland rejeita CSD de propósito ("CSD? Forcefully disabled", `hyprwm/Hyprland#1556`) e já teve regressão em que parar de anunciar `zxdg_decoration_manager_v1` fez todo cliente `libdecor` ganhar barra por cima (`hyprwm/Hyprland#15078`).

Consequência para o Tsuro: ou o app aceita a barra do `libdecor` e desenha o chrome abaixo dela (é exatamente o caminho `unified_title() == false` que já existe), ou assume uma titlebar própria e pede `start_dragging` ao winit. A segunda opção é onde `Message::DragWindow` precisa de implementação real: hoje ela devolve `Task::none()` porque no macOS o arrasto tem de acontecer dentro do próprio `update` (`session.rs:3266`). No Wayland o comando equivalente é `window.drag`, que o `iced_winit` já chama (`program.rs:1255`).

Recomendação: fase 1 usa decoração de cliente (zero código novo). Titlebar própria é fase 4, só se a barra dupla incomodar em uso real.

### 4.2 Pdfium como biblioteca do sistema

O motor já tem a ordem de busca certa. No Linux o arquivo é `libpdfium.so`, e o AUR oferece quatro pacotes que o fornecem (`pdfium-binaries`, `pdfium-binaries-bin`, `pdfium-binaries-v8`, `libpdfium`, todos `Provides: libpdfium`). Duas consequências:

- empacotar `libpdfium.so` no `.pkg.tar.zst` é permitido (licenças BSD/Apache do PDFium) e é o que faz o app funcionar sem `yay`; o `release chromium/8044` do `bblanchon/pdfium-binaries` já publica `pdfium-linux-x64.tgz` e `pdfium-linux-arm64.tgz`;
- depender do pacote AUR do sistema é mais leve de manter, mas obriga o usuário a instalar um pacote AUR antes do Tsuro.

O teste `pdfium_binds_when_ci_requires_it` (`engine.rs:1288`) pula silenciosamente quando não há sample; em CI Linux com `CI=1` ele estoura se o bind falhar. Então a CI prova o bind, desde que alguém ponha `libpdfium.so` no runner.

### 4.3 Impressão via CUPS

`printers 2.3.0` faz binding direto em `libcups` (`printers/src/unix/cups/dests.rs:7`), então `libcups` é dependência de build e de runtime. O job vai com `Converter::None`, que o CUPS aceita com `application/pdf` no `cupsPrintFile`. No Arch `libcups` é subpacote de `cups` em `extra` (`cups.spec`/`libcups 2:2.4.20`), instalado por padrão num Omarchy. No runner Debian/Ubuntu da CI, o build precisa de `libcups2-dev`.

Risco real: o `print.rs` tem `#[cfg(test)]` que stubba a lista de impressoras, então a CI não cobre o caminho CUPS. A validação é manual numa máquina com `cupsd` de pé.

### 4.4 Atualização in-app

Hoje o updater só sabe macOS e Windows: `asset_name()` devolve `None`, `install_from` devolve `Unsupported`. Na prática o painel de Configurações vai cair em `UpdateState::Failed` no Linux, o que é honesto mas feio. Três saídas, em ordem de custo:

1. esconder o updater no Linux (uma linha de `cfg`);
2. entregar o pacote ao `pacman` e nunca atualizar por dentro do app;
3. `Omarchy Packages` + AUR com o PKGBUILD apontando para a release: o `yay`/`pacman -Syu` faz o trabalho.

A 2 e a 3 são a mesma coisa na prática. A cultura do Omarchy é "o pacman atualiza", e isso remove a fase de updater do port.

### 4.5 Tema do Omarchy

O `kiri.rs` tem tokens fixos (Kiri). O Omarchy tem 22 temas em `colors.toml`, expandidos por template. Não existe API; existe convenção de arquivo. Duas rotas:

- **respeitar**: ler `~/.local/state/omarchy/current/theme/colors.toml` (ou o caminho que `omarchy-theme-set` deixa) e mapear para `Tokens`. Acopla o app a uma estrutura de arquivo de terceiro.
- **não respeitar**: Tsuro entra claro/escuro como hoje, e o app fica visualmente alheio ao resto do desktop.

Minha leitura: um leitor de PDF deve respeitar o desktop em que vive, mas o mapeamento só vale a pena depois que o app roda. Fase 5, não fase 1.

## 5. Como testar sem hardware Omarchy

Este Mac não tem Docker, Podman, Lima, Multipass nem OrbStack, e o Rust é o do Homebrew (1.95, sem `rustup`, sem alvo cruzado). Toda a validação real acontece fora desta máquina:

| Nível | O que prova | Como |
|---|---|---|
| A. Compilação | O `cfg` Linux compila e os testes passam | GitHub Actions `ubuntu-latest`: `apt install libcups2-dev`, baixar `pdfium-linux-x64.tgz` para o cwd, `cargo test -p tsuro`. O `release.yml` já tem job `ubuntu-latest`, é só reusar o molde. |
| B. Abre e renderiza | O binário abre um PDF headless | `xvfb-run -a cargo run -p tsuro -- public/samples/guia-folio.pdf` com um `grim`/`import` de screenshot no runner. Prova que o `iced` liga GL sob X11; não prova Wayland. |
| C. Wayland e Hyprland | A coisa que o usuário vai ver | Máquina Omarchy real (a do usuário, ou uma VM com Hyprland). Nada aqui substitui isto. |
| D. Impressão | CUPS de verdade | Só em C, com `cupsd`. |

O teste B não é opcional: o `iced` no Linux tem histórico de regressão em resize e DPI sob Wayland (`iced-rs/iced#2750`, `#2404`, `#2687`).

## 6. Fases

Cada fase termina num estado verificável. Nenhuma fase publica release.

| Fase | Entrega | Verificação | Esforço |
|---|---|---|---|
| 0 | Branch de trabalho, `cargo build -p tsuro` no Linux, `cargo test -p tsuro` no Linux | CI Linux verde | 0,5 dia |
| 1 | `scripts/bundle-linux.sh` gerando `dist/TsuroPDF` + `.desktop` + ícone; `libpdfium.so` baixado do `pdfium-binaries` | Abre PDF em Omarchy real, duplo clique no Nautilus | 1–2 dias |
| 2 | PKGBUILD + `pacman -U` local; dependência `cups`; `libpdfium.so` empacotado | Instala e desinstala limpo numa máquina | 1 dia |
| 3 | Caminho CUPS de impressão verificado, diálogo próprio intacto | Imprime em impressora local e em PDF | 0,5–1 dia |
| 4 | (opcional) titlebar própria com `start_dragging` | Arrastar janela, maximizar, atalhos não roubados | 1–2 dias |
| 5 | (opcional) mapear `colors.toml` do Omarchy para `kiri::Tokens` | Trocar de tema no Omarchy e ver o Tsuro acompanhar | 1–2 dias |

Total até um app instalável e imprimindo: **3 a 4 dias de trabalho**. Fases 4 e 5 são produto, não viabilidade.

### Critérios de kill

Parar e reportar se, na fase 1:

- o `iced 0.13` não sustenta resize ou escala fracionária em Hyprland sem workaround feio;
- o `libdecor` desenha barra sobre a nossa, e desligar a feature `wayland-csd-adwaita` quebra o arrasto da janela;
- o Pdfium do AUR não carrega no binário empacotado (ABI, `soname`, ou conflito com os pacotes `pdfium-binaries-v8` instalados pelo usuário).

## 7. Trade-offs das alternativas

| Alternativa | O que ganha | O que custa | Quando faz sentido |
|---|---|---|---|
| **Port nativo (recomendado)** | O app é o mesmo código; toda a UI, anotação, assinatura e impressão vêm de graça; zero fork | 3–4 dias + manutenção de PKGBUILD e de um terceiro alvo na CI | O usuário usa Omarchy no dia a dia e quer o leitor dele lá |
| **Empacotar o build Windows/Wine** | Nada a escrever | Wine para um leitor de PDF é o oposto do "leve e veloz" do mission gate; Pdfium e CUPS no Wine são frágeis | Nunca, para este produto |
| **Não portar; indicar Okular/Zathura** | Zero custo | Perde o produto no único desktop onde o usuário do projeto já vive | Se o objetivo for "ler PDF no Omarchy hoje" |

O estudo de instaladores de 2026-09-07 já registrou "Linux é extra" (`docs/installer-updater-research.md`, linha 5). Este estudo não contradiz isso: ele mostra que "extra" custa 3–4 dias e não 3 semanas, porque a abstração de plataforma já foi feita em `kiri` e nas métricas de chrome.

## 8. O que este estudo não faz

- Não escreve código de plataforma.
- Não abre PR nem issue.
- Não decide sobre Flatpak/AppImage: a cultura do Omarchy é pacman, e o custo de manter dois canais não se paga.
- Não mede performance no Omarchy real: isso é fase 1.

## 9. Fontes lidas nesta sessão

- Código: `crates/tsuro/src/{main.rs,lib.rs,kiri.rs,view.rs,session.rs,engine.rs,print.rs,spool.rs,update.rs,prefs.rs,positions.rs,browse.rs}`, `crates/tsuro/Cargo.toml`, `.github/workflows/{pr,release}.yml`, `scripts/bundle-macos.sh`, `docs/installer-updater-research.md`.
- Dependências instaladas: `pdfium-render 0.8.37` (`pdfium.rs:136`), `printers 2.3.0` (`src/unix/cups/dests.rs`), `iced_winit 0.13.0` (features default), `rfd 0.15.4` (features default `xdg-portal`).
- Omarchy: `omarchy.org/manual/{other-packages,guis,themes}`, `github.com/basecamp/omarchy` (releases v3.4.0–v3.8.3, `docs/theming.md`, `bin/omarchy-theme-set-qt`).
- AUR: `pdfium-binaries`, `pdfium-binaries-bin`, `pdfium-binaries-v8`, `libpdfium`.
- Hyprland: `hyprwm/Hyprland#1556`, `hyprwm/Hyprland#15078`.
- iced/winit: `iced-rs/iced#759`, `#2625`, `#2657`, `#2687`, `#2750`, `#2404`.
