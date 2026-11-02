#!/usr/bin/env python3
"""Generate RapidTally Ace image assets (icons + README banner) from the source logo.

Usage:  python3 tools/gen_assets.py --src path/to/logo.png --out assets/
"""
import argparse
import colorsys
import os
import shutil

from PIL import Image, ImageDraw, ImageFont

SIZES = [512, 256, 128, 64, 32]
FONT_CANDIDATES = [
    "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
    "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
]


def find_font(bold: bool):
    paths = FONT_CANDIDATES
    if bold:
        paths = [p for p in FONT_CANDIDATES if "Bold" in p] + FONT_CANDIDATES
    for p in paths:
        if os.path.exists(p):
            return p
    return None


def dominant_colors(flat: Image.Image):
    """Return (teal, orange) RGB tuples sampled from the logo, with fallbacks."""
    small = flat.resize((64, 64))
    px = small.load()
    buckets = {"teal": [], "orange": []}
    for y in range(64):
        for x in range(64):
            r, g, b = px[x, y][:3]
            h, l, s = colorsys.rgb_to_hls(r / 255, g / 255, b / 255)
            if s < 0.4 or not (0.25 < l < 0.75):
                continue
            hue = int(h * 360)
            if 140 <= hue <= 200:
                buckets["teal"].append((r, g, b))
            elif 5 <= hue <= 45:
                buckets["orange"].append((r, g, b))

    def avg(items, fallback):
        if not items:
            return fallback
        n = len(items)
        return (sum(c[0] for c in items) // n, sum(c[1] for c in items) // n, sum(c[2] for c in items) // n)

    return avg(buckets["teal"], (16, 128, 120)), avg(buckets["orange"], (226, 90, 60))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--src", required=True, help="source logo PNG")
    ap.add_argument("--out", required=True, help="output assets directory")
    args = ap.parse_args()

    os.makedirs(args.out, exist_ok=True)
    src = Image.open(args.src).convert("RGBA")
    w, h = src.size

    # Background color sampled from a corner pixel (cream in the project logo).
    corner = src.getpixel((min(3, w - 1), min(3, h - 1)))
    bg = (corner[0], corner[1], corner[2])

    # Square canvas with the logo centered (logo is already square, but be safe).
    side = max(w, h)
    square = Image.new("RGBA", (side, side), bg + (255,))
    square.alpha_composite(src, ((side - w) // 2, (side - h) // 2))
    flat = square.convert("RGB")

    # Original logo, copied verbatim.
    shutil.copyfile(args.src, os.path.join(args.out, "logo.png"))

    # Icon set.
    for size in SIZES:
        flat.resize((size, size), Image.LANCZOS).save(
            os.path.join(args.out, f"icon-{size}.png")
        )

    # README banner.
    teal, orange = dominant_colors(flat)
    BW, BH = 1280, 400
    banner = Image.new("RGB", (BW, BH), bg)
    draw = ImageDraw.Draw(banner)

    logo = flat.resize((300, 300), Image.LANCZOS)
    banner.paste(logo, (56, 50))

    bold_path = find_font(bold=True)
    reg_path = find_font(bold=False)
    f_title = ImageFont.truetype(bold_path, 92) if bold_path else ImageFont.load_default()
    f_sub = ImageFont.truetype(reg_path, 33) if reg_path else ImageFont.load_default()

    title = "RapidTally Ace"
    subtitle_text = "A clean-room accounting suite - phase 0 read-path spike"
    draw.text((404, 112), title, font=f_title, fill=teal)
    bbox = draw.textbbox((404, 112), title, font=f_title)
    underline_w = min(bbox[2] - bbox[0], BW - 404 - 40)
    draw.rectangle([404, 218, 404 + underline_w, 226], fill=orange)

    # Auto-fit the subtitle inside the banner.
    max_w = BW - 404 - 40
    sub_size = 33
    while sub_size > 16:
        f_sub_try = (
            ImageFont.truetype(reg_path, sub_size) if reg_path else ImageFont.load_default()
        )
        sb = draw.textbbox((0, 0), subtitle_text, font=f_sub_try)
        if sb[2] - sb[0] <= max_w:
            f_sub = f_sub_try
            break
        sub_size -= 1
    draw.text((404, 248), subtitle_text, font=f_sub, fill=(70, 66, 58))

    banner.save(os.path.join(args.out, "banner.png"))
    print(f"assets written to {args.out}: logo.png, icons {SIZES}, banner.png")
    print(f"sampled colors: teal={teal}, orange={orange}")


if __name__ == "__main__":
    main()
