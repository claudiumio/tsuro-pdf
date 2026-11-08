# Modelo de ficha de função

Toda função do catálogo vira uma ficha neste formato. A ficha é a especificação que o desenvolvedor, humano ou agente, vai ler para implementar, e o texto que vira o corpo de uma issue do GitHub. O verificador `verificar_entrega.py` confere a estrutura.

## Regras de formato

- Cada ficha começa com um título de terceiro nível: `### F-DOM-NN. Rótulo funcional (Codinome)`. Para função sem codinome, `### F-DOM-NN. Rótulo funcional (sem codinome)`.
- Cada campo é uma linha que começa com `- **Rótulo.**` e traz o texto na mesma linha. Conteúdo longo usa subitens indentados com dois espaços e um hífen.
- Os rótulos têm o texto exato abaixo, com ponto dentro do negrito. Não renomeie e não omita.
- Use o tipo de ficha indicado no catálogo. Há quatro tipos.

## Ficha completa

Para funções de prioridade alta e média. Os 19 campos, nesta ordem.

1. `**Ficha.**` o texto `completa`.
2. `**Domínio e tipo.**` O domínio (marcar, navegar, arquivo, ambiente) e o tipo (função nova, evolução de função existente, correção de lacuna).
3. `**Promessa.**` Uma frase, do ponto de vista de quem lê o PDF. O que muda para a pessoa.
4. `**Origem (evidência).**` De 2 a 5 referências no formato `arquivo § seção`, só dos documentos locais de pesquisa. Inclua pelo menos uma de reclamação ou pedido de usuários e pelo menos uma de código ou documentação de concorrente. Cite `codigo-e-usuarios.md`, `ui-ux.md` ou `relatorio.md` com o número da seção.
5. `**Não copiar.**` O que de concorrentes não será trazido, com o motivo. Licença, identidade visual, complexidade fora do mission gate.
6. `**Mission gate.**` Qual das quatro tarefas melhora (ler, anotar, marcar, imprimir), se deixa mais rápida, leve ou clara, e o veredito (passa, ajuste ou recusar) com a justificativa.
7. `**Situação atual no Tsuro.**` O que já existe, com `arquivo:linha` verificado, ou "não verificado". Se a função já existe, escreva "já existe" e reduza o escopo ao que falta.
8. `**História de usuário.**` Como (papel), quero (ação), para (valor). Uma só.
9. `**Critérios de aceite.**` De 3 a 6 subitens no formato Gherkin, cada um numa linha só, com `Dado`, `Quando` e `Então`. Testáveis por quem não leu a pesquisa. Inclua pelo menos um caso de borda e um de erro.
10. `**Experiência no Kiri.**` Onde a função vive (barra, HUD, painel, paleta, diálogo), os estados (vazio, carregando, erro, sucesso), o teclado, a acessibilidade e o que não muda no chrome.
11. `**Expressão temática.**` O codinome com kanji e tradução. A descrição geométrica de cada ícone Ori novo (formas, posição do vinco). A microcopy em pt-BR e em EN. O movimento, só se comunicar estado.
12. `**Dados e código.**` Os tipos propostos (enum, struct) e onde entram segundo o mapa do `AGENTS.md`. O que não muda (Pdfium, motor, `tsuro-sign`). Nunca escreva código de implementação, só a forma dos dados.
13. `**Testes.**` Testes de comportamento, não de implementação. As fixtures de `public/samples/`, os casos de borda e como provar que o defeito sumiu.
14. `**Riscos e premissas.**` Pelo menos 2 riscos no formato causa, evento, efeito. As premissas, marcadas como suposição.
15. `**Dependências.**` IDs de outras funções e spikes, crates, aprovações "perguntar antes" e ordem de entrega.
16. `**Tamanho e confiança.**` P, M, G ou GG, mais confiança alta, média ou baixa, mais a lista de incógnitas.
17. `**Prioridade sugerida.**` MoSCoW (Must, Should, Could, Won't) e as parcelas do WSJF de 1 a 5, valor ao usuário, urgência ou redução de risco, oportunidade, tamanho, com a conta e a justificativa.
18. `**Definição de pronto.**` Lista de verificação. Inclui `cargo test -p tsuro`, `cargo fmt --check`, `cargo clippy`, textos em pt-BR e EN, nota de release e verificação em macOS e Windows quando aplicável.
19. `**Rascunho de issue.**` Com três subitens. `Título:` num formato Conventional Commits entre crases. `Labels:` entre crases, só do vocabulário do repositório, sempre com `enhancement` e `needs-triage`. `Corpo:` de 3 a 6 linhas.

## Ficha curta

Para funções exploratórias ou de prioridade baixa. Doze campos, nesta ordem.

1. `**Ficha.**` o texto `curta`.
2. `**Promessa.**`
3. `**Origem (evidência).**` De 1 a 3 referências.
4. `**Não copiar.**`
5. `**Mission gate.**`
6. `**Situação atual no Tsuro.**`
7. `**Hipótese e como validar.**` O que precisa ser verdade para valer a pena e o experimento mais barato que prova ou derruba.
8. `**Critério de saída.**` Condição verificável para decidir seguir, adiar ou recusar.
9. `**Dependências.**`
10. `**Tamanho e prioridade.**` P, M, G ou GG e MoSCoW, com a razão em uma frase.
11. `**Expressão temática.**` Codinome ou "sem codinome", e a descrição do ícone Ori se houver.
12. `**Rascunho de issue.**` Como na ficha completa.

## Ficha de épico

Só para F-AMB-01. Quatorze campos.

1. `**Ficha.**` o texto `épico`.
2. `**Visão.**` O que o produto vira, em linguagem de quem usa.
3. `**Conflito de princípio e controle de mudança.**` Qual princípio do Kiri muda, o que as outras partes do projeto assumem sobre ele e como a mudança deve ser aprovada.
4. `**Origem (evidência).**`
5. `**Não copiar.**`
6. `**Mission gate.**`
7. `**Hipóteses.**` O que precisa ser verdade.
8. `**Experimentos.**` O mais barato primeiro, com material, método e critério de leitura do resultado.
9. `**Faseamento sugerido.**` Fases, cada uma com objetivo e saída.
10. `**Expressão temática.**`
11. `**Riscos e premissas.**`
12. `**Critérios para reabrir a decisão.**` O que precisa acontecer para o PO reabrir o princípio.
13. `**Dependências.**`
14. `**Rascunho de issue.**` Para o épico.

## Ficha de pacto

Só para F-AMB-04. Sete campos.

1. `**Ficha.**` o texto `pacto`.
2. `**Propósito.**` Por que um produto leve precisa de promessas explícitas.
3. `**Cláusulas.**` Uma tabela com `ID`, `O Tsuro nunca...`, `Evidência`, `Como verificar`. De 8 a 14 cláusulas.
4. `**Como verificar a cada release.**` Lista de verificação rodável em dois minutos.
5. `**Origem (evidência).**`
6. `**Expressão temática.**` O codinome Pacto do Ma e o tom.
7. `**Rascunho de issue.**` Uma issue de documentação que registra o pacto.

## Exemplo fictício (fora do escopo)

A ficha abaixo ilustra o formato. A função é inventada e não faz parte do catálogo. O verificador ignora o prefixo `F-EXE`.

### F-EXE-01. Copiar o número da página atual (sem codinome)

- **Ficha.** completa
- **Domínio e tipo.** navegar, função nova
- **Promessa.** Quem cita uma página num e-mail copia o número com um gesto, sem digitar.
- **Origem (evidência).**
  - `codigo-e-usuarios.md § 3` (seção "Página do arquivo e página do livro"), a queixa do LiquidText sobre números de página em citações.
  - `ui-ux.md § 5.1`, que trata comandos com atalho que a paleta não lista.
- **Não copiar.** Nenhum recurso de citação bibliográfica, que é de gerenciadores de referência e fica fora do mission gate.
- **Mission gate.** Melhora ler. Deixa a tarefa de citar uma posição mais rápida. Veredito: passa, com a ressalva de que não vira ferramenta de citação.
- **Situação atual no Tsuro.** Não existe. O contador mostra `n / total` no HUD (`crates/tsuro/src/view.rs:202`). A cópia de seleção existe em `crates/tsuro/src/session.rs:5431`.
- **História de usuário.** Como leitor de contratos, quero copiar o número da página atual, para citá-lo numa mensagem.
- **Critérios de aceite.**
  - **Dado** um PDF aberto na página 7, **quando** o leitor aciona o comando, **então** a área de transferência contém `7`.
  - **Dado** um PDF em rolagem contínua com a página 12 visível, **quando** o leitor aciona o comando, **então** a área de transferência contém `12`.
  - **Dado** nenhum documento aberto, **quando** o leitor aciona o comando, **então** nada é copiado e nenhuma mensagem de erro aparece.
- **Experiência no Kiri.** Vive na paleta (Ctrl+K), sem botão novo no chrome. Mostra um aviso curto na linha de status, que some sozinho. Funciona só com teclado.
- **Expressão temática.** Sem codinome. Sem ícone novo. Microcopy em pt-BR "Número da página copiado." e em EN "Page number copied."
- **Dados e código.** Uma nova variante em `PaletteAction` e uma mensagem que escreve o número na área de transferência. Não toca o motor.
- **Testes.** Estado puro, sem fixture. Com a página visível 7 o texto é `7`. Com nenhum documento, nada é escrito.
- **Riscos e premissas.**
  - O leitor esperar o rótulo impresso e não o índice. Efeito, citação errada em livros. Mitigação, depender de F-NAV-02.
  - Suposição, o leitor quer o número e não um link.
- **Dependências.** F-NAV-02 para o rótulo impresso. Sem aprovação especial.
- **Tamanho e confiança.** P, confiança alta. Incógnita, a posição do aviso na linha de status.
- **Prioridade sugerida.** Could. Valor 2, urgência 1, oportunidade 1, tamanho 1. WSJF igual a (2+1+1) dividido por 1, ou 4. Baixo valor, esforço mínimo.
- **Definição de pronto.** `cargo test -p tsuro` verde. `cargo fmt --check` e `cargo clippy` limpos. Texto em pt-BR e EN. Linha na nota de release. Verificado em macOS e em Windows.
- **Rascunho de issue.**
  - Título: `feat(nav): copiar o número da página atual pela paleta`
  - Labels: `enhancement`, `navigation`, `needs-triage`
  - Corpo: Quem cita uma página precisa digitar o número. Esta função copia o número da página visível pela paleta. Sem botão novo. Depende do rótulo impresso para livros.
