#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="k2-wide-phase296x-runtime-data-dispatch-consumer-callsite-attribution-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_400="docs/development/current/main/phases/phase-296x/296x-400-RUNTIME-DATA-DISPATCH-ROUTE-POLICY-OWNER-REFRESH.md"
CARD_401="docs/development/current/main/phases/phase-296x/296x-401-RUNTIME-DATA-DISPATCH-CONSUMER-CALLSITE-ATTRIBUTION-REFRESH.md"
CARD_402="docs/development/current/main/phases/phase-296x/296x-402-COLLECTION-METHOD-SURFACE-OWNER-SELECTION.md"
CARD_403="docs/development/current/main/phases/phase-296x/296x-403-COLLECTION-METHOD-ROUTE-ORDER-INVENTORY.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_runtime_data_dispatch_consumer_callsite_attribution_refresh_guard.sh"

echo "[$TAG] checking RuntimeDataBox consumer callsite attribution refresh"

guard_require_files "$TAG" "$CARD_400" "$CARD_401" "$CARD_402" "$CARD_403" "$STATE" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_400" "row400 must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_401" "row401 must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_402" "row402 must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_403" "row403 must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=runtime-data-dispatch-consumer-callsite-attribution-refresh-v0' "$CARD_401" "row401 must define output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=runtime-data-dispatch-route-policy-owner-refresh-v0' "$CARD_401" "row401 must consume the row400 owner refresh"
guard_expect_fixed_in_file "$TAG" 'runtime_data_dispatch_thin_consumer=1' "$CARD_401" "row401 must keep runtime_data_dispatch thin"
guard_expect_fixed_in_file "$TAG" 'runtime_data_route_policy_source_stable=1' "$CARD_401" "row401 must keep the policy source stable"
guard_expect_fixed_in_file "$TAG" 'runtime_data_consumer_surface_attributed_file_by_file=1' "$CARD_401" "row401 must keep the consumer surfaces attributed"
guard_expect_fixed_in_file "$TAG" 'selected_next=collection_method_call_surface_owner_selection' "$CARD_401" "row401 must choose the next owner"
guard_expect_fixed_in_file "$TAG" 'selected_reason=collection_method_call_py_owns_the_shared_collection_route_order_and_the_direct_array_lane_so_it_is_the_highest_leverage_remaining_consumer_surface_after_the_row400_inventory_split' "$CARD_401" "row401 must explain why collection_method_call is next"
guard_expect_fixed_in_file "$TAG" 'CCA-001: Collection Method Surface Compare' "$CARD_401" "row401 must compare the collection-method surface"
guard_expect_fixed_in_file "$TAG" 'CCA-005: Next Owner Selection' "$CARD_401" "row401 must select exactly one next owner"
guard_expect_fixed_in_file "$TAG" 'output_contract=collection-method-surface-owner-selection-v0' "$CARD_402" "row402 must define output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=runtime-data-dispatch-consumer-callsite-attribution-refresh-v0' "$CARD_402" "row402 must consume row401"
guard_expect_fixed_in_file "$TAG" 'shared_collection_route_order_is_highest_leverage=1' "$CARD_402" "row402 must keep the shared route order highest leverage"
guard_expect_fixed_in_file "$TAG" 'selected_next=collection_method_call_route_order_inventory' "$CARD_402" "row402 must choose the route-order inventory"
guard_expect_fixed_in_file "$TAG" 'collection_method_surface_primary=collection_method_call.py' "$CARD_402" "row402 must keep collection_method_call.py as the primary surface"
guard_expect_fixed_in_file "$TAG" 'output_contract=collection-method-route-order-inventory-v0' "$CARD_403" "row403 must define output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=collection-method-surface-owner-selection-v0' "$CARD_403" "row403 must consume row402"
guard_expect_fixed_in_file "$TAG" 'shared_collection_route_order_is_highest_leverage=1' "$CARD_403" "row403 must keep the shared route order highest leverage"
guard_expect_fixed_in_file "$TAG" 'selected_next=collection_method_call_direct_array_lane_owner_selection' "$CARD_403" "row403 must narrow to the direct-array lane owner selection"
guard_expect_fixed_in_file "$TAG" 'collection_method_surface_primary=collection_method_call.py' "$CARD_403" "row403 must keep collection_method_call.py as the primary surface"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "COLLECTION-METHOD-ROUTE-ORDER-INVENTORY-296X-001"' "$STATE" "current state must point to row403"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-402-COLLECTION-METHOD-SURFACE-OWNER-SELECTION"' "$STATE" "current state must keep row402 as the latest landed card"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

echo "[$TAG] ok"
