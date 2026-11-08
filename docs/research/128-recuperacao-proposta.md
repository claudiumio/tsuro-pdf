# #128 Recuperar marcações de sessões interrompidas: proposta de contrato

Investigação de 2026-10-13 sobre `main` `1001a57`. Nenhum código alterado.

## Resumo

O código atual permite uma recuperação mais simples do que a issue presume. A issue fala em diário de operações, IDs estáveis e sequência de transações. Mas o estado das marcações já é um conjunto completo e pequeno (`Ready.annotations: Vec<Annotation>`), e toda mudança passa por um único ponto (`apply_annot_action`). Um **checkpoint do conjunto inteiro**, gravado de forma atômica, cumpre os seis critérios sem log de operações. Restaurar é substituir o conjunto, o que já é idempotente por construção. Os IDs só precisam ser únicos dentro do checkpoint. Tamanho cai de G para M. Faltam quatro decisões suas (no fim).

## Fatos do código

| Fato | Onde |
|---|---|
| As marcações da aba são um `Vec<Annotation>` com `id`, `page`, `range`, `quads`, `kind`, `text`, `marker` | `session.rs` `struct Annotation` |
| Toda mudança vira `AnnotAction::Add`/`Remove` aplicada em `apply_annot_action`. Editar nota e mover marcador são `Remove` + `Add` (a edição troca o ID) | `session.rs` `apply_annot_action`, `save_note_draft`, `move_note_marker` |
| Já existe "sujo" = `annotations != saved_marks` | `session.rs` `marks_dirty` |
| Já existe um tique de 2 s por aba (`FileTick`, `RELOAD_POLL`) que compara a identidade do arquivo | `session.rs` `Message::FileTick`, `const RELOAD_POLL` |
| A identidade atual do arquivo é tamanho + mtime em ns | `positions.rs` `file_identity` |
| O arquivo de posições é reescrito direto com `std::fs::write`, sem temporário nem `fsync`, e limitado a 50 entradas | `positions.rs` `save_positions`, `MAX_POSITIONS` |
| Não há `serde` nem hash nas dependências do `tsuro` | `crates/tsuro/Cargo.toml` |

## Proposta

**Dado.** `RecoveryCheckpoint { version, doc: DocKey, content: ContentId, saved_at, annotations: Vec<Annotation> }`.
- `DocKey` é o caminho canônico do PDF. Duas cópias idênticas em lugares diferentes não se misturam (D-15).
- `ContentId` é tamanho + FNV-1a 64 dos bytes do PDF, calculado na abertura. Não usa mtime, porque copiar ou tocar o arquivo não muda o conteúdo. FNV tem umas 10 linhas e não precisa de dependência nova. Não é segurança, só identidade.
- Formato de texto próprio, versionado, uma marcação por linha, no estilo de `positions.rs`. A última linha é `end <n>` com a contagem, para detectar truncamento.

**Onde fica.** `…/Tsuro/recovery/<fnv do caminho>.marks`, ao lado de `positions`, num módulo novo `recovery.rs`. Fica fora de `positions`, que é escrito sem atomicidade e corta em 50 (a issue proíbe reaproveitá-lo).

**Quando grava.** No `FileTick` que já existe: se `annotations` mudou desde o último checkpoint da aba, grava. O atraso fica em 2 s ou menos, abaixo da meta de 5 s da ficha. Gravação atômica: arquivo temporário na mesma pasta, `sync_all`, `rename` por cima, e `sync` da pasta no macOS. Roda em `spawn_blocking`, sem travar a UI. Conjunto vazio grava um checkpoint vazio, que é diferente de "sem checkpoint" (apagar tudo também é trabalho).

**Quando oferece.** Ao abrir um PDF, se existe checkpoint para o `DocKey`:

