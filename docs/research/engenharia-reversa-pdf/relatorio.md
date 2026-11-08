# Inspirações para o TsuroPDF a partir de leitores comerciais e open source

Pesquisa de 12 de outubro de 2026. Análise funcional e documental de 18 produtos ou famílias. Recomendação de produto, não especificação de implementação.

## Conclusão

O Tsuro deve aprender com os fluxos dos concorrentes, sem reproduzir a aparência nem a extensão de suas suítes. Minha recomendação é concentrar o investimento em reencontrar marcações, consultar referências sem perder o ponto da leitura e ter segurança ao salvar e imprimir.

As referências mais úteis para a missão do projeto são PDF Expert, SumatraPDF, Sioyek, Skim e Okular. Acrobat é uma referência de comportamento documental e impressão. Bluebeam e Zotero ensinam como manter notas ligadas às suas fontes, embora seus produtos completos estejam fora da missão do Tsuro.

Uma hipótese de posicionamento para validar com usuários é: **leitura local, com marcações portáteis e navegação que preserva o contexto**. Não é uma afirmação de exclusividade. Há concorrentes que já oferecem partes dessa combinação.

## 1. O que foi investigado e o que não foi

Este trabalho faz engenharia reversa funcional a partir de manuais, páginas oficiais e fluxos descritos publicamente. Não houve descompilação, instalação dos concorrentes, inspeção de tráfego privado, uso de contas pagas ou leitura do código dos concorrentes. Também não houve benchmark de desempenho ou teste de usabilidade.

Os termos usados no relatório têm estes significados:

- **Documentado** significa que uma fonte primária descreve a capacidade. Não comprova qualidade, velocidade ou disponibilidade em todas as plataformas.
- **Verificado no código local** significa que o trecho do Tsuro foi lido nesta sessão. Não substitui execução do aplicativo.
- **Inferência** é uma interpretação do benefício ou do custo de um padrão.
- **Proposta** é uma adaptação sugerida para o Tsuro, ainda não validada com usuários.

A seleção cobre suítes comerciais conhecidas, leitores focados em leitura, ferramentas de estudo e substitutos gratuitos do sistema e do navegador. Não é ranking de participação de mercado. Preços não foram comparados porque planos, países, versões e sistemas alteram a disponibilidade dos recursos.

A pesquisa anterior em `docs/research/leitores-pdf-mundo.md` serviu de contexto. Suas afirmações sobre lacunas do Tsuro não foram consideradas atuais automaticamente. Este documento a complementa, sem sobrescrevê-la.

### Limitações das fontes

As páginas são documentação viva. Registrei a versão quando ela aparece na fonte, como PDF-XChange V11, Foxit Reader 2025.2 e Bluebeam Revu 20/21. As páginas do Sumatra podem acompanhar recursos recentes ou de pré-lançamento. Antes de testar, é necessário fixar o binário e a versão.

O fetch direto da página comercial do Acrobat retornou HTTP 403. Sua classificação de Reader gratuito e Pro pago veio do resultado indexado da página oficial. O manual de impressão da Adobe foi consultado diretamente. A página de ajuda do Firefox retornou um desafio no fetch direto. Seus recursos foram extraídos do resultado indexado da documentação oficial e merecem reconfirmação no aplicativo.

Não usei reclamações isoladas para declarar que um concorrente é lento, pesado ou confuso. Não há evidência medida para esse tipo de comparação nesta pesquisa.

## 2. O ponto de partida real do Tsuro

A missão em `AGENTS.md:7` limita o produto a ler, anotar, marcar e imprimir. Nuvem, formulários, colaboração e suíte ficam de fora. Essa regra decide o que aproveitar, mesmo quando um recurso concorrente parece atraente.

