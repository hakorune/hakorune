#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
BIN="${HAKORUNE_BIN:-$ROOT_DIR/target/release/hakorune}"
OUT="${TMPDIR:-/tmp}/hakorune_allocator_stress.out"
ERR="${TMPDIR:-/tmp}/hakorune_allocator_stress.err"

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  "$BIN" --backend vm "$ROOT_DIR/apps/allocator-stress/main.hako" >"$OUT" 2>"$ERR"

grep -q '^allocator-stress$' "$OUT"
grep -q '^small_allocs=11 frees=3 reused=3 peak=8 free=0$' "$OUT"
grep -q '^medium_allocs=6 frees=2 reused=2 peak=4 free=0$' "$OUT"
grep -q '^requested_bytes=454$' "$OUT"
grep -q '^outstanding=12$' "$OUT"
grep -q '^rejects=4$' "$OUT"
grep -q '^summary=ok$' "$OUT"

cat "$OUT"
