#!/usr/bin/env python3
"""Verifica fichas de função do Programa Senbazuru.

Uso:
  python3 verificar_entrega.py --pacote FA --arquivo /tmp/tsuro-roadmap/saida/FA-orime-fusen.md
  python3 verificar_entrega.py --exemplo --arquivo /tmp/exemplo.md

Saída com código 0 só quando todas as fichas passam. Lê o catálogo em
04-catalogo-de-candidatas.md, então tipo de ficha e IDs vêm de uma fonte só.
"""
import argparse
import glob
import os
import re
import sys

ROOT = "/Users/claudio/leitor-pdf-opensource"
CAT = os.path.join(ROOT, "docs/roadmap/_insumos/04-catalogo-de-candidatas.md")

REQ = {
    "completa": [
        "Ficha", "Domínio e tipo", "Promessa", "Origem (evidência)", "Não copiar",
        "Mission gate", "Situação atual no Tsuro", "História de usuário",
        "Critérios de aceite", "Experiência no Kiri", "Expressão temática",
        "Dados e código", "Testes", "Riscos e premissas", "Dependências",
        "Tamanho e confiança", "Prioridade sugerida", "Definição de pronto",
        "Rascunho de issue",
    ],
    "curta": [
        "Ficha", "Promessa", "Origem (evidência)", "Não copiar", "Mission gate",
        "Situação atual no Tsuro", "Hipótese e como validar", "Critério de saída",
        "Dependências", "Tamanho e prioridade", "Expressão temática",
        "Rascunho de issue",
    ],
    "épico": [
        "Ficha", "Visão", "Conflito de princípio e controle de mudança",
        "Origem (evidência)", "Não copiar", "Mission gate", "Hipóteses",
        "Experimentos", "Faseamento sugerido", "Expressão temática",
        "Riscos e premissas", "Critérios para reabrir a decisão", "Dependências",
        "Rascunho de issue",
    ],
    "pacto": [
        "Ficha", "Propósito", "Cláusulas", "Como verificar a cada release",
        "Origem (evidência)", "Expressão temática", "Rascunho de issue",
    ],
}
ALLOWED_LABELS = {
    "accessibility", "bug", "documentation", "duplicate", "enhancement",
    "good first issue", "help wanted", "invalid", "question", "wontfix",
    "ready-for-agent", "needs-triage", "needs-info", "ready-for-human", "ui",
    "navigation", "ux", "annotation", "print",
}
EMOJI = re.compile("[\U0001F300-\U0001FAFF☀-➿]")
PLACEHOLDER = re.compile(r"a preencher|\bTODO\b|\bTBD\b|\bXXX\b|lorem ipsum", re.I)
ANCHOR = re.compile(r"([A-Za-z0-9_./-]+\.(?:rs|md|toml|yml|json)):(\d+)")
DOCS = re.compile(r"(relatorio|ui-ux|codigo-e-usuarios)\.md")
CONVENTIONAL = re.compile(r"^(feat|fix|perf|docs|refactor|test|chore)\([a-z0-9-]+\): .+")


def sem_acento(s):
    return s.replace("é", "e").replace("É", "E").lower()


def carregar_catalogo():
    ids = {}
    for linha in open(CAT, encoding="utf8"):
        if not linha.startswith("| F-"):
            continue
        c = [x.strip() for x in linha.strip().strip("|").split("|")]
        ids[c[0]] = {"ficha": c[3], "pacote": c[4]}
    return ids


def indice_arquivos():
    idx = {}
    padroes = [
        "crates/**/*.rs", "crates/**/*.toml", "docs/**/*.md", "*.md",
        ".github/workflows/*.yml", "scripts/*",
    ]
    for p in padroes:
        for f in glob.glob(os.path.join(ROOT, p), recursive=True):
            idx.setdefault(os.path.basename(f), []).append(f)
    return idx


def resolver(caminho, idx):
    direto = os.path.join(ROOT, caminho)
    if os.path.isfile(direto):
        return direto
    achados = idx.get(os.path.basename(caminho), [])
    return achados[0] if achados else None


def blocos(texto):
    partes = re.split(r"(?m)^### (?=F-)", texto)
    saida = []
    for p in partes[1:]:
        cab, _, corpo = p.partition("\n")
        m = re.match(r"(F-[A-Z]+-\d+)\.\s+(.+)$", cab.strip())
        if not m:
            saida.append((cab.strip(), "", corpo, False))
            continue
        corpo = re.split(r"(?m)^## ", corpo)[0]
        saida.append((m.group(1), m.group(2), corpo, True))
    return saida


