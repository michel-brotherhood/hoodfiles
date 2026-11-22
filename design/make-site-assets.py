#!/usr/bin/env python3
"""Gera os assets do site a partir da arte de identidade (design/identidade-original.webp).
Uso: python3 design/make-site-assets.py   (requer Pillow). Escreve em site/assets/img/.

A arte original tem fundo #000 puro; o hero do site usa uma versão com fundo transparente (alfa derivado da luminância).
Paleta: violeta #9506d0/#c00fe3, magenta #dd1be0/#e21779, vermelho #e90d35, lilás #e9d5ff."""
import os, pathlib
from PIL import Image, ImageDraw, ImageFilter, ImageFont

root = pathlib.Path(__file__).resolve().parent.parent
src = Image.open(root / "design" / "identidade-original.webp").convert("RGB")
out = root / "site" / "assets" / "img"
out.mkdir(parents=True, exist_ok=True)

# 1) arte recortada ao conteúdo (com folga), em duas larguras, com fundo transparente.
# O preto vira alfa: alfa = max(R,G,B) e a cor é "desmultiplicada" (como o mix-blend-mode: screen sobre preto),
# então o brilho e as linhas de varredura esmaecem em transparência e a arte funciona sobre qualquer fundo.
art = src.crop((70, 160, 1185, 1085))

def black_to_alpha(im, floor=4):
    px = im.load(); w, h = im.size
    out = Image.new("RGBA", (w, h))
    po = out.load()
    for y in range(h):
        for x in range(w):
            r, g, b = px[x, y]; a = max(r, g, b)
            if a <= floor: po[x, y] = (0, 0, 0, 0); continue
            k = 255 / a
            po[x, y] = (min(255, round(r * k)), min(255, round(g * k)), min(255, round(b * k)), a)
    return out

art_a = black_to_alpha(art)
art_a.save(out / "hero-1115.webp", "WEBP", quality=88, alpha_quality=92, method=6)
art_a.resize((640, int(640 * art.height / art.width)), Image.LANCZOS).save(out / "hero-640.webp", "WEBP", quality=86, alpha_quality=90, method=6)

