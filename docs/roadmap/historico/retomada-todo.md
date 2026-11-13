# Retomada das funções e do roadmap

## Playbook de planejamento

1. When the change is one or two files with an obvious approach, skip the plan. Say so and stop.
   - Não se aplica. Entrega solicitada é catálogo de funções e roadmap, sem implementação.
2. Settle open questions by prototype before you write. Run `playbooks/prototype.md` for each. Keep the branch, the SHA, and the screenshots for Appendix A. Ask the operator only about a product or preference call that no run can settle. Give options (the **never-block-on-the-human** principle skill).
   - skip: não implementar nem prototipar nesta retomada. Incertezas tornam-se spikes propostos, sem alegar evidência de execução.
3. Explore in subagents with `subagent_type: "pstack:poteto-agent"` and an explicit model per the Subagents section (the **guard-the-context-window** principle skill). Each returns file pointers, conventions, test commands, and entry points. No inlined dumps.
   - Adaptação ao pedido explícito. T3 Step 5 Preview com effort máximo, especialistas somente leitura, um ativo por vez. Sem aninhamento.
4. Copy the skeleton below into the plan file and fill every placeholder. Unless the operator names a path, write the file under the agent store (`~/.claude/orchestrate/<slug>/docs/`). Keep every heading and every sub-block in the order shown. One section per PR. One PR is one change with its own evidence (the **sequence-verifiable-units** principle skill). Name the execution playbook in **How to read this**. Pick between `playbooks/autopilot-full.md` and `playbooks/autopilot-stack.md` per the rule at the end of `playbooks/autopilot-stack.md`. A standing program takes `playbooks/orchestrate.md`. The execution playbook owns base selection, topology changes, and merge authority. Do not copy its rebase steps into the plan.
   - Adaptação. Usar `docs/roadmap/`, o catálogo já preparado e a governança do briefing. Não criar promessa de PRs nem programa autônomo. Execução e merge exigem pedido futuro.
5. Write under `/technical-writing` in full, then `/unslop`. The body is one Diátaxis mode, how-to. Appendices hold explanation and reference. Each heading states the task or the finding. No long dashes. No mid-sentence colons.
   - Skills carregadas. Fichas são referência de requisitos propostos. Roadmap é guia de execução futura.
6. Run `node skills/poteto-mode/scripts/check-plan.mjs <plan.md>` from the installed plugin and fix every line it prints (the **encode-lessons-in-structure** principle skill).
   - skip: o esquema de plano autônomo por PR, com dez lanes simultâneas, não é esta entrega. Conferir o catálogo, os campos e as dependências sem fan-out.
7. Hand back. Post the plan path and the script's output, then stop. Execution starts on the operator's explicit go, under the execution playbook the plan names.
   - Pendente. Entregar os caminhos e o resultado fiel da verificação documental. Não iniciar implementação.

## Throughput checkpoint

- Passo bloqueante. Conferir arquivos sobreviventes, modelo disponível e protocolo de segurança antes de delegar.
- Frentes independentes. Marcações, navegação e arquivos podem ser analisadas separadamente, mas serão executadas em série para limitar memória.
- Estado compartilhado. Especialistas somente leitura. Apenas o coordenador escreve as entregas revisadas.
- Menor decomposição segura. Três especialistas de funções e um de roadmap. No máximo um ativo. Sem builds, navegador ou worktrees.

## Checklist de entrega

- [x] Conferir insumos e disponibilidade de Step 5 Preview no catálogo T3.
- [x] Registrar o limite de um subagente e as correções de autoridade no protocolo 05.
- [x] Delegar e revisar as oito funções de marcações.
- [x] Gravar `funcoes/01-marcacoes.md`.
- [x] Delegar e revisar as nove funções de navegação.
- [x] Gravar `funcoes/02-leitura-navegacao.md`.
- [x] Delegar e revisar as sete funções de arquivo e quatro de ambiente.
- [x] Gravar `funcoes/03-arquivos-impressao-ambiente.md`.
- [x] Delegar, revisar e gravar `roadmap.md`.
- [x] Verificar IDs, campos, critérios, referências locais, dependências e cobertura de fases.
- [x] Entregar resultados. Não implementar, commitar ou publicar.

## Registro das tarefas

### Etapa 1. Marcações

- Estado. Concluída e revisada. `task_status` retornou `completed`, `result_available` e `hasPendingChildRuns=false`.
- Verificação documental. PASS. Oito IDs, tipos corretos, 19 campos por completa e 12 por curta, seis critérios por completa. Não validou código, comportamento ou desempenho.
- Modelo. `claudeAgent`, `ocx-claude2-nous--stepfun~sstep-5-preview:free`, `effort=max`.
- taskId. `node:delegated-task:command%3Amcp%3A211e713c-6e56-4c81-9c9f-bdeb6cfead38%3Adelegate-task%3Atsuro-roadmap-retomada-marcacoes-20261013-r1`.
- childThreadId. `thread:delegated-task:command%3Amcp%3A211e713c-6e56-4c81-9c9f-bdeb6cfead38%3Adelegate-task%3Atsuro-roadmap-retomada-marcacoes-20261013-r1`.
- Escopo. Oito fichas, F-MARC-01 até F-MARC-08. Somente leitura, sem trabalhos filhos. Resultado revisado em `funcoes/01-marcacoes.md`.
- Correções. Não apagar diário ao salvar cópia. Não recuperar sequência corrompida por leitura permissiva. Persistência de marcadores separada de retenção de recentes. Não prometer fallback em leitor externo que o Tsuro não controla.

