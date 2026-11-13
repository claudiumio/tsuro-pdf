# Roadmap do Tsuro PDF

Proposta revisada em 13 de outubro de 2026. O coordenador revisou o roadmap produzido por Step 5 Preview e corrigiu autoridade, dependências e condições de início. Senbazuru é apenas um nome interno proposto para este conjunto de melhorias.

Este documento ordena trabalho futuro. Em 13 de outubro de 2026, o usuário autorizou criar issues bilíngues no GitHub para o trabalho restante e excluiu aumento de DPI de impressão. Essa autorização não inclui implementação, commit, alteração de princípio ou release. Aplica práticas de Product Owner e gerenciamento adaptadas ao PMBOK, sem afirmar certificação. Não há datas, capacidade de equipe, velocidade ou orçamento presumidos.

## Retomar pelo próximo resultado verificável

1. Leia [AGENTS.md](../../AGENTS.md) e o [protocolo de retomada](./_insumos/05-protocolo-retomada.md).
2. Consulte as [fichas de marcações](./funcoes/01-marcacoes.md), as [fichas de navegação](./funcoes/02-leitura-navegacao.md) e as [fichas de arquivos e ambiente](./funcoes/03-arquivos-impressao-ambiente.md).
3. Antes de implementar uma unidade, revalide seu estado atual, suas dependências e suas âncoras. Leia as definições reais conforme o protocolo de descoberta do repositório.
4. Faça somente as investigações necessárias à unidade escolhida. Um spike pode concluir seguir, limitar, adiar ou recusar.
5. Registre decisões de produto e autorizações específicas antes das mudanças que dependem delas.
6. Implemente uma unidade pequena e verifique seu comportamento. Não deixe a verificação só para o fim da fase.

As fichas revisadas prevalecem sobre este roadmap em requisitos e critérios. O protocolo 05 prevalece sobre o briefing anterior. O [snapshot técnico](./_insumos/01-estado-do-produto-2026-10-13.md) é histórico. A [avaliação técnica](./historico/avaliacao-tecnica.html) confrontou os contratos principais com o código no commit `f6540e9`. O backlog GitHub foi consultado novamente para a [publicação bilíngue](./issues-publicadas.md). Não inferir ausência de uma função de uma busca antiga.

A primeira sequência recomendada é revalidar e medir, definir o contrato mínimo de recuperação e tratar o risco de perder marcações. A exportação simples é um incremento independente que pode sair antes, se o mantenedor decidir aproveitar seu menor escopo. Não exige que o diário inteiro esteja pronto.

## Usar fases sem criar bloqueios artificiais

Fase agrupa resultados. Pacote da EAP identifica trabalho. Gate é condição para comprometer ou entregar uma unidade. A ordem relativa não obriga um PR gigante, uma release por fase ou a conclusão de todas as funções de uma fase antes de outra.

Um bloqueio técnico impede a implementação dependente. Não impede a investigação que pretende resolvê-lo. Uma preferência de sequência não vira dependência. Uma melhoria opcional pode chegar depois da v1.

Manter no máximo um subagente ativo é o limite operacional desta entrega, após o relato de travamento do Mac de 8 GB. Para implementação futura, a proposta inicial também é uma unidade pesada ativa por vez, com builds e validações serializados quando necessário. Isso reduz concorrência, mas não garante um teto de memória. Não retomar planos anteriores de sete escritores simultâneos ou dez frentes paralelas.

WSJF é julgamento herdado das fichas. Segurança, risco de perda de dados, acessibilidade e dependências prevalecem sobre score. Must é relativo ao pacote proposto, não compromisso de calendário. Investigações de acessibilidade começam cedo. Teclado e foco são gates em todas as fases.

## Referência de escopo dos 28 IDs

Cada ID aparece uma vez nesta tabela, com sua fase inicial responsável. Quando a fase recebe investigação e não implementação, isso está explícito. P, M, G e GG são tamanhos relativos, não dias. Confiança não é probabilidade de sucesso.

