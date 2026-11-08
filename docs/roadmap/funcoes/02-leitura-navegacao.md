# Funções de leitura e navegação

Nove requisitos propostos. O coordenador revisou a entrega Step 5 Preview de 13 de outubro de 2026. Nenhuma função foi implementada, testada ou aprovada para release.

Os fatos técnicos vêm de `../_insumos/01-estado-do-produto-2026-10-13.md`, não revalidado nesta rodada. Âncoras não confirmam comportamento atual. Busca textual não prova ausência. O protocolo de execução está em `../_insumos/05-protocolo-retomada.md`.

Prioridades e WSJF são propostas, não compromissos de calendário. A conta usa valor, urgência ou redução de risco, oportunidade e tamanho de 1 a 5. Segurança, acessibilidade, dependências e capacidade prevalecem sobre a pontuação. Codinomes ficam na documentação até revisão linguística.

## Preservar o contexto

### F-NAV-01. Voltar depois de um salto (Kaerimichi)

- **Ficha.** completa
- **Domínio e tipo.** navegar, evolução da descoberta do histórico existente.
- **Promessa.** O leitor consulta uma referência e retorna ao lugar de onde saiu.
- **Origem (evidência).** `codigo-e-usuarios.md § 3`, voltar depois de seguir um link. `codigo-e-usuarios.md § 4.6`, histórico. `relatorio.md § 5.2`, preservar contexto.
- **Não copiar.** Não reproduzir gestos ou canvas de LiquidText, nem criar outro histórico paralelo.
- **Mission gate.** Passa. Torna ler referências menos interrompido.
- **Situação atual no Tsuro.** O snapshot registra histórico e atalhos. Âncoras não revalidadas são `crates/tsuro/src/session.rs:2355`, `crates/tsuro/src/session.rs:2356` e `crates/tsuro/src/view.rs:202`. O incremento é descoberta e entrada por mouse, não introdução do histórico.
- **História de usuário.** Como leitor que consulta notas e figuras, quero retornar à origem de um salto para continuar o parágrafo.
- **Critérios de aceite.** Verificar os seguintes comportamentos.
  - Dado um salto concluído, Quando o destino aparece, Então uma ação transitória Voltar indica a origem e usa o histórico existente.
  - Dado a ação visível, Quando o leitor usa teclado, Então pode acioná-la sem foco preso ou dependência de mouse.
  - Dado um retorno válido, Quando a origem reaparece, Então restaura âncora e os parâmetros de visualização registrados para essa origem, sem criar uma entrada de histórico duplicada.
  - Dado um mouse com ações Back e Forward, Quando elas são acionadas, Então têm o mesmo efeito dos comandos de histórico correspondentes.
  - Dado histórico vazio, Quando Voltar é acionado, Então mantém o documento e não apresenta erro.
  - Dado uma origem inválida depois de mudança de arquivo, Quando o retorno é solicitado, Então não salta a conteúdo errado e informa a indisponibilidade.
