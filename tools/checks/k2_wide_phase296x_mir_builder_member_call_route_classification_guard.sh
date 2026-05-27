#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mir-builder-member-call-route-classification"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_142="docs/development/current/main/phases/phase-296x/296x-142-MIR-BUILDER-MEMBER-CALL-ROUTE-CLASSIFICATION.md"
CARD_143="docs/development/current/main/phases/phase-296x/296x-143-MIR-BUILDER-MEMBER-CALL-ROUTE-PLAN-PILOT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mir_builder_member_call_route_classification_guard.sh"

echo "[$TAG] checking member-call route classification"

guard_require_files "$TAG" "$CARD_142" "$CARD_143" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_142" "row142 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_143" "row143 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=mir-builder-member-call-route-classification-v0' "$CARD_142" "row142 must record output contract"
guard_expect_fixed_in_file "$TAG" 'route_owner=src/mir/builder/calls/member_route.rs' "$CARD_142" "row142 must select route owner"
guard_expect_fixed_in_file "$TAG" 'source_preflight_owner=src/mir/builder/calls/function_preflight.rs' "$CARD_142" "row142 must select function preflight owner"
guard_expect_fixed_in_file "$TAG" 'generic_cse_opened=0' "$CARD_142" "row142 must keep generic CSE closed"
guard_expect_fixed_in_file "$TAG" 'selected_next=mir_builder_member_call_route_plan_pilot' "$CARD_142" "row142 must select route plan pilot"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-142-MIR-BUILDER-MEMBER-CALL-ROUTE-CLASSIFICATION"' "$CURRENT_STATE" "current state latest card must advance to row142"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIR-BUILDER-MEMBER-CALL-ROUTE-PLAN-PILOT-296X-001"' "$CURRENT_STATE" "current state must select row143"
guard_expect_fixed_in_file "$TAG" '| 142 | `MIR-BUILDER-MEMBER-CALL-ROUTE-CLASSIFICATION-296X-001` | Landed |' "$TASKBOARD" "taskboard row142 must be landed"
guard_expect_fixed_in_file "$TAG" '| 143 | `MIR-BUILDER-MEMBER-CALL-ROUTE-PLAN-PILOT-296X-001` | Current |' "$TASKBOARD" "taskboard row143 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

echo "[$TAG] ok"
