# Catálogo de funções candidatas

Este é o escopo fechado do planejamento. São 28 funções em 7 pacotes. Cada pacote trata de um grupo de funções e escreve um arquivo. Você só escreve as funções do seu pacote.

Referências de evidência usam os documentos locais em `docs/research/engenharia-reversa-pdf/`, abreviados `R` para `relatorio.md`, `U` para `ui-ux.md` e `C` para `codigo-e-usuarios.md`. Na ficha, escreva sempre o nome completo do arquivo, como `codigo-e-usuarios.md § 4.1`. A coluna de âncoras traz o ponto de partida no código, que você deve reverificar.

## 1. Funções

| ID | Nome funcional | Codinome | Ficha | Pacote | Evidência local | Evidência complementar do PO | Âncoras no código | Notas do PO |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| F-MARC-01 | Diário de recuperação das marcações | Orime | completa | FA | C § 3 (Perder trabalho sem aviso), C § 4.1 | Nenhuma | `session.rs:486`, `positions.rs:20`, `view.rs:2405` | Nunca altera o PDF do usuário. Guarda só o necessário, em pasta de dados do app. Pense em privacidade das notas |
| F-MARC-02 | Aba Marcações | Fusen | completa | FA | C § 4.3, U § 5.2, R § 5.1 | Nenhuma | `session.rs:2279`, `view.rs:2489`, `view.rs:2511`, `session.rs:411` | Exige trocar `OutlineTab(bool)` por um tipo de aba. Reaproveite a linha única com truncamento do sumário e da paleta |
| F-MARC-03 | Marcações na paleta | Fusen | completa | FA | U § 5.3, C § 2 (Sumatra coleta sob demanda), C § 4.3 | Nenhuma | `session.rs:632`, `session.rs:873`, `view.rs:1860` | Lista coletada sob demanda e reaproveitada, nunca recalculada a cada tecla |
| F-MARC-04 | Marcas na barra de progresso | Shirube | curta | FA | C § 4.3 (ticks na barra) | Sugestão de usuário no Hacker News sobre minimapa de marcações (ver C § 4.3). Hipótese, sem outro leitor que faça | `view.rs:1988` | Tratar como hipótese a validar |
| F-MARC-05 | Cor nomeada e memória da última escolha | Pigmentos | completa | FB | C § 3 (Marcar sem lembrar a cor), C § 4.5, C § 2 (paleta fechada do Zotero), U § 5.4 | `docs/research/kiri-cor-cerebro.md` (cor com sentido, acessibilidade) | `session.rs:486`, `session.rs:475`, `engine.rs:720`, `view.rs:1390`, `kiri.rs:29` | `Annotation` não tem cor. Poucas cores fechadas e nomeadas. Cor nunca é o único sinal, acessibilidade. A cor tem de sobreviver a salvar a cópia e a abrir em outro leitor |
| F-MARC-06 | Marcação contínua | Pincelada | curta | FB | C § 4.5 (modo pegajoso), C § 3 (Nitro e Foxit), U § 5.4 | Nenhuma | `view.rs:202` (rótulo "Modo Anotação"), `view.rs:1390` | Decisão do PO em 13 de outubro de 2026. Rebaixada. Só avança depois de F-MARC-05 e com saída automática |
| F-MARC-07 | Marcadores de página | Dobras | completa | FB | C § 2 (Sioyek: tabelas de marcas e favoritos), R § 4 | GNOME Papers, issue 298, bookmarks como anotação, 9 votos (gitlab.gnome.org/GNOME/Incubator/papers). Skim lista Bookmarks entre os recursos (sourceforge.net/p/skim-app/wiki/Features) | `session.rs:2279`, `view.rs:2511`, `positions.rs:20` | Decida onde o marcador mora (no PDF, como anotação padrão, ou fora dele) e justifique pela interoperabilidade. Ícone de orelha dobrada |
| F-MARC-08 | Exportar marcações para arquivo | Sem codinome | curta | FB | R § 5.4, C § 4.3 (resumo de anotações sem número de página), C § 4.7 | Nenhuma | `session.rs:411` | Evolui o Markdown que já existe. Usa o rótulo de página de F-NAV-02 |
| F-NAV-01 | Voltar depois de um salto | Kaerimichi | completa | FC | C § 3 (Voltar depois de seguir um link), C § 4.6 | Nenhuma | `session.rs:2355`, `session.rs:2356`, `view.rs:202`, `session.rs:5088` | O histórico já existe. O trabalho é a superfície, o chip de voltar e os botões 4 e 5 do mouse |
| F-NAV-02 | Rótulo de página impresso | Fólio | completa | FC | C § 3 (Página do arquivo e página do livro), C § 4.7 | `pdfium-render 0.8.37` expõe `PdfPage::label()` | `session.rs:411`, `view.rs:202`, `session.rs:5431` | Sem rótulos no PDF, nada muda. Decida a regra de "ir para página" entre rótulo e índice |
| F-NAV-05 | Posição que acompanha o conteúdo | Shiori | completa | FC | C § 2 (Sioyek: tabela `document_hash`), C § 5 (Posição de leitura pelo conteúdo) | Nenhuma | `positions.rs:236`, `positions.rs:81`, `positions.rs:20` | Risco de restaurar a página errada se o PDF mudou. Defina a regra de decisão |
| F-NAV-06 | Rolagem natural | Sem codinome | curta | FC | C § 5 (Sensibilidade do trackpad e da roda) | Sioyek, issues 157 (17 votos) e 1039 | Nenhuma | Não há tratamento próprio no código, a confirmar em execução em Mac |
| F-NAV-03 | Copiar como parágrafo | Tsunagi | completa | FD | C § 3 (Copiar texto), C § 4.4 | Nenhuma | `session.rs:5431`, `page.rs:224`, `session.rs:411` | Heurística conservadora. Se tiver dúvida, mantém a quebra. O padrão (parágrafo ou cru) é decisão do PO |
| F-NAV-04 | Paleta com todos os comandos | Sem codinome | completa | FD | U § 5.1, U § 2, C § 2 (Sumatra: paleta coleta sob demanda) | Nenhuma | `session.rs:599`, `session.rs:5088`, `session.rs:1007` | Sete comandos têm atalho e não têm linha na paleta. Inclua um teste estrutural que impede a lacuna de voltar |
| F-NAV-07 | Espiar o destino de um link | Nozoki | curta | FD | U § 5.7, C § 5 (Prévia de link interno), C § 3 | Skim e Evince têm prévia de link. Zathura issue 577 pede | Nenhuma | Depende de S-02 (geometria dos links e custo do recorte) |
| F-NAV-08 | Sumário gerado | Kari-mokuji | curta | FD | C § 5 (Sumário gerado quando o PDF não tem) | Extensão Google Scholar PDF Reader (100 mil usuários, segundo a issue 17921 do pdf.js). Contra-evidência, usuários do PDF Expert dizem que a detecção automática erra | `view.rs:2545` | Sempre rotulado como gerado e provisório |
| F-NAV-09 | Seleção por teclado | Sem codinome | curta | FD | C § 5 (Seleção de texto por teclado) | Zathura issue 748 (19 votos). Edge tem caret browsing | `session.rs:4889` | Alto esforço. Fica para depois |
| F-ARQ-01 | Senha ao abrir | Kagi | completa | FE | C § 3 (Abrir arquivos protegidos), C § 4.2 | Nenhuma | `engine.rs:198`, `engine.rs:279` | A senha nunca é gravada nem registrada em log. Depende de S-05. Gere o PDF de teste com `qpdf --encrypt` se o `qpdf` existir |
| F-ARQ-02 | Salvar sem quebrar assinaturas | Hanko | completa | FE | C § 2 (O pdf.js salva anexando), C § 4.9 | Nenhuma | `engine.rs:396`, `engine.rs:720`, `view.rs:1926` | Mexer aqui toca `crates/tsuro-sign/`, que é "perguntar antes". Depende de S-01. A ficha deve deixar a decisão com o PO |
| F-ARQ-03 | Preparação de impressão leve e fiel | Sumi | completa | FG | C § 3 (Impressão borrada ou errada), C § 4.8 | Decisão explícita de 13 de outubro de 2026. Não aumentar DPI | `print.rs:18`, `print.rs:62`, `spool.rs:35` | Raster mantém 200 DPI. Investigar memória agregada, cancelamento e fidelidade. Preservação vetorial apenas se viável por S-03, sem elevar resolução |
| F-ARQ-04 | Imprimir com ou sem marcações | Sem codinome | completa | FG | U § 5.6, R § 5.6, C § 4.8 | Nenhuma | `session.rs:1856`, `print.rs:50`, `view.rs:1191` | Hoje as marcações da sessão vão sempre para a impressão |
| F-ARQ-05 | Folha de marcações impressa | Sem codinome | curta | FG | U § 3 (Nitro, Bluebeam e PDFelement geram resumos), R § 5.6 | Nenhuma | `session.rs:411`, `print.rs:62` | Usa o rótulo de F-NAV-02 e a lista de F-MARC-02 |
| F-ARQ-06 | Proteção e permissões explicadas | Sem codinome | curta | FE | C § 3 (Abrir arquivos protegidos: Edge e Preview) | Nenhuma | `engine.rs:198` | Não contorna proteção nenhuma. Só explica o estado ao usuário. Depende de S-05 |
| F-ARQ-07 | Clareza de confiança nas assinaturas | Selo | curta | FE | R § 5.7 | Acrobat usa listas de confiança AATL e EUTL (R, fontes S03 e S31) | `view.rs:3231` | Toca `crates/tsuro-sign/`, "perguntar antes". Não promete equivalência com ICP-Brasil |
| F-AMB-01 | Tema de papel oriental | Washi | épico | FF | C § 3 (Ler com conforto), C § 5 (Tema do papel), U § 6 (Folha sempre branca), R § 5.5 | `docs/ui/kiri-spec.md` | `kiri.rs:29`, `view.rs:3095` | Decisão do PO em 13 de outubro de 2026. Fica para depois, e o PO acha a ideia boa. Muda um princípio do Kiri |
| F-AMB-02 | Modo foco | Névoa | completa | FF | U § 5.5, C § 5 (HUD que se esconde) | Sumatra alterna a barra com F8. Acrobat tem Read Mode | `view.rs:202`, `view.rs:466`, `view.rs:2434` | O HUD já some com o mouse parado (#109). O novo é a barra superior e a faixa de abas |
| F-AMB-03 | Régua de leitura | Ito | curta | FF | U § 5.8, R § 5.5 | Nenhuma | `view.rs:3095` | Depende de S-04 (geometria de linhas). A faixa sobre o bitmap precisa de transparência, e o Kiri proíbe transparência decorativa no chrome, não no documento |
| F-AMB-04 | Anti-funções do Tsuro | Pacto do Ma | pacto | FF | C § 3 (Interface que enche de coisas) | PDFgear muda o leitor padrão sem avisar. PDF-XChange estampa marca d'água por engano (C, Fontes) | `session.rs:2277` (leitor padrão explícito), `session.rs:2272` | Cláusulas curtas e verificáveis. Cada uma diz como verificar a cada release |

## 2. Pacotes

| Pacote | Arquivo de saída | Funções |
| --- | --- | --- |
| FA | `FA-orime-fusen.md` | F-MARC-01, F-MARC-02, F-MARC-03, F-MARC-04 |
| FB | `FB-pigmentos-dobras.md` | F-MARC-05, F-MARC-06, F-MARC-07, F-MARC-08 |
| FC | `FC-caminhos.md` | F-NAV-01, F-NAV-02, F-NAV-05, F-NAV-06 |
| FD | `FD-texto-e-paleta.md` | F-NAV-03, F-NAV-04, F-NAV-07, F-NAV-08, F-NAV-09 |
| FE | `FE-kagi-hanko.md` | F-ARQ-01, F-ARQ-02, F-ARQ-06, F-ARQ-07 |
| FG | `FG-sumi-impressao.md` | F-ARQ-03, F-ARQ-04, F-ARQ-05 |
| FF | `FF-nevoa-washi-pacto.md` | F-AMB-01, F-AMB-02, F-AMB-03, F-AMB-04 |

## 3. Spikes referenciados

Um spike é uma investigação curta. As fichas citam estes IDs. O gerente do roadmap detalha cada um depois.

| ID | Pergunta | Destrava |
| --- | --- | --- |
| S-01 | É possível anexar objetos de anotação ao PDF original, preservando os bytes e a validade da assinatura de `contrato-assinado.pdf`? | F-ARQ-02 |
| S-02 | O motor extrai a geometria dos links internos e renderiza um recorte pequeno rápido o bastante para uma prévia? | F-NAV-07 |
| S-03 | É possível limitar memória agregada e cancelar a preparação sem perder fidelidade? Raster mantém 200 DPI. Preservação vetorial só se viável, sem aumento de resolução ou seletor de DPI | F-ARQ-03 |
| S-04 | A camada de texto entrega linhas com geometria confiável em duas colunas, tabelas e páginas rotacionadas? | F-AMB-03 |
| S-05 | Como o Pdfium carrega um PDF protegido por senha, e que permissões ele reporta? | F-ARQ-01, F-ARQ-06 |
| S-06 | Qual é a linha de base de tempo até a primeira página, memória por aba, latência de busca e CPU em repouso? | Critérios de regressão de todas as funções |
| S-07 | Existe uma heurística de junção de linhas e de hífen que acerta em português e em inglês numa amostra de PDFs reais? | F-NAV-03 |
| S-08 | Um protótipo de papel oriental em HTML, com PDFs reais, é melhor de ler sem fadiga e sem perder fidelidade de cor? | F-AMB-01 |

## 4. Dependências e conflitos conhecidos

- F-MARC-05 (cor) afeta o filtro de F-MARC-02, o export de F-MARC-08 e a gravação da cópia em `engine.rs`.
- F-NAV-02 (fólio) é usado por F-MARC-02, F-MARC-08 e F-ARQ-05.
- F-MARC-01 (diário) e F-NAV-05 (posição) compartilham a identidade do arquivo. O diário também convive com o aviso de arquivo alterado por fora (`view.rs:2405`).
- F-NAV-04 (paleta completa) recebe um comando de cada função nova que tiver atalho. A regra é geral, e o teste estrutural dela mora em F-NAV-04.
- F-ARQ-02 e F-ARQ-07 tocam `crates/tsuro-sign/`, que exige aprovação do PO.
- F-ARQ-03 e F-ARQ-04 mexem na mesma tubulação de impressão.
- F-AMB-02 muda a altura útil do visor, e isso pode forçar nova rasterização. Precisa de teste.
- F-AMB-01 depende de uma mudança de princípio do Kiri e do spike S-08.
- F-AMB-04 vale para todas as funções. Qualquer ficha que contrarie uma cláusula dele precisa dizer isso.

## 5. Decisões do PO já tomadas

1. Em 13 de outubro de 2026, o tema do papel (Washi) é uma boa ideia e fica para depois.
2. Em 13 de outubro de 2026, a marcação contínua é rebaixada. A memória da cor vem primeiro.
3. O padrão de copiar texto (parágrafo ou cru) ainda não foi decidido.
4. Salvar de forma incremental depende de aprovação, porque toca `crates/tsuro-sign/`.
