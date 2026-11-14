# Fundação visual do Tsuro: base macOS, paridade Windows e Linux

Estado: aprovada em 2026-11-01 — decisões D-1..D-6 ratificadas com os valores finais na seção 10. Fontes: Apple macOS 27 UI Kit (Sketch, 4.683 símbolos, 110 variáveis de cor, 67 estilos de texto) e Human Interface Guidelines. Valores marcados `[kit]` saíram do inspector do kit; `[HIG]` da documentação da Apple; `[INF]` são inferência nossa.

## 1. Princípios

1. **A moldura segue a plataforma, o papel é nosso.** Chrome imita o sistema (macOS hoje, Windows/Linux depois); a página continua branca em qualquer tema.
2. **Uma fonte, uma escala.** Não inventamos tamanhos: usamos a escala da Apple e substituímos só a família tipográfica, porque SF Pro não pode ser distribuída.
3. **Cinza é estrutura, cor é significado.** Fundo, borda e texto saem da paleta neutra; cor só em ação primária, seleção, marcação e estado.
4. **Nada de efeito que não conseguimos desenhar.** Sem Liquid Glass real nem vibração (o iced não amostra o desktop atrás da janela). Usamos superfícies chapadas com o mesmo raio, sombra e hierarquia do sistema.
5. **Cor nunca é a única informação.** Todo estado (aviso, erro, seleção) leva ícone ou texto junto `[HIG]`.

## 2. Tipografia

SF Pro não pode ser embarcada `[HIG]` e não pode ir para Windows/Linux (licença). Proposta: **Inter** (OFL 1.1), com as métricas da tabela abaixo. Em macOS, se quisermos o rosto do sistema, resolvemos em runtime e caímos para Inter quando não existir — decisão D-1.

| Estilo | Tamanho / entrelinha | Peso | Uso no Tsuro |
|---|---|---|---|
| Title 2 | 17 / 22 | 400 (600 enfático) | Título de cartão grande, nome do PDF no Início |
| Title 3 | 15 / 20 | 400 (600) | Cabeçalho de seção do Início |
| Headline | 13 / 16 | **700** | Título de alerta, nome da aba ativa |
| Body | 13 / 16 | 400 (600) | Padrão: linhas de painel, menu, botão, campo |
| Callout | 12 / 15 | 400 (600) | Texto secundário em cartões |
| Subheadline | 11 / 14 | 400 (600) | Rótulo de seção da sidebar (700), dica de atalho |
| Footnote | 10 / 13 | 400 (600) | Rótulo de grupo, contador |

- Piso de 10 pt, padrão 13 pt `[HIG]`. Hoje o app usa 10, 11, 12, 13, 14 e 16 soltos, sem par tamanho/peso.
- Não usar peso abaixo de Regular `[HIG]`. Sem caixa alta em rótulo de seção (o sistema atual usa capitalização de título).
- Tracking da SF não se aplica à Inter `[INF]`: copiamos tamanho e entrelinha, não o tracking.

## 3. Cor

Papéis semânticos, não nomes de cor `[HIG]`. Valores do kit `[kit]`.

| Papel | Claro | Escuro | Onde |
|---|---|---|---|
| `label` (texto primário) | #000 85% | #FFF 100% | todo texto |
| `label_2` (secundário) | #000 50% | #FFF 55% | dica, subtítulo |
| `label_3` (terciário) | #000 25% | #FFF 25% | atalho, desabilitado, cabeçalho de sidebar |
| `fill_1` (superfície de controle) | #000 10% | #FFF 10% | botão, campo, segmento |
| `fill_3` | #000 5% | #FFF 5% | separador, trilho |
| `separator` | #3C3C43 29% | #FFF 10% | hairline de 1 px |
| `window_bg` | #FFFFFF | #1E1E1E | fundo da janela |
| `under_page` | #EFEFED (nosso) | #181818 (nosso) | mesa atrás da folha |
| `accent` | #08F → #0078F0 (vibrante) | #0091FF → #0A99FF | ação primária, seleção, foco |
| `red` / `green` / `orange` | #FF383C / #34C759 / #FF8D28 | #FF4747 / #3BDB63 / #FF9E33 | erro, ok, aviso |

