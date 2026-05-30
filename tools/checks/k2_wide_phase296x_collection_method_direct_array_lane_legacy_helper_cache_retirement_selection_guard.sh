#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="k2-wide-phase296x-collection-method-direct-array-lane-legacy-helper-cache-retirement-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_408="docs/development/current/main/phases/phase-296x/296x-408-COLLECTION-METHOD-DIRECT-ARRAY-LANE-POST-SEMANTIC-PERF-OWNER-REFRESH.md"
CARD_409="docs/development/current/main/phases/phase-296x/296x-409-COLLECTION-METHOD-DIRECT-ARRAY-LANE-LEGACY-HELPER-CACHE-RETIREMENT-SELECTION.md"
CARD_410="docs/development/current/main/phases/phase-296x/296x-410-COLLECTION-METHOD-DIRECT-ARRAY-LANE-LEGACY-HELPER-CACHE-RETIREMENT-IMPLEMENTATION.md"
CARD_411="docs/development/current/main/phases/phase-296x/296x-411-COLLECTION-METHOD-DIRECT-ARRAY-LANE-LEGACY-HELPER-CACHE-RETIREMENT-SEMANTIC-SMOKE.md"
CARD_412="docs/development/current/main/phases/phase-296x/296x-412-COLLECTION-METHOD-DIRECT-ARRAY-LANE-POST-RETIREMENT-PERF-OWNER-REFRESH.md"
CARD_413="docs/development/current/main/phases/phase-296x/296x-413-POST-DIRECTARRAY-REMAINING-DIRECT-PATH-SURFACE-CHECK.md"
CARD_414="docs/development/current/main/phases/phase-296x/296x-414-MIMALLOC-SOURCE-LEVEL-OWNER-REFRESH.md"
CARD_373="docs/development/current/main/phases/phase-296x/296x-373-ARRAYBOX-PUBLIC-SEMANTICS-AND-DIRECTARRAY-SPLIT-SSOT.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TOOL="tools/allocator/collection_method_call_direct_array_lane_legacy_helper_cache_retirement_selection.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_collection_method_direct_array_lane_legacy_helper_cache_retirement_selection_guard.sh"

echo "[$TAG] checking collection-method direct-array lane legacy helper/cache retirement selection"

guard_require_files "$TAG" "$CARD_408" "$CARD_409" "$CARD_410" "$CARD_411" "$CARD_412" "$CARD_413" "$CARD_414" "$CARD_373" "$TOOL" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_409" "row409 must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_410" "row410 must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_411" "row411 must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_412" "row412 must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_413" "row413 must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_414" "row414 must be current"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_408" "row408 must be landed"
guard_expect_fixed_in_file "$TAG" 'output_contract=collection-method-direct-array-lane-legacy-helper-cache-retirement-selection-v0' "$CARD_409" "row409 must define output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=collection-method-direct-array-lane-post-semantic-perf-owner-refresh-v0' "$CARD_409" "row409 must consume row408"
guard_expect_fixed_in_file "$TAG" 'selected_boundary=arraybox_public_semantics_and_directarray_split_ssot' "$CARD_409" "row409 must point to the split SSOT"
guard_expect_fixed_in_file "$TAG" 'selected_next=arraybox_public_semantics_and_directarray_split_ssot' "$CARD_409" "row409 must point to the split SSOT"
guard_expect_fixed_in_file "$TAG" 'selected_retirement_candidate=single_thread_exact_array_helper_backend|array_slot_handle_entry_cache|array_slot_public_helper_fast_lane' "$CARD_409" "row409 must choose a retirement candidate"
guard_expect_fixed_in_file "$TAG" 'selected_retirement_reason=direct_array_path_dominates_legacy_helper_cache_after_semantic_smoke' "$CARD_409" "row409 must explain the selection"
guard_expect_fixed_in_file "$TAG" 'output_contract=collection-method-direct-array-lane-legacy-helper-cache-retirement-implementation-v0' "$CARD_410" "row410 must define output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=collection-method-direct-array-lane-legacy-helper-cache-retirement-selection-v0' "$CARD_410" "row410 must consume row409"
guard_expect_fixed_in_file "$TAG" 'implementation_scope=single_thread_exact_array_helper_backend' "$CARD_410" "row410 must scope only the exact-array helper backend"
guard_expect_fixed_in_file "$TAG" 'selected_retirement_target=single_thread_exact_array_helper_backend' "$CARD_410" "row410 must keep the scoped retirement target fixed"
guard_expect_fixed_in_file "$TAG" 'selected_backend=direct_array_i64_exact' "$CARD_410" "row410 must keep the direct backend fixed"
guard_expect_fixed_in_file "$TAG" 'selected_next=collection_method_call_direct_array_lane_legacy_helper_cache_retirement_semantic_smoke' "$CARD_410" "row410 must point to the semantic smoke"
guard_expect_fixed_in_file "$TAG" 'output_contract=collection-method-direct-array-lane-legacy-helper-cache-retirement-semantic-smoke-v0' "$CARD_411" "row411 must define output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=collection-method-direct-array-lane-legacy-helper-cache-retirement-implementation-v0' "$CARD_411" "row411 must consume row410"
guard_expect_fixed_in_file "$TAG" 'selected_method=HakoAllocPageModel.acquire_usize/1' "$CARD_411" "row411 must keep the selected method fixed"
guard_expect_fixed_in_file "$TAG" 'selected_backend=direct_array_i64_exact' "$CARD_411" "row411 must keep the direct backend fixed"
guard_expect_fixed_in_file "$TAG" 'selected_next=collection_method_call_direct_array_lane_post_retirement_perf_owner_refresh' "$CARD_411" "row411 must point to the post-retirement perf owner refresh"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_373" "row373 split SSOT must stay landed"
guard_expect_fixed_in_file "$TAG" 'selected_next=array_repr_design_row' "$CARD_412" "row412 must point to the ArrayRepr design row"
guard_expect_fixed_in_file "$TAG" 'selected_next=mimalloc_source_level_owner_refresh' "$CARD_413" "row413 must point to mimalloc source-level owner refresh"
guard_expect_fixed_in_file "$TAG" 'selected_next=mimalloc_source_level_owner_selection' "$CARD_414" "row414 must point to the source-level owner selection row"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-SOURCE-LEVEL-OWNER-SELECTION-296X-001"' "$STATE" "current state must point to row415"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-414-MIMALLOC-SOURCE-LEVEL-OWNER-REFRESH"' "$STATE" "current state must land row414"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_collection_method_direct_array_lane_legacy_helper_cache_retirement_selection.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
perf_report="$tmp_dir/perf.report"
report="$tmp_dir/report.out"

