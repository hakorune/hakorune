#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="k2-wide-phase296x-collection-method-route-order-inventory"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_403="docs/development/current/main/phases/phase-296x/296x-403-COLLECTION-METHOD-ROUTE-ORDER-INVENTORY.md"
CARD_404="docs/development/current/main/phases/phase-296x/296x-404-COLLECTION-METHOD-DIRECT-ARRAY-LANE-OWNER-SELECTION.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_collection_method_route_order_inventory_guard.sh"

echo "[$TAG] checking Collection Method route order inventory"

guard_require_files "$TAG" "$CARD_403" "$CARD_404" "$STATE" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_403" "row403 must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_404" "row404 must be landed"
guard_expect_fixed_in_file "$TAG" 'output_contract=collection-method-route-order-inventory-v0' "$CARD_403" "row403 must define output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=collection-method-surface-owner-selection-v0' "$CARD_403" "row403 must consume row402"
guard_expect_fixed_in_file "$TAG" 'shared_collection_route_order_is_highest_leverage=1' "$CARD_403" "row403 must keep the shared route order highest leverage"
guard_expect_fixed_in_file "$TAG" 'direct_array_lane_exact_only=1' "$CARD_403" "row403 must keep the direct array lane exact-only"
guard_expect_fixed_in_file "$TAG" 'selected_next=collection_method_call_direct_array_lane_owner_selection' "$CARD_403" "row403 must choose the direct-array lane owner selection"
guard_expect_fixed_in_file "$TAG" 'selected_reason=the_shared_collection_route_order_is_now_pinned_enough_to_narrow_into_the_direct_array_lane_exact_only_owner_selection_before_any_implementation' "$CARD_403" "row403 must explain why the direct-array lane is next"
guard_expect_fixed_in_file "$TAG" 'shared_route_order_surface=collection_method_call.py' "$CARD_403" "row403 must keep collection_method_call.py as the primary route-order surface"
guard_expect_fixed_in_file "$TAG" 'selected_next=collection_method_call_direct_array_lane_guard_surface' "$CARD_404" "row404 must choose the guard surface"
guard_expect_fixed_in_file "$TAG" 'selected_reason=the_exact_only_direct_array_lane_is_the_remaining_highest_leverage_owner_and_should_freeze_a_guard_surface_before_any_implementation' "$CARD_404" "row404 must explain why the guard surface is next"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "COLLECTION-METHOD-DIRECT-ARRAY-LANE-GUARD-SURFACE-296X-001"' "$STATE" "current state must point to row405"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-404-COLLECTION-METHOD-DIRECT-ARRAY-LANE-OWNER-SELECTION"' "$STATE" "current state must keep row404 as the latest landed card"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

echo "[$TAG] ok"
