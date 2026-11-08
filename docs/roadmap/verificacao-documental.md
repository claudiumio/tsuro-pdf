# Verificação documental da retomada

Verificação em 13 de outubro de 2026 pelo coordenador. Escopo limitado aos três arquivos de fichas em `funcoes/` e ao `roadmap.md` desta pasta. Os checks usam Python e biblioteca padrão, sem execução do aplicativo, build, instalação ou serviços de navegador.

Repetição em 7 de novembro de 2026 na raiz do repositório, depois de alinhar o rótulo do último campo das fichas com o modelo. O resultado foi `PASS documental. 28 IDs, 90 critérios, 15 contas e 34 referências locais.` com `python3 -I` e código de saída 0. O mesmo dia registrou 24 issues abertas e quatro fechadas, `#128`, `#129`, `#135` e `#151`, na verificação de `publicar_issues.py --verify`.

## Resultado estrutural

PASS para IDs, tipos e campos das fichas, formato dos rascunhos locais de issues, contas WSJF e cobertura do roadmap.

- 28 IDs únicos, sem falta ou duplicação.
- 15 fichas completas com 19 campos na ordem do modelo e seis critérios cada.
- 11 fichas curtas com 12 campos, um épico com 14 campos e um pacto com sete campos.
- 90 critérios no padrão textual Dado, Quando e Então. O check não prova que o aplicativo os satisfaz.
- 28 títulos no formato Conventional Commits e conjuntos de labels restritos ao snapshot. Todas incluem `enhancement` e `needs-triage`. Não houve consulta de labels atuais ao GitHub.
- 15 contas WSJF com parcelas de 1 a 5 e resultado aritmético coerente, tolerância de 0,01 para arredondamento. O check não valida a escolha das notas.
- 34 referências únicas no padrão de citação local conferido apontam a seções existentes dos documentos de pesquisa. Existência de seção não prova suporte semântico integral da alegação.
- 28 linhas na tabela de cobertura e 28 na matriz de dependências, sem ID estranho ao catálogo.
- Oito spikes propostos e 12 cláusulas do pacto presentes.
- 16 links Markdown relativos no conjunto conferido, todos com destino existente, incluindo este relatório. O script reproduzível abaixo foi executado com `python3 -I` na raiz do repositório, saiu com código 0 e retornou `PASS documental. 28 IDs, 90 critérios, 15 contas e 34 referências locais.`

Distribuição da cobertura. F0 recebe 2 IDs, F1 recebe 3, F2 recebe 5, F3 recebe 3, F4 recebe 3, F5 recebe 3 e F6 recebe 3. Seis IDs ficam no backlog adiado. A entrada de F-NAV-09 em F0 é investigação, não implementação completa.

## Revisão de dependências e autoridade

O coordenador revisou a matriz para separar bloqueios técnicos, gates de risco, preferências e melhorias opcionais. As arestas obrigatórias diretas entre funções explicitadas no check foram F-MARC-02 para F-MARC-04 e F-MARC-05 e F-NAV-04 para F-MARC-06. Esse grafo não contém ciclo. Alternativas por modelo mínimo, infraestrutura existente e gates não foram tratados como exigência de entregar outra função inteira.

A revisão também conferiu os seguintes limites.

- Diário e marcadores usam contrato mínimo de identidade, sem esperar retomada entre caminhos ou salvamento incremental.
- Marcadores não herdam evicção de recentes. Salvar cópia não autoriza apagar diário.
- Lista e exportação simples não esperam labels, cores ou recuperação por dependência técnica.
- Omitir marcas da sessão não espera impressão vetorial.
- Clareza textual de assinatura não espera salvar assinados.
- Foco e régua não dependem de Washi ou um do outro.
- A implementação completa de seleção por teclado permanece proposta de adiamento, com investigação de barreiras antecipada.
- Washi futuro é instrução explícita do usuário. Adiar marcação contínua é recomendação, não decisão atribuída ao usuário.

Não foi executada análise estática do aplicativo ou prova formal da matriz. A revisão de dependências é documental e terá de ser refeita contra o código real antes de implementação.

## Correção da própria checagem

A primeira execução agregada falhou nas comparações de IDs porque `Counter` recebeu o dicionário de tipos, interpretando seus valores como contagens. O coordenador corrigiu a checagem para usar as chaves do catálogo. A nova execução passou. Não foram encontrados IDs ausentes ou duplicados nesse resultado corrigido.

Uma label `security`, fora do vocabulário do snapshot, foi removida antes da checagem final. A matriz do roadmap também deixou de tratar todo spike de segurança como bloqueio global das unidades independentes.

O último campo das 28 fichas chamava `Minuta local da issue`, enquanto o modelo exigia `Rascunho de issue`. A comparação de campos parava em `F-MARC-01`. As fichas passaram a usar o rótulo do modelo, sem mudar o texto do campo.

## Repetir a checagem estrutural principal

O trecho abaixo permite repetir IDs, campos, critérios, contas, cobertura, labels, seções e links. Não implementa função do produto nem substitui os roteiros futuros das fichas. Execute com o caminho da pasta `docs/roadmap` como argumento.

