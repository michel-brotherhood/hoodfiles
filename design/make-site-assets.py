#!/usr/bin/env python3
"""Gera os assets do site a partir da arte de identidade (design/identidade-original.webp).
Uso: python3 design/make-site-assets.py   (requer Pillow). Escreve em site/assets/img/.

A arte tem fundo #000 puro; no site ela é fundida ao brilho com mix-blend-mode: screen (ver styles.css).
Paleta: violeta #9506d0/#c00fe3, magenta #dd1be0/#e21779, vermelho #e90d35, lilás #e9d5ff."""
import os, pathlib
from PIL import Image, ImageDraw, ImageFilter, ImageFont

root = pathlib.Path(__file__).resolve().parent.parent
src = Image.open(root / "design" / "identidade-original.webp").convert("RGB")
out = root / "site" / "assets" / "img"
out.mkdir(parents=True, exist_ok=True)

# 1) arte recortada ao conteúdo (com folga), em duas larguras
art = src.crop((70, 160, 1185, 1085))
art.save(out / "hero-1115.webp", "WEBP", quality=84, method=6)
art.resize((640, int(640 * art.height / art.width)), Image.LANCZOS).save(out / "hero-640.webp", "WEBP", quality=82, method=6)

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

# 3) imagem de compartilhamento 1200x630
W, H = 1200, 630
og = Image.new("RGB", (W, H), (0, 0, 0))
a = art.copy(); a.thumbnail((int(H * 1.1), int(H * 1.1)), Image.LANCZOS)
og.paste(a, (W - a.width + 120, (H - a.height) // 2 + 14))
fade = Image.new("L", (W, H), 0); fd = ImageDraw.Draw(fade)
for x in range(W): fd.line((x, 0, x, H), fill=int(255 * max(0, min(1, (x - 500) / 260))))
og = Image.composite(og, Image.new("RGB", (W, H), (0, 0, 0)), fade)
font = next((p for p in ("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", "/usr/share/fonts/dejavu/DejaVuSans-Bold.ttf") if os.path.exists(p)), None)
f1, f2, f3 = (ImageFont.truetype(font, n) for n in (112, 31, 25)) if font else (ImageFont.load_default(),) * 3
glow = Image.new("RGB", (W, H), (0, 0, 0)); ImageDraw.Draw(glow).text((70, 190), "HoodFiles", font=f1, fill=(221, 27, 224))
glow = glow.filter(ImageFilter.GaussianBlur(16)); og = Image.blend(og, Image.composite(glow, og, glow.convert("L")), 0.9)
d = ImageDraw.Draw(og)
d.text((70, 190), "HoodFiles", font=f1, fill=(255, 255, 255))
d.text((74, 330), "Converta arquivos", font=f2, fill=(233, 213, 255))
d.text((74, 372), "no ritmo do arraste.", font=f2, fill=(233, 213, 255))
d.text((74, 430), "Local · Windows · macOS · Linux Mint", font=f3, fill=(221, 27, 224))
d.rectangle((74, 484, 74 + 190, 488), fill=(233, 13, 53))
d.text((74, 506), "PRÉ-LANÇAMENTO", font=f3, fill=(160, 160, 175))
og.save(out / "og.jpg", "JPEG", quality=86, optimize=True, progressive=True)

for f in sorted(out.iterdir()): print(f"{f.name:22s} {f.stat().st_size // 1024:4d} KB")
