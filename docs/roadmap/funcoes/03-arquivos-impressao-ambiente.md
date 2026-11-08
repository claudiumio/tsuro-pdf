# Funções de arquivos, impressão e ambiente

Onze requisitos propostos. O coordenador revisou a entrega Step 5 Preview de 13 de outubro de 2026. Nenhuma função foi implementada, testada ou aprovada para release.

Os fatos técnicos vêm de `../_insumos/01-estado-do-produto-2026-10-13.md`, não revalidado nesta rodada. As âncoras são pontos de partida para conferir antes de implementar. Busca textual não prova ausência. O protocolo 05 prevalece sobre o briefing 00.

Prioridades, tamanhos e WSJF são propostas. Valor, urgência ou redução de risco, oportunidade e tamanho usam notas de 1 a 5. Dependências e segurança prevalecem sobre pontuação. Não há compromisso de calendário. Codinomes ficam na documentação até revisão linguística. A interface mantém nomes funcionais em pt-BR e EN.

O usuário decidiu explorar papel oriental mais adiante. Washi continua adiado. A folha branca é a regra vigente do Kiri. Nenhuma outra ficha depende dessa exploração.

## Abrir e salvar com segurança

### F-ARQ-01. Senha ao abrir (Kagi)

- **Ficha.** completa
- **Domínio e tipo.** arquivo, evolução da abertura de PDFs protegidos, com lacuna a confirmar.
- **Promessa.** O leitor abre um PDF com a senha legítima sem expor a senha nem reduzir sua proteção em silêncio.
- **Origem (evidência).** `codigo-e-usuarios.md § 3`, problemas de abertura e proteção. `codigo-e-usuarios.md § 2`, mecanismos dos leitores abertos. `codigo-e-usuarios.md § 4.2`, proposta para senha de abertura.
- **Não copiar.** Não copiar criptografia ou diálogo de outro leitor. Não guardar senha por conveniência, nem oferecer remoção de proteção como consequência automática da abertura.
- **Mission gate.** Passa. Resolve bloqueio direto de leitura, se S-05 confirmar suporte e limites.
- **Situação atual no Tsuro.** O snapshot registra leitura de bytes pelo motor, mas não confirma o fluxo completo de senha. Âncoras não revalidadas são `crates/tsuro/src/engine.rs:198` e `crates/tsuro/src/engine.rs:279`. Não afirmar inexistência sem revalidar.
- **História de usuário.** Como leitor de documento protegido, quero fornecer a senha de abertura para ler sem alterar o arquivo original.
- **Critérios de aceite.** Verificar os seguintes comportamentos.
  - Dado senha válida e suporte confirmado, Quando confirma a abertura, Então abre o documento solicitado e preserva os bytes do original.
  - Dado senha incorreta, Quando confirma, Então permite nova tentativa e distingue senha recusada de arquivo corrompido ou mecanismo não suportado quando o motor permitir essa distinção.
  - Dado pedido de senha aberto, Quando cancela, Então remove o pedido e preserva documentos e trabalho já abertos.
  - Dado um PDF sem senha, Quando abre, Então não exibe pedido de senha.
  - Dado um PDF protegido aberto, Quando salva ou exporta conteúdo, Então não reduz a proteção silenciosamente e bloqueia saída incompatível ou exige decisão explícita antes de escrever uma saída menos protegida.
  - Dado qualquer fluxo de senha, Quando termina a necessidade do segredo, Então descarta referências controladas pelo Tsuro e não registra a senha em log, preferência, cache, diário, arquivo ou telemetria.