- **Experiência no Kiri.** Ação discreta perto da navegação existente. Não ocultar enquanto tiver foco ou estiver sendo usada. Sem timer que retire um alvo de teclado. Escolher permanência por estado, com fechamento previsível. Reusar histórico da aba, sem adicionar barra permanente.
- **Expressão temática.** Kaerimichi, 帰り道, caminho de volta, como codinome. Reusar seta Ori existente. PT "Voltar ao ponto de leitura" e "Ponto de leitura indisponível." EN "Return to reading position" e "Reading position unavailable."
- **Dados e código.** Revalidar a estrutura de histórico antes de propor campos. Se faltar contexto, ampliar a entrada existente com âncora, modo e parâmetros necessários, mantendo estado em `session.rs`. `view.rs` exibe ação. Mapear eventos de mouse ao mesmo comando. Não alterar Pdfium ou `tsuro-sign`.
- **Testes.** Saltar, voltar e avançar entre links, sumário e marcações. Verificar zoom, rotação, modo e âncora sem presumir suporte atual. Histórico vazio, referência inválida e teclado. Medir custo contra S-06.
- **Riscos e premissas.** Timer de desaparecimento pode retirar foco e bloquear uso. Evitar expiração enquanto focado. Entradas incompletas podem retornar à página sem retornar ao parágrafo. Conferir estrutura existente. Suposição, o histórico atual pode atender sem outro subsistema.
- **Dependências.** Histórico existente e S-06. F-NAV-04 melhora descoberta. Lista e paleta de marcações não precisam esperar o novo chip para usar o histórico existente.
- **Tamanho e confiança.** M, confiança média. Incógnitas de contexto guardado, foco e eventos de mouse por plataforma.
- **Prioridade sugerida.** Should. Valor 3, urgência 2, oportunidade 2, tamanho 2. WSJF = (3+2+2)/2 = 3,5.
- **Definição de pronto.** Comportamento real e teclado em macOS e Windows, PT e EN e nota de release. `cargo test -p tsuro`, `cargo fmt --check` e `cargo clippy`, com resultados registrados. Medição de regressão por S-06.
- **Minuta local da issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(nav): tornar o retorno de saltos descobrível`
  - Labels: `enhancement`, `navigation`, `ux`, `needs-triage`
  - Corpo: Reusar histórico existente. Expor retorno transitório e eventos de mouse. Preservar foco e contexto da origem. Não criar histórico paralelo ou alvo que desaparece durante uso.

### F-NAV-02. Rótulo de página impresso (Fólio)

- **Ficha.** completa
- **Domínio e tipo.** navegar, evolução da identificação de páginas.
- **Promessa.** O leitor distingue o rótulo informado pelo PDF da posição física da página.
- **Origem (evidência).** `codigo-e-usuarios.md § 3`, página do arquivo e do livro. `codigo-e-usuarios.md § 4.7`, labels e exportação. `relatorio.md § 5.4`, origem das marcações exportadas.
- **Não copiar.** Não copiar editor de labels de suítes nem inferir números visuais por OCR.
- **Mission gate.** Passa. Torna localização e revisão de marcações mais claras.
- **Situação atual no Tsuro.** O snapshot registra contador e disponibilidade de API de label. Âncoras não revalidadas são `crates/tsuro/src/view.rs:202` e `crates/tsuro/src/session.rs:411`. O nome do catálogo é mantido, mas a v1 lê metadados, não comprova correspondência com o número impresso na imagem.
- **História de usuário.** Como leitor de livro com páginas preliminares, quero usar labels do arquivo para localizar uma referência sem confundir numeração e posição física.
- **Critérios de aceite.** Verificar os seguintes comportamentos.
  - Dado um PDF com label romano válido, Quando a página aparece, Então exibe esse label e a posição física em base 1, sem expor `PageNo` em base 0.
  - Dado um label repetido, Quando o leitor consulta esse label, Então mostra candidatos com posições físicas e não escolhe silenciosamente.
  - Dado label numérico que aponta a outra posição física, Quando a consulta é ambígua, Então explicita as duas interpretações antes de saltar.
  - Dado um PDF sem labels, Quando navega ou exporta marcações, Então continua usando posição física em base 1.
  - Dado falha de extração ou label inválido, Quando o documento abre, Então mantém leitura e posição física, sem ocultar o erro por um salto inventado.
  - Dado exportação com labels, Quando o texto é gerado, Então mantém label original legível e posição física para desambiguação.
- **Experiência no Kiri.** Contador compacto e detalhe de posição. Campo de navegação aceita modo explícito de posição ou label. Lista de candidatos funciona com teclado. Labels longos ou com controles são tratados para exibição segura sem perder o valor original. Acessibilidade anuncia label e posição como informações diferentes.
- **Expressão temática.** Fólio é codinome em português. Sem ícone novo. PT "Rótulo do PDF" e "Posição no arquivo". EN "PDF page label" e "Position in file". Não usar "número impresso" como promessa de OCR.
- **Dados e código.** Propor `PageLabel` opcional e mapa de label para candidatos `PageNo`. Guardar valor original e chave de consulta separadamente. Não colapsar `i`, `I` e labels prefixados sem regra explícita. Extrair por motor e exibir em `view.rs`; manter índices internos em base 0. Não alterar conteúdo ou `tsuro-sign`.
- **Testes.** Labels romanos, prefixados, duplicados, numéricos ambíguos, ausentes, longos e inválidos. Criar fixture nova por método aprovado; não presumir que as fixtures atuais contenham labels. Testar navegação e exportação com fallback.
- **Riscos e premissas.** Label incorreto pode aparentar número visual que não existe. Identificar origem como metadado. Normalização agressiva pode juntar labels diferentes. Preservar original e desambiguar. Suposição, extração não bloqueia primeira página e pode ser feita sob demanda.
- **Dependências.** S-06 para custo e verificação da API instalada antes de implementar. Consumidores F-MARC-02, F-MARC-08 e F-ARQ-05 continuam com posição física na v1 e não esperam esta função.
- **Tamanho e confiança.** M, confiança média. Incógnitas de extração, normalização, ambiguidades e disponibilidade de fixtures.
- **Prioridade sugerida.** Should. Valor 4, urgência 3, oportunidade 3, tamanho 2. WSJF = (4+3+3)/2 = 5.
- **Definição de pronto.** Navegação e exportação desambiguadas em aplicação real. PT e EN, nota de release, `cargo test -p tsuro`, `cargo fmt --check`, `cargo clippy`, macOS e Windows. Custo comparado com S-06.
- **Minuta local da issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(nav): distinguir labels do PDF e posições físicas`
  - Labels: `enhancement`, `navigation`, `needs-triage`
  - Corpo: Ler metadados de página e manter posição física. Desambiguar consultas repetidas ou numéricas. Não usar OCR nem bloquear consumidores sem labels. Preservar índices internos em base 0.