| ID | Fase inicial | Incremento | Tamanho e confiança | Prioridade da ficha |
| --- | --- | --- | --- | --- |
| F-MARC-01 | F1 | Orime. Checkpoint e restauração idempotente das marcações | G, baixa | Must, WSJF 3,25 |
| F-MARC-02 | F1 | Fusen. Lista e busca de marcações da sessão | M, média | Must, WSJF 4,5 |
| F-MARC-03 | F2 | Fusen. Marcações na paleta com coleção versionada | M, média | Should, WSJF 3,5 |
| F-MARC-04 | F5 | Shirube. Investigar indicadores na barra de progresso | P, baixa | Could |
| F-MARC-05 | F5 | Pigmentos. Cores nomeadas e memória da escolha | G, baixa | Should, WSJF 2,25 |
| F-MARC-06 | ADIADO | Pincelada. Investigar marcação contínua | M, baixa | Won't nesta sequência, recomendação |
| F-MARC-07 | F2 | Dobras. Marcadores pessoais com persistência própria | M, média | Should, WSJF 3,5 |
| F-MARC-08 | F1 | Exportar coleção completa de marcações para arquivo | P, média | Should |
| F-NAV-01 | F2 | Kaerimichi. Descoberta do retorno por histórico existente | M, média | Should, WSJF 3,5 |
| F-NAV-02 | F2 | Fólio. PageLabels distintos da posição física | M, média | Should, WSJF 5 |
| F-NAV-03 | F5 | Tsunagi. Cópia de parágrafo opt-in e conservadora | G, baixa | Should condicionado, WSJF 2,33 |
| F-NAV-04 | F2 | Descoberta de comandos, atalhos e condições coerentes | M, média | Should, WSJF 4 |
| F-NAV-05 | ADIADO | Shiori. Retomar conteúdo movido sem misturar instâncias | G, baixa | Could, WSJF 2,33 |
| F-NAV-06 | F6 | Diagnosticar direção, precisão e resposta da rolagem | P ou M, baixa | Could |
| F-NAV-07 | ADIADO | Nozoki. Prévia local de destinos internos declarados | G, baixa | Could |
| F-NAV-08 | ADIADO | Kari-mokuji. Sumário provisório, sem substituir autoral | G, baixa | Won't nesta sequência, recomendação |
| F-NAV-09 | F0 | Investigar seleção por teclado. Implementação ampla adiada | GG, baixa | Won't na implementação desta sequência, recomendação |
| F-ARQ-01 | F3 | Kagi. Abertura com senha transitória e saídas seguras | M, baixa | Must condicionado, WSJF 5,5 |
| F-ARQ-02 | ADIADO | Hanko. Avaliar preservação de assinaturas ao salvar | GG, baixa | Should condicionado, WSJF 2,2 |
| F-ARQ-03 | F4 | Sumi. Limitar memória e preservar fidelidade da impressão, sem aumentar DPI | G, baixa, reestimar após recorte | Should condicionado, WSJF anterior 2,25 |
| F-ARQ-04 | F4 | Incluir ou omitir marcas da sessão na prévia e saída | M, média na v1 | Should, WSJF 4,5 |
| F-ARQ-05 | F4 | Folha de marcações com origem, posição e notas opt-in | M, baixa | Could |
| F-ARQ-06 | F3 | Explicar permissões por ação, inclusive desconhecidas | M, baixa | Should |
| F-ARQ-07 | F3 | Selo. Separar integridade, revisão e confiança | P ou G, conforme escopo | Should para clareza |
| F-AMB-01 | ADIADO | Washi. Registrar interesse em papel oriental futuro | Épico, hipóteses não validadas | Adiado pelo usuário |
| F-AMB-02 | F6 | Névoa. Reduzir chrome além do HUD com saída segura | M, média | Could, WSJF 3,5 |
| F-AMB-03 | F6 | Ito. Investigar guia de linha opcional | G, baixa | Could |
| F-AMB-04 | F0 | Pacto do Ma. Propor limites verificáveis de produto | Pacto, sem estimativa aprovada | Governança proposta, sem score |

Contagem da tabela. F0 recebe 2 IDs, F1 recebe 3, F2 recebe 5, F3 recebe 3, F4 recebe 3, F5 recebe 3 e F6 recebe 3. O backlog recebe 6. Total de 28. F-NAV-09 entra em F0 para investigação, não como promessa de implementação de seleção completa.

## Entregar por fases relativas

### F0. Revalidar a realidade e fixar gates

- Objetivo. Conhecer estado real, corpus, linha de base e decisões necessárias à próxima unidade.
- Entrada. Repositório, regras atuais, fichas revisadas e snapshot tratado como rascunho técnico.
- Entregas. RV-01, RV-02 e RV-03 por escopo. S-06 com hardware, versão, corpus e método. Contrato mínimo de identidade e proposta de política de recuperação. F-AMB-04 para decisão de governança. INV-A11Y-01 e INV-FOCUS-01 para conhecer barreiras.
- Saída. A unidade seguinte tem fatos revalidados, orçamento aplicável e gates identificados. A linha de base registra amostra e dispersão. Não é necessário terminar a revalidação dos 28 IDs para iniciar uma unidade independente.
- Papel responsável. Mantenedor técnico conduz. QA confere evidências. UX e acessibilidade avaliam barreiras. Usuário decide escopo, metas e políticas que exigem aprovação.
- Parar ou limitar. Se o método não produzir comparação confiável, ajustar corpus e declarar limite. Não declarar regressão aprovada com medição inválida.
- Release parcial. Documentação e instrumentos podem formar unidade independente. Isso não autoriza publicação nesta entrega. S-01 e S-05 são executados quando seus consumidores forem escolhidos, não bloqueiam toda F1.

### F1. Não perder e reencontrar marcações

- Objetivo. Reduzir risco de perder trabalho e revisar o que já está marcado.
- Entrada. Estado e custo dos caminhos de anotação revalidados. Contrato de identidade, política de checkpoint e privacidade aprovados para o diário. Exportação e lista usam apenas seus gates aplicáveis.
- Entregas. F-MARC-01, F-MARC-02 e F-MARC-08 como unidades separadas.
- Saída. Recuperação restaura checkpoint validado sem duplicação, preserva original e informa persistência indisponível. Lista navega com histórico. Exportação trata cancelar, sobrescrever e disco cheio sem falso sucesso. Salvar cópia não limpa automaticamente o diário.
- Papel responsável. Implementação Rust entrega. Mantenedor revisa persistência. UX e acessibilidade conferem lista e diálogo. QA simula falhas e uso real.
- Parar ou limitar. Divergência de identidade, replay inconsistente, vazamento de conteúdo ou perda de trabalho bloqueia entrega do diário. Arquivo protegido não recebe recuperação em texto claro por omissão de política.
- Release parcial. Lista e exportação não dependem tecnicamente do diário. Recuperação merece precedência de risco, sobretudo sobre conforto, mas não vira bloqueio artificial de exportação simples.

