#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mir-builder-member-call-route-plan-pilot"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_143="docs/development/current/main/phases/phase-296x/296x-143-MIR-BUILDER-MEMBER-CALL-ROUTE-PLAN-PILOT.md"
CARD_144="docs/development/current/main/phases/phase-296x/296x-144-MIR-BUILDER-FIELD-PROPERTY-RECEIVER-FACTS-CLEANUP.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mir_builder_member_call_route_plan_pilot_guard.sh"
ROUTE="src/mir/builder/calls/member_route.rs"
PREFLIGHT="src/mir/builder/calls/function_preflight.rs"
BUILD="src/mir/builder/calls/build.rs"
CALLS_MOD="src/mir/builder/calls/mod.rs"
APP_SINGLE_EVAL="apps/mir-single-eval-surface-sweep/main.hako"

echo "[$TAG] checking member-call route plan pilot"

guard_require_files "$TAG" "$CARD_143" "$CARD_144" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$SELF_SCRIPT" "$ROUTE" "$PREFLIGHT" "$BUILD" "$CALLS_MOD" "$APP_SINGLE_EVAL"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_143" "row143 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_144" "row144 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=mir-builder-member-call-route-plan-pilot-v0' "$CARD_143" "row143 must record output contract"
guard_expect_fixed_in_file "$TAG" 'route_plan_owner=src/mir/builder/calls/member_route.rs' "$CARD_143" "row143 must record route owner"
guard_expect_fixed_in_file "$TAG" 'function_preflight_owner=src/mir/builder/calls/function_preflight.rs' "$CARD_143" "row143 must record preflight owner"
guard_expect_fixed_in_file "$TAG" 'generic_cse_opened=0' "$CARD_143" "row143 must keep generic CSE closed"
guard_expect_fixed_in_file "$TAG" 'pub(in crate::mir::builder) enum MemberCallRoutePlan' "$ROUTE" "route plan enum must exist"
guard_expect_fixed_in_file "$TAG" 'fn plan_member_call_route' "$ROUTE" "route planner must exist"
guard_expect_fixed_in_file "$TAG" 'fn emit_member_call_from_plan' "$ROUTE" "route emitter must exist"
guard_expect_fixed_in_file "$TAG" 'fn try_handle_function_preflight' "$PREFLIGHT" "function preflight must exist"
guard_expect_fixed_in_file "$TAG" 'self.try_handle_function_preflight' "$BUILD" "build entry must use preflight"
guard_expect_fixed_in_file "$TAG" 'self.plan_member_call_route' "$BUILD" "build entry must plan member route"
guard_expect_fixed_in_file "$TAG" 'self.emit_member_call_from_plan' "$BUILD" "build entry must emit from plan"
guard_expect_fixed_in_file "$TAG" 'pub mod member_route' "$CALLS_MOD" "calls mod must expose member_route"
guard_expect_fixed_in_file "$TAG" 'pub mod function_preflight' "$CALLS_MOD" "calls mod must expose function_preflight"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-143-MIR-BUILDER-MEMBER-CALL-ROUTE-PLAN-PILOT"' "$CURRENT_STATE" "current state latest card must advance to row143"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIR-BUILDER-FIELD-PROPERTY-RECEIVER-FACTS-CLEANUP-296X-001"' "$CURRENT_STATE" "current state must select row144"
guard_expect_fixed_in_file "$TAG" '| 143 | `MIR-BUILDER-MEMBER-CALL-ROUTE-PLAN-PILOT-296X-001` | Landed |' "$TASKBOARD" "taskboard row143 must be landed"
guard_expect_fixed_in_file "$TAG" '| 144 | `MIR-BUILDER-FIELD-PROPERTY-RECEIVER-FACTS-CLEANUP-296X-001` | Current |' "$TASKBOARD" "taskboard row144 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

summary="$(target/release/hakorune "$APP_SINGLE_EVAL")"
printf '%s\n' "$summary" | grep -Fx 'summary=ok' >/dev/null || {
  echo "[$TAG] ERROR: single-eval surface sweep must end summary=ok" >&2
  printf '%s\n' "$summary" >&2
  exit 1
}

echo "[$TAG] ok"
