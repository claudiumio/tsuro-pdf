# Funções de marcações

Oito requisitos propostos a partir da pesquisa. Revisão do coordenador sobre a entrega Step 5 Preview de 13 de outubro de 2026. Nenhuma função foi implementada ou aprovada para release.

Os fatos técnicos vêm de `../_insumos/01-estado-do-produto-2026-10-13.md`, não revalidado nesta rodada. As âncoras de código são pontos de partida, não confirmação atual de comportamento. Busca textual não prova ausência. Consulte `../_insumos/05-protocolo-retomada.md` antes de executar.

Prioridades e WSJF são julgamentos propostos. O score usa valor, urgência ou redução de risco, oportunidade e tamanho numa escala de 1 a 5. Não é prazo. Dependências, segurança e capacidade prevalecem. Codinomes são propostas internas, sujeitos a revisão linguística. Rótulos de interface são funcionais.

## Preservar e reencontrar o trabalho

### F-MARC-01. Diário de recuperação das marcações (Orime)

- **Ficha.** completa
- **Domínio e tipo.** marcar, função nova de recuperação das marcações existentes.
- **Promessa.** O leitor recupera marcações de uma sessão interrompida sem alterar o PDF original.
- **Origem (evidência).** `codigo-e-usuarios.md § 3`, perda de trabalho sem aviso. `codigo-e-usuarios.md § 4.1`, persistência e recuperação. `relatorio.md § 6`, confiança na gravação antes de funções novas.
- **Não copiar.** Não copiar o banco do Sioyek, código copyleft ou armazenamento proprietário de projetos. Especificar uma recuperação local própria.
- **Mission gate.** Passa. Preserva o trabalho de anotar e marcar sem transformar o leitor em serviço de nuvem.
- **Situação atual no Tsuro.** O snapshot registra marcações de sessão e cópia anotada. Não confirma um diário. Âncoras não revalidadas são `crates/tsuro/src/session.rs:486`, `crates/tsuro/src/engine.rs:396` e `crates/tsuro/src/positions.rs:81`.
- **História de usuário.** Como leitor que anota, quero recuperar o trabalho após um encerramento inesperado para não marcar o documento novamente.
- **Critérios de aceite.** Verificar os seguintes comportamentos.
  - Dado um checkpoint confirmado com três marcações, Quando o processo é encerrado e o mesmo conteúdo é reaberto, Então restaurar oferece exatamente essas três marcações e o hash do PDF original permanece igual.
  - Dado o mesmo checkpoint já restaurado, Quando a recuperação é tentada novamente, Então não duplica criação, edição ou exclusão de marcações.
  - Dado um arquivo alterado externamente, Quando há uma recuperação pendente, Então bloqueia aplicação automática, preserva o diário e explica a incompatibilidade.
  - Dado um diário truncado ou com sequência inválida, Quando ele é lido, Então o leitor abre o PDF, não aplica operações inconsistentes e mantém disponível o último checkpoint validado, se existir.
  - Dado falha de escrita por disco cheio, Quando a gravação do diário falha, Então o trabalho em memória permanece e a interface informa que a recuperação não está garantida.
  - Dado uma cópia anotada salva, Quando o leitor volta ao original, Então não perde o diário do original nem aplica ao novo arquivo as mesmas marcações em duplicidade.
