#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-277-RESULT-CAPSULE-RECORD-SUCCESS-REPRESENTATION-GUARD-SURFACE.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-276-RESULT-CAPSULE-OWNER-SELECTION-AFTER-RELEASE-KNOWN-LIVE-ROLLBACK.md"
SSOT="$ROOT_DIR/docs/development/current/main/design/result-capsule-value-representation-ssot.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row277-result-capsule-representation] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$SSOT" "Status: Active"
require_line "$DOC" "output_contract=result-capsule-record-success-representation-guard-surface-v0"
require_line "$DOC" "input_contract=result-capsule-owner-selection-after-release-known-live-rollback-v0"
require_line "$DOC" "current_representation=ExactSlotObject"
require_line "$DOC" "candidate_representation_1=ValueAggregate"
require_line "$DOC" "record_success_combined_field_op_count=14"
require_line "$DOC" "internal_call_count=0"
require_line "$DOC" "identity_observed=1"
require_line "$DOC" "unknown_escape=0"
require_line "$DOC" "observer_boundary_count=4"
require_line "$DOC" "materialization_required=1"
require_line "$DOC" "helper_fusion_net_delta=12"
require_line "$DOC" "helper_fusion_net_delta_positive=1"
require_line "$DOC" "value_aggregate_net_delta_known=0"
require_line "$DOC" "value_aggregate_requires_contract=1"
require_line "$DOC" "selected_next=capsule_value_result_contract_ssot"
require_line "$DOC" "rejected_owner=record_success_helper_fusion_implementation"
require_line "$DOC" "rejected_owner_1=generic_typed_field_residence_retry"
require_line "$DOC" "rejected_owner_2=source_inline_success_result_fast_path"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

require_line "$SSOT" "## Representation Ladder"
require_line "$SSOT" "## Decision Rules"
require_line "$SSOT" "## Current Phase-296x Decision"

cat <<REPORT
output_contract=result-capsule-record-success-representation-guard-surface-v0
current_representation=ExactSlotObject
candidate_representation=ValueAggregate
helper_fusion_net_delta=12
value_aggregate_requires_contract=1
selected_next=capsule_value_result_contract_ssot
optimization_open=0
summary=ok
REPORT
