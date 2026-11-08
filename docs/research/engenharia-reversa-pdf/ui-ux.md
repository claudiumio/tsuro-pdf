# UI e UX dos leitores de PDF: padrões e mecanismos para o TsuroPDF

Pesquisa de 13 de outubro de 2026. Continua `relatorio.md` (funções) e olha para interface e interação. Análise de capturas e manuais oficiais, não de aplicativos em execução. Recomendação de produto, não especificação.

## Conclusão

Os leitores mais próximos da missão do Tsuro resolvem três problemas de interface que o Tsuro ainda não resolve por completo. Eles dão um lugar único para reencontrar o que foi marcado, uma paleta que executa qualquer comando e um jeito de reduzir o chrome sem esconder o documento.

Minha recomendação é investir, nesta ordem, em oito mecanismos pequenos. Os três primeiros têm custo baixo ou médio e evidência forte.

1. Completar a paleta com os comandos que já têm atalho e ficaram de fora.
2. Criar uma aba **Marcações** no painel lateral, com busca e salto.
3. Acrescentar o escopo de marcações à paleta, reaproveitando os dados da aba.

Os mecanismos 4 a 8 (cor e modo contínuo, modo foco, imprimir sem marcações, prévia de link e régua de leitura) estão na seção 5. Dois deles exigem protótipo antes de qualquer decisão.

O Tsuro já tem boa parte da base. Há HUD flutuante, barra de seleção com quatro ações, paleta com seis tipos de item, abas de documentos, histórico, desfazer e impressão com prévia. Os mecanismos abaixo estendem essa base. Nenhum pede nova suíte de ferramentas.

## 1. Método e limites

**O que observei.** Vi cinco capturas oficiais de documentação e baixei cada uma para uma pasta temporária fora do repositório, porque não tenho licença para redistribuí-las. Cada captura mostra a interface de SumatraPDF (paleta), Sioyek (marca visual e visão geral de link), Zotero (leitor) e Okular (barra de anotações). A data de cada captura varia. A do Zotero tem metadado de março de 2022.

**O que li.** Li manuais e páginas oficiais de Acrobat, Foxit, PDF-XChange, PDF Expert, Nitro, Bluebeam, PDFelement, Skim, Zathura, GNOME Papers, Preview, Edge e Firefox. Para esses produtos a descrição de interface vem só de texto.

**O que não fiz.** Não executei nenhum concorrente, não fiz teste de usabilidade e não medi tempo nem memória. Não descompilei nada. LiquidText e Xodo ficaram sem evidência de interface nesta rodada, então não derivei padrões deles.

Cada afirmação carrega uma de três etiquetas. **Capturado** significa que vi a imagem. **Manual** significa que li o texto da documentação. **Código** significa que li o trecho do Tsuro nesta sessão. Qualquer coisa fora disso é inferência minha e está marcada como tal.

## 2. A interface do Tsuro hoje

Fonte. Código lido em `crates/tsuro/src/view.rs` e `session.rs`. Não executei o aplicativo.

| Elemento | Onde | Observação |
| --- | --- | --- |
| Barra superior de 36px e faixa de abas | `view.rs:466` (`topbar`), `view.rs:2434` (`tab_strip`) | Base do chrome Kiri. |
| HUD flutuante | `view.rs:202` (`hud`) | Pílula centrada, a 24px do rodapé. Tem página anterior e seguinte, `n / total`, ajustar à largura e zoom. As mensagens `HudWake`, `HudTick` e `HudHover` e o comentário em `view.rs:271` indicam auto-ocultação. |
| Rótulo "Modo Anotação" | `view.rs:248` | Aparece no HUD quando há seleção viva ou marcações na sessão. |
| Barra da seleção | `view.rs:1390` (`selection_bar_layer`) | Quatro ações (destacar, sublinhar, riscar, nota) com atalhos H, U, S e N. |
| Nota | `view.rs:1445` (`note_card`) | Cartão amarelo com editor, cancelar e salvar. |
| Paleta | `view.rs:1824` e `session.rs` (`PaletteItem`) | Seis tipos de item. Ação, trecho do documento, linha do sumário, recente, arquivo da pasta e trecho em outras abas. Não há escopos por prefixo. |
| Painel lateral | `view.rs:2489` (`pages_panel`), `2511` (`panel_tabs`) | Duas abas, miniaturas e estrutura. A escolha é `Message::OutlineTab(bool)`. |
| Assinaturas | `view.rs:3231` (`signatures_panel`) | Painel separado, com ponto de estado. |
| Histórico e desfazer | `Message::HistoryBack`, `HistoryForward`, `AnnotUndo`, `AnnotRedo` | Presentes com atalhos. |
| Impressão | `view.rs:1143` (`print_preview`) e `session.rs:1856` (`print_job_pdf`) | Prévia no diálogo. Com marcações na sessão, a impressão sempre rasteriza uma cópia marcada. |
| Estado não salvo | `session.rs` (`unsaved_mark`) e `Message::CloseDiscard`, `CloseCancel`, `CloseSave` | Marcador na aba e diálogo de fechamento com três saídas. |

