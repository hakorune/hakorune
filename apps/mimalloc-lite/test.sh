#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
BIN="${HAKORUNE_BIN:-$ROOT_DIR/target/release/hakorune}"
OUT="${TMPDIR:-/tmp}/hakorune_mimalloc_lite.out"
ERR="${TMPDIR:-/tmp}/hakorune_mimalloc_lite.err"

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  "$BIN" --backend vm "$ROOT_DIR/apps/mimalloc-lite/main.hako" >"$OUT" 2>"$ERR"

grep -q '^mimalloc-lite$' "$OUT"
grep -q '^small_allocs=9 frees=3 reused=3 peak=6 free=2$' "$OUT"
grep -q '^medium_allocs=4 frees=1 reused=1 peak=3 free=1$' "$OUT"
grep -q '^requested_bytes=360$' "$OUT"
grep -q '^outstanding=9$' "$OUT"
grep -q '^summary=ok$' "$OUT"

cat "$OUT"