- **Experiência no Kiri.** Cartão discreto de restauração com Restaurar, Descartar e Decidir depois. Teclado percorre ações e devolve foco ao documento. Estados sem diário, disponível, restaurando, incompatível e falha têm texto. A folha e a barra principal não mudam. Descartar pede confirmação.
- **Expressão temática.** Orime, 折り目, vinco, como codinome interno. Ori proposto com dois polígonos formando uma folha e um vinco diagonal. PT "Marcações recuperáveis encontradas." e "Não foi possível guardar a recuperação." EN "Recoverable annotations found." e "Recovery could not be saved." Sem animação decorativa.
- **Dados e código.** Propor `RecoveryCheckpoint`, identidade de conteúdo, versão de esquema, IDs de marcação, sequência de operações e estado de recuperação explícito em `crates/tsuro/`. Preservar geometria e ancoragem necessárias, não só texto. Estado em `session.rs`, interface em `view.rs`, persistência em módulo próprio. Escrever temporário, validar e substituir atomicamente. Não alterar Pdfium nem `tsuro-sign`. Anotações do PDF e da sessão têm origens distintas.
- **Testes.** Criar, editar, excluir, desfazer, recuperar duas vezes e simular truncamento, disco cheio e mudança externa. Comparar hash do original antes e depois. Usar cópias de fixtures, nunca sobrescrevê-las. Medir atraso entre ação e checkpoint. A meta de até cinco segundos é proposta, não resultado medido.
- **Riscos e premissas.** Conteúdo alterado pode levar a recuperação em trecho errado. Mitigar com identidade e validação conservadora. Notas em arquivo local podem expor conteúdo a quem acessa a conta. Minimizar dados, restringir acesso disponível no sistema e definir retenção. Uma expiração automática pode apagar trabalho ainda não exportado. Não eliminar diários pendentes por limite de recentes, idade ou simples salvamento de cópia. Suposição a validar, o formato consegue recuperar notas, geometria e exclusões sem nova dependência.
- **Dependências.** S-06 para custo e metas. Contrato mínimo de identidade compartilhado com F-NAV-05, sem exigir toda a função de mudança de caminho. Política de retenção precisa de decisão de produto. Recuperação não bloqueia tecnicamente a lista F-MARC-02, mas o risco de perda de dados merece precedência na release.
- **Tamanho e confiança.** G, confiança baixa. Incógnitas de durabilidade em dois sistemas, custo de gravação, identidade, geometria e retenção.
- **Prioridade sugerida.** Must para a fase de confiança, sujeito à validação. Valor 5, urgência 5, oportunidade 3, tamanho 4. WSJF = (5+5+3)/4 = 3,25. O risco de perda de dados prevalece sobre scores de funções menores.
- **Definição de pronto.** Comportamentos de recuperação e privacidade aprovados. `cargo test -p tsuro`, `cargo fmt --check` e `cargo clippy` executados, com falhas ou baseline registrados. Verificação real em macOS e Windows. Textos PT e EN, nota de release e custo comparado com S-06. Não confundir esta lista com testes já realizados.
- **Rascunho de issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(annot): recuperar marcações de sessões interrompidas`
  - Labels: `enhancement`, `annotation`, `needs-triage`
  - Corpo: Guardar checkpoints locais sem alterar o original. Recuperar de forma idempotente. Tratar corrupção, mudança externa e disco cheio sem descartar trabalho. Definir privacidade e retenção antes da implementação.

### F-MARC-02. Aba Marcações (Fusen)

- **Ficha.** completa
- **Domínio e tipo.** marcar, evolução do painel lateral.
- **Promessa.** O leitor encontra uma marcação pelo trecho ou pela nota e volta ao seu contexto.
- **Origem (evidência).** `relatorio.md § 5.1`, coleção navegável. `ui-ux.md § 5.2`, lista enxuta. `codigo-e-usuarios.md § 4.3`, reencontrar marcações.
- **Não copiar.** Não reproduzir sidebar do PDF Expert nem tabela corporativa do Bluebeam. Reusar painel e gramática visual do Tsuro.
- **Mission gate.** Passa. Torna revisar o que foi marcado mais claro sem ampliar a missão.
- **Situação atual no Tsuro.** O snapshot registra painel com miniaturas e sumário, além de marcações da sessão e Markdown. Âncoras não revalidadas são `crates/tsuro/src/session.rs:2279` e `crates/tsuro/src/view.rs:2489`. Importação de anotações externas não está confirmada.
- **História de usuário.** Como leitor de documento longo, quero buscar uma palavra nas minhas marcações para revisar o trecho sem percorrer todas as páginas.
- **Critérios de aceite.** Verificar os seguintes comportamentos.
  - Dado doze marcações da sessão, Quando a aba abre, Então lista doze itens com página, tipo, trecho e nota quando houver.
  - Dado uma palavra presente numa nota, Quando a busca é aplicada, Então mostra os itens correspondentes e informa quando não há resultado.
  - Dado um item selecionado, Quando Enter é pressionado, Então revela o trecho e registra a origem do salto no histórico existente.
  - Dado nenhuma marcação da sessão, Quando a aba abre, Então mostra estado vazio que não promete listar anotações externas não suportadas.
  - Dado remoção ou edição de uma marcação, Quando a lista é consultada, Então não mantém item removido nem trecho antigo.
  - Dado uma referência inválida após mudança de documento, Quando o leitor tenta saltar, Então não salta à página errada e informa que a referência está indisponível.
- **Experiência no Kiri.** Terceira aba no painel existente. Busca textual, setas, Enter e Escape. Foco visível e tipo textual não dependem de cor. Ordenar por página e posição, com truncamento que preserve a nota no detalhe. Filtros por cor só quando F-MARC-05 existir. Sem painel permanentemente aberto ou nova barra.
- **Expressão temática.** Fusen, 付箋, etiqueta adesiva, como codinome. Ori com dois retângulos deslocados e uma ponta dobrada. PT "Marcações deste documento" e "Nenhuma marcação nesta sessão." EN "Annotations in this document" e "No annotations in this session." Nenhum termo japonês substitui o rótulo Marcações.
- **Dados e código.** Propor `PanelTab` explícito e `AnnotationRow` com ID estável, origem, página e textos sem formatação Markdown. Estado em `session.rs`, desenho em `view.rs`. Lista e exportação compartilham dados sem reutilizar texto de apresentação como formato de arquivo. Sem mudança de motor na v1 de marcações da sessão.
- **Testes.** Criar, editar, remover, pesquisar nota e trecho, saltar e voltar. Lista vazia, multiline, teclado e documento com centenas de marcações. Comparar latência e memória com S-06. Não alegar virtualização ou desempenho sem medição.
- **Riscos e premissas.** Lista longa pode atrasar rolagem. Medir antes de escolher renderização. Misturar anotações do PDF com marcações da sessão pode criar expectativa falsa. Exibir origem e só oferecer importação depois de investigação. Suposição, os dados da sessão bastam para montar cada linha.
- **Dependências.** Paleta existente, histórico existente e S-06. F-NAV-02 melhora rótulos, mas o índice legível continua como fallback. F-MARC-05 habilita filtro opcional, não bloqueia lista textual. F-MARC-01 é precedência de risco, não dependência de código.
- **Tamanho e confiança.** M, confiança média. Incógnitas de foco, migração de abas, custo da lista e texto de origem.
- **Prioridade sugerida.** Must para a fase de revisão. Valor 4, urgência 3, oportunidade 2, tamanho 2. WSJF = (4+3+2)/2 = 4,5.
- **Definição de pronto.** Critérios atendidos com testes de comportamento e sessão real em macOS e Windows. `cargo test -p tsuro`, `cargo fmt --check`, `cargo clippy`, PT, EN e nota de release. Lista medida contra S-06.
- **Rascunho de issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(annot): listar e pesquisar marcações no painel`
  - Labels: `enhancement`, `annotation`, `ui`, `needs-triage`
  - Corpo: Acrescentar aba ao painel existente. Buscar por trecho e nota e saltar com histórico. Começar pelas marcações da sessão. Não bloquear lançamento por filtro de cor ou importação externa.

