#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

SOURCE="apps/mirbuilder-binding-context-ordered-map-probe/main.hako"
EXE="/tmp/hako_mirbuilder_binding_context_ordered_map_probe"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"

echo "[binding-context-ordered-map/smoke] EXE"
rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED"
rm -f tmp/nyash_cli_emit.json

./target/release/hakorune --emit-exe "$EXE" "$SOURCE" >"$EXE.build.log" 2>&1
"$EXE" >"$RAW" 2>"$EXE.err"
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
binding_count=3
binding[0]=alpha:10
binding[1]=beta:21
binding[2]=gamma:30
lookup.missing=null
summary=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"
echo "[binding-context-ordered-map/smoke] summary=ok"