| Evidência local | O que isso muda na recomendação |
| --- | --- |
| `crates/tsuro/src/session.rs:411` gera Markdown com marcações em ordem de página e posição, incluindo notas e tipos | Não propor "adicionar exportação Markdown" como se nada existisse. Avaliar arquivo exportável, descoberta do comando e uso de subconjuntos. |
| `crates/tsuro/src/session.rs:2950` envia esse Markdown à área de transferência | Diferenciar copiar de exportar para arquivo. O primeiro fluxo já tem implementação. |
| `crates/tsuro/src/view.rs:3095` desenha resultados de busca, seleção e marcações | A pesquisa trata de evolução de um leitor com marcações, não da introdução do conceito. |
| `crates/tsuro/src/engine.rs:396` oferece `save_copy` com as anotações da sessão | Há uma base de gravação de cópia anotada. Compatibilidade de ida e volta com outros leitores ainda precisa de teste. |
| `crates/tsuro/src/print.rs:18` define impressão a 200 DPI e `print.rs:30` define todos, atual ou intervalo contínuo | Fidelidade impressa e seleção de páginas são áreas concretas para comparação. O número de DPI sozinho não demonstra um defeito. |
| `README.pt-BR.md` descreve retomada, histórico, sumário, busca, rotação e painel de assinaturas | Não reapresentar essas categorias como novas. O README ainda diz que não há anotações, em desacordo com o código lido. Isso é uma divergência documental, não uma avaliação do aplicativo em execução. |

Não fiz auditoria exaustiva de todas as interfaces. Onde não confirmei uma capacidade, a prioridade abaixo significa "verificar e evoluir se necessário", nunca "ausência comprovada".

## 3. Leitores comerciais e proprietários

A coluna final é uma decisão de escopo para o Tsuro. Não é acusação de que o recurso seja ruim no produto de origem.

| Produto e modelo | Comportamento documentado | Princípio que podemos adaptar | O que não trazer |
| --- | --- | --- | --- |
| Adobe Acrobat. Reader gratuito, Standard e Pro pagos | Reader anota e imprime. O manual de impressão trata de tamanho real, ajuste, comentários, livreto e pôster. A Adobe distingue serviços de confiança para certificados. [S01, S02, S03] | Tornar explícito o resultado de operações importantes. A prévia de impressão e os estados de verificação precisam explicar o que aconteceu. | Reproduzir a suíte, os painéis comerciais ou sugerir que qualquer assinatura detectada tem confiança jurídica comprovada. |
| Foxit. Reader gratuito, Editor comercial | O Reader oferece marcações. O manual descreve ferramentas contextuais, painel de comentários e acesso rápido a comandos. [S04, S05] | Aproximar a ação do texto selecionado. Manter um caminho visível para quem não conhece atalhos. | Ribbon inteira, integrações corporativas, revisão compartilhada e a quantidade de preferências do produto. |
| PDF-XChange Editor. Gratuito com recursos licenciados | Manual V11 descreve painel de comentários, estilos de anotação e ferramentas de bookmarks. [S06] | Reutilizar um pequeno conjunto de estilos de marcação e navegar documentos longos sem trabalho repetitivo. | Centenas de ferramentas, edição estrutural de bookmarks e a aparência da interface. O manual não basta para determinar o plano de cada função. |
| PDF Expert. Comercial, ecossistema Apple | Lista de anotações por página, busca, filtro por cor e exportação em HTML, texto e Markdown no Mac. Tem temas Day, Night e Sepia. [S07, S08, S09] | Tratar as marcações como material de leitura que pode ser reencontrado e levado para outro lugar. Separar conforto da página do tema da interface. | Copiar a sidebar, os ícones ou a composição visual. Não presumir que o comportamento do Mac seja igual ao do iPhone. |
| Nitro PDF Pro. Comercial | Painel ordena e filtra comentários. Gera resumo em PDF e resumo para impressão. [S10] | Fechar o percurso entre ler, marcar e revisar no papel. | Conversão Office, edição completa, fluxo de assinatura e controles de relatório que não atendam à leitura pessoal. |
| Bluebeam Revu. Comercial, foco em documentos técnicos | Markups List organiza anotações por dados como autor, data e status. Filtra e gera resumos. Formatos variam por assinatura. [S11] | Dar uma visão navegável do que foi marcado. Uma marcação deve poder ser localizada sem procurar a página manualmente. | Tabela de gestão de obra, medições, estados de equipe e campos personalizados. No Tsuro, a proposta é uma lista enxuta. |
| PDFelement. Comercial | Comment Summary agrega marcações com opções de layout e intervalo de páginas no Windows. [S12] | Gerar uma revisão útil das marcações já feitas, sem exigir que o usuário percorra todo o arquivo. | Transformar resumo de marcações em resumo generativo, análise de contratos ou editor Office. |
| Xodo PDF Studio. Comercial, desktop | A página oficial descreve edição e anotação offline em Windows, macOS e Linux. O manual trata as anotações como elementos editáveis separados do conteúdo original. [S13, S14] | Processamento local e portabilidade do arquivo são referências de produto, não exclusividades do Tsuro. | Importar a suíte de conversão, formulários e automação em lote. Não confundir PDF Studio, Reader e Xodo Web. |
| LiquidText. Comercial, leitura e estudo | Workspace ao lado do documento, trechos conectados às fontes e relações entre partes distantes do texto. [S15] | Preservar o contexto da nota e reduzir a necessidade de alternar entre lugares do documento. | O canvas característico, os gestos distintivos, o formato de projeto e a gestão de conhecimento inteira. Interações muito particulares exigem análise de propriedade intelectual antes de uma implementação próxima. |

