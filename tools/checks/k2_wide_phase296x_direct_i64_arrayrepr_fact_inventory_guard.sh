#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="k2-wide-phase296x-direct-i64-arrayrepr-fact-inventory"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_381="docs/development/current/main/phases/phase-296x/296x-381-DIRECTARRAY-FAMILY-NEXT-ORDER-TASKBOARD.md"
CARD_382="docs/development/current/main/phases/phase-296x/296x-382-DIRECTI64-ARRAYREPR-FACT-INVENTORY.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
NEWBOX="src/llvm_py/instructions/newbox.py"
CONSTRUCTOR="src/llvm_py/instructions/mir_call/constructor_call.py"
CONSUMER="src/llvm_py/instructions/mir_call/collection_method_call.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_direct_i64_arrayrepr_fact_inventory_guard.sh"

echo "[$TAG] checking DirectI64 ArrayRepr fact inventory"

guard_require_files "$TAG" "$CARD_381" "$CARD_382" "$STATE" "$INDEX" "$NEWBOX" "$CONSTRUCTOR" "$CONSUMER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_381" "row381 taskboard must be landed"
guard_expect_fixed_in_file "$TAG" 'selected_next=direct_i64_arrayrepr_fact_inventory' "$CARD_381" "row381 must point to row382"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_382" "row382 inventory must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=direct-i64-arrayrepr-fact-inventory-v0' "$CARD_382" "row382 must define the inventory output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=directarray-family-next-order-taskboard-v0' "$CARD_382" "row382 must consume the taskboard"
guard_expect_fixed_in_file "$TAG" 'selected_boundary=direct_i64_arrayrepr_fact_inventory' "$CARD_382" "row382 must stay on the inventory boundary"
guard_expect_fixed_in_file "$TAG" 'selected_next=direct_i64_arrayrepr_producer_contract' "$CARD_382" "row382 must point to the producer contract"
guard_expect_fixed_in_file "$TAG" 'producer_fact_name=resolver.direct_array_i64_ids' "$CARD_382" "row382 must name the direct-array origin fact"
guard_expect_fixed_in_file "$TAG" 'producer_birth_symbol=nyash.array.direct_i64.birth_h' "$CARD_382" "row382 must name the direct birth symbol"
guard_expect_fixed_in_file "$TAG" 'consumer_site=src/llvm_py/instructions/mir_call/collection_method_call.py' "$CARD_382" "row382 must name the current consumer"
guard_expect_fixed_in_file "$TAG" 'bridge_gap=ArrayRepr::DirectI64 producer fact not yet explicit' "$CARD_382" "row382 must state the bridge gap"
guard_expect_fixed_in_file "$TAG" 'resolver.direct_array_i64_ids' "$NEWBOX" "newbox must still produce the direct-array origin fact"
guard_expect_fixed_in_file "$TAG" 'resolver.direct_array_i64_ids' "$CONSTRUCTOR" "constructor lowering must still produce the direct-array origin fact"
guard_expect_fixed_in_file "$TAG" 'resolver.direct_array_i64_ids' "$CONSUMER" "collection lowering must still consume the direct-array origin fact"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "DIRECTI64-ARRAYREPR-FACT-INVENTORY-296X-001"' "$STATE" "current state must point at row382"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-381-DIRECTARRAY-FAMILY-NEXT-ORDER-TASKBOARD"' "$STATE" "current state must keep row381 as latest landed card"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

echo "[$TAG] ok"