## Copiar e descobrir ações

### F-NAV-03. Copiar como parágrafo (Tsunagi)

- **Ficha.** completa
- **Domínio e tipo.** ler, evolução opt-in da cópia de seleção.
- **Promessa.** O leitor copia um parágrafo contínuo sem perder palavras ou unir colunas diferentes.
- **Origem (evidência).** `codigo-e-usuarios.md § 3`, problemas de cópia. `codigo-e-usuarios.md § 4.4`, junção conservadora. `relatorio.md § 2`, cópia existente.
- **Não copiar.** Não reproduzir reflow de editor, modelo generativo ou heurística de terceiros. Não alterar silenciosamente o texto original.
- **Mission gate.** Passa condicionada a validação. Ajuda a levar um trecho de leitura para outra tarefa.
- **Situação atual no Tsuro.** O snapshot registra cópia do texto bruto. Âncoras não revalidadas são `crates/tsuro/src/session.rs:5431` e `crates/tsuro/src/page.rs:224`. Ordem e geometria de linhas precisam de confirmação.
- **História de usuário.** Como leitor que copia trechos, quero um comando de parágrafo para evitar remover manualmente quebras de linha.
- **Critérios de aceite.** Verificar os seguintes comportamentos.
  - Dado linhas do mesmo parágrafo com geometria validada, Quando usa Copiar como parágrafo, Então junta com espaços sem perder caracteres.
  - Dado hífen lexical ou caso ambíguo, Quando a junção é avaliada, Então preserva o hífen e não inventa uma palavra.
  - Dado hifenização de quebra identificada pelos critérios de S-07, Quando o comando copia, Então remove somente a quebra e o hífen de divisão comprovado.
  - Dado tabela, lista, linhas em branco ou colunas distintas, Quando o comando copia, Então preserva limites estruturais que não tenham confiança para junção.
  - Dado geometria ou ordem indisponível, Quando o comando é solicitado, Então usa o texto original e informa que não aplicou junção segura.
  - Dado falha da área de transferência, Quando a escrita falha, Então não informa sucesso nem altera seleção ou PDF.
