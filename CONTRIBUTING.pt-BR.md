<p align="right">🇺🇸 <a href="CONTRIBUTING.md">English</a></p>

# Contribuindo com o TsuroPDF

Issues, PRs e commits em **inglês ou PT-BR** — os dois valem igual. PRs pequenos, um assunto por vez.

## Missão

TsuroPDF é um leitor de PDF rápido e leve: ler, marcar, imprimir. Nada além disso. Antes de propor qualquer coisa, pergunte: isso deixa ler, marcar ou imprimir mais rápido, mais leve ou mais claro? Fora do escopo, de propósito: formulários, nuvem, colaboração, suítes. Proposta fora da missão é recusada ou adiada.

## O que queremos primeiro

Quando duas propostas disputam o tempo de revisão, esta é a ordem:

1. **Bugs** — trava, corrompe, lê errado. Reproduza com um PDF de exemplo antes; correção com teste que falhava antes e passa agora entra rápido.
2. **UI/UX** — a superfície de leitura, o chrome, os painéis, os atalhos, o contraste, o layout. Bugs de gosto e clareza também contam aqui: controle desalinhado, dica ilegível, foco quebrado.
3. **Melhorias** — fazer melhor o que já funciona: render mais rápido, menos memória, defaults melhores, copy mais enxuta, lacunas de i18n.
4. **Recursos novos** — capacidade realmente nova. Prioridade mais baixa e portão mais rígido: passa pela missão, raio de explosão pequeno, nenhuma dependência nova sem discussão na issue antes.

Se você vai escolher um trabalho, escolha do topo. Se você vai propor, essa ordem também orienta as labels de triagem: bug é triado antes de feature.

## Issues

Veja as issues abertas antes — duplicada é fechada. Um bom relato tem:

- Versão do TsuroPDF (tag da release, ex. `v0.6.0`, ou hash do commit)
- Sistema (macOS Apple Silicon, Windows x64)
- Passos para reproduzir, comportamento esperado vs. obtido
- Um PDF de exemplo: anexe um, ou indique um arquivo em `public/samples/`
- Para relatos de assinatura: o estado mostrado no painel de assinaturas
- Em qual faixa de prioridade entra (bug, UI/UX, melhoria, feature)

Issues e PRs levam corpo bilíngue: `## English` primeiro, depois `## Português`, mesmo conteúdo nos dois.

Uma issue nova recebe a label `needs-triage` de `.github/workflows/issues.yml`. Um corpo com menos de 80 caracteres também recebe `needs-info` e um comentário pedindo os campos acima. Depois da triagem, uma de `ready-for-agent`, `ready-for-human` ou `wontfix` se aplica; a `needs-info` sai sozinha quando o corpo fica longo o suficiente.

## Pull requests

1. Crie a branch a partir da `main` (`feat/<tema>`, `fix/<tema>`, `docs/<tema>`).
2. Reproduza bugs com um PDF de exemplo antes (`public/samples/`).
3. Rode os testes antes de abrir o PR:

   ```bash
   cargo test -p tsuro
   cargo test -p tsuro-sign   # só se mexeu em assinaturas
   ```

   `.github/workflows/pr.yml` roda os mesmos comandos em macos-15 e windows-latest, com a biblioteca Pdfium baixada e plantada ao lado do binário de teste, testes com `--test-threads=1`.
   `.github/workflows/review.yml` roda `cargo fmt -p tsuro -p tsuro-sign -- --check` e `cargo clippy -p tsuro -p tsuro-sign --all-targets` em macos-15.
4. Código novo do visor → `crates/tsuro/...`; de assinaturas → `crates/tsuro-sign/...`.

Commits seguem [Conventional Commits v1.0.0](https://www.conventionalcommits.org/en/v1.0.0/): `tipo(escopo): descrição (#issue) (#PR)`.

```
feat(view): rolagem contínua por modo de página (#26) (#36)
fix(tsuro): mantém bitmap antigo ao trocar o zoom (#21)
```

Tipos: `feat`, `fix`, mais `docs`, `chore`, `assets`, `ci` e os outros tipos recomendados pela spec. Escopos neste repo incluem `view`, `window`, `kiri`, `font`, `theme`, `session`, `nav`, `print`, `tsuro`, `sign`, `docs`, `ci` e `mac`. Breaking changes usam `!` (`feat(view)!: ...`) com rodapé `BREAKING CHANGE:`.

Títulos de PR usam a mesma forma `tipo(escopo): descrição`. O corpo é bilíngue (`## English` depois `## Português`).

PRs de bug entram primeiro na fila, depois UI/UX, depois melhorias, depois features. Se o seu PR fica abaixo de um bug, ele pode esperar.

## Não

- Mexer no legado (`src/`, `src-tauri/`) — só consulta.
- Editar fixtures em `public/samples/` à mão — regenere com `scripts/generate_samples.py`.
- Adicionar dependências sem perguntar na issue/PR antes.
- Commitar segredos ou chaves. Nunca `git push --force`.
