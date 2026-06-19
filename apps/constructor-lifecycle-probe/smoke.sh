#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

SOURCE="apps/constructor-lifecycle-probe/main.hako"
EXE="/tmp/hako_constructor_lifecycle_probe"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"

echo "[constructor-lifecycle/smoke] EXE"
rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED"
rm -f tmp/nyash_cli_emit.json

./target/release/hakorune --emit-exe "$EXE" "$SOURCE" >"$EXE.build.log" 2>&1
"$EXE" >"$RAW" 2>"$EXE.err"
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
constructor_lifecycle_probe=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"
echo "[constructor-lifecycle/smoke] summary=ok"
