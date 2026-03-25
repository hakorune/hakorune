#!/bin/bash
# phase21_5_perf_kilo_runtime_data_array_route_contract_vm.sh
#
# Contract pin (LLVM-HOT-20 cleanup-8):
# - RuntimeDataBox get/set in kilo main should prefer Array direct int-key route.
# - main IR should use nyash.array.slot_load_hi / nyash.array.set_hih.
# - main IR should not keep nyash.array.set_h or nyash.runtime_data.get_hh / set_hhh on this path.

set -euo pipefail

source "$(dirname "$0")/../../../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase21_5_perf_kilo_runtime_data_array_route_contract_vm"
EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
EMIT_TIMEOUT_SECS="${EMIT_TIMEOUT_SECS:-30}"
MIR_BUILDER="$NYASH_ROOT/tools/ny_mir_builder.sh"
BENCH="$NYASH_ROOT/benchmarks/bench_kilo_kernel_small.hako"
ROUTE_ARRAY_GET="nyash.array.slot_load_hi"
ROUTE_ARRAY_SET="nyash.array.set_hih"
LEGACY_ARRAY_GET="nyash.array.get_hi"
LEGACY_ARRAY_GET_COMPAT="nyash.array.get_hh"
LEGACY_ARRAY_SET="nyash.array.set_hhh"
LEGACY_ARRAY_SET_COMPAT="nyash.array.set_h"
RUNTIME_GET="nyash.runtime_data.get_hh"
RUNTIME_SET="nyash.runtime_data.set_hhh"

if [ ! -x "$EMIT_ROUTE" ]; then
  test_fail "$SMOKE_NAME: emit route helper missing/executable: $EMIT_ROUTE"
  exit 2
fi
if [ ! -f "$MIR_BUILDER" ]; then
  test_fail "$SMOKE_NAME: MIR builder missing: $MIR_BUILDER"
  exit 2
fi
if [ ! -f "$BENCH" ]; then
  test_fail "$SMOKE_NAME: benchmark missing: $BENCH"
  exit 2
fi

tmp_mir="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.mir.json")"
tmp_ir="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.ll")"
tmp_exe="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.exe")"
tmp_log="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.log")"
tmp_main="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.main.ll")"

cleanup() {
  rm -f "$tmp_mir" "$tmp_ir" "$tmp_exe" "$tmp_log" "$tmp_main" >/dev/null 2>&1 || true
}
trap cleanup EXIT

set +e
"$EMIT_ROUTE" --route hako-helper --timeout-secs "$EMIT_TIMEOUT_SECS" --out "$tmp_mir" --input "$BENCH" >"$tmp_log" 2>&1
emit_rc=$?
set -e
if [ "$emit_rc" -ne 0 ]; then
  tail -n 60 "$tmp_log" || true
  test_fail "$SMOKE_NAME: MIR emit failed rc=$emit_rc"
  exit 1
fi

set +e
NYASH_LLVM_FAST=1 \
NYASH_LLVM_FAST_INT="${NYASH_LLVM_FAST_INT:-1}" \
NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
NYASH_LLVM_AUTO_SAFEPOINT=0 \
NYASH_LLVM_DUMP_IR="$tmp_ir" \
bash "$MIR_BUILDER" --in "$tmp_mir" --emit exe -o "$tmp_exe" --quiet >>"$tmp_log" 2>&1
build_rc=$?
set -e
if [ "$build_rc" -ne 0 ]; then
  tail -n 80 "$tmp_log" || true
  test_fail "$SMOKE_NAME: AOT build failed rc=$build_rc"
  exit 1
fi

if [ ! -s "$tmp_ir" ]; then
  test_fail "$SMOKE_NAME: expected IR dump is empty"
  exit 1
fi

if ! extract_ir_entry_function "$tmp_ir" "$tmp_main"; then
  test_fail "$SMOKE_NAME: entry function not found in dumped IR"
  exit 1
fi

count_symbol() {
  local symbol="$1"
  grep -Fc "\"$symbol\"" "$tmp_main" || true
}

array_get_count="$(count_symbol "$ROUTE_ARRAY_GET")"
array_set_count="$(count_symbol "$ROUTE_ARRAY_SET")"
runtime_get_count="$(count_symbol "$RUNTIME_GET")"
runtime_set_count="$(count_symbol "$RUNTIME_SET")"
legacy_array_get_count="$(count_symbol "$LEGACY_ARRAY_GET")"
legacy_array_get_compat_count="$(count_symbol "$LEGACY_ARRAY_GET_COMPAT")"
legacy_array_set_count="$(count_symbol "$LEGACY_ARRAY_SET")"
legacy_array_set_compat_count="$(count_symbol "$LEGACY_ARRAY_SET_COMPAT")"

if [ "$array_get_count" -lt 1 ]; then
  test_fail "$SMOKE_NAME: ${ROUTE_ARRAY_GET} not observed in main"
  exit 1
fi
if [ "$array_set_count" -lt 1 ]; then
  test_fail "$SMOKE_NAME: ${ROUTE_ARRAY_SET} not observed in main"
  exit 1
fi
if [ "$runtime_get_count" -ne 0 ]; then
  test_fail "$SMOKE_NAME: ${RUNTIME_GET} remained in main (${runtime_get_count})"
  exit 1
fi
if [ "$runtime_set_count" -ne 0 ]; then
  test_fail "$SMOKE_NAME: ${RUNTIME_SET} remained in main (${runtime_set_count})"
  exit 1
fi
if [ "$legacy_array_get_count" -ne 0 ]; then
  test_fail "$SMOKE_NAME: legacy ${LEGACY_ARRAY_GET} remained in main (${legacy_array_get_count})"
  exit 1
fi
if [ "$legacy_array_get_compat_count" -ne 0 ]; then
  test_fail "$SMOKE_NAME: legacy ${LEGACY_ARRAY_GET_COMPAT} remained in main (${legacy_array_get_compat_count})"
  exit 1
fi
if [ "$legacy_array_set_count" -ne 0 ]; then
  test_fail "$SMOKE_NAME: legacy ${LEGACY_ARRAY_SET} remained in main (${legacy_array_set_count})"
  exit 1
fi
if [ "$legacy_array_set_compat_count" -ne 0 ]; then
  test_fail "$SMOKE_NAME: legacy ${LEGACY_ARRAY_SET_COMPAT} remained in main (${legacy_array_set_compat_count})"
  exit 1
fi

test_pass "$SMOKE_NAME"
