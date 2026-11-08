# Atualização do roadmap e publicação das issues

## Playbook de planejamento adaptado

1. When the change is one or two files with an obvious approach, skip the plan. Say so and stop.
   - Não se aplica. Atualização de planejamento e publicação de múltiplas issues.
2. Settle open questions by prototype before you write. Run `playbooks/prototype.md` for each. Keep the branch, the SHA, and the screenshots for Appendix A. Ask the operator only about a product or preference call that no run can settle. Give options (the **never-block-on-the-human** principle skill).
   - skip: não implementar. A decisão de excluir aumento de DPI já foi tomada pelo usuário. Incertezas técnicas permanecem gates nas issues.
3. Explore in subagents with `subagent_type: "pstack:poteto-agent"` and an explicit model per the Subagents section (the **guard-the-context-window** principle skill). Each returns file pointers, conventions, test commands, and entry points. No inlined dumps.
   - skip: sem subagentes. Reutilizar avaliação e pesquisar backlog atual em série.
4. Copy the skeleton below into the plan file and fill every placeholder. Unless the operator names a path, write the file under the agent store (`~/.claude/orchestrate/<slug>/docs/`). Keep every heading and every sub-block in the order shown. One section per PR. One PR is one change with its own evidence (the **sequence-verifiable-units** principle skill). Name the execution playbook in **How to read this**. Pick between `playbooks/autopilot-full.md` and `playbooks/autopilot-stack.md` per the rule at the end of `playbooks/autopilot-stack.md`). A standing program takes `playbooks/orchestrate.md`. The execution playbook owns base selection, topology changes, and merge authority. Do not copy its rebase steps into the plan.
   - Adaptação. Manter catálogo e roadmap existentes. Manifesto local com uma unidade por issue, idioma PT e EN, estado e link confirmado. Sem plano de PRs ou execução autônoma.
5. Write under `/technical-writing` in full, then `/unslop`. The body is one Diátaxis mode, how-to. Appendices hold explanation and reference. Each heading states the task or the finding. No long dashes. No mid-sentence colons.
   - Skills já carregadas nesta sessão. Aplicar às issues e decisões.
6. Run `node skills/poteto-mode/scripts/check-plan.mjs <plan.md>` from the installed plugin and fix every line it prints (the **encode-lessons-in-structure** principle skill).
   - skip: esquema de plano por PR não se aplica. Conferir IDs, dois idiomas, exclusão de aumento de DPI, labels existentes e correspondência com issues remotas.
7. Hand back. Post the plan path and the script's output, then stop. Execution starts on the operator's explicit go, under the execution playbook the plan names.
   - Entregar links publicados, itens reaproveitados e limites. Não implementar.

throughput checkpoint: serial publication, one GitHub write at a time; shared issue IDs are recorded locally before proceeding.

## Checklist

- [x] Ler AGENTS.md, identificar repo e conferir HEAD.
- [x] Consultar issues abertas e fechadas e labels atuais. Duas abertas e 41 fechadas antes da publicação.
- [x] Remover aumento de DPI das fichas, roadmap e parecer visual atuais. Raster mantém 200 DPI, sem seletor de resolução.
- [x] Preparar issues bilíngues com escopo, aceite, gates e referências públicas existentes. 28 IDs únicos.
- [x] Reutilizar issue equivalente aberta em vez de criar duplicata. Nenhuma equivalente encontrada. #44 e #127 preservadas.
- [x] Publicar em série e registrar cada número e URL retornados. #128 a #155, um retorno registrado antes da criação seguinte.
- [x] Conferir conteúdo remoto, idiomas e cobertura. 28 títulos, corpos completos, labels e estados abertos correspondem ao manifesto.
- [x] Entregar índice local e links em `issues-publicadas.md`.

## Verificação final

`rtk proxy python3 -I docs/roadmap/_insumos/publicar_issues.py` validou as 28 especificações antes de publicar. `--verify` confirmou as 28 issues remotas. Nenhum subagente, instalação, build, suíte Rust, benchmark ou spike foi executado. Nenhum código do aplicativo ou CI foi alterado. Sem commit, PR, push ou release. HTML conferido estruturalmente, sem nova verificação visual completa em celular.

## Autorização e limites

Pedido de 13 de outubro de 2026 autoriza publicar issues no GitHub para o restante do trabalho. Aumento de DPI não será feito. Não autoriza implementação, commit, PR, novas dependências, CI ou deploy. Itens futuros continuam backlog, não promessa de implementação imediata. Nenhuma issue existente será fechada ou reaberta automaticamente.
