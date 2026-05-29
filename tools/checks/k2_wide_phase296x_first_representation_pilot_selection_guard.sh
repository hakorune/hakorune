#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-300-FIRST-REPRESENTATION-PILOT-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-299-REPRESENTATION-CANDIDATE-INVENTORY.md"
SSOT="$ROOT_DIR/docs/development/current/main/design/representation-direct-lowering-ssot.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row300-first-representation-pilot-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$SSOT" "Status: Active"
require_line "$DOC" "output_contract=first-representation-pilot-selection-v0"
require_line "$DOC" "input_contract=representation-candidate-inventory-v0"
require_line "$DOC" "candidate_count=3"
require_line "$DOC" "positive_net_candidate_count=2"
require_line "$DOC" "selected_candidate=typed_object_exact_slot_residence"
require_line "$DOC" "selected_candidate_representation=ResidentScalar"
require_line "$DOC" "selected_hot_pct=50.97"
require_line "$DOC" "selected_net_helper_delta=80"
require_line "$DOC" "selected_net_helper_delta_positive=1"
require_line "$DOC" "selected_implementation_risk=high"
require_line "$DOC" "next_row=typed_object_resident_scalar_guard_surface"
require_line "$DOC" "guard_surface_required=1"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "rejected_candidate_0=array_slot_native_direct"
require_line "$DOC" "rejected_reason_0=low_risk_but_small_net_delta_and_direct_op_pipeline_already_proved"
require_line "$DOC" "rejected_candidate_1=result_capsule_value_aggregate"
require_line "$DOC" "rejected_reason_1=net_zero_due_public_method_return_materialization"
require_line "$DOC" "silent_fallback_allowed=0"
require_line "$DOC" "materialization_policy_required=1"
require_line "$DOC" "net_helper_delta_positive_required=1"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

echo "[row300-first-representation-pilot-selection] ok"
