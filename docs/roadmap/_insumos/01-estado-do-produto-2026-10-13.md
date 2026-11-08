# Estado do produto em 13 de outubro de 2026

Linha de base do planejamento. Tudo aqui foi medido ou lido em 13 de outubro de 2026. Número de linha muda quando o código muda, então reverifique antes de citar.

## 1. Versão, cadência e tamanho

| Item | Valor | Fonte |
| --- | --- | --- |
| Versão atual | v0.5.5, publicada em 12 de outubro de 2026 | `gh release list` |
| Plataformas | macOS Apple Silicon (DMG) e Windows x64 (instalador NSIS) | `README.pt-BR.md` |
| Linguagem e interface | Rust edição 2021, `iced 0.13`, `pdfium-render 0.8` (0.8.37 no `Cargo.lock`) | `crates/tsuro/Cargo.toml` |
| Outras dependências do crate `tsuro` | `rfd 0.15`, `tokio 1`, `thiserror 2`, `unicode-normalization 0.1`, `pdf-writer 0.15`, `miniz_oxide 0.8`, `printers 2`, `tsuro-sign` | `crates/tsuro/Cargo.toml` |
| Tamanho | 24.772 linhas de Rust em `crates/*/src`, 341 funções `#[test]` (contagem estática) | script de 13 de outubro de 2026 |
| Crates | `tsuro` (visor) e `tsuro-sign` (assinaturas) | `AGENTS.md` |

Cadência das releases, em dias desde a anterior.

| Release | Publicação | Dias desde a anterior |
| --- | --- | --- |
| v0.1.0 | 8 de setembro | 0 |
| v0.1.1 | 8 de setembro | 0,3 |
| v0.2.0 | 12 de setembro | 3,8 |
| v0.3.0 | 21 de setembro | 8,9 |
| v0.3.1 | 22 de setembro | 1,0 |
| v0.4.0 | 3 de outubro | 11,2 |
| v0.5.0 | 11 de outubro | 8,6 |
| v0.5.5 | 12 de outubro | 0,5 |

Entre as releases menores (x.y.0), os intervalos foram 4,1, 8,9, 12,2 e 8,6 dias, média de 8,4 dias. Foram 8 releases em 34,3 dias. Use isso como referência de ritmo recente, nunca como promessa. A capacidade futura é desconhecida.

## 2. Backlog atual no GitHub

Repositório: `github.com/claudioorjunior/tsuro-pdf`. Não há milestones nem PRs abertas.

| Issue | Labels | Título |
| --- | --- | --- |
| #127 | `enhancement`, `needs-triage` | Limpar instaladores antigos baixados pela atualização no Windows |
| #44 | `enhancement`, `ui`, `ux` | feat(read): render Markdown files read-only |

A issue #44 pede renderizar arquivos Markdown. O gerente do roadmap deve aplicar o mission gate nela e recomendar manter, adiar ou fechar.

Labels existentes (use só estes): `accessibility`, `bug`, `documentation`, `duplicate`, `enhancement`, `good first issue`, `help wanted`, `invalid`, `question`, `wontfix`, `ready-for-agent`, `needs-triage`, `needs-info`, `ready-for-human`, `ui`, `navigation`, `ux`, `annotation`, `print`.

Vocabulário de triagem (`docs/agents/triage-labels.md`): `needs-triage` (aguardando avaliação), `needs-info` (aguardando mais informações), `ready-for-agent` (pronto para um agente AFK), `ready-for-human` (requer implementação humana), `wontfix` (não será trabalhado). Uma issue só merece `ready-for-agent` quando a ficha tem contexto, critérios de aceite e testes suficientes para um agente implementar sem perguntar.

Dependências entre issues usam o recurso nativo do GitHub (`blocked_by`). Sem ele, uma linha `Blocked by: #n` no topo do corpo (`docs/agents/issue-tracker.md`).

Os arquivos `.github-issue-*.md` na raiz do repositório são rascunhos antigos e estão fora do controle de versão. Várias das funções que descrevem já foram entregues (veja a seção 4). Não trate esses rascunhos como backlog.

## 3. Âncoras de código verificadas

Em `crates/tsuro/src/`, na data acima.