def campos(corpo):
    out, atual = {}, None
    for linha in corpo.split("\n"):
        m = re.match(r"^- \*\*(.+?)\.\*\*\s*(.*)$", linha)
        if m:
            atual = m.group(1)
            out[atual] = [m.group(2)]
        elif atual is not None:
            out[atual].append(linha)
    return {k: "\n".join(v).strip() for k, v in out.items()}


def verificar_bloco(id_, tipo, cab, corpo, idx):
    erros = []
    f = campos(corpo)
    obrig = REQ[tipo]
    faltam = [c for c in obrig if c not in f]
    if faltam:
        erros.append("campos ausentes: " + ", ".join(faltam))
    if "Ficha" in f and sem_acento(tipo) not in sem_acento(f["Ficha"]):
        erros.append(f"campo Ficha deveria ser '{tipo}', veio '{f['Ficha']}'")
    for nome in obrig:
        if nome in f and nome != "Ficha" and len(f[nome].strip()) < 12:
            erros.append(f"campo '{nome}' vazio ou curto demais")
    if "(" not in cab or ")" not in cab:
        erros.append("título sem '(Codinome)' ou '(sem codinome)'")

    def tem(campo, *padroes, tudo=True):
        txt = f.get(campo, "")
        res = [bool(re.search(p, txt, re.I)) for p in padroes]
        return all(res) if tudo else any(res)

    if tipo == "completa":
        gh = [l for l in f.get("Critérios de aceite", "").split("\n")
              if re.search(r"dado", l, re.I) and re.search(r"quando", l, re.I)
              and re.search(r"então|entao", l, re.I)]
        if len(gh) < 3:
            erros.append(f"critérios de aceite Gherkin: {len(gh)} de no mínimo 3")
        origem = f.get("Origem (evidência)", "")
        if len(DOCS.findall(origem)) < 2:
            erros.append("origem cita menos de 2 referências aos documentos de pesquisa")
        if "codigo-e-usuarios.md" not in origem:
            erros.append("origem sem referência a codigo-e-usuarios.md (evidência de usuários)")
        if not tem("Prioridade sugerida", r"must|should|could|won't", r"wsjf"):
            erros.append("prioridade sem MoSCoW e WSJF")
        if not tem("Tamanho e confiança", r"\b(P|M|G|GG)\b") or \
                not tem("Tamanho e confiança", r"alta|média|media|baixa"):
            erros.append("tamanho e confiança sem P/M/G/GG e alta/média/baixa")
        if not tem("Definição de pronto", r"cargo test -p tsuro"):
            erros.append("definição de pronto sem 'cargo test -p tsuro'")
        riscos = [l for l in f.get("Riscos e premissas", "").split("\n") if l.strip().startswith("-")]
        if len(riscos) < 2:
            erros.append("riscos e premissas com menos de 2 itens")
    if tipo in ("completa", "curta", "épico"):
        if not tem("Mission gate", r"ler|anotar|marcar|imprimir") or \
                not tem("Mission gate", r"passa|ajuste|recusar"):
            erros.append("mission gate sem tarefa (ler/anotar/marcar/imprimir) ou veredito (passa/ajuste/recusar)")
    if tipo == "curta":
        if not tem("Tamanho e prioridade", r"\b(P|M|G|GG)\b") or \
                not tem("Tamanho e prioridade", r"must|should|could|won't"):
            erros.append("tamanho e prioridade sem P/M/G/GG e MoSCoW")
    if tipo in ("completa", "curta", "épico"):
        if "Expressão temática" in f and not re.search(r"\bOri\b|sem codinome|nenhum ícone", f["Expressão temática"], re.I):
            erros.append("expressão temática sem menção a Ori ou a 'sem codinome' ou 'nenhum ícone'")
    if tipo == "completa":
        if not tem("Expressão temática", r"pt-BR", r"\bEN\b"):
            erros.append("expressão temática sem microcopy em pt-BR e EN")
    if tipo == "pacto":
        linhas = [l for l in f.get("Cláusulas", "").split("\n") if re.match(r"^\|\s*C-\d+", l.strip())]
        if not 8 <= len(linhas) <= 14:
            erros.append(f"cláusulas: {len(linhas)} linhas C-NN, esperado de 8 a 14")
    if tipo == "épico":
        if len(f.get("Experimentos", "")) < 120:
            erros.append("experimentos curto demais (mínimo 120 caracteres)")
    issue = f.get("Rascunho de issue", "")
    mt = re.search(r"Título:\s*`([^`]+)`", issue)
    if not mt or not CONVENTIONAL.match(mt.group(1)):
        erros.append("rascunho de issue sem 'Título:' em Conventional Commits entre crases")
    ml = re.search(r"Labels:\s*(.+)", issue)
    if not ml:
        erros.append("rascunho de issue sem 'Labels:'")
    else:
        rot = set(re.findall(r"`([^`]+)`", ml.group(1)))
        if not rot <= ALLOWED_LABELS:
            erros.append("labels fora do vocabulário: " + ", ".join(sorted(rot - ALLOWED_LABELS)))
        if "needs-triage" not in rot:
            erros.append("labels sem needs-triage")
        if tipo != "pacto" and "enhancement" not in rot:
            erros.append("labels sem enhancement")
    for m in ANCHOR.finditer(corpo):
        caminho, n = m.group(1), int(m.group(2))
        real = resolver(caminho, idx)
        if real is None:
            erros.append(f"âncora para arquivo inexistente: {caminho}:{n}")
        else:
            total = sum(1 for _ in open(real, encoding="utf8", errors="ignore"))
            if n < 1 or n > total:
                erros.append(f"âncora fora do arquivo: {caminho}:{n} (arquivo tem {total} linhas)")
    return erros


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--arquivo", required=True)
    ap.add_argument("--pacote")
    ap.add_argument("--exemplo", action="store_true")
    a = ap.parse_args()
    if not os.path.isfile(a.arquivo):
        print(f"ERRO: arquivo não existe: {a.arquivo}")
        return 2
    texto = open(a.arquivo, encoding="utf8").read()
    cat = carregar_catalogo()
    idx = indice_arquivos()
    problemas_globais = []
    for simbolo, nome in (("—", "travessão"), ("–", "meia-risca")):
        n = texto.count(simbolo)
        if n:
            problemas_globais.append(f"{n} ocorrência(s) de {nome}, proibido")
    for m in PLACEHOLDER.finditer(texto):
        problemas_globais.append(f"placeholder proibido: '{m.group(0)}'")
        break
    if EMOJI.search(texto):
        problemas_globais.append("emoji encontrado, proibido")
    esperado = set()
    if a.exemplo:
        esperado = {"F-EXE-01"}
        cat["F-EXE-01"] = {"ficha": "completa", "pacote": "EXE"}
    elif a.pacote:
        esperado = {k for k, v in cat.items() if v["pacote"] == a.pacote}
        if not esperado:
            print(f"ERRO: pacote desconhecido: {a.pacote}")
            return 2
    vistos, resultado = {}, {}
    for id_, cab, corpo, ok in blocos(texto):
        if not ok:
            problemas_globais.append(f"título de ficha mal formado: '{id_}'")
            continue
        if id_ in vistos:
            problemas_globais.append(f"ficha duplicada: {id_}")
            continue
        vistos[id_] = True
        if id_ not in cat:
            problemas_globais.append(f"ID fora do catálogo: {id_}")
            continue
        resultado[id_] = verificar_bloco(id_, cat[id_]["ficha"], cab, corpo, idx)
    ausentes = sorted(esperado - set(vistos))
    for i in ausentes:
        problemas_globais.append(f"ficha ausente: {i}")
    for i in sorted(set(vistos) - esperado):
        if esperado and i in cat:
            problemas_globais.append(f"ficha de outro pacote: {i}")
    total_erros = len(problemas_globais) + sum(len(v) for v in resultado.values())
    print(f"Arquivo: {a.arquivo}")
    print(f"Fichas esperadas: {len(esperado)} | encontradas: {len(vistos)}")
    for i in sorted(resultado):
        e = resultado[i]
        print(f"  {i} [{cat[i]['ficha']}]: {'OK' if not e else str(len(e)) + ' problema(s)'}")
        for x in e:
            print(f"      - {x}")
    for p in problemas_globais:
        print(f"  GLOBAL - {p}")
    print("RESULTADO: " + ("APROVADO" if total_erros == 0 else f"REPROVADO ({total_erros} problema(s))"))
    return 0 if total_erros == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