- **Experiência no Kiri.** Comando separado e explícito na paleta ou menu contextual. Cópia original continua padrão nesta proposta. Informar resultado sem alertas excessivos. Nenhuma nova barra ou processamento remoto.
- **Expressão temática.** Tsunagi, 繋ぎ, junção, sujeito a revisão. Sem ícone novo. PT "Copiar como parágrafo" e "Texto original copiado. Não foi possível unir com segurança." EN "Copy as paragraph" e "Original text copied. Safe joining was unavailable."
- **Dados e código.** Propor função pura sobre linhas com texto, geometria, ordem e confiança. O resultado distingue texto unido e preservado. Só texto bruto não prova separação de colunas. `session.rs` integra cópia; `page.rs` fornece dados existentes se suficientes. Não alterar marcações nem normalizar exportação automaticamente. Mudança de extração do motor só se declarada após S-07.
- **Testes.** Corpus local PT e EN com prosa, hifenização, compostos legítimos, duas colunas, tabelas e listas. Comparar saída com referência humana. Cópia padrão não muda. Verificar erro da área de transferência e ausência de geometria.
- **Riscos e premissas.** Junção incorreta pode modificar significado. Preservar texto quando incerto e permitir comando original. Extração pode ter ordem errada antes da junção. Avaliar origem das linhas no spike. Suposição, uma v1 conservadora ajuda sem dicionário ou dependência nova.
- **Dependências.** S-07 antes de compromisso de implementação. S-04 contribui se for preciso investigar geometria. F-NAV-04 divulga comando. Exportação F-MARC-08 não depende nem passa a usar junção automaticamente.
- **Tamanho e confiança.** G, confiança baixa. Incógnitas de ordem, classificação de hífen, qualidade do corpus e custo.
- **Prioridade sugerida.** Should se S-07 for favorável. Valor 3, urgência 2, oportunidade 2, tamanho 3. WSJF = (3+2+2)/3 = 2,33. Se o spike falhar, adiar.
- **Definição de pronto.** Corpus e limites documentados, original preservado, erros de cópia tratados. PT e EN, nota de release, `cargo test -p tsuro`, `cargo fmt --check`, `cargo clippy` e verificação real em macOS e Windows. Medir com S-06.
- **Minuta local da issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(read): copiar parágrafos com junção conservadora`
  - Labels: `enhancement`, `ux`, `needs-triage`
  - Corpo: Comando opt-in, sem alterar cópia padrão. Validar texto e geometria em corpus PT e EN. Preservar hifens e estruturas ambíguas. S-07 pode concluir que a função deve ser adiada.

### F-NAV-04. Paleta com todos os comandos (sem codinome)

- **Ficha.** completa
- **Domínio e tipo.** navegar, evolução da descoberta de comandos.
- **Promessa.** O leitor descobre ações e atalhos sem memorizar comandos ocultos.
- **Origem (evidência).** `ui-ux.md § 5.1`, comandos e descoberta. `ui-ux.md § 3`, interfaces dos leitores pesquisados. `codigo-e-usuarios.md § 2`, coleta sob demanda no Sumatra.
- **Não copiar.** Não copiar registro de comandos, ranking ou nomes de concorrentes. Manter escopo do Tsuro.
- **Mission gate.** Passa. Torna ler, anotar, marcar e imprimir mais descobríveis.
- **Situação atual no Tsuro.** A paleta já existe no snapshot. Âncoras não revalidadas são `crates/tsuro/src/session.rs:599`, `crates/tsuro/src/session.rs:1007` e `crates/tsuro/src/session.rs:5088`. Não fixar como requisito a quantidade antiga de comandos faltantes.
- **História de usuário.** Como leitor que não conhece os atalhos, quero procurar uma ação pelo nome para executá-la no contexto correto.
- **Critérios de aceite.** Verificar os seguintes comportamentos.
  - Dado um comando suportado com atalho, Quando a paleta é consultada, Então exibe nome e atalho coerentes com sua execução no sistema atual.
  - Dado um comando indisponível, Quando a paleta o mostra, Então explica a condição necessária e não permite ação inválida.
  - Dado foco num campo de texto, Quando uma tecla de edição é usada, Então não dispara uma ação global que consuma a digitação.
  - Dado um comando novo registrado, Quando ele é integrado, Então o teste de cobertura exige descoberta e execução, salvo exceção explícita justificada.
  - Dado uma exceção de navegação ou edição local, Quando a cobertura é verificada, Então não obriga essa tecla a virar item de paleta.
  - Dado uma ação cujo contexto muda antes da confirmação, Quando o leitor confirma, Então revalida disponibilidade e não despacha ação inválida.
- **Experiência no Kiri.** Reusar cartão de paleta. Nomes PT e EN, estado indisponível, foco e dica de atalho. Evitar ocultar todas as ações sem explicar contexto. Não converter teclas de edição locais em comandos globais.
- **Expressão temática.** Sem codinome novo. Reusar gramática Ori existente. PT "Indisponível neste contexto". EN "Unavailable in this context." Sem prefixos secretos como única descoberta.
- **Dados e código.** Propor registro com ID, nome, atalho por plataforma, condição e ação. Paleta e hints compartilham metadados. Disponibilidade também é validada no despacho. Estado em `session.rs`, interface em `view.rs`. Migrar consumidores existentes em unidades pequenas, sem registro legado permanente.
- **Testes.** Cobertura do registro, atalhos por plataforma, equivalência de resultado entre paleta e entrada direta, foco de texto e mudança de contexto. Não usar uma contagem fixa nem um teste que derive registro e expectativa da mesma lista sem verificar atalhos reais.
- **Riscos e premissas.** Registro parcial pode manter divergência de atalhos. Testar consumidores externos ao próprio registro. Comando inválido pode ser despachado após abrir paleta. Revalidar no limite. Suposição, centralização cabe numa mudança pequena sem nova camada genérica.
- **Dependências.** Paleta existente e S-06. Fornece fundação para novos comandos. Não depende da lista de marcações ou das funções consumidoras. Consumidor pode acrescentar um item à infraestrutura atual se a migração total ainda não estiver entregue.
- **Tamanho e confiança.** M, confiança média. Incógnitas de atalhos por plataforma, exceções e foco.
- **Prioridade sugerida.** Should. Valor 3, urgência 2, oportunidade 3, tamanho 2. WSJF = (3+2+3)/2 = 4.
- **Definição de pronto.** Registro e dispatch coerentes, exceções documentadas, teclado real em macOS e Windows. PT, EN e nota de release. `cargo test -p tsuro`, `cargo fmt --check`, `cargo clippy` e comparação com S-06.
- **Minuta local da issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(nav): unificar descoberta e atalhos de comandos`
  - Labels: `enhancement`, `navigation`, `accessibility`, `needs-triage`
  - Corpo: Registrar comandos suportados e condições. Testar atalhos reais e despacho, sem quantidade fixa. Preservar edição de texto e justificar exceções. Não bloquear novos consumidores pela migração completa.

