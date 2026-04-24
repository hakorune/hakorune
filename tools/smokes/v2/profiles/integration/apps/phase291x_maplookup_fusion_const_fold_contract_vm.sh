#!/bin/bash
# phase291x_maplookup_fusion_const_fold_contract_vm.sh
#
# Contract pin:
# - The measured MapLookupSameKey scalar-constant get/has pair must stay
#   visible in MIR metadata.
# - The lowered entry LLVM IR must not regress to the runtime get/has calls
#   removed by 291x-148 / 291x-149.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase291x_maplookup_fusion_const_fold_contract_vm"
BENCH="$NYASH_ROOT/benchmarks/bench_kilo_leaf_map_getset_has.hako"
MIR_BUILDER="$NYASH_ROOT/tools/ny_mir_builder.sh"
HAKO_BIN="${NYASH_BIN:-$NYASH_ROOT/target/release/hakorune}"

if [ ! -f "$BENCH" ]; then
  test_fail "$SMOKE_NAME: benchmark missing: $BENCH"
  exit 2
fi
if [ ! -x "$HAKO_BIN" ]; then
  test_fail "$SMOKE_NAME: hakorune binary missing: $HAKO_BIN"
  exit 2
fi
if [ ! -f "$MIR_BUILDER" ]; then
  test_fail "$SMOKE_NAME: MIR builder missing: $MIR_BUILDER"
  exit 2
fi

TMP_MIR="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.mir.json")"
TMP_IR="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.ll")"
TMP_EXE="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.exe")"
TMP_LOG="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.log")"
TMP_MAIN="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.main.ll")"
cleanup() {
  rm -f "$TMP_MIR" "$TMP_IR" "$TMP_EXE" "$TMP_LOG" "$TMP_MAIN" >/dev/null 2>&1 || true
}
trap cleanup EXIT

set +e
"$HAKO_BIN" --emit-mir-json "$TMP_MIR" "$BENCH" >"$TMP_LOG" 2>&1
emit_rc=$?
set -e
if [ "$emit_rc" -ne 0 ]; then
  tail -n 60 "$TMP_LOG" || true
  test_fail "$SMOKE_NAME: direct MIR emit failed rc=$emit_rc"
  exit 1
fi

fusion_count="$(jq '[.. | objects | select(.route_id? == "map_lookup.same_key" and (.stored_value_const? != null))] | length' "$TMP_MIR")"
if [ "$fusion_count" -lt 1 ]; then
  test_fail "$SMOKE_NAME: MapLookupSameKey route with stored_value_const not observed"
  exit 1
fi

set +e
NYASH_LLVM_FAST=1 \
NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
NYASH_LLVM_DUMP_IR="$TMP_IR" \
bash "$MIR_BUILDER" --in "$TMP_MIR" --emit exe -o "$TMP_EXE" --quiet >>"$TMP_LOG" 2>&1
build_rc=$?
set -e
if [ "$build_rc" -ne 0 ]; then
  tail -n 80 "$TMP_LOG" || true
  test_fail "$SMOKE_NAME: AOT build failed rc=$build_rc"
  exit 1
fi

if [ ! -s "$TMP_IR" ]; then
  test_fail "$SMOKE_NAME: expected IR dump is empty"
  exit 1
fi

if ! extract_ir_entry_function "$TMP_IR" "$TMP_MAIN"; then
  test_fail "$SMOKE_NAME: entry function not found in dumped IR"
  exit 1
fi

count_symbol() {
  local symbol="$1"
  local unquoted quoted
  unquoted="$(grep -Fc "@${symbol}(" "$TMP_MAIN" 2>/dev/null || true)"
  quoted="$(grep -Fc "@\"${symbol}\"(" "$TMP_MAIN" 2>/dev/null || true)"
  echo $((unquoted + quoted))
}

runtime_get_count="$(count_symbol "nyash.runtime_data.get_hh")"
map_has_count="$(count_symbol "nyash.map.has_h")"
map_probe_count="$(count_symbol "nyash.map.probe_hi")"

if [ "$runtime_get_count" -ne 0 ]; then
  test_fail "$SMOKE_NAME: nyash.runtime_data.get_hh remained in entry IR ($runtime_get_count)"
  exit 1
fi
if [ "$map_has_count" -ne 0 ]; then
  test_fail "$SMOKE_NAME: nyash.map.has_h remained in entry IR ($map_has_count)"
  exit 1
fi
if [ "$map_probe_count" -ne 0 ]; then
  test_fail "$SMOKE_NAME: nyash.map.probe_hi remained in entry IR ($map_probe_count)"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (MapLookupSameKey const folds pinned)"
