# Protocolo de retomada do planejamento

Este protocolo prevalece sobre o briefing 00 onde houver conflito. A entrega é um planejamento proposto. Não autoriza implementar funções, abrir issues, publicar conteúdo, commitar ou alterar regras do produto.

## Limite operacional

O usuário relatou que o fan-out anterior travou seu Mac de 8 GB. O coordenador mantém no máximo um subagente ativo por vez. Cada tarefa usa Step 5 Preview com `effort=max`. Não usar workflows, subagentes aninhados, navegador, instalações, builds, servidores, indexação ou worktrees neste planejamento.

Os especialistas são somente leitura. Devolvem o documento completo na resposta. O coordenador revisa e grava a entrega com Write. Nenhum especialista escreve arquivos, executa o verificador ou altera o checkout. O verificador existente pode ser executado pelo coordenador, se necessário, em leitura e sem imports locais de dados não confiáveis.

As quatro etapas são sequenciais.

1. Especificar F-MARC-01 até F-MARC-08 em `funcoes/01-marcacoes.md`.
2. Especificar F-NAV-01 até F-NAV-09 em `funcoes/02-leitura-navegacao.md`.
3. Especificar F-ARQ-01 até F-ARQ-07 e F-AMB-01 até F-AMB-04 em `funcoes/03-arquivos-impressao-ambiente.md`.
4. Consolidar todas as fichas em `roadmap.md`, com fases, EAP, dependências, riscos, decisões e critérios de saída.

Só iniciar a tarefa seguinte depois que a anterior terminar, sem trabalhos filhos pendentes, e sua entrega for revisada. Se o provedor falhar, registrar o erro. Não trocar o modelo sem autorização nem duplicar uma tarefa ainda ativa.

## Autoridade e fatos

O usuário é patrocinador e autoridade final de produto. O coordenador prepara briefings com práticas de Product Owner e gerenciamento adaptado ao PMBOK. Não afirma possuir certificação nem apresenta os documentos como conformidade certificada.

O briefing 00, o glossário 02 e o catálogo 04 são rascunhos de uma tentativa anterior, não decisões automaticamente aprovadas pelo usuário. Seus nomes, prioridades, metas numéricas e escolhas de interação são propostas.

A decisão explícita do usuário é explorar papel oriental mais adiante. Washi continua adiado. A folha branca continua sendo a regra vigente do Kiri. O usuário não autorizou modificar a renderização agora.

A prioridade relativa da marcação contínua, as cores e seus significados, a identidade dos arquivos e a política de cópia de parágrafos permanecem propostas. Não chamar essas escolhas de decisões do usuário.

Os fatos sobre código e GitHub no insumo 01 são um snapshot de 13 de outubro de 2026. Não foram revalidados nesta retomada. Referenciar esse insumo e marcar a implementação e os números de linha como não revalidados. Não afirmar que uma função não existe só porque uma busca antiga não encontrou a função. A pesquisa também é documental, não benchmark.

## Qualidade da entrega

Manter os 28 IDs e os tipos de ficha do catálogo. Usar os campos do modelo 03, com texto conciso e critérios testáveis. Não tratar funções candidatas como compromissos de release.

Cada ficha separa a capacidade existente do incremento proposto. Cada nome temático fica em documentação até revisão linguística. A interface usa rótulos funcionais claros em pt-BR e EN. Um ícone novo usa a gramática Ori, não ativos de terceiros.

A recuperação de marcações precisa tratar privacidade, mudança externa do arquivo e recuperação sem duplicar operações. Anotações já incorporadas ao PDF não se confundem com marcações criadas na sessão.

Rótulos de página são metadados do PDF, não números descobertos por OCR. Os índices internos continuam 0-based. Duplicidade e ambiguidade de rótulos exigem interação explícita.

Uma atualização incremental não garante preservação de assinatura. Distinguir integridade dos bytes assinados, alterações posteriores permitidas e confiança no certificado. O spike pode concluir que o recurso deve ser adiado ou limitado. Qualquer mudança em `crates/tsuro-sign/` precisa de aprovação específica.

Imprimir sem marcações precisa distinguir as marcações da sessão, as anotações padrão do arquivo e as marcas já achatadas no conteúdo. Não prometer apagar conteúdo achatado nem alterar o original.

MoSCoW e WSJF são julgamentos propostos. Explicitar parcelas, divisor e incertezas. Dependências e riscos de perda de dados prevalecem sobre a pontuação. Não inventar prazos, capacidade, benchmarks, métricas de usuários ou números de issues.

## Saída do planejamento

O roadmap atribui cada ID a uma fase ou ao backlog adiado. Define responsável por papel, entrada e saída de cada fase, spikes com pergunta e resultado esperado, matriz de dependências, registro de riscos e decisões. Explica como um mantenedor retoma sem esta conversa.

O roadmap não inicia execução. Nenhum cenário de teste, protótipo, métrica ou desempenho é declarado aprovado sem sua evidência. A verificação das fichas avalia a estrutura documental, não a correção do aplicativo.