## Retomar sem confundir documentos

### F-NAV-05. Posição que acompanha o conteúdo (Shiori)

- **Ficha.** completa
- **Domínio e tipo.** navegar, evolução da retomada existente.
- **Promessa.** O leitor recupera o ponto de leitura quando muda o lugar de um PDF sem receber a posição de outro documento.
- **Origem (evidência).** `codigo-e-usuarios.md § 2`, identidade por conteúdo no Sioyek. `codigo-e-usuarios.md § 5`, posição pelo conteúdo. `relatorio.md § 2`, retomada existente.
- **Não copiar.** Não copiar tabelas ou algoritmo de identificação de outro leitor. Não usar nuvem ou identificador que revele conteúdo externamente.
- **Mission gate.** Passa condicionada à segurança da identificação e ao custo.
- **Situação atual no Tsuro.** O snapshot registra retomada por caminho, tamanho e modificação. Âncoras não revalidadas são `crates/tsuro/src/positions.rs:81` e `crates/tsuro/src/positions.rs:236`.
- **História de usuário.** Como leitor que reorganiza pastas, quero reconhecer um PDF movido para retomar sem abrir o caminho antigo.
- **Critérios de aceite.** Verificar os seguintes comportamentos.
  - Dado o mesmo caminho e identidade válida, Quando o leitor abre, Então usa a posição vinculada àquela instância.
  - Dado um PDF movido com uma única origem conhecida e conteúdo idêntico confirmado, Quando ele é aberto no novo caminho, Então oferece retomar sem exigir abrir o original ausente.
  - Dado duas cópias idênticas usadas em paralelo, Quando cada caminho conhecido é aberto, Então preserva posições independentes por instância.
  - Dado novo caminho com várias posições candidatas para o mesmo conteúdo, Quando a retomada é oferecida, Então explicita candidatos e não escolhe silenciosamente o mais recente.
  - Dado arquivo editado ou confirmação de identidade inconclusiva, Quando abre, Então não restaura uma âncora de conteúdo diferente.
  - Dado erro de leitura ou custo acima do orçamento aprovado, Quando identifica conteúdo, Então permite abrir sem retomada por conteúdo e informa o limite, sem enviar bytes à rede.