### F-MARC-03. Marcações na paleta (Fusen)

- **Ficha.** completa
- **Domínio e tipo.** marcar, evolução da paleta.
- **Promessa.** O leitor encontra e abre uma marcação usando somente teclado.
- **Origem (evidência).** `ui-ux.md § 5.3`, marcações na paleta. `codigo-e-usuarios.md § 2`, coleta sob demanda no Sumatra. `codigo-e-usuarios.md § 4.3`, revisão de marcações.
- **Não copiar.** Não copiar implementação de comandos, prefixos ou ranking do Sumatra. Integrar ao modelo de paleta do Tsuro.
- **Mission gate.** Passa. Reduz passos para reencontrar trechos marcados.
- **Situação atual no Tsuro.** O snapshot registra paleta com várias fontes. Âncoras não revalidadas são `crates/tsuro/src/session.rs:632`, `crates/tsuro/src/session.rs:873` e `crates/tsuro/src/view.rs:1860`.
- **História de usuário.** Como leitor que usa teclado, quero pesquisar minhas marcações na paleta para abrir o trecho sem alternar ao painel lateral.
- **Critérios de aceite.** Verificar os seguintes comportamentos.
  - Dado uma nota contendo a consulta, Quando o escopo Marcações é escolhido, Então lista a nota e sua página com o tipo identificado.
  - Dado um resultado ativo, Quando Enter é pressionado, Então fecha a paleta, revela a marcação e permite voltar à origem.
  - Dado uma consulta sem correspondência, Quando a busca termina, Então informa ausência de resultado sem mostrar resultado de outro escopo como se fosse marcação.
  - Dado alterações nas marcações entre duas aberturas, Quando a paleta reabre, Então usa uma versão atualizada da coleção.
  - Dado a versão da coleção sem alteração, Quando a consulta muda, Então filtra os dados existentes sem reconstruir a coleção inteira a cada tecla.
  - Dado um item removido depois da seleção, Quando a confirmação ocorre, Então não salta a uma referência obsoleta e informa a indisponibilidade.
