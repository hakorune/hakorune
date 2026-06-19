#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$ROOT_DIR"

SOURCE="apps/lib/collections/tests/ordered_map_smoke.hako"
EXE="/tmp/hako_ordered_map_smoke"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"

echo "[ordered-map/smoke] EXE"
rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED"
rm -f tmp/nyash_cli_emit.json

./target/release/hakorune --emit-exe "$EXE" "$SOURCE" >"$EXE.build.log" 2>&1
"$EXE" >"$RAW" 2>"$EXE.err"
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
ordered_map_smoke=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"
echo "[ordered-map/smoke] summary=ok"
