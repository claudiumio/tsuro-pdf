# Issues bilíngues do roadmap

Publicação autorizada em 13 de outubro de 2026 no repositório `claudiumio/tsuro-pdf`. Uma issue por ID. As classificações técnicas e os gates continuam válidos. Fases não são releases nem datas. Nenhuma implementação foi autorizada por esta publicação.

Aumento de DPI e seletor de resolução foram excluídos. F-ARQ-03 investiga memória agregada, preparação, cancelamento e fidelidade. Raster permanece em 200 DPI.

## Índice e registro de publicação

| ID | Estado técnico | Fase | Issue |
| --- | --- | --- | --- |
| F-MARC-01 | Investigar primeiro | F1 | [#128](https://github.com/claudiumio/tsuro-pdf/issues/128) |
| F-MARC-02 | Incremento delimitado | F1 | [#129](https://github.com/claudiumio/tsuro-pdf/issues/129) |
| F-MARC-03 | Incremento delimitado | F2 | [#130](https://github.com/claudiumio/tsuro-pdf/issues/130) |
| F-MARC-04 | Investigar primeiro | F5 | [#131](https://github.com/claudiumio/tsuro-pdf/issues/131) |
| F-MARC-05 | Investigar primeiro | F5 | [#132](https://github.com/claudiumio/tsuro-pdf/issues/132) |
| F-MARC-06 | Implementação adiada | ADIADO | [#133](https://github.com/claudiumio/tsuro-pdf/issues/133) |
| F-MARC-07 | Incremento delimitado | F2 | [#134](https://github.com/claudiumio/tsuro-pdf/issues/134) |
| F-MARC-08 | Reuso pequeno | F1 | [#135](https://github.com/claudiumio/tsuro-pdf/issues/135) |
| F-NAV-01 | Incremento delimitado | F2 | [#136](https://github.com/claudiumio/tsuro-pdf/issues/136) |
| F-NAV-02 | Investigar primeiro | F2 | [#137](https://github.com/claudiumio/tsuro-pdf/issues/137) |
| F-NAV-03 | Investigar primeiro | F5 | [#138](https://github.com/claudiumio/tsuro-pdf/issues/138) |
| F-NAV-04 | Reuso pequeno | F2 | [#139](https://github.com/claudiumio/tsuro-pdf/issues/139) |
| F-NAV-05 | Implementação adiada | ADIADO | [#140](https://github.com/claudiumio/tsuro-pdf/issues/140) |
| F-NAV-06 | Investigar primeiro | F6 | [#141](https://github.com/claudiumio/tsuro-pdf/issues/141) |
| F-NAV-07 | Implementação adiada | ADIADO | [#142](https://github.com/claudiumio/tsuro-pdf/issues/142) |
| F-NAV-08 | Implementação adiada | ADIADO | [#143](https://github.com/claudiumio/tsuro-pdf/issues/143) |
| F-NAV-09 | Investigação cedo, implementação adiada | F0 | [#144](https://github.com/claudiumio/tsuro-pdf/issues/144) |
| F-ARQ-01 | Investigar primeiro | F3 | [#145](https://github.com/claudiumio/tsuro-pdf/issues/145) |
| F-ARQ-02 | Implementação adiada | ADIADO | [#146](https://github.com/claudiumio/tsuro-pdf/issues/146) |
| F-ARQ-03 | Investigar primeiro, sem aumento de DPI | F4 | [#147](https://github.com/claudiumio/tsuro-pdf/issues/147) |
| F-ARQ-04 | Incremento delimitado | F4 | [#148](https://github.com/claudiumio/tsuro-pdf/issues/148) |
| F-ARQ-05 | Investigar primeiro | F4 | [#149](https://github.com/claudiumio/tsuro-pdf/issues/149) |
| F-ARQ-06 | Investigar primeiro | F3 | [#150](https://github.com/claudiumio/tsuro-pdf/issues/150) |
| F-ARQ-07 | Reuso textual, verificação nova separada | F3 | [#151](https://github.com/claudiumio/tsuro-pdf/issues/151) |
| F-AMB-01 | Épico futuro adiado pelo usuário | ADIADO | [#152](https://github.com/claudiumio/tsuro-pdf/issues/152) |
| F-AMB-02 | Incremento delimitado | F6 | [#153](https://github.com/claudiumio/tsuro-pdf/issues/153) |
| F-AMB-03 | Investigar primeiro | F6 | [#154](https://github.com/claudiumio/tsuro-pdf/issues/154) |
| F-AMB-04 | Governança proposta | F0 | [#155](https://github.com/claudiumio/tsuro-pdf/issues/155) |

## Fontes e reprodução

`issues-specs.json` guarda títulos, estados, escopo bilíngue, critérios em inglês, gates e referências da linha de base. As fichas em `funcoes/` fornecem critérios em português, exceto os recortes explicitamente revisados no manifesto.

`_insumos/publicar_issues.py` gera corpos e valida idiomas e labels. Sem flags, somente valida. Com um ID, mostra o corpo. `--publish` exige exatamente um ID, consulta todos os estados e recusa duplicata ou divergência antes de criar. `--verify` compara títulos, corpos, labels e estado aberto com o remoto. O script não altera arquivos locais nem fecha ou reabre issues.

```sh
rtk proxy python3 -I docs/roadmap/_insumos/publicar_issues.py
rtk proxy python3 -I docs/roadmap/_insumos/publicar_issues.py F-ARQ-03
rtk proxy python3 -I docs/roadmap/_insumos/publicar_issues.py --verify
```

## Limites e verificação

Antes de publicar, foram consultadas 43 issues existentes, duas abertas e 41 fechadas. #44 é leitura de Markdown, não exportação de marcações. #127 é limpeza de instaladores de atualização. Nenhuma equivalente aberta foi encontrada. Funcionalidades fechadas são contexto do delta, não trabalho a recriar. Nenhuma issue preexistente será fechada, reaberta ou alterada por esta publicação.

Publicação concluída e verificada. Foram criadas 28 issues, #128 a #155. Cada retorno foi registrado antes da próxima criação. A consulta remota confirmou ID único, título, corpo bilíngue completo, labels e estado aberto em todas as 28. O total de 109 itens de aceite por idioma corresponde ao recorte das issues, não aos 90 critérios da verificação documental anterior. Nenhum código do aplicativo, CI, dependência, fixture, commit, PR ou release foi alterado por esta tarefa.

O `AGENTS.md` pede título de issue no formato Conventional Commits. As 28 issues foram publicadas antes dessa regra com o prefixo `[ID]`. Elas preservam o título atual. `publicar_issues.py` passou a gerar `type(scope): descrição [ID]` e reconhece as antigas pelo ID dentro do título, então uma nova execução não cria duplicata. A repetição de `--verify` em 7 de novembro de 2026 confirmou corpo e labels das 28 e registrou 24 abertas e quatro fechadas, `#128`, `#129`, `#135` e `#151`.

Classificação técnica preservada. Três incrementos de reuso, seis delimitados, onze com investigação inicial, sete com implementação adiada e um pacto de governança. F-NAV-09 requer investigação antecipada de barreiras apesar do adiamento da implementação completa.

O script de publicação foi validado sem escrita remota antes da execução e depois comparou os corpos esperados com o GitHub. A checagem estrutural do HTML foi repetida. Nenhuma suíte Rust, benchmark, spike ou impressão física foi executada nesta tarefa documental. Não houve nova verificação visual completa em celular após o recorte de DPI.