| Âncora | Arquivo e linha |
| --- | --- |
| Estado e mensagens | `session.rs:2137` (`enum Message`), `session.rs:203` (`enum ViewMode`), `session.rs:475` (`enum AnnotKind`) |
| Anotação | `session.rs:486` (`pub struct Annotation`: `id`, `page`, `range`, `quads`, `kind`, `text`, `marker`, sem campo de cor) |
| Cópia e exportação das marcações | `session.rs:411` (`annotations_markdown`), `session.rs:5431` (`selection_plain_text`, devolve o trecho cru da camada de texto) |
| Paleta | `session.rs:599` (`enum PaletteAction`, 9 ações), `session.rs:632` (`PaletteItem`, 6 tipos), `session.rs:873` (`SourceRank`), `session.rs:1007` (`palette_match`) |
| Teclado e atalhos | `session.rs:4889` (`keyboard_message`), `session.rs:5088` (`shortcut_hint`) |
| Histórico | `Message::HistoryBack` em `session.rs:2355` e `HistoryForward` em `session.rs:2356` |
| Painel lateral | `session.rs:2279` (`Message::OutlineTab(bool)`), `view.rs:2489` (`pages_panel`), `view.rs:2511` (`panel_tabs`), `view.rs:2545` (`outline_tab`), `view.rs:2611` (`thumbs_tab`) |
| Chrome | `view.rs:466` (`topbar`), `view.rs:2434` (`tab_strip`), `view.rs:202` (`hud`), `view.rs:1988` (`progress`), `view.rs:809` (`overflow_menu`) |
| Marcação na tela | `view.rs:1390` (`selection_bar_layer`), `view.rs:1445` (`note_card`), `view.rs:3095` (`with_marks`) |
| Paleta na tela | `view.rs:1824` (`palette_layer`), `view.rs:1860` (`palette_card`) |
| Diálogos | `view.rs:1788` (`close_prompt_card`), `view.rs:1926` (`save_warning_card`), `view.rs:1652` (`settings_card`), `view.rs:2405` (`disk_stale_row`) |
| Assinaturas | `view.rs:3231` (`signatures_panel`) |
| Impressão | `print.rs:18` (`PRINT_DPI = 200.0`), `print.rs:50` (`PrintSelection`), `print.rs:62` (`print_selection_pdf`), `session.rs:1856` (`print_job_pdf`), `view.rs:1143` (`print_preview`), `view.rs:1191` (`print_controls`), `spool.rs:16` e `spool.rs:35` |
| Motor | `engine.rs:396` (`save_copy`), `engine.rs:263` (`Request::SaveCopy`), `engine.rs:720` (`save_copy_from_doc`), `engine.rs:198` e `engine.rs:279` (`load_pdf_from_byte_slice`) |
| Posição de leitura | `positions.rs:236` (`find_position`), `positions.rs:81` (`file_identity`), `positions.rs:20` (`MAX_POSITIONS`, 50) |
| Tokens do Kiri | `kiri.rs:29` (`Tokens`), `kiri.rs:49` (`mark_current`) |
| Texto | `page.rs:178` (`TextLayer`), `page.rs:224` (`slice`) |
| Biblioteca Pdfium (fora do repositório) | `pdfium-render 0.8.37`: `PdfPage::label()` em `src/pdf/document/page.rs:256`, TODO sobre `FPDF_INCREMENTAL` em `src/pdf/document.rs:314` |

O caminho do Pdfium está em `~/.cargo/registry/src/*/pdfium-render-0.8.37/`.

## 4. O que já está entregue (não proponha de novo)

Fonte: `README.pt-BR.md`, notas das releases v0.5.0 e v0.5.5, código.