## 4. Open source e substitutos gratuitos

Gratuito e open source não são sinônimos. Preview é proprietário e vem com o macOS. O visualizador do Firefox tem base open source. Licenças específicas de código não foram auditadas porque nenhuma reutilização foi proposta.

| Produto | Comportamento documentado | Inspiração para o Tsuro | Cuidado na adaptação |
| --- | --- | --- | --- |
| SumatraPDF | Documentação descreve atalhos para marcar, desfazer, salvar no PDF e salvar nova cópia. Também documenta paleta de comandos. [S16, S17] | Tornar as tarefas frequentes curtas e as mudanças reversíveis. O Tsuro não precisa se tornar uma suíte para ter bons comandos. | Não assumir superioridade de RAM ou latência sem benchmark. Não presumir que todos os itens da documentação estejam na versão estável instalada. |
| Okular | Anotações no PDF, cópia anotada e arquivo `.okular` opcional. Documenta modos de memória e opções de cor da página. [S18, S19, S20] | Preservar interoperabilidade e tornar o custo de recursos previsível. Oferecer conforto visual sem alterar o conteúdo. | Não transformar o usuário em administrador de cache. O formato específico do aplicativo não deve ser necessário para compartilhar uma marcação. |
| Sioyek | Prévia de referências, Smart Jump mesmo sem links explícitos, portais, marcas de posição e busca em highlights. [S21] | Consultar uma referência e continuar no mesmo parágrafo. É uma contribuição direta à missão de leitura. | Começar por destinos já presentes no PDF. Inferir referências sem link aumenta o risco de saltos errados. Não transportar o sistema inteiro de portais. |
| Skim | Snapshots mantêm trechos visíveis. Reading bar acompanha a linha lida e aceita teclado. Snapshots não são salvos no PDF. [S22, S23] | Oferecer referência temporária e régua opcional de leitura. | Deixar claro o que é temporário e o que acompanha o arquivo. Não prometer benefício clínico ou cognitivo sem pesquisa apropriada. |
| Zotero | Notas extraídas de anotações incluem citações e links de retorno à página. Anotações ficam no banco local e podem ser exportadas para o PDF. [S24, S25] | Uma nota deve manter sua origem. O usuário precisa voltar ao trecho para conferir o contexto. | Copiar o armazenamento em banco como requisito de uso. Ele atende à biblioteca e à sincronização do Zotero, não necessariamente à missão do Tsuro. |
| Zathura | Navegação por teclado, links numerados, atalhos configuráveis e interface mínima. [S26] | Permitir concluir a leitura sem mouse e sem perder acesso aos links. | Exigir memorização de comandos ou edição de arquivos de configuração para tarefas básicas. |
| GNOME Papers e Evince, tratados como uma família nesta análise | Anotações e navegação em painel com miniaturas, sumário, notas e bookmarks. [S27, S28] | Usar hierarquia simples para revelar apenas as ferramentas pertinentes. | Duplicar a mesma navegação em painéis independentes ou confundir a implementação de Papers com a de Evince. |
| Firefox e seu visualizador PDF | Ajuda oficial descreve texto, desenho, destaque e download de cópia anotada. Evidência indexada, sem teste prático nesta sessão. [S29] | O fluxo básico de abrir, marcar e salvar precisa funcionar sem onboarding. | Supor que destacar texto, sozinho, justifique instalar outro aplicativo. Isso é uma inferência competitiva, não pesquisa de preferência. |
| Apple Preview. Gratuito no sistema, proprietário | Selecionar e marcar, modo de marcação contínua e lista de highlights e notas. [S30] | Oferecer uma ação ocasional e um modo repetitivo para quem vai marcar vários trechos. | Copiar o visual do macOS para todas as plataformas ou adotar silenciosamente políticas de gravação automática. |