- **Experiência no Kiri.** Reusar retomada e aviso de arquivo alterado. Só mostrar escolha quando ambígua. Foco e texto explicam documento, instância e posição sem expor caminhos em exportação. Nada permanente na barra.
- **Expressão temática.** Shiori, 栞, marcador de livro, como codinome. Reusar marcador Ori quando apropriado, sem outra metáfora de navegação. PT "Retomar a leitura desta cópia?" e "Há mais de um ponto de leitura para este conteúdo." EN "Resume reading this copy?" e "More than one reading position matches this content."
- **Dados e código.** Propor identidade de conteúdo separada de identidade da instância e de posição. Hash local é candidato a confirmar conteúdo, não prova de equivalência sem algoritmo e validação definidos. Hash parcial só seleciona candidatos. Calcular fora do caminho crítico quando possível. Persistência em `positions.rs` ou módulo próprio, com formato versionado. Não unificar sessões, diários ou marcadores de cópias idênticas automaticamente.
- **Testes.** Mover, copiar, editar, alterar metadados, ter candidatos conflitantes e simular erro de leitura. Corpus com arquivos grandes. Testar orçamento de abertura com e sem identificação. Identidade inconclusiva nunca autoriza salto silencioso.
- **Riscos e premissas.** Mesmo conteúdo com usos distintos pode misturar posições ou notas. Separar instâncias e pedir escolha em ambiguidade. Hash integral na abertura pode prejudicar primeira página. Medir custo e usar trabalho posterior limitado. Suposição, é possível confirmar identidade sem dependência nova.
- **Dependências.** S-06 e investigação local de identidade antes da função completa. Alinhar contrato mínimo com F-MARC-01 e F-MARC-07. Eles não esperam retomada entre caminhos para lançar suas v1 seguras.
- **Tamanho e confiança.** G, confiança baixa. Incógnitas de hash, migração, ambiguidades, retenção e custo de I/O.
- **Prioridade sugerida.** Could. Valor 3, urgência 2, oportunidade 2, tamanho 3. WSJF = (3+2+2)/3 = 2,33.
- **Definição de pronto.** Contrato de instância e conteúdo documentado, corpus sem restauração errada e custo aprovado. PT, EN, nota de release, `cargo test -p tsuro`, `cargo fmt --check`, `cargo clippy` e aplicação real em macOS e Windows.
- **Minuta local da issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(nav): retomar PDFs movidos sem misturar cópias`
  - Labels: `enhancement`, `navigation`, `needs-triage`
  - Corpo: Separar conteúdo, instância e posição. Reconhecer movimento seguro e desambiguar cópias idênticas. Medir custo de identificação. Não aplicar posição antiga a conteúdo editado.

### F-NAV-06. Rolagem natural (sem codinome)

- **Ficha.** curta
- **Promessa.** O leitor rola com direção, precisão e resposta compatíveis com seu dispositivo e suas preferências.
- **Origem (evidência).** `codigo-e-usuarios.md § 5`, trackpad e roda. Pedidos de outros leitores não comprovam defeito no Tsuro.
- **Não copiar.** Não copiar multiplicador universal nem substituir preferência de direção do sistema.
- **Mission gate.** Ajuste. Só modificar input quando houver problema reproduzível ou ganho observado.
- **Situação atual no Tsuro.** O snapshot não confirma defeito de execução. É necessária avaliação real em macOS e Windows, não uma inferência pela ausência de código especial.
- **Hipótese e como validar.** Registrar direção, velocidade, passos, inércia e ancoragem com trackpad, roda e dispositivos disponíveis. Comparar preferências ligadas e desligadas. Testar rolagem curta, longa e zoom. Sensibilidade incorreta também conta, não só inversão de direção.
- **Critério de saída.** Corrigir apenas cenário reproduzível. A mesma bateria precisa continuar seguindo preferências do sistema, sem salto de páginas ou atraso de resposta. Se não houver defeito nem ganho de usabilidade, recusar incremento e guardar resultado.
- **Dependências.** Diagnóstico real e S-06. Não bloqueia outras funções.
- **Tamanho e prioridade.** P ou M após diagnóstico. Could, confiança baixa. Se surgir defeito de leitura grave, reclassificar como correção prioritária com evidência.
- **Expressão temática.** Sem codinome, sem ícone e sem preferência nova antes de provar necessidade. Rótulo eventual PT "Rolagem" e EN "Scrolling".
- **Minuta local da issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(nav): avaliar rolagem por dispositivo e plataforma`
  - Labels: `enhancement`, `navigation`, `ux`, `needs-triage`
  - Corpo: Reproduzir direção, sensibilidade e resposta. Respeitar preferência do sistema. Não aplicar uma configuração universal baseada em queixa de outro leitor. Registrar recusa se não houver problema.

