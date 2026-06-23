#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-ordered-map-text-order"
LIB="apps/lib/collections/ordered_map.hako"
APP="apps/lib/collections/tests/ordered_map_smoke.hako"
EXE="/tmp/hako_ordered_map_text_order"
RAW="/tmp/hako_ordered_map_text_order.raw"
OUT="/tmp/hako_ordered_map_text_order.out"
EXPECTED="/tmp/hako_ordered_map_text_order.expected"
BUILD_LOG="/tmp/hako_ordered_map_text_order.build.log"

if ! rg -q 'TextOrder.compare_rust_string_v1' "$LIB"; then
  echo "[$TAG] ERROR: OrderedMapBox must use TextOrder.compare_rust_string_v1" >&2
  exit 1
fi

if rg -n 'if a < b or a == b' "$LIB" >/tmp/"$TAG".local_compare 2>&1; then
  echo "[$TAG] ERROR: OrderedMapBox must not own the local comparator expression" >&2
  cat /tmp/"$TAG".local_compare >&2
  exit 1
fi

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED" "$BUILD_LOG"

./target/release/hakorune --emit-exe "$EXE" "$APP" >"$BUILD_LOG" 2>&1
"$EXE" >"$RAW" 2>/tmp/"$TAG".err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
ordered_map_smoke=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-ordered-map-text-order
ordered_map_uses_text_order=1
ordered_map_local_compare=0
ordered_map_exe=green
summary=ok
REPORT
