#!/bin/bash
# phase21_5_concat3_assoc_contract_vm.sh
#
# Contract pin (concat3-normalization phase):
# - `.hako` source input with both chain shapes must lower to concat3_hhh on AOT main IR.
# - direct emit route only (hakorune --emit-mir-json), no helper/delegate fallback.
# - standalone direct-emit owner canary; suite promotion is handled separately from
#   the phase29ck boundary owner lane.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase21_5_concat3_assoc_contract_vm"
EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
MIR_BUILDER="$NYASH_ROOT/tools/ny_mir_builder.sh"
FIXTURE="$NYASH_ROOT/apps/tests/phase21_5_concat3_assoc_contract.hako"
PARITY_FIXTURE="$NYASH_ROOT/apps/tests/phase21_5_concat3_semantics_parity_contract.hako"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"

if [ ! -x "$EMIT_ROUTE" ]; then
  test_fail "$SMOKE_NAME: emit route helper missing/executable: $EMIT_ROUTE"
  exit 2
fi
require_smoke_path "$SMOKE_NAME" "MIR builder" "$MIR_BUILDER" || exit 2
require_smoke_path "$SMOKE_NAME" "fixture" "$FIXTURE" || exit 2
require_smoke_path "$SMOKE_NAME" "parity fixture" "$PARITY_FIXTURE" || exit 2
if ! [[ "$RUN_TIMEOUT_SECS" =~ ^[0-9]+$ ]]; then
  test_fail "$SMOKE_NAME: RUN_TIMEOUT_SECS must be integer: $RUN_TIMEOUT_SECS"
  exit 2
fi

tmp_mir="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.mir.json")"
tmp_ir="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.ll")"
tmp_exe="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.exe")"
tmp_main="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.main.ll")"
tmp_log="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.log")"
tmp_vm0="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.vm0.out")"
tmp_vm1="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.vm1.out")"
tmp_expected="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.expected.out")"

cleanup() {
  rm -f "$tmp_mir" "$tmp_ir" "$tmp_exe" "$tmp_main" "$tmp_log" "$tmp_vm0" "$tmp_vm1" "$tmp_expected" >/dev/null 2>&1 || true
}
trap cleanup EXIT

set +e
NYASH_VM_USE_FALLBACK=0 \
NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
NYASH_MIR_CONCAT3_CANON=1 \
"$EMIT_ROUTE" --route direct --timeout-secs "$RUN_TIMEOUT_SECS" --out "$tmp_mir" --input "$FIXTURE" >"$tmp_log" 2>&1
emit_rc=$?
set -e
if [ "$emit_rc" -ne 0 ]; then
  tail -n 80 "$tmp_log" || true
  test_fail "$SMOKE_NAME: direct MIR emit failed rc=$emit_rc"
  exit 1
fi

mir_concat3_count="$(count_mir_call_callee_in_function_json "$tmp_mir" "main" "nyash.string.concat3_hhh")"
mir_concat_hh_count="$(count_mir_call_callee_in_function_json "$tmp_mir" "main" "nyash.string.concat_hh")"

if [ "$mir_concat3_count" = "ERR" ] || [ "$mir_concat_hh_count" = "ERR" ]; then
  test_fail "$SMOKE_NAME: main function not found in MIR JSON"
  exit 1
fi

if [ "$mir_concat3_count" -lt 2 ]; then
  test_fail "$SMOKE_NAME: expected >=2 concat3_hhh in MIR main (got $mir_concat3_count)"
  exit 1
fi

if [ "$mir_concat_hh_count" -ne 0 ]; then
  test_fail "$SMOKE_NAME: concat_hh call remained in MIR main (count=$mir_concat_hh_count)"
  exit 1
fi

set +e
NYASH_LLVM_FAST=1 \
NYASH_LLVM_FAST_INT="${NYASH_LLVM_FAST_INT:-1}" \
NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
NYASH_LLVM_DUMP_IR="$tmp_ir" \
bash "$MIR_BUILDER" --in "$tmp_mir" --emit exe -o "$tmp_exe" --quiet >>"$tmp_log" 2>&1
build_rc=$?
set -e
if [ "$build_rc" -ne 0 ]; then
  tail -n 80 "$tmp_log" || true
  test_fail "$SMOKE_NAME: MIR->AOT build failed rc=$build_rc"
  exit 1
fi

if [ ! -s "$tmp_ir" ]; then
  test_fail "$SMOKE_NAME: expected IR dump is empty"
  exit 1
fi

require_ir_entry_function "$SMOKE_NAME" "$tmp_ir" "$tmp_main" || exit 1

concat3_count="$(count_fixed_pattern_in_file "$tmp_main" 'nyash.string.concat3_hhh')"
concat_hh_count="$(count_fixed_pattern_in_file "$tmp_main" 'nyash.string.concat_hh')"

if [ "$concat3_count" -lt 2 ]; then
  test_fail "$SMOKE_NAME: expected >=2 concat3_hhh in main (got $concat3_count)"
  exit 1
fi

if [ "$concat_hh_count" -ne 0 ]; then
  test_fail "$SMOKE_NAME: concat_hh call remained in main (count=$concat_hh_count)"
  exit 1
fi

cat > "$tmp_expected" <<'EOF'
hakorun
hakorun
x1020
10x20
x30
EOF

set +e
NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
NYASH_MIR_CONCAT3_CANON=0 run_nyash_vm "$PARITY_FIXTURE" >"$tmp_vm0" 2>&1
vm0_rc=$?
NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
NYASH_MIR_CONCAT3_CANON=1 run_nyash_vm "$PARITY_FIXTURE" >"$tmp_vm1" 2>&1
vm1_rc=$?
set -e

if [ "$vm0_rc" -ne 0 ] || [ "$vm1_rc" -ne 0 ]; then
  test_fail "$SMOKE_NAME: parity fixture vm run failed (canon0_rc=$vm0_rc canon1_rc=$vm1_rc)"
  echo "[canon0-output]" >&2
  cat "$tmp_vm0" >&2 || true
  echo "[canon1-output]" >&2
  cat "$tmp_vm1" >&2 || true
  exit 1
fi

if ! diff -u "$tmp_vm0" "$tmp_vm1" >/dev/null 2>&1; then
  test_fail "$SMOKE_NAME: output mismatch between NYASH_MIR_CONCAT3_CANON=0 and =1"
  diff -u "$tmp_vm0" "$tmp_vm1" >&2 || true
  exit 1
fi

if ! diff -u "$tmp_expected" "$tmp_vm0" >/dev/null 2>&1; then
  test_fail "$SMOKE_NAME: parity fixture output drifted from expected contract"
  diff -u "$tmp_expected" "$tmp_vm0" >&2 || true
  exit 1
fi

test_pass "$SMOKE_NAME"
