#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="k2-wide-phase296x-directarray-family-next-order-taskboard"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_380="docs/development/current/main/phases/phase-296x/296x-380-DIRECTARRAY-FAMILY-EXTENSION-GATE.md"
CARD_381="docs/development/current/main/phases/phase-296x/296x-381-DIRECTARRAY-FAMILY-NEXT-ORDER-TASKBOARD.md"
CARD_382="docs/development/current/main/phases/phase-296x/296x-382-DIRECTI64-ARRAYREPR-FACT-INVENTORY.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_directarray_family_next_order_taskboard_guard.sh"

echo "[$TAG] checking DirectArray family next order taskboard"

guard_require_files "$TAG" "$CARD_380" "$CARD_381" "$CARD_382" "$STATE" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_380" "row380 extension gate must be landed"
guard_expect_fixed_in_file "$TAG" 'selected_next=directarray_family_next_order_taskboard' "$CARD_380" "row380 must point to row381"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_381" "row381 next order taskboard must be landed"
guard_expect_fixed_in_file "$TAG" 'output_contract=directarray-family-next-order-taskboard-v0' "$CARD_381" "row381 must define the next-order output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=directarray-family-extension-gate-v0' "$CARD_381" "row381 must consume row380"
guard_expect_fixed_in_file "$TAG" 'selected_next=direct_i64_arrayrepr_fact_inventory' "$CARD_381" "row381 must select the DirectI64 ArrayRepr inventory"
guard_expect_fixed_in_file "$TAG" 'new_directarray_member_selected=0' "$CARD_381" "row381 must not select a new member yet"
guard_expect_fixed_in_file "$TAG" 'direct_i64_first_member_stays_primary=1' "$CARD_381" "row381 must keep DirectI64 as the first member"
guard_expect_fixed_in_file "$TAG" 'arrayrepr_bridge_must_precede_new_member=1' "$CARD_381" "row381 must require ArrayRepr before new members"
guard_expect_fixed_in_file "$TAG" '### DA-SEQ-001: DirectI64 ArrayRepr Fact Inventory' "$CARD_381" "row381 must expose DA-SEQ-001"
guard_expect_fixed_in_file "$TAG" '### DA-SEQ-002: DirectI64 ArrayRepr Producer Contract' "$CARD_381" "row381 must expose DA-SEQ-002"
guard_expect_fixed_in_file "$TAG" '### DA-SEQ-003: DirectI64 ArrayRepr Producer Implementation' "$CARD_381" "row381 must expose DA-SEQ-003"
guard_expect_fixed_in_file "$TAG" '### DA-SEQ-004: DirectI64 Lowering Consumer Rebase' "$CARD_381" "row381 must expose DA-SEQ-004"
guard_expect_fixed_in_file "$TAG" '### DA-SEQ-005: DirectI64 Materialization Smoke Refresh' "$CARD_381" "row381 must expose DA-SEQ-005"
guard_expect_fixed_in_file "$TAG" '### DA-SEQ-006: Post-Rebase Perf Owner Refresh' "$CARD_381" "row381 must expose DA-SEQ-006"
guard_expect_fixed_in_file "$TAG" '### DA-SEQ-007: Optional Next Member Selection' "$CARD_381" "row381 must expose DA-SEQ-007"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_382" "row382 fact inventory must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=direct-i64-arrayrepr-fact-inventory-v0' "$CARD_382" "row382 must define the inventory output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=directarray-family-next-order-taskboard-v0' "$CARD_382" "row382 must consume row381"
guard_expect_fixed_in_file "$TAG" 'selected_next=direct_i64_arrayrepr_producer_contract' "$CARD_382" "row382 must point to the producer contract"
guard_expect_fixed_in_file "$TAG" 'producer_fact_name=resolver.direct_array_i64_ids' "$CARD_382" "row382 must name the direct-array origin fact"
guard_expect_fixed_in_file "$TAG" 'producer_birth_symbol=nyash.array.direct_i64.birth_h' "$CARD_382" "row382 must name the direct birth symbol"
guard_expect_fixed_in_file "$TAG" 'consumer_site=src/llvm_py/instructions/mir_call/collection_method_call.py' "$CARD_382" "row382 must name the current consumer"
guard_expect_fixed_in_file "$TAG" 'bridge_gap=ArrayRepr::DirectI64 producer fact not yet explicit' "$CARD_382" "row382 must state the bridge gap"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "DIRECTI64-ARRAYREPR-FACT-INVENTORY-296X-001"' "$STATE" "current state must point to row382"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-381-DIRECTARRAY-FAMILY-NEXT-ORDER-TASKBOARD"' "$STATE" "current state must keep row381 as latest landed card"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

echo "[$TAG] ok"
