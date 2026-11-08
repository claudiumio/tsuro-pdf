# Tsuro UI — Kiri

Mockup: [`e-kiri.html`](./e-kiri.html)

## Nome

**Kiri** (きり) — estratégia visual do Tsuro. Chrome fino como neblina; documento em evidência. Nome para issues, PRs e código (`kiri::tokens`, namespace `kiri` em `view.rs`).

## Objetivo

UI moderna, slim, orientada a ícones, baixa carga cognitiva, robustez visual/técnica, zero impacto em performance (Rust + iced + Pdfium).

## Princípio

**O PDF é a interface** — pílula central = trilho de navegação.

## Aparência

| Regra | Detalhe |
|---|---|
| Dark mode padrão | Primeira abertura = escuro |
| Light opt-in | Menu ⋯ → Aparência |
| Cores sólidas | Sem gradiente, glass ou rgba decorativo |
| PDF sempre branco | Folha = papel, em qualquer tema |

## Pilares

1. Document-first
2. Chrome quieto (36px)
3. Pílula central (busca + página)
4. Ori (origami tsuru) + tooltip pt-BR
5. Estado sem texto (dots)
6. Progressive disclosure (menu ⋯)

## Ori — iconografia origami

**Ori** (折) substitui Lucide no chrome Kiri. Símbolos desenhados como origami geométrico, mesma família do tsuru do logo.

Galeria: [`ori-icons.html`](./ori-icons.html) · SVGs: [`ori/`](./ori/)

| Regra | Detalhe |
|---|---|
| Forma | 2–4 polígonos chapados + linha de vinco |
| Paleta | `#5ec9c1` / `#3a9d95` / `#247870` + vinco `#1a5c56` |
| Tamanho | 16–17px na barra (24×24 viewBox) |
| Cor | Fixa no SVG — identidade tsuru, não `currentColor` |
| Performance | `include_bytes!` — igual hoje |

Conjunto v1: abrir, buscar, ◀▶, zoom −/+, ajustar, páginas, assinaturas, overflow.


## Tokens dark

| Token | Hex |
|---|---|
| bg | `#181818` |
| surface | `#222222` |
| chrome | `#1a1a1a` |
| elevated | `#262626` |
| ink | `#ececea` |
| muted | `#9a9890` |
| line | `#333331` |
| accent | `#4db8b0` |
| page | `#ffffff` |

## Tokens light

| Token | Hex |
|---|---|
| bg | `#efefed` |
| surface | `#ffffff` |
| chrome | `#f7f7f6` |
| elevated | `#ffffff` |
| accent | `#1f7d76` |
| page | `#ffffff` |

Barra **36px**, ícone **16px**, hit **36×36px**.

## Fases

1. Tokens + palco + barra 36px
2. Fundir statusbar na topbar
3. Dots de estado
4. Menu overflow
5. Tema persistido
6. Modo foco (opt-in)
7. Substituir Lucide por Ori (`assets/icons/ori/`)

## Aceite

- App abre em dark
- Zero gradientes no chrome
- Folha branca em ambos os temas
- Uma linha de chrome com PDF aberto
- Pdfium inalterado
- Toolbar usa ícones Ori (origami), não Lucide