- **Experiência no Kiri.** Escopo descobrível com nome funcional. Não exigir prefixo secreto. Reusar setas, Enter, Escape e foco da paleta. Ranking e limite de resultados são propostas a validar. Nenhum botão permanente novo.
- **Expressão temática.** Fusen, 付箋, com o ícone da lista, sem símbolo adicional. PT "Buscar nas marcações" e "Marcação indisponível." EN "Search annotations" e "Annotation unavailable."
- **Dados e código.** Propor item de paleta com ID estável, página e origem. Coleção versionada, invalidada por edição, remoção, recuperação e troca de documento. Coleta sob demanda e filtragem reaproveitada. Compartilhar dados de `AnnotationRow`, não componente visual entre estado e chrome.
- **Testes.** Consulta por trecho e nota, coleção vazia, resultados limitados, item removido, troca de aba e atualização da coleção. Medir a busca e instrumentar a reconstrução sem fazer desse detalhe o único teste de qualidade.
- **Riscos e premissas.** Cache sem invalidação exibe notas antigas. Associar coleção à versão e ao documento. Ranking agressivo pode esconder resultados do texto. Validar ordem com tarefas reais. Suposição, IDs da sessão permitem confirmar o item sem usar texto como chave.
- **Dependências.** Dados da lista F-MARC-02 ou seu pequeno modelo compartilhado. F-NAV-04 fornece registro de comandos, mas não depende desta função. F-NAV-01 melhora descoberta do histórico existente e não bloqueia o salto.
- **Tamanho e confiança.** M, confiança média. Incógnitas de escopo, ranking, limites e invalidação.
- **Prioridade sugerida.** Should. Valor 3, urgência 2, oportunidade 2, tamanho 2. WSJF = (3+2+2)/2 = 3,5.
- **Definição de pronto.** Consulta e confirmação seguras, PT e EN, verificação real por teclado em macOS e Windows. `cargo test -p tsuro`, `cargo fmt --check`, `cargo clippy`, nota de release e comparação com S-06.
- **Rascunho de issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(annot): pesquisar marcações pela paleta`
  - Labels: `enhancement`, `annotation`, `navigation`, `needs-triage`
  - Corpo: Adicionar escopo de marcações com dados versionados. Saltar por ID e preservar histórico. Tratar item removido e troca de documento. Não reconstruir toda a coleção por tecla.

### F-MARC-04. Marcas na barra de progresso (Shirube)

- **Ficha.** curta
- **Promessa.** O leitor percebe onde há marcações sem abrir uma lista.
- **Origem (evidência).** `codigo-e-usuarios.md § 4.3`, sugestão de minimapa. Evidência de demanda limitada, não comprovação de benefício.
- **Não copiar.** Não importar minimapa de editor ou adicionar segunda faixa de navegação permanente.
- **Mission gate.** Ajuste. Só passa se ajudar a localizar trechos sem ruído e sem prejudicar a navegação existente.
- **Situação atual no Tsuro.** O snapshot aponta `crates/tsuro/src/view.rs:1988` para progresso e `crates/tsuro/src/view.rs:202` para HUD, não revalidados. Ausência de marcadores precisa de confirmação real.
- **Hipótese e como validar.** Comparar protótipo com e sem indicadores em tarefas iguais, usando documentos com poucas e muitas marcações. Contar erros de clique e sucesso de localização. Não executar protótipo nesta rodada.
- **Critério de saída.** Só avançar se a localização melhorar na amostra, sem aumentar erros ou ocupar outra linha de chrome. Densidade alta agrupa indicadores. Toda navegação continua disponível por teclado e pela lista.
- **Dependências.** F-MARC-02 e S-06. Rótulos de F-NAV-02 são melhoria opcional, não bloqueio.
- **Tamanho e prioridade.** P, Could, confiança baixa. Incógnitas de densidade, escala e benefício.
- **Expressão temática.** Shirube, 標, baliza. Sem ícone novo. Traços na barra não dependem exclusivamente da cor e não viram alvos de 2 px. PT "Página com marcações". EN "Page with annotations." Dica e lista oferecem alternativa acessível.
- **Rascunho de issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(annot): avaliar indicadores de marcações no progresso`
  - Labels: `enhancement`, `annotation`, `ux`, `needs-triage`
  - Corpo: Comparar localização com e sem indicadores. Tratar agrupamento e alternativa por teclado. Adiar se aumentar ruído ou erros. Não criar outra barra.