## 5. Transformar a observação em comportamento próprio

### 5.1. Reencontrar o que foi marcado

**Origem documentada.** PDF Expert, Bluebeam, Okular e Preview tratam anotações como uma coleção navegável.

**Problema.** Marcar um trecho tem pouco valor se o usuário precisa procurá-lo de novo em duzentas páginas.

**Proposta Tsuro.** Uma lista de marcações com trecho, página, tipo e conteúdo da nota. Pesquisa textual primeiro. Filtros adicionais entram apenas se resolverem um problema demonstrado. Selecionar um item revela o trecho e permite voltar à posição anterior.

**Diferença deliberada.** Não reproduzir a planilha do Bluebeam nem a sidebar do PDF Expert. Usar os tokens, ícones e navegação próprios do Tsuro.

**Critério de aceitação proposto.** Encontrar uma nota por uma palavra, abrir seu contexto e voltar à leitura usando somente teclado. O item ativo deve ter identificação além de cor. Com quinhentas marcações, a lista não deve tornar a rolagem do PDF perceptivelmente pior, a verificar por medição.

### 5.2. Consultar uma referência sem perder o lugar

**Origem documentada.** Prévia do Sioyek e snapshots do Skim. LiquidText reforça a utilidade de preservar conexões com a origem.

**Problema.** Consultar uma nota de rodapé ou figura interrompe o parágrafo em leitura.

**Proposta Tsuro.** Uma prévia transitória para links internos que o PDF já fornece. Abrir por ação explícita de teclado ou mouse. Fechar devolve foco e posição. Um comando separado efetiva o salto.

**Diferença deliberada.** Sem canvas, sem gestos especiais e sem interpretar automaticamente qualquer número como referência. O histórico já descrito no README é a base conceitual, não um recurso a refazer.

**Critério de aceitação proposto.** Consultar três notas e voltar ao mesmo deslocamento visual. Se não houver destino, informar isso sem inventar uma página. Não carregar dezenas de páginas para abrir uma prévia.

### 5.3. Marcar com poucos passos e sem acidentes

**Origem documentada.** Atalhos do Sumatra, ferramentas contextuais do Foxit e modo contínuo do Preview.

**Proposta Tsuro.** Manter seleção e marcação como ações distinguíveis. Permitir marcação ocasional e, se validado com usuários, modo contínuo com estado visível e saída por Escape. Desfazer deve ser acessível sem procurar menus.

**Critério de aceitação proposto.** Selecionar, marcar, desfazer, refazer e salvar usando teclado. Um comando de marcação nunca deve ser disparado enquanto a pessoa digita uma nota. O teste precisa incluir duas colunas e páginas rotacionadas.

### 5.4. Levar as marcações para fora do leitor

**Origem documentada.** Exportação do PDF Expert, retorno à fonte do Zotero e resumos do Nitro.