### F2. Identificar páginas, navegar e descobrir ações

- Objetivo. Diminuir confusão de página, salto e comando.
- Entrada. Infraestrutura relevante revalidada. Dados mínimos de marcação disponíveis para a paleta. Contrato mínimo de identidade disponível para marcadores pessoais.
- Entregas. F-NAV-04, F-NAV-01, F-NAV-02, F-MARC-03 e F-MARC-07. Registro cedo é preferência, não bloqueio da migração total.
- Saída. Comandos, atalhos e dispatch coerentes. Retorno usa histórico existente. Labels repetidos ou numéricos ambíguos pedem escolha. Marcadores não somem com evicção de recentes. Instâncias diferentes não misturam dados pessoais.
- Papel responsável. Implementação Rust, revisão do mantenedor, UX de desambiguação e QA por plataforma.
- Parar ou limitar. Retorno ao lugar errado, mistura de instâncias ou consumo de tecla de edição bloqueia unidade afetada. Extração de label que excede orçamento exige ajuste ou fallback físico.
- Release parcial. As cinco unidades podem sair separadamente. F-MARC-03 pode usar modelo mínimo antes do painel completo. Consumidores com posição física não precisam esperar labels.

### F3. Abrir e explicar arquivos

- Objetivo. Abrir arquivos protegidos legítimos e comunicar capacidade e confiança sem promessas falsas.
- Entrada. S-05 favorável para senha e permissões. Para F-ARQ-07 textual, basta revalidar dados e textos do painel existente. Política de saídas protegidas decidida antes de implementação que a afete.
- Entregas. F-ARQ-01, F-ARQ-06 e F-ARQ-07.
- Saída. Senha não persistida, cancelamento preserva sessão anterior, capacidades desconhecidas declaradas e estado de assinatura corresponde à verificação executada. Saída menos protegida não ocorre silenciosamente.
- Papel responsável. Implementação Rust, mantenedor de segurança, QA de armazenamento e revisão humana da comunicação. Usuário autoriza alterações que exigem decisão específica.
- Parar ou limitar. Mecanismo não suportado, segredo persistido ou política inconclusiva bloqueia caminho afetado. Não criar crypto ou remover restrições.
- Release parcial. Clareza do painel pode sair antes de senha e antes do salvamento incremental. Mudança em `crates/tsuro-sign/` só se necessária e autorizada, não presumida por uma mudança de texto.

### F4. Imprimir com resultado previsível

- Objetivo. Controlar origem das marcas na saída e avaliar qualidade sem sacrificar leveza.
- Entrada. Preparação, prévia e spool revalidados. S-06 aplicável. S-03 necessário somente para trocar estratégia de saída.
- Entregas. F-ARQ-04 limitada à sessão primeiro, S-03 e F-ARQ-03 condicionada, F-ARQ-05 depois de investigação de layout.
- Saída. Prévia e saída seguem a mesma política. Anotações padrão são distintas de conteúdo achatado. Estado do spool não é comprovante de impressão física. Corpus de qualidade e custo sustenta qualquer mudança do raster atual. Folha de marcações tem origem e posições identificáveis.
- Papel responsável. Implementação Rust prepara saída. QA compara arquivo preparado, prévia e impressão física disponível. Mantenedor revisa memória e driver.
- Parar ou limitar. Divergência de prévia ou perda de conteúdo bloqueia unidade. S-03 desfavorável mantém caminho atual, não bloqueia omitir marcas da sessão.
- Release parcial. F-ARQ-04 pode sair sozinha. Folha não espera vetor, labels, cores ou painel de marcações completo se o modelo semântico existente bastar.

### F5. Refinar marcação e cópia

- Objetivo. Escolher cores reconhecíveis e copiar trechos sem corrupção silenciosa.
- Entrada. Gravação de cores e custos revalidados. S-07 favorável só para cópia de parágrafo. Dados da lista e hipótese de indicadores disponíveis para progresso.
- Entregas. F-MARC-05, F-NAV-03 condicionada e investigação de F-MARC-04 antes de implementar indicadores.
- Saída. Cor é nomeada e armazenada por anotação, distinta da preferência atual. Cópia de parágrafo preserva estrutura e hífen ambíguos, mantendo cópia original. Indicadores têm alvo acessível e demonstram ganho antes de entrar.
- Papel responsável. Implementação Rust, UX e acessibilidade de cores, QA do corpus e interoperabilidade.
- Parar ou limitar. Perda de cor na gravação ou junção que muda significado bloqueia unidade afetada. Não prometer controlar a aparência em leitores externos.
- Release parcial. Cor não espera S-07. Indicador não espera cor se usa agrupamento e tipos existentes. Marcação contínua permanece adiada por recomendação, não por decisão atribuída ao usuário.

### F6. Avaliar conforto opcional