## Investigar incrementos de leitura

### F-NAV-07. Espiar o destino de um link (Nozoki)

- **Ficha.** curta
- **Promessa.** O leitor consulta um destino interno sem abandonar o parágrafo atual.
- **Origem (evidência).** `ui-ux.md § 5.7`, prévia. `codigo-e-usuarios.md § 5`, links internos. `relatorio.md § 5.2`, contexto da leitura.
- **Não copiar.** Não reproduzir canvas ou gestos distintivos de LiquidText, nem inferir referências acadêmicas sem link declarado.
- **Mission gate.** Passa se prévia local reduzir interrupção sem custo desproporcional.
- **Situação atual no Tsuro.** A existência de extração e geometria para essa prévia não foi revalidada. O histórico já é registrado no snapshot, mas não comprova suporte a prévia.
- **Hipótese e como validar.** S-02 investiga destinos internos declarados, geometria, recorte e custo. Preparar fixture que realmente tenha links internos, sem presumir que bookmarks de sumário sejam links na página. Comparar consulta com salto e retorno.
- **Critério de saída.** Exibir destino correto, fechar devolvendo foco e posição, e oferecer salto explícito. Limitar uma prévia ativa e seu custo. Link quebrado informa ausência. Não executar URI, JavaScript, download ou rede. Sem validação de geometria e orçamento, adiar.
- **Dependências.** S-02 é rígida. S-06 fornece orçamento. Histórico existente permite salto opcional. F-NAV-01 e F-NAV-04 melhoram descoberta sem bloquear a prévia transitória.
- **Tamanho e prioridade.** G, Could, confiança baixa. Incógnitas de link, destino, recorte, foco e memória.
- **Expressão temática.** Nozoki, 覗き, espiar, codinome interno. Ori candidato com duas folhas parcialmente sobrepostas e vinco no canto. PT "Espiar destino" e "Ir para o destino". EN "Preview destination" e "Go to destination".
- **Minuta local da issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(nav): avaliar prévia local de links internos`
  - Labels: `enhancement`, `navigation`, `ux`, `needs-triage`
  - Corpo: Investigar S-02 e usar links declarados. Prévia transitória acessível por teclado. Fechar sem mover origem. Não executar ações externas nem buscar antecipadamente sem limite.

### F-NAV-08. Sumário gerado (Kari-mokuji)

- **Ficha.** curta
- **Promessa.** O leitor dispõe de uma estrutura provisória quando o PDF não contém sumário autoral.
- **Origem (evidência).** `codigo-e-usuarios.md § 5`, sumário gerado e contra-evidência sobre erros. `ui-ux.md § 5.1`, descoberta de navegação.
- **Não copiar.** Não copiar heurística ou interface de extensão, nem tratar resultado inferido como estrutura do autor.
- **Mission gate.** Ajuste. Só passa quando orientação útil supera o risco de navegação enganosa.
- **Situação atual no Tsuro.** O snapshot registra sumário existente. Âncora não revalidada é `crates/tsuro/src/view.rs:2545`. Detecção de títulos não está confirmada.
- **Hipótese e como validar.** S-04 avalia geometria. Em experimento separado, comparar títulos inferidos com referência humana em PDFs sem outline, prosa e tabelas. Definir antes da avaliação o limite de erros e custo. Não escolher o limite depois de ver resultado.
- **Critério de saída.** Avançar somente se a amostra cumprir limite aprovado. Identificar sempre como gerado, permitir ignorar, não substituir outline autoral e não gravar no PDF. Adiar ou recusar se confundir tabelas e cabeçalhos com títulos.
- **Dependências.** S-04 e experimento de inferência. S-06 para custo. Nenhuma lista de marcações depende desta função.
- **Tamanho e prioridade.** G, Won't na primeira sequência, confiança baixa. Incógnitas de atributos do texto e precisão.
- **Expressão temática.** Kari-mokuji, 仮目次, sumário provisório, sujeito a revisão. Reusar ícone de sumário. PT "Sumário gerado. Pode conter erros." EN "Generated outline. May contain errors."
- **Minuta local da issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(nav): avaliar sumário provisório para PDFs sem outline`
  - Labels: `enhancement`, `navigation`, `ux`, `needs-triage`
  - Corpo: Comparar inferência com referência humana e custo. Identificar origem provisória e permitir ignorar. Não substituir estrutura autoral ou gravar o original. Recusa é resultado válido.

