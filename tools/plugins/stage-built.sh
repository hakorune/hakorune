#!/usr/bin/env bash
set -euo pipefail

ROOT="${NYASH_ROOT:-$(git rev-parse --show-toplevel 2>/dev/null || pwd)}"
cd "$ROOT"

stage_so() {
  local src="$1"
  local file_name
  file_name="$(basename "$src")"
  local crate_base="${file_name#lib}"
  crate_base="${crate_base%.so}"
  local crate_dir="${crate_base//_/-}"
  local dst_dir="plugins/${crate_dir}"
  mkdir -p "$dst_dir"
  cp -f "$src" "$dst_dir/$file_name"
  echo "[plugins/stage-built] staged: $dst_dir/$file_name" >&2
}

shopt -s nullglob
for so in target/release/libnyash_*_plugin.so; do
  stage_so "$so"
done
