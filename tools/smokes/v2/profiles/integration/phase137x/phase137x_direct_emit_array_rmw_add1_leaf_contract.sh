#!/bin/bash
# phase-137x direct emit smoke for the array RMW add1 leaf exact seed route
#
# Contract:
# 1) direct MIR owns both the inner array RMW window and the whole-function
#    exact seed route for `bench_kilo_leaf_array_rmw_add1.hako`.
# 2) the active AOT lowering consumes the function-level backend tag before the
#    compatibility ladder.
# 3) the entry function lowers through the temporary exact emitter, not through
#    runtime/public array helper calls.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "phase137x_direct_emit_array_rmw_add1_leaf_contract: LLVM backend not available"
    exit 0
fi

SMOKE_NAME="phase137x_direct_emit_array_rmw_add1_leaf_contract"
EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
MIR_BUILDER="$NYASH_ROOT/tools/ny_mir_builder.sh"
INPUT="$NYASH_ROOT/benchmarks/bench_kilo_leaf_array_rmw_add1.hako"
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

metadata = main_fn.get("metadata") or {}
inner_routes = metadata.get("array_rmw_window_routes") or []
if not any(route.get("proof") == "array_get_add1_set_same_slot" for route in inner_routes):
    raise SystemExit("missing inner array_rmw_window route proof")

seed = metadata.get("array_rmw_add1_leaf_seed_route")
if not isinstance(seed, dict):
    raise SystemExit("missing array_rmw_add1_leaf_seed_route")
expected_seed = {
    "size": 128,
    "ops": 2000000,
    "init_push_count": 1,
    "final_get_count": 2,
    "selected_rmw_block": 23,
    "selected_rmw_instruction_index": 8,
    "selected_rmw_set_instruction_index": 13,
    "proof": "kilo_leaf_array_rmw_add1_7block",
    "rmw_proof": "array_get_add1_set_same_slot",
}
for key, expected in expected_seed.items():
    if seed.get(key) != expected:
        raise SystemExit(f"wrong {key}: {seed}")

backend = metadata.get("exact_seed_backend_route")
if not isinstance(backend, dict):
    raise SystemExit("missing exact_seed_backend_route")
if backend.get("tag") != "array_rmw_add1_leaf":
    raise SystemExit(f"wrong exact seed backend route tag: {backend}")
if backend.get("source_route") != "array_rmw_add1_leaf_seed_route":
    raise SystemExit(f"wrong exact seed backend route source: {backend}")
if backend.get("proof") != "kilo_leaf_array_rmw_add1_7block":
    raise SystemExit(f"wrong exact seed backend route proof: {backend}")
if backend.get("selected_value") is not None:
    raise SystemExit(f"unexpected selected_value: {backend}")
PY
then
    echo "[INFO] emitted MIR:"
    sed -n '1,240p' "$OUT_JSON" || true
    test_fail "$SMOKE_NAME: MIR metadata did not expose the array RMW exact backend route"
    exit 1
fi

set +e
NYASH_LLVM_FAST=1 \
NYASH_LLVM_FAST_INT="${NYASH_LLVM_FAST_INT:-1}" \
NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
NYASH_LLVM_ROUTE_TRACE=1 \
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

if ! grep -Fq "stage=exact_seed_backend_route result=hit reason=mir_route_metadata" "$BUILD_LOG"; then
    echo "[INFO] compile log:"
    tail -n 160 "$BUILD_LOG" || true
    test_fail "$SMOKE_NAME: exact seed backend route tag was not consumed"
    exit 1
fi

if ! grep -Fq "stage=array_rmw_add1_leaf result=emit reason=exact_match" "$BUILD_LOG"; then
    echo "[INFO] compile log:"
    tail -n 160 "$BUILD_LOG" || true
    test_fail "$SMOKE_NAME: array RMW add1 leaf exact emitter did not match"
    exit 1
fi

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

if ! grep -Fq "%arr = alloca [128 x i64]" "$OUT_MAIN"; then
    echo "[INFO] lowered entry IR:"
    sed -n '1,220p' "$OUT_MAIN" || true
    test_fail "$SMOKE_NAME: specialized stack array IR not emitted"
    exit 1
fi

for symbol in \
    "nyash.array.rmw_add1_hi" \
    "nyash.runtime_data.get_hh" \
    "nyash.runtime_data.set_hhh" \
    "nyash.array.get_hi" \
    "nyash.array.set_hii"; do
    if [ "$(count_symbol "$symbol")" -ne 0 ]; then
        echo "[INFO] lowered entry IR:"
        sed -n '1,220p' "$OUT_MAIN" || true
        test_fail "$SMOKE_NAME: entry function still lowers through ${symbol}"
        exit 1
    fi
done

if ! grep -Fq "ret i64" "$OUT_MAIN"; then
    echo "[INFO] lowered entry IR:"
    sed -n '1,220p' "$OUT_MAIN" || true
    test_fail "$SMOKE_NAME: lowered entry IR is missing an i64 return"
    exit 1
fi

test_pass "$SMOKE_NAME"