**Proposta Tsuro.** Evoluir o Markdown já existente. Avaliar salvar em arquivo, exportar apenas a seleção da lista e manter nome do documento e página. A anotação deve continuar útil se o destinatário não tiver Tsuro.

**Limite.** Links locais podem quebrar quando o arquivo muda de pasta e podem revelar caminhos pessoais. Não os adicionar indiscriminadamente. Nome e página são uma base portátil. Um resumo de marcações é uma compilação de trechos escolhidos pelo usuário, não resumo por IA.

**Critério de aceitação proposto.** Reabrir a cópia anotada em pelo menos dois motores distintos e conferir posição, texto e aparência. Exportar notas multilineares, acentos e caracteres Markdown sem perda de conteúdo. O texto exportado deve deixar claro o que é citação e o que é nota do usuário.

### 5.5. Conforto sem modificar o documento

**Origem documentada.** Temas de leitura do PDF Expert, opções de cor do Okular e régua do Skim.

**Proposta Tsuro.** Avaliar uma opção de cor do papel independente do tema da interface e uma régua de leitura desligada por padrão. Ambas são preferências de visualização, não edições do PDF.

**Limite.** Inverter tudo pode prejudicar fotografias, gráficos e marcações. Preservar a legibilidade vale mais do que anunciar um modo noturno. A régua não substitui suporte a leitores de tela.

**Critério de aceitação proposto.** Arquivo salvo e impressão conservam as cores originais. A seleção, o foco e os destaques continuam distinguíveis. Testar documentos coloridos antes de decidir a transformação de cor.

### 5.6. Saber o que será impresso

**Origem documentada.** Acrobat explicita tamanho real, ajuste e impressão de comentários. Nitro cria resumos para impressão.

**Proposta Tsuro.** Auditar fidelidade e previsibilidade da impressão atual antes de acrescentar opções. Depois avaliar intervalos como `1-3, 8`, imprimir com ou sem marcações e apenas páginas marcadas.

**Limite.** Impressão rasterizada a 200 DPI é uma escolha observada no código, não prova de falha. A comparação deve incluir texto pequeno, gráficos vetoriais, códigos de barras e páginas de tamanhos diferentes.

**Critério de aceitação proposto.** A prévia corresponde às páginas e marcações enviadas. Cancelar não deixa a interface bloqueada. Testes de tamanho real verificam dimensões físicas. A saída não duplica uma anotação já embutida no arquivo.

### 5.7. Explicar confiança sem exagerar

**Origem documentada.** A Adobe tem serviços de confiança e configuração de validação. Isso mostra que detectar assinatura e confiar em certificado são coisas diferentes. [S03, S31]

**Proposta Tsuro.** Revisar a clareza do painel existente. Separar integridade dos bytes, cadeia de confiança, revogação e tempo quando essas verificações forem realmente executadas. Mostrar "não verificado" quando não forem.

**Limite.** Não anunciar equivalência a um validador ICP-Brasil nem validade jurídica universal. Não transformar assinatura desenhada em assinatura criptográfica. Esta é uma proposta de comunicação e verificação, não autorização para alterar `crates/tsuro-sign`.

## 6. Priorização recomendada

As prioridades são julgamento de produto. O esforço é relativo, não estimativa de prazo. Todas exigem conferir o comportamento atual antes de abrir trabalho de implementação.

| Ordem | Trabalho recomendado | Por que priorizar | Esforço relativo e principal incerteza |
| --- | --- | --- | --- |
| 0 | Validar gravação, reabertura e impressão das marcações existentes | Perder ou compartilhar uma marcação incorreta destrói a confiança no fluxo inteiro | Médio a alto. Depende do resultado dos testes entre leitores e com documentos assinados. |
| 1 | Reencontrar marcações por lista e pesquisa | Completa o uso do destaque depois que ele é criado | Médio. Conferir painel existente e importação de anotações externas. |
| 2 | Polir ações de marcar, desfazer e salvar | Melhora uma tarefa recorrente sem ampliar a missão | Baixo a médio. Teclado, foco, modos e conflitos exigem testes. |
| 3 | Melhorar a descoberta do Markdown existente e avaliar exportação em arquivo | Aproveita código já implementado e facilita revisão fora do aplicativo | Baixo a médio. Referências locais e formatação precisam de política clara. |
| 4 | Prototipar prévia de links internos | Pode reduzir interrupções em documentos longos | Médio a alto. Exige validar foco, destinos e custo de renderização. |
| 5 | Testar conforto da página e régua opcional | Pode ajudar sessões longas sem virar suíte | Médio. Preferência e qualidade visual precisam de testes com usuários. |
| Contínuo | Regressões de abertura, busca, rolagem, memória e impressão | Protege o atributo central de leveza | Exige uma base medida. Não há números comparativos nesta pesquisa. |

