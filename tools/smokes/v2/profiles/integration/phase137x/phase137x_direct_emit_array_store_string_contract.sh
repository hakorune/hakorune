#!/bin/bash
# phase-137x direct emit smoke for the current array-store string route
#
# Contract:
# 1) the trustworthy direct MIR probe (`emit_mir_route.sh --route direct`) still
#    emits generic `RuntimeDataBox.set(...)` for
#    `bench_kilo_micro_array_string_store.hako`.
# 2) the active AOT lowering still concretizes that call to
#    `nyash.array.set_his` in the entry function.
# 3) the entry function no longer lowers this front through
#    `nyash.runtime_data.set_hhh` or `nyash.array.slot_store_hih`.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "phase137x_direct_emit_array_store_string_contract: LLVM backend not available"
    exit 0
fi

SMOKE_NAME="phase137x_direct_emit_array_store_string_contract"
EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
MIR_BUILDER="$NYASH_ROOT/tools/ny_mir_builder.sh"
INPUT="$NYASH_ROOT/benchmarks/bench_kilo_micro_array_string_store.hako"
OUT_DIR="$NYASH_ROOT/target/smokes/phase137x"
OUT_JSON="$OUT_DIR/${SMOKE_NAME}.$$.$RANDOM.json"
OUT_EXE="${TMPDIR:-/tmp}/${SMOKE_NAME}_$$.exe"
OUT_LL="${TMPDIR:-/tmp}/${SMOKE_NAME}_$$.ll"
OUT_MAIN="${TMPDIR:-/tmp}/${SMOKE_NAME}_$$.main.ll"
BUILD_LOG="${TMPDIR:-/tmp}/${SMOKE_NAME}_$$.log"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-120}"

cleanup() {
    rm -f "$OUT_JSON" "$OUT_EXE" "$OUT_LL" "$OUT_MAIN" "$BUILD_LOG"
}
trap cleanup EXIT

mkdir -p "$OUT_DIR"

require_smoke_path "$SMOKE_NAME" "emit route helper" "$EMIT_ROUTE" executable || exit 1
require_smoke_path "$SMOKE_NAME" "MIR builder" "$MIR_BUILDER" || exit 1
require_smoke_path "$SMOKE_NAME" "benchmark input" "$INPUT" || exit 1

if ! timeout "$RUN_TIMEOUT_SECS" "$EMIT_ROUTE" \
    --route direct \
    --timeout-secs "$RUN_TIMEOUT_SECS" \
    --out "$OUT_JSON" \
    --input "$INPUT"; then
    test_fail "$SMOKE_NAME: direct emit route failed"
    exit 1
fi

require_smoke_path "$SMOKE_NAME" "output json" "$OUT_JSON" || exit 1

if ! python3 - <<'PY' "$OUT_JSON"
import json
import sys

path = sys.argv[1]
with open(path, "r", encoding="utf-8") as f:
    data = json.load(f)

main_fn = next((fn for fn in data.get("functions", []) if fn.get("name") == "main"), None)
if main_fn is None:
    raise SystemExit("missing main function")

runtime_data_set = 0
runtime_data_substring = 0

for block in main_fn.get("blocks", []):
    for inst in block.get("instructions", []):
        if inst.get("op") != "mir_call":
            continue
        call = inst.get("mir_call") or {}
        callee = call.get("callee") or {}
        if callee.get("type") != "Method":
            continue
        if callee.get("box_name") != "RuntimeDataBox":
            continue
        name = callee.get("name") or callee.get("method")
        if name == "set":
            runtime_data_set += 1
        if name == "substring":
            runtime_data_substring += 1

if runtime_data_set < 1:
    raise SystemExit("missing RuntimeDataBox.set in direct MIR")
if runtime_data_substring < 1:
    raise SystemExit("missing RuntimeDataBox.substring in direct MIR")
PY
then
    echo "[INFO] emitted MIR:"
    sed -n '1,240p' "$OUT_JSON" || true
    test_fail "$SMOKE_NAME: direct MIR no longer matches the current generic RuntimeDataBox contract"
    exit 1
fi

set +e
NYASH_LLVM_FAST=1 \
NYASH_LLVM_FAST_INT="${NYASH_LLVM_FAST_INT:-1}" \
NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
NYASH_LLVM_AUTO_SAFEPOINT=0 \
NYASH_LLVM_DUMP_IR="$OUT_LL" \
bash "$MIR_BUILDER" --in "$OUT_JSON" --emit exe -o "$OUT_EXE" --quiet >"$BUILD_LOG" 2>&1
BUILD_RC=$?
set -e

if [ "$BUILD_RC" -ne 0 ]; then
    echo "[INFO] compile log:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "$SMOKE_NAME: AOT build failed rc=$BUILD_RC"
    exit 1
fi

require_smoke_path "$SMOKE_NAME" "LLVM IR dump" "$OUT_LL" || exit 1

if ! extract_ir_entry_function "$OUT_LL" "$OUT_MAIN"; then
    test_fail "$SMOKE_NAME: entry function not found in dumped IR"
    exit 1
fi

count_symbol() {
    local symbol="$1"
    local unquoted quoted
    unquoted="$(grep -Fc "@${symbol}(" "$OUT_MAIN" 2>/dev/null || true)"
    quoted="$(grep -Fc "@\"${symbol}\"(" "$OUT_MAIN" 2>/dev/null || true)"
    echo $((unquoted + quoted))
}

array_set_string_count="$(count_symbol "nyash.array.set_his")"
array_set_any_count="$(count_symbol "nyash.array.slot_store_hih")"
runtime_set_count="$(count_symbol "nyash.runtime_data.set_hhh")"

if [ "$array_set_string_count" -lt 1 ]; then
    echo "[INFO] lowered entry IR:"
    sed -n '1,220p' "$OUT_MAIN" || true
    test_fail "$SMOKE_NAME: entry function did not lower through nyash.array.set_his"
    exit 1
fi

if [ "$array_set_any_count" -ne 0 ]; then
    echo "[INFO] lowered entry IR:"
    sed -n '1,220p' "$OUT_MAIN" || true
    test_fail "$SMOKE_NAME: entry function still lowers through nyash.array.slot_store_hih"
    exit 1
fi

if [ "$runtime_set_count" -ne 0 ]; then
    echo "[INFO] lowered entry IR:"
    sed -n '1,220p' "$OUT_MAIN" || true
    test_fail "$SMOKE_NAME: entry function still lowers through nyash.runtime_data.set_hhh"
    exit 1
fi

test_pass "$SMOKE_NAME: PASS (direct MIR stays generic, but active AOT lowering concretizes the array string-store call to nyash.array.set_his)"