## Tornar a marcação previsível

### F-MARC-05. Cor nomeada e memória da última escolha (Pigmentos)

- **Ficha.** completa
- **Domínio e tipo.** marcar, evolução das marcações existentes.
- **Promessa.** O leitor escolhe uma cor reconhecível e repete sua escolha sem configurar cada destaque.
- **Origem (evidência).** `codigo-e-usuarios.md § 3`, marcar sem lembrar a cor. `codigo-e-usuarios.md § 4.5`, memória e modos. `ui-ux.md § 5.4`, ação contextual. `docs/research/kiri-cor-cerebro.md` complementa acessibilidade e uso de cor.
- **Não copiar.** Não copiar paleta, seletor ou significados fixos do Zotero. Usar um conjunto pequeno, acessível e próprio.
- **Mission gate.** Passa. Reduz repetição ao marcar e mantém seleção explícita.
- **Situação atual no Tsuro.** O snapshot registra `Annotation` sem cor e uma barra contextual de ações. Âncoras não revalidadas são `crates/tsuro/src/session.rs:486`, `crates/tsuro/src/view.rs:1390` e `crates/tsuro/src/engine.rs:720`.
- **História de usuário.** Como leitor que diferencia trechos por cor, quero repetir minha última escolha para marcar sem reabrir configurações.
- **Critérios de aceite.** Verificar os seguintes comportamentos.
  - Dado uma escolha de cor, Quando uma nova marcação do tipo compatível é criada, Então usa a escolha e não altera as marcações anteriores.
  - Dado o seletor usado por teclado, Quando a cor muda, Então o nome funcional e o indicador de seleção permitem reconhecer a escolha sem distinguir matizes.
  - Dado a operação desfeita e refeita, Quando a marcação reaparece, Então preserva a cor original dessa operação.
  - Dado uma cópia anotada salva, Quando ela é aberta nos dois motores escolhidos para interoperabilidade, Então cor e conteúdo coincidem com a referência, dentro da tolerância documentada.
  - Dado uma preferência de cor desconhecida numa versão antiga, Quando o leitor abre, Então escolhe um padrão conhecido sem perder conteúdo nem falhar.
  - Dado falha ao salvar a cópia, Quando a gravação termina com erro, Então a seleção e as cores continuam na sessão e não informa sucesso.