# 2) cápsula (marca): desenhada em supersampling, magenta + lilás
def capsule(size, bg=True, radius=0.22):
    S = size * 4
    im = Image.new("RGBA", (S, S), (0, 0, 0, 0))
    if bg:
        m = Image.new("L", (S, S), 0); ImageDraw.Draw(m).rounded_rectangle((0, 0, S - 1, S - 1), radius=int(S * radius), fill=255)
        im.paste(Image.new("RGBA", (S, S), (0, 0, 0, 255)), (0, 0), m)
    cap = Image.new("RGBA", (S, S), (0, 0, 0, 0)); d = ImageDraw.Draw(cap)
    L, Wd = int(S * 0.64), int(S * 0.26); cx = cy = S // 2
    x0, y0, x1, y1 = cx - L // 2, cy - Wd // 2, cx + L // 2, cy + Wd // 2
    d.rounded_rectangle((x0, y0, x1, y1), radius=Wd // 2, fill=(233, 213, 255, 255))
    d.rounded_rectangle((x0, y0, cx, y1), radius=Wd // 2, fill=(221, 27, 224, 255))
    d.rectangle((cx - Wd // 2, y0, cx, y1), fill=(221, 27, 224, 255))
    d.rectangle((cx, y0, cx + Wd // 2, y1), fill=(233, 213, 255, 255))
    d.rounded_rectangle((x0, y0, x1, y1), radius=Wd // 2, outline=(20, 0, 30, 255), width=max(2, S // 90))
    d.line((cx, y0, cx, y1), fill=(20, 0, 30, 255), width=max(2, S // 110))
    cap = cap.rotate(35, resample=Image.BICUBIC, center=(cx, cy))
    glow = cap.filter(ImageFilter.GaussianBlur(S * 0.035))
    g = Image.new("RGBA", (S, S), (0, 0, 0, 0)); g.paste((221, 27, 224, 140), (0, 0), glow.split()[3])
    im = Image.alpha_composite(Image.alpha_composite(im, g), cap)
    return im.resize((size, size), Image.LANCZOS)

capsule(512).save(out / "icon-512.png")
capsule(192).save(out / "icon-192.png")
capsule(180, radius=0.0).convert("RGB").save(out / "apple-touch-icon.png")
capsule(64).save(out / "favicon-64.png")

# 3) cartões com a arte à direita e o texto à esquerda: compartilhamento do site, banner do README e prévia social do GitHub
font = next((p for p in ("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", "/usr/share/fonts/dejavu/DejaVuSans-Bold.ttf") if os.path.exists(p)), None)

def card(W, H, title_px, tag_px, small_px, art_h, art_dx, x0, ty, fade0, fade1):
    img = Image.new("RGB", (W, H), (0, 0, 0))
    a = art.copy(); a.thumbnail((art_h, art_h), Image.LANCZOS)
    img.paste(a, (W - a.width + art_dx, (H - a.height) // 2 + 14))
    fade = Image.new("L", (W, H), 0); fd = ImageDraw.Draw(fade)
    for x in range(W): fd.line((x, 0, x, H), fill=int(255 * max(0, min(1, (x - fade0) / fade1))))
    img = Image.composite(img, Image.new("RGB", (W, H), (0, 0, 0)), fade)
    f1, f2, f3 = (ImageFont.truetype(font, n) for n in (title_px, tag_px, small_px)) if font else (ImageFont.load_default(),) * 3
    glow = Image.new("RGB", (W, H), (0, 0, 0)); ImageDraw.Draw(glow).text((x0, ty), "HoodFiles", font=f1, fill=(221, 27, 224))
    glow = glow.filter(ImageFilter.GaussianBlur(16)); img = Image.blend(img, Image.composite(glow, img, glow.convert("L")), 0.9)
    d = ImageDraw.Draw(img)
    d.text((x0, ty), "HoodFiles", font=f1, fill=(255, 255, 255))
    y = ty + int(title_px * 1.18)
    d.text((x0 + 4, y), "Converta arquivos", font=f2, fill=(233, 213, 255))
    d.text((x0 + 4, y + int(tag_px * 1.35)), "no ritmo do arraste.", font=f2, fill=(233, 213, 255))
    y2 = y + int(tag_px * 2.9)
    d.text((x0 + 4, y2), "Local · Windows · macOS · Linux Mint", font=f3, fill=(221, 27, 224))
    d.rectangle((x0 + 4, y2 + int(small_px * 2.1), x0 + 194, y2 + int(small_px * 2.1) + 4), fill=(233, 13, 53))
    d.text((x0 + 4, y2 + int(small_px * 2.6)), "PRÉ-LANÇAMENTO", font=f3, fill=(160, 160, 175))
    return img

card(1200, 630, 112, 31, 25, int(630 * 1.1), 120, 70, 190, 500, 260).save(out / "og.jpg", "JPEG", quality=86, optimize=True, progressive=True)
docs_img = root / "docs" / "img"; docs_img.mkdir(parents=True, exist_ok=True)
card(1600, 520, 104, 30, 24, int(520 * 1.3), 95, 90, 120, 640, 320).save(docs_img / "banner.png", optimize=True)
gh = root / ".github"; gh.mkdir(exist_ok=True)
card(1280, 640, 116, 32, 26, int(640 * 1.1), 120, 74, 192, 520, 270).save(gh / "social-preview.png", optimize=True)   # envie em Settings > General > Social preview

for f in sorted(out.iterdir()): print(f"{f.name:22s} {f.stat().st_size // 1024:4d} KB")
print("docs/img/banner.png", (docs_img / "banner.png").stat().st_size // 1024, "KB;  .github/social-preview.png", (gh / "social-preview.png").stat().st_size // 1024, "KB")
