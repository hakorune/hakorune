#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
BIN="${HAKORUNE_BIN:-$ROOT_DIR/target/release/hakorune}"
OUT="${TMPDIR:-/tmp}/hakorune_boxtorrent_mini.out"
ERR="${TMPDIR:-/tmp}/hakorune_boxtorrent_mini.err"

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  "$BIN" --backend vm "$ROOT_DIR/apps/boxtorrent-mini/main.hako" >"$OUT" 2>"$ERR"

grep -q '^boxtorrent-mini$' "$OUT"
grep -q '^root_equal=true$' "$OUT"
grep -q '^roundtrip=true$' "$OUT"
grep -q '^ref_before_release=2$' "$OUT"
grep -q '^ref_after_release=1$' "$OUT"
grep -q '^summary=ok$' "$OUT"

cat "$OUT"