O painel de confiança merece revisão contínua de linguagem e escopo. Não o trato como novo motor de assinatura nem como promessa de conformidade.

### O que fica fora

- Editor de texto e imagens do PDF, conversão Office e reorganização estrutural de documentos.
- Formulários, assinatura remota, colaboração e armazenamento em nuvem.
- Canvas de pesquisa, biblioteca bibliográfica e sistema de conhecimento.
- Medições de engenharia, carimbos corporativos e gestão de revisão de equipes.
- Chat com PDF, resumo automático e OCR embutido por padrão.

OCR pode ajudar leitura de scans, mas adiciona dependências, distribuição e custo computacional. Só merece estudo separado se documentos sem texto aparecerem como obstáculo recorrente. Nesta análise, não há evidência de demanda que justifique antecipá-lo.

## 7. Inspirar sem copiar

A unidade de inspiração é o problema resolvido e o comportamento observável. A implementação, o desenho visual, os textos e os testes devem ser nossos.

| Pode orientar uma solução independente | Exige análise específica | Não é o objetivo deste trabalho |
| --- | --- | --- |
| Filtrar anotações, preservar posição, mostrar prévia de um destino, oferecer desfazer | Reproduzir interação distintiva, usar ativos de terceiros, incorporar bibliotecas ou trechos de código | Copiar layout pixel a pixel, ícones, ilustrações, textos, marca ou código proprietário |
| Documentar requisitos com base em manuais públicos | Examinar termos de uso, patentes relevantes e licenças de componentes | Burlar licença, controle de acesso ou obter implementação confidencial |
| Implementar e testar comportamentos descritos em padrões públicos | Validar direitos de distribuição e compatibilidade de licenças | Supor que "open source" significa "pode copiar sem condições" |

No Brasil, o art. 8º da Lei 9.610 distingue ideias e métodos, como tais, das obras protegidas. Programas de computador têm proteção específica na Lei 9.609. Isso não concede uma autorização geral para clonar interfaces ou contornar contratos. Marcas, desenhos, patentes, concorrência e jurisdição podem exigir análise adicional. Estas são precauções de projeto, não parecer jurídico. [S32, S33]

O Tsuro declara licença MIT em `LICENSE:1`. Não incorporar código de outro projeto sem verificar a licença do arquivo e dos componentes envolvidos. Uma licença copyleft não significa simplesmente "proibido usar", mas pode impor obrigações incompatíveis com a distribuição pretendida se tratada de modo errado.

Uma ficha de desenvolvimento independente para cada inspiração deve registrar:

1. A fonte pública e o problema observado.
2. O comportamento desejado, sem screenshots como especificação pixel a pixel.
3. As diferenças de interação e aparência que fazem sentido para o Tsuro.
4. A origem do código e dos ativos utilizados.
5. Os testes de resultado e de interoperabilidade.

Essa rastreabilidade reduz confusão sobre a origem do trabalho. Não certifica, por si só, um processo jurídico formal de clean room.

## 8. Validação prática que falta executar

### Corpus comum

Usar os mesmos arquivos em cada aplicativo, sem dados privados. Não alterar as fixtures originais de `public/samples/`.