- **Experiência no Kiri.** Cartão local com campo mascarado, revelar temporariamente, confirmar e cancelar. Erro não fecha o documento anterior. Foco inicial no campo e saída por teclado. Explicar limite de suporte sem sugerir contorno de permissões. Não oferecer lembrar senha na v1.
- **Expressão temática.** Kagi, 鍵, chave, codinome interno. Ori candidato com dois polígonos que formam uma chave compacta e vinco no encontro da haste. PT "Senha para abrir este PDF" e "Não foi possível abrir com esta senha." EN "Password to open this PDF" e "Could not open with this password."
- **Dados e código.** Propor estado de abertura pendente e resultado de abertura que distingue pedido de senha, recusa, erro e sucesso. Estado em `session.rs`, cartão em `view.rs`, senha transitória na requisição ao motor em `engine.rs`. A avaliação técnica confirmou que a construção de `Ready` exige análise pelo segundo parser. Verificar o fluxo completo e definir estado explícito de análise indisponível, sem representar ausência de análise como ausência de assinaturas. Não mudar arquitetura de worker nem reimplementar criptografia. Mudanças em `tsuro-sign` exigem autorização específica, não são consequência automática desta issue. Descartar strings controladas não prova zeroização de cópias internas do motor.
- **Testes.** Novas fixtures autorizadas para senha correta, incorreta, mecanismo não suportado e permissões diferentes. Não alterar fixtures existentes. Cancelar com outra aba marcada aberta, repetir tentativa e verificar logs. Testar save-copy, exportação e recuperação para não criar saída desprotegida sem escolha explícita.
- **Riscos e premissas.** Logs de mensagens podem expor segredo. Excluir segredo de formatação, diagnóstico e serialização. Recuperação ou exportação de texto pode expor conteúdo de PDF protegido. Proposta de v1, não persistir recuperação em texto claro desses documentos sem política segura aprovada. A abertura pode precisar de mudanças de mensagens do motor. Suposição, os bindings instalados suportam o mecanismo do corpus.
- **Dependências.** S-05 favorável é rígida. Alinhar política de saídas e recuperação com F-MARC-01. F-ARQ-06 fornece explicação, mas seu painel não bloqueia um aviso mínimo de abertura. F-ARQ-02 é outro caminho de salvamento, não pré-requisito geral. Nova dependência ou mudança de esquema persistido exige aprovação prévia.
- **Tamanho e confiança.** M, confiança baixa. Incógnitas de API, mecanismo de proteção, erros, saída criptografada e resíduos em memória.
- **Prioridade sugerida.** Must no pacote de abertura, condicionado a S-05. Valor 4, urgência 4, oportunidade 3, tamanho 2. WSJF = (4+4+3)/2 = 5,5. A classificação não prova suporte ou estimativa.
- **Definição de pronto.** Abertura real, cancelamento seguro e política de saídas protegidas validada em macOS e Windows. Textos PT e EN, nota de release, `cargo test -p tsuro`, `cargo fmt --check` e `cargo clippy`. Comparar abertura com S-06 e registrar limites de memória do segredo.
- **Rascunho de issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(file): abrir PDFs com senha sem persistir o segredo`
  - Labels: `enhancement`, `needs-triage`
  - Corpo: Revalidar abertura e S-05. Usar senha transitória, tratar tentativa e cancelamento sem perder sessão anterior. Definir proteção das saídas e recuperação. Não guardar senha nem reduzir proteção silenciosamente.

### F-ARQ-02. Salvar sem quebrar assinaturas (Hanko)

- **Ficha.** completa
- **Domínio e tipo.** arquivo, evolução condicionada do salvamento de cópias anotadas.
- **Promessa.** O leitor entende se pode salvar marcações sem comprometer a avaliação das assinaturas, e conserva o original quando isso não for garantido.
- **Origem (evidência).** `codigo-e-usuarios.md § 2`, escrita incremental no pdf.js. `codigo-e-usuarios.md § 3`, salvar e interoperabilidade. `codigo-e-usuarios.md § 4.9`, salvamento de assinados. `relatorio.md § 5.7`, limites de confiança.
- **Não copiar.** Não copiar escritor PDF ou código criptográfico. Não chamar append incremental de garantia de assinatura válida. Não modificar original ou incorporar uma assinatura nova por conta própria.
- **Mission gate.** Ajuste. Passa para salvamento com avaliação explícita e preservação do original. A promessa absoluta do título não é garantia técnica ou jurídica.
- **Situação atual no Tsuro.** O snapshot registra salvar cópia, aviso de assinatura e painel existentes. Âncoras não revalidadas são `crates/tsuro/src/engine.rs:396`, `crates/tsuro/src/engine.rs:720` e `crates/tsuro/src/view.rs:1926`. O incremento depende de S-01, não recria painel ou cópia.
- **História de usuário.** Como leitor de contrato assinado, quero conhecer o efeito de salvar minhas notas para não perder confiança no documento.
- **Critérios de aceite.** Verificar os seguintes comportamentos.
  - Dado um documento assinado, Quando avalia o salvamento, Então distingue bytes assinados, revisão assinada, alterações posteriores e permissões de alteração conhecidas, sem inferir validade global de bytes preservados.
  - Dado caminho aprovado por S-01 para um caso suportado, Quando grava e reabre a cópia, Então confere o resultado contra o contrato de preservação daquele caso e registra divergências com validadores independentes.
  - Dado permissão incompatível ou avaliação inconclusiva, Quando pretende salvar marcações, Então não apresenta salvamento como seguro para assinaturas e oferece cancelar ou manter as marcações fora do PDF.
  - Dado várias assinaturas, Quando avalia a cópia, Então apresenta resultado por assinatura e revisão, inclusive estados não verificados.
  - Dado erro na preparação, escrita ou verificação da cópia, Quando o erro ocorre, Então não informa preservação bem-sucedida, conserva original e trabalho da sessão e identifica eventual arquivo parcial sem apagar arquivo anterior.
  - Dado PDF não assinado ou cancelamento, Quando usa salvar cópia, Então conserva o contrato existente de cópia ou não grava após cancelar, sem alterar o original.
- **Experiência no Kiri.** Evoluir aviso existente com resumo do efeito e opção de cancelar. Um estado desconhecido não recebe selo verde. Explicar que a cópia anotada pode ser uma revisão posterior mesmo com bytes antigos intactos. Permitir exportação local separada sem afirmar que substitui assinatura válida.
- **Expressão temática.** Hanko, 判子, carimbo, codinome interno. Reusar selo Ori do painel em vez de ícone de aprovação universal. PT "Efeito sobre as assinaturas não verificado" e "Original preservado. A cópia contém alterações posteriores." EN "Effect on signatures not verified" e "Original preserved. The copy contains later changes."
- **Dados e código.** Propor avaliação de salvamento por assinatura e revisão com estados permitido, incompatível e não verificado. Não reduzir a avaliação a `preserves_signed_bytes: bool`. `engine.rs` prepara a cópia pelo caminho aprovado, `session.rs` acompanha avaliação e resultado, `view.rs` comunica. Reusar o motor CMS de `crates/tsuro-sign` sem reimplementar. Modificá-lo exige autorização específica. Disponibilidade de escritor ou bindings continua a verificar.
- **Testes.** `contrato-assinado.pdf` é entrada somente leitura, não prova cobertura de certificação ou múltiplas revisões. Propor corpus adicional com várias assinaturas, restrições de alteração, arquivos alterados e proteção. Comparar bytes, revisão, validação e leitura em outros motores autorizados, sem instalar concorrentes nesta tarefa. Simular falha de disco e cancelamento.
- **Riscos e premissas.** Preservar bytes pode deixar alteração posterior proibida. Avaliar permissão e revisão separadamente. Verificador externo pode divergir por cadeia, revogação ou política. Registrar estado e limite, sem prometer equivalência. Suposição, S-01 pode achar pelo menos um caso seguro. Recusa ou salvamento externo às marcações são resultados válidos.
- **Dependências.** S-01 favorável e contrato de preservação aprovado são rígidos. Aprovação antes de mudanças em `crates/tsuro-sign`, dependências ou esquema. F-ARQ-07 alinha linguagem, não cria ciclo técnico. F-MARC-01 e F-MARC-07 mantêm v1 local independente do salvamento incremental. S-06 mede custo.
- **Tamanho e confiança.** GG, confiança baixa. Incógnitas de escritor, permissões de alteração, revisões, confiança e interoperabilidade.
- **Prioridade sugerida.** Should condicionado a S-01. Valor 4, urgência 4, oportunidade 3, tamanho 5. WSJF = (4+4+3)/5 = 2,2. O risco prevalece sobre score e promessa do título.
- **Definição de pronto.** Casos suportados e recusados publicados em documentação local. Evidência por revisão e erro de saída, não só PDF que abre. PT e EN, nota de release, `cargo test -p tsuro`, `cargo fmt --check`, `cargo clippy` e macOS e Windows. Testes do crate de assinatura conforme regras atualizadas, somente se autorizado modificar esse crate.
- **Rascunho de issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(file): avaliar salvamento de marcações em PDFs assinados`
  - Labels: `enhancement`, `needs-triage`
  - Corpo: Executar S-01 antes de compromisso. Separar bytes, revisões, alterações permitidas e confiança. Preservar original e não afirmar validade global por append incremental. Limitar, adiar ou recusar casos inconclusivos.

