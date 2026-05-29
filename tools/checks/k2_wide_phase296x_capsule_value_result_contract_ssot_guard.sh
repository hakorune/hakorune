#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-278-CAPSULE-VALUE-RESULT-CONTRACT-SSOT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-277-RESULT-CAPSULE-RECORD-SUCCESS-REPRESENTATION-GUARD-SURFACE.md"
SSOT="$ROOT_DIR/docs/development/current/main/design/capsule-value-result-contract-ssot.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row278-capsule-value-result-contract] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$SSOT" "Status: Active"
require_line "$DOC" "output_contract=capsule-value-result-contract-ssot-v0"
require_line "$DOC" "input_contract=result-capsule-record-success-representation-guard-surface-v0"
require_line "$DOC" "representation_before=ExactSlotObject"
require_line "$DOC" "representation_after=ValueAggregateDelta"
require_line "$DOC" "public_capsule_object_preserved=1"
require_line "$DOC" "hot_update_value_delta_allowed=1"
require_line "$DOC" "observer_materialization_required=1"
require_line "$DOC" "same_module_method_required=1"
require_line "$DOC" "receiver_capsule_type_known_required=1"
require_line "$DOC" "receiver_slot_plan_known_required=1"
require_line "$DOC" "internal_call_count_required=0"
require_line "$DOC" "unknown_escape_required=0"
require_line "$DOC" "stored_into_other_object_required=0"
require_line "$DOC" "returned_as_object_required=0"
require_line "$DOC" "all_observer_boundaries_known_required=1"
require_line "$DOC" "materialization_policy_known_required=1"
require_line "$DOC" "net_helper_delta_positive_required=1"
require_line "$DOC" "selected_next=capsule_value_result_plan_inventory"
require_line "$DOC" "rejected_owner=record_success_helper_fusion_implementation"
require_line "$DOC" "rejected_owner_1=public_capsule_object_erasure"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

require_line "$SSOT" "## CapsuleValueResultPlan"
require_line "$SSOT" "## Eligibility"
require_line "$SSOT" "## Observer Boundaries"
require_line "$SSOT" "## Required Row Order"
require_line "$SSOT" "## Fail-Fast Boundary"

cat <<REPORT
output_contract=capsule-value-result-contract-ssot-v0
representation_after=ValueAggregateDelta
public_capsule_object_preserved=1
selected_next=capsule_value_result_plan_inventory
optimization_open=0
summary=ok
REPORT