cat >"$perf_report" <<'REPORT'
    38.21%  app.exe  app.exe               [.] nyash_kernel::plugin::array_direct_i64_buffer::direct_array_i64_store_i64
    21.74%  app.exe  app.exe               [.] nyash_kernel::plugin::array_direct_i64_buffer::direct_array_i64_load_i64
     6.50%  app.exe  app.exe               [.] nyash.array.slot_load_store_i64_hihi
     5.42%  app.exe  app.exe               [.] HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
     4.30%  app.exe  app.exe               [.] HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
     3.15%  app.exe  app.exe               [.] nyash.object.field_set_hii
     2.27%  app.exe  app.exe               [.] nyash.object.field_get_u64_hii
     1.72%  app.exe  app.exe               [.] core::hash::BuildHasher::hash_one
     1.28%  app.exe  app.exe               [.] <core::hash::sip::Hasher<S> as core::hash::Hasher>::write
     0.90%  app.exe  app.exe               [.] nyash.array.slot_store_hii
     0.80%  app.exe  app.exe               [.] nyash.runtime_data.get_hh
     0.70%  app.exe  app.exe               [.] nyash.runtime_data.set_hhh
REPORT

python3 "$TOOL" --perf-report "$perf_report" --out "$report" >/dev/null

guard_expect_fixed_in_file "$TAG" 'output_contract=collection-method-direct-array-lane-legacy-helper-cache-retirement-selection-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=collection-method-direct-array-lane-post-semantic-perf-owner-refresh-v0' "$report" "tool must record perf refresh input contract"
guard_expect_fixed_in_file "$TAG" 'workload_id=representative-object-lifecycle-small-block-v0' "$report" "tool must preserve workload id"
guard_expect_fixed_in_file "$TAG" 'selected_method=HakoAllocPageModel.acquire_usize/1' "$report" "tool must preserve selected method"
guard_expect_fixed_in_file "$TAG" 'direct_array_backend_total_pct=66.45' "$report" "tool must report direct total pct"
guard_expect_fixed_in_file "$TAG" 'legacy_helper_cache_total_pct=9.32' "$report" "tool must report legacy helper/cache pct"
guard_expect_fixed_in_file "$TAG" 'direct_array_dominates_legacy_helper_cache=1' "$report" "tool must report direct dominance"
guard_expect_fixed_in_file "$TAG" 'legacy_helper_cache_retirement_open=1' "$report" "tool must open retirement selection"
guard_expect_fixed_in_file "$TAG" 'selected_retirement_candidate=single_thread_exact_array_helper_backend' "$report" "tool must choose the exact-array helper backend first"
guard_expect_fixed_in_file "$TAG" 'selected_retirement_reason=direct_array_path_dominates_legacy_helper_cache_after_semantic_smoke' "$report" "tool must explain the selection"
guard_expect_fixed_in_file "$TAG" 'selected_boundary=arraybox_public_semantics_and_directarray_split_ssot' "$report" "tool must point to the split SSOT"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=arraybox_public_semantics_and_directarray_split_ssot' "$report" "tool must point to the split SSOT"
guard_expect_fixed_in_file "$TAG" 'selected_next=arraybox_public_semantics_and_directarray_split_ssot' "$report" "tool must emit the split SSOT"
guard_expect_fixed_in_file "$TAG" 'legacy_retirement_now=0' "$report" "tool must not delete legacy code in this row"
guard_expect_fixed_in_file "$TAG" 'optimization_open=0' "$report" "tool must keep optimization closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$report" "tool must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$report" "tool must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