## Imprimir com resultado previsível

### F-ARQ-03. Preparação de impressão leve e fiel (Sumi)

- **Ficha.** completa
- **Domínio e tipo.** imprimir, evolução condicionada da preparação de saída.
- **Promessa.** O leitor prepara a impressão com memória controlada, cancelamento e fidelidade preservada. Aumento de DPI e seletor de resolução foram excluídos pelo usuário em 13 de outubro de 2026. O caminho raster mantém os 200 DPI atuais.
- **Origem (evidência).** `codigo-e-usuarios.md § 3`, queixas de nitidez e impressão. `codigo-e-usuarios.md § 4.8`, rasterização e vetores. `relatorio.md § 5.6`, saída previsível.
- **Não copiar.** Não copiar driver ou caminho de impressão de outro leitor. Não considerar a rasterização deliberada um bug comprovado. Não prometer impressão vetorial sem confirmar suporte.
- **Mission gate.** Passa se a avaliação demonstrar melhoria de imprimir sem perda de fidelidade ou leveza.
- **Situação atual no Tsuro.** O snapshot registra prévia e rasterização deliberada a 200 DPI. Âncoras não revalidadas são `crates/tsuro/src/print.rs:18`, `crates/tsuro/src/print.rs:62` e `crates/tsuro/src/spool.rs:35`. O incremento é qualidade e custo, não criação da impressão.
- **História de usuário.** Como leitor que imprime documentos longos, quero preparação com memória controlada e cancelamento para enviar uma saída fiel sem elevar a resolução.
- **Critérios de aceite.** Verificar os seguintes comportamentos.
  - Dado corpus e critérios fixados antes de S-03, Quando compara caminhos de preparação sem elevar DPI, Então registra fidelidade, pico de memória do trabalho inteiro, tempo e cancelamento e só aceita caminho que cumpra o orçamento aprovado.
  - Dado caminho de saída aceito, Quando imprime intervalo, cópias e orientação escolhidos, Então o trabalho preparado corresponde às escolhas, sem página omitida ou duplicação não solicitada.
  - Dado tamanhos de página mistos, Quando escolhe escala ou ajuste, Então a saída e a prévia refletem a política explícita e não cortam conteúdo silenciosamente.
  - Dado via vetorial incompatível, Quando prepara a saída, Então mantém caminho raster aprovado ou informa o limite antes de enviar, sem fallback invisível que altere fidelidade.
  - Dado preparação cancelada antes da submissão, Quando cancela, Então não submete o trabalho e não informa impressão concluída.
  - Dado falha ou estado incerto do spool, Quando recebe retorno, Então mostra falha ou estado desconhecido e não promete que nenhuma página saiu nem que a impressão física terminou.