| Situação | Comportamento |
|---|---|
| `ContentId` igual e checkpoint diferente das marcações abertas (hoje sempre vazias) | Cartão "Marcações recuperáveis encontradas (N)." com Restaurar, Descartar (pede confirmação) e Decidir depois |
| `ContentId` diferente | Não aplica. Cartão "O arquivo mudou desde a última sessão. A recuperação foi guardada." O checkpoint fica |
| Checkpoint ilegível (versão desconhecida, linha inválida, `end` faltando ou com contagem errada) | Não aplica nada. Abre o PDF normalmente. O arquivo ruim vira `*.marks.bad` e fica guardado |
| Gravação falha (disco cheio, permissão) | As marcações em memória seguem intactas. Aviso "Não foi possível guardar a recuperação." até a próxima gravação dar certo |

**Restaurar.** Substitui `annotations` pelo conjunto do checkpoint, ajusta `next_annot_id` para `max(id) + 1` e limpa as pilhas de desfazer e refazer. Restaurar de novo produz o mesmo conjunto, então nada se duplica.

**Salvar cópia.** Não apaga nem altera o checkpoint do original. A cópia nova tem outro `DocKey`, então nunca recebe as marcações do original em dobro.

## Como cada critério da issue é atendido

| Critério | Resposta | Teste |
|---|---|---|
| 3 marcações, processo encerrado, reabrir: oferece exatamente 3, hash do PDF igual | Checkpoint no tique; oferta na abertura; o PDF só é lido | Gravar checkpoint, simular nova sessão, comparar conjunto e bytes do PDF |
| Restaurar de novo não duplica | Restaurar substitui o conjunto | Restaurar duas vezes, comparar |
| Arquivo alterado bloqueia, preserva e explica | `ContentId` diferente vira "incompatível" | Alterar bytes de uma cópia da fixture |
| Diário truncado ou inválido não aplica nada | Linha `end <n>` e versão; o arquivo ruim é guardado | Cortar o arquivo no meio, linha inválida |
| Disco cheio mantém a memória e avisa | Erro de gravação vira aviso; nada é apagado | Pasta de recuperação sem permissão de escrita |
| Cópia salva não apaga o diário nem duplica | `DocKey` por caminho | Salvar cópia e conferir os dois checkpoints |

## Riscos

- **Privacidade.** O checkpoint guarda o texto das notas em claro, na pasta de dados do usuário, com permissão `0600` no macOS. PDF com senha (#145) não recebe checkpoint em claro sem uma política sua.
- **PDF muito grande.** Calcular o FNV de um PDF de 200 MB leva uns 200 ms [INFERENCE]. Roda na worker, junto da abertura, e precisa ser medido.
- **Duas instâncias do Tsuro com o mesmo PDF.** A última gravação vence. É aceitável na v1, e o `rename` atômico evita arquivo corrompido.
- **Arquivo movido.** Muda o `DocKey`, então não oferece a recuperação. Isso é o #140 (adiado).

## Decisões para o usuário

1. **Retenção.** Quando apagar um checkpoint? Proposta: só quando você descartar, ou quando fechar sem marcações sujas (tudo salvo ou nada marcado). Nunca por idade, por limite de recentes ou só por salvar cópia.
2. **Privacidade.** Aceita notas em claro na pasta de dados do usuário, com permissão restrita? E PDFs com senha ficam fora da recuperação na v1?
3. **Ao abrir.** O cartão aparece sozinho (proposta) ou só um aviso discreto na barra?
4. **Esquema persistido.** Aprova o arquivo novo `recovery/*.marks` (a issue exige aprovação para esquema persistido)?

## Fatia de implementação (próxima sprint, depois das decisões)

1. `recovery.rs`: formato, leitura, gravação atômica, FNV, com testes de truncamento e falha de gravação.
2. Checkpoint no `FileTick` e aviso de falha.
3. Oferta na abertura com cartão e teclado, PT e EN.
4. Teste ponta a ponta com cópia de `guia-folio.pdf` e conferência manual no macOS e no Windows.
