<p align="right">🇺🇸 <a href="README.md">English</a></p>

<p align="center">
  <img src="docs/hero.png" alt="TsuroPDF hero — tsuru de origami sobre montanhas em névoa" width="100%">
</p>

<h1 align="center">TsuroPDF</h1>

<p align="center">
  <a href="https://github.com/claudiumio/tsuro-pdf/releases"><img src="https://img.shields.io/github/v/release/claudiumio/tsuro-pdf?sort=semver&display_name=release" alt="Última release"></a>
  <a href="LICENSE"><img src="https://img.shields.io/github/license/claudiumio/tsuro-pdf" alt="Licença: MIT"></a>
  <img src="https://img.shields.io/badge/macOS-Apple_Silicon-000?logo=apple&logoColor=white" alt="macOS Apple Silicon">
  <img src="https://img.shields.io/badge/Windows-x64-0078D4?logo=windows&logoColor=white" alt="Windows x64">
  <img src="https://img.shields.io/badge/built_with-Rust-CE422B?logo=rust&logoColor=white" alt="Feito com Rust">
</p>

<p align="center"><strong>Leitor de PDF de código aberto, rápido e leve, para o desktop. Ler, marcar, imprimir. Nada além disso.</strong></p>

TsuroPDF é um visor nativo escrito em Rust ([iced](https://iced.rs/) + [Pdfium](https://pdfium.googlesource.com/pdfium/)). O alvo é qualquer PDF: arquivo grande, tipografia exigente, documento com assinatura digital. Se um recurso não deixa ler, marcar ou imprimir mais rápido, mais leve ou mais claro, ele não entra aqui. Sem formulários, sem nuvem, sem colaboração, sem suíte.

## Recursos

- **Abrir** pela barra de ferramentas, arrastando o arquivo, ou pelo navegador vazio (pastas e arquivos recentes)
- **Abas** — abra em uma aba nova com Ctrl+T, alterne com Ctrl+Tab
- **Páginas** — painel de miniaturas com aba de sumário, anterior/próxima, contador Página N / total, ir para página com navegação por teclado
- **Modos de vista** — página única ou rolagem contínua
- **Retomar leitura** — reabre cada arquivo de onde você parou (página + zoom); histórico voltar/avançar
- **Zoom** — ajustar à largura, encaixar a página, `+` / `-`
- **Buscar** no texto extraído, com a conta de ocorrências e `F3` / `Shift+F3` para navegar
- **Marcações** — destaque, sublinhado, riscado e notas ancoradas a um trecho (`H` / `U` / `S` / `N`)
- **Exportar marcações** — copiar como Markdown (`M`) ou gravar um arquivo Markdown (`Cmd/Ctrl+E`)
- **Salvar cópia** com as marcações aplicadas (`Cmd/Ctrl+S`)
- **Diário de recuperação** — marcações não salvas sobrevivem a uma falha e são oferecidas na próxima abertura
- **Imprimir** — diálogo próprio com spool direto (gera um PDF de impressão a 200 DPI, sem diálogo nativo)
- **Assinaturas** — painel sob demanda, com o signatário e o estado criptográfico
- **Paleta de comandos** — `Cmd/Ctrl+K` para ações e resultados no documento
- **Configurações** — tema (sistema, escuro, claro), idioma (português, inglês), leitor padrão, permissões no macOS
- **Atualização pelo app** — verifica a última GitHub Release e instala por dentro do app
- **Rotação de vista** — 90° por sessão, além de tema claro/escuro com Inter embutida

As marcações são estado da sessão e não são gravadas de volta no arquivo aberto. O que persiste é a posição de leitura, as cópias salvas que você gera e o diário de recuperação. O que fica de fora — de propósito — é todo o resto.

## Instalar

A forma curta, com [Bun](https://bun.sh/):

```bash
bun run install:tsuro -- --install
```

Consulta a última [GitHub Release](https://github.com/claudiumio/tsuro-pdf/releases), baixa o artefato do seu sistema e confere o SHA-256. Sem `--install` só baixa. `--check` diz se há versão nova; `--version v0.6.0` pina uma tag. Se ainda não houver release para o seu sistema, o script imprime o caminho de clone e sai com código 1.

O app também verifica na abertura (o estado fica em Configurações → Sobre) e pode instalar a partir de lá: no macOS o bundle é trocado no lugar e reaberto; no Windows o instalador NSIS roda em silêncio depois que o app fecha.

Ou baixe o arquivo da release:

- **Mac Apple Silicon** — `TsuroPDF-{versão}-aarch64-apple-darwin.dmg`. Abra o DMG e arraste TsuroPDF para Applications.
- **Windows x64** — `TsuroPDF-{versão}-x86_64-pc-windows-msvc-setup.exe`. Instala em `%LOCALAPPDATA%\Programs\TsuroPDF`, sem admin. `/S` é a instalação silenciosa.

Ainda não há build para Mac Intel nem Linux.

**Primeira abertura no Mac.** A assinatura é ad-hoc (sem Apple Developer Program). O Gatekeeper avisa: clique com o botão direito em TsuroPDF → Abrir.

**Primeira abertura no Windows.** Se o SmartScreen aparecer: Mais informações → Executar assim mesmo.

## Atalhos

`Cmd` no macOS, `Ctrl` nos outros sistemas, salvo menção. As teclas simples são ignoradas quando um campo de texto está focado.

| Teclas | Ação |
| --- | --- |
| `Cmd/Ctrl+O` | Abrir PDF |
| `Cmd/Ctrl+T` / `Cmd/Ctrl+W` | Abrir em aba nova / fechar aba |
| `Ctrl+Tab` / `Ctrl+Shift+Tab` | Alternar abas (nos dois sistemas; `Cmd+Tab` fica com o macOS) |
| `Cmd/Ctrl+P` | Imprimir |
| `Cmd/Ctrl+S` | Salvar cópia com marcações |
| `Cmd/Ctrl+E` | Exportar marcações como Markdown |
| `Cmd/Ctrl+,` | Configurações |
| `Cmd/Ctrl+K` | Paleta de comandos |
| `Cmd/Ctrl+F` | Focar a busca; `Enter` próximo hit, `Shift+Enter` anterior |
| `F3` / `Shift+F3` | Próximo / anterior resultado |
| `Cmd/Ctrl+C` | Copiar o texto selecionado |
| `M` | Copiar marcações como Markdown |
| `Cmd/Ctrl+Z` / `Cmd/Ctrl+Shift+Z` | Desfazer / refazer marcação |
| `H` / `U` / `S` / `N` | Destaque / sublinhado / riscado / nota |
| `Delete` | Remover marcação selecionada |
| `R` | Girar a vista 90° |
| `+` / `-` | Aumentar / diminuir zoom |
| `PageUp` / `PageDown`, `←` / `→` | Página anterior / próxima |
| `Home` / `End` | Primeira / última página |
| `↑` / `↓` / `Enter` | Navegar o painel aberto (sumário, marcações) |
| `Cmd/Ctrl+Enter` | Salvar o rascunho de nota |
| `Esc` | Fechar o diálogo / a paleta |

Os controles também ficam na barra e no menu `⋯`; a paleta lista cada ação com o atalho correspondente.

## Compilar do código

```bash
git clone https://github.com/claudiumio/tsuro-pdf.git
cd tsuro-pdf
./scripts/bundle-macos.sh        # .app macOS em dist/
```

```powershell
powershell -File scripts/bundle-windows.ps1   # instalador NSIS no Windows
```

Desenvolver o visor pede [Rust](https://rustup.rs/) estável (`rustup default stable`) mais a biblioteca [Pdfium](https://github.com/bblanchon/pdfium-binaries) no diretório do projeto ou no sistema — o `bundle-macos.sh` resolve isso no Mac. Node.js só entra para o visor legado (Tauri + PDF.js), que não é o produto.

Rodar a partir do código:

```bash
cargo test -p tsuro
cargo test -p tsuro-sign
cargo run -p tsuro -- public/samples/guia-folio.pdf
```

Sem argumento, a janela abre vazia.

## Exemplos

Há três PDFs em `public/samples/`:

| Arquivo | O que testa |
| --- | --- |
| `guia-folio.pdf` | Tipografia, tabela, acentos, busca |
| `sumario-folio.pdf` | Bookmarks (sumário) alimentando a aba Sumário |
| `contrato-assinado.pdf` | Campo `/Sig` com certificado autoassinado de demonstração |

O certificado do contrato é **autoassinado**. TsuroPDF trata isso como assinatura criptograficamente íntegra, mas sem cadeia de confiança pública. O estado esperado é "íntegra (sem confiança pública)".

## Assinaturas digitais

O reconhecimento e a verificação vivem no crate Rust `tsuro-sign`:

- percorre AcroForm e dicionários `/Sig`
- lê o PKCS#7/CMS (perfil `adbe.pkcs7.detached`)
- confere o `ByteRange` e o `messageDigest`
- verifica RSA + SHA-256 sobre os atributos assinados

Estados que o painel pode mostrar: válida, íntegra (sem confiança pública), documento alterado, inválida, não suportada, certificado expirado ou ainda não válido.

## Estrutura

```
crates/tsuro            visor nativo (iced + Pdfium) — o produto
crates/tsuro-sign       motor de assinaturas (PDF + CMS)
public/samples          PDFs de exemplo
docs/hero.png           hero do README
src-tauri, src          visor legado (Tauri + React + PDF.js)
```

O legado continua no repositório para consulta. Não é o que se empacota como TsuroPDF.

```bash
npm install
npm run test:sign
npm run dev          # visor legado no navegador (http://127.0.0.1:43177)
npm run tauri dev
```

## Contribuir

Contribuições de leitura, verificação e empacotamento são bem-vindas. TsuroPDF pretende continuar pequeno: toda proposta passa pela missão (ler, marcar ou imprimir mais rápido, mais leve ou mais claro). Issues, PRs e commits em inglês ou PT-BR; PRs e issues com corpo bilíngue, PRs pequenos, um assunto por vez. Veja o [guia de contribuição](CONTRIBUTING.pt-BR.md) para o guia completo e a ordem de prioridade das contribuições.

## Licença

[MIT](LICENSE). Copyright (c) 2026 Tsuro contributors.