O que não encontrei. Procurei no código e não achei painel de marcações, linha de marcação na paleta, favoritos, prévia de link, régua de leitura, página dupla, tela cheia ou tema de papel. Uma busca textual não prova ausência. Já a falta de cor nas marcações eu confirmei lendo a struct `Annotation` (`session.rs:486`).

## 3. Anatomia de interface por produto

A coluna final é uma decisão de escopo para o Tsuro, não um julgamento sobre o produto.

| Produto | Estrutura e interação observadas | Padrão que interessa | Não trazer |
| --- | --- | --- | --- |
| Acrobat Reader | Read Mode (Ctrl/Cmd+H) esconde barras e painéis. Uma barra flutuante semitransparente aparece ao aproximar o cursor da borda inferior. Autoscroll em Ctrl/Cmd+Shift+H. Manual, de um texto de 2016 que pode ter mudado. Em 2022 a Adobe lançou uma interface moderna com alternância para a clássica e um tour de onboarding. Manual. | Chrome some sob demanda e volta por proximidade. | Tour de onboarding e duas interfaces convivendo. A coexistência é minha inferência de custo de manutenção. |
| Foxit | Ribbon com abas como Home, Comment e View. Painel de navegação à esquerda com Bookmarks, Pages, Comments, Attachments e assinaturas, que encaixa ou flutua. F4 alterna os painéis. Barra flutuante ao selecionar texto ou imagem, com copiar, marcar e criar bookmark. Barra flutuante ao criar ou selecionar um comentário. A caixa de busca do título pode recolher num ícone. Manual. | Ações perto da seleção e busca recolhível. | Ribbon, painéis flutuantes e agrupáveis. |
| PDF-XChange | Painéis de comentários e de bookmarks. Paleta de estilos de comentário reutilizáveis. Recurso Thin Lines reduz todas as anotações a 1px para facilitar a leitura de documentos com muitas marcações. Manual. | Estilos nomeados e reutilizáveis. Uma forma de ver o texto sob marcações densas. | Mais de 400 funções e edição de estrutura. |
| PDF Expert | Um painel esquerdo reúne bookmarks, estrutura e resumo de anotações. As anotações ficam ordenadas por página, com busca, filtro por cor e exportação. Abas de topo Annotate e Edit, que o manual manda desativar durante a leitura. Temas Day, Night e Sepia. Manual. | Um lugar único para o que importa no documento. Modos de topo explícitos. | A aba Edit e a identidade visual. |
| Preview | Selecionar texto e clicar no botão de marcação revela cor e tipo. Um modo de destaque aplica o estilo escolhido a cada nova seleção. View, Highlights and Notes lista tudo na barra lateral. Manual. | Dois caminhos para marcar, um ocasional e um repetitivo. | Gravação automática sem aviso. |
| Okular | Barra de menu, barra principal e barra de anotações, mais título. Quatro linhas de chrome. Barra lateral com quatro abas por ícone (miniaturas, anotações, favoritos, conteúdo) e campo de busca. Botões Show Sidebar e Browse no topo. A barra de anotações abre em F6, usa as teclas de 1 a 9 e fecha com Esc. Quick Annotations tem presets configuráveis. Capturado e manual. | Presets de marcação, saída por Esc e navegação entre anotações por setas e Enter. | A densidade de chrome. Exigir a ferramenta Browse para clicar em links é custo de modo, inferência minha. |
| SumatraPDF | Paleta em Ctrl+K com prefixos para comandos, histórico, abas, sumário, miniaturas, anotações, configurações e favoritos. Uma linha de chips mostra os escopos. As linhas têm tipo, conteúdo em cinza e página à direita. A barra de ferramentas tem layout customizável e F8 alterna mostrar, sobrepor e ocultar. A aba ganha um ponto vermelho quando há alterações não salvas. A tecla a marca o trecho selecionado. Capturado e manual. | A paleta como superfície única de comandos e de navegação. Chrome em três estados. | Configuração só por arquivo de texto e sintaxe de filtro como caminho principal. |
| Sioyek | Quase nenhum chrome. Uma linha de status monoespaçada no rodapé mostra página, capítulo e modo ativo. A visão geral de um link abre como retângulo branco sem moldura sobre o texto. A marca visual tinge a linha atual e escurece o resto. Histórico entre documentos como um navegador. Destaques tipados pesquisáveis. Capturado e manual. | Consultar uma referência sem sair do lugar. Mostrar o nome do modo ativo em texto curto. Marcador da linha atual. | Vocabulário de teclas do vim e configuração em arquivo. |
| Skim | A barra de leitura acompanha a linha e se move com Cmd mais as setas. Snapshots mantêm um trecho visível numa tabela do painel de notas e não são gravados no PDF. Manual. | Referência temporária e explicitamente temporária. | Painel de notas como espaço de trabalho. |
| Zotero | Três colunas. A barra lateral esquerda tem três abas por ícone, um campo de busca de anotações e cartões de anotação. Cada cartão traz a página, uma faixa de cor, a citação e o comentário. Três bolinhas de cor no rodapé parecem um filtro por cor, o que é inferência minha a partir da captura. A barra de ferramentas tem destaque, nota, área e um seletor de cor. Capturado e manual. | Lista de marcações em cartões, com cor como filtro. | O painel de notas à direita e a biblioteca. |
| Zathura | Teclado primeiro. A tecla f numera os links da tela e digitar o número os segue. Tab abre o índice. Recolorir em Ctrl+R. Manual. | Seguir link sem mouse. | Memorizar comandos para tarefas básicas. |
| GNOME Papers e Evince | Barra de cabeçalho com campo de página. O painel em F9 tem botões de miniaturas, estrutura, anotações e favoritos. F11 abre a tela cheia e Esc sai. Contínuo e dual ficam no menu. Manual. | Painel único com quatro visões e saída de tela cheia por Esc. | Duplicar a mesma navegação em mais de um painel. |
| Edge | Barra do PDF fixável no topo, com o sumário no extremo esquerdo. Destaque pelo menu de contexto da seleção, com cor. Caret browsing em F7. Leitura em voz alta destaca o trecho lido. O Immersive Reader destaca uma, três ou cinco linhas. Manual. | Marcar pelo menu de contexto e marcar a linha lida. | Resumo por IA e tradução em painel lateral. |
| Bluebeam, Nitro e PDFelement | O Bluebeam tem painel horizontal inferior com tabela de marcações. Selecionar uma linha leva à marcação e as setas percorrem a lista. O Nitro gera um resumo com balões na margem e uma tabela. O PDFelement gera um resumo com layout configurável. Manual. | Percorrer as marcações com setas e saltar. | Colunas, status e campos personalizados. |

