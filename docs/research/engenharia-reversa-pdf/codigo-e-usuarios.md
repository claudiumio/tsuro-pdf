# O que o código e os usuários dos leitores de PDF ensinam ao TsuroPDF

Pesquisa de 13 de outubro de 2026. Continua `relatorio.md` (funções) e `ui-ux.md` (interface). Aqui o ponto de partida são duas fontes que os documentos anteriores não usaram. O código dos leitores abertos e o que os usuários dos leitores fechados e abertos reclamam ou pedem.

Nenhum aplicativo foi instalado ou executado para este documento. A análise é de código-fonte público, rastreadores de issues, fóruns e avaliações.

## Conclusão

Os usuários de todos esses leitores reclamam de poucas coisas, e elas se repetem entre produtos que não têm nada em comum. Perda de trabalho no salvamento, interface que se enche de coisas que ninguém pediu, marcar sem lembrar a cor, copiar texto com quebras de linha no meio do parágrafo, não conseguir voltar depois de seguir um link, e impressão borrada.

O Tsuro já acerta o essencial de duas delas, a interface quieta e o salvamento explícito de uma cópia. Ele ainda não resolve as outras. Em ordem de valor e de evidência, recomendo estes mecanismos.

1. **Diário de recuperação das marcações.** As marcações vivem só na sessão até o usuário salvar. Um travamento as perde, e é a perda que mais aparece nas avaliações do PDF Expert, do PDFgear e do Nitro.
2. **Senha ao abrir.** Não encontrei tela de senha nem a palavra `password` em `engine.rs` e `session.rs`, e o caminho de salvar cópia carrega o PDF com `password = None`. Extratos e informes brasileiros costumam vir protegidos por senha. Verificar em execução antes de abrir issue.
3. **Aba Marcações e marcações na paleta**, com marcas na barra de progresso.
4. **Copiar como parágrafo**, sem quebras de linha no meio da frase e sem hífen de fim de linha.
5. **Lembrar a última cor e o último tipo**, com cuidado para não criar um modo pegajoso.
6. **Voltar depois de um salto.** Botão contextual e botões 4 e 5 do mouse.
7. **Rótulo de página impresso**, para PDFs em que a página 7 do arquivo é a página 1 do livro.
8. **Impressão nítida e opção sem marcações.**
9. **Salvar sem quebrar assinaturas**, por atualização incremental. É o mecanismo de maior alcance e o único que exige autorização antes, porque toca `tsuro-sign`.

Prévia de link, sumário gerado, seleção por teclado, tema do papel e modo foco ficam depois, como protótipo ou decisão. O motivo está em cada item.

## 1. Método, cobertura e limites

**Regra de inspiração.** O Tsuro é MIT. SumatraPDF, Sioyek, MuPDF e Okular são GPL ou AGPL, e o Zotero é AGPL. Li esse código para entender comportamento e estrutura de dados. Nenhum trecho deve ser copiado ou adaptado linha a linha. O pdf.js é Apache-2.0 e Zathura é zlib, que são compatíveis com MIT, mas a regra continua a mesma para o projeto. Cada mecanismo abaixo descreve o comportamento desejado e deixa a implementação para o Tsuro.

**Como medi o que os usuários pedem.** Para os projetos no GitHub e no GitLab, listei as issues abertas e ordenei por 👍. Para o Okular, usei o Bugzilla do KDE, que devolveu os bugs sem campo de votos, então a lista é só uma amostra. Para os apps fechados, li fóruns oficiais, Hacker News, Capterra, Software Advice, App Store e comunidades de usuários. Os números de 👍 são pequenos e variam por projeto. Servem como sinal de convergência, não como ranking estatístico.

**Citações.** O que aparece entre aspas vem de textos em inglês e é tradução minha.

**Viés das fontes.** Quem escreve avaliação tende a reclamar. Reclamações de preço e de suporte aparecem muito e ficaram de fora, porque não orientam o desenho do Tsuro.