- **Experiência no Kiri.** Seletor contextual compacto, nomes funcionais e foco visível. Poucas cores fechadas. Não impor significados como importante ou dúvida. Estados de seleção e erro ficam explícitos. Decidir se memória é por tipo, por sessão ou persistida antes de implementar.
- **Expressão temática.** Pigmentos é o nome interno. Yamabuki, Shu, Asagi e Wakakusa são candidatas a nomes de documentação, não traduções aprovadas. PT usa "Amarelo", "Vermelho", "Azul esverdeado" e "Verde". EN usa "Yellow", "Red", "Teal" e "Green". Sem ícone Ori novo nem codinome obrigatório na interface. Validar valores RGB, contraste e opacidade por tipo.
- **Dados e código.** Propor `MarkColor` fechado e campo de cor em marcações, com padrão para dados antigos. Preferência separada do valor de cada marcação. `session.rs` mantém estado, `view.rs` seletor, `engine.rs` transmite cores ao PDF salvo. Essa transmissão altera o caminho de gravação do motor, mas não sua arquitetura de threads ou cache. Não tocar `tsuro-sign`.
- **Testes.** Memória, desfazer, refazer, dados antigos, tema claro e escuro, interoperabilidade e falha de gravação. Não prometer controlar como um leitor externo renderiza cores. Se a comparação falhar, a release não passa, em vez de inventar fallback externo.
- **Riscos e premissas.** Cor perdida no PDF salvo pode apagar a intenção do leitor. Testar gravação real em dois motores. Cor exclusiva como sinal cria barreira de acesso. Dar nomes e indicação de seleção independentes de matiz. Seletor permanente aumenta chrome. Mantê-lo contextual. Suposição, suporte atual de Pdfium permite gravar sem dependência nova.
- **Dependências.** S-06 e validação de gravação existente. F-MARC-02 e F-MARC-08 recebem cor depois, sem bloquear suas v1. Antes de F-MARC-06. Aprovação de nova dependência se a investigação mostrar necessidade.
- **Tamanho e confiança.** G, confiança baixa. Incógnitas de persistência, contrastes, memória por tipo e compatibilidade externa.
- **Prioridade sugerida.** Should. Valor 4, urgência 3, oportunidade 2, tamanho 4. WSJF = (4+3+2)/4 = 2,25.
- **Definição de pronto.** Interoperabilidade aprovada com evidência, teclado e contraste verificados, PT e EN, nota de release. `cargo test -p tsuro`, `cargo fmt --check`, `cargo clippy` e aplicação real em macOS e Windows. Custo medido com S-06.
- **Rascunho de issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(annot): escolher cores e repetir a última escolha`
  - Labels: `enhancement`, `annotation`, `accessibility`, `needs-triage`
  - Corpo: Implementar pequeno conjunto nomeado. Guardar cor por marcação e separar preferência atual. Preservar desfazer e cor da cópia salva. Não impor significado semântico nem depender só do matiz.

### F-MARC-06. Marcação contínua (Pincelada)

- **Ficha.** curta
- **Promessa.** O leitor faz uma sequência de marcações com menos ações, sabendo quando o modo está ativo.
- **Origem (evidência).** `codigo-e-usuarios.md § 3`, dificuldades de modos. `codigo-e-usuarios.md § 4.5`, marcação contínua. `ui-ux.md § 5.4`, ferramentas contextuais.
- **Não copiar.** Não reproduzir ferramentas pegajosas ou modos ambíguos de suítes.
- **Mission gate.** Ajuste. Menos gestos só justifica o modo se não aumentar marcações acidentais.
- **Situação atual no Tsuro.** O snapshot registra ações contextuais em `crates/tsuro/src/view.rs:1390` e rótulo de modo em `crates/tsuro/src/view.rs:202`, não revalidados. Confirmar significado real antes de propor outro modo.
- **Hipótese e como validar.** Comparar tarefa sequencial com ações individuais e modo explícito. Medir ações, acidentes e retorno à cópia. Comparar saída por Escape e mudança de contexto com timeout. Timeout é hipótese, não decisão do usuário.
- **Critério de saída.** Avançar só com menos ações e sem aumento de acidentes na amostra. Teclado sai imediatamente. O leitor identifica o estado sem depender de cor. Documentar amostra e limitações. Sem evidência, adiar.
- **Dependências.** F-MARC-05, F-NAV-04 e S-06. Aprovar política de saída e preservação de seleção.
- **Tamanho e prioridade.** M, Won't nesta primeira sequência, confiança baixa. Adiamento é recomendação de planejamento, não decisão previamente confirmada do usuário.
- **Expressão temática.** Pincelada, nome interno em português. Indicador funcional explícito, sem ícone novo. PT "Marcação contínua ativa. Esc para sair." EN "Continuous annotation active. Esc to exit."
- **Rascunho de issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(annot): avaliar modo de marcação contínua`
  - Labels: `enhancement`, `annotation`, `ux`, `needs-triage`
  - Corpo: Testar ganho de ações e risco de marcação acidental. Comparar políticas de saída. Não comprometer release antes da validação. Aproveitar memória de cor sem obrigar o modo.