- Objetivo. Demonstrar ganho de foco, guia ou rolagem sem mudar o papel ou prender a leitura.
- Entrada. Custo de S-06 e realidade do HUD conhecidos. S-04 favorável para régua. Diagnóstico de input antes de modificar rolagem.
- Entregas. F-AMB-02, investigação de F-AMB-03 e F-NAV-06. Cada uma é independente.
- Saída. Foco melhora além do auto-hide, preserva âncora e avisos. Guia funciona nos layouts declarados, desligado por padrão e ausente de saídas. Rolagem só muda por problema reproduzível ou ganho observado, respeitando preferência do sistema.
- Papel responsável. Implementação Rust, UX, acessibilidade e QA de teclado, geometria e dispositivo.
- Parar ou limitar. Sem ganho além do HUD, recusar modo extra. Sem geometria confiável, adiar régua. Sem problema ou ganho de rolagem, guardar diagnóstico e não alterar input.
- Release parcial. Nenhuma dessas funções exige outra ou Washi. Evidência de barreira real pode promover investigação e correção antes de F6.

### Reabrir o backlog somente com evidência

| ID | Motivo | Gate para reconsiderar |
| --- | --- | --- |
| F-MARC-06 | Risco de acidentes e benefício não validado | F-MARC-05, descoberta de comando, política de saída e preservação de seleção validadas |
| F-NAV-05 | Identidade e custo inconclusivos | INV-ID-01 favorável, política de instâncias e fallback aprovada |
| F-NAV-07 | Geometria e custo de prévia desconhecidos | S-02 favorável, foco correto e orçamento aprovado |
| F-NAV-08 | Inferência pode orientar para trecho errado | S-04 e INV-OUTLINE-01 favoráveis, limite de erros fixado antes da avaliação |
| F-ARQ-02 | Salvamento seguro ainda não demonstrado | S-01 favorável e contrato por caso. Autorização se exigir mudança em assinatura |
| F-AMB-01 | Papel oriental deixado para mais adiante pelo usuário | Autorização para explorar. S-08 informa decisão posterior, não aprova princípio automaticamente |

A implementação ampla de F-NAV-09 também fica adiada na proposta, embora sua investigação esteja em F0. Se forem observadas barreiras reais, registrar impacto e recomendar promoção imediata. Tamanho GG não basta para justificar adiamento longo de uma barreira de acesso.

## Referência da EAP

EAP é a estrutura analítica do projeto. Os números identificam pacotes, não calendário ou sequência técnica obrigatória. Cada unidade de implementação incorpora verificação dos critérios da ficha e definição de pronto. As revisões de fase consolidam evidências, não substituem essas verificações.

| Pacote | Resultado | Tipo | Referência de aceite | Papel que entrega |
| --- | --- | --- | --- | --- |
| 1.0.1 | Revalidação por unidade, RV-01 | Investigação | Situação atual das fichas | Mantenedor |
| 1.0.2 | Repositório e backlog atual, RV-02 | Investigação | Snapshot e regras atuais | Mantenedor |
| 1.0.3 | Inventário de fixtures e lacunas, RV-03 | Investigação | Testes das fichas | QA |
| 1.0.4 | Linha de base S-06 | Investigação e verificação | Corpus e orçamento aprovados | Mantenedor e QA |
| 1.0.5 | Contrato mínimo de identidade e privacidade | Investigação e decisão | F-MARC-01 e F-MARC-07 | Mantenedor, usuário decide |
| 1.0.6 | Pacto proposto e barreiras de acesso | Documentação e investigação | F-AMB-04 e F-NAV-09 | UX, acessibilidade e mantenedor |
| 1.1.1 | Recuperação segura | Implementação e verificação | F-MARC-01 | Implementação Rust |
| 1.1.2 | Lista de marcações | Implementação e verificação | F-MARC-02 | Implementação Rust |
| 1.1.3 | Exportação simples | Implementação e verificação | F-MARC-08 | Implementação Rust |
| 1.2.1 | Registro e retorno | Duas unidades independentes | F-NAV-04 e F-NAV-01 | Implementação Rust |
| 1.2.2 | Labels e desambiguação | Implementação e verificação | F-NAV-02 | Implementação Rust |
| 1.2.3 | Paleta de marcações e marcadores | Duas unidades independentes | F-MARC-03 e F-MARC-07 | Implementação Rust |
| 1.3.1 | Suporte de proteção S-05 | Investigação | F-ARQ-01 e F-ARQ-06 | Mantenedor |
| 1.3.2 | Abertura e permissões | Duas unidades independentes | F-ARQ-01 e F-ARQ-06 | Implementação Rust |
| 1.3.3 | Comunicação de assinatura | Investigação, texto e verificação | F-ARQ-07 | Mantenedor e UX |
| 1.4.1 | Política de impressão | Implementação e verificação | F-ARQ-04 | Implementação Rust |
| 1.4.2 | Qualidade S-03 e caminho de saída | Investigação, decisão e implementação condicionada | F-ARQ-03 | Mantenedor e QA |
| 1.4.3 | Folha de marcações | Investigação e implementação condicionada | F-ARQ-05 | Implementação Rust e UX |
| 1.5.1 | Cor por anotação | Implementação e verificação | F-MARC-05 | Implementação Rust |
| 1.5.2 | S-07 e parágrafo | Investigação e implementação condicionada | F-NAV-03 | Mantenedor e QA |
| 1.5.3 | Indicadores de progresso | Investigação e implementação condicionada | F-MARC-04 | UX e implementação Rust |
| 1.6.1 | Foco além do HUD | Investigação e implementação condicionada | F-AMB-02 | UX e implementação Rust |
| 1.6.2 | S-04 e régua | Investigação e implementação condicionada | F-AMB-03 | Mantenedor e acessibilidade |
| 1.6.3 | Input por dispositivo | Investigação e correção condicionada | F-NAV-06 | QA e implementação Rust |
| 1.7.1 | Revisão de candidatos adiados | Decisão e investigação quando autorizada | Tabela de backlog | Usuário e mantenedor |
| 1.8.1 | Gates de release por unidade | Verificação | Definição de pronto e pacto aprovado | QA e mantenedor |

