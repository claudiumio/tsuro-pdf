# Glossário temático do Tsuro

O Tsuro tem uma identidade, e as funções novas precisam falar essa língua. Este documento fixa o vocabulário, as regras de nomeação e os codinomes que eu, o PO, atribuí a cada função. Você elabora a experiência em torno desses nomes. Se tiver um nome melhor, registre em "Alternativas" na ficha, mas use o nome atribuído como principal, para não haver colisão entre pacotes que rodam em paralelo.

## 1. O que a identidade já tem

| Termo | Significado | Onde aparece |
| --- | --- | --- |
| Tsuro (鶴, tsuru, o grou) | O nome do produto e o logo, um grou de origami sobre montanhas em névoa | `docs/hero.png`, `README.pt-BR.md` |
| Kiri (霧, névoa) | A estratégia visual. O chrome é fino como névoa e o documento aparece. Dark mode padrão, cores sólidas, uma barra, folha sempre branca | `docs/ui/kiri-spec.md`, `crates/tsuro/src/kiri.rs` |
| Ori (折, dobra) | A família de ícones. De 2 a 4 polígonos chapados mais uma linha de vinco, em 16 px | `docs/ui/ori-icons.html`, `crates/tsuro/assets/icons/ori/` |

Paleta fixa dos ícones Ori: `#5ec9c1`, `#3a9d95`, `#247870` e vinco `#1a5c56`. A cor fica dentro do SVG, não vem do tema. Um ícone novo segue a mesma gramática. Um polígono para a forma, outro para a sombra da dobra e uma linha de vinco.

Tokens do Kiri em uso: fundo `#181818`, palco `#222222`, barra `#1a1a1a`, elevado `#262626`, texto `#ececea`, apagado `#9a9890`, linha `#333331`, destaque `#4db8b0`. Destaque de busca `#fde68a`, resultado atual `#f59e0b`. Estado de assinatura, verde `#5cb85c`, âmbar `#d4a017` e vermelho `#c0392b`. Fonte: `docs/ui/kiri-spec.md` e `kiri.rs`.

## 2. Regras de nomeação

1. **O rótulo funcional manda.** Toda função tem um nome claro em português do Brasil, que é o que aparece na interface, como "Marcações" ou "Voltar". O codinome japonês vive em documentação, código, nome de ícone, nota de release e microcopy de apoio. Nunca substitui o rótulo funcional.
2. **Uma palavra, uso correto.** O codinome é uma palavra japonesa real, com o sentido usado corretamente, escrita em rōmaji e acompanhada do kanji e da tradução na primeira menção.
3. **Revisão antes de aparecer.** Todo termo japonês que chegar à interface passa por revisão de um falante nativo antes do release. Enquanto isso, ele fica só na documentação. Isto é um risco registrado, não um detalhe.
4. **Sem caricatura.** Nada de dragão, bambu, ideograma decorativo aleatório, fonte que imita pincel ou frase de efeito. O tom é sóbrio, respeitoso e funcional. A cultura inspira o jeito de pensar, não vira fantasia.
5. **Temática com função.** Um elemento temático só entra se carrega significado de uso. A dobra é o vinco que persiste. A névoa esconde o chrome. O fio guia a linha. Se o tema for só enfeite, corte.
6. **O chrome continua quieto.** O tema vive no nome, no ícone, no movimento que comunica estado e na materialidade do papel. Não vive em ornamento nem em textura sobre o chrome. Cores sólidas, sem gradiente.
7. **Microcopy.** Frases curtas, sem exclamação, sem humor forçado. Exemplos já existentes: "Abra um PDF ou arraste para cá" e "Destaques copiados como Markdown." Todo texto existe em pt-BR e EN (`t.lang.pick("...", "...")`).

## 3. Codinomes atribuídos

Os significados abaixo foram escritos pelo PO. O que estiver marcado com asterisco precisa de revisão de falante nativo antes de aparecer na interface.

