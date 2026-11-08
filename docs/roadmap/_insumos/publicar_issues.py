import argparse
import json
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DATA = json.loads((ROOT / 'issues-specs.json').read_text())
REPO = DATA['repository']
CATEGORIES = {
    'investigar': ('Investigation first', 'Investigar primeiro'),
    'incremento': ('Bounded increment', 'Incremento delimitado'),
    'adiar': ('Future backlog, implementation deferred', 'Backlog futuro, implementação adiada'),
    'reuso': ('Small reuse increment', 'Incremento pequeno de reuso'),
    'pacto': ('Proposed product governance', 'Governança de produto proposta'),
}


def gh(*args, stdin=None):
    result = subprocess.run(
        ['rtk', 'proxy', 'gh', *args], input=stdin,
        text=True, capture_output=True, timeout=120,
    )
    if result.returncode:
        raise RuntimeError(result.stderr.strip() or result.stdout.strip())
    return result.stdout.strip()


def section_of(issue):
    for source in sorted((ROOT / 'funcoes').glob('*.md')):
        pattern = rf'^### {re.escape(issue["id"])}\.[^\n]*\n(.*?)(?=^### |^## |\Z)'
        match = re.search(pattern, source.read_text(), re.M | re.S)
        if match:
            return match[1]
    raise ValueError(f'No sheet for {issue["id"]}')


def portuguese_acceptance(issue):
    if 'custom_acceptance_pt' in issue:
        return issue['custom_acceptance_pt']
    section = section_of(issue)
    checks = re.findall(r'^  - (Dado.*)$', section, re.M)
    if checks:
        return checks
    for heading in ['Critério de saída', 'Critérios para reabrir a decisão']:
        checks = re.findall(rf'^- \*\*{heading}\.\*\* (.*)$', section, re.M)
        if checks:
            return checks
    raise ValueError(f'No Portuguese criteria for {issue["id"]}')


def conventional_title(issue):
    found = re.findall(r'^  - Título: `((?:feat|docs|fix|refactor|chore)\([\w-]+\): .+)`$', section_of(issue), re.M)
    if len(found) != 1:
        raise ValueError(f'No single Conventional Commits title for {issue["id"]}')
    return f'{found[0]} [{issue["id"]}]'