```bash
rtk proxy python3 -I - docs/roadmap <<'PY'
from pathlib import Path
from collections import Counter
import re
import sys

base = Path(sys.argv[1]).resolve()
model = (base / '_insumos/03-modelo-de-funcao.md').read_text()
fields = {}
for heading, kind in [('Ficha completa', 'completa'), ('Ficha curta', 'curta'), ('Ficha de épico', 'épico'), ('Ficha de pacto', 'pacto')]:
	part = re.split(r'^## ' + heading + r'\n', model, flags=re.M)[1].split('\n## ')[0]
	fields[kind] = re.findall(r'^\d+\. `\*\*(.+?)\.\*\*`', part, re.M)
expected = {}
for domain, total, complete in [('MARC', 8, {1, 2, 3, 5, 7}), ('NAV', 9, {1, 2, 3, 4, 5}), ('ARQ', 7, {1, 2, 3, 4})]:
	for n in range(1, total + 1):
		expected[f'F-{domain}-{n:02}'] = 'completa' if n in complete else 'curta'
expected.update({'F-AMB-01': 'épico', 'F-AMB-02': 'completa', 'F-AMB-03': 'curta', 'F-AMB-04': 'pacto'})
allowed = set('accessibility|bug|documentation|duplicate|enhancement|good first issue|help wanted|invalid|question|wontfix|ready-for-agent|needs-triage|needs-info|ready-for-human|ui|navigation|ux|annotation|print'.split('|'))
files = sorted((base / 'funcoes').glob('*.md'))
ids = []
cases = 0
formulas = 0
refs = set()
for path in files:
	text = path.read_text()
	refs.update(re.findall(r'`((?:codigo-e-usuarios|ui-ux|relatorio)\.md) § (\d+(?:\.\d+)*)`', text))
	for block in re.split(r'(?=^### F-)', text, flags=re.M)[1:]:
		id = re.match(r'### (F-\w+-\d+)', block)[1]
		ids.append(id)
		kind = re.search(r'^- \*\*Ficha\.\*\* (\w+)', block, re.M)[1]
		assert kind == expected[id], id
		assert re.findall(r'^- \*\*(.+?)\.\*\*', block, re.M) == fields[kind], id
		found = re.findall(r'^  - Dado .*Quando .*Então .*', block, re.M)
		cases += len(found)
		if kind == 'completa':
			assert 3 <= len(found) <= 6, id
		assert re.search(r'^  - Título: `(feat|docs|fix|refactor|chore)\([\w-]+\): .+`', block, re.M), id
		labels = set(re.findall(r'`([^`]+)`', re.search(r'^  - Labels: (.*)', block, re.M)[1]))
		assert labels <= allowed and {'enhancement', 'needs-triage'} <= labels, id
		for match in re.finditer(r'WSJF\s*=\s*\((\d)\+(\d)\+(\d)\)/(\d)\s*=\s*([\d,.]+)', block):
			a, b, c, n = map(int, match.groups()[:4])
			value = float(match[5].rstrip('.').replace(',', '.'))
			assert all(1 <= x <= 5 for x in [a, b, c, n]) and abs((a + b + c) / n - value) <= 0.01, id
			formulas += 1
assert Counter(ids) == Counter(expected.keys())
for name, section in refs:
	text = (base.parent / 'research/engenharia-reversa-pdf' / name).read_text()
	assert re.search(r'^#{1,4}\s+' + re.escape(section) + r'(?:[.\s])', text, re.M)
roadmap = (base / 'roadmap.md').read_text()
for heading in ['Referência de escopo dos 28 IDs', 'Referência de dependências']:
	part = roadmap.split('## ' + heading)[1].split('\n## ')[0]
	assert Counter(re.findall(r'^\| (F-\w+-\d+) \|', part, re.M)) == Counter(expected.keys()), heading
spikes = roadmap.split('## Referência dos spikes')[1].split('\n## ')[0]
assert set(re.findall(r'^\| (S-\d+) \|', spikes, re.M)) == {f'S-{n:02}' for n in range(1, 9)}
clauses = re.findall(r'^  \| (P-\d+) \|', (base / 'funcoes/03-arquivos-impressao-ambiente.md').read_text(), re.M)
assert len(clauses) == len(set(clauses)) == 12
files.append(base / 'roadmap.md')
for path in files:
	text = path.read_text()
	assert set(re.findall(r'\bF-(?:MARC|NAV|ARQ|AMB)-\d+\b', text)) <= set(expected)
	for target in re.findall(r'\[[^\]]+\]\(([^)]+)\)', text):
		if '://' not in target:
			assert (path.parent / target.split('#')[0]).resolve().exists(), target
print(f'PASS documental. {len(ids)} IDs, {cases} critérios, {formulas} contas e {len(refs)} referências locais.')
PY
```

## Não confundir documento verificado com produto pronto

Não foram executados `cargo test -p tsuro`, `cargo fmt --check`, `cargo clippy`, protótipos, benchmarks ou spikes. Não foram revalidadas linhas de código, APIs instaladas ou status atual de GitHub. Não houve teste em leitores externos, impressão física ou auditoria de privacidade. O antigo `_insumos/verificar_entrega.py` não foi executado, e seu resultado não é alegado.

As quatro tarefas Step 5 Preview terminaram com estado `completed`, resultado disponível e sem trabalhos filhos pendentes no status consultado. Houve apenas uma tarefa delegada ativa por vez nesta retomada. Isso não é prova de ausência de todo processo órfão no sistema nem medição de RAM.

A entrega é planejamento local. Não foram publicados rascunhos de issues, criados PRs, feitos commits ou alterados princípios de renderização.