| ID | Rótulo funcional (pt-BR) | Codinome | Sentido e razão |
| --- | --- | --- | --- |
| F-MARC-01 | Diário de recuperação das marcações | Orime (折り目) | Vinco. O rastro que a dobra deixa mesmo depois que o papel é desdobrado. As marcações deixam rastro mesmo se o app fechar |
| F-MARC-02 | Aba Marcações | Fusen (付箋) | Etiqueta adesiva que se cola nas páginas para achar depois. O Tsuro já tem um post-it amarelo nas notas |
| F-MARC-03 | Marcações na paleta | Fusen (derivado) | A mesma etiqueta, buscada pela paleta |
| F-MARC-04 | Marcas na barra de progresso | Shirube (標) | Baliza, marco de orientação |
| F-MARC-05 | Cor nomeada e memória da última escolha | Pigmentos | Cores tradicionais do Japão como codinomes. Yamabuki (山吹, amarelo dourado), Shu (朱, vermelhão), Asagi (浅葱, azul esverdeado claro), Wakakusa (若草, verde de broto). O significado funcional de cada cor é decidido na ficha, com acessibilidade |
| F-MARC-06 | Marcação contínua | Pincelada | Sem codinome japonês. Português claro |
| F-MARC-07 | Marcadores de página | Dobras | O canto da página dobrado, a orelha. É o gesto físico, e o Tsuro é origami |
| F-MARC-08 | Exportar marcações para arquivo | Sem codinome | Nome funcional basta |
| F-NAV-01 | Voltar depois de um salto | Kaerimichi (帰り道) | O caminho de volta |
| F-NAV-02 | Rótulo de página impresso | Fólio | O número impresso na página. Os arquivos de teste do projeto já usam "folio". Termo japonês de tipografia, nonburu (ノンブル)* |
| F-NAV-03 | Copiar como parágrafo | Tsunagi (繋ぎ)* | Emenda, junta. Une as linhas de uma frase |
| F-NAV-04 | Paleta com todos os comandos | Sem codinome | A paleta já é o nome |
| F-NAV-05 | Posição que acompanha o conteúdo | Shiori (栞) | Marcador de livro que acompanha a obra, esteja onde estiver |
| F-NAV-06 | Rolagem natural | Sem codinome | Nome funcional basta |
| F-NAV-07 | Espiar o destino de um link | Nozoki (覗き) | Espiar, dar uma olhada sem entrar |
| F-NAV-08 | Sumário gerado | Kari-mokuji (仮目次)* | Sumário provisório. O prefixo kari marca que é gerado e pode errar |
| F-NAV-09 | Seleção por teclado | Sem codinome | Nome funcional basta |
| F-ARQ-01 | Senha ao abrir | Kagi (鍵) | Chave |
| F-ARQ-02 | Salvar sem quebrar assinaturas | Hanko (判子) | O selo pessoal. O selo que já estava no documento permanece intacto |
| F-ARQ-03 | Preparação de impressão leve e fiel | Sumi (墨) | Tinta. Fidelidade da saída com custo controlado, sem aumento de DPI |
| F-ARQ-04 | Imprimir com ou sem marcações | Sem codinome | Nome funcional basta |
| F-ARQ-05 | Folha de marcações impressa | Sem codinome | Nome funcional basta |
| F-ARQ-06 | Proteção e permissões explicadas | Sem codinome | Nome funcional basta |
| F-ARQ-07 | Clareza de confiança nas assinaturas | Selo | Português claro. Não repete Hanko |
| F-AMB-01 | Tema de papel oriental | Washi (和紙) | O papel artesanal japonês. É a visão do PO para depois, e exige mudar um princípio do Kiri |
| F-AMB-02 | Modo foco | Névoa (Kiri) | O chrome se dissolve e o documento fica |
| F-AMB-03 | Régua de leitura | Ito (糸) | Fio. Um fio que guia a linha em leitura |
| F-AMB-04 | Anti-funções | Pacto do Ma (間) | Ma é o intervalo, o espaço vazio que dá forma. O pacto de silêncio do produto |

## 4. Termos a evitar ou a reservar

- Hanko só para a função de salvar sem quebrar assinaturas. Selo, em português, para a clareza de confiança.
- Kiri é o nome da estratégia visual inteira. O modo foco usa "Névoa", que é a tradução, para não confundir.
- Não invente codinome com palavra japonesa de que você não tem certeza. Se tiver dúvida, escreva "sem codinome" e justifique.
- Nunca use kanji como ornamento sem tradução ao lado.

## 5. Washi, o papel oriental

A visão do PO é um tema em que a folha deixa de ser branca pura e passa a lembrar um papel oriental artesanal. Isso fica para depois. Hoje a regra do Kiri é que a folha do PDF é sempre branca (`#ffffff`), em qualquer tema. Mudar isso é uma mudança de princípio. Ela passa pelo controle de mudanças e pelo registro de decisões, e nenhuma ficha implementa isso por conta própria. A ficha F-AMB-01 trata do tema como épico, com hipóteses e experimentos, sem comprometer a implementação.