## 4. Padrões de UX que se repetem

| Padrão | Quem faz | O Tsuro hoje | Veredito |
| --- | --- | --- | --- |
| Seleção revela ações no local | Foxit, Preview, Edge | Já tem (`view.rs:1390`) | Estender com cor e modo contínuo (5.4) |
| Um comando, várias superfícies | Sumatra | Parcial. Atalhos e paleta não cobrem os mesmos comandos | Adaptar (5.1) |
| Paleta com escopos | Sumatra | Paleta mista, sem escopos | Adaptar (5.3) |
| Painel único com abas por ícone | Okular, Zotero, Papers, PDF Expert | Duas abas num booleano, assinaturas à parte | Adaptar (5.2) |
| Lista de marcações em cartões | Zotero, PDF Expert, Okular, Preview, Bluebeam | Não encontrei no código. Há só a cópia em Markdown | Adaptar (5.2) |
| Cor como categoria e filtro | Zotero, PDF Expert, Sioyek | `Annotation` não tem campo de cor (`session.rs:486`) | Adaptar com cautela (5.4) |
| Chrome em três estados | Sumatra, Acrobat, Papers | HUD flutuante. Modo foco planejado na fase 6 do Kiri | Adaptar (5.5) |
| Prévia de referência sem sair do lugar | Sioyek, Skim | Há histórico. Não encontrei prévia | Protótipo (5.7) |
| Marcador da linha atual | Sioyek, Skim, Edge | Não encontrei | Protótipo (5.8) |
| Nome do modo ativo em texto | Sioyek, Okular | Rótulo "Modo Anotação" existe | Verificar o significado (seção 6) |
| Estado não salvo e destino explícito | Sumatra | Já tem marcador de aba e diálogo de fechamento | Só conferir os rótulos |
| Tema do papel | PDF Expert, Okular, Zathura | Ausente, por decisão do Kiri | Conflito de princípio (seção 6) |
| Página dupla | Okular, Papers, Edge | Existe a issue de modos de página. O README cita página única e contínua | Em andamento |
| Imprimir com ou sem marcações | Acrobat, Foxit | Sempre com marcações | Adaptar (5.6) |