- Texto digital com uma coluna e com duas colunas.
- Livro longo com sumário, links e notas de rodapé.
- Scan sem camada textual.
- Páginas rotacionadas e de tamanhos diferentes.
- Arquivo com muitas marcações de tipos distintos.
- PDF assinado de teste, incluindo assinatura autoassinada e arquivo alterado.
- Arquivo com erro ou recursos não suportados, para avaliar mensagens e recuperação.

### Tarefas comuns

1. Abrir o arquivo e chegar à primeira página legível.
2. Buscar um termo, visitar uma ocorrência e voltar à posição inicial.
3. Selecionar, marcar, escrever uma nota e desfazer uma alteração.
4. Salvar uma cópia, fechar e reabrir em outro leitor.
5. Reencontrar uma marcação por conteúdo.
6. Exportar os trechos marcados quando disponível.
7. Consultar uma referência e retomar o mesmo parágrafo.
8. Imprimir um intervalo e conferir marcações, escala e nitidez.
9. Repetir o percurso com teclado e verificar foco e rótulos acessíveis.

### Medidas a coletar

Registrar sistema, hardware, versão, plano e configuração. Separar abertura fria de quente. Medir tempo até página legível, latência de busca, picos de memória, CPU em repouso, tempo de execução das tarefas e erros observados. Repetir medições e reportar dispersão, não apenas o melhor caso.

No macOS, comparar inicialmente Tsuro, Preview, PDF Expert, Skim e Acrobat Reader. No Windows, comparar Tsuro, SumatraPDF, PDF-XChange e Acrobat Reader. O benchmark deve comparar os mesmos sistemas e arquivos. Não usar resultados de plataformas diferentes como ranking único de desempenho.

Para interoperabilidade, incluir dois motores distintos. Verificar anotações já embutidas, alterações feitas em outro leitor, salvamento repetido, permissão de escrita, interrupção de gravação e arquivos assinados. Uma cópia preserva o original, mas não garante automaticamente que a assinatura da cópia continue válida.

Recrutar usuários para as tarefas, sem apresentar uma interface como a vencedora esperada. Só após essa rodada decidir se prévia, régua e novos filtros justificam seu custo. Este protocolo está proposto, não executado.

## 9. Decisões e verificação desta entrega

O princípio Experience First levou a priorizar o percurso de leitura e a rejeitar a contagem de funcionalidades como medida de qualidade. O princípio Prove It Works levou a conferir o código atual do Tsuro e a não tratar documentação de concorrentes como benchmark.

A indexação da raiz apresentou falha de integridade. A indexação limitada a `crates/tsuro` concluiu, permitindo localizar os trechos citados. Os trechos foram lidos antes de aparecerem no relatório. Não houve alteração de código, instalação de dependências, execução de testes, commit, push ou PR. Testes de software não foram executados porque esta entrega é pesquisa documental, não mudança no produto.

A divergência entre README e anotações implementadas foi registrada aqui para revisão futura. Não foi criada issue externa. O artefato final distingue capacidades documentadas, inferências e propostas, e mantém o roteiro necessário para validar as conclusões na prática.

## Fontes

Fontes consultadas em 12 de outubro de 2026. Fontes de fornecedores documentam o que eles declaram oferecer, não constituem avaliação independente de qualidade.

