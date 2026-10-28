#!/usr/bin/env bash
# Regenerate the default BIOXUS icons from their SVGs: the 512 PNG (rounded, with alpha) and
# the 13-file iOS set (full-bleed, no alpha). Fails loudly without rsvg-convert or magick.
set -euo pipefail
for tool in rsvg-convert magick; do
  command -v "$tool" >/dev/null || { echo "error: $tool not found" >&2; exit 1; }
done
root="$(cd "$(dirname "$0")/.." && pwd)"
assets="$root/packages/bioxus-design/assets"
rsvg-convert -w 512 -h 512 "$assets/icon.svg" -o "$assets/icon.png"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
rsvg-convert -w 1024 -h 1024 "$assets/icon-ios.svg" -o "$tmp/master.png"
"$root/scripts/make-ios-set.sh" "$tmp/master.png" "$assets/ios"
echo "wrote $assets/icon.png and $assets/ios/"