## 5. Mecanismos propostos

Cada mecanismo descreve o problema, o comportamento, a forma dos dados e o aceite. Os arquivos citados foram lidos nesta sessão. A ordem combina valor, custo e incerteza. O esforço é relativo, não estimativa de prazo.

### 5.1 Paleta com todos os comandos

**Origem.** A paleta do SumatraPDF executa qualquer comando e também navega. Capturado e manual.

**Problema.** Um comando que tem atalho mas não tem linha na paleta é invisível para quem não decorou o atalho. A paleta do Tsuro oferece nove ações (`PaletteAction::ALL`). `shortcut_hint` em `session.rs` declara atalho para sete comandos que a paleta não lista. São copiar marcações, voltar e avançar no histórico, desfazer e refazer marcação, copiar seleção e apagar marcação selecionada. Os dois últimos só fazem sentido com seleção.

**Comportamento.** Cada comando com atalho ganha uma ação na paleta, com o atalho como dica. Ações que dependem de contexto aparecem desabilitadas ou ocultas fora dele, como `on_home` já faz para o Início. Alternar tema, mostrar assinaturas e trocar o modo de página entram junto.

**Dados e código.** `PaletteAction` em `session.rs` ganha variantes. Um teste estrutural falha quando uma `Message` com `shortcut_hint` não tem `PaletteAction` correspondente. O teste impede que a lacuna volte.

**Aceite.** Todo comando com atalho é alcançável por Ctrl+K. A dica de atalho na linha confere com `shortcut_hint`.

**Esforço e risco.** Baixo. O risco é poluir a paleta, então o teste deve exigir cobertura de comandos, não de toda `Message`.

### 5.2 Aba Marcações

**Origem.** Zotero, PDF Expert, Okular, Preview e Bluebeam. Capturado e manual.

**Problema.** Marcar não ajuda se o usuário precisa rolar o documento para achar de novo. Hoje a única saída da lista de marcações é copiar tudo como Markdown.

**Comportamento.** O painel lateral ganha uma terceira aba. Cada linha é um cartão com a página, o tipo, o trecho e a nota, no estilo do Zotero. A busca filtra por texto. Setas percorrem a lista, Enter salta para a marcação e a marcação fica selecionada na folha. O trecho segue a mesma regra de linha única da paleta. Os testes `palette_title_keeps_rows_single_line` e `outline_title_truncates_long_labels` em `view.rs` já mostram o padrão a reaproveitar, junto de `middle_truncate`.