- **Experiência no Kiri.** Reusar prévia e controles. Manter configuração pequena. Mostrar progresso de preparação e estado de envio. Não exibir indicador técnico de vetor como substituto de qualidade. Definir interface para cancelar antes do envio e explicar limites após envio.
- **Expressão temática.** Sumi, 墨, tinta, codinome interno. Reusar impressora Ori. PT "Preparando impressão" e "Enviado à fila. A conclusão depende da impressora." EN "Preparing print job" e "Sent to queue. Completion depends on the printer."
- **Dados e código.** Investigar preparação limitada por página, orçamento agregado e cancelamento. O caminho raster permanece em 200 DPI. Preservação vetorial somente após S-03 confirmar viabilidade e fidelidade, nunca como promessa de aumentar resolução. `print.rs` prepara trabalho, `spool.rs` informa submissão, `session.rs` acompanha estado, `view.rs` mostra prévia. Verificar importação de páginas e efeito de anotações antes de escolher caminho. Não alterar cache de leitura por conveniência. `tsuro-sign` não muda.
- **Testes.** Texto pequeno, linhas finas, gráficos, imagens, códigos de barras e páginas mistas. Conferir saída preparada e impressão física futura em plataformas e drivers disponíveis, identificando cobertura faltante. `guia-folio.pdf` pode servir para parte do corpus após inspeção. Medir memória, escala e tempo com S-06. Não sobrescrever fixtures.
- **Riscos e premissas.** Vetores podem perder fontes ou mudar anotações no driver. Comparar saída, não só existência de objetos vetoriais. Acumular todas as páginas pode elevar memória mesmo nos 200 DPI atuais. Limitar preparação e medir o trabalho inteiro. Aumento de resolução não é alternativa deste plano. Suposição, o caminho atual pode continuar como solução adequada em parte dos casos.
- **Dependências.** S-03 favorável e orçamento de S-06 são rígidos para trocar caminho. Coordenar contrato com F-ARQ-04, sem obrigar sua entrega conjunta. Nova dependência exige aprovação. Não depende de assinatura ou Washi.
- **Tamanho e confiança.** G, confiança baixa. Incógnitas de API, fontes, drivers, memória e fidelidade.
- **Prioridade sugerida.** Should condicionado ao spike. Valor 4, urgência 3, oportunidade 2, tamanho 4. WSJF = (4+3+2)/4 = 2,25.
- **Definição de pronto.** Corpus, orçamento e resultados de qualidade documentados. Estados de preparação e spool corretos. PT e EN, nota de release, `cargo test -p tsuro`, `cargo fmt --check`, `cargo clippy`, macOS e Windows com cobertura de driver declarada. Não marcar impressão física validada se só foi inspecionado PDF de saída.
- **Rascunho de issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(print): limitar custo da preparação sem aumentar DPI`
  - Labels: `enhancement`, `print`, `needs-triage`
  - Corpo: Investigar memória agregada, preparação limitada e cancelamento por S-03. Manter 200 DPI no raster, sem seletor ou aumento de resolução. Preservação vetorial somente se viável e fiel. Fixar corpus e orçamento. Manter intervalo, escala, cópias e orientação. Diferenciar preparação, fila e impressão física.

### F-ARQ-04. Imprimir com ou sem marcações (sem codinome)

- **Ficha.** completa
- **Domínio e tipo.** imprimir, evolução das opções de prévia e saída.
- **Promessa.** O leitor escolhe incluir ou omitir as marcações da sessão e sabe quais marcas do PDF continuarão na impressão.
- **Origem (evidência).** `ui-ux.md § 3`, controles de impressão dos leitores pesquisados. `ui-ux.md § 5.6`, imprimir sem marcações. `codigo-e-usuarios.md § 3`, saída inesperada. `codigo-e-usuarios.md § 4.8`, limites de rasterização e anotações.
- **Não copiar.** Não copiar tela avançada de suíte. Não prometer apagar marcas achatadas nem tratar toda anotação incorporada como conteúdo inseparável.
- **Mission gate.** Passa. Torna imprimir previsível sem editar o documento.
- **Situação atual no Tsuro.** O snapshot registra prévia e preparação com marcações. Âncoras não revalidadas são `crates/tsuro/src/session.rs:1856`, `crates/tsuro/src/print.rs:50` e `crates/tsuro/src/view.rs:1191`. Revalidar ramificação com lista vazia antes de afirmar reuso completo.
- **História de usuário.** Como leitor que entrega uma cópia em papel, quero omitir minhas notas da sessão para imprimir somente o documento conforme os limites conhecidos.
- **Critérios de aceite.** Verificar os seguintes comportamentos.
  - Dado marcações da sessão e inclusão ligada, Quando prepara impressão, Então prévia e saída contêm as mesmas marcações elegíveis.
  - Dado inclusão desligada, Quando prepara impressão, Então prévia e saída omitem marcações da sessão sem apagar os dados da sessão.
  - Dado anotações padrão incorporadas ao PDF, Quando escolhe omitir marcas, Então a interface declara o comportamento suportado para elas e não as confunde com marcas achatadas.
  - Dado marcas achatadas, Quando consulta a opção, Então informa que essa opção não remove conteúdo já incorporado à página.
  - Dado nenhuma marcação da sessão, Quando alterna a opção, Então não altera conteúdo, intervalo, escala ou orientação.
  - Dado cancelamento antes de enviar ou erro de preparação e spool, Quando a operação termina, Então informa o estado real, não indica impressão física concluída e mantém sessão e original intactos.
- **Experiência no Kiri.** Controle "Incluir marcações desta sessão" na impressão e legenda sobre marcas do arquivo. Uma v1 pode não oferecer ocultar anotações padrão, mas deve distinguir seu limite. Revalidar persistência e padrão antes de escolher. Prévia não pode mostrar uma política e spool usar outra. Foco e rótulo acessíveis por teclado.
- **Expressão temática.** Sem codinome novo. Reusar impressora Ori. PT "Incluir marcações desta sessão" e "Marcas achatadas fazem parte da página." EN "Include session annotations" e "Flattened marks are part of the page."
- **Dados e código.** Propor política explícita de inclusão por origem, inicialmente sessão, arquivo padrão e conteúdo achatado. Separar capacidade de omitir de escolha do usuário. Não reduzir toda marca a booleano universal. `print.rs` e a prévia recebem a mesma política. Estado em `session.rs`, controle em `view.rs`. Mudanças de parâmetros do motor podem ser necessárias para anotações padrão, a verificar. `tsuro-sign` não muda.
- **Testes.** PDF sem marcas, marcações da sessão, anotações padrão, marcas achatadas e combinação das três. Comparar prévia e arquivo preparado. Simular falha e cancelamento antes de envio. Criar corpus adicional sem modificar fixtures existentes. Não inferir ausência de impressão física de erro tardio de spool.
- **Riscos e premissas.** Nome genérico pode sugerir apagar todas as marcas. Nomear origem e declarar limites. Prévia e saída podem divergir por cache ou estratégia. Compartilhar política e invalidar prévia quando ela muda. Suposição, sessão pode ser omitida sem alterar o PDF original.
- **Dependências.** Revalidar preparação existente é rígida. Confirmar capacidades para qualquer opção de anotações padrão. F-ARQ-03 informa caminho futuro, mas não bloqueia v1 que omite apenas sessão no caminho atual. S-06 mede custo. Sem nova dependência prevista.
- **Tamanho e confiança.** M, confiança média para omitir sessão, baixa para opção ampliada de anotações padrão. Incógnitas de motor, cache e política padrão.
- **Prioridade sugerida.** Should. Valor 4, urgência 3, oportunidade 2, tamanho 2. WSJF = (4+3+2)/2 = 4,5. Estimativa da v1 limitada à sessão.
- **Definição de pronto.** Prévia e saída coerentes nos três tipos de marcas, erros honestos e original preservado. PT e EN, nota de release, `cargo test -p tsuro`, `cargo fmt --check`, `cargo clippy`, macOS e Windows. Cobertura de impressão física declarada, não presumida.
- **Rascunho de issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(print): escolher inclusão de marcações da sessão`
  - Labels: `enhancement`, `ux`, `needs-triage`
  - Corpo: V1 omite ou inclui marcas da sessão. Explicar anotações padrão e conteúdo achatado separadamente. Compartilhar política entre prévia e saída. Não alterar original ou depender da impressão vetorial.

