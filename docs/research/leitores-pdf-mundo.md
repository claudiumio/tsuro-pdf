# Leitores PDF pelo mundo — inventário, funções ideais e diferenciais para o Tsuro

> Pesquisa feita em 2026-09-09 via 5 levantamentos paralelos contra fontes primárias (help/docs oficiais de cada produto; Wikipedia só para inventário). Claims importantes verificados por busca independente. Convenção: função citada sem fonte primária explícita no texto tem a URL na tabela do apêndice. `[não verificado]` = ausente nas fontes consultadas.

## 1. Escopo e método

| Cluster | Apps cobertos | Evidência bruta |
|---|---|---|
| Desktop comercial | Adobe Acrobat Reader, Foxit PDF Reader/Editor, PDF-XChange Editor, Nitro PDF Pro, PDFgear, PDFelement, Master PDF Editor, WPS PDF, PDF Expert, PDFpen, Apple Preview | `local://survey-comercial.md` |
| Desktop FOSS | Okular, Evince/Atril, SumatraPDF, MuPDF/mupdf-gl, Zathura, qpdfview, Skim, llpp, apvlv, ePDFView, Xpdf, Firefox pdf.js | `local://survey-foss.md` |
| Navegador/SO | Chrome, Edge (PDF do Windows 11), Firefox, Safari, Quick Look, Google Drive preview | `local://survey-web.md` |
| Mobile | Adobe (iOS/Android), Xodo, Foxit Mobile, PDF Expert, GoodReader, Google Files/Chrome, Apple Books/Preview, WPS, ReadEra, LiquidText, MarginNote, Flexcil, Kami | `local://survey-mobile.md` |
| Estudo/pesquisa | sioyek, Zotero reader, ReadCube Papers, Skim, LiquidText, MarginNote, PDF Expert | `local://survey-estudo.md` |

