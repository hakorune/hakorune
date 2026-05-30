#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="k2-wide-phase296x-collection-method-direct-array-lane-legacy-helper-cache-retirement-semantic-smoke"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_410="docs/development/current/main/phases/phase-296x/296x-410-COLLECTION-METHOD-DIRECT-ARRAY-LANE-LEGACY-HELPER-CACHE-RETIREMENT-IMPLEMENTATION.md"
CARD_411="docs/development/current/main/phases/phase-296x/296x-411-COLLECTION-METHOD-DIRECT-ARRAY-LANE-LEGACY-HELPER-CACHE-RETIREMENT-SEMANTIC-SMOKE.md"
CARD_412="docs/development/current/main/phases/phase-296x/296x-412-COLLECTION-METHOD-DIRECT-ARRAY-LANE-POST-RETIREMENT-PERF-OWNER-REFRESH.md"
CARD_413="docs/development/current/main/phases/phase-296x/296x-413-POST-DIRECTARRAY-REMAINING-DIRECT-PATH-SURFACE-CHECK.md"
CARD_414="docs/development/current/main/phases/phase-296x/296x-414-MIMALLOC-SOURCE-LEVEL-OWNER-REFRESH.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_collection_method_direct_array_lane_legacy_helper_cache_retirement_semantic_smoke_guard.sh"

echo "[$TAG] checking collection-method direct-array lane legacy helper/cache retirement semantic smoke"

guard_require_files "$TAG" "$CARD_410" "$CARD_411" "$CARD_412" "$CARD_413" "$CARD_414" "$STATE" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_410" "row410 implementation must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_411" "row411 semantic smoke must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_412" "row412 perf owner refresh must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_413" "row413 remaining direct-path surface check must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_414" "row414 source-level owner refresh must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=collection-method-direct-array-lane-legacy-helper-cache-retirement-semantic-smoke-v0' "$CARD_411" "row411 must define output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=collection-method-direct-array-lane-legacy-helper-cache-retirement-implementation-v0' "$CARD_411" "row411 must consume row410"
guard_expect_fixed_in_file "$TAG" 'selected_method=HakoAllocPageModel.acquire_usize/1' "$CARD_411" "row411 must keep the selected method fixed"
guard_expect_fixed_in_file "$TAG" 'selected_backend=direct_array_i64_exact' "$CARD_411" "row411 must keep the direct backend fixed"
guard_expect_fixed_in_file "$TAG" 'default_public_birth_symbol=nyash.array.birth_h' "$CARD_411" "row411 must keep the public birth symbol fixed"
guard_expect_fixed_in_file "$TAG" 'selected_direct_birth_symbol=nyash.array.direct_i64.birth_h' "$CARD_411" "row411 must keep the direct birth symbol fixed"
guard_expect_fixed_in_file "$TAG" 'receiver_origin_fact=resolver.direct_array_i64_ids' "$CARD_411" "row411 must keep the receiver origin fact fixed"
guard_expect_fixed_in_file "$TAG" 'receiver_origin_fact_required=1' "$CARD_411" "row411 must require the receiver origin fact"
guard_expect_fixed_in_file "$TAG" 'public_arraybox_birth_smoke=ok' "$CARD_411" "row411 must smoke public ArrayBox birth"
guard_expect_fixed_in_file "$TAG" 'direct_array_birth_smoke=ok' "$CARD_411" "row411 must smoke DirectArray birth"
guard_expect_fixed_in_file "$TAG" 'direct_array_materialization_snapshot_smoke=ok' "$CARD_411" "row411 must smoke DirectArray materialization"
guard_expect_fixed_in_file "$TAG" 'selected_method_direct_array_lowering_smoke=ok' "$CARD_411" "row411 must smoke selected-method direct lowering"
guard_expect_fixed_in_file "$TAG" 'proof_app_summary=ok' "$CARD_411" "row411 must keep proof-app summary ok"
guard_expect_fixed_in_file "$TAG" 'public_arraybox_handle_reinterpret_as_direct=0' "$CARD_411" "row411 must not reinterpret public handles as direct"
guard_expect_fixed_in_file "$TAG" 'legacy_helper_cache_retirement_now=0' "$CARD_411" "row411 must not delete legacy helper/cache paths"
guard_expect_fixed_in_file "$TAG" 'public_arraybox_behavior_deletion=0' "$CARD_411" "row411 must keep public ArrayBox behavior intact"
guard_expect_fixed_in_file "$TAG" 'handle_entry_cache_deletion=0' "$CARD_411" "row411 must keep handle-entry cache retirement deferred"
guard_expect_fixed_in_file "$TAG" 'public_helper_abi_removal=0' "$CARD_411" "row411 must keep public helper ABI intact"
guard_expect_fixed_in_file "$TAG" 'silent_fallback_allowed=0' "$CARD_411" "row411 must keep silent fallback forbidden"
guard_expect_fixed_in_file "$TAG" 'selected_next=collection_method_call_direct_array_lane_post_retirement_perf_owner_refresh' "$CARD_411" "row411 must point to the post-retirement perf owner refresh"
guard_expect_fixed_in_file "$TAG" 'selected_next=array_repr_design_row' "$CARD_412" "row412 must point to the ArrayRepr design row"
guard_expect_fixed_in_file "$TAG" 'selected_next=mimalloc_source_level_owner_refresh' "$CARD_413" "row413 must point to mimalloc source-level owner refresh"
guard_expect_fixed_in_file "$TAG" 'selected_next=mimalloc_source_level_owner_selection' "$CARD_414" "row414 must point to the source-level owner selection row"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-SOURCE-LEVEL-OWNER-SELECTION-296X-001"' "$STATE" "current state must point to row415"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-414-MIMALLOC-SOURCE-LEVEL-OWNER-REFRESH"' "$STATE" "current state must land row414"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_collection_method_direct_array_lane_legacy_helper_cache_retirement_semantic_smoke.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/readiness.report"