**Dados e código.** `Message::OutlineTab(bool)` só distingue duas abas. Uma terceira aba pede um tipo `PanelTab { Thumbs, Outline, Marks }` no lugar do booleano. A lista vem de `ready.annotations`, em ordem de página e posição como já faz `annotations_markdown` (`session.rs:411`). A linha usa o mesmo texto que o Markdown exportado, para as duas saídas não divergirem.

**Diferença deliberada.** Sem tabela de colunas e sem status de revisão. Sem painel de notas à parte.

**Aceite.** Com quinhentas marcações, abrir a aba não trava a rolagem do documento, a verificar por medição. Achar uma nota por uma palavra, saltar e voltar com o histórico leva poucos passos e funciona só com teclado.

**Esforço e risco.** Médio. O risco principal é a migração do booleano para o tipo novo em todos os pontos que usam `OutlineTab`.

### 5.3 Marcações na paleta

**Origem.** O modo `*` do SumatraPDF. A captura mostra uma linha de chips com a tecla de cada escopo e linhas com tipo, conteúdo em cinza e página à direita. Capturado.

**Comportamento.** A paleta ganha um tipo de item para marcação, com tipo, trecho, nota e página. A linha segue o formato da captura do Sumatra, escrito com os tokens do Kiri. Um prefixo curto restringe o resultado a marcações. Escopos para sumário e histórico podem entrar no mesmo desenho depois, sem mudar o mecanismo.

**Dados e código.** Um novo `PaletteItem::Mark` ao lado de `DocHit` e `OutlineRow`. A ordem entre fontes já existe em `SourceRank`. Escolher onde marcações entram nela é uma decisão de produto.

**Aceite.** Digitar o prefixo e uma palavra lista só marcações. Enter salta e fecha a paleta. Com o documento sem marcações, o escopo explica isso em vez de ficar vazio.

**Esforço e risco.** Baixo, depois do 5.2. Compartilham a linha e a ordenação.

### 5.4 Cor semântica e modo de marcação contínuo

**Origem.** O modo de destaque do Preview, o seletor de cor do Zotero, os presets do Okular e o filtro por cor do PDF Expert. Manual e capturado.

**Problema.** A barra da seleção marca um trecho por vez. Quem destaca vinte parágrafos repete seleção e clique vinte vezes. Quem quer distinguir "dúvida" de "importante" não tem como.

**Comportamento.** Um modo de marcação contínuo aplica o tipo e a cor ativos a cada seleção terminada. Esc sai do modo, como no Okular. Quatro cores com significado fixo aparecem como chips na barra e viram filtro na aba de 5.2, como as bolinhas do Zotero. O rótulo "Modo Anotação" do HUD passaria a indicar um estado real.

**Dados e código.** `AnnotKind` tem quatro tipos hoje. A struct `Annotation` (`session.rs:486`) guarda `id`, `page`, `range`, `quads`, `kind`, `text` e `marker`, e não tem campo de cor. A cor exige um campo novo, um tipo fechado com as quatro cores (não um valor livre) e a passagem por `save_copy`. O teste `save_copy_highlight_paints_line_band_under_ink` em `engine.rs` mostra que a cópia já pinta uma faixa sob o texto, então a cor entra nesse ponto.

**Aceite.** Marcar dez trechos em sequência com um gesto cada. Desfazer reverte uma marcação por vez. A cor sobrevive a salvar a cópia e a reabrir em outro leitor, a verificar em dois motores.

**Esforço e risco.** Médio a alto. A persistência da cor e o risco de marcar sem querer são as incertezas. O desfazer existente reduz o segundo.

### 5.5 Modo foco com três estados

**Origem.** O F8 do SumatraPDF alterna mostrar, sobrepor e ocultar a barra. O Read Mode do Acrobat esconde tudo e devolve uma barra ao aproximar o cursor. O F11 do Papers abre a tela cheia e Esc sai.

**Comportamento.** Uma tecla percorre três estados do chrome. Mostrado é o atual. Sobreposto mostra a barra superior só ao aproximar o cursor. Oculto deixa a folha e o HUD. O HUD flutuante do Tsuro já cobre a parte do Acrobat. A novidade é a barra superior e a faixa de abas. A spec do Kiri já prevê o modo foco na fase 6, então esta é a forma proposta para ela.

