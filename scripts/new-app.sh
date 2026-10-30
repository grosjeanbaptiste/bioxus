#!/usr/bin/env bash
# New multiplatform Dioxus app from template/: scripts/new-app.sh <name> <dir>
# <name> is the crate name (lowercase, digits, - or _). BIOXUS_TAG overrides the pinned tag.
set -euo pipefail
here="$(cd "$(dirname "$0")/.." && pwd)"
name="${1:?usage: new-app.sh <name> <dir>}"
dir="${2:?usage: new-app.sh <name> <dir>}"
[[ "$name" =~ ^[a-z][a-z0-9_-]*$ ]] || { echo "error: bad crate name '$name'" >&2; exit 1; }
[ ! -e "$dir" ] || { echo "error: $dir exists" >&2; exit 1; }
version="$(sed -n 's/^version = "\(.*\)"/\1/p' "$here/Cargo.toml" | head -1)"
tag="${BIOXUS_TAG:-v$version}"
title="$(echo "$name" | tr '_-' '  ' | awk '{for(i=1;i<=NF;i++)$i=toupper(substr($i,1,1)) substr($i,2)}1')"
id="${name//[_-]/}"
mkdir -p "$dir"
cp -R "$here/template/." "$dir/"
cp "$here/packages/bioxus-design/assets/icon.png" "$dir/assets/icon.png"
while IFS= read -r f; do
  sed -e "s/__NAME__/$name/g" -e "s/__TITLE__/$title/g" -e "s/__ID__/$id/g" \
      -e "s/__TAG__/$tag/g" -e "s/__DATE__/$(date +%F)/g" "$f" > "${f%.tmpl}"
  rm "$f"
done < <(find "$dir" -name '*.tmpl')
ln -s AGENTS.md "$dir/CLAUDE.md"
echo "created $dir (bioxus $tag). Next: cd $dir && make ci-local"
