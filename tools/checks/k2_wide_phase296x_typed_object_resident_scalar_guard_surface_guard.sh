#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-301-TYPED-OBJECT-RESIDENT-SCALAR-GUARD-SURFACE.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-300-FIRST-REPRESENTATION-PILOT-SELECTION.md"
SSOT="$ROOT_DIR/docs/development/current/main/design/representation-direct-lowering-ssot.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row301-typed-object-resident-scalar-guard] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$SSOT" "Status: Active"
require_line "$DOC" "output_contract=typed-object-resident-scalar-guard-surface-v0"
require_line "$DOC" "input_contract=first-representation-pilot-selection-v0"
require_line "$DOC" "selected_family=typed_object_exact_slot_residence"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "current_representation=ExactSlotObject"
require_line "$DOC" "candidate_representation=ResidentScalar"
require_line "$DOC" "selected_method_helper_ops_before=21"
require_line "$DOC" "planned_erased_helper_ops=21"
require_line "$DOC" "planned_materialization_ops_added=0"
require_line "$DOC" "planned_net_helper_delta=21"
require_line "$DOC" "planned_net_helper_delta_positive=1"
require_line "$DOC" "dynamic_planned_net_helper_delta=11010048"
require_line "$DOC" "storage_or_slot_proven=1"
require_line "$DOC" "unknown_call_barrier_policy=materialize_or_no_plan"
require_line "$DOC" "observer_return_barrier_policy=materialize_only_if_net_positive"
require_line "$DOC" "writeback_policy=forbidden_unless_positive_net_after_writeback"
require_line "$DOC" "selected_plan_silent_fallback_allowed=0"
require_line "$DOC" "materialization_policy_required=1"
require_line "$DOC" "previous_block_local_residence_zero_net_guard=1"
require_line "$DOC" "generic_typed_field_residence_retry=0"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "selected_next=typed_object_resident_scalar_selected_method_plan"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

echo "[row301-typed-object-resident-scalar-guard] ok"
