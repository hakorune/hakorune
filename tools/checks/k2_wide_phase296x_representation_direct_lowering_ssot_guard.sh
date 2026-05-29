#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-298-REPRESENTATION-DIRECT-LOWERING-SSOT.md"
SSOT="$ROOT_DIR/docs/development/current/main/design/representation-direct-lowering-ssot.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-297-MICRO-HELPER-LANE-CLOSEOUT-AND-REPRESENTATION-DIRECT-LOWERING-SELECTION.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row298-representation-direct-ssot] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_text() {
  local file="$1"
  local expected="$2"
  if ! grep -q "$expected" "$file"; then
    echo "[row298-representation-direct-ssot] missing text in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$SSOT" "Status: Active"
require_line "$PREV" "Status: Landed"

require_line "$DOC" "output_contract=representation-direct-lowering-ssot-v0"
require_line "$DOC" "representation_ladder=PublicObject,ExactSlotObject,ResidentScalar,ValueAggregate,NativeDirect"
require_line "$DOC" "runtime_helper_role=fallback_materialization_debug_proof"
require_line "$DOC" "mirbuilder_policy_owner=semantic_ops_and_source_shape_facts_only"
require_line "$DOC" "representation_planner_owner=RepresentationFact_and_RepresentationPlan"
require_line "$DOC" "lowerer_policy_owner=consume_selected_plan_only"
require_line "$DOC" "silent_fallback_allowed=0"
require_line "$DOC" "net_helper_delta_positive_required=1"
require_line "$DOC" "materialization_policy_required=1"
require_line "$DOC" "first_inventory_candidate_0=typed_object_exact_slot_residence"
require_line "$DOC" "first_inventory_candidate_1=result_capsule_value_aggregate"
require_line "$DOC" "first_inventory_candidate_2=array_slot_native_direct"
require_line "$DOC" "selected_next=representation_candidate_inventory"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

require_text "$SSOT" "PublicObject"
require_text "$SSOT" "ExactSlotObject"
require_text "$SSOT" "ResidentScalar"
require_text "$SSOT" "ValueAggregate"
require_text "$SSOT" "NativeDirect"
require_text "$SSOT" "selected plan silently falls back"
require_text "$SSOT" "typed_object_exact_slot_residence"
require_text "$SSOT" "result_capsule_value_aggregate"
require_text "$SSOT" "array_slot_native_direct"

echo "[row298-representation-direct-ssot] ok"