def render(issue):
    pt = portuguese_acceptance(issue)
    assert len(pt) == len(issue['acceptance']), issue['id']
    state_en, state_pt = CATEGORIES[issue['category']]
    english_checks = '\n'.join('- [ ] ' + c for c in issue['acceptance'])
    portuguese_checks = '\n'.join('- [ ] ' + c for c in pt)
    base = f'https://github.com/{REPO}/blob/{DATA["baseline"]}'
    refs = [f'- [{ref}]({base}/crates/tsuro/src/{ref})' for ref in issue['refs']]
    refs.append(f'- [Repository engineering rules]({base}/AGENTS.md)')
    references = '\n'.join(refs)
    body = f'''## English

### Scope and planning status

Roadmap ID `{issue['id']}`. Phase `{issue['phase']}`. {state_en}.

{issue['scope'][0]}

### Acceptance or investigation exit criteria

{english_checks}

### Technical gates and non-goals

{issue['gate'][0]}

This issue records authorized backlog publication, not implementation, release, or a deadline. Revalidate the current code before implementation. New production dependencies, persisted-schema changes, CI changes, legacy-tree changes, and changes under `crates/tsuro-sign` require separate approval. Keep Kiri document-first and use original assets, not competitor code or designs. No DPI increase or resolution selector is in this roadmap.

### Verification before delivery

For code changes, run `cargo test -p tsuro`, `cargo fmt --check`, and the repository's applicable clippy checks. Record failures and existing baselines. For documentation-only work, verify wording, references, and the affected contract instead of claiming native execution. Test native behavior on macOS and Windows where applicable and declare missing platform, device, engine, or printer coverage.

A green test suite does not prove a native scenario ran. Record whether Pdfium, fixtures, and geometry were available and whether assertions executed. Declare skipped scenarios explicitly. Measure only costs affected by this increment against a stated baseline. Add Portuguese and English UI wording and release notes where applicable. Original PDFs and read-only fixtures remain unchanged. No tests, spikes, benchmarks, or physical printing are claimed by publishing this issue.

## Português

### Escopo e estado do planejamento

ID `{issue['id']}`. Fase `{issue['phase']}`. {state_pt}.

{issue['scope'][1]}

### Critérios de aceite ou saída da investigação

{portuguese_checks}

### Gates técnicos e fora de escopo

{issue['gate'][1]}

Esta issue registra publicação autorizada do backlog, não implementação, release ou prazo. Revalidar código atual antes de implementar. Novas dependências de produção, esquema persistido, CI, árvore legada e mudanças em `crates/tsuro-sign` exigem aprovação específica. Manter Kiri document-first e assets próprios, sem copiar código ou design de concorrentes. Aumento de DPI e seletor de resolução não fazem parte deste roadmap.

### Verificação antes da entrega

Para código, executar `cargo test -p tsuro`, `cargo fmt --check` e clippy aplicável conforme regras do repositório. Registrar falhas e baseline existente. Para trabalho só documental, conferir textos, referências e contrato afetado, sem alegar execução nativa. Verificar comportamento nativo em macOS e Windows quando aplicável e declarar plataforma, dispositivo, motor ou impressora não cobertos.

Suíte verde não prova cenário nativo executado. Registrar disponibilidade de Pdfium, fixtures e geometria e execução das asserções. Declarar cenários pulados. Medir apenas custos afetados contra baseline declarado. Incluir textos PT e EN e nota de release quando aplicável. Originais e fixtures somente leitura permanecem intactos. Publicar esta issue não afirma teste, spike, benchmark ou impressão física executados.

## Baseline references / Referências da linha de base

Static technical review on {DATA['date']}, commit `{DATA['baseline']}`. These references are starting points, not a claim about future HEAD.

Avaliação técnica estática de {DATA['date']}, commit `{DATA['baseline']}`. Referências são pontos de partida, não afirmações sobre HEAD futuro.

{references}
'''
    return conventional_title(issue), body


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('id', nargs='?')
    parser.add_argument('--publish', action='store_true')
    parser.add_argument('--verify', action='store_true')
    args = parser.parse_args()
    ids = [item['id'] for item in DATA['issues']]
    assert len(ids) == 28 and len(set(ids)) == 28
    selected = [item for item in DATA['issues'] if not args.id or item['id'] == args.id]
    if not selected:
        raise ValueError('Unknown roadmap ID')
    if args.publish and (not args.id or args.verify):
        raise ValueError('Publish exactly one ID per invocation')
    if args.publish or args.verify:
        remote = json.loads(gh('issue', 'list', '--repo', REPO, '--state', 'all', '--limit', '1000', '--json', 'number,title,body,url,state,labels'))
        labels = {item['name'] for item in json.loads(gh('label', 'list', '--repo', REPO, '--limit', '1000', '--json', 'name'))}
    for issue in selected:
        title, body = render(issue)
        assert len(title) <= 256
        if args.publish or args.verify:
            assert set(issue['labels']) <= labels, issue['id']
            matches = [item for item in remote if f'[{issue["id"]}]' in item['title']]
            if len(matches) > 1:
                raise ValueError(f'Duplicate roadmap ID {issue["id"]}')
            if matches:
                item = matches[0]
                assert item['body'] == body, f'Remote content differs for {issue["id"]}'
                # Titles published before the Conventional Commits rule keep
                # the roadmap prefix. They are matched by ID, so publishing
                # does not duplicate them.
                assert item['title'] == title or item['title'].startswith(f'[{issue["id"]}] '), item['title']
                assert item['state'] in ('OPEN', 'CLOSED'), f'Unknown state for {issue["id"]}'
                assert set(issue['labels']) <= {label['name'] for label in item['labels']}
                print(json.dumps({'id': issue['id'], 'number': item['number'], 'url': item['url'], 'state': item['state'], 'result': 'verified'}, ensure_ascii=False), flush=True)
            elif args.verify:
                raise ValueError(f'Missing remote issue {issue["id"]}')
            else:
                url = gh('issue', 'create', '--repo', REPO, '--title', title, '--body-file', '-', '--label', ','.join(issue['labels']), stdin=body)
                assert re.fullmatch(rf'https://github.com/{re.escape(REPO)}/issues/\d+', url), url
                print(json.dumps({'id': issue['id'], 'number': int(url.rsplit('/', 1)[1]), 'url': url, 'result': 'created'}, ensure_ascii=False), flush=True)
        elif args.id:
            print(title + '\n\n' + body)
        else:
            print(f'{issue["id"]}: {len(body)} characters; bilingual acceptance verified')


if __name__ == '__main__':
    main()