| App | Código lido | Licença | Reclamações e pedidos | Profundidade |
| --- | --- | --- | --- | --- |
| SumatraPDF | Estrutura e cabeçalho da paleta | GPL-3.0 | 22 issues abertas por 👍, fórum, HN | Média |
| Sioyek | Esquema do banco (`database.cpp`) | GPL-3.0 | 22 issues por 👍, HN | Média a alta |
| Zotero e leitor | Paleta de cores do leitor | AGPL | Issues do leitor e do app, documentação | Média |
| pdf.js (Firefox) | Gravação incremental e arquivos do editor | Apache-2.0 | 30 issues por 👍 | Média |
| Zathura | Não li código | zlib | 14 issues por 👍 | Média, só rastreador |
| Okular | Não li código | GPL | 16 bugs do Bugzilla, sem votos | Baixa |
| GNOME Papers e Evince | Não li código | GPL-2.0 | 18 issues por 👍 (GitLab) | Média, só rastreador |
| Skim | Não li código | Aberto | Wiki, FAQ e pedidos no SourceForge | Média |
| Acrobat, Foxit, PDF Expert, PDF-XChange | Fechado | Proprietário | Fóruns, HN, avaliações | Média a alta |
| Bluebeam, Nitro, PDFelement, Xodo, LiquidText | Fechado | Proprietário | Avaliações e fóruns | Baixa a média |
| PDFgear, Preview, Edge | Fechado | Proprietário | App Store, fóruns, artigos | Média |

O PDFgear está instalado na sua máquina, mas eu não o executei. Nitro, PDFelement, Xodo e Edge têm cobertura rasa, e os padrões que cito deles vêm de poucas fontes. MuPDF, que alimenta Sumatra e Sioyek, não foi lido.

## 2. O que o código dos leitores abertos mostra

### Sioyek guarda o estado por documento, e a identidade é o conteúdo

O esquema do banco do Sioyek tem seis tabelas.

- `opened_books` com zoom, deslocamento em x e y e hora do último acesso.
- `marks` com um símbolo de uma letra por documento.
- `bookmarks` com descrição e posição.
- `highlights` com tipo de uma letra e coordenadas de início e fim.
- `links`, que são os portais entre dois pontos.
- `document_hash`, que associa caminho e hash do conteúdo.

A tabela `document_hash` significa que o estado de leitura acompanha o conteúdo do arquivo, não só o caminho. O Tsuro guarda a posição por caminho e a invalida se o tamanho ou a data de modificação mudam (`positions.rs`, `find_position`). Mover o arquivo de pasta, ou salvá-lo em outro leitor, apaga a posição de leitura. O PDFelement tem a queixa oposta ("não abre no último ponto") e o Papers tem a issue #375 sobre recarregar e voltar ao topo da página.

O que importa mais é o custo que o Sioyek paga por não escrever no PDF. A issue mais votada do projeto, a #160 com 30 votos, pede compatibilidade com outros leitores. O autor relata que, ao abrir um PDF com anotações do Acrobat ou do Zotero, as anotações não são pesquisáveis nem removíveis, que o comando `embed_annotations` apaga as anotações dos outros, e que o que foi feito no Sioyek não aparece em outro leitor. A decisão do Tsuro de gravar anotações padrão dentro do PDF é a escolha certa para isso. O diferencial é real e vale proteger.

### O pdf.js salva anexando, e o Tsuro reescreve o arquivo

Em `src/core/writer.js`, a função `incrementalUpdate` recebe os dados originais, calcula as mudanças e acrescenta objetos novos ao fim, com uma nova tabela de referência cruzada. Os bytes originais ficam intactos como prefixo do arquivo novo.

O Tsuro grava a cópia com `save_to_bytes` do `pdfium-render`, que reescreve o documento. Isso explica o aviso ao salvar um PDF assinado que o rascunho da issue de marcação já previa. Uma atualização incremental preservaria os bytes cobertos pela assinatura original. Um comentário do Hacker News sobre o SumatraPDF dá o motivo prático. Na minha tradução, "muitos PDFs são usados como fonte de verdade compartilhada, e um hash simples verifica que todos usam o mesmo documento sem alteração. Isso complica quando as anotações são escritas dentro do arquivo".

Dois fatos limitam a viabilidade. O `pdfium-render` 0.8.37 deixa um TODO em `document.rs:314` para suportar `FPDF_INCREMENTAL`, então a API pública não expõe esse modo, embora a constante exista nos bindings. E mexer em assinatura toca `crates/tsuro-sign`, que o `AGENTS.md` coloca na lista de "pergunte antes".

### O Zotero restringe a paleta, e o Sumatra coleta marcações sob demanda

O leitor do Zotero define oito cores nomeadas (amarelo, vermelho, verde, azul, roxo, magenta, laranja, cinza) em `src/common/defines.js`. A lista é fechada. O usuário escolhe entre poucas cores com nome, não uma cor livre.

