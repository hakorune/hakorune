#!/bin/bash
# phase21_5_perf_kilo_text_concat_contract_vm.sh
#
# Contract pin (LLVM-HOT-20 structural hotspot):
# - kilo text loop must preserve string concat route in nested append path.
# - main IR should keep concat helper density (concat_hh / concat3_hhh).
# - data set route (runtime_data.set_hhh / array.set_hhh / map.set_hh) must consume concat result
#   and must not fall back to literal 0.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase21_5_perf_kilo_text_concat_contract_vm"
EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
EMIT_TIMEOUT_SECS="${EMIT_TIMEOUT_SECS:-30}"
MIR_BUILDER="$NYASH_ROOT/tools/ny_mir_builder.sh"
BENCH="$NYASH_ROOT/benchmarks/bench_kilo_kernel_small.hako"

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
NYASH_LLVM_HOT_TRACE=1 \
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

awk '
  /^define .*@"main"\(/ { in_main=1 }
  in_main { print }
  in_main && /^}$/ { exit }
' "$tmp_ir" >"$tmp_main"

if ! grep -q '^define .*@"main"' "$tmp_main"; then
  test_fail "$SMOKE_NAME: main function not found in dumped IR"
  exit 1
fi

concat_hh_count="$(grep -c 'nyash.string.concat_hh' "$tmp_main" || true)"
concat3_count="$(grep -c 'nyash.string.concat3_hhh' "$tmp_main" || true)"
concat_total_count=$((concat_hh_count + concat3_count))
if [ "${concat_total_count}" -lt 2 ]; then
  test_fail "$SMOKE_NAME: concat helper density too low in main (expected >=2, got total=${concat_total_count}, concat_hh=${concat_hh_count}, concat3_hhh=${concat3_count})"
  exit 1
fi

if ! grep -q 'nyash.string.indexOf_hh' "$tmp_main"; then
  test_fail "$SMOKE_NAME: main missing indexOf_hh call"
  exit 1
fi

if grep -q 'nyash.any.length_h' "$tmp_main"; then
  test_fail "$SMOKE_NAME: main still contains generic any.length_h route"
  exit 1
fi

if ! grep -Eq '(nyash\.runtime_data\.set_hhh|nyash\.array\.set_hhh|nyash\.array\.set_hih|nyash\.array\.set_hii|nyash\.map\.set_hh)"\(.*%\"(concat_hh_|concat3_hhh_)' "$tmp_main"; then
  test_fail "$SMOKE_NAME: set route does not consume concat result"
  exit 1
fi

if grep -Eq '(nyash\.runtime_data\.set_hhh|nyash\.array\.set_hhh|nyash\.array\.set_hih|nyash\.array\.set_hii)"\(.*i64 0\)' "$tmp_main"; then
  test_fail "$SMOKE_NAME: set route fallback to literal 0 detected"
  exit 1
fi

hot_line="$(grep '\[llvm/hot\] fn=main' "$tmp_log" | tail -n 1 || true)"
if [ -z "$hot_line" ]; then
  test_fail "$SMOKE_NAME: missing llvm hot trace summary for main"
  exit 1
fi

fallback_call="$(printf '%s\n' "$hot_line" | sed -n 's/.*resolve_fallback_call=\([0-9][0-9]*\).*/\1/p')"
if [ -z "$fallback_call" ]; then
  test_fail "$SMOKE_NAME: failed to parse resolve_fallback_call from hot trace"
  exit 1
fi

if [ "$fallback_call" -ne 0 ]; then
  test_fail "$SMOKE_NAME: resolve_fallback_call must stay 0 (got $fallback_call)"
  exit 1
fi

test_pass "$SMOKE_NAME"
