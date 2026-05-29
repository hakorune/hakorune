#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="k2-wide-phase296x-directarray-family-extension-gate"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_379="docs/development/current/main/phases/phase-296x/296x-379-HAKO-ARRAYCORE-OWNER-ALIGNMENT.md"
CARD_380="docs/development/current/main/phases/phase-296x/296x-380-DIRECTARRAY-FAMILY-EXTENSION-GATE.md"
CARD_381="docs/development/current/main/phases/phase-296x/296x-381-DIRECTARRAY-FAMILY-NEXT-ORDER-TASKBOARD.md"
CARD_382="docs/development/current/main/phases/phase-296x/296x-382-DIRECTI64-ARRAYREPR-FACT-INVENTORY.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_directarray_family_extension_gate_guard.sh"

echo "[$TAG] checking DirectArray family extension gate"

guard_require_files "$TAG" "$CARD_379" "$CARD_380" "$CARD_381" "$CARD_382" "$STATE" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_379" "row379 owner alignment must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_380" "row380 extension gate must be landed"
guard_expect_fixed_in_file "$TAG" 'output_contract=directarray-family-extension-gate-v0' "$CARD_380" "row380 must define the extension gate output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=hako-arraycore-owner-alignment-note-v0' "$CARD_380" "row380 must consume the ArrayCore alignment note"
guard_expect_fixed_in_file "$TAG" 'selected_boundary=directarray_family_extension_gate_row' "$CARD_380" "row380 must stay at the extension gate boundary"
guard_expect_fixed_in_file "$TAG" 'selected_next=directarray_family_next_order_taskboard' "$CARD_380" "row380 must point to row381"
guard_expect_fixed_in_file "$TAG" 'new_member_requires_explicit_storage_contract=1' "$CARD_380" "row380 must require explicit storage contracts"
guard_expect_fixed_in_file "$TAG" 'materialization_route_required=1' "$CARD_380" "row380 must require materialization routes"
guard_expect_fixed_in_file "$TAG" 'public_arraybox_facade_preserved=1' "$CARD_380" "row380 must preserve the public ArrayBox facade"
guard_expect_fixed_in_file "$TAG" 'silent_fallback_allowed=0' "$CARD_380" "row380 must forbid silent fallback"
guard_expect_fixed_in_file "$TAG" 'mixed_storage_shortcut_allowed=0' "$CARD_380" "row380 must forbid mixed storage shortcuts"
guard_expect_fixed_in_file "$TAG" 'nyash_array_birth_h_behavior_change=0' "$CARD_380" "row380 must not change nyash.array.birth_h"
guard_expect_fixed_in_file "$TAG" 'new_member_implementation_open=0' "$CARD_380" "row380 must not implement a new member"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_381" "row381 next-order taskboard must be landed"
guard_expect_fixed_in_file "$TAG" 'selected_next=direct_i64_arrayrepr_fact_inventory' "$CARD_381" "row381 must point to row382"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_382" "row382 fact inventory must be current"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "DIRECTI64-ARRAYREPR-FACT-INVENTORY-296X-001"' "$STATE" "current state must point to row382"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-381-DIRECTARRAY-FAMILY-NEXT-ORDER-TASKBOARD"' "$STATE" "current state must keep row381 as the latest landed card"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

echo "[$TAG] ok"
