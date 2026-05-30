#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="k2-wide-phase296x-collection-method-direct-array-lane-semantic-smoke"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_406="docs/development/current/main/phases/phase-296x/296x-406-COLLECTION-METHOD-DIRECT-ARRAY-LANE-SELECTED-METHOD-PILOT.md"
CARD_407="docs/development/current/main/phases/phase-296x/296x-407-COLLECTION-METHOD-DIRECT-ARRAY-LANE-SEMANTIC-SMOKE.md"
CARD_408="docs/development/current/main/phases/phase-296x/296x-408-COLLECTION-METHOD-DIRECT-ARRAY-LANE-POST-SEMANTIC-PERF-OWNER-REFRESH.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_collection_method_direct_array_lane_semantic_smoke_guard.sh"

echo "[$TAG] checking collection-method direct-array lane semantic smoke"

guard_require_files "$TAG" "$CARD_406" "$CARD_407" "$CARD_408" "$STATE" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_406" "row406 must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_407" "row407 must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_408" "row408 must be landed"
guard_expect_fixed_in_file "$TAG" 'output_contract=collection-method-direct-array-lane-semantic-smoke-v0' "$CARD_407" "row407 must define output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=collection-method-direct-array-lane-selected-method-pilot-v0' "$CARD_407" "row407 must consume row406"
guard_expect_fixed_in_file "$TAG" 'selected_method=HakoAllocPageModel.acquire_usize/1' "$CARD_407" "row407 must keep the selected method pinned"
guard_expect_fixed_in_file "$TAG" 'selected_backend=direct_array_i64_exact' "$CARD_407" "row407 must keep the direct-array backend pinned"
guard_expect_fixed_in_file "$TAG" 'receiver_origin_fact_value=ArrayRepr::DirectI64' "$CARD_407" "row407 must consume the explicit DirectI64 fact"
guard_expect_fixed_in_file "$TAG" 'python_lowering_smoke=ok' "$CARD_407" "row407 must include the Python lowering smoke"
guard_expect_fixed_in_file "$TAG" 'rust_direct_array_substrate_smoke=ok' "$CARD_407" "row407 must include the Rust substrate smoke"
guard_expect_fixed_in_file "$TAG" 'selected_next=collection_method_call_direct_array_lane_post_semantic_perf_owner_refresh' "$CARD_407" "row407 must choose the perf owner refresh row next"
guard_expect_fixed_in_file "$TAG" 'shared_route_order_surface=collection_method_call.py' "$CARD_407" "row407 must keep collection_method_call.py as the primary route-order surface"
guard_expect_fixed_in_file "$TAG" 'selected_next=collection_method_call_direct_array_lane_legacy_helper_cache_retirement_selection|collection_method_call_direct_array_lane_post_semantic_perf_owner_refresh' "$CARD_408" "row408 must choose the retirement selection or keep the perf refresh boundary"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "COLLECTION-METHOD-DIRECT-ARRAY-LANE-LEGACY-HELPER-CACHE-RETIREMENT-SELECTION-296X-001"' "$STATE" "current state must point to row409"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-408-COLLECTION-METHOD-DIRECT-ARRAY-LANE-POST-SEMANTIC-PERF-OWNER-REFRESH"' "$STATE" "current state must keep row408 as the latest landed card"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

echo "[$TAG] ok"
