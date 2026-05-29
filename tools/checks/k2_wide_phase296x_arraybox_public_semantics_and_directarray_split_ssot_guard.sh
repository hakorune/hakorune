#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-arraybox-public-semantics-and-directarray-split-ssot"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_372="docs/development/current/main/phases/phase-296x/296x-372-ARRAY-SLOT-NATIVEDIRECT-LEGACY-HELPER-CACHE-RETIREMENT-SELECTION.md"
CARD_373="docs/development/current/main/phases/phase-296x/296x-373-ARRAYBOX-PUBLIC-SEMANTICS-AND-DIRECTARRAY-SPLIT-SSOT.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_arraybox_public_semantics_and_directarray_split_ssot_guard.sh"
DIRECT_ARRAY="crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs"
ARRAY_BACKEND="crates/nyash_kernel/src/plugin/array_slot_backend.rs"

echo "[$TAG] checking ArrayBox public semantics / DirectArray split SSOT"

guard_require_files "$TAG" "$CARD_372" "$CARD_373" "$STATE" "$INDEX" "$SELF_SCRIPT" "$DIRECT_ARRAY" "$ARRAY_BACKEND"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_372" "retirement selection must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_373" "split SSOT must be landed"
guard_expect_fixed_in_file "$TAG" 'output_contract=arraybox-public-semantics-and-directarray-split-ssot-v0' "$CARD_373" "row373 must define output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=array-slot-nativedirect-legacy-helper-cache-retirement-selection-v0' "$CARD_373" "row373 must consume row372"
guard_expect_fixed_in_file "$TAG" 'public_arraybox_owner=plugin_runtime_public_semantics' "$CARD_373" "ArrayBox must remain public semantics owner"
guard_expect_fixed_in_file "$TAG" 'direct_array_owner=native_direct_i64_hot_storage_substrate' "$CARD_373" "DirectArray must own hot exact i64 storage"
guard_expect_fixed_in_file "$TAG" 'nyash_array_birth_h_public=1' "$CARD_373" "public ArrayBox birth must remain public"
guard_expect_fixed_in_file "$TAG" 'nyash_array_direct_i64_birth_h_separate=1' "$CARD_373" "DirectArray birth must stay separate"
guard_expect_fixed_in_file "$TAG" 'public_arraybox_handle_reinterpret_as_direct=0' "$CARD_373" "public handles must not be reinterpreted"
guard_expect_fixed_in_file "$TAG" 'plugin_internal_cache_as_llvm_abi=0' "$CARD_373" "plugin internals must not become LLVM ABI"
guard_expect_fixed_in_file "$TAG" 'direct_array_materialization_route_required=1' "$CARD_373" "materialization route must be explicit"
guard_expect_fixed_in_file "$TAG" 'silent_fallback_allowed=0' "$CARD_373" "silent fallback must stay forbidden"
guard_expect_fixed_in_file "$TAG" 'selected_next=directarray_family_storage_substrate_roadmap' "$CARD_373" "row373 must point to the DirectArray family roadmap"
guard_expect_fixed_in_file "$TAG" 'selected_next=arraybox_public_semantics_and_directarray_split_ssot' "$CARD_372" "row372 must point to split SSOT"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "DIRECTARRAY-FAMILY-STORAGE-SUBSTRATE-ROADMAP-296X-001"' "$STATE" "current state must point to row374"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-373-ARRAYBOX-PUBLIC-SEMANTICS-AND-DIRECTARRAY-SPLIT-SSOT"' "$STATE" "current state must land row373"
guard_expect_fixed_in_file "$TAG" 'nyash.array.direct_i64.birth_h' "$DIRECT_ARRAY" "DirectArray birth symbol must stay separate"
guard_expect_fixed_in_file "$TAG" 'nyash.array.birth_h' "$CARD_373" "row373 must name public ArrayBox birth boundary"
guard_expect_fixed_in_file "$TAG" 'helper route closed' "$ARRAY_BACKEND" "DirectArray helper route must remain closed until implementation owner opens it"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

echo "[$TAG] ok"
