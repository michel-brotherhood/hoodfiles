# Site do HoodFiles

Página estática de pré-lançamento, em `site/`, publicada com GitHub Pages. Por enquanto o projeto existe só online (este site e o repositório); o aplicativo para baixar vem depois, quando a prova nativa (HF-00) for concluída. Todo conteúdo que descreve recursos é meta de projeto e diz isso na página.

## Estrutura

| Caminho | Conteúdo |
| --- | --- |
| `site/index.html` | Página única: hero, demonstração do fluxo, recursos, plataformas, roadmap e download |
| `site/assets/css/styles.css` | Sistema de design (paleta da arte de identidade), mobile-first, `prefers-reduced-motion` |
| `site/assets/js/app.js` | Menu móvel, link ativo, revelar ao rolar e a demonstração (sem dependências) |
| `site/assets/img/` | Arte do hero (WebP), ícones, favicon e imagem de compartilhamento (`og.jpg`) |
| `site/404.html` | Página de erro (usa caminhos com o prefixo `/hoodfiles/`, exigido pelo GitHub Pages) |
| `design/identidade-original.webp` | Arte de identidade original (fonte dos assets; **não** é publicada) |
| `design/make-site-assets.py` | Regenera hero, ícones e `og.jpg` a partir da arte original (Pillow) |
| `.github/scripts/check-site.py` | Verificador: ligações, âncoras, `alt`/dimensões das imagens, zero dependências externas, tamanho |
| `.github/workflows/pages.yml` | Verifica e publica `site/` na `main` |

## Identidade visual

Preto `#000` com violeta (`#9506d0`, `#c00fe3`), magenta (`#dd1be0`, `#e21779`), vermelho (`#e90d35`) e lilás (`#e9d5ff`), extraídos da arte. A cápsula da jaqueta virou a marca (logo, favicon e ícones). A arte tem fundo preto puro e é fundida ao brilho da página com `mix-blend-mode: screen`; por isso o `<img>` anima sozinho (animação no contêiner isolaria a mistura e faria o retângulo preto aparecer).

## Visualizar e verificar localmente

```text
python3 -m http.server --directory site 8080     # abre http://localhost:8080
python3 .github/scripts/check-site.py site        # deve terminar com "site OK"
python3 design/make-site-assets.py                # só se mudar a arte ou a marca
```

O site não usa fontes, scripts nem imagens de terceiros, não coloca cookies e não rastreia visitantes. O verificador falha se alguém introduzir uma dependência externa.

## Publicação (GitHub Pages)

1. Uma única vez, no GitHub: **Settings → Pages → Build and deployment → Source: GitHub Actions**.
2. O workflow `Site (GitHub Pages)` publica a cada push na `main` que altere `site/` (ou manualmente, em Actions). A URL esperada é `https://michel-brotherhood.github.io/hoodfiles/`.

Pontos de atenção:

- **Repositório privado:** o GitHub Pages para repositórios privados depende do plano da conta. Em plano gratuito o Pages só publica de repositório público. Tornar o repositório público é uma decisão do responsável e não foi feita.
- Os links "GitHub", "Status do projeto" e "Ler a especificação" apontam para o repositório; enquanto ele for privado, só abrem para quem tem acesso.
- A imagem de compartilhamento (`og:image`) usa a URL absoluta acima; se o endereço mudar (domínio próprio, outro nome de repositório), atualize `site/index.html` e o prefixo `SITE_PREFIX` em `.github/scripts/check-site.py`.
- O link de download ainda não existe: a seção "Download" diz "em breve". Quando houver instaladores, publique-os como Releases e aponte a seção para eles.
