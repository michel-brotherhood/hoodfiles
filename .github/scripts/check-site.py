#!/usr/bin/env python3
"""Verifica o site estático antes do deploy: ligações locais e âncoras, imagens, ausência de dependências
externas e tamanho. Uso: python3 .github/scripts/check-site.py [pasta]  (padrão: site). Sai com 1 se houver erro."""
import json, re, sys, pathlib
from html.parser import HTMLParser
from urllib.parse import urlparse

root = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else "site").resolve()
MAX_TOTAL = 3 * 1024 * 1024          # 3 MB no total
MAX_IMG = 600 * 1024                 # nenhuma imagem acima de 600 KB
SITE_PREFIX = "/hoodfiles/"       # caminho do projeto no GitHub Pages (https://<usuario>.github.io/hoodfiles/)
ALLOWED_EXTERNAL_LINKS = {"github.com", "michel-brotherhood.github.io"}   # só <a href>, nunca recursos
errors, notes = [], []

class P(HTMLParser):
    def __init__(self):
        super().__init__(); self.ids, self.refs, self.res, self.imgs, self.anchors, self.h = set(), [], [], [], [], []
        self.title = self.lang = None; self._t = False
    def handle_starttag(self, tag, a):
        a = dict(a)
        if "id" in a: self.ids.add(a["id"])
        if tag == "html": self.lang = a.get("lang")
        if tag == "title": self._t = True
        if tag in ("h1", "h2", "h3"): self.h.append(int(tag[1]))
        if tag == "a" and a.get("href"): self.anchors.append((a["href"], a.get("rel", ""), a.get("target")))
        if tag == "img": self.imgs.append(a)
        for k in ("src", "srcset", "href"):
            if k in a and tag != "a":
                vals = [v.strip().split()[0] for v in a[k].split(",")] if k == "srcset" else [a[k]]
                self.res += [(tag, v) for v in vals]
        if tag == "meta" and a.get("property") == "og:image": self.res.append(("og:image", a.get("content", "")))
        for k in ("aria-controls", "aria-labelledby"):
            if k in a: self.refs += a[k].split()
    def handle_data(self, d):
        if self._t: self.title = (self.title or "") + d
    def handle_endtag(self, tag):
        if tag == "title": self._t = False

def local_target(page: pathlib.Path, ref: str):
    u = urlparse(ref)
    if u.scheme or u.netloc or ref.startswith(("data:", "mailto:")): return None
    path = u.path
    if not path: return page
    # Páginas 404 do GitHub Pages precisam de caminhos absolutos com o prefixo do projeto ("/hoodfiles/...").
    if path.startswith(SITE_PREFIX): path = "/" + path[len(SITE_PREFIX):]
    base = root if path.startswith("/") else page.parent
    target = (base / path.lstrip("/")).resolve()
    return target / "index.html" if target.is_dir() else target

pages = sorted(root.glob("*.html"))
if not pages: errors.append(f"nenhuma página .html em {root}")
for page in pages:
    name = page.name; p = P(); p.feed(page.read_text(encoding="utf-8"))
    if p.lang != "pt-BR": errors.append(f"{name}: <html lang> deveria ser pt-BR (é {p.lang!r})")
    if not (p.title or "").strip(): errors.append(f"{name}: sem <title>")
    if name == "index.html":
        if p.h.count(1) != 1: errors.append(f"{name}: deve haver exatamente um <h1> (há {p.h.count(1)})")
        prev = 1
        for lv in p.h:
            if lv > prev + 1: errors.append(f"{name}: salto de nível de título (h{prev} -> h{lv})"); break
            prev = lv
    for tag, ref in p.res:
        if not ref: continue
        u = urlparse(ref)
        if u.scheme or u.netloc:
            if tag == "og:image" and u.netloc in ALLOWED_EXTERNAL_LINKS:
                local = root / pathlib.Path(u.path).relative_to("/hoodfiles") if u.path.startswith("/hoodfiles/") else None
                if local and not local.exists(): errors.append(f"{name}: og:image aponta para arquivo inexistente: {u.path}")
            else:
                errors.append(f"{name}: recurso externo ({tag}) proibido: {ref}")
            continue
        t = local_target(page, ref)
        if t is not None and not t.exists(): errors.append(f"{name}: <{tag}> aponta para arquivo inexistente: {ref}")
    for href, rel, target in p.anchors:
        u = urlparse(href)
        if href.startswith("#"):
            if href[1:] and href[1:] not in p.ids: errors.append(f"{name}: âncora #{href[1:]} sem destino")
        elif u.scheme in ("http", "https"):
            if u.netloc not in ALLOWED_EXTERNAL_LINKS: errors.append(f"{name}: link externo não previsto: {href}")
            if target == "_blank" and "noopener" not in rel: errors.append(f"{name}: target=_blank sem rel=noopener: {href}")
        else:
            t = local_target(page, href)
            if t is not None and not t.exists(): errors.append(f"{name}: link local quebrado: {href}")
    for r in p.refs:
        if r not in p.ids: errors.append(f"{name}: aria-controls/labelledby '{r}' sem elemento com esse id")
    for im in p.imgs:
        if "alt" not in im: errors.append(f"{name}: <img src={im.get('src')}> sem alt")
        if not (im.get("width") and im.get("height")): errors.append(f"{name}: <img src={im.get('src')}> sem width/height (evita salto de layout)")

for m in root.glob("*.webmanifest"):
    try:
        data = json.loads(m.read_text(encoding="utf-8"))
        for ic in data.get("icons", []):
            if not (root / ic["src"]).exists(): errors.append(f"{m.name}: ícone inexistente {ic['src']}")
    except Exception as e: errors.append(f"{m.name}: JSON inválido ({e})")

total = 0
for f in root.rglob("*"):
    if f.is_file():
        sz = f.stat().st_size; total += sz
        if f.suffix.lower() in (".png", ".jpg", ".jpeg", ".webp") and sz > MAX_IMG:
            errors.append(f"imagem grande demais ({sz // 1024} KB > {MAX_IMG // 1024} KB): {f.relative_to(root)}")
if total > MAX_TOTAL: errors.append(f"site com {total // 1024} KB (limite {MAX_TOTAL // 1024} KB)")
notes.append(f"{len(pages)} página(s), {sum(1 for f in root.rglob('*') if f.is_file())} arquivos, {total // 1024} KB no total")

for n in notes: print("info:", n)
if errors:
    print(f"\n{len(errors)} problema(s):"); [print(" -", e) for e in errors]; sys.exit(1)
print("site OK: ligações, âncoras, imagens e dependências verificadas")
