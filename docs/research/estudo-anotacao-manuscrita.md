# Estudo: anotação com aparência manuscrita, em bloco móvel

Repo `leitor-pdf-opensource`, `main` em `1001a57`, 2026-10-13. Somente leitura, sem protótipo executado.

## Resumo

Veredito: **seguir, limitado, depois de um spike curto.** A ideia cabe na missão porque continua sendo anotar. Ela não edita o conteúdo do PDF. Na tela é viável com o que o app já tem: canvas sobre a página, `text_editor` do post-it e arrasto de marcador (`NoteDrag`). O ponto incerto é o arquivo salvo. A pdfium-render 0.8 cria `/FreeText`, mas não deixa controlar a aparência dela. O caminho para a letra manuscrita aparecer em outros leitores é um `/Stamp` com objeto de texto e fonte TTF embutida, e isso precisa ser provado no Preview, no Acrobat e no Chrome antes de qualquer compromisso. Tamanho estimado G.

## Estado atual

- **Modelo.** `Annotation` tem `id`, `page`, `range`, `quads`, `kind`, `text` e `marker` (`session.rs:487-499`). `AnnotKind` = Highlight, Underline, Strikeout e Note (`session.rs:476-482`). Toda marcação é ancorada a um trecho de texto (`range` + `quads`).
- **Post-it.** A nota é um `text_editor` num cartão (`view.rs:1557-1562`, `NoteDraft` em `session.rs:523`). Só o ícone amarelo se move. O trecho nunca se move (`session.rs:495-498`, `NoteDrag` em `session.rs:557`, `move_note_marker` em `session.rs:5823`).
- **Desenho.** As marcações são desenhadas num `canvas::Program` sobre a página (`view.rs:3083-3132`). A nota desenha um sublinhado sutil e o ícone `STICKY`.
- **Gravação.** "Salvar cópia" grava `/Highlight`, `/Underline`, `/Strikeout` e `/Text` numa carga nova dos bytes originais e depois chama `regenerate_content` (`engine.rs:712-814`). A nota vira um ícone `/Text` de 12 pt (`engine.rs:28`, `engine.rs:779-801`).
- **Leitura de volta.** `annotations_from_doc` é `#[cfg(test)]` (`engine.rs:829-877`). O app não importa anotações de um PDF aberto. Depois de salvar e reabrir, nenhuma marcação, de nenhum tipo, volta a ser editável ou móvel no Tsuro. Isso vale para o bloco manuscrito também.
- **Exportação para Markdown.** `annotations_markdown` pula toda marcação sem trecho citado (`session.rs:425-432`, `if quote.is_empty() { continue; }`). Um bloco solto, sem texto do PDF por baixo, **sumiria em silêncio da exportação**. Isso já fere o critério do #135 ("nunca perda silenciosa").

## Opções de representação no PDF

| Opção | Como | Manuscrita em outros leitores? | Editável em outros leitores? | API na pdfium-render 0.8.37 |
|---|---|---|---|---|
| A. `/FreeText` | `create_free_text_annotation(text)` + `set_bounds` | Não. O Pdfium gera o `/AP` com fonte padrão [INFERENCE: gerador de AP do Pdfium para FreeText usa Helvetica], e não dá para anexar objetos | Sim, como caixa de texto | Existe (`annotations.rs:213-225`). Objetos de anotação são só leitura fora de Ink e Stamp (`annotation/objects.rs:27-29`) |
| B. `/Stamp` com objeto de texto e TTF embutida | `create_stamp_annotation` + `objects_mut()` com `PdfPageTextObject` usando a fonte de `fonts_mut().load_true_type_from_bytes` + `set_contents(text)` | Sim, se o `/AP` gerado a partir dos objetos for respeitado [INFERENCE, precisa do spike] | Não como texto. O texto puro fica em `/Contents` para busca e exportação | `create_stamp_annotation` (`annotations.rs:328`), `objects_mut` (`stamp.rs:48`), `load_true_type_from_bytes` (`document/fonts.rs:535`) |
| C. `/AP` escrito à mão | `FPDFAnnot_SetAP` com stream e recurso de fonte | Sim | Não | Só o binding cru (`bindings.rs:4103-4112`). Montar o dicionário de recursos da fonte não é exposto. Descartada |
| D. Achatar no conteúdo da página | Objeto de texto direto na página | Sim | Não | Viola "não mudar o PDF original" em espírito e mistura com o conteúdo do autor. Descartada |

Recomendação: **B**, com o texto também em `/Contents`. **A** fica como alternativa se você preferir que outros apps editem a nota e aceitar perder a letra manuscrita fora do Tsuro.

## Renderização e interação no iced 0.13

- **Fonte na tela.** O iced aceita fonte embutida por bytes (`application(...).font(bytes)`) e `canvas::Text` com `font` e `size`. O app já usa `Font` com nome em `view.rs:2366`.
- **Zoom.** O bloco vive em espaço de página (pt), como o `marker`. Na tela, o tamanho da fonte é `pt × escala`.
- **Rotação.** Risco. [INFERENCE: no iced 0.13, `fill_text` do canvas não gira o texto junto com a transformação do frame.] Com `view_rotation` diferente de 0, o texto pode não acompanhar a página. O spike precisa testar 90°. Plano B: na v1, mostrar o bloco na horizontal e girar só a posição.
- **Edição.** Reaproveitar o `text_editor` do post-it, posicionado sobre o bloco e sem borda, com a fonte manuscrita. Enter quebra linha. Esc e clique fora confirmam (mesmo fluxo de `NoteSave`).
- **Arrasto.** Reaproveitar o `NoteDrag` (`session.rs:557-5837`). Hoje ele move só o ícone. Para o bloco, move a âncora inteira. O hit-test usa o retângulo do bloco medido em pt.
- **Teclado.** Obrigatório pelos gates do roadmap. Com o bloco em foco, as setas movem 1 pt e Shift+seta move 10 pt. Enter edita e Delete remove com confirmação.

