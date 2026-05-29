#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-339-DIRECT-SLOT-NATIVEDIRECT-LOWERING-DAILY-OWNER-GAP-DIAGNOSTIC.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-338-DIRECT-SLOT-NATIVEDIRECT-LOWERING-SELECTED-METHOD-PILOT.md"
NYLL_README="$ROOT_DIR/crates/nyash-llvm-compiler/README.md"
LLVM_PY_README="$ROOT_DIR/src/llvm_py/README.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row339-direct-slot-daily-owner-gap] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row339-direct-slot-daily-owner-gap] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-nativedirect-lowering-daily-owner-gap-diagnostic-v0"
require_line "$DOC" "input_contract=direct-slot-nativedirect-lowering-selected-method-pilot-v0"
require_line "$DOC" "daily_exact_exe_owner=ny_llvmc_boundary_route"
require_line "$DOC" "row338_owner=llvmlite_keep_lane_field_access_py"
require_line "$DOC" "daily_owner_gap_detected=1"
require_line "$DOC" "observed_failure=exact_exe_trap_before_semantic_report"
require_line "$DOC" "failure_reason=boundary_route_still_emits_exact_slot_helpers_for_direct_slot_handles"
require_line "$DOC" "python_lowering_is_not_daily_owner=1"
require_line "$DOC" "measurement_acceptance=blocked"
require_line "$DOC" "selected_next=boundary_route_selected_method_nativedirect_lowering_pilot"
require_line "$DOC" "selected_owner_family=ny_llvmc_boundary_route_selected_method_nativedirect_lowering"
require_line "$DOC" "summary=ok"

require_pattern "$NYLL_README" 'daily mainline: `ny-llvmc` の default boundary route'
require_pattern "$LLVM_PY_README" 'ny-llvmc` の daily caller route はここを通らない'
require_pattern "$LLVM_PY_README" '`src/llvm_py/**` は current daily owner ではなく'

echo "[row339-direct-slot-daily-owner-gap] ok"
