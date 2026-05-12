#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

OUT="${TMPDIR:-/tmp}/hakorune_hako_alloc_usize_field_probe.out"
ERR="${TMPDIR:-/tmp}/hakorune_hako_alloc_usize_field_probe.err"

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  cargo run -q --bin hakorune -- --backend vm \
    "$ROOT_DIR/apps/hako-alloc-usize-field-probe/main.hako" >"$OUT" 2>"$ERR"

grep -q '^hako-alloc-usize-field-probe$' "$OUT"
grep -q '^accept=1,1,0$' "$OUT"
grep -q '^state=2,2,2,56$' "$OUT"
grep -q '^shape=7$' "$OUT"
grep -q '^summary=ok$' "$OUT"

cat "$OUT"