## Referência dos spikes

S-01 a S-08 são propostas anteriores, não experimentos executados. Para cada spike, registrar pergunta, corpus, método, critérios definidos antes, resultado e veredito. Prazo e limite de esforço serão definidos quando houver capacidade conhecida.

| ID | Pergunta e entrada | Método mínimo futuro | Resultado e decisão | Responsável e consumidores |
| --- | --- | --- | --- | --- |
| S-01 | Existe caminho de salvar marcações que preserve o contrato das assinaturas? Corpus de revisões e permissões | Conferir bindings e escritor, preservar original, comparar bytes, revisões e validadores independentes autorizados | Seguir apenas para casos comprovados. Limitar ou adiar se inconclusivo. Não confundir bytes intactos com alteração permitida | Mantenedor e QA. F-ARQ-02 |
| S-02 | Links internos declarados têm geometria e recorte confiáveis? Fixture de links, não só outline | Validar destino e geometria, limitar uma prévia, medir memória e latência | Seguir se destino, foco e orçamento passarem. Limitar layouts ou adiar. Não executar rede, URI ou JavaScript | Mantenedor e QA. F-NAV-07 |
| S-03 | É possível limitar memória e cancelamento da preparação sem perder fidelidade? Corpus de texto, fontes, imagens e páginas mistas. Aumento de DPI excluído | Avaliar processamento limitado, raster nos 200 DPI atuais e preservação vetorial somente se viável. Comparar saída, escala, orientação, memória agregada, cancelamento e driver disponível | Seguir se fidelidade e orçamento do trabalho inteiro passarem. Resultado negativo mantém caminho atual. Não elevar DPI nem acrescentar seletor de resolução | Mantenedor e QA. F-ARQ-03 |
| S-04 | Linhas e ordem são confiáveis em colunas, tabelas e rotação? Corpus com referência humana | Comparar geometria e ordem, declarar layouts suportados | Seguir nos layouts validados, limitar outros. Falha de ordem não autoriza inferência silenciosa | Mantenedor e acessibilidade. F-AMB-03, F-NAV-08, apoio a F-NAV-03 e F-NAV-09 |
| S-05 | Quais senhas, erros e permissões o motor suporta? Corpus legítimo de PDFs protegidos | Conferir API instalada, abrir e cancelar, comparar capacidade reportada por ação e avaliar saídas | Seguir onde suporte for confirmado. Limitar mecanismo e declarar desconhecido. Política insegura bloqueia compromisso | Mantenedor e QA. F-ARQ-01 e F-ARQ-06 |
| S-06 | Qual é o custo de primeira página, memória por aba, busca e CPU ociosa? Corpus e hardware declarados | Separar abertura fria e quente, repetir e registrar dispersão, versão e condições | Produzir baseline comparável e propor orçamento ao usuário. Se método falhar, ajustar e declarar limite. Não inventar percentual aceitável | Mantenedor e QA. Unidades que afetam custo |
| S-07 | Junção de linhas e hífen pode ser conservadora em PT e EN? Corpus de prosa, colunas e tabelas | Fixar limite de erro antes de avaliar, comparar referência humana, preservar texto incerto | Seguir só se critério for cumprido. Limitar ou adiar em ambiguidades. Alteração de significado bloqueia regra | Mantenedor e QA. F-NAV-03 |
| S-08 | Papel oriental próprio tem benefício de leitura e preferência? Corpus definido quando autorizado | Não executar agora. Futuro mockup avalia preferência e legibilidade, protótipo nativo mede custo e fidelidade | Autorização para explorar precede experimento. Resultado informa nova decisão de princípio, não a substitui. Negativo mantém branco | UX e mantenedor. F-AMB-01 |

Investigações locais adicionais são propostas novas, não novos compromissos ou spikes já executados.

| ID | Pergunta | Saída esperada e condição de avanço |
| --- | --- | --- |
| RV-01 | O incremento e suas âncoras ainda correspondem ao código? | Nota por unidade com fatos conferidos e lacunas, antes de implementar |
| RV-02 | O backlog e as regras atuais conflitam com o snapshot? | Estado atual registrado, sem fechar issue ou PR automaticamente |
| RV-03 | Quais fixtures realmente cobrem os casos? | Inventário e plano de novos inputs. Fixtures existentes somente leitura |
| INV-ID-01 | Como confirmar conteúdo sem prejudicar abertura ou misturar cópias? | Contrato mínimo seguro primeiro. Movimento entre caminhos só após custo e ambiguidade validados |
| INV-SCROLL-01 | Há problema de direção, precisão, inércia ou resposta? | Reprodução por plataforma e dispositivo. Sem defeito ou ganho, não mudar input |
| INV-FOCUS-01 | Caret e seleção podem seguir ordem e manter foco? | Limites e protótipo futuro, distinguindo seleção de leitura de tela |
| INV-A11Y-01 | Quais barreiras impedem ler, marcar ou imprimir? | Impacto e proposta de promoção de prioridade. Gates de teclado seguem imediatos |
| INV-OUTLINE-01 | Títulos inferidos orientam corretamente PDFs sem outline? | Limite fixado antes, comparação humana e veredito. Outline autoral preservado |

