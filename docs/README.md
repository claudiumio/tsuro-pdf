# Documentos do Tsuro PDF

Comece por [`../AGENTS.md`](../AGENTS.md) e depois pelo [roadmap](./roadmap/roadmap.md). Esta pasta guarda especificação, pesquisa e o registro de como as decisões foram tomadas.

## O que está vivo

| Caminho | O que é |
|---|---|
| [`design/01-fundacao-visual.md`](./design/01-fundacao-visual.md) | Spec da fundação visual e fonte dos tokens de `kiri.rs`. Fases 1 a 3 fechadas, fase 4 em andamento. |
| [`roadmap/roadmap.md`](./roadmap/roadmap.md) | Guia de execução das 28 funções, com fases, gates e ordem relativa. |
| [`roadmap/funcoes/`](./roadmap/funcoes/) | As três fichas de requisitos. Prevalecem sobre o roadmap em requisitos e critérios. |
| [`roadmap/issues-publicadas.md`](./roadmap/issues-publicadas.md) | Mapa de cada ficha para a issue correspondente, #128 a #155. |
| [`roadmap/investigacao-f-marc-01.md`](./roadmap/investigacao-f-marc-01.md) | Desenho do diário de recuperação de marcações, implementado em `recovery.rs`. |
| [`roadmap/_insumos/`](./roadmap/_insumos/) | Briefing, snapshot técnico, glossário, modelo das fichas, catálogo e protocolo de retomada. São insumos, não aprovações. |
| [`research/`](./research/) | Pesquisa que sustentou as decisões de leitura, anotação, cor e concorrência. |
| [`ui/`](./ui/) | Spec do Kiri, os cinco estudos de layout (A a E) e os ícones Ori em SVG. |
| [`agents/`](./agents/) | Convenções do issue tracker e das labels de triagem. |
| `hero.png`, `social-preview.png` | Imagens usadas pelo README e pela página do projeto. |

## O que é registro

| Caminho | O que é |
|---|---|
| [`roadmap/historico/`](./roadmap/historico/) | Trilha do planejamento de outubro de 2026. Avaliação técnica, registro de retomada, verificação documental, publicação das issues e as ferramentas que geraram o manifesto. |
| [`superpowers/specs/2026-09-10-print-dialog-design.md`](./superpowers/specs/2026-09-10-print-dialog-design.md) | Desenho do diálogo de impressão, implementado em `session.rs` e `print.rs`. |

Os arquivos de `roadmap/historico/` explicam de onde vieram os requisitos. Nada ali manda no trabalho atual, e o roadmap aponta para eles quando cita a evidência.

Os dois scripts em `roadmap/historico/` resolvem o repositório a partir do próprio arquivo, então continuam executáveis de qualquer clone.
