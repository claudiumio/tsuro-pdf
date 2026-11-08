# Briefing do Product Owner e gerente do projeto TsuroPDF

Versão 1.0, 13 de outubro de 2026. Vale para todos os pacotes de trabalho do Programa Senbazuru.

## 1. Quem fala e o que isto autoriza

Eu sou o Product Owner e o gerente deste projeto. Trabalho com o PMBOK, adaptado (tailoring) ao tamanho do TsuroPDF, que é um leitor de PDF open source de uma pessoa só, com apoio de agentes de IA. Você é um especialista da equipe de planejamento. Este documento é a autorização de trabalho (work authorization) do seu pacote. Ele diz o que entregar, em que forma, com que qualidade e dentro de quais limites.

Eu decido escopo, prioridade e mudanças. Você propõe, justifica com evidência e registra o que precisa de decisão minha. Você não muda o catálogo, não decide por mim e não amplia o escopo por conta própria.

## 2. Termo de abertura resumido (project charter)

| Item | Conteúdo |
| --- | --- |
| Nome | Programa Senbazuru (千羽鶴, mil grous). Uma série de pequenas dobras entregues em releases incrementais |
| Propósito | Transformar a pesquisa de concorrentes (três documentos em `docs/research/engenharia-reversa-pdf/`) em funções adaptadas ao Tsuro e em um roadmap executável |
| Justificativa | Usuários de todos os leitores reclamam de perda de trabalho, interface inchada, marcação sem memória, cópia de texto com quebras e impressão borrada. O Tsuro já resolve parte disso e pode resolver o resto sem virar suíte |
| Objetivos mensuráveis (propostos por mim, a validar) | O1. Nenhuma marcação criada há mais de 5 segundos se perde num encerramento forçado. O2. Achar uma marcação por uma palavra e saltar até ela em no máximo 3 ações. O3. Nenhuma regressão de leveza (tempo até a primeira página, memória por aba) em relação à linha de base que será medida. O4. Todo item do roadmap tem critério de aceite testável |
| Requisitos de alto nível | Passar no mission gate. Usar o vocabulário Kiri e Ori. Funcionar em macOS (Apple Silicon) e Windows x64. Ter interface em pt-BR e EN |
| Premissas | Há um mantenedor, com agentes de IA como apoio (a confirmar). A capacidade por período é desconhecida. A cadência recente de releases é uma referência, não uma promessa. Os documentos de pesquisa são a única fonte de evidência sobre concorrentes |
| Restrições | As regras da seção 4 |
| Marcos de alto nível | M0, planejamento aprovado (este pacote). M1, primeira release do roadmap. Os demais marcos são definidos pelo gerente do roadmap |
| Partes interessadas | PO e patrocinador, mantenedores, contribuidores open source, usuários finais |
| Autoridade | O PO aprova escopo, prioridade, mudanças e a abertura de issues |

## 3. Abordagem e adaptação ao projeto (tailoring)

O ciclo de vida é híbrido. Os marcos são preditivos, ou seja, cada release tem um objetivo e critérios de saída. A execução é adaptativa, com backlog priorizado, incrementos pequenos e PRs pequenos.

Não existem datas de calendário neste planejamento. A capacidade não foi informada. Use ordem, dependências, tamanhos relativos e a cadência observada como referência. Fatos medidos estão em `01-estado-do-produto-2026-10-13.md`.

Estimativas usam tamanho relativo (P, M, G, GG) e um nível de confiança (alta, média, baixa), sempre com a lista de incógnitas. Estimativa sem incógnitas listadas não é aceita.

## 4. Regras inegociáveis (restrições do projeto)

1. **Mission gate.** O Tsuro é um leitor veloz, otimizado e leve para ler, anotar, marcar e imprimir, e nada além disso. Toda função precisa responder se deixa uma dessas quatro tarefas mais rápida, mais leve ou mais clara. Suíte, nuvem, formulário e colaboração ficam de fora. Recusar uma função é um resultado válido e esperado.
2. **Código novo.** Só em `crates/tsuro/` ou `crates/tsuro-sign/`. A árvore legada `src/` e `src-tauri/` não se estende. As fixtures de `public/samples/` são só leitura.
3. **Perguntar antes.** Novas dependências, qualquer mudança em `crates/tsuro-sign/` e qualquer toque na árvore legada. Qualquer função que dependa disso declara a dependência e marca "aprovação do PO necessária".
4. **Proibido.** Commitar segredos, mexer à mão em fixtures, `git push --force`, implementar criptografia própria.
5. **Regra de inspiração.** O Tsuro é MIT. SumatraPDF, Sioyek, MuPDF e Okular são GPL ou AGPL, e o Zotero é AGPL. A pesquisa leu esse código só para entender comportamento. Você descreve o comportamento desejado com palavras do Tsuro. Não cole código, não adapte código linha a linha, não copie texto de interface, ícones ou identidade visual de concorrentes. Cada função tem o campo "Não copiar".
6. **Princípios do Kiri.** O documento é a interface. Chrome quieto. Ícones com dica, não rótulos longos. Cores sólidas, sem gradiente, vidro ou transparência decorativa no chrome. O Pdfium, o cache de superfícies e as threads do motor ficam inalterados, a menos que a função declare o contrário com justificativa. A folha do PDF é sempre branca por enquanto. Qualquer proposta que mude isso é uma mudança de princípio e vai para o registro de decisões, não para a implementação.
7. **Verificação.** Antes de citar um arquivo, uma função, uma linha, uma issue ou um número, confirme no insumo ou no arquivo. Se não confirmou, escreva "não verificado". Nunca preencha lacuna com suposição fora do campo "Riscos e premissas", e marque como suposição.
8. **Convenções do repositório.** Commits e títulos de issue seguem Conventional Commits (`feat(annot): ...`). Labels só do vocabulário do repositório, listado em `01-estado-do-produto-2026-10-13.md`. Dependências entre issues usam o recurso nativo do GitHub, não texto solto.
9. **Idioma.** Português do Brasil. Termos japoneses aparecem como codinome com tradução e nunca substituem o rótulo funcional. Detalhes em `02-glossario-tematico.md`.