**Dados e código.** Um tipo `ChromeMode { Shown, Overlay, Hidden }` no estado do visor. A altura do chrome entra no cálculo da área visível. Não verifiquei se mudar essa altura força nova rasterização das páginas, então isso é um teste necessário.

**Aceite.** Alternar o modo não muda a página visível nem a posição de rolagem. Esc volta ao estado anterior. O estado persiste entre sessões só se for decidido pelo usuário.

**Esforço e risco.** Médio. O risco é o recálculo de viewport.

### 5.6 Imprimir sem marcações

**Origem.** Acrobat e Foxit oferecem imprimir o documento com ou sem marcações. Manual.

**Problema.** `print_job_pdf` em `session.rs:1856` rasteriza uma cópia marcada sempre que há marcações. Quem quer imprimir o original limpo para entregar a outra pessoa não tem escolha. A hipótese contrária, de que marcações ficariam de fora da impressão, não procede.

**Comportamento.** O diálogo de impressão ganha uma opção para incluir as marcações, ligada por padrão. A prévia reflete a escolha.

**Dados e código.** `PrintSelection` em `print.rs:50` ganha um campo. `print_job_pdf` já ramifica por `annotations.is_empty()`, então o caminho sem marcações existe.

**Aceite.** Com a opção desligada, a página impressa não contém nenhuma marcação da sessão. Com a opção ligada, o resultado é igual ao de hoje.

**Esforço e risco.** Baixo.

### 5.7 Prévia de destino de link

**Origem.** A visão geral do Sioyek e os snapshots do Skim. A captura mostra um retângulo sem moldura sobre o texto, com a lista de referências, e a linha de status no rodapé. Capturado.

**Problema.** Seguir uma nota de rodapé ou uma figura tira o leitor do parágrafo. Voltar depende do histórico.

**Comportamento.** Um gesto sobre um link interno abre uma prévia do destino sobre o texto. Esc fecha e devolve o foco. Enter ou clique na prévia salta e empurra o ponto de origem no histórico que já existe. Começa só por destinos que o PDF já declara. Inferir referências em PDFs sem links é um risco de salto errado e fica de fora.

**Dependências.** Não verifiquei se o motor extrai a geometria dos links nem o custo de renderizar um recorte pequeno. Os dois pontos pedem um protótipo descartável antes de qualquer decisão, porque são fatos medíveis e não preferência.

**Aceite proposto.** Consultar três notas e voltar ao mesmo deslocamento visual. Sem destino, a interface diz isso sem inventar uma página.

**Esforço e risco.** Médio a alto. A latência do recorte é a incerteza principal.

### 5.8 Régua de leitura

**Origem.** A marca visual do Sioyek, a barra de leitura do Skim e o Immersive Reader do Edge, que destaca uma, três ou cinco linhas. Capturado e manual.

**Comportamento.** Recurso opt-in que marca a linha atual e se move por teclado. A largura em uma ou três linhas acompanha o Edge e o Skim. O resto da página pode escurecer, como na captura do Sioyek.

**Observação sobre o Kiri.** A spec pede cores sólidas no chrome e proíbe alfa decorativo. A folha é um bitmap, então a faixa precisa ser uma sobreposição translúcida, como as marcas de busca que `with_marks` (`view.rs:3095`) já desenha. Isso vale para o documento e não para o chrome, mas merece uma decisão explícita.

**Dependências.** Não verifiquei se a camada de texto entrega as linhas com geometria confiável em colunas, tabelas e páginas rotacionadas. O protótipo deve testar um PDF de duas colunas.

**Aceite proposto.** A régua nunca cobre o texto ao ponto de torná-lo ilegível. Funciona em página rotacionada e continua no lugar certo ao mudar o zoom.

**Esforço e risco.** Médio a alto, por causa da geometria de linhas.

## 6. Decisões abertas