- [S01. Adobe Acrobat Reader, diferença entre Reader e produtos pagos](https://www.adobe.com/uk/acrobat/pdf-reader.html). Resultado indexado. Fetch direto bloqueado.
- [S02. Adobe, Basic PDF printing tasks](https://helpx.adobe.com/acrobat/using/basic-pdf-printing-tasks-challenger.html).
- [S03. Adobe Trust Services](https://helpx.adobe.com/acrobat/kb/trust-services.html).
- [S04. Foxit PDF Reader](https://www.foxit.com/pdf-reader/).
- [S05. Foxit PDF Reader User Manual 2025.2](https://cdn01.foxitsoftware.com/pub/foxit/manual/reader/en_us/foxit-pdf-reader-user-manual-2025.2.pdf). Trechos indexados do manual.
- [S06. PDF-XChange Editor V11, Features Overview](https://help.pdf-xchange.com/pdfxt11/features-overview.html).
- [S07. PDF Expert, View, search, and remove annotations](https://support.readdle.com/pdfexpert/en_US/annotate-pdfs/view-and-export-annotation-summary).
- [S08. PDF Expert, Export Annotation Summary](https://support.readdle.com/pdfexpert/en_US/annotate-pdfs/export-annotation-summary).
- [S09. PDF Expert, Night or Sepia theme](https://support.readdle.com/pdfexpert/en_US/reading-pdfs/turn-on-the-night-or-sepia-theme).
- [S10. Nitro PDF Pro, Comment Tools](https://www.gonitro.com/user-guide/pro/article/combine-comments-from-multiple-files).
- [S11. Bluebeam, Track and manage markups using the Markups List](https://support.bluebeam.com/revu/how-to/track-and-manage-markups-using-markups-list.html).
- [S12. PDFelement, Comment Summary on Windows](https://pdf.wondershare.com/guide/use-comment-summary.html).
- [S13. Xodo PDF Studio](https://xodo.com/pdf-studio).
- [S14. Xodo, Annotating and Commenting PDFs](https://feedback.xodo.com/support/solutions/articles/35000251170-annotating-and-commenting-pdfs). Resultado indexado.
- [S15. LiquidText, Features](https://www.liquidtext.net/features).
- [S16. SumatraPDF, Editing annotations](https://www.sumatrapdfreader.org/docs/Editing-annotations).
- [S17. SumatraPDF documentation](https://www.sumatrapdfreader.org/docs/SumatraPDF-documentation). Resultado indexado.
- [S18. Okular, Annotations](https://docs.kde.org/stable_kf6/en/okular/okular/annotations.html).
- [S19. Okular, Performance](https://docs.kde.org/stable_kf6/en/okular/okular/configperformance.html).
- [S20. Okular, Accessibility](https://docs.kde.org/stable_kf6/en/okular/okular/configaccessibility.html). Resultado indexado.
- [S21. Sioyek, recursos e demonstrações](https://sioyek.info/).
- [S22. Skim, Taking snapshots](https://skim-app.sourceforge.io/manual/SkimHelp_23.html).
- [S23. Skim, Showing the reading bar](https://skim-app.sourceforge.io/manual/SkimHelp_30.html).
- [S24. Zotero PDF Reader and Note Editor](https://www.zotero.org/support/pdf_reader).
- [S25. Zotero, Why annotations are stored in its database](https://www.zotero.org/support/kb/annotations_in_database).
- [S26. Zathura, apresentação oficial](https://pwmt.org/projects/zathura/).
- [S27. GNOME Papers, Adding annotations](https://help.gnome.org/papers/annotations.html).
- [S28. Evince, Adding annotations](https://help.gnome.org/evince/annotations.html). Resultado indexado.
- [S29. Firefox, View and edit PDF files](https://support.mozilla.org/en-US/kb/view-pdf-files-firefox-or-choose-another-viewer?redirectlocale=en-US&redirectslug=view-pdf-files-firefox-without-downloading-them). Resultado indexado. Fetch direto da URL canônica retornou desafio.
- [S30. Apple Preview, Highlight, underline, and strike out text](https://support.apple.com/guide/preview/highlight-underline-and-strike-out-text-prvw757d4d5b/mac).
- [S31. Adobe Community, Setting up digital signature validation](https://community.adobe.com/questions-9/setting-up-digital-signature-validation-in-acrobat-reader-1257530). Orientação publicada por integrante identificado da equipe Adobe, de 2021. Apoio conceitual, não prova de configuração atual.
- [S32. Lei 9.610, texto disponibilizado pelo governo federal](https://www.gov.br/propriedade-intelectual/pt-br/legislacao/direitos-autorais/legislacao-nacional/lei-9610.pdf/@@display-file/file). Art. 8º consultado via resultado indexado.
- [S33. Lei 9.609, Planalto](https://planalto.gov.br/ccivil_03/leis/l9609.htm). Resultado indexado.