### F-ARQ-05. Folha de marcações impressa (sem codinome)

- **Ficha.** curta
- **Promessa.** O leitor revisa trechos e notas numa folha separada sem imprimir todo o PDF.
- **Origem (evidência).** `ui-ux.md § 3`, resumos em leitores comerciais. `relatorio.md § 5.4`, marcações fora do leitor. `relatorio.md § 5.6`, revisão no papel.
- **Não copiar.** Não copiar layout ou relatórios de suíte, nem exigir um editor de documentos.
- **Mission gate.** Passa se uma folha simples reduzir trabalho de revisão.
- **Situação atual no Tsuro.** O snapshot registra dados de marcações e exportação Markdown, não um layout impresso validado. Âncora não revalidada é `crates/tsuro/src/session.rs:411`. Não depender do texto Markdown como modelo de apresentação.
- **Hipótese e como validar.** Gerar protótipo futuro a partir de dados semânticos com nome do documento, posição física em base 1 e trecho. Notas são opt-in. Conferir retorno ao trecho e paginação com marcações longas, múltiplas páginas e caracteres PT e EN.
- **Critério de saída.** Toda marca incluída tem origem e posição identificáveis, texto legível sem corte e escolha explícita de notas. Prévia corresponde à saída. Cancelamento e erro não indicam sucesso. Original intacto. Recusar ampliação para editor de relatório.
- **Dependências.** Dados semânticos existentes e impressão, ambos a revalidar. F-MARC-02, F-MARC-05 e F-NAV-02 são opcionais. V1 completa com posições físicas não espera lista, cores, labels ou impressão vetorial.
- **Tamanho e prioridade.** M, Could, confiança baixa. Incógnitas de layout, fontes, paginação e privacidade.
- **Expressão temática.** Sem codinome. Reusar marcação e impressora Ori. PT "Imprimir folha de marcações" e "Incluir notas". EN "Print annotation sheet" e "Include notes".
- **Rascunho de issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(print): avaliar folha separada de marcações`
  - Labels: `enhancement`, `needs-triage`
  - Corpo: Usar dados semânticos, origem e posição física. Notas opt-in e saída legível com paginação. Não bloquear pela lista de marcas ou labels. Sem layout de concorrente ou editor de relatórios.

### F-ARQ-06. Proteção e permissões explicadas (sem codinome)

- **Ficha.** curta
- **Promessa.** O leitor entende quais ações o documento permite e quais capacidades o Tsuro não verificou.
- **Origem (evidência).** `codigo-e-usuarios.md § 3`, problemas de arquivo protegido. `codigo-e-usuarios.md § 4.2`, senha e permissões distintas.
- **Não copiar.** Não remover restrições ou apresentar uma senha de abertura como autorização de editar, copiar ou imprimir.
- **Mission gate.** Passa. Evita tentativas repetidas de tarefas indisponíveis sem criar editor de segurança.
- **Situação atual no Tsuro.** Estado e apresentação das permissões não foram revalidados. Existe aviso de salvamento no snapshot, mas não prova explicação por capacidade.
- **Hipótese e como validar.** S-05 identifica o que o motor informa. Comparar painel e disponibilidade de ações com corpus de senha de abertura, permissões de impressão, cópia e anotação. Avaliar entendimento dos estados liberado, restrito e não verificado.
- **Critério de saída.** Cada ação tem estado próprio e justificativa conhecida. Estado desconhecido não vira permissão concedida ou restrição inventada. Explicação e despacho são coerentes. Não oferecer bypass. Se o motor não distinguir capacidades, declarar limite.
- **Dependências.** S-05 rígida. Compartilha modelo com F-ARQ-01 sem exigir sua conclusão para documentar permissões de arquivos já abertos. Consumidores usam aviso mínimo e política de segurança sem esperar painel completo.
- **Tamanho e prioridade.** M, Should, confiança baixa. Incógnitas de capacidade do motor e política para estado desconhecido.
- **Expressão temática.** Sem codinome. Reusar cadeado Ori se disponível. PT "Permissões do documento" e "Não verificado pelo Tsuro". EN "Document permissions" e "Not verified by Tsuro".
- **Rascunho de issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(file): explicar permissões por ação`
  - Labels: `enhancement`, `needs-triage`
  - Corpo: Usar capacidades verificadas por S-05. Separar abertura, cópia, anotação e impressão. Mostrar desconhecido sem inferir autorização. Alinhar interface e despacho, sem remover restrições.

### F-ARQ-07. Clareza de confiança nas assinaturas (Selo)

