# Investigação F-MARC-01 (#128): diário de recuperação de marcações

Estado: implementado na unidade 2 (`feat/recovery-session`, #128), sobre o módulo de persistência da unidade 1 (PR #159). As seções 2 a 4 valem como foram implementadas, com os desvios e as decisões registrados na seção 6. Base da investigação: `main` em `af001dc` (21/out/2026), leitura estática de `crates/tsuro/src/{session,engine,positions,prefs,browse}.rs`. Verificação nativa em macOS e Windows ainda não foi feita (seção 6).

## 1. O que existe hoje

As marcações vivem só em memória, em `Ready.annotations: Vec<Annotation>` (`session.rs:1683`, comentário "Sem persistência na v1"). Cada aba tem a sua lista. Um crash, `kill -9`, queda de energia ou panic perde tudo o que não foi salvo como cópia.

- `Annotation` guarda `id: u64`, `page`, `range` (bytes no texto do Pdfium), `quads` (geometria em espaço da mídia), `kind`, `text` e `marker`. Não há campo de cor: a cor sai do `kind`.
- `next_annot_id` começa em 0 a cada abertura e a cada recarga (`session.rs:6639`, `apply_reload` em `:6287` faz `*self = fresh`).
- Editar nota cria ID novo. `save_note_draft` (`:5816`) faz `Remove(antiga)` + `Add(id novo)`, e o teste `note_edit_undo_restores_old_text` exige `assert_ne!` no ID. Mover o marcador mantém o ID.
- Toda mutação passa por `apply_annot_action` (`:6062`). `Add` só insere se o ID não existe, `Remove` filtra pelo ID.
- `marks_dirty()` compara `annotations != saved_marks` (`:5351`), com ordem. `saved_marks` só muda quando a cópia anotada é salva.
- Identidade do arquivo: caminho sem canonicalizar mais `(tamanho, mtime_ns)` (`positions::file_identity`). Ninguém calcula hash do conteúdo. Os bytes do PDF ficam em memória (`LiveDoc.bytes: Arc<[u8]>`, `engine.rs:168`).
- Mudança externa: `FileTick` a cada 2 s compara a identidade. Com marcações não salvas só marca `disk_stale`. `ReloadDisk` descarta as marcações sem conferir nada no handler.
- Persistência existente (`positions`, `prefs`, `recents`): `std::fs::write` direto, sem arquivo temporário, sem rename, sem fsync, sem lock e com leitura cega (erro vira lista vazia). Por isso a issue proíbe reaproveitar `positions` como diário.
- Dependências do crate `tsuro`: sem serde, sem hash, sem tempfile, sem lock. `sha2` e `serde` já estão no lockfile via `tsuro-sign`, mas usar no `tsuro` conta como dependência nova.
- Duas abas podem abrir o mesmo arquivo (`PickFile` e `FileDropped` não procuram aba existente).

## 2. Proposta

### 2.1 Checkpoint é um snapshot completo, não um log de operações

Cada checkpoint grava o conjunto inteiro de marcações da aba. Restaurar substitui o conjunto. Com isso:

- Restaurar duas vezes dá o mesmo resultado (critério 2) sem precisar de números de sequência nem de deduplicação de operações.
- Não existe "sequência inválida" para validar (critério 4): o arquivo ou está íntegro ou é rejeitado inteiro.
- O tamanho é pequeno. Uma marcação ocupa algumas dezenas de bytes mais o texto da nota.

O log de operações só compensaria com milhares de marcações por documento, o que o produto não tem.

### 2.2 Um ponto de checkpoint, como `saved_marks`

`Ready` ganha `journaled: Vec<Annotation>`, o último conjunto gravado com sucesso. Depois de cada `update`, se `encode(annotations)` difere do último texto gravado, agenda um checkpoint com atraso `D`. Comparar o texto codificado, e não `Vec<Annotation>` com `PartialEq`, evita dois erros de `f32`: um NaN nunca seria igual a si mesmo e regravaria para sempre, e `-0.0 == 0.0` esconderia uma mudança. Isso:

- cobre criar, editar, apagar, mover, desfazer e refazer sem tocar nos seis métodos de mutação;
- nunca grava o estado intermediário de uma edição (`Remove` + `Add` acontecem dentro do mesmo handler);
- reaproveita o padrão que `marks_dirty` já usa.

A fronteira de transação é "fim do handler", não o gesto. Desfazer uma edição continua exigindo dois Cmd+Z (comportamento atual, fora do escopo).

### 2.3 IDs estáveis

Editar nota passa a manter o ID: `id: previous.map(|o| o.id).unwrap_or(next)`. Ao restaurar, `next_annot_id = max(id) + 1`. O ID só precisa ser estável dentro da linhagem de um diário. Isso também serve ao painel do F-MARC-02 (#129), que precisa de referência estável por linha. Muda um teste existente (`note_edit_undo_restores_old_text`).

### 2.4 Identidade do conteúdo

Hash do conteúdo calculado sobre os bytes que já estão em memória na abertura, numa `spawn_blocking`, fora do caminho crítico (exigência do F-NAV-05). O diário guarda hash, tamanho, número de páginas e o caminho só para exibição.

- Hash igual: oferecer restauração.
- Hash diferente no mesmo caminho: estado `Incompatível`. Não aplica, preserva o diário e explica (critério 3).
- Cópias idênticas em caminhos diferentes: oferecer, mas mostrar o caminho original. Nunca mesclar automaticamente (D-15).

### 2.5 Formato e gravação

Arquivo de texto versionado, sem dependência nova:

```
tsuro-recovery 1
content <hash-hex> <tamanho> <páginas>
path <caminho escapado como em positions>
annot <id> <page> <kind> <start> <end> <marker|-> <quads em f32::to_bits hex> <texto escapado>
...
end <quantidade> <checksum-hex>
```

- Os floats vão como bits em hexadecimal para a geometria voltar exata.
- A linha `end` com quantidade e checksum detecta truncamento e corrupção. Sem ela, o arquivo é rejeitado inteiro.
- Gravação: escrever `<id>.tmp`, `sync_all`, renomear `<id>.journal` para `<id>.prev`, renomear `.tmp` para `<id>.journal` e fsync do diretório no Unix. Leitura tenta `.journal` e cai para `.prev`. Assim o último checkpoint validado sempre existe (critério 4), inclusive se o processo morrer entre os dois renames.
- Isso cobre crash do processo e queda de energia em sistemas de arquivos que honram fsync.
- O leitor separa linhas só por `\n`. O escape não cobre `\r`, e `str::lines()` cortaria um `\r` final do texto da nota sem o checksum perceber.

### 2.6 Concorrência: cada aba escreve só no seu arquivo

Cada aba aberta cria seu próprio diário, `recovery/<instância>.journal`, em que `<instância>` é aleatória (pid + nanos). Nenhum arquivo tem dois escritores, então não há escrita concorrente para serializar.

A aba segura um lock exclusivo num arquivo lateral `<instância>.lock` enquanto vive (`std::fs::File::try_lock`, estável desde Rust 1.89, sem dependência nova). O lock fica no `.lock`, não no `.journal`, porque o rename troca o inode do diário. O sistema operacional solta o lock quando o processo morre. Na abertura de um PDF:

1. listar os diários em `recovery/`;
2. ignorar os que têm `.lock` ainda preso (outra aba ou outro processo vivo);
3. dos restantes, oferecer os de hash igual.

Restaurar copia as marcações para a aba atual, que passa a gravar no próprio diário. O diário antigo só é apagado depois de o novo ter o primeiro checkpoint confirmado. Uma falha no meio deixa os dois arquivos, nunca nenhum. `adopt` recusa se a aba ainda não tem checkpoint próprio. Se o lock do diário antigo estiver ocupado por um instante (a sonda de outro processo), `adopt` devolve `WouldBlock` e a sessão tenta de novo depois, sem tratar como erro.

O `Journal` só é criado no primeiro checkpoint. Aba sem marcações nunca deixa `.lock` no disco.

### 2.7 Ciclo de vida

| Evento | Diário |
|---|---|
| Mudança nas marcações | checkpoint após `D` |
| Falha de escrita (disco cheio, permissão) | mantém memória, estado `Falhou`, aviso "recuperação não garantida", tenta de novo na próxima mudança (critério 5) |
| Salvar cópia anotada | não apaga, não grava nada na cópia (critério 6) |
| Exportar Markdown ou copiar | não apaga |
| Fechar com "Descartar" confirmado | apaga |
| Fechar sem marcações | nada a apagar (o diário nunca foi criado) |
| Fechar limpo depois de salvar cópia | **decisão D-B abaixo** |
| `ReloadDisk` | mantém até o usuário descartar |
| Crash | fica no disco, oferecido na próxima abertura |
| PDF com senha | não cria diário na v1 (D-14) |

Nunca apagar por idade, por limite de recentes ou por exportação.

### 2.8 Onde o código entra

- `crates/tsuro/src/recovery.rs` (`pub(crate)`) cuida de caminho, formato, gravação atômica, lock e leitura. Segue o padrão de `positions.rs`, com override `thread_local!` para os testes.
- `session.rs`: `journaled`, `RecoveryState { Nenhum, Disponível, Restaurando, Incompatível, Falhou }`, agendamento do checkpoint e mensagens Restaurar, Descartar e Decidir depois.
- `view.rs`: cartão Kiri com os textos PT/EN já definidos em `funcoes/01-marcacoes.md`.

## 3. Decisões para o Junior

| # | Decisão | Recomendação | Alternativa |
|---|---|---|---|
| D-A | Aprovar o novo esquema persistido (seção 2.5) | aprovar | log de operações (mais código, sem ganho no tamanho atual) |
| D-B | Fechar limpo depois de salvar cópia apaga o diário? | apagar: as marcações já estão na cópia | manter, com regra de retenção explícita e "esquecer" na interface |
| D-C | Hash do conteúdo | `sha2` no `tsuro` (já no lockfile, exige aprovação de dependência) | FNV-1a 64 escrito à mão: sem dependência, suficiente contra mudança acidental, não contra adulteração |
| D-D | Atraso `D` do checkpoint | medir (S-06) com 500 ms e 2 s em PDF grande e decidir pelo número | fixar agora sem medição (a roadmap pede para não assumir os 5 s antigos) |
| D-E | Editar nota mantém o ID | sim (muda um teste) | manter o ID novo e criar outro identificador só para o diário |
| D-F | Rascunho de nota em digitação entra no diário? | não na v1 | sim, com gancho em `NoteEdit` |
| D-G | Pasta no Windows | `%LOCALAPPDATA%\TsuroPDF\recovery`, como o updater | `%USERPROFILE%\.local\share\tsuro`, como positions e prefs |
| D-H | Privacidade | permissão 0600 no Unix, nada além do caminho e das notas, opção "esquecer diários" | só documentar |

## 4. Plano de teste (para a implementação)

Cada critério da issue vira um teste em `recovery.rs` ou `session.rs`, sobre cópias das fixtures em diretório temporário:

1. Três marcações com checkpoint, descartar o `Ready` sem fechar, reabrir: a restauração oferece exatamente as três, e o hash do PDF original continua igual.
2. Restaurar duas vezes dá o mesmo conjunto.
3. Mudar um byte do PDF: estado `Incompatível` e diário intacto.
4. Truncar o `.journal` em cada byte: cai para `.prev` ou para "nada a oferecer", nunca aplica parcial.
5. Diretório sem permissão de escrita (substituto de disco cheio): estado `Falhou` e marcações em memória intactas.
6. Salvar a cópia: o diário do original continua lá e a cópia abre sem oferta de restauração.

Mais um teste de dois processos com o lock e a medição de `D`. Verificação nativa em macOS e Windows antes da entrega.

## 5. Riscos conhecidos

- `range` depende da extração de texto do Pdfium. Uma versão nova do Pdfium pode deslocar offsets. A geometria (`quads`) continua certa, e só a citação do Markdown sofreria. Mitigação: guardar o hash do texto da página no checkpoint (opcional).
- O cmd+Q do menu nativo do macOS pode não passar por `CloseRequested`. Não verificado. Se pular a confirmação, o diário cobre o caso, que é justamente o objetivo.
- `ReloadDisk` não confere marcações não salvas no handler. Não verifiquei se a interface bloqueia o botão. Fora do escopo, mas vale uma issue se confirmar.

## 6. Implementação (unidade 2)

### Decisões

| # | Resultado |
|---|---|
| D-A | Esquema da seção 2.5 implementado (`tsuro-recovery 1`), com validação de tudo ou nada. |
| D-B | Fechar limpo, fechar após salvar cópia, ou descartar apaga o diário da aba. Encerrar a janela também, por `CloseWindow`. |
| D-C | FNV-1a 64. A identidade do conteúdo também inclui tamanho e número de páginas. |
| D-D | Atraso de **500 ms**, o menor valor da faixa, pela medição abaixo. |
| D-E | Editar nota mantém o ID. Já estava em `main`; o teste `note_edit_undo_restores_old_text` cobre. |
| D-F | Rascunho de nota em digitação não entra no diário. |
| D-G | Windows: `%LOCALAPPDATA%\TsuroPDF\recovery`. macOS: `~/Library/Application Support/Tsuro/recovery`. |
| D-H | Pasta `0700`, arquivos `0600`. Configurações → Recuperação → "Esquecer diários de recuperação…", com confirmação. Remove só diários sem dono vivo. |

### Medição de D (D-D)

Comando: `cargo test -p tsuro checkpoint_cost_by_size -- --ignored --nocapture`. Build de depuração, macOS arm64, APFS. Cada marcação tem uma nota de cerca de 120 bytes e um quad. O tempo inclui `sync_all` e o rename. Linha de base: nenhuma, esta é a primeira medição do custo do checkpoint.

| Marcações | Tamanho | Encode | Encode + gravação |
|---|---|---|---|
| 10 | 2,3 KB | 0,04 ms | 5,0 ms |
| 100 | 22 KB | 0,5 ms | 6,2 ms |
| 1000 | 223 KB | 4,3 ms | 14,2 ms |

O custo fica muito abaixo de qualquer valor da faixa, e roda fora da UI. Por isso o menor atraso (500 ms) é o que reduz a janela de perda. Build de release não foi medido.

### Desvios do texto das seções 2 a 4

- A comparação de mudança é bit a bit (`same_marks`), não a string codificada. O critério é o mesmo (NaN igual a si mesmo, `-0.0` diferente de `0.0`), sem alocar a cada mensagem.
- O agendamento arma um timer por mudança pendente e não o reinicia a cada mensagem. Hover ou arrasto contínuo não adia o checkpoint indefinidamente.
- `Falhou` é um sinalizador separado do cartão. Uma falha de gravação não esconde uma oferta de restauração.
- `Restaurando` guarda a adoção pendente. O diário antigo sai só no primeiro checkpoint próprio gravado.
- O cartão é modal, como o de assinatura. O fundo equivale a "Decidir depois". "Descartar" pede confirmação.
- Diário sem marcações não é oferecido. Apagar todas as marcações e gravar deixa um checkpoint vazio, que também não é oferecido.
- Com mais de um diário do mesmo conteúdo, restaura-se o mais recente. Os outros ficam no disco e voltam na próxima abertura.
- Restaurar substitui o conjunto em memória, inclusive marcações feitas com o cartão aberto, e limpa desfazer e refazer.
- Recarga do disco (`ReloadDisk` e o auto-reload) não varre diários. O diário da memória anterior fica no disco; na próxima abertura, se o arquivo mudou, aparece como incompatível.
- Não há navegação por teclado no cartão.

### Limites conhecidos

- Um checkpoint que termina depois de a aba fechar com descarte pode deixar um diário. Ele volta como oferta na próxima abertura. A janela é de milissegundos e não está tratada.
- Durabilidade contra queda de energia depende do `fsync` do sistema. No macOS, `sync_all` não usa `F_FULLFSYNC`.
- PDF com senha: o app não abre PDF com senha (`browse.rs` trata a falha como "não abriu"), então não há diário. Não testado com PDF cifrado real.
- Linux usa o caminho XDG; sem verificação nativa.

### Testes

Os testes usam cópias do fixture em pasta temporária, nunca os fixtures no lugar, e um diretório de recuperação próprio por teste.

| Critério da issue / seção 4 | Teste |
|---|---|
| Critério 1 / §4.1 | `three_marks_survive_a_crash_and_restore_offers_exactly_them` |
| Critério 2 / §4.2 | `restoring_twice_gives_the_same_set_without_duplicates` |
| Critério 3 / §4.3 | `changed_pdf_is_incompatible_and_keeps_the_journal` |
| Critério 4 / §4.4 | `truncated_journal_falls_back_at_every_byte`, `every_truncation_is_rejected`, `every_flipped_byte_is_rejected`, `corrupt_journal_falls_back_to_prev` |
| Critério 5 / §4.5 | `failed_write_keeps_memory_warns_and_retries_on_the_next_change` (só Unix) |
| Critério 6 / §4.6 | `saving_a_copy_keeps_the_original_journal_and_the_copy_offers_nothing` |
| Dois processos | `live_owner_in_another_process_is_skipped_until_it_exits` (processo filho real) |
| Ciclo de vida | `closing_after_a_saved_copy_retires_the_journal`, `discarding_on_close_deletes_the_journal`, `reload_from_disk_keeps_the_journal_on_disk`, `decide_later_keeps_the_journal_for_the_next_open`, `discard_asks_for_confirmation_and_only_then_deletes`, `forget_removes_unowned_journals_from_settings`, `journal_appears_only_with_the_first_checkpoint`, `removing_every_mark_leaves_nothing_to_offer`, `same_content_at_another_path_is_offered_with_its_origin`, `hud_wake_arms_one_checkpoint_timer_per_pending_change` |
| Unidade | `same_marks_compares_bits_not_float_equality`, `abandoned_lists_the_newest_first`, `retired_journal_refuses_writes`, `forget_removes_only_unowned_instances`, `adopt_removes_old_instance_after_own_checkpoint` |

### Verificação nativa pendente

- macOS: matar o app (`kill -9`) com marcações, reabrir e conferir o cartão em PT e EN, o layout e cada botão. Descartar com confirmação. Falha real de gravação (pasta sem escrita). Aviso de falha na faixa do documento. Configurações → Esquecer.
- Windows: pasta `%LOCALAPPDATA%\TsuroPDF\recovery`. Remoção do `.lock` depois de fechar o handle (no NTFS um arquivo aberto não é apagado). Dois processos com `LockFileEx`. Permissões: `0600` e `0700` não se aplicam, a pasta herda ACL do usuário.
- Cenários de Pdfium (ranges de texto) reabrindo marcações de texto real, não só o fixture.
