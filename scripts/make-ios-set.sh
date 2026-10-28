#!/usr/bin/env bash
# Build the 13-file iOS icon set (icon_<n>x<n>.png, RGB, no alpha: App Store rule) from one
# square master image (PNG or SVG). Usage: make-ios-set.sh <master> <out-dir>
# Same file names the apps' embed scripts already read (stores/ios/AppIcon.appiconset).
set -euo pipefail
[ $# -eq 2 ] || { echo "usage: $0 <master.png|svg> <out-dir>" >&2; exit 2; }
command -v magick >/dev/null || { echo "error: magick (ImageMagick) not found" >&2; exit 1; }
master="$1"; out="$2"
[ -f "$master" ] || { echo "error: $master not found" >&2; exit 1; }
mkdir -p "$out"
for n in 20 29 40 58 60 76 80 87 120 152 167 180 1024; do
  magick -background none "$master" -resize "${n}x${n}!" -background "#ffffff" -alpha remove -alpha off "PNG24:$out/icon_${n}x${n}.png"
done
echo "wrote 13 icons in $out"