### F-MARC-07. Marcadores de página (Dobras)

- **Ficha.** completa
- **Domínio e tipo.** marcar, função nova candidata.
- **Promessa.** O leitor guarda páginas para reler sem precisar destacar texto.
- **Origem (evidência).** `codigo-e-usuarios.md § 2`, persistência de marcas no Sioyek. `relatorio.md § 4`, leitores orientados à leitura. O catálogo cita pedidos de marcadores como evidência complementar, sem validação nova nesta rodada.
- **Não copiar.** Não copiar banco de marcas nem editor estrutural de bookmarks. Não confundir marcadores pessoais com sumário do documento.
- **Mission gate.** Passa. Ajuda a marcar e reencontrar páginas para leitura.
- **Situação atual no Tsuro.** O snapshot registra sumário e posição de leitura, mas não confirma marcadores pessoais. Âncoras não revalidadas são `crates/tsuro/src/session.rs:2279` e `crates/tsuro/src/positions.rs:20`. O limite de 50 posições não é política aceitável para apagar marcadores.
- **História de usuário.** Como leitor que revisa páginas, quero criar um marcador pessoal para retornar sem alterar o sumário do PDF.
- **Critérios de aceite.** Verificar os seguintes comportamentos.
  - Dado uma página visível, Quando o leitor cria um marcador, Então um único marcador aparece na lista e o conteúdo do PDF original não muda.
  - Dado um marcador existente, Quando a ação é repetida conforme a política de alternância aprovada, Então não cria uma duplicata silenciosa.
  - Dado um marcador com nota curta, Quando o aplicativo reabre o mesmo arquivo, Então preserva página e nota.
  - Dado mais arquivos abertos que o limite de posições recentes, Quando o histórico é reduzido, Então os marcadores pessoais permanecem.
  - Dado arquivo alterado externamente, Quando há marcadores salvos, Então não reaplica posições em silêncio a conteúdo incompatível.
  - Dado falha ao persistir ou remover, Quando o sistema responde, Então informa a falha sem afirmar que a alteração foi guardada.