O contrato mínimo de identidade para diário e marcadores não exige implementar toda F-NAV-05 ou finalizar sua política de arquivos movidos. A investigação precisa separar instância, conteúdo e confiança de correspondência desde o começo.

## Referência de dependências

B é bloqueio técnico para implementação dependente. G é gate de risco ou decisão. P é preferência de sequência. O é melhoria opcional. Infraestrutura atual precisa de revalidação em todas as linhas. S-06 não exige medir dados irrelevantes para uma mudança puramente textual.

| ID | B e G | P e O |
| --- | --- | --- |
| F-MARC-01 | B contrato mínimo de identidade. G checkpoint, retenção, privacidade e custo | P antes de conforto por risco. Não espera F-NAV-05 completo ou salvar incremental |
| F-MARC-02 | B dados de marcação, navegação e painel existentes. G custo e foco | P lista antes da paleta. O labels e filtro de cor. Diário não é bloqueio técnico |
| F-MARC-03 | B dados da lista ou modelo mínimo versionado. G ranking, limite e custo | O registro unificado e retorno descobrível. Histórico existente basta para salto |
| F-MARC-04 | B dados e agrupamento de F-MARC-02. G ganho e alvo acessível | O labels. Cor não é requisito obrigatório |
| F-MARC-05 | B gravação de cor revalidada. G nomes, conjunto, interoperabilidade e custo | P antes de modo contínuo. Lista e exportação recebem cor depois |
| F-MARC-06 | B cores F-MARC-05 e descoberta de comando F-NAV-04. G saída e seleção | P adiada nesta sequência. Não promover por score sem validar acidentes |
| F-MARC-07 | B contrato mínimo de identidade e persistência própria. G política local e esquema | O lista, paleta e labels. Não depende de retenção de recentes ou journal |
| F-MARC-08 | B gerador semântico existente. G sobrescrita, caminhos e proteção de saída | O labels, subconjunto da lista, cores e registro. V1 inteira e física é independente |
| F-NAV-01 | B histórico e eventos revalidados. G contexto inválido, foco e custo | O registro de comandos. Não criar histórico paralelo |
| F-NAV-02 | B extração de labels confirmada. G ambiguidade, sanitização e custo | O consumidores. Nenhum consumidor com fallback físico espera labels |
| F-NAV-03 | B S-07 favorável e dados suficientes. G política opt-in e cópia segura | O S-04 se faltar geometria e F-NAV-04 para descoberta. Não mudar exportação |
| F-NAV-04 | B paleta, atalhos e dispatch revalidados. G foco e teste não tautológico | P cedo. Consumidores podem acrescentar item antes de migração total |
| F-NAV-05 | B identidade validada. G custo e política de instâncias | P depois do contrato mínimo. Não unificar posições ou diários de cópias idênticas |
| F-NAV-06 | B diagnóstico real. G defeito ou ganho observado | O S-06. Não bloqueia outras funções |
| F-NAV-07 | B S-02 favorável. G foco e orçamento | O retorno transitório e registro. Apenas destinos internos declarados |
| F-NAV-08 | B S-04 e inferência favoráveis. G limite prévio e origem provisória | O registro. Adiada, sem substituir estrutura autoral |
| F-NAV-09 | B foco, ordem e acessibilidade investigados. G limites e promoção por barreira | O S-04 conforme geometria. Não bloqueia teclado da lista, paleta ou diálogo |
| F-ARQ-01 | B S-05 favorável. G proteção de saída, privacidade e custo | O painel de permissões. Aviso mínimo e política não esperam painel |
| F-ARQ-02 | B S-01 favorável. G contrato por caso e autorizações aplicáveis | O linguagem de F-ARQ-07. Diário e marcadores locais independentes |
| F-ARQ-03 | B S-03 favorável. G qualidade e orçamento | O política de impressão coordenada. F-ARQ-04 não espera vetor |
| F-ARQ-04 | B preparação e prévia revalidadas. G limites por origem | O caminho futuro de F-ARQ-03. V1 pode omitir só sessão |
| F-ARQ-05 | B dados semânticos e layout impresso. G notas opt-in e privacidade | O lista, cor, labels e vetor. Posição física basta na v1 |
| F-ARQ-06 | B S-05. G estados desconhecidos e despacho coerente | P compartilhar modelo de proteção. Não precisa concluir abertura para explicar dados disponíveis |
| F-ARQ-07 | B painel e resultados revalidados. G texto fiel e autorização se tocar assinatura | Independente de salvamento incremental. Verificação nova é outro escopo |
| F-AMB-01 | G autorização para explorar, S-08 e decisão posterior de princípio | P futuro. Nenhuma função depende de Washi |
| F-AMB-02 | B ganho além do HUD demonstrado. G foco, saída e custo | O registro. Não espera guia ou papel |
| F-AMB-03 | B S-04 favorável. G ganho, contraste, teclado e custo | O foco, registro e seleção. Sem ciclo com F-NAV-09 |
| F-AMB-04 | G cláusulas aprovadas antes de virarem regra | P governança cedo. Triagem consulta evidências, não audita tudo |

As dependências técnicas nomeadas entre funções não formam ciclo. Contratos compartilhados são pacotes próprios, não exigência de concluir mutuamente diário, marcador e retomada. Os consumidores de labels, cores e registro continuam com v1 menor quando a ficha permitir.