- Abrir pela barra, arrastando ou pelo Início, que é uma grade de pastas e PDFs com prévia da primeira página (#113).
- Painel de páginas com miniaturas e aba de sumário. Anterior e próxima. Contador `n / total`. Ir para página.
- Modos de vista, página única ou rolagem contínua. Rotação de vista. Zoom. Tema escuro e claro.
- Retomar a leitura (página, zoom e modo, por arquivo, no máximo 50 arquivos). Histórico de voltar e avançar com Alt e seta.
- Busca no texto, com contagem. A busca varre o documento inteiro (#107).
- Abas de documentos.
- HUD flutuante com navegação e zoom, que some quando o mouse fica parado (#109).
- Paleta de comandos Ctrl+K (#45) com ações, trechos do documento, linhas do sumário, recentes, arquivos da pasta e trechos de outras abas abertas.
- Marcações. Destacar, sublinhar, riscar e nota, com H, U, S e N. Barra flutuante na seleção. Desfazer e refazer. Marcador de nota arrastável. Salvar uma cópia com marcações. Copiar as marcações como Markdown. Destaque com faixa contínua (#122).
- Impressão com diálogo próprio, prévia, intervalo, cópias, orientação e spool direto. As marcações da sessão vão para a impressão.
- Painel de assinaturas sob demanda, com o estado criptográfico.
- Configurações com tema, permissões, idioma pt-BR e EN (#119) e versão. Leitor padrão (#103). Verificação de atualização.
- Desempenho. Primeira página na tela antes do texto e do sumário. Tamanhos de página sem carregar as páginas (#106). O documento é liberado da memória ao fechar a aba (issue #40).

## 5. Lacunas verificadas por busca textual no código

Busca textual não prova ausência. Cada lacuna precisa de confirmação em execução antes de virar issue.

| Lacuna | O que foi visto |
| --- | --- |
| Diário de recuperação das marcações | Nenhum mecanismo de diário, rascunho ou recuperação em `session.rs`, `engine.rs`, `prefs.rs` e `positions.rs`. As marcações vivem na sessão até salvar a cópia |
| Senha ao abrir | `load_pdf_from_byte_slice(&bytes, None)` no caminho de salvar cópia. Nenhuma ocorrência da palavra `password` em `engine.rs` e `session.rs` |
| Rótulo de página impresso | Nenhum uso de rótulo de página. O `pdfium-render` expõe `PdfPage::label()` |
| Botões 4 e 5 do mouse | Nenhum tratamento de `Button::Back` e `Button::Forward` |
| Cor da marcação | `Annotation` não tem campo de cor |
| Copiar texto | `selection_plain_text` devolve o trecho cru. Não há junção de linhas nem tratamento de hífen de fim de linha |
| Impressão | Cada página vira imagem a 200 DPI, de propósito, para assar intervalo, cópias e orientação no PDF (`print.rs`, comentário do módulo) |
| Salvar | `save_to_bytes` do `pdfium-render`, que reescreve o documento. O `pdfium-render 0.8.37` deixa um TODO sobre `FPDF_INCREMENTAL` |
| Posição de leitura | Guardada por caminho, com identidade de tamanho e data de modificação. Mover o arquivo ou salvá-lo em outro leitor invalida a posição |
| Painel lateral | `Message::OutlineTab(bool)` só distingue duas abas |
| Marcadores de página | Nenhuma função de favoritos ou dobras |
| Tema do papel | A folha é sempre branca, por regra do Kiri |

## 6. Verificação e entrega

Antes de qualquer PR, `cargo test -p tsuro` e `cargo test -p tsuro-sign`. O CI roda isso em macOS e Windows (`pr.yml`), e `rustfmt --check` e `clippy` no macOS (`review.yml`). O release gera o DMG e o instalador NSIS ao criar uma tag `v*` (`release.yml`). As notas de release listam as PRs mescladas com os autores, e cada versão tem destaques manuais.

Verificação manual do visor: `cargo run -p tsuro -- public/samples/guia-folio.pdf`.

Fixtures somente leitura em `public/samples/`.

| Arquivo | Para que serve |
| --- | --- |
| `guia-folio.pdf` | Tipografia, tabela, acentos, busca |
| `sumario-folio.pdf` | Bookmarks que alimentam a aba de sumário |
| `contrato-assinado.pdf` | Campo de assinatura com certificado autoassinado de demonstração |

## 7. O que ainda não foi medido

Tempo até a primeira página, memória por aba, latência de busca e uso de CPU em repouso não têm linha de base. A tarefa S-06 do catálogo cria essa linha de base. Nenhum critério de regressão pode citar número antes dela.