- **Ficha.** curta
- **Promessa.** O leitor distingue assinatura detectada de verificações de integridade e confiança executadas.
- **Origem (evidência).** `relatorio.md § 3`, distinção de confiança nos leitores comerciais. `relatorio.md § 5.7`, estados de assinatura sem exagero.
- **Não copiar.** Não copiar selo de validade de outro leitor ou prometer validade jurídica, cadeia e revogação que não foram verificadas.
- **Mission gate.** Passa. Ajuda ler e salvar com entendimento do documento, sem ampliar para serviço de assinatura.
- **Situação atual no Tsuro.** O painel já existe no snapshot. Âncora não revalidada é `crates/tsuro/src/view.rs:3231`. Incremento é texto e estados, não criação de verificador.
- **Hipótese e como validar.** Revalidar painel e dados do crate de assinatura. Separar integridade dos bytes, revisão e alterações posteriores, permissões de alteração, cadeia, tempo e revogação quando disponíveis. Usar corpus adicional quando a fixture atual não cobrir um estado.
- **Critério de saída.** Cada conclusão exibida corresponde à verificação realmente executada. Dados ausentes aparecem como não verificados. Conclusões não se fundem num selo universal de confiança. Revisão humana do texto confirma limites técnicos e jurídicos.
- **Dependências.** Revalidar painel e contrato de resultados. F-ARQ-02 compartilha linguagem, mas nenhuma função espera a outra. Autorização específica antes de tocar `crates/tsuro-sign`. Rede e dependências novas não são presumidas necessárias.
- **Tamanho e prioridade.** P para texto com dados já disponíveis, G se exigir verificação nova. Should para clareza, confiança média na parte textual e baixa no acréscimo de capacidades.
- **Expressão temática.** Selo é codinome em português. Reusar ícone do painel sem cor única que misture estados. PT "Integridade verificada nesta revisão" e "Confiança do certificado não verificada". EN "Integrity verified for this revision" e "Certificate trust not verified".
- **Rascunho de issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(file): separar integridade e confiança nas assinaturas`
  - Labels: `enhancement`, `ux`, `needs-triage`
  - Corpo: Revalidar dados exibidos pelo painel. Separar revisão, alteração e confiança. Nomear verificações ausentes. Não prometer validade jurídica ou ampliar criptografia sem autorização.

## Explorar conforto sem mudar o papel agora

### F-AMB-01. Tema de papel oriental (Washi)

- **Ficha.** épico
- **Visão.** Mais adiante, avaliar um modo opcional de leitura inspirado em papel oriental artesanal, com legibilidade e cores verificadas, sem alterar o arquivo ou a saída.
- **Conflito de princípio e controle de mudança.** A regra vigente do Kiri mantém a folha branca nos dois temas. O usuário mostrou interesse em explorar depois, não aprovou recoloração, textura, padrão novo ou mudança de princípio. Registrar pedido de mudança antes de qualquer experimento de renderização. Uma experiência visual não muda a especificação automaticamente.
- **Origem (evidência).** `codigo-e-usuarios.md § 3`, conforto e pedidos de textura e recoloração. `codigo-e-usuarios.md § 5`, tema do papel. `ui-ux.md § 6`, princípio da folha branca. `relatorio.md § 5.5`, conforto sem modificar documento.
- **Não copiar.** Não copiar texturas, shaders, papéis, paletas ou layout de outro leitor. Não usar asset cultural ou termo japonês sem origem e revisão. Não colocar ornamento sobre conteúdo por identidade de marca.
- **Mission gate.** Ajuste. Só passa se melhora ler sem comprometer documento primeiro, fidelidade e leveza. Continua Won't na sequência atual.
- **Hipóteses.** Uma experiência discreta de papel pode ajudar parte dos leitores. A textura pode ser desnecessária ou prejudicar texto. Imagens, tabelas e cores precisam manter sentido. Todas são suposições, não resultados de uso.
- **Experimentos.** Depois de autorização para exploração, S-08 compara folha branca, tom discreto e eventual textura própria com mesmo texto e condições declaradas. Fixar corpus, critério de legibilidade e cor antes de avaliar. Mockup estático avalia preferência visual, não desempenho do aplicativo ou fadiga clínica. Um protótipo nativo posterior mede custo e saída preservada com S-06. Não executar nesta entrega.
- **Faseamento sugerido.** Registrar interesse e princípio vigente agora. Autorizar investigação futura. Executar S-08 em baixo custo e decidir recusa ou continuação. Só depois, aprovar exceção de princípio e protótipo nativo reversível. Release exige evidência e nova decisão explícita, opt-in nesta proposta.
- **Expressão temática.** Washi, 和紙, papel japonês, codinome interno. Inspiração no material e no espaço do papel, não caricatura ou kanji decorativo. Reusar folha Ori. PT eventual "Aparência do papel" e "Esta opção muda só a leitura na tela." EN "Paper appearance" e "This option changes on-screen reading only."
- **Riscos e premissas.** Tom ou textura pode diminuir contraste e falsificar cor. Comparar corpus e permitir desligar. Recoloração pode provocar piscar e custo de memória. Medir antes da decisão. Representação cultural superficial pode parecer identidade copiada. Usar material próprio e revisão linguística. Suposição, existe benefício que justifique exceção ao Kiri.
- **Critérios para reabrir a decisão.** Autorização explícita do usuário para explorar, problema de leitura ou preferência a avaliar, corpus e método definidos. Depois do experimento, resultado favorável, custo aprovado e saída original preservada são condições para considerar mudança de princípio, não autorização automática. Resultado negativo mantém folha branca.
- **Dependências.** S-08, S-06, revisão linguística e autorização de mudança de princípio antes de implementar exceção. Nenhuma função espera Washi. Não altera cópia, salvamento ou impressão nesta proposta.
- **Rascunho de issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(theme): registrar exploração futura de papel Washi`
  - Labels: `enhancement`, `ux`, `needs-triage`
  - Corpo: Manter folha branca agora. Explorar papel oriental só quando autorizado. Separar preferência de legibilidade, fidelidade e custo. Uma experiência favorável não aprova automaticamente mudança de princípio ou release.

### F-AMB-02. Modo foco (Névoa)

