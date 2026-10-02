#!/usr/bin/env python3
"""Generate the Open Graph image of the landing page (apps/web/public/og-image.png).

Usage:
    pip install pillow fonttools brotli
    python scripts/generate-og-image.py

Inter is read from the self-hosted variable font in apps/web/public/fonts/.
If that file is missing, it is downloaded from the Fontsource CDN.
"""

from io import BytesIO
from pathlib import Path
from urllib.request import urlopen

from fontTools.ttLib import TTFont
from PIL import Image, ImageDraw, ImageFilter, ImageFont

ROOT = Path(__file__).resolve().parent.parent
WEB_PUBLIC = ROOT / "apps" / "web" / "public"
LOGO = WEB_PUBLIC / "verto.png"
FONT = WEB_PUBLIC / "fonts" / "inter-latin-wght-normal.woff2"
FONT_URL = "https://cdn.jsdelivr.net/fontsource/fonts/inter:vf@latest/latin-wght-normal.woff2"
OUT = WEB_PUBLIC / "og-image.png"

W, H = 1200, 630
BG = (24, 24, 27)  # #18181b
ACCENT = (16, 185, 129)  # #10b981
ACCENT_BRIGHT = (52, 211, 153)  # #34d399
TEXT = (244, 244, 245)  # #f4f4f5
TEXT_2 = (161, 161, 170)  # #a1a1aa


def load_inter_ttf() -> bytes:
    """Return Inter as TTF bytes (Pillow cannot always read WOFF2 directly)."""
    raw = FONT.read_bytes() if FONT.exists() else urlopen(FONT_URL).read()
    font = TTFont(BytesIO(raw))
    font.flavor = None
    buf = BytesIO()
    font.save(buf)
    return buf.getvalue()


def inter(ttf: bytes, size: int, weight: int) -> ImageFont.FreeTypeFont:
    font = ImageFont.truetype(BytesIO(ttf), size)
    font.set_variation_by_axes([weight])
    return font


def background() -> Image.Image:
    img = Image.new("RGB", (W, H), BG)

    # Faint grid, like the hero section
    grid = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    gd = ImageDraw.Draw(grid)
    for x in range(0, W, 48):
        gd.line([(x, 0), (x, H)], fill=(255, 255, 255, 7))
    for y in range(0, H, 48):
        gd.line([(0, y), (W, y)], fill=(255, 255, 255, 7))
    img.paste(grid, (0, 0), grid)

    # Emerald glow behind the logo
    glow = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    ImageDraw.Draw(glow).ellipse([80, 115, 520, 515], fill=(*ACCENT, 60))
    glow = glow.filter(ImageFilter.GaussianBlur(90))
    img.paste(glow, (0, 0), glow)
    return img


def main() -> None:
    ttf = load_inter_ttf()
    img = background()
    draw = ImageDraw.Draw(img)

    # Logo mark, scaled to 280px wide
    logo = Image.open(LOGO).convert("RGBA")
    logo_w = 280
    logo = logo.resize((logo_w, round(logo.height * logo_w / logo.width)), Image.LANCZOS)
    img.paste(logo, (160, (H - logo.height) // 2), logo)

    x = 520
    wordmark = inter(ttf, 132, 600)
    headline = inter(ttf, 44, 600)
    sub = inter(ttf, 26, 400)

    draw.text((x - 6, 150), "Verto", font=wordmark, fill=TEXT)

    draw.text((x, 330), "Convert anything.", font=headline, fill=TEXT)
    offset = draw.textlength("Convert anything. ", font=headline)
    draw.text((x + offset, 330), "Locally.", font=headline, fill=ACCENT_BRIGHT)

    draw.text(
        (x, 400),
        "Images, documents, audio and video.\nOffline, open source, no telemetry.",
        font=sub,
        fill=TEXT_2,
        spacing=12,
    )

    # Accent bar at the bottom
    draw.rectangle([0, H - 6, W, H], fill=ACCENT)

    img.save(OUT, "PNG", optimize=True)
    print(f"Wrote {OUT.relative_to(ROOT)} ({OUT.stat().st_size // 1024} KB)")


if __name__ == "__main__":
    main()