No SumatraPDF, a paleta de comandos tem modos por prefixo e uma função `CollectAnnotations` que coleta as anotações do documento em segundo plano na primeira vez que o modo é aberto. O resultado é reutilizado, inclusive quando não há nenhuma. Para o Tsuro isso é uma regra de arquitetura. A paleta não deve recalcular a lista de marcações a cada tecla.

### Onde cada leitor guarda as marcações

A discussão sobre guardar dentro do PDF ou fora é antiga, e cada produto escolheu um lado por um motivo registrado.

| Produto | Onde guarda | Motivo declarado | Custo relatado |
| --- | --- | --- | --- |
| Tsuro | No PDF, ao salvar uma cópia | Portabilidade e original intacto | Reescreve o arquivo |
| pdf.js | No PDF, por atualização incremental | Compatibilidade | Complexidade de escrita |
| Zotero | No banco, exportável para o PDF | Sincronização sem conflito e notas ricas | Outro leitor não vê as marcações |
| Skim | Em atributos estendidos do arquivo | PDFKit grava mal; notas ricas não cabem na especificação; PDFs protegidos não podem ser alterados | Git e Dropbox perdem os atributos |
| Sioyek | Em banco próprio | Não tocar o PDF | Outro leitor não vê, e a issue #160 pede interoperabilidade |
| SumatraPDF | No PDF (antes, também em arquivo `.smx`) | Compatibilidade | Perdeu o conjunto de anotações por tema de estudo |

O Tsuro está no lado com mais interoperabilidade. O que ele pode aprender dos outros é que gravar no PDF não pode custar o original, e o pdf.js mostra um jeito.

## 3. O que os usuários reclamam e sentem falta

Cada tema abaixo reúne evidência de mais de um produto. A coluna final diz o que o Tsuro faz hoje, verificado no código.

### Perder trabalho sem aviso

- **PDF Expert.** "Perdi dois trabalhos depois de uma hora de edição", salvamento que não grava as anotações, e um app que "trava a cada 5 a 10 minutos e eu perco as anotações".
- **PDFgear.** "O PDF não salva em mais da metade das vezes, é preciso salvar com outro nome."
- **Nitro.** O OCR da versão 26 "destrói os PDFs", com páginas duplicadas e cortadas.
- **Preview.** Salva sozinho ao fechar, sem aviso, e altera anotações feitas por PDF Expert e Acrobat. Um texto detalhado do Eclectic Light descreve o mecanismo.
- **Bluebeam.** Marcações "mudam de lugar depois de salvar e reabrir".
- **Edge.** Só oferece Salvar como, nunca salvar no original.

**Tsuro hoje.** Salva uma cópia por pedido, com diálogo de fechamento de três saídas, e `save_copy` relê os bytes originais, então o documento aberto não é alterado. Isso evita o defeito do Preview. Não encontrei diário de recuperação. Se o app travar, as marcações da sessão se perdem.

### Marcar sem lembrar a cor

- **LiquidText.** "O marca-texto sempre volta ao amarelo e ao mesmo tamanho, a caneta lembra."
- **PDF Expert 7.** Perdeu a maioria das cores favoritas e agora cada cor fora das cinco exige três ou quatro cliques.
- **PDF-XChange.** Um usuário pede que "a seleção de cor lembre o que foi escolhido da última vez".
- **Zotero.** A issue #4439, com 9 votos, pede lembrar cor, fonte e largura.
- **Nitro.** O contraponto. Um usuário reclama que o marca-texto tem uma cor só e que é preciso "lembrar de desligar o destaque antes de voltar a selecionar texto para copiar".
- **Foxit.** Depois de anotar um trecho, "não dá para copiar e colar, clicar abre um pop-up".

**Tsuro hoje.** A barra da seleção tem quatro ações com H, U, S e N e não tem modo. Isso evita o problema do Nitro e do Foxit. Mas `Annotation` não guarda cor (`session.rs:486`), então a pergunta de lembrar a cor ainda nem existe.

### Copiar texto

- **pdf.js.** A issue #19343 pede copiar um parágrafo sem quebrar nas linhas visuais. O comentário aponta os `<br>` entre os trechos como a causa.
- **LiquidText.** "Preciso remover as quebras de linha à mão a cada colagem no Word."
- **Foxit.** Colar às vezes "perde os espaços".

**Tsuro hoje.** `selection_plain_text` (`session.rs:5431`) devolve o trecho cru da camada de texto. A exportação em Markdown colapsa espaços em branco, mas a cópia comum não junta linhas nem trata hífen de fim de linha. Não achei tratamento de hifenização no código.

### Voltar depois de seguir um link