- **Exceção declarada:** o papel fica #FFFFFF nos dois temas (regra "folha sempre branca"). A HIG manda suavizar branco no escuro; resolvemos escurecendo só a mesa, nunca a folha.
- **Contraste:** 4.5:1 até 17 pt, 3:1 acima disso ou em negrito, 7:1 desejável em texto pequeno `[HIG]`. O `warn` atual (#d4a017) dá ~2.4:1 sobre branco e reprova.
- Acento do sistema: quando o usuário escolhe outro acento no macOS, o sistema troca o nosso. Ler `NSColor.controlAccentColor` é possível, mas fica para depois (D-4).

## 4. Espaço, raio e elevação

- **Grade de 4 px.** Espaçamentos: 4, 8, 12, 16, 20, 24.
- **Alturas de controle:** 20 (mini), 24 (regular), 28 (padrão de botão de alerta), 36 (grande). Área de clique mínima 20×20, padrão 28×28 `[HIG]`.
- **Padding:** ~12 pt em volta de controle com bezel, ~24 pt sem `[HIG]`.
- **Raios `[kit]`:** 4 (mini), 6 (botão/campo/segmento de conteúdo), 8 (item de sidebar, item de menu, seleção), 9 (aba), 12 (menu), 16 (janela), 20 (popover), 26 (folha de diálogo), ∞ (pílula: botão de barra, alerta, campo de busca, trilho).
- **Concêntrico:** raio interno = raio externo − padding `[HIG]`.
- **Sombras `[kit]`:**
  - janela: `0 0 0 1px rgba(0,0,0,.2)`, `0 16px 33px rgba(0,0,0,.4)`
  - menu: `0 0 0 .5px rgba(0,0,0,.2)`, `0 4px 24px rgba(0,0,0,.3)`
  - popover: `0 18px 46px rgba(0,0,0,.25)` + hairline
  - tooltip: `0 0 2px rgba(0,0,0,.4)`, `0 2px 8px rgba(0,0,0,.2)`
- **Scrim** de modal: preto 25% (hoje há dois valores, 0.25 e 0.55, repetidos em sete lugares).

## 5. Componentes

### Janela e barra de título
- Janela: raio 16, hairline de 1 px, sombra acima.
- **Título unificado** (barra + título na mesma faixa): altura **46** (botão 28 + 8 de respiro em cima + 10 embaixo, que põe os controles na linha dos semáforos e separa a linha do progresso), padding 8 à esquerda mais a reserva dos semáforos, gap 6 `[kit]`. Compacto: **40**, padding 13/8. Expandido: título 33 + barra 44.
- No macOS: `fullsize_content_view` + `titlebar_transparent` + `title_hidden`, deixando os semáforos nativos. Nunca desenhar semáforo falso `[HIG]`.
- Título: 13/15 w700; subtítulo 11/13 w500 terciário ("— Editado"). Máximo ~15 caracteres `[HIG]`; nome do app nunca.
- Windows/Linux: barra de título nativa do sistema; nossa faixa de 44/40 começa abaixo dela.

### Barra de ferramentas
- Três zonas `[HIG]`: **início** (alternar painel, título), **centro** (controles comuns, colapsam em excedente), **fim** (busca, menu ⋯, ação primária).
- Máximo 3 grupos; grupos separados por espaço fixo; ícones monocromáticos, sem bezel; no máximo uma ação primária, à direita.
- Botão de barra: 28×28, raio ∞, ícone 16; grupo de botões: 36 de altura, padding 4, gap 9 `[kit]`. Versão compacta: 20×20 em 24.
- Todo item da barra também existe no menu `[HIG]`.

### Abas de documento
- Botão de aba do kit: **44×26, raio 9**, fundo translúcido e desfoque `[kit]`. Nossa tira hoje tem 30 px e pílula — aproximar de 26 + raio 9 + gap 4.
- Aba ativa com peso 600 e fundo `fill_1`; inativa sem fundo, texto `label_2`.

### Sidebar / painel lateral
- Largura **256** (nossa: 216/220), padding 14 no topo/baixo e 16 à esquerda `[kit]`.
- Item: **32** de altura (small 24, large 40), raio 8, padding 14 à esquerda, ícone 16, texto 13/16 w400, detalhe 13/16 w500 secundário `[kit]`.
- Selecionado: fundo `fill` 11% com raio 8. Cabeçalho de seção: 11/14 w700 terciário, 18 de altura.
- Sem borda de cartão: a sidebar vai de ponta a ponta e o conteúdo passa por baixo `[HIG]`. Nossa sidebar é um cartão com hairline.
- Esconder/mostrar sempre disponível; nunca escondida por padrão.

### Controle segmentado
- Conteúdo: 24 de altura, raio 6, fundo 8%, segmentos iguais, separadores 1×14 `[kit]`. Grande: 36 de altura, raio ∞.
- Selecionado: pílula elevada; texto/ícone, nunca os dois juntos; 5–7 segmentos no máximo `[HIG]`.
- Trocar de área principal é aba, não segmento `[HIG]`. Nosso painel usa segmento para Páginas/Sumário/Marcações: aceitável como "troca de painel", mas os rótulos precisam caber (hoje forçam 216 px).

### Campo de busca
- 24 de altura, raio ∞, fundo branco, borda 1 px 5%, padding 8/0/4, ícone 16, texto 13/16 `[kit]`.
- Foco: anel de 0.5 px dentro + 3.5 px fora na cor de acento `[kit]`.
- Filtra ao digitar, botão limpar à direita, placeholder diz o escopo.

### Botões
- Conteúdo: 24 de altura, raio 6, padding 16/4, rótulo 13/16 w500; fundo 8%, pressionado 16%, desabilitado 4% com texto terciário `[kit]`.
- Primário: fundo no acento, texto branco. Destrutivo: vermelho. Só um primário por tela.
- Diálogo/alerta: 28 de altura, raio ∞, padding 8, gap 8; Cancelar à esquerda, ação preferida à direita `[HIG]`.
- Mini: 16 de altura, raio 4, rótulo 10/12.

### Menu
- Contêiner: raio **12**, padding 16/6, vidro #E6E6E6 55% (claro) / #5A5A5A 50% (escuro) `[kit]`; sem vidro real, usamos superfície elevada opaca.
- Item: **24** de altura, gap 8, rótulo 13/16 w500, atalho 13/16 terciário `[kit]`.
- Hover: fundo de acento com **raio 8**, ocupando a largura toda `[kit]`.
- Separador: 11 de altura com linha de 1 px; cabeçalho de grupo 13/16 w700 secundário.
- Itens indisponíveis: esmaecidos no menu de barra, **escondidos** no menu de contexto `[HIG]`.

### Popover, tooltip, rolagem, progresso
- Popover: raio **20**, sombra `0 18px 46px`, um por vez, fecha ao clicar fora `[kit]`/`[HIG]`.
- Tooltip: 19 de altura, padding 6/3, 11/13 w500, fundo #E7E7E7 85% / #262626 80% `[kit]`.
- Barra de rolagem: trilho 12, polegar **6** de largura, raio ∞ `[kit]`.
- Progresso: linear **6** (small) ou 10 (regular) de altura, trilho 6% + hairline 0.5 px, preenchimento no acento `[kit]`. Nosso progresso tem 2 px.
- Rolagem automática só o necessário para revelar o item `[HIG]` (vale para busca e marcações).

### Diálogos e alertas
- Alerta: largura 260, padding 16/20/16/16, gap 14, ícone 72, título 13/16 w700, descrição 13/16 w400, até 3 botões `[kit]`/`[HIG]`.
- Folha de diálogo (nosso cartão de Configurações, salvar cópia, recuperação): raio **26**, padding 20, gap 20, largura 390 `[kit]`; no macOS desenhamos como folha centrada com scrim de 25%.
- Botão padrão nunca é Cancelar; Esc e Cmd+. dispensam.

## 6. Onde isso bate no app de hoje

| Superfície | Hoje | Alvo |
|---|---|---|
| Barra superior | 36 px abaixo do título nativo, tudo num nível | 46 px unificados no macOS, tudo numa linha só com os semáforos, zonas início/centro/fim, ícones 16, botões 28 |
| Tira de abas | 30 px, pílula | 26 px, raio 9, ativa com peso 600 |
| Painel lateral | cartão 216/220 com hairline, linha de 41/62 | sidebar 256, linha 32, raio 8, cabeçalho 11/14 w700, de ponta a ponta |
| Segmentado Páginas/Sumário/Marcações | largura fixa por texto | 24 px, raio 6, larguras por conteúdo |
| Busca | campo próprio, sem anel de foco | 24 px, pílula, anel de acento, limpar |
| Menu ⋯ | raio 8, item ~30, sombra própria | raio 12, item 24, hover de acento raio 8, atalho terciário |
| Diálogos | cartão de 440/620 px, scrim 25%/55% | folha raio 26, scrim 25%, botões 28 pílula, primário à direita |
| Tooltip | 13 px, padding 4/8 | 11/13, 19 de altura, padding 6/3 |
| Progresso | 2 px | 6 px |
| Ícones | SVG em `currentColor`, tingido pelo token `accent` do tema | monocromático por token, com estado claro/escuro |
| Tema | pref `dark` por padrão | `system` por padrão com override (D-2) |
| Fonte | fonte padrão do iced | Inter 13 como base |
| Cores | 18 tokens sem papéis semânticos | papéis semânticos + variantes de contraste |

## 7. O que não dá para copiar (e o substituto)

| Recurso Apple | Por que não | Substituto |
|---|---|---|
| SF Pro | licença só para mockup de UI Apple | Inter (OFL), embarcada |
| SF Symbols | licença só para apps em sistemas Apple | Ori, desenhados por nós, monocromáticos |
| Liquid Glass (refração, blur real) | exige superfície nativa | superfície chapada com raio, sombra e hairline equivalentes |
| Vibração atrás da janela | iced não amostra o desktop | scrim sólido com alfa |
| Barra de menu nativa | iced não desenha | AppKit direto via `objc2-app-kit` (D-3 revisada em 13/11/2026) |
| Painel de impressão do sistema | idem | nosso cartão, com os mesmos campos |
| VoiceOver | iced não expõe árvore de acessibilidade | pendência separada; não prometer |

## 8. Windows e Linux

A mesma estrutura, com valores da plataforma `[INF]` a confirmar na implementação:

| Item | macOS | Windows 11 (Fluent 2) | Linux (GNOME/libadwaita) |
|---|---|---|---|
| Fonte | Inter (ou sistema) | Segoe UI Variable | Adwaita Sans / Cantarell |
| Corpo | 13 | 14 | 11 pt (~14,7 px) |
| Controle | 24/28 | 32 (compacto 24) | 34 |
| Raio | 6/8/12/16 | 4/8 | 6/12 |
| Grade | 4 | 4 | 4 |
| Controles de janela | semáforos à esquerda | botões à direita | à direita |
| Menu | barra do sistema | barra na janela | header bar |
| Atalho | Cmd | Ctrl | Ctrl |

Regra: a estrutura (zonas, hierarquia, escala relativa) é uma só; mudam fonte, altura de controle, raio e lado dos controles de janela. `#[cfg(target_os = ...)]` escolhe o conjunto.

## 9. Fases

1. **Tokens.** Reescrever `kiri.rs` com papéis semânticos, escala tipográfica, espaçamento, raios e sombras; embarcar Inter; tema `system`. Sem mudança de layout. Aceite: nenhuma cor/tamanho literal fora de `kiri.rs` nas superfícies já migradas.
2. **Chrome.** Barra unificada (macOS), zonas, botões 28, tira de abas 26/raio 9, sidebar 256 com linha 32 e raio 8, segmentado 24, busca pílula com anel.
3. **Sobreposições.** Menu raio 12/item 24, popover raio 20, diálogos como folha raio 26 com scrim 25%, tooltip 19, progresso 6.
4. **Plataforma.** Conjuntos Windows/Linux, barra de menu nativa no macOS, glifos de atalho quando o backend permitir.

Cada fase: `cargo test -p tsuro`, `cargo fmt --check`, clippy sem aviso novo, e conferência visual no app antes do PR.

**Estado (2026-11-13).** Fases 1 a 3 implementadas, e a fase 4 começou pela barra
unificada. A fase 2 migrou o chrome para os papéis semânticos (`Roles`), a escala
tipográfica (`TextKind`), a faixa de abas 26/raio 9 com a ativa em `fill_1` e peso
600, a sidebar 256 com padding 14/16 e item de 32, o segmentado de 24 com
separadores de 1×14 e a busca como campo próprio de 24 na zona final da barra
(início: Início/Abrir/título; centro: página, zoom, anotar; fim: busca, painéis,
⋯). A fase 3 trouxe a linha de menu de 24 e a dica 11/13 de 19 de altura; o menu
já estava no raio 12, o popover no 20, a folha no 26, o scrim em 0,475 linear
(que mede 25% na tela) e o progresso em 6.

Os ícones fecharam a dívida da fase 1: os 28 Ori saem em `currentColor` com a
profundidade dos três tons na opacidade (1, 0,78 e 0,56) e o `kiri::ori_icon`
tinge com o token do tema, então o mesmo arquivo serve claro e escuro. Antes o
teal vinha gravado no SVG e o traço mais claro media 1,98:1 sobre branco, que é
o fundo do tema claro.

A barra unificada (fase 4) não precisou de AppKit: o `iced` 0.13 expõe
`title_hidden`, `titlebar_transparent` e `fullsize_content_view` no
`PlatformSpecific` do macOS. Com eles a faixa encosta no topo (sem o respiro
de 4) e fica com 46: o botão de 28 mais 8 em cima e 10 embaixo, o que põe o
centro dos controles a 22 px do topo, logo abaixo dos semáforos nativos
(medidos a 15,5), e separa a linha do progresso. Abaixo do progresso ficam mais
6 px (`CHROME_GAP`) antes de o painel começar. À esquerda ficam 68 reservados
aos três botões, que ocupam de 6 a 65 na medição (`Metrics::unified_title` e
`Metrics::traffic_lights`).
Como o conteúdo corre sob a barra, os semáforos nativos são deslocados para a
linha dos controles (`macos_window::align_lights`, que mede cada botão pela
superview e é idempotente, chamado de novo a cada resize), e os gestos dela
são do sistema: o duplo clique (`performZoom:`) e o arrasto implícito
(`-mouseDownCanMoveWindow` da view) foram desligados, e o arrasto passou a ser
explícito, só nas faixas vazias: os vãos entre os controles
(`view::drag_band`), a fileira de 8 px acima deles, a de 10 abaixo e a área dos
semáforos. Texto e controle não arrastam mais — o `mouse_area` da faixa
inteira pegava o clique no nome do arquivo e no número da página, que não
consomem o evento. A barra de menu do macOS foi feita em AppKit
(`objc2-app-kit`), sem o `muda`: os itens são `NSMenuItem` que enviam a sua
`Message` de volta para a sessão, e o iced não participa. No Windows e no Linux
o menu continua sendo a lista do ⋯ desenhada no iced. Falta da fase 4: o
comportamento de título nas plataformas Windows e Linux, onde a barra nativa
continua acima da nossa faixa.

## 10. Decisões para o Junior

Ratificadas em 2026-11-01: D-1 a D-6 aprovadas com os valores finais abaixo.

| # | Decisão | Recomendação | Alternativa | Valor final aprovado |
|---|---|---|---|---|
| D-1 | Fonte | Inter embarcada em todas as plataformas | resolver SF no macOS e cair para Inter; duas aparências | Inter embarcada em todas as plataformas |
| D-2 | Tema | `system` por padrão, com claro/escuro manuais | manter `dark` fixo (contra a HIG) | `system` por padrão, com claro/escuro manuais |
| D-3 | Barra de menu nativa | sim no macOS via `muda` | só atalhos, sem menu (menos nativo) | sim no macOS, feito com `objc2-app-kit` direto em vez do `muda` (fase 4) |
| D-4 | Acento | teal do Kiri | seguir o acento do sistema (mais nativo, perde a marca) | teal do Kiri fica — escuro #4db8b0 / claro #1f7d76 |
| D-5 | Densidade | regular (24/28) | compacta (20/24), cabe mais na tela | regular (24/28) |
| D-6 | Sidebar | de ponta a ponta, sem cartão | manter o cartão atual | painel flutuante fica: manter o cartão atual |

**Estouro do menu ⋯ (ratificado com a fase 1):** altura máxima com rolagem interna (itens nunca ficam fora da tela), largura 300, e o rótulo longo de "Salvar cópia com marcações…" quebra em vez de ser cortado.

## 11. Referências

- Apple macOS 27 UI Kit (Sketch): https://www.sketch.com/s/57153a31-3379-4737-8ac6-dbfd6525f052/symbols
- macOS get-started: https://developer.apple.com/macos/get-started/
- HIG: https://developer.apple.com/design/human-interface-guidelines/
- Adopting Liquid Glass: https://developer.apple.com/documentation/TechnologyOverviews/adopting-liquid-glass
- Inter (OFL 1.1): https://github.com/rsms/inter
- Coleta bruta desta investigação: `local://ui-research/` (fora do repo)
