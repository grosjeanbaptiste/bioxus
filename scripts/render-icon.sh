#!/usr/bin/env bash
# Regenerate the default app icon PNG from its SVG. Fails loudly without rsvg-convert.
set -euo pipefail
command -v rsvg-convert >/dev/null || { echo "error: rsvg-convert not found (brew formula librsvg)" >&2; exit 1; }
dir="$(cd "$(dirname "$0")/.." && pwd)/packages/bioxus-design/assets"
rsvg-convert -w 512 -h 512 "$dir/icon.svg" -o "$dir/icon.png"
echo "wrote $dir/icon.png"
