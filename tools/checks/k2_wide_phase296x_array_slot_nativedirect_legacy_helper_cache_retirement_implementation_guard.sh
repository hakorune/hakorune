#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="k2-wide-phase296x-array-slot-nativedirect-legacy-helper-cache-retirement-implementation"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_373="docs/development/current/main/phases/phase-296x/296x-373-ARRAYBOX-PUBLIC-SEMANTICS-AND-DIRECTARRAY-SPLIT-SSOT.md"
CARD_374="docs/development/current/main/phases/phase-296x/296x-374-DIRECTARRAY-FAMILY-STORAGE-SUBSTRATE-ROADMAP.md"
CARD_375="docs/development/current/main/phases/phase-296x/296x-375-ARRAY-SLOT-NATIVEDIRECT-LEGACY-HELPER-CACHE-RETIREMENT-IMPLEMENTATION.md"
CARD_376="docs/development/current/main/phases/phase-296x/296x-376-ARRAY-SLOT-NATIVEDIRECT-LEGACY-HELPER-CACHE-RETIREMENT-SEMANTIC-SMOKE.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_array_slot_nativedirect_legacy_helper_cache_retirement_implementation_guard.sh"

echo "[$TAG] checking ArraySlot NativeDirect legacy helper/cache retirement implementation"

guard_require_files "$TAG" "$CARD_373" "$CARD_374" "$CARD_375" "$STATE" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_require_files "$TAG" "$CARD_376"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_373" "split SSOT must remain landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_374" "roadmap must remain current"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_375" "implementation row must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_376" "semantic smoke row must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=array-slot-nativedirect-legacy-helper-cache-retirement-implementation-v0' "$CARD_375" "row375 must define output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=directarray-family-storage-substrate-roadmap-v0' "$CARD_375" "row375 must consume the roadmap contract"
guard_expect_fixed_in_file "$TAG" 'implementation_scope=single_thread_exact_array_helper_backend' "$CARD_375" "row375 must scope only the exact-array helper backend"
guard_expect_fixed_in_file "$TAG" 'handle_entry_cache_retirement_deferred=1' "$CARD_375" "row375 must defer handle-entry cache retirement"
guard_expect_fixed_in_file "$TAG" 'public_helper_fast_lane_retirement_deferred=1' "$CARD_375" "row375 must defer public helper fast-lane retirement"
guard_expect_fixed_in_file "$TAG" 'public_arraybox_semantics_preserved=1' "$CARD_375" "row375 must preserve public ArrayBox semantics"
guard_expect_fixed_in_file "$TAG" 'public_arraybox_behavior_deletion=0' "$CARD_375" "row375 must forbid public ArrayBox deletion"
guard_expect_fixed_in_file "$TAG" 'handle_entry_cache_deletion=0' "$CARD_375" "row375 must forbid handle-entry cache deletion"
guard_expect_fixed_in_file "$TAG" 'public_helper_abi_removal=0' "$CARD_375" "row375 must forbid public helper ABI removal"
guard_expect_fixed_in_file "$TAG" 'directarray_helper_route_fail_fast_until_scoped_replaced=1' "$CARD_375" "row375 must keep the DirectArray helper route fail-fast until replaced"
guard_expect_fixed_in_file "$TAG" 'selected_next=array_slot_nativedirect_legacy_helper_cache_retirement_semantic_smoke' "$CARD_375" "row375 must point to the post-retirement semantic smoke"
guard_expect_fixed_in_file "$TAG" 'selected_next=array_slot_nativedirect_legacy_helper_cache_retirement_implementation' "$CARD_374" "row374 must point to the scoped implementation boundary"
guard_expect_fixed_in_file "$TAG" 'output_contract=array-slot-nativedirect-legacy-helper-cache-retirement-semantic-smoke-v0' "$CARD_376" "row376 must define output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=array-slot-nativedirect-legacy-helper-cache-retirement-implementation-v0' "$CARD_376" "row376 must consume row375"
guard_expect_fixed_in_file "$TAG" 'selected_next=array_slot_nativedirect_post_retirement_perf_owner_refresh' "$CARD_376" "row376 must point to the post-retirement perf owner refresh"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "ARRAY-SLOT-NATIVEDIRECT-LEGACY-HELPER-CACHE-RETIREMENT-SEMANTIC-SMOKE-296X-001"' "$STATE" "current state must point to row376"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-375-ARRAY-SLOT-NATIVEDIRECT-LEGACY-HELPER-CACHE-RETIREMENT-IMPLEMENTATION"' "$STATE" "current state must keep latest landed card at row375"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

echo "[$TAG] ok"