### Etapa 2. Leitura e navegação

- Estado. Concluída e revisada. `task_status` retornou `completed`, `result_available` e `hasPendingChildRuns=false`.
- Verificação documental. PASS. Nove IDs, tipos corretos, 19 campos por completa e 12 por curta, seis critérios por completa. Não validou código, comportamento ou desempenho.
- Modelo. `claudeAgent`, `ocx-claude2-nous--stepfun~sstep-5-preview:free`, `effort=max`.
- taskId. `node:delegated-task:command%3Amcp%3A211e713c-6e56-4c81-9c9f-bdeb6cfead38%3Adelegate-task%3Atsuro-roadmap-retomada-navegacao-20261013-r1`.
- childThreadId. `thread:delegated-task:command%3Amcp%3A211e713c-6e56-4c81-9c9f-bdeb6cfead38%3Adelegate-task%3Atsuro-roadmap-retomada-navegacao-20261013-r1`.
- Escopo. Nove fichas, F-NAV-01 até F-NAV-09. Somente leitura, sem trabalhos filhos. Resultado revisado em `funcoes/02-leitura-navegacao.md`.
- Correções. Geometria na cópia de parágrafos, identidade por instância separada de conteúdo, labels de metadados separados de numeração visual e dependências opcionais que não bloqueiam v1. Cobertura da paleta não deriva expectativa só do próprio registro.

### Etapa 3. Arquivos, impressão e ambiente

- Estado. Concluída e revisada. `task_status` retornou `completed`, `result_available` e `hasPendingChildRuns=false`.
- Verificação documental. PASS. Onze IDs e tipos corretos. Cinco completas com seis critérios cada, quatro curtas, um épico e um pacto com 12 cláusulas. Estrutura não valida comportamento do aplicativo.
- Modelo. `claudeAgent`, `ocx-claude2-nous--stepfun~sstep-5-preview:free`, `effort=max`.
- taskId. `node:delegated-task:command%3Amcp%3A211e713c-6e56-4c81-9c9f-bdeb6cfead38%3Adelegate-task%3Atsuro-roadmap-retomada-arquivos-20261013-r1`.
- childThreadId. `thread:delegated-task:command%3Amcp%3A211e713c-6e56-4c81-9c9f-bdeb6cfead38%3Adelegate-task%3Atsuro-roadmap-retomada-arquivos-20261013-r1`.
- Escopo. Sete fichas F-ARQ e quatro fichas F-AMB. Somente leitura, sem trabalhos filhos. Resultado revisado em `funcoes/03-arquivos-impressao-ambiente.md`.
- Correções. Incremental não garante assinatura. Separar marcação de sessão, anotação padrão e conteúdo achatado. Spool não prova impressão física. Senha e saídas de PDF protegido exigem política explícita. Triagem de pacto não substitui auditoria.

### Etapa 4. Roadmap consolidado

- Estado. Concluída e revisada. `task_status` retornou `completed`, `result_available` e `hasPendingChildRuns=false`.
- Verificação documental. PASS. 28 IDs únicos, 15 completas, 11 curtas, um épico, um pacto, 90 critérios, 15 contas WSJF, 34 referências a seções existentes e 16 links relativos válidos. Cobertura e matriz com 28 linhas. Script reproduzível do relatório executado com saída 0. Não valida aplicativo.
- Modelo. `claudeAgent`, `ocx-claude2-nous--stepfun~sstep-5-preview:free`, `effort=max`.
- taskId. `node:delegated-task:command%3Amcp%3A211e713c-6e56-4c81-9c9f-bdeb6cfead38%3Adelegate-task%3Atsuro-roadmap-retomada-consolidacao-20261013-r1`.
- childThreadId. `thread:delegated-task:command%3Amcp%3A211e713c-6e56-4c81-9c9f-bdeb6cfead38%3Adelegate-task%3Atsuro-roadmap-retomada-consolidacao-20261013-r1`.
- Escopo. Consolidar os 28 IDs, fases, EAP, spikes, riscos, decisões e critérios de saída. Somente leitura, sem trabalhos filhos. Resultado revisado em `roadmap.md`.
- Correções. Adiamento da marcação contínua não é decisão do usuário. Spikes de assinatura e senha não bloqueiam toda a preparação. Autorização para explorar Washi precede experimento, mudança de princípio é decisão posterior. Revalidação ocorre por unidade, não exige rever todas as fichas antes de um incremento seguro.

## Fechamento documental

- Relatório. `verificacao-documental.md` contém resultado, limites e script reproduzível.
- Uma primeira checagem agregada falhou por uso incorreto de `Counter` sobre dicionário de tipos. A checagem corrigida passou. Nenhum ID faltava.
- Revisão documental separou arestas técnicas, decisões, preferências e opções. Não houve prova formal de dependências do aplicativo.
- `rtk git diff --name-only` não listou alteração de arquivo rastreado. As entregas locais permanecem sem commit.
- Não foram executados testes do aplicativo, benchmarks, protótipos ou o antigo `verificar_entrega.py`. Não houve publicação, PR, alteração de princípio ou implementação.
- Todas as quatro tarefas delegadas terminaram sem trabalhos filhos pendentes no status consultado. Não há tarefa delegada deste planejamento a aguardar.
