#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
BIN="${HAKORUNE_BIN:-$ROOT_DIR/target/release/hakorune}"
OUT="${TMPDIR:-/tmp}/hakorune_json_stream_aggregator.out"
ERR="${TMPDIR:-/tmp}/hakorune_json_stream_aggregator.err"

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  "$BIN" --backend vm "$ROOT_DIR/apps/json-stream-aggregator/main.hako" >"$OUT" 2>"$ERR"

grep -q '^json-stream-aggregator$' "$OUT"
grep -q '^events=5$' "$OUT"
grep -q '^users=3$' "$OUT"
grep -q '^ana_bytes=42 ok=2 fail=0$' "$OUT"
grep -q '^bob_bytes=27 ok=1 fail=1$' "$OUT"
grep -q '^cy_bytes=9 ok=1 fail=0$' "$OUT"
grep -q '^total_bytes=78$' "$OUT"
grep -q '^ok=4 fail=1$' "$OUT"
grep -q '^summary=ok$' "$OUT"

cat "$OUT"