## 5. Padrão de qualidade do que você entrega

Uma entrega só está pronta quando cumpre tudo abaixo.

- Segue o modelo de ficha de `03-modelo-de-funcao.md`, campo por campo.
- Cada afirmação sobre concorrentes cita a origem no formato `arquivo § seção`, usando só os documentos locais. Não pesquise na web e não use MCPs.
- Cada afirmação sobre o Tsuro cita `arquivo:linha` verificado, ou diz "não verificado".
- Cada critério de aceite é testável por alguém que não leu a pesquisa. Escreva no formato Dado, Quando, Então.
- A parte temática é concreta. Nome, descrição geométrica do ícone Ori e microcopy em pt-BR e EN. Nada de adjetivo vazio.
- O verificador `verificar_entrega.py` passa sem erro.

Estilo de escrita. Frases curtas e declarativas, uma ideia por frase. Voz ativa. Sem travessão (—) nem meia-risca (–). Sem dois-pontos no meio da frase, só antes de lista ou exemplo. Títulos só com a primeira palavra em maiúscula. Sem emojis. Evite as palavras robusto, poderoso, seamless, alavancar, incrível e holístico. Nomes de arquivo, símbolos e comandos ficam entre crases.

## 6. Protocolo de execução em turno único

Você tem um único turno. Não faça perguntas ao PO e não espere resposta. Vá até o fim.

1. Leia, nesta ordem, os insumos `01`, `02`, `03` e `04` desta pasta e depois só os trechos dos documentos de pesquisa que o seu pacote cita.
2. Na primeira ação de escrita, crie o arquivo de saída com o cabeçalho de cada função do seu pacote, cada um com todos os rótulos do modelo e o texto "a preencher" em cada campo.
3. Preencha uma função por vez e salve depois de cada uma. Não segure o texto na memória até o fim.
4. Rode o verificador. Corrija até ele passar. Ele não aceita "a preencher".
5. Só então escreva o relatório final.

Se o contexto ou o tempo estiver acabando, complete as fichas completas antes de ampliar qualquer outra. Nunca deixe campo vazio. Se não conseguir concluir uma ficha, escreva "não concluído" com o motivo no campo "Riscos e premissas" e deixe o resto coerente.

## 7. Relatório final (máximo 15 linhas)

1. Arquivo escrito, com caminho completo.
2. Quantas fichas por tipo.
3. A saída literal do verificador.
4. IDs com decisão pendente do PO.
5. O que ficou "não verificado".
6. Desvios propostos ao catálogo, com a evidência.
7. O que você não fez.

## 8. Escalonamento e controle de mudanças

Quando a evidência contradiz o catálogo ou uma prioridade minha, você não altera o catálogo. Registre em "Desvios propostos" com a evidência. Quando o código já faz o que a função pede, marque "já existe" e ajuste o escopo para o que falta de verdade. Quando uma função fere o mission gate, recomende "recusar" e explique. Quando duas funções entram em conflito de princípio ou de dependência, registre o conflito nas duas.

## 9. Glossário rápido de gerenciamento

| Termo | Uso neste projeto |
| --- | --- |
| Ficha | Especificação de uma função, pronta para virar issue |
| Ficha completa, curta, épico, pacto | Os quatro tipos de ficha de `03-modelo-de-funcao.md` |
| Mission gate | Teste de escopo da seção 4, item 1 |
| Spike | Investigação curta com pergunta, método e critério de saída, para tirar uma incógnita antes de comprometer a função |
| EAP | Estrutura analítica do projeto. Decompõe o escopo em pacotes de trabalho |
| Critério de saída | Condição verificável para considerar uma release ou um spike concluído |
| Linha de base | Medida de partida, feita antes de mudar, para detectar regressão |
| WSJF | Custo do atraso dividido pelo tamanho. O método de priorização do roadmap |
| MoSCoW | Must, Should, Could, Won't. Classe de prioridade |
| Decisão pendente | Ponto que só o PO resolve. Vai para o registro de decisões |