- **Ficha.** completa
- **Domínio e tipo.** ambiente, evolução do chrome existente.
- **Promessa.** O leitor reduz controles persistentes durante a leitura e retorna a eles sem perder posição ou acesso a alertas.
- **Origem (evidência).** `ui-ux.md § 3`, mecanismos de leitura dos produtos pesquisados. `ui-ux.md § 5.5`, foco em estados. `codigo-e-usuarios.md § 3`, interface sobrecarregada.
- **Não copiar.** Não copiar barra ou tela de modo leitura. Não recriar auto-hide de HUD como função nova. Não transformar foco em modo que esconda avisos críticos.
- **Mission gate.** Passa se o incremento ao HUD existente produzir ganho de leitura observável sem prender o usuário.
- **Situação atual no Tsuro.** O snapshot registra HUD auto-hide. Âncoras não revalidadas são `crates/tsuro/src/view.rs:202`, `crates/tsuro/src/view.rs:466` e `crates/tsuro/src/view.rs:2434`. Incremento proposto, reduzir chrome persistente selecionado e oferecer retorno explícito. Quais controles ainda são persistentes precisa de revalidação.
- **História de usuário.** Como leitor concentrado, quero reduzir controles para acompanhar o documento sem perder meios de navegar ou sair.
- **Critérios de aceite.** Verificar os seguintes comportamentos.
  - Dado o modo ativado, Quando entra ou sai, Então mantém âncora de leitura e não muda documento ou seleção inadvertidamente.
  - Dado modo ativo sem diálogo prioritário, Quando usa Escape, Então restaura chrome e foco de forma previsível.
  - Dado pedido de senha, aviso de salvamento ou erro crítico, Quando ele ocorre, Então permanece visível e acessível mesmo no modo foco.
  - Dado controle temporariamente revelado, Quando ele recebe foco por teclado, Então não desaparece enquanto é usado.
  - Dado falha ou impossibilidade de restaurar geometria, Quando sai do modo, Então mantém acesso aos controles e usa fallback seguro sem deixar foco preso.
  - Dado nenhuma preferência persistida, Quando reabre a sessão, Então inicia no modo normal desta proposta e não herda estado oculto inesperado.
