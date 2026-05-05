#!/bin/bash
# phase291x_maplookup_fusion_const_fold_contract_llvm.sh
#
# Contract pin:
# - The minimal MapLookupSameKey scalar-constant get/has pair must stay visible
#   in MIR metadata.
# - The lowered entry LLVM IR must not regress to the runtime get/has calls
#   removed by 291x-148 / 291x-149.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase291x_maplookup_fusion_const_fold_contract_llvm"
FIXTURE="$NYASH_ROOT/apps/tests/mir_shape_guard/maplookup_fusion_const_fold_min_v1.mir.json"
MIR_BUILDER="$NYASH_ROOT/tools/ny_mir_builder.sh"

if [ ! -f "$FIXTURE" ]; then
  test_fail "$SMOKE_NAME: fixture missing: $FIXTURE"
  exit 2
fi
if [ ! -f "$MIR_BUILDER" ]; then
  test_fail "$SMOKE_NAME: MIR builder missing: $MIR_BUILDER"
  exit 2
fi

TMP_IR="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.ll")"
TMP_EXE="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.exe")"
TMP_LOG="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.log")"
TMP_MAIN="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.main.ll")"
cleanup() {
  rm -f "$TMP_IR" "$TMP_EXE" "$TMP_LOG" "$TMP_MAIN" >/dev/null 2>&1 || true
}
trap cleanup EXIT

fusion_count="$(jq '[.. | objects | select(.route_id? == "map_lookup.same_key" and (.stored_value_const? != null))] | length' "$FIXTURE")"
if [ "$fusion_count" -lt 1 ]; then
  test_fail "$SMOKE_NAME: MapLookupSameKey route with stored_value_const not observed"
  exit 1
fi

set +e
NYASH_LLVM_FAST=1 \
NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
NYASH_LLVM_DUMP_IR="$TMP_IR" \
bash "$MIR_BUILDER" --in "$FIXTURE" --emit exe -o "$TMP_EXE" --quiet >>"$TMP_LOG" 2>&1
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
folded_one_count="$(grep -Ec ' = add i64 0, 1$' "$TMP_MAIN" 2>/dev/null || true)"

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
if [ "$folded_one_count" -lt 2 ]; then
  test_fail "$SMOKE_NAME: expected get/has const folds were not both emitted"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (MapLookupSameKey const folds pinned)"