- **GNOME Papers.** A issue #277 pede uma UI para voltar depois de clicar num link interno, com 17 comentários.
- **Zotero.** A issue #3436, com 26 votos, pede que os botões de voltar e avançar do mouse funcionem no leitor.
- **Zathura.** A issue #577 pede prévia do link sem segui-lo, citando o Evince, que já faz isso.
- **Skim.** Tem prévia de link interno e histórico visual como recursos documentados.

**Tsuro hoje.** Tem `HistoryBack` e `HistoryForward` com Alt e seta, e não tem tratamento dos botões 4 e 5 do mouse (verificado por busca no código). Não há botão visível de voltar.

### Página do arquivo e página do livro

- **Sioyek.** A issue #1132, com 7 votos, pede um deslocamento de página persistente.
- **LiquidText.** A citação exportada usa 1, 2, 3 como número de página, e o usuário nota que "documentos acadêmicos raramente começam na página 1".

**Tsuro hoje.** Não encontrei suporte a rótulos de página. O `pdfium-render` 0.8.37 expõe `PdfPage::label()` (`page.rs:256`), então o dado está disponível.

### Impressão borrada ou errada

- **pdf.js.** "A impressão fica borrada", issue #2750 com 46 comentários, aberta desde 2013.
- **Okular.** Cinco dos 16 bugs abertos que li são de impressão. Margens que não salvam, saída reduzida mesmo com escala de 100%, resolução baixa em imagens e falha com rasterização forçada.
- **GNOME Papers.** Diálogo de impressão com elementos faltando e travado ao buscar a impressora.
- **SumatraPDF.** Pedidos de diálogo de impressão próprio e de melhorias.
- **Bluebeam.** Um usuário imprime pelo Acrobat para o plotter "porque sai melhor".

**Tsuro hoje.** Imprime rasterizando cada página a 200 DPI (`print.rs:18`), de propósito, para assar intervalo, cópias e orientação dentro do PDF. É o mesmo risco que o pdf.js e o Okular carregam. Não medi a nitidez do texto pequeno.

### Interface que enche de coisas

- **Acrobat.** Os relatos mais fortes. Painéis que abrem sozinhos, "resumo por IA flutuante que precisa ser fechado a cada documento", banners, upsell, "o Reader parece adware". Um usuário diz que a busca da tela inicial busca ferramentas e não o documento. Outro aponta que a lista de recentes, que carrega miniaturas, trava a tela inicial.
- **Foxit.** A faixa de ribbon, um botão flutuante de IA difícil de remover e anúncios.
- **PDF-XChange.** A interface "ocupada demais".
- **Bluebeam.** Atalhos de três teclas e barras personalizadas que voltam ao padrão sozinhas.
- **Nitro.** Mais cliques do que o necessário em tarefas simples, como girar o documento inteiro.

**Tsuro hoje.** É o ponto forte. O chrome de uma linha, a ausência de contas e de nuvem e a tela inicial com miniaturas assíncronas (`ThumbReady`) vão na direção oposta. Vale conferir que o carregamento das miniaturas nunca bloqueia a abertura, que é a causa que os usuários do Acrobat apontam para o travamento.

### Busca

- **PDF Expert.** O painel de busca fica aberto ocupando a tela e o destaque ficou amarelo pálido.
- **PDFgear.** Pede setas para ir ao próximo resultado e nota que é difícil ver qual é o resultado atual.
- **SumatraPDF.** A issue #2376, a mais votada do projeto com 8 votos, pede buscar em todos os PDFs abertos.

**Tsuro hoje.** Já busca em todas as abas pela paleta (`PaletteItem::GlobalHit`). O resultado atual tem cor própria (`mark_current`, âmbar, com teste de que difere das demais). Enter e F3 avançam.

### Ler com conforto