## Referência de riscos

Probabilidade e impacto são avaliações qualitativas propostas, não frequências medidas. Revalidar quando houver evidência real. A falta de baseline é fato do planejamento atual, não prova de probabilidade de regressão.

| ID | Causa, evento e efeito | Probabilidade e impacto | Resposta e gatilho | Papel e evidência |
| --- | --- | --- | --- | --- |
| R-01 | Encerramento sem checkpoint perde trabalho da sessão | Média, alto | Recuperação antes de conforto. Gatilho é falha ou relato de perda | Mantenedor e QA. Replay, truncamento e disco cheio |
| R-02 | Correspondência incorreta restaura marcas no documento errado | Média, alto | Validar conteúdo e instância, bloquear dúvida. Gatilho é mudança externa | Mantenedor. Corpus de mudança e cópias conflitantes, não só hash |
| R-03 | Sem baseline, custo de função nova passa despercebido | Média, alto | Medir S-06. Gatilho é mudança de extração, render ou persistência | Mantenedor e QA. Hardware, método e dispersão |
| R-04 | Append é interpretado como assinatura globalmente válida | Média, alto | Separar bytes, revisão, permissão e confiança. Gatilho é save assinado | Mantenedor e QA. Contrato por caso e validadores |
| R-05 | Senha ou conteúdo protegido vai para log, diário ou saída clara | Média, alto | Excluir segredo e aprovar política. Gatilho é abertura protegida | Mantenedor e QA. Inspeção com segredo e texto de teste |
| R-06 | Cache ou política distinta faz saída divergir da prévia | Média, médio | Compartilhar política e invalidar prévia. Gatilho é opção de marcas | QA. Comparação de prévia e arquivo preparado |
| R-07 | Lista ou prévia sem limite aumenta memória e latência | Média, médio | Medir e limitar coleta. Gatilho é corpus denso ou grande | Mantenedor. Latência e memória comparáveis |
| R-08 | Cor ou alvo minúsculo vira único meio de descobrir informação | Média, alto | Nome, seleção independente de matiz e alternativa por teclado | UX e acessibilidade. Contraste, foco e uso real |
| R-09 | Tema implementado sem decisão muda princípio da folha branca | Baixa, alto | Não iniciar Washi agora. Gatilho é proposta de recoloração | Usuário decide. Registro de mudança e evidência |
| R-10 | Funções fora da missão transformam leitor em suíte | Média, alto | Mission gate e pacto aprovado. Gatilho é candidato novo | Usuário e mantenedor. Escopo e alternativas |
| R-11 | Snapshot obsoleto induz mudança no lugar errado | Média, médio | Revalidar por unidade. Gatilho é início de implementação | Mantenedor. Definições lidas e incremento confirmado |
| R-12 | Junção de texto ou sumário inventa ordem e significado | Média, alto | Critério prévio e fallback conservador. Gatilho é layout ambíguo | QA e mantenedor. Referência humana PT e EN |
| R-13 | Muitos processos simultâneos pressionam o Mac de 8 GB | Alta com fan-out, alto | Um delegado ativo, sem filhos. Serializar trabalhos pesados futuros | Coordenador. Estado de tarefas, sem alegar teto de RAM ou órfãos resolvidos |
| R-14 | Adiamento por tamanho mantém barreira real de acesso | Média, alto | Investigar em F0 e propor promoção. Gatilho é impedimento observado | Acessibilidade e usuário. Impacto e cenários reproduzíveis |

## Referência de decisões e controle de mudança

Vigente é regra do repositório ou acordo global. Explícita é uma instrução real do usuário. Recomendação é proposta a avaliar. Pendente exige decisão no momento em que a unidade a afeta. Não atribuir escolhas antigas do briefing ao usuário.

| ID | Decisão ou política | Estado e autoridade |
| --- | --- | --- |
| D-01 | Folha branca nos temas e documento primeiro | Vigente no [Kiri](../ui/kiri-spec.md) |
| D-02 | Ler, anotar, marcar e imprimir, nada além disso | Vigente em AGENTS.md |
| D-03 | `PageNo` interno em base 0 | Vigente em AGENTS.md. Apresentação física em base 1 é convenção legível das fichas |
| D-04 | Código novo nas crates nativas, sem estender legado | Vigente em AGENTS.md |
| D-05 | Aprovar dependência, esquema, CI, assinatura ou deploy quando aplicável | Regras vigentes do repo e acordos globais. Autorizações são específicas |
| D-06 | Explorar papel oriental mais adiante | Explícita do usuário. Não autoriza renderização agora |
| D-07 | Evitar vários subagentes simultâneos após travamento | Feedback explícito. Protocolo operacional mantém um ativo |
| D-08 | Patrocinador decide produto, mantenedor decide viabilidade técnica | Governança proposta. Papéis não representam equipe contratada |
| D-09 | Recuperação antes de conforto e unidades independentes menores | Recomendação deste roadmap |
| D-10 | Marcadores locais separados de recentes e diário | Recomendação da ficha. Schema e retenção precisam de decisão aplicável |
| D-11 | Cópia original padrão e parágrafo opt-in | Recomendação da ficha, não aprovação do usuário |
| D-12 | Marcação contínua adiada depois de cor e validação de acidentes | Recomendação da ficha, não decisão do usuário |
| D-13 | Intervalo de checkpoint, retenção, privacidade e descarte | Pendente antes de compromisso de recuperação. Não assumir a meta antiga de cinco segundos |
| D-14 | Saídas e recuperação de PDF protegido | Pendente. V1 proposta não cria texto claro por omissão de política |
| D-15 | Posições independentes para cópias e fallback ambíguo | Proposta conservadora, pendente antes de retomada entre caminhos |
| D-16 | Conjunto de cores, nomes e significado | Pendente. Nome acessível não obriga semântica universal |
| D-17 | Inclusão de marcas e padrão de impressão | Pendente no incremento concreto. Limites por origem já descritos |
| D-18 | Foco sem persistência oculta na primeira versão | Recomendação, com interação a validar |
| D-19 | Promover função se investigação mostrar barreira de acesso | Recomendação de governança. Usuário aprova repriorização |
| D-20 | Orçamentos, corpus e critérios de erro e qualidade | Pendentes antes de aceitar resultados dos respectivos experimentos |
| D-21 | Não aumentar DPI da impressão nem criar seletor de resolução | Explícita do usuário em 13 de outubro de 2026. F-ARQ-03 e S-03 limitados a preparação, memória, cancelamento e fidelidade sem elevar resolução |
| D-22 | Publicar issues do restante do planejamento em inglês e português | Explícita do usuário em 13 de outubro de 2026. Itens adiados continuam backlog futuro e gates técnicos permanecem. Não autoriza implementação ou release |