## Fonte

- **Candidatas com licença OFL e acentos do PT-BR** [INFERENCE: licenças e cobertura conferidas de memória, não baixadas]: Caveat, Kalam, Patrick Hand, Indie Flower.
- **Custo.** Um peso estático tem uns 50 a 150 KB [INFERENCE]. Embutir no binário e em cada PDF salvo é aceitável para "leve", mas precisa ser medido.
- **Dependência.** Embutir uma fonte é um asset, não um crate. Pelo `AGENTS.md`, "no new deps without asking" fala de dependências. Mesmo assim, uma fonte de terceiros com licença própria merece a sua aprovação explícita e o arquivo `OFL.txt` no repo.

## Interações e riscos

- **Missão.** É anotar, não editar. O risco é virar "preencher PDF digitando" (formulários estão fora da missão). Guarda: o bloco é uma camada de anotação, com uma fonte só, um tamanho e uma cor, sem caixa de formulário.
- **Modelo de dados.** O `Annotation` atual exige `range` e `quads`. Um bloco solto não tem trecho. Enfiar o bloco nesse struct cria estados inválidos (range vazio fingindo ser trecho). Proposta: separar a âncora num tipo `Anchor::Text { range, quads } | Anchor::Point([f32; 2])`, ou criar um tipo de marcação próprio com `at` + `text`.
- **Exportação (#135).** Precisa de uma linha própria ("p.3 — Nota livre: …"). O `continue` atual a perderia.
- **Lista no painel (#129).** Um item sem trecho mostra a nota no lugar da citação.
- **Recuperação (#128).** É mais um tipo de operação no diário. Mover vira edição. Pede ID estável (o mesmo gate do #128).
- **Assinaturas (#146).** Igual às marcações atuais: "Salvar cópia" grava uma cópia nova. Sem mudança em `tsuro-sign`.
- **Impressão (#148).** Segue a mesma decisão pendente de incluir ou não as marcações da sessão.
- **Rotação.** Ver acima.
- **Reabrir.** O bloco salvo não volta a ser móvel no Tsuro, porque o app não importa anotações. É uma limitação que já existe para todos os tipos.

## Proposta de v1

1. **Spike (P, 1 dia).** Num `/tmp`, gravar um `/Stamp` com texto em Caveat embutida num `guia-folio.pdf` copiado. Abrir no Preview, no Chrome e no Acrobat, se houver. Reabrir no Tsuro. Testar `fill_text` com rotação de 90°. Veredito: B funciona, cai para A, ou adia.
2. **Modelo.** Separar a âncora (`Anchor::Point`) e incluir a nota livre no `AnnotKind`.
3. **Tela.** Ferramenta "Escrever" na barra. Clique na folha cria o bloco, digitar escreve, arrastar ou usar as setas move, Delete remove.
4. **Gravação.** `/Stamp` + texto em `/Contents`.
5. **Exportação e lista.** Linha própria, sem perda silenciosa.
6. **Testes.** Criar, mover por arrasto e por teclado, editar, remover, salvar cópia e ler de volta o `/Stamp` com `/Contents` e `/Rect` corretos (via `annotations_from_doc`), e exportar.

Fora da v1: escolher fonte, tamanho ou cor; importar blocos de PDFs já salvos; escrever à mão livre com caneta ou mouse (`/Ink`), que é outro recurso.

Tamanho: G, com o spike em P na frente.

## Decisões para o usuário

1. **Convive ou substitui?** O bloco manuscrito convive com o post-it ou o substitui? Recomendo conviver na v1: o post-it ancora num trecho e o bloco é livre.
2. **Formato no arquivo.** B (letra manuscrita em todo leitor, sem edição em outros apps) ou A (editável em outros apps, letra comum fora do Tsuro)?
3. **Fonte.** Qual (Caveat é a sugestão) e aprovação para embutir o asset OFL.
4. **Sprint.** Entra nesta sprint como spike, ou vira issue para a próxima?

## Evidências

- Lidos: `session.rs:412-510` (modelo e exportação), `session.rs:523-566` (`NoteDraft`, `NoteDrag`), `session.rs:5823-5837` (`move_note_marker`), `engine.rs:712-901` (gravação e leitura de volta), `view.rs:1557-1562` e `view.rs:3083-3132` (editor e canvas).
- API da pdfium-render 0.8.37 em `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pdfium-render-0.8.37/`: `src/pdf/document/page/annotations.rs:213` (FreeText), `:250` (Ink), `:328` (Stamp), `:358` (Text); `annotation/objects.rs:27-29` (objetos só em Ink e Stamp); `annotation/stamp.rs:48` e `ink.rs:48` (`objects_mut`); `document/fonts.rs:535` (`load_true_type_from_bytes`); `bindings.rs:4103-4112` (`FPDFAnnot_SetAP`).
- Assets atuais: só SVGs em `crates/tsuro/assets/`, nenhuma fonte embutida.
- Busca de issues relacionadas (`manuscrit`, `handwrit`, `FreeText`, `caneta`): nenhuma.
- Não executado: nenhum build, nenhum protótipo, nenhum teste em outro leitor.
