<p align="right">🇧🇷 <a href="CONTRIBUTING.pt-BR.md">Português</a></p>

# Contributing to TsuroPDF

Issues, PRs, and commits in **English or PT-BR** — both are first-class. Keep PRs small and one-topic.

## Mission gate

TsuroPDF is a fast, lightweight PDF reader: read, mark, print. Nothing else. Before proposing anything, ask: does it make reading, marking, or printing faster, lighter, or clearer? Out of scope, on purpose: forms, cloud, collaboration, suites. Proposals outside the mission will be declined or deferred.

## What we want first

When two proposals compete for review time, this is the order:

1. **Bugs** — it crashes, it corrupts, it misreads. Reproduce with a fixture PDF first; a fix with a failing-before/passing-after test lands fast.
2. **UI/UX** — the reading surface, chrome, panels, shortcuts, contrast, layout. Bugs of taste and clarity count here too: misaligned controls, unreadable hints, broken focus.
3. **Improvements** — making something that works today work better: faster render, lower memory, better defaults, tighter copy, i18n gaps.
4. **New features** — genuinely new capability. Lowest priority and the strictest gate: passes the mission, small blast radius, no new dependencies without an issue discussion first.

If you are picking up work, pick from the top. If you are proposing, this order is also how triage labels are applied: bugs are triaged before features.

## Issues

Check open issues first — duplicates get closed. A good report has:

- TsuroPDF version (release tag, e.g. `v0.6.0`, or commit hash)
- OS (macOS Apple Silicon, Windows x64)
- Steps to reproduce, expected vs. actual behavior
- A sample PDF: attach one, or name a file in `public/samples/`
- For signature reports: the state shown in the signatures panel
- Which priority bucket it falls in (bug, UI/UX, improvement, feature)

Issues and PRs carry a bilingual body: `## English` first, then `## Português`, same content in both.

A new issue gets the `needs-triage` label from `.github/workflows/issues.yml`. A body shorter than 80 characters also gets `needs-info` and a comment asking for the fields above. After triage, one of `ready-for-agent`, `ready-for-human`, or `wontfix` applies; `needs-info` is removed automatically once the body is long enough.

## Pull requests

1. Branch from `main` (`feat/<topic>`, `fix/<topic>`, `docs/<topic>`).
2. Reproduce bugs with a fixture PDF first (`public/samples/`).
3. Run the tests before opening the PR:

   ```bash
   cargo test -p tsuro
   cargo test -p tsuro-sign   # only if you touched signatures
   ```

   `.github/workflows/pr.yml` runs the same commands on macos-15 and windows-latest, with the Pdfium library downloaded and planted next to the test binary, tests with `--test-threads=1`.
   `.github/workflows/review.yml` runs `cargo fmt -p tsuro -p tsuro-sign -- --check` and `cargo clippy -p tsuro -p tsuro-sign --all-targets` on macos-15.
4. New viewer code → `crates/tsuro/...`; signature code → `crates/tsuro-sign/...`.

Commits follow [Conventional Commits v1.0.0](https://www.conventionalcommits.org/en/v1.0.0/): `type(scope): description (#issue) (#PR)`.

```
feat(view): continuous scroll page mode (#26) (#36)
fix(tsuro): keep old bitmap when switching zoom (#21)
```

Types: `feat`, `fix`, plus `docs`, `chore`, `assets`, `ci` and the other spec-recommended types. Scopes in this repo include `view`, `window`, `kiri`, `font`, `theme`, `session`, `nav`, `print`, `tsuro`, `sign`, `docs`, `ci` and `mac`. Breaking changes use `!` (`feat(view)!: ...`) with a `BREAKING CHANGE:` footer.

PR titles use the same `type(scope): description` form. The body is bilingual (`## English` then `## Português`).

Bug-fix PRs land first in the queue, then UI/UX, then improvements, then features. If your PR sits below a bug fix, it may wait.

## Don't

- Touch the legacy tree (`src/`, `src-tauri/`) — reference only.
- Hand-edit fixtures in `public/samples/` — regenerate via `scripts/generate_samples.py`.
- Add dependencies without asking in the issue/PR first.
- Commit secrets or keys. Never `git push --force`.