- **Experiência no Kiri.** Comando na paleta, saída por teclado e retorno por região previsível. Proposta inicial não persiste foco entre sessões. Prioridade de Escape respeita diálogo ou seleção ativa antes do modo. Estado escondido não é estado indisponível. Não adicionar nova barra ou botão permanente.
- **Expressão temática.** Névoa é codinome em português, sem efeito de névoa sobre a página. Reusar Ori existente. PT "Modo foco" e "Sair do modo foco". EN "Focus mode" e "Exit focus mode". Movimento apenas para revelar estado, respeitando redução de movimento.
- **Dados e código.** Propor modo normal e foco com restauração dos controles anteriores. Revalidar necessidade de estado intermediário antes de adotar três modos. `session.rs` guarda estado e foco de retorno, `view.rs` compõe chrome. Área útil pode mudar, portanto preservar âncora e medir renderização. Não mudar Pdfium, cache ou workers sem necessidade demonstrada.
- **Testes.** Entrada e saída por teclado, alertas, diálogo, seleção, abas, zoom, rotação, área útil e janela pequena. Testar erro de restauração e foco de controle temporário. Comparar custo com S-06. Avaliar ganho além do HUD já existente.
- **Riscos e premissas.** Ocultar controle pode retirar ação necessária. Retorno explícito, foco estável e avisos prioritários. Mudança de área útil pode mudar zoom ou renderizar novamente. Preservar âncora e medir. Suposição, chrome persistente atual justifica modo adicional.
- **Dependências.** Revalidar ganho em relação ao HUD e orçamento de S-06 são rígidos. F-NAV-04 melhora descoberta sem bloquear comando na paleta atual. F-AMB-03 é independente. Nenhuma dependência de Washi.
- **Tamanho e confiança.** M, confiança média. Incógnitas de área útil, foco, Escape e ganho além do auto-hide.
- **Prioridade sugerida.** Could. Valor 3, urgência 2, oportunidade 2, tamanho 2. WSJF = (3+2+2)/2 = 3,5.
- **Definição de pronto.** Ganho e saída por teclado demonstrados, alertas visíveis, âncora preservada e custo documentado. PT e EN, nota de release, `cargo test -p tsuro`, `cargo fmt --check`, `cargo clippy`, macOS e Windows. Não aprovar só por screenshot sem verificar foco e geometria.
- **Rascunho de issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(ui): reduzir chrome com retorno seguro no modo foco`
  - Labels: `enhancement`, `ux`, `accessibility`, `needs-triage`
  - Corpo: Demonstrar incremento além do HUD auto-hide. Preservar âncora, controles focados e alertas. Oferecer saída por teclado. Começar sem persistência de modo oculto e medir recálculo.

### F-AMB-03. Régua de leitura (Ito)

- **Ficha.** curta
- **Promessa.** O leitor acompanha uma linha com guia visual opcional sem ocultar ou alterar o texto.
- **Origem (evidência).** `ui-ux.md § 5.8`, marcas de leitura em Sioyek, Skim e Edge. `relatorio.md § 5.5`, conforto opcional.
- **Não copiar.** Não copiar faixa ou interação de outro leitor. Não anunciar benefício clínico ou acessibilidade comprovada sem avaliação.
- **Mission gate.** Ajuste. Passa se guia demonstrar ajuda e não criar barreiras de teclado, contraste ou geometria.
- **Situação atual no Tsuro.** Geometria e faixa de leitura não foram revalidadas. A camada de texto do snapshot é ponto de descoberta, não prova de linha correta em todo layout.
- **Hipótese e como validar.** S-04 avalia linhas e ordem em colunas, tabelas e rotação. Comparar guia ligado e desligado em tarefas de leitura e retomada. Prototipar movimento por teclado, zoom e saída. Não confundir seleção por teclado com guia visual.
- **Critério de saída.** Faixa acompanha linha definida, mantém contraste e não bloqueia seleção, links ou foco. Funciona nos layouts declarados e informa limites. Desligada por padrão e não aparece em cópia ou impressão. Se geometria ou ganho forem insuficientes, adiar ou recusar.
- **Dependências.** S-04 favorável e S-06 para custo. F-AMB-02, F-NAV-04 e F-NAV-09 são opcionais, sem bloqueio circular.
- **Tamanho e prioridade.** G, Could, confiança baixa. Incógnitas de geometria, ganho e mecanismo acessível.
- **Expressão temática.** Ito, 糸, fio, codinome interno. Ori candidato com dois polígonos finos e vinco central, se ícone realmente necessário. PT "Régua de leitura". EN "Reading guide". Sem animação decorativa.
- **Rascunho de issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `feat(read): avaliar guia opcional de linha`
  - Labels: `enhancement`, `accessibility`, `ux`, `needs-triage`
  - Corpo: Validar geometria em S-04 e ganho de leitura. Guia desligado por padrão, acessível por teclado e ausente de saídas. Não bloquear seleção e links nem prometer efeito clínico.

### F-AMB-04. Anti-funções do Tsuro (Pacto do Ma)

- **Ficha.** pacto
- **Propósito.** Registrar limites verificáveis para que melhorias não transformem o leitor leve numa suíte. As cláusulas abaixo são propostas de governança, não novos compromissos automaticamente aprovados.
- **Cláusulas.** As evidências fundamentam os limites, mas uma referência não prova que o Tsuro já os cumpre.

  | ID | O Tsuro nunca... | Evidência | Como verificar |
  | --- | --- | --- | --- |
  | P-01 | Exige conta ou upload para leitura e marcação locais | `relatorio.md § 6` | Testar tarefas locais offline e revisar chamadas de rede do fluxo |
  | P-02 | Acrescenta edição estrutural sob o nome de leitura | `relatorio.md § 6` | Revisar comandos e escopo contra a missão |
  | P-03 | Converte assinatura local em serviço remoto sem mudança de escopo aprovada | `relatorio.md § 6` | Revisar desenho e dependências de rede, sem confundir assinatura existente com serviço novo |
  | P-04 | Muda o leitor padrão sem ação explícita | `codigo-e-usuarios.md § 3` | Testar configuração, instalação e atualização quando fizerem parte da release |
  | P-05 | Acrescenta marca d'água do produto a cópias ou impressão | `codigo-e-usuarios.md § 3` | Comparar saídas com original, distinguindo marcas do próprio documento |
  | P-06 | Cria interface legada paralela como atalho de implementação | `ui-ux.md § 7` e `AGENTS.md` | Revisar diff e destino das mudanças, mantendo limites do legado |
  | P-07 | Acrescenta chat ou resumo remoto dentro do leitor por conveniência | `ui-ux.md § 7` | Revisar comandos, dependências e escopo, registrando mudança se houver pedido futuro |
  | P-08 | Exige arquivo de configuração manual para tarefa básica | `ui-ux.md § 7` | Executar abrir, ler, marcar e imprimir numa configuração nova |
  | P-09 | Inclui caminho absoluto pessoal em exportação sem escolha explícita | `relatorio.md § 5.4` | Inspecionar exportações com caminhos pessoais controlados |
  | P-10 | Afirma validade jurídica ou verificação de assinatura que não executou | `relatorio.md § 5.7` | Conferir cada texto contra dados e verificação do painel |
  | P-11 | Persiste senhas em logs, preferências ou recuperação | `codigo-e-usuarios.md § 4.2` | Revisar fluxo e inspecionar armazenamento com segredo de teste |
  | P-12 | Muda cores do original ou da saída para aplicar tema de leitura | `relatorio.md § 5.5` | Comparar original, cópia e saída preparada. Washi futuro não é exceção automática |

- **Como verificar a cada release.** Triagem de aproximadamente dois minutos, não auditoria completa. Revisar comandos novos contra missão. Conferir a matriz de evidências da release para rede, leitor padrão, exportação, senha e assinatura. Marcar cada cláusula como evidência disponível, não aplicável justificada ou pendente. Se faltar evidência de uma cláusula afetada, abrir pendência antes de aprovar. Testes de impressão física, privacidade e instalação seguem roteiro próprio e não cabem nessa triagem. Não salvar sobre fixtures nem tomar ausência de item na paleta como prova de ausência de código.
- **Origem (evidência).** `codigo-e-usuarios.md § 3`, interface sobrecarregada e saídas inesperadas. `ui-ux.md § 7`, comportamentos a não trazer. `relatorio.md § 6`, fronteiras da missão.
- **Expressão temática.** Pacto do Ma. Ma, 間, espaço ou intervalo, codinome sujeito a revisão. Tom direto, sem manifesto ornamental. Nenhum ícone novo. PT "Limites do Tsuro". EN "Tsuro boundaries".
- **Rascunho de issue.** Publicação bilíngue e estado técnico no [índice de issues](../issues-publicadas.md). O texto publicado incorpora a avaliação técnica posterior.
  - Título: `docs(product): propor limites verificáveis do Tsuro`
  - Labels: `enhancement`, `documentation`, `needs-triage`
  - Corpo: Aprovar ou ajustar cláusulas antes de incorporá-las à governança. Triagem de release consulta evidências, não promete provar tudo em dois minutos. Preservar leitura local, saída sem marca do produto e limites do Kiri.

## Decisões e contratos a resolver antes de executar

O usuário aprova políticas que reduzem proteção de uma saída, mudanças de esquema, dependências, modificações de assinatura e mudanças de princípio do Kiri. Esta entrega não solicita aprovação antecipada de tudo. Cada implementação futura apresenta o incremento concreto e as evidências para a decisão correspondente.

S-05 precede compromisso de senha e permissões. S-01 precede compromisso de salvar assinados. S-03 precede mudança do caminho de impressão, mas não bloqueia omitir marcações da sessão. S-04 informa régua. S-08 continua futuro e requer autorização para explorar. S-06 fornece baseline e orçamento, ainda não medidos.

O coordenador corrigiu o rascunho para não garantir assinatura por append, não confundir anotações padrão com conteúdo achatado e não inferir impressão física de estado do spool. Também restringiu promessas de descarte de senha ao que o Tsuro controla e separou triagem rápida de auditoria de privacidade. Não há resultado de spike, benchmark ou teste do aplicativo nesta entrega.