### F-NAV-09. Seleção por teclado (sem codinome)

- **Ficha.** curta
- **Promessa.** O leitor seleciona trechos para copiar e marcar sem depender de apontar com mouse.
- **Origem (evidência).** `codigo-e-usuarios.md § 5`, seleção por teclado e Zathura. `relatorio.md § 5.3`, fluxo de marcação por teclado.
- **Não copiar.** Não copiar vocabulário de teclas ou modo do navegador. Não anunciar compatibilidade de leitor de tela só por haver navegação de caret.
- **Mission gate.** Passa. Remove uma barreira de ler e marcar. É uma das tarefas de acessibilidade do pacote, não a única.
- **Situação atual no Tsuro.** O snapshot registra teclado centralizado em `crates/tsuro/src/session.rs:4889`, não revalidado. Seleção e exposição acessível do texto precisam de confirmação real.
- **Hipótese e como validar.** Prototipar foco, cursor, extensão por caractere e palavra, e operações de seleção. Avaliar ordem de leitura, colunas, tabelas, rotação, zoom e anúncio por tecnologia assistiva. Testar navegação por teclado separadamente de leitura de tela.
- **Critério de saída.** Seleção acompanha texto na ordem validada e funciona sem perda de foco. Identificar explicitamente layouts ainda não suportados. Fazer revisão de acessibilidade antes de comprometer implementação ampla. Se houver bloqueio real de usuários, reavaliar prioridade em vez de adiar só por tamanho.
- **Dependências.** Investigação de foco e acessibilidade, S-04 quando geometria for necessária e S-06. Lista e busca de marcações continuam acessíveis por teclado sem esperar seleção de texto completa.
- **Tamanho e prioridade.** GG, Won't nesta sequência de implementação, confiança baixa. A investigação de barreiras deve ocorrer antes de decidir um adiamento longo.
- **Expressão temática.** Sem codinome ou ícone novo. PT "Selecionar texto com teclado". EN "Select text with keyboard". Anúncios de posição e seleção dependem de mecanismo acessível verificado.
- **Minuta local da issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(read): investigar seleção de texto por teclado`
  - Labels: `enhancement`, `accessibility`, `needs-triage`
  - Corpo: Avaliar foco, ordem e seleção em layouts reais. Separar caret de leitor de tela. Registrar limites e barreiras observadas. A investigação pode justificar aumento de prioridade.

## Dependências e decisões pendentes

| Função | Bloqueio de compromisso | Melhorias opcionais |
| --- | --- | --- |
| F-NAV-01 | Revalidar contexto do histórico existente | F-NAV-04 |
| F-NAV-02 | Confirmar extração e desambiguação | S-06 |
| F-NAV-03 | S-07 favorável | S-04, F-NAV-04 |
| F-NAV-04 | Revalidar atalhos e contextos | S-06 |
| F-NAV-05 | Identidade e política de instâncias validadas | Contrato comum de recuperação e marcadores |
| F-NAV-06 | Defeito ou ganho observado em aplicação real | S-06 |
| F-NAV-07 | S-02 favorável e orçamento aprovado | F-NAV-01, F-NAV-04 |
| F-NAV-08 | S-04 e avaliação de inferência favoráveis | F-NAV-04 |
| F-NAV-09 | Revisão de foco, ordem e acessibilidade | S-04, F-NAV-04 |

O usuário aprova mudanças de interação, orçamento de identidade e âmbito da cópia de parágrafos. A proposta mantém cópia original como padrão. O índice interno em base 0 é regra vigente do repositório, não decisão pendente. A apresentação física em base 1 é a convenção legível proposta, não convite para expor índices internos.

O coordenador corrigiu quatro problemas da entrega. Texto bruto não garante identificação de colunas. Hash de conteúdo não unifica instâncias automaticamente. Custo relativo de um chip não foi medido. Preferências de rolagem incluem precisão e resposta, não só direção.

Não há resultado de protótipo, benchmark ou teste do aplicativo nesta entrega. As âncoras e a disponibilidade de APIs precisam ser conferidas antes de implementar.