**Folha sempre branca.** A spec do Kiri tem essa regra como inegociável. PDF Expert, Okular e Zathura oferecem tema de papel, o que a contraria. Recomendo manter a regra e só reavaliá-la com evidência de leitura noturna real. Se o produto abrir a exceção, ela deve ser opt-in, nunca o padrão, e a impressão e a cópia salva devem manter as cores originais.

**Significado de "Modo Anotação".** `view.rs:248` mostra o rótulo quando há seleção viva ou marcação na sessão. Não executei o aplicativo para ver como ele se comporta. Se o rótulo indica um estado e não um modo, ele sugere algo que o usuário não consegue desligar. Vale um teste rápido na interface real antes de qualquer mudança.

**Cor das marcações.** Confirmei que `Annotation` não guarda cor. O mecanismo 5.4 só avança se o produto aceitar um campo novo no modelo e a regra de que a cópia salva preserve essa cor em outros leitores. A segunda parte precisa de teste em dois motores antes de abrir issue. O modo contínuo pode andar sem cor, com o tipo ativo apenas.

## 7. O que não trazer

- Ribbon, painéis flutuantes e agrupáveis (Foxit), ferramentas aos montes (PDF-XChange) e uma barra de anotações separada com modo Browse obrigatório (Okular).
- Tour de onboarding e duas interfaces coexistentes (Acrobat).
- Painel de notas, biblioteca e canvas de pesquisa (Zotero, LiquidText).
- Configuração só por arquivo de texto e vocabulário de teclas do vim (Sumatra, Sioyek, Zathura). A paleta e as preferências visíveis cobrem o mesmo ganho.
- Resumo e tradução por IA em painel lateral (Edge).
- Tabelas de gestão de revisão com status e campos (Bluebeam, Nitro).

## 8. Como completar a engenharia reversa

Esta rodada cobre documentação e cinco capturas. Uma engenharia reversa completa de interface exige ver cada aplicativo funcionando. Proponho três níveis.

1. **Documentação e capturas oficiais.** Feito para a maioria. Faltam capturas oficiais de Acrobat, Foxit, PDF Expert, Preview e Edge. Essas fontes ficaram só em texto.
2. **Teardown em execução, só open source.** Instalar Sioyek, Zotero e Skim no macOS e fazer as nove tarefas comuns do relatório anterior. Okular, SumatraPDF e Zathura dependem de plataforma, então a disponibilidade precisa ser conferida antes. Peço autorização antes de instalar qualquer aplicativo na máquina.
3. **Teardown dos pagos, por avaliação gratuita onde os termos permitirem.** Os termos de cada teste precisam ser lidos antes. Nada de credenciais de terceiros nem contornar licença.

Medidas por aplicativo e tarefa. Linhas de chrome visíveis. Percentual da área da janela ocupada pelo chrome. Cliques e teclas por tarefa. Tempo até a primeira marca. Número de modos que o usuário precisa lembrar. Se um usuário novo encontra a função sem dica. Na leitura das cinco capturas, contei quatro linhas de chrome no Okular (título, menu e duas barras), duas no Zotero (aba e barra) e uma linha de status no Sioyek. É uma contagem a olho, não uma medição.

## 9. Verificação desta entrega

O princípio Experience First levou a ancorar cada proposta em um problema de quem lê e a rejeitar a contagem de recursos como critério. O princípio Prove It Works levou a ler o código antes de afirmar o que o Tsuro já faz. Isso derrubou uma hipótese minha, a de que a impressão omitia as marcações, e confirmou que o painel lateral usa um booleano para duas abas.

Os trechos citados do Tsuro foram lidos nesta sessão. As cinco capturas foram vistas e ficaram fora do repositório. Nada foi executado, testado ou alterado no código do produto, e nenhum commit foi criado.

## Fontes

Consultadas em 13 de outubro de 2026. Fontes de fornecedores descrevem o que eles declaram oferecer.