**Patamar 2026 (só de ter navegador/SO):** zoom + fit largura/página, rotação, busca Ctrl+F, seleção/copiar, imprimir, baixar — e **Edge/Chrome já marcam texto** (highlight + notas) no visualizador embutido; Chrome ganhou Draw/Highlighter/Eraser/assinatura livre em 2024+ (verificado: https://support.google.com/chrome/answer/16215622). Um leitor desktop "só leitura" não é mais diferencial: anotação deixou de ser luxo.

## 2. Funções presentes nos melhores leitores (mapa por categoria)

### Navegação/exibição (leitura)
- **Modos de página explícitos** — Single / Continuous / Facing / Facing Continuous, com capa separada: Adobe, Foxit, PDF-XChange, Nitro (Win), PDF Expert, Okular, Evince (Dual), SumatraPDF (Book View), Skim, Master PDF, pdf.js (scroll vertical/horizontal/wrapped + spread odd/even via barra `>>`).
- **Controles de leitura compactos num cluster** (zoom + ir-para-página + layout): Adobe/Foxit/Nitro barra inferior direita; Evince header bar minimalista com select de página.
- **Ir para página digitável**: todos os desktop sérios; Edge inclui campo Go-to-page. **Tsuro não tem** (issue #25 já planeja).
- **Teclado**: PgUp/PgDn/Home/End + setas; foco na busca com atalho (pdf.js Ctrl+Alt+G); "/" incremental; SumatraPDF é vim-like (`hjkl`, `g`); caret browsing no Chrome/Edge (F7).
- **Sumário/TOC + miniaturas em sidebar**: Firefox (dual), Edge (TOC), Okular, Evince, SumatraPDF, Preview (só se embutido). TOC embutido é o padrão para documentos longos.
- **Histórico voltar/avançar dentro do documento** (depois de salto de busca/link): Master PDF (Previous/Next View), SumatraPDF (Alt+←/→), Adobe, sioyek (histórico estilo navegador), Zotero. **Ninguém leve dá destaque** — joia escondida.
- **Restaurar posição por arquivo** (reabrir na mesma página/zoom/modo): SumatraPDF (persistência por doc), Nitro (restore last view), llpp (por-documento), PDF-XChange (session). Preview tem preferência "abrir na última página vista".
- **Reta final de leitura — régua/barra de leitura**: sioyek (visual mark, linha ativa), Skim (reading bar). Nenhum app mainstream casual tem; possível diferencial discreto.
- **Fullscreen/F11** e modo apresentação: okular/presentation, Evince, SumatraPDF (F5), pdf.js, PDFgear (slideshow).
- **Rotação de vista**: Chrome/Edge/Firefox/Preview/Quick Look/Sumatra/Okular etc. — **Tsuro não tem**; essencial para scans fotografados de lado.
- **Zoom modes**: fit page/width/actual/content (SumatraPDF), loupa e pan & zoom (Adobe); zoom to selection no Preview.
- **Night mode do documento separado do tema da UI** (inverte/amarela só a folha): PDF Expert (Day/Night/Sepia), PDFgear (Day/Night/Eye Protection), Foxit/Adobe (Replace Document Colors), WPS/Pdfelement (modos de conforto), Okular (invert/recolor), SumatraPDF (`Shift+i`), mupdf-gl (`-I`), ReadEra. Padrão forte de leitura noturna sem tocar no chrome.
- **Reflow / Reading Mode** (reformatar em coluna única): Foxit, GoodReader, PDF Expert (iPhone). Mobile-first; no desktop é controverso (quebra fidelidade do PDF).

### Texto
- Busca com contagem e navegação entre ocorrências: todos; highlight simultâneo de todas as ocorrências (pdf.js, Foxit), busca em comentários/anotações (PDF Expert), busca multi-doc (Adobe/Foxit Pro), regex fora do GUI (SumatraPDF-tool, mutool grep, sioyek com índice), OCR em scans (Chrome local, Adobe/Foxit pagos, GoodReader). **Sem regex no GUI mainstream** — consistente.
- Seleção/cópia universal; Preview busca em notas; Evince busca incremental com case/whole-word.

### Marcação/anotação
- **Kit básico universal** (desktop+web+mobile): highlight, sublinhado, riscado, sticky note, caixa de texto; desenho livre como extra (Adobe, Foxit, Preview, Chrome). Tipos adicionais: formas/setas, carimbo, assinatura por imagem (Preview, Chrome), lupa (Preview).
- **Persistência no próprio PDF** (compatível entre apps): padrão de todos (Adobe, Foxit, Okular, SumatraPDF, WPS explicitamente "PDF standard, compatível com Acrobat"). Preview persiste no arquivo; Files by Google salva **cópia** preservando original (fluxo não-destrutivo).
- **Painel de marcações** com lista + salto + filtro (por página/cor/tipo/autor): Foxit (Summarize), PDF Expert, Okular (Annotations pane), Evince, PDFelement (sort multi-critério), PDFpen (print list), Preview (Highlights and Notes).
- **Exportar/extrair marcações**: PDF Expert (Annotation Summary → HTML/Text/**Markdown**, Mac), ReadCube (TXT/CSV), Skim (RTF), Zotero (nota filha com citações + "Show on Page"), Okular (exportar anotações no PDF), sioyek (embute no PDF/JSON). Nenhum leitor leve e local faz resumo markdown bem.
- **Highlights tipados por cor/finalidade**: sioyek (a–z pesquisáveis), PDF Expert (filtro por cor), Zotero (cores 1–8 + tags).
- **Régua de marcação rápida**: Okular Alt+0…9, Zotero ferramenta com trava para marcar em série.
- **Anotações fora do PDF (sidecar)**: Skim `.skim`, apvlv notes externas — anti-padrão de portabilidade; Okular `.okular` é opcional, padrão é no PDF.

### Impressão
- Todos usam **diálogo nativo** com faixa de páginas, ímpares/pares, booklet, imprimir comentários ou não (Adobe/Foxit/Nitro/PDF-XChange/Master), "Print as Image". Preview: selecionar páginas via miniaturas → imprimir seleção; flatten via "Save as PDF" no diálogo.
- **Ninguém** gera "PDF de impressão 200 DPI que abre no visualizador" — a forma leve do Tsuro (#27) não tem paralelo direto: ou é diálogo nativo (pesado) ou export achatado com escolha de app.

### Assinatura digital (verificação CMS/PAdES)
- **Quem verifica de verdade**: Adobe Reader (grátis), Foxit Reader (automática + CRL), Okular (painel; depende do Poppler), Edge (PDF do Windows), PDF-XChange (PRO), PDFgear, Zotero(?) [não verificado]. SumatraPDF assina mas verificação limitada.
- **Quem só faz assinatura visual/imagem (sem validar certificado)**: Chrome, Preview, PDF Expert, PDFpen, Drive eSignature. É a maioria.
- Tsuro já verifica (crate `tsuro-sign`) — posição de destaque num mercado onde quase todo mundo confunde "desenhar assinatura" com "validar".

### Performance e arquivos grandes
- Referências: GoodReader (marketing: maiores PDFs offline), SumatraPDF/MuPDF engine, PDF-XChange (render sync/async adaptativo — Auto), Okular (perfis de memória Low→Greedy + cache tunável), Zathura (page-cache-size), llpp (OpenGL). **Cache de página vizinha para seta sem flicker** (#25-C) está alinhado ao estado da arte.

## 3. Funções ideais para a missão Tsuro (ler · anotar · marcar · imprimir)

Critério: serve leitor casual + profissional que lê/assina contrato; leve; nada de suíte. Ordem = prioridade sugerida.

### Fase atual — navegação (issues abertas #25, #26, #27)
1. **Ir para página + teclado + cache ±1** (#25) — patamar mínimo de qualquer leitor; sem isso não há "leitor".
2. **Rolagem contínua + espelhadas com capa separada** (#26) — padrão Adobe/Foxit/pdf.js/Sumatra (Book View)/Skim; "página única" vira default herdado, não escolha.
3. **Impressão leve 200 DPI** (#27) — sem paralelo; validar no smoke que Preview abre com as mesmas páginas/MediaBox.
4. **Rotação de vista** (Ctrl/Cmd+Shift+R ou menu) — **gap real hoje**; Chrome/Edge/Firefox/Preview têm; PDF escaneado torto é caso de uso comum; custo baixo no engine (render com rotação é transformação de bitmap/quad). Pdfium suporta FPDFPage_SetRotation / render rotate.

### Fase 2 — retomar e navegar estrutural
5. **Restaurar posição por arquivo** (última página + zoom + modo ao reabrir recente) — Preview/SumatraPDF/Nitro provam o valor; infra de recents já existe; custo: persistir em prefs sidecar.
6. **Histórico voltar/avançar no documento** (após salto de busca ou go-to; Alt+←/→) — barato (pilha de PageNo) e corrige o maior atrito de busca: "voltei para onde estava?". Diferencial de conforto raro nos leves.
7. **Sumário/TOC em aba no painel de páginas** (quando o PDF tem bookmarks) — Edge/Firefox/Okular; contratos longos têm outline. Exige ler bookmarks do Pdfium (engine.rs) — pedir aprovação; alternativa v1: derivar de favoritos? Não — sem bookmarks do Pdfium, adiar.
8. **Fullscreen (F11) como modo de leitura** — universal; iced tem `window::toggle_maximize`/fullscreen? (modo foco Kiri previsto na spec como opt-in).

### Fase 3 — anotar e marcar (o coração da missão que falta no visor)
9. **Marcação mínima interoperável**: highlight, sublinhado, riscado (seleção de texto → toolbar flutuante/atalho) + sticky note ancorada + borracha + undo/redo. **Persistência no próprio PDF** (padrão de mercado — Acrobat/Foxit/Okular/Sumatra/WPS). Sem sidecar proprietário.
10. **Salvar = cópia não-destrutiva** (como Files by Google / Okular "salvar cópia anotada"): original intacto até o usuário decidir; diálogo "Salvar uma cópia com anotações".
11. **Painel "Marcações"** (lista por página com salto e remoção) — revisar sem reler; padrão PDF Expert/Okular/Foxit.
12. **Cores tipadas** (paleta fixa 4–6, ex.: dúvida/citar/importante) com filtro por cor no painel — sioyek/PDF Expert/Zotero, mas simplificado.
13. **Exportar marcações** — v1: resumo texto/HTML simples (PDF Expert faz markdown); leve, local, útil para quem anota contrato. Possível diferencial "extraia o que você marcou".
14. Assinatura **de imagem** (capturar assinatura → inserir): Preview/Chrome têm; é marcação no fundo (anotação de imagem), não CMS — NÃO confundir com o painel de validação Tsuro. Opcional, demanda real de "assinar contrato" (mas missão diz anotar/marcar; assinar com imagem = carimbo/estilo, decidir depois).

### Fase 4 — leitura confortável (opt-in, sem virar suíte)
15. **Night mode do documento** (folha escura/sépia só na leitura; chrome intacto) — PDF Expert/PDFgear/Okular; alto valor percebido, custo médio (render com tint/invert via engine? requer mudança no render — pedir aprovação; alternativa: overlay de cor sobre bitmap na view).
16. **Régua de leitura discreta** (opcional, modo foco) — sioyek/Skim; nenhum leitor casual mainstream tem → micro-diferencial Kiri coerente ("barra que acompanha a linha"). Barato se view já tem overlay.
17. **Recarregar arquivo alterado** (Evince/Zathura live) — nicho LaTeX; só se pedirem. **Fora.**

## 4. O que NÃO copiar (anti-padrões verificados + fora da missão)

- **Fricção de monetização**: watermark ao salvar (PDFelement, Master demo), upsell Pro intrusivo (Adobe), reader grátis descontinuado → trial pago (Nitro), IA/Copilot/chat na toolbar como destaque (PDFgear, WPS, PDFelement). Tsuro: sem conta, sem watermark, sem IA.
- **Suíte creep**: edição de texto/páginas, OCR, conversão, redação, forms authoring, nuvem, colaboração, eSign SaaS, mind maps/flashcards (MarginNote), workspaces (LiquidText), biblioteca (Zotero/ReadEra). Fora da missão por decisão registrada.
- **Anotação não-portável** (sidecar `.skim`, notas externas apvlv, banco Zotero-only): usuário perde ao trocar de leitor — persistir no PDF.
- **"Assinar" sem validar** (Chrome/Preview/PDF Expert/Drive): confunde imagem com certificado; o Tsuro já tem o lado sério — não diluir.
- **Regex/scripts no GUI** (sioyek/CLI tools): público casual não pede; a busca com contagem + navegação basta.
- **Reflow desktop**: quebra a fidelidade visual do PDF; é feature mobile.

## 5. Diferenciais (o que os outros não têm — posicionamento Tsuro)

| Diferencial | Evidência da lacuna | Custo |
|---|---|---|
| **Verificação de assinatura CMS/PAdES transparente e local** no leitor leve | Apenas Adobe Reader, Foxit Reader, Okular, Edge fazem; Preview/Chrome/PDF Expert não | já feito (tsuro-sign) — falta visibilidade boa |
| **Cache ±1 + pré-render: ir/voltar sem "Renderizando…"** | #21 provou o problema; nenhum leve anuncia zero-flicker; PDF-XChange tem render adaptativo como referência | #25-C |
| **Impressão leve (PDF 200 DPI → visualizador)** | Ninguém faz; todos diálogo nativo pesado ou export com escolha | #27 |
| **Zero atrito** — sem conta, sem telemetria, sem nuvem, sem assinatura de dados | Chrome/Edge/Adobe empurram conta; PDF Expert/Okular mordem em cloud | cultura + decisão de design |
| **Chrome Kiri quieto 36px, pílula central, pt-BR** | Safari/Quick Look mostram a direção; nenhum leitor leve tem identidade visual própria assim | feito |
| **Histórico voltar/avançar + restaurar posição por arquivo** | Master PDF/Sumatra/Nitro têm, mas escondidos; Tsuro pode torná-los óbvios | baixo |
| **Exportar "minhas marcações"** simples (resumo com página) | PDF Expert (pago, Mac-only) é o único caprichado; Okular exporta anotações no PDF | médio (com anotações) |
| **Régua de leitura Kiri** (régua fina na linha, modo foco) | sioyek/Skim só; mainstream não tem | baixo-médio |

Risco de posicionamento: Chrome/Edge gratuitos já cobrem 80% do leitor casual. A defesa do Tsuro não é feature-vs-feature — é **abrir e ler sem atrito** (nativo, local, rápido, bonito), **validar assinaturas** (que browser nenhum faz a sério), e depois **marcar e imprimir melhor que eles** com persistência padrão-PDF.

## 6. Roadmap sugerido (ordem proposta, alinhado às issues abertas)

1. #25 navegação direta/seamless (base de tudo)
2. #26 modos de página (contínuo + facing) — com **rotação de vista** junto (gap pequeno, dor real)
3. #27 impressão leve
4. Restaurar posição por arquivo + histórico voltar/avançar (fase 2 do backlog)
5. Marcação v1 (highlight/sublinhado/riscar/nota, persistência no PDF, cópia não-destrutiva, undo) — decidir engine (incremental update do PDF) com aprovação
6. Painel de marcações + exportar resumo
7. Night mode do documento (opt-in) — aprovação de mudança no render
8. Régua de leitura/fullscreen como modo foco Kiri

Cada fase respeita o portão da missão: só entra se deixar ler, anotar, marcar ou imprimir mais rápido, leve ou claro.

## 7. Apêndice — inventário compacto (fonte primária por app)

| App | Plataforma · modelo | Posição | Fonte |
|---|---|---|---|
| Adobe Acrobat Reader | Win/mac/mob · free | Referência mundial; valida CMS no reader grátis | adobe.com/acrobat/pdf-reader.html |
| Foxit PDF Reader | Win/mac/mob · free | Leitor+anotação+forms+assinatura; facing configurável | foxit.com/pdf-reader/ |
| PDF-XChange Editor | Win · free/PRO | Viewer potente; render sync/async | pdf-xchange.com |
| Nitro PDF Pro | Win/mac · pago | Suíte; reader grátis descontinuado | gonitro.com |
| PDFgear | Win/mac/mob · free | "100% free" + IA; modos de fundo do doc | pdfgear.com |
| PDFelement | Win/mac · trial/pago | Suíte IA; watermark no trial | pdf.wondershare.com |
| Master PDF Editor | Win/mac/Linux · pago | Histórico de view; Linux | code-industry.net |
| WPS PDF | multi · free/Pro | Módulo office; anotações PDF standard | wps.com/feature/read-pdf/ |
| PDF Expert | mac/iOS · freemium | UX Apple-first; export markdown | pdfexpert.com |
| PDFpen | mac · descontinuado | → Nitro | pdfpen.com |
| Apple Preview | macOS · SO | Benchmark casual Mac; markup+imagem; sem CMS/facing contínuo | support.apple.com/guide/preview |
| Okular | Linux+ · GPL | FOSS completo: modos, cores do doc, assinatura, perfis de memória | docs.kde.org/okular |
| Evince | Linux · GPL | Simples GNOME; dual; notas/highlight | help.gnome.org/evince |
| SumatraPDF | Win · GPL | Ultraleve tabbed; Book View; cmd palette; persistência por doc | sumatrapdfreader.org |
| MuPDF/mupdf-gl | multi · AGPL | Engine de performance; invert/tint | mupdf.readthedocs.io |
| Zathura | Linux · zlib | Vim-like; dual; assinatura opcional; cache tunável | pwmt.org/projects/zathura |
| qpdfview | Linux · GPL | Qt tabbed; restaura sessão | launchpad.net/qpdfview |
| Skim | macOS · BSD | Acadêmico; reading bar; sidecar .skim | skim-app.sourceforge.io |
| llpp | Linux · MuPDF | Pager + OpenGL; memória por doc | repo.or.cz/llpp |
| apvlv | Linux/Win · GPL | Vim-like multi-formato | naihe2010.github.io/apvlv |
| ePDFView | Linux · GPL | Fork GTK4 minimalista | github.com/jotarandom/epdfview |
| Xpdf toolkit | Linux · GPL | CLI-first | xpdfreader.com |
| Chrome PDF | multi · free | Draw/Highlighter/Sign 2024+; OCR local em scans | support.google.com/chrome/answer/16215622 |
| Edge PDF (Win11) | Win/mac · free | TOC, spread, highlight+notas, Read Aloud, valida CMS | learn.microsoft.com/deployedge/microsoft-edge-pdf |
| Firefox pdf.js | multi · free | Sidebar thumbs+outline; scroll/spread modes | mozilla.github.io/pdf.js |
| Safari PDF | macOS · free | Overlay mínimo; sem tools | support.apple.com/guide/safari |
| Quick Look | macOS · SO | Space→ver; Markup rápido | support.apple.com/guide/mac-help |
| Google Drive preview | web · conta | Preview passivo; eSignature separado | support.google.com/drive |
| Adobe mobile | iOS/And · freemium | Night mode do doc; contínuo | helpx.adobe.com/acrobat/mobile |
| Xodo | mob · freemium | Smart Pen; night/sepia | xodo.com/mobile-apps |
| PDF Expert iOS | iOS · freemium | Reading Mode reflow; Annotated pages | pdfexpert.com |
| GoodReader | iOS · pago | PDFs enormes; crop margens; page-lock | goodreader.com |
| Google Files | And · free | Anotação leve; cópia preserva original | support.google.com/files |
| Apple Books/Preview iOS | iOS · SO | Markup unificado | support.apple.com/108401 |
| ReadEra | And · free/Pro | Leitor offline; citações; brilho extra | readera.org |
| LiquidText | iPad/mac · freemium | Workspace; excertos linkados | liquidtext.net |
| MarginNote | mac/iPad · pago | Card único; mind-map (exagero p/ Tsuro) | marginnote.com |
| Flexcil | iOS/And · freemium | Gesture capture; masking pen | flexcil.com |
| Kami | web · freemium | Educacional browser-first | help.kamiapp.com |
| sioyek | multi · GPL | Leitura profunda: régua, portals, highlights tipados | sioyek.info |
| Zotero reader | multi · FOSS | Anotações cidadãs; extração p/ nota | zotero.org/support/pdf_reader |
| ReadCube Papers | multi · freemium | Figures tab; busca em highlights | about.readcube.com |
