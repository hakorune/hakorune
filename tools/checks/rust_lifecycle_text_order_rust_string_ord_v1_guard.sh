#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-text-order-rust-string-ord-v1"
APP="apps/tests/phase296x_text_order_rust_string_ord_v1_min.hako"
LIB="apps/lib/collections/text_order.hako"
MIR="/tmp/hako_text_order_rust_string_ord_v1.mir.json"
EXE="/tmp/hako_text_order_rust_string_ord_v1"
VM_OUT="/tmp/hako_text_order_rust_string_ord_v1.vm.out"
MIR_LOG="/tmp/hako_text_order_rust_string_ord_v1.mir.log"
BUILD_LOG="/tmp/hako_text_order_rust_string_ord_v1.build.log"
EXE_RAW="/tmp/hako_text_order_rust_string_ord_v1.exe.raw"
EXE_OUT="/tmp/hako_text_order_rust_string_ord_v1.exe.out"
EXPECTED="/tmp/hako_text_order_rust_string_ord_v1.expected"

if rg -n 'OrderedMapBox|RegionObserver' "$LIB" "$APP" >/tmp/"$TAG".special_case 2>&1; then
  echo "[$TAG] ERROR: text-order capability must not depend on OrderedMapBox or RegionObserver" >&2
  cat /tmp/"$TAG".special_case >&2
  exit 1
fi

rm -f "$MIR" "$EXE" "$VM_OUT" "$MIR_LOG" "$BUILD_LOG" "$EXE_RAW" "$EXE_OUT" "$EXPECTED"

cargo run -q --features vm-reference --bin hakorune -- --backend vm "$APP" >"$VM_OUT" 2>/tmp/"$TAG".vm.err
./target/release/hakorune --emit-mir-json "$MIR" "$APP" >"$MIR_LOG" 2>&1
./target/release/hakorune --emit-exe "$EXE" "$APP" >"$BUILD_LOG" 2>&1
"$EXE" >"$EXE_RAW" 2>/tmp/"$TAG".exe.err
sed '/^Result: /d' "$EXE_RAW" >"$EXE_OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
text_order_rust_string_ord_v1=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$VM_OUT"
diff -u "$EXPECTED" "$EXE_OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-text-order-rust-string-ord-v1
comparator=RustStringOrdV1
vm_reference=green
mir_emit=green
exe_aot=green
ordered_map_special_case=0
region_observer_special_case=0
summary=ok
REPORT