- [U1. SumatraPDF, Command Palette](https://www.sumatrapdfreader.org/docs/Command-Palette).
- [U2. SumatraPDF, Customize toolbar](https://www.sumatrapdfreader.org/docs/Customize-toolbar).
- [U3. SumatraPDF, Editing annotations](https://www.sumatrapdfreader.org/docs/Editing-annotations).
- [U4. Sioyek, Usage](https://sioyek-documentation.readthedocs.io/en/latest/usage.html).
- [U5. Zotero PDF Reader and Note Editor](https://www.zotero.org/support/pdf_reader).
- [U6. Okular, Annotations](https://docs.kde.org/stable_kf6/en/okular/okular/annotations.html).
- [U7. GNOME Papers, Moving around a document](https://help.gnome.org/papers/movingaround.html).
- [U8. Microsoft Edge, PDF reader](https://learn.microsoft.com/en-us/deployedge/microsoft-edge-pdf).
- [U9. Microsoft Edge, E-reading tools](https://www.microsoft.com/en-us/edge/learning-center/ereading-tools-for-book-lovers).
- [U10. Adobe, Hidden Gems in Acrobat DC (2016)](https://blog.adobe.com/en/publish/2016/04/18/hidden-gems-in-acrobat-dc-enhance-your-pdf-reading-experience).
- [U11. Adobe, July 2022 updates, modern viewer](https://www.adobe.com/devnet-docs/acrobatetk/tools/Preferences_by_version/Macintosh/07_12_2022.html).
- [U12. Foxit PDF Reader User Manual 2026.2](https://cdn01.foxitsoftware.com/pub/foxit/manual/reader/en_us/foxit-pdf-reader-user-manual-2026.2.pdf).
- [U13. PDF-XChange Editor V11, Features Overview](https://help.pdf-xchange.com/pdfxt11/features-overview.html).
- [U14. PDF Expert, View, search, and remove annotations](https://support.readdle.com/pdfexpert/en_US/annotate-pdfs/view-and-export-annotation-summary).
- [U15. PDF Expert, Reading mode](https://support.readdle.com/pdfexpert/en_US/reading-pdfs/reading-mode).
- [U16. Apple Preview, Highlight, underline, and strike out text](https://support.apple.com/guide/preview/highlight-underline-and-strike-out-text-prvw757d4d5b/mac).
- [U17. Skim, Showing the reading bar](https://skim-app.sourceforge.io/manual/SkimHelp_30.html).
- [U18. Skim, Taking snapshots](https://skim-app.sourceforge.io/manual/SkimHelp_23.html).
- [U19. Zathura](https://pwmt.org/projects/zathura/).
- [U20. Bluebeam, Markups List](https://support.bluebeam.com/user-manual/menus/window/markups-list.html).
- [U21. Nitro PDF Pro, Comment Tools](https://www.gonitro.com/user-guide/pro/article/combine-comments-from-multiple-files).
- [U22. Adobe, Basic PDF printing tasks](https://helpx.adobe.com/acrobat/using/basic-pdf-printing-tasks-challenger.html).
- [U23. The Okular Handbook, navegação e lista de anotações](https://docslib.org/doc/9029042/the-okular-handbook). Espelho de terceiros do manual oficial.
- [U24. Microsoft Edge, página do leitor de PDF](https://www.microsoft.com/en-us/edge/features/pdf-reader). Sumário no extremo esquerdo da barra.
- [U25. PDF-XChange, lista de recursos, Thin Lines](https://www.pdf-xchange.com/product/pdf-xchange-editor/features-group/108/features).
- [U26. PDFelement, Comment Summary no Windows](https://pdf.wondershare.com/guide/use-comment-summary.html).

Capturas oficiais vistas e mantidas fora do repositório.

- [C1. SumatraPDF, paleta no modo de anotações](https://files.sumatrapdfreader.org/assets/sumatrapdf/docs/img/command-palette-annotations-f38d.png).
- [C2. Sioyek, marca visual](https://sioyek-documentation.readthedocs.io/en/latest/_images/visual_mark.png).
- [C3. Sioyek, visão geral de link](https://sioyek-documentation.readthedocs.io/en/latest/_images/overview.png).
- [C4. Zotero, leitor de PDF](https://www.zotero.org/static/images/blog/6.0/pdf-reader.jpg).
- [C5. Okular, barra de anotações](https://docs.kde.org/stable_kf6/en/okular/okular/annotations.png).
