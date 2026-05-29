#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-directarray-family-storage-substrate-roadmap"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_373="docs/development/current/main/phases/phase-296x/296x-373-ARRAYBOX-PUBLIC-SEMANTICS-AND-DIRECTARRAY-SPLIT-SSOT.md"
CARD_374="docs/development/current/main/phases/phase-296x/296x-374-DIRECTARRAY-FAMILY-STORAGE-SUBSTRATE-ROADMAP.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_directarray_family_storage_substrate_roadmap_guard.sh"
DIRECT_ARRAY="crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs"

echo "[$TAG] checking DirectArray family storage substrate roadmap"

guard_require_files "$TAG" "$CARD_373" "$CARD_374" "$STATE" "$INDEX" "$SELF_SCRIPT" "$DIRECT_ARRAY"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_373" "split SSOT must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_374" "roadmap must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=directarray-family-storage-substrate-roadmap-v0' "$CARD_374" "row374 must define output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=arraybox-public-semantics-and-directarray-split-ssot-v0' "$CARD_374" "row374 must consume row373"
guard_expect_fixed_in_file "$TAG" 'long_term_primary_storage=directarray_family' "$CARD_374" "DirectArray family must be long-term primary storage"
guard_expect_fixed_in_file "$TAG" 'first_directarray_member=DirectArrayI64BufferV0' "$CARD_374" "DirectArrayI64 must be first family member"
guard_expect_fixed_in_file "$TAG" 'arraybox_long_term_role=public_facade|materialized_view|dynamic_fallback|generic_api' "$CARD_374" "ArrayBox must remain facade/materialized view"
guard_expect_fixed_in_file "$TAG" 'arraybox_long_term_performance_substrate=0' "$CARD_374" "ArrayBox must not be long-term performance substrate"
guard_expect_fixed_in_file "$TAG" 'array_repr_layer_planned=1' "$CARD_374" "ArrayRepr layer must be planned before internal convergence"
guard_expect_fixed_in_file "$TAG" 'stage0_array_seed=rust_keep' "$CARD_374" "stage0 Rust ArraySeed must remain a bootstrap keep"
guard_expect_fixed_in_file "$TAG" 'stage0_rust_array_seed_is_semantics_owner=0' "$CARD_374" "stage0 Rust ArraySeed must not be semantics owner"
guard_expect_fixed_in_file "$TAG" 'array_semantics_owner=hako_ring1_array_core' "$CARD_374" ".hako ring1 ArrayCore must own visible semantics"
guard_expect_fixed_in_file "$TAG" 'array_storage_substrate=directarray_family' "$CARD_374" "DirectArray family must be storage substrate"
guard_expect_fixed_in_file "$TAG" 'public_materialized_view=arraybox' "$CARD_374" "ArrayBox must remain public materialized view"
guard_expect_fixed_in_file "$TAG" 'rust_private_layout_as_semantic_truth=0' "$CARD_374" "Rust private layout must not be semantic truth"
guard_expect_fixed_in_file "$TAG" 'rust_private_layout_as_llvm_abi=0' "$CARD_374" "Rust private layout must not become LLVM ABI"
guard_expect_fixed_in_file "$TAG" 'public_handle_reinterpret_as_direct=0' "$CARD_374" "public handles must not be reinterpreted"
guard_expect_fixed_in_file "$TAG" 'materialization_route_required=1' "$CARD_374" "materialization route must remain required"
guard_expect_fixed_in_file "$TAG" 'silent_fallback_allowed=0' "$CARD_374" "silent fallback must stay forbidden"
guard_expect_fixed_in_file "$TAG" 'selected_next=array_slot_nativedirect_legacy_helper_cache_retirement_implementation' "$CARD_374" "row374 must reopen scoped implementation"
guard_expect_fixed_in_file "$TAG" 'selected_next=directarray_family_storage_substrate_roadmap' "$CARD_373" "row373 must point to row374"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "DIRECTARRAY-FAMILY-STORAGE-SUBSTRATE-ROADMAP-296X-001"' "$STATE" "current state must point to row374"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-373-ARRAYBOX-PUBLIC-SEMANTICS-AND-DIRECTARRAY-SPLIT-SSOT"' "$STATE" "current state must land row373"
guard_expect_fixed_in_file "$TAG" 'DirectArrayI64BufferV0' "$DIRECT_ARRAY" "DirectArrayI64 storage pilot must exist"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

echo "[$TAG] ok"