- **Experiência no Kiri.** Ação na paleta e seção identificada no painel de marcações. Setas e Enter localizam a página. Tipo Marcador distingue item de anotação textual. Vazio e falha têm texto. Informar que a v1 local não acompanha o PDF compartilhado.
- **Expressão temática.** Dobras, canto dobrado como codinome interno. Ori com folha de dois polígonos e triângulo de canto, mais vinco diagonal. PT "Marcar esta página" e "Este marcador fica neste dispositivo." EN "Bookmark this page" e "This bookmark stays on this device."
- **Dados e código.** Propor `PageBookmark` com ID, identidade do documento, `PageNo`, nome e nota opcional. Persistência própria, separada da retenção de recentes e do diário temporário. Compartilhar contrato de identidade, não o arquivo inteiro de posições. V1 fora do PDF é recomendação, sujeita à aprovação. Sem modificar `tsuro-sign` ou conteúdo original.
- **Testes.** Criar, remover, renomear, reabrir, mover arquivo e exceder limite de recentes. Confirmar que marcadores não somem por limpeza automática. Usar cópias de fixtures e verificar hash do original.
- **Riscos e premissas.** Armazenamento local pode ser confundido com portabilidade. Informar limite e não prometer sincronização. Identidade fraca pode apontar ao PDF errado. Validar conteúdo antes de reaplicar. Suposição, v1 local atende às tarefas prioritárias. A alternativa de anotação no PDF exige estudo próprio de interoperabilidade e assinaturas.
- **Dependências.** Modelo mínimo de identidade alinhado com F-MARC-01 e F-NAV-05. F-MARC-02 e F-NAV-04 para descoberta. F-NAV-02 melhora apresentação sem bloquear fallback numérico. Armazenar no PDF só após nova decisão e investigação F-ARQ-02.
- **Tamanho e confiança.** M, confiança média. Incógnitas de identidade, persistência e política de alternância.
- **Prioridade sugerida.** Should. Valor 3, urgência 2, oportunidade 2, tamanho 2. WSJF = (3+2+2)/2 = 3,5.
- **Definição de pronto.** Persistência sem perda por retenção, originais intactos, teclado, PT, EN e nota de release. `cargo test -p tsuro`, `cargo fmt --check`, `cargo clippy` e verificação real em macOS e Windows.
- **Rascunho de issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(annot): guardar marcadores pessoais de página`
  - Labels: `enhancement`, `annotation`, `navigation`, `needs-triage`
  - Corpo: Guardar marcadores locais independentes do histórico recente. Listar e saltar com teclado. Avisar sobre limite de portabilidade. Não modificar sumário, original ou assinaturas.

### F-MARC-08. Exportar marcações para arquivo (sem codinome)

- **Ficha.** curta
- **Promessa.** O leitor guarda suas marcações num arquivo Markdown escolhido por ele.
- **Origem (evidência).** `relatorio.md § 5.4`, evolução da exportação existente. `codigo-e-usuarios.md § 4.3`, resumos e origem. `codigo-e-usuarios.md § 4.7`, rótulos de página.
- **Não copiar.** Não trazer biblioteca de conhecimento, resumos generativos ou links que exponham caminhos locais.
- **Mission gate.** Passa. Completa a revisão do que foi marcado usando dados que o Tsuro já produz.
- **Situação atual no Tsuro.** O snapshot registra Markdown e cópia à área de transferência. Âncora não revalidada é `crates/tsuro/src/session.rs:411`. Confirmar fluxo atual antes de criar issue.
- **Hipótese e como validar.** Propor v1 que exporta toda a coleção, reaproveitando o gerador. Confirmar notas multiline, acentos, caracteres Markdown, página física e nome de arquivo sem caminho absoluto. Cancelar não escreve. Sobrescrever exige confirmação. Falha de disco não informa sucesso.
- **Critério de saída.** Arquivo exportado reproduz trechos e notas com origem legível ao abrir num editor externo. Original permanece intacto. Cancelamento, sobrescrita e falha de gravação passam em teste. Não limpar diário de recuperação só por exportar.
- **Dependências.** Gerador existente e F-NAV-04 para descoberta. F-NAV-02 acrescenta rótulos quando disponível. F-MARC-02 permite exportar subconjunto depois. F-MARC-05 acrescenta cor depois. Nenhum desses incrementos bloqueia v1 de coleção inteira e índice físico.
- **Tamanho e prioridade.** P, Should, confiança média. Incógnitas de escaping, caminhos e política de sobrescrita. Prioridade maior que a sugerida pelo especialista porque reutiliza uma capacidade existente sem exigir novo modelo de leitura.
- **Expressão temática.** Sem codinome e sem ícone novo. PT "Exportar marcações" e "O arquivo já existe. Substituir?" EN "Export annotations" e "The file already exists. Replace it?"
- **Rascunho de issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(annot): exportar marcações para arquivo Markdown`
  - Labels: `enhancement`, `annotation`, `needs-triage`
  - Corpo: Reusar exportação textual existente. Escolher destino, confirmar sobrescrita e tratar cancelamento e erro. Preservar notas e origem. Adiar subconjuntos e metadados opcionais sem bloquear a v1.

## Decisões pendentes e limites da revisão

O usuário aprova escopo de release, retenção do diário, memória de cor e armazenamento dos marcadores. A revisão recomenda v1 local de Dobras, mas não presume aprovação.

O coordenador corrigiu quatro riscos da proposta do especialista. Um diário não some ao salvar uma cópia. Uma sequência corrompida não é reconstruída por leitura permissiva. Dobras não usa a retenção de 50 arquivos recentes. Uma falha em leitor externo não pode ser resolvida por um fallback que o Tsuro não controla.

Lista textual, exportação inteira e rótulo físico são incrementos independentes. Cor, labels do PDF e subconjuntos são melhorias posteriores. Isso evita dependências artificiais e permite entregas pequenas.

Não houve execução do aplicativo, benchmarks, testes entre motores ou aprovação de traduções japonesas. Os critérios são requisitos futuros, não evidência de resultados atuais.
