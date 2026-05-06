#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
BIN="${HAKORUNE_BIN:-$ROOT_DIR/target/release/hakorune}"
OUT="${TMPDIR:-/tmp}/hakorune_binary_trees.out"
ERR="${TMPDIR:-/tmp}/hakorune_binary_trees.err"

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  "$BIN" --backend vm "$ROOT_DIR/apps/binary-trees/main.hako" >"$OUT" 2>"$ERR"

grep -q '^binary-trees$' "$OUT"
grep -q '^stretch_depth=7 check=-1$' "$OUT"
grep -q '^long_lived_depth=6 check=-1$' "$OUT"
grep -q '^iterations_depth_4=64 check=-128$' "$OUT"
grep -q '^iterations_depth_6=16 check=-32$' "$OUT"
grep -q '^summary=ok$' "$OUT"

cat "$OUT"