- **LiquidText.** Pede modo escuro para PDFs de texto.
- **PDFgear.** O "modo noturno" é só um cinza mais escuro, "não é um modo escuro de verdade".
- **PDFelement.** Um usuário usa inverter cores por acessibilidade para economizar bateria em tela OLED.
- **SumatraPDF.** Pedidos de brilho e contraste (#4788) e de textura de papel no modo de leitura (#4141).
- **Zathura.** A recoloração pisca ao dar zoom (#429 e #765).

**Tsuro hoje.** A folha é sempre branca, por regra do Kiri. Existe demanda forte em seis produtos diferentes, e há um custo técnico conhecido, o piscar na recoloração.

### Abrir arquivos protegidos

- **Preview.** Abre PDFs protegidos por senha. Um relato de 2025 mostra um PDF de banco com proteção de escrita que desativa a marcação.
- **Edge.** A barra de anotações some em alguns formulários protegidos e só oferece Salvar como.
- **Sioyek.** Tem um comando `enter_password`.

**Tsuro hoje.** Vi `load_pdf_from_byte_slice(&bytes, None)` no caminho de salvar cópia, e não achei a palavra `password` em `engine.rs` nem em `session.rs`. O caminho de abrir pode tratar isso de outro modo que a busca não pegou. Não executei o app com um PDF protegido, então isso é uma leitura de código, a confirmar.

### Memória e desempenho

- **Preview.** "Mantém todos os PDFs abertos em memória" e chegou a 40 GB com 200 abas, segundo um comentário. Trava com documentos de mais de 500 páginas.
- **Bluebeam.** Fica lento com muitas camadas de anotação, e a recomendação oficial é achatar as marcações.
- **Okular.** Um bug aberto diz que "demora muito para abrir".
- **pdf.js e GNOME Papers.** Pedidos de renderização em ladrilhos, com 14 votos no pdf.js (#6419) e uma issue no Papers (#210).

**Tsuro hoje.** `engine.close()` solta o documento ao fechar a aba (issue #40), o que evita o caso do Preview. Não verifiquei o limite de escala de renderização em zoom alto.

## 4. Mecanismos propostos

Cada um descreve o comportamento, a evidência, o que não copiar e como verificar. O esforço é relativo. Os mecanismos 1 a 3 são os que eu começaria.

### 4.1 Diário de recuperação das marcações

**Evidência.** PDF Expert (duas avaliações), PDFgear, Nitro e Bluebeam. Em todos o usuário perdeu marcações que ainda não tinham sido salvas ou que foram salvas errado.

**Comportamento.** Cada marcação criada, alterada ou removida acrescenta uma linha a um arquivo de diário na pasta de dados do app, ao lado de `positions`. A chave é a identidade do arquivo. Ao reabrir um PDF com diário pendente, o Tsuro oferece restaurar as marcações. Salvar a cópia ou descartar apaga o diário.

**Dados.** O formato segue o padrão do `positions.rs`, que já é uma linha por entrada, em texto, com leitura cega que nunca derruba o app e escapes no caminho. Uma operação por linha (criar, mudar, remover) permite reconstruir a lista sem gravar o estado inteiro a cada toque.

**Não copiar.** O comportamento de salvar sozinho dentro do PDF. É o defeito do Preview. O diário nunca altera o arquivo do usuário.

**Verificar.** Matar o processo no meio de uma sessão e conferir que as marcações voltam. Verificar que um arquivo alterado por fora invalida o diário.

**Esforço.** Baixo a médio. O risco é o diário divergir de um PDF que mudou, e a identidade do arquivo já resolve parte disso.

### 4.2 Senha ao abrir

**Evidência.** O Sioyek tem `enter_password` e o Preview desbloqueia PDFs protegidos. No código do Tsuro vejo `load_pdf_from_byte_slice(&bytes, None)` no caminho de salvar cópia e nenhuma tela de senha. Extratos bancários e informes de rendimentos no Brasil costumam vir com senha, o que é uma inferência minha de contexto, não uma fonte.

**Comportamento.** Um PDF protegido abre num cartão de senha em vez de uma falha genérica. A senha não é gravada. Errar mostra a mensagem e deixa tentar de novo.

**Não copiar.** Qualquer coisa que contorne a proteção. O Tsuro só aceita a senha que o usuário digita.

**Verificar.** Gerar um PDF de teste protegido com `qpdf --encrypt` (o `qpdf` entrou na sua máquina nesta sessão, por um `brew install` que eu não deveria ter disparado) e abrir no Tsuro. Conferir também que a cópia salva de um PDF protegido não perde a proteção.

**Esforço.** Baixo a médio. Marcado como lacuna de leitura, não de anotação.

### 4.3 Aba Marcações, paleta e marcas na barra de progresso

**Evidência.** Zotero, PDF Expert, Okular e Preview têm a lista. A issue #4305 do SumatraPDF pede ver as anotações ao lado do texto na segunda leitura. Um comentário do Hacker News propõe um minimapa com os pontos marcados e uma faixa de calor das partes vistas, "para não se perder ao pular entre lugares". Os usuários do PDF Expert reclamaram quando o resumo de anotações perdeu os números de página.

**Comportamento.** A terceira aba do painel lista as marcações com página, tipo, trecho e nota, uma linha por marcação, com busca. A paleta ganha o escopo de marcações, e a lista é coletada sob demanda e reutilizada, como faz o Sumatra. A barra de progresso fina do Kiri (`progress`, `view.rs:1988`) ganha ticks nos pontos marcados e nos resultados de busca.

**Dados.** `OutlineTab(bool)` só distingue duas abas. Uma terceira pede um tipo `PanelTab { Thumbs, Outline, Marks }`.

**Não copiar.** A tabela com colunas e status do Bluebeam. E o painel de notas à direita do Zotero.

**Esforço.** Médio. Os ticks na barra são a parte nova, porque nenhum leitor que li faz isso e a única fonte é uma sugestão de usuário. Tratar como hipótese.

### 4.4 Copiar como parágrafo

**Evidência.** pdf.js (#19343), LiquidText e Foxit. A causa no pdf.js é uma quebra entre cada trecho de texto.

**Comportamento.** Um atalho ou item de menu copia a seleção como parágrafo. Junta as linhas de uma mesma frase, remove o hífen de fim de linha e preserva a quebra onde há linha em branco entre blocos. A cópia crua continua disponível. Qual dos dois é o padrão é decisão de produto, e a evidência favorece o parágrafo, mas código e tabelas precisam das quebras.

**Dados.** Uma função pura sobre o texto da `TextLayer`, testável com `guia-folio.pdf`, que já tem tipografia e acentos.

**Não copiar.** Heurística opaca. Quando a função não tiver certeza, deve manter a quebra.

**Verificar.** Texto em duas colunas, listas, tabelas, hifenização em português e em inglês.

**Esforço.** Baixo a médio.

### 4.5 Lembrar a última cor e o último tipo, sem modo pegajoso

**Evidência.** Cinco fontes pedem lembrar a cor (seção 3), e duas mostram o custo de um modo que fica ligado (Nitro e Foxit).

**Comportamento.** A barra da seleção usa o último tipo e a última cor como padrão. Um modo contínuo, se existir, precisa de saída automática depois de um tempo sem uso e de um indicador visível. Sem indicador, vira o problema do Nitro.

**Revisão ao `ui-ux.md`.** O mecanismo 5.4 de lá propunha um modo contínuo. A evidência desta rodada mostra que ele pode confundir quem volta a copiar texto. Recomendo começar só com a memória da cor, e deixar o modo contínuo para depois de testar com usuários.

**Dados.** `Annotation` não tem cor. É preciso um campo novo, um tipo fechado com poucas cores nomeadas, como no Zotero, e a passagem por `save_copy`.

**Esforço.** Médio a alto, por causa da persistência da cor entre leitores.

### 4.6 Voltar depois de um salto

**Evidência.** Papers (#277), Zotero (#3436, com 26 votos) e a prévia do Skim.

**Comportamento.** Depois de um salto por link, sumário, paleta ou resultado de busca, aparece um chip discreto "Voltar à página N" perto do HUD, que some depois de alguns segundos ou ao navegar. Os botões 4 e 5 do mouse chamam `HistoryBack` e `HistoryForward`.

**Dados.** O histórico já existe. O trabalho é só a superfície, o chip e o mapeamento dos botões.

**Esforço.** Baixo.

### 4.7 Rótulo de página impresso

**Evidência.** Sioyek (#1132) e a queixa do LiquidText sobre citações. O dado já está no `pdfium-render`, em `PdfPage::label()`.

**Comportamento.** Quando o PDF tem rótulos, o contador mostra "xii (14 / 220)" ou "245 (252 / 300)". A exportação das marcações em Markdown e o campo "ir para página" usam o rótulo. Sem rótulos, nada muda.

**Dados.** `annotations_markdown` hoje escreve `p.{n}` com o índice. O rótulo entra ali.

**Esforço.** Baixo a médio. O risco é a ambiguidade entre ir para "12" do rótulo e para a página 12 do arquivo.

### 4.8 Impressão nítida e sem marcações

**Evidência.** Impressão borrada em pdf.js, Okular e Bluebeam, e a opção de imprimir com ou sem marcações no Acrobat e no Foxit (do `ui-ux.md`).

**Comportamento.** Duas coisas separadas. Primeiro, uma opção de incluir as marcações, ligada por padrão. Segundo, investigar uma saída vetorial. Hoje o texto vira imagem a 200 DPI. Se o Pdfium puder importar as páginas escolhidas para um documento novo, o texto e os gráficos continuam nítidos e o arquivo fica menor. Não verifiquei se o `pdfium-render` oferece essa importação nem como ela trata cópias e orientação.

**Verificar.** Imprimir texto pequeno, código de barras e página de tamanho diferente nos dois caminhos e comparar a olho nu e por ampliação.

**Esforço.** Baixo para a opção. Médio a alto para o caminho vetorial.

### 4.9 Salvar sem quebrar assinaturas

**Evidência.** O código do pdf.js, o comentário do Hacker News sobre hash da fonte, o Foxit que "não deixa sobrescrever depois de assinar", e o aviso que o próprio Tsuro já mostra ao salvar um PDF assinado.

**Comportamento.** Gravar as anotações como uma atualização incremental, acrescentada ao fim do arquivo original, em vez de reescrever. O prefixo do arquivo fica idêntico ao original. Uma assinatura anterior continua valendo para a sua revisão, e outro leitor pode apontar o documento como alterado depois, o que é o comportamento correto.

**Limites.** O `pdfium-render` não expõe o modo incremental. Seria preciso chamar os bindings brutos de `FPDF_SaveAsCopy` com o flag, ou escrever a atualização com `pdf-writer`, que já é dependência. Qualquer das duas vias exige validar com `contrato-assinado.pdf` e com o `tsuro-sign`, e mexer lá pede autorização prévia, pelo `AGENTS.md`.

**Esforço e risco.** Alto. É o mecanismo de maior alcance, porque protege a assinatura, que é um diferencial do projeto no Brasil. Não recomendo começar por ele. Recomendo abrir uma investigação com este escopo.

## 5. Para depois, ou para decidir

| Tema | Evidência | Por que fica para depois |
| --- | --- | --- |
| Prévia de link interno | Skim e Sioyek têm, Zathura #577 pede, Evince faz | Custo alto. Falta confirmar que o motor extrai a geometria dos links |
| Sumário gerado quando o PDF não tem | Extensão Google Scholar PDF Reader (100 mil usuários, segundo a issue), pdf.js #17921 com 44 votos, Sioyek | O contra-argumento é forte. Usuários do PDF Expert dizem que a detecção automática erra tanto que é mais rápido fazer à mão. Se existir, deve vir marcado como gerado |
| Seleção de texto por teclado | Zathura #748 com 19 votos, caret browsing do Edge | Um comentário na issue do Zathura diz que dá "trabalho sério" e que falta um desenho que funcione com a interface de plugins. No Tsuro a camada de texto própria facilita, mas é desenho grande |
| Tema do papel | Seis fontes, mais Okular e Zathura | Contraria a regra "folha sempre branca" do Kiri, que é inegociável pela spec. Decisão sua |
| HUD que se esconde | Papers #302, "não dá para esconder os botões de zoom sobrepostos", e Skim #1763 | O HUD do Tsuro já tem sinais de auto-ocultação. Conferir se o usuário consegue escondê-lo de vez |
| Posição de leitura pelo conteúdo | Tabela `document_hash` do Sioyek | Baixo custo, mas há risco de restaurar a página errada se o PDF mudou. Se a contagem de páginas for igual, é razoável |
| Sensibilidade do trackpad e da roda | Sioyek #157, com 17 votos | Não encontrei tratamento próprio no código do Tsuro. Testar em Mac com trackpad e com mouse |

## 6. O que continuou valendo e o que mudou nos documentos anteriores

| Item | Situação |
| --- | --- |
| Aba Marcações e escopo na paleta | Continua. Agora com evidência extra de pedido real (Sumatra #4305, usuários do PDF Expert) |
| Modo contínuo de marcação | Rebaixado. Nitro e Foxit mostram o custo de um modo que fica ligado |
| Cor das marcações | Continua. A evidência de pedido é mais forte (cinco fontes), mas exige campo novo em `Annotation` |
| Imprimir sem marcações | Continua. Soma-se a questão da nitidez |
| Prévia de link e régua de leitura | Continuam como protótipo. A prévia tem agora quatro fontes. A régua não ganhou evidência nova de pedido |
| Modo foco | Continua, com a evidência do Papers #302 de que um HUD que não se esconde incomoda |
| Folha sempre branca | Decisão aberta. A demanda por tema de papel é ampla |
| Busca em todas as abas | Já existe no Tsuro, e é o pedido mais votado do SumatraPDF |

## 7. Verificação desta entrega

Li nesta sessão os trechos citados do Tsuro (`session.rs`, `positions.rs`, `print.rs`, `view.rs`, `engine.rs`), o esquema do Sioyek, o `writer.js` do pdf.js, a paleta do Zotero e o cabeçalho da paleta do Sumatra. As contagens de 👍 e votos vêm dos rastreadores no momento da consulta.

Não executei nenhum concorrente nem o Tsuro. As afirmações sobre senha, nitidez de impressão, limite de zoom e comportamento do HUD saem da leitura de código e precisam ser confirmadas em execução. Os números das avaliações de produtos fechados refletem quem escreve avaliação.

Durante esta rodada um `brew install` que eu disparei em segundo plano, contra o seu pedido de não instalar, chegou a instalar `qpdf` e `mupdf` e a atualizar dependências do Homebrew. Isso está descrito na resposta ao pedido, não neste documento.

## Fontes

Consultadas em 13 de outubro de 2026.

- Rastreadores. [SumatraPDF](https://github.com/sumatrapdfreader/sumatrapdf/issues), [Sioyek](https://github.com/ahrm/sioyek/issues), [Zathura](https://github.com/pwmt/zathura/issues), [pdf.js](https://github.com/mozilla/pdf.js/issues), [Zotero leitor](https://github.com/zotero/reader/issues), [Zotero](https://github.com/zotero/zotero/issues), [GNOME Papers](https://gitlab.gnome.org/GNOME/Incubator/papers/-/issues), [KDE Bugzilla, Okular](https://bugs.kde.org/buglist.cgi?product=okular), [Skim, pedidos](https://sourceforge.net/p/skim-app/feature-requests/).
- Issues citadas. [Sioyek #160](https://github.com/ahrm/sioyek/issues/160), [Sioyek #157](https://github.com/ahrm/sioyek/issues/157), [Zathura #577](https://github.com/pwmt/zathura/issues/577), [Zathura #748](https://github.com/pwmt/zathura/issues/748), [pdf.js #17921](https://github.com/mozilla/pdf.js/issues/17921), [pdf.js #19343](https://github.com/mozilla/pdf.js/issues/19343), [SumatraPDF #4305](https://github.com/sumatrapdfreader/sumatrapdf/issues/4305), [SumatraPDF #2085](https://github.com/sumatrapdfreader/sumatrapdf/issues/2085).
- Código. [Sioyek, `database.cpp`](https://github.com/ahrm/sioyek/blob/main/pdf_viewer/database.cpp), [pdf.js, `writer.js`](https://github.com/mozilla/pdf.js/blob/master/src/core/writer.js), [Zotero leitor, `defines.js`](https://github.com/zotero/reader/blob/master/src/common/defines.js), [SumatraPDF, `CommandPalette.h`](https://github.com/sumatrapdfreader/sumatrapdf/blob/master/src/shared/CommandPalette.h).
- Documentação. [Zotero, por que anotações ficam no banco](https://www.zotero.org/support/kb/annotations_in_database), [Skim, FAQ](https://sourceforge.net/p/skim-app/wiki/FAQ/), [Skim, recursos](https://sourceforge.net/p/skim-app/wiki/Features/), [Eclectic Light, Inside PDF annotations](https://eclecticlight.co/2025/08/14/inside-pdf-annotations/).
- Usuários. [Fórum Adobe, painéis e upsell no Reader](https://community.adobe.com/questions-12/is-there-a-way-to-use-adobe-acrobat-reader-as-a-clean-distraction-free-pdf-reader-without-constant-popups-banners-forced-integrations-and-automatically-opening-panels-1551308), [Fórum Adobe, lentidão](https://community.adobe.com/questions-12/adobe-acrobat-reader-is-incredibly-slow-keeps-hanging-again-1505975/index2.html), [Hacker News, Acrobat](https://news.ycombinator.com/item?id=45598776), [Hacker News, Sioyek e Sumatra](https://news.ycombinator.com/item?id=37998258), [Capterra, Foxit Reader](https://www.capterra.com/p/175376/Foxit-PDF-Reader/reviews/), [PDF Expert, avaliações negativas](https://appsupports.co/743974925/pdf-expert-editor-converter/negative-reviews), [Software Advice, PDF-XChange](https://www.softwareadvice.com/product/476965-PDF-XChange/reviews/), [Software Advice, LiquidText](https://www.softwareadvice.com/productivity/liquidtext-profile/reviews/), [PDFgear, avaliações](https://justuseapp.com/en/app/1615523079/pdfgear-pdf-editor-for-adobe/reviews), [Bluebeam, comunidade](https://community.bluebeam.com/discussion/6224/issues-with-measurements-and-markups-in-bluebeam-revu), [Apple Community, Preview](https://discussions.apple.com/thread/256257514).
