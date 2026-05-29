#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="k2-wide-phase296x-direct-i64-arrayrepr-producer-contract"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_382="docs/development/current/main/phases/phase-296x/296x-382-DIRECTI64-ARRAYREPR-FACT-INVENTORY.md"
CARD_383="docs/development/current/main/phases/phase-296x/296x-383-DIRECTI64-ARRAYREPR-PRODUCER-CONTRACT.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_direct_i64_arrayrepr_producer_contract_guard.sh"

echo "[$TAG] checking DirectI64 ArrayRepr producer contract"

guard_require_files "$TAG" "$CARD_382" "$CARD_383" "$STATE" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_382" "row382 inventory must be landed"
guard_expect_fixed_in_file "$TAG" 'selected_next=direct_i64_arrayrepr_producer_contract' "$CARD_382" "row382 must point to row383"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_383" "row383 producer contract must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=direct-i64-arrayrepr-producer-contract-v0' "$CARD_383" "row383 must define the producer contract output"
guard_expect_fixed_in_file "$TAG" 'input_contract=direct-i64-arrayrepr-fact-inventory-v0' "$CARD_383" "row383 must consume the inventory row"
guard_expect_fixed_in_file "$TAG" 'selected_boundary=direct_i64_arrayrepr_producer_contract' "$CARD_383" "row383 must stay on the producer-contract boundary"
guard_expect_fixed_in_file "$TAG" 'selected_next=direct_i64_arrayrepr_producer_implementation' "$CARD_383" "row383 must point to the producer implementation"
guard_expect_fixed_in_file "$TAG" 'arrayrepr_fact_name=ArrayRepr::DirectI64' "$CARD_383" "row383 must name the explicit ArrayRepr fact"
guard_expect_fixed_in_file "$TAG" 'arrayrepr_fact_owner=representation_planner' "$CARD_383" "row383 must assign fact ownership"
guard_expect_fixed_in_file "$TAG" 'producer_source_fact=resolver.direct_array_i64_ids' "$CARD_383" "row383 must keep the source fact"
guard_expect_fixed_in_file "$TAG" 'producer_birth_symbol=nyash.array.direct_i64.birth_h' "$CARD_383" "row383 must keep the birth symbol"
guard_expect_fixed_in_file "$TAG" 'lowerer_must_consume_fact_without_reproof=1' "$CARD_383" "row383 must prevent re-proof in the lowerer"
guard_expect_fixed_in_file "$TAG" 'public_arraybox_birth_unchanged=1' "$CARD_383" "row383 must keep public ArrayBox birth unchanged"
guard_expect_fixed_in_file "$TAG" 'silent_fallback_allowed=0' "$CARD_383" "row383 must forbid silent fallback"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "DIRECTI64-ARRAYREPR-PRODUCER-CONTRACT-296X-001"' "$STATE" "current state must point to row383"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-382-DIRECTI64-ARRAYREPR-FACT-INVENTORY"' "$STATE" "current state must keep row382 as the latest landed card"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

echo "[$TAG] ok"