PYTHONPATH="$ROOT_DIR/src/llvm_py:$ROOT_DIR" \
  python3 -m unittest "$ROOT_DIR/src/llvm_py/tests/test_direct_array_i64_constructor_lowering.py"

PYTHONPATH="$ROOT_DIR/src/llvm_py:$ROOT_DIR" \
  python3 -m unittest "$ROOT_DIR/src/llvm_py/tests/test_collection_method_call.py"

(
  cd "$ROOT_DIR"
  HAKO_ARRAY_SLOT_STORE=direct_array_i64_exact \
    cargo test -p nyash_kernel direct_array_i64 --lib -- --nocapture
)

python3 "$ROOT_DIR/tools/allocator/array_slot_nativedirect_lowering_readiness_inventory.py" --out "$report" >/dev/null

guard_expect_fixed_in_file "$TAG" 'output_contract=array-slot-nativedirect-lowering-readiness-inventory-v0' "$report" "readiness tool must report its contract"
guard_expect_fixed_in_file "$TAG" 'candidate_representation=NativeDirect' "$report" "readiness tool must report NativeDirect"
guard_expect_fixed_in_file "$TAG" 'storage_substrate=DirectArrayI64BufferV0' "$report" "readiness tool must report DirectArrayI64BufferV0"
guard_expect_fixed_in_file "$TAG" 'direct_array_buffer_available=1' "$report" "readiness tool must see the direct array buffer"
guard_expect_fixed_in_file "$TAG" 'helper_free_bridge_available=1' "$report" "readiness tool must see the helper-free bridge"
guard_expect_fixed_in_file "$TAG" 'planned_net_helper_delta_positive=1' "$report" "readiness tool must preserve positive-net helper delta"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "readiness tool must end ok"

echo "[$TAG] ok"
