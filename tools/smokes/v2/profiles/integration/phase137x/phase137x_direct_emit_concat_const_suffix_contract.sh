#!/bin/bash
# phase-137x direct emit smoke for the concat-const-suffix exact seed route
#
# Contract:
# 1) direct MIR owns the exact seed payload and function-level backend tag for
#    `bench_kilo_micro_concat_const_suffix.hako`.
# 2) the active AOT lowering consumes the function-level tag before the
#    compatibility ladder.
# 3) the entry function lowers through the temporary exact emitter, not through
#    runtime/public string helpers.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "phase137x_direct_emit_concat_const_suffix_contract: LLVM backend not available"
    exit 0
fi

SMOKE_NAME="phase137x_direct_emit_concat_const_suffix_contract"
EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
MIR_BUILDER="$NYASH_ROOT/tools/ny_mir_builder.sh"
INPUT="$NYASH_ROOT/benchmarks/bench_kilo_micro_concat_const_suffix.hako"
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
route = metadata.get("concat_const_suffix_micro_seed_route")
if not isinstance(route, dict):
    raise SystemExit("missing concat_const_suffix_micro_seed_route")
if route.get("proof") != "kilo_micro_concat_const_suffix_5block":
    raise SystemExit("wrong concat_const_suffix proof")
if route.get("seed_len") != 16 or route.get("suffix_len") != 2 or route.get("ops") != 600000:
    raise SystemExit("wrong concat_const_suffix payload")

backend = metadata.get("exact_seed_backend_route")
if not isinstance(backend, dict):
    raise SystemExit("missing exact_seed_backend_route")
if backend.get("tag") != "concat_const_suffix_micro":
    raise SystemExit("wrong exact seed backend route tag")
if backend.get("source_route") != "concat_const_suffix_micro_seed_route":
    raise SystemExit("wrong exact seed backend route source")
if backend.get("proof") != "kilo_micro_concat_const_suffix_5block":
    raise SystemExit("wrong exact seed backend route proof")
PY
then
    echo "[INFO] emitted MIR:"
    sed -n '1,240p' "$OUT_JSON" || true
    test_fail "$SMOKE_NAME: MIR metadata did not expose the concat exact backend route"
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

if ! grep -Fq "stage=concat_const_suffix_micro result=emit reason=exact_match" "$BUILD_LOG"; then
    echo "[INFO] compile log:"
    tail -n 160 "$BUILD_LOG" || true
    test_fail "$SMOKE_NAME: concat const-suffix exact emitter did not match"
    exit 1
fi

if ! extract_ir_entry_function "$OUT_LL" "$OUT_MAIN"; then
    test_fail "$SMOKE_NAME: entry function not found in dumped IR"
    exit 1
fi

for symbol in \
    "nyash.string.concat_hs" \
    "nyash.string.substring_hii" \
    "nyash.string.len_h" \
    "nyash.string.piecewise_subrange_hsiii"; do
    if grep -Fq "@${symbol}" "$OUT_MAIN" || grep -Fq "@\"${symbol}\"" "$OUT_MAIN"; then
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

test_pass "$SMOKE_NAME: PASS (concat const-suffix exact seed is selected by function-level backend route tag)"