Para mudar escopo, política ou princípio, registrar solicitante, problema, alternativas, impacto em dados, custo, acessibilidade e testes. Anexar evidências e identificar a autorização necessária. Atualizar decisão, ficha, dependências e roadmap juntos. Resultado favorável de spike não substitui aprovação de produto.

O usuário é autoridade final para a proposta, mas isso não exige uma pergunta para cada detalhe técnico. Observar fatos, escolher defaults reversíveis coerentes e apresentar recomendação. Pedir aprovação apenas das decisões reais de produto e das ações protegidas pelas regras vigentes.

## Verificar cada unidade antes de considerar pronta

Consultar AGENTS.md atualizado antes de executar comandos. A definição de pronto comum exige critérios da ficha, testes de comportamento, execução real e evidência dos erros. `cargo test -p tsuro`, `cargo fmt --check` e `cargo clippy` precisam de resultados registrados. Falha conhecida deve ter causa e decisão documentadas, não ser chamada de aprovação.

Verificar macOS e Windows quando aplicável. Declarar plataforma ou driver não exercitado e o impacto dessa lacuna. Não declarar cobertura completa se só uma plataforma foi testada. Textos em pt-BR e EN, nota de release e limites publicados fazem parte da entrega. Comparar custo com S-06 quando afetado.

Teclado e foco valem em todas as fases. Ações novas são alcançáveis e operáveis, foco é visível e não desaparece durante uso. Edição local não dispara comando global indevido. Estados vazio, carregando, erro e sucesso têm comunicação compreensível. Cor não é único sinal. Seleção por caret e tecnologia assistiva são verificações diferentes.

Impressão tem três evidências distintas, preparação, aceitação pelo spool e resultado físico. Assinatura também tem estados distintos, bytes, revisão, alteração permitida e confiança. Não fundir estados em uma conclusão de sucesso.

Uma release parcial só é candidata se a unidade for segura e sua cobertura estiver clara. O mantenedor recomenda prontidão e o usuário decide escopo de publicação. Este planejamento não concede autorização de merge, deploy ou publicação.

## Consultar os insumos e os limites da verificação atual

- [Modelo das fichas](./_insumos/03-modelo-de-funcao.md) e [catálogo de candidatas](./_insumos/04-catalogo-de-candidatas.md) preservam tipos e IDs.
- [Briefing anterior](./_insumos/00-briefing-po-pm.md) e [glossário temático](./_insumos/02-glossario-tematico.md) são insumos, não aprovações.
- [Pesquisa geral](../research/engenharia-reversa-pdf/relatorio.md), [UI e UX](../research/engenharia-reversa-pdf/ui-ux.md) e [código e pedidos de usuários](../research/engenharia-reversa-pdf/codigo-e-usuarios.md) fundamentam inspiração sem cópia.
- [Registro de retomada](./historico/retomada-todo.md) identifica delegações, revisões e verificação documental.
- [Relatório de verificação documental](./historico/verificacao-documental.md) registra apenas o que o coordenador conferiu nesta entrega.

As 28 fichas e este roadmap são documentos revisados. A avaliação técnica posterior leu os caminhos principais de abertura, recuperação, histórico, exportação, impressão e assinaturas. Nenhuma função foi implementada nesta retomada. Não há benchmark, spike executado, validação entre motores, impressão física ou suíte do aplicativo executada. O verificador antigo `historico/verificar_entrega.py` não foi executado. As checagens documentais e a leitura estática não provam comportamento do produto.

As [28 issues bilíngues](./issues-publicadas.md), #128 a #155, foram publicadas e verificadas no GitHub em 13 de outubro de 2026, conforme autorização do usuário. Títulos, corpos, labels e estado aberto foram conferidos contra as especificações. #44 e #127 foram consultadas e não são equivalentes às candidatas. Nenhuma issue preexistente foi alterada, fechada ou reaberta. Nenhum PR ou commit foi criado. Implementação futura depende de pedido explícito, começando pela unidade e gates aplicáveis, não por todas as 28 funções de uma vez.
