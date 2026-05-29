#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-array-slot-nativedirect-legacy-helper-cache-retirement-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_371="docs/development/current/main/phases/phase-296x/296x-371-ARRAY-SLOT-NATIVEDIRECT-POST-SEMANTIC-PERF-OWNER-REFRESH.md"
CARD_372="docs/development/current/main/phases/phase-296x/296x-372-ARRAY-SLOT-NATIVEDIRECT-LEGACY-HELPER-CACHE-RETIREMENT-SELECTION.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/array_slot_nativedirect_legacy_helper_cache_retirement_selection.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_array_slot_nativedirect_legacy_helper_cache_retirement_selection_guard.sh"

echo "[$TAG] checking ArraySlot NativeDirect legacy helper/cache retirement selection"

guard_require_files "$TAG" "$CARD_371" "$CARD_372" "$STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_371" "perf owner refresh card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_372" "retirement selection card must be landed"
guard_expect_fixed_in_file "$TAG" 'output_contract=array-slot-nativedirect-legacy-helper-cache-retirement-selection-v0' "$CARD_372" "row372 must define output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=array-slot-nativedirect-post-semantic-perf-owner-refresh-v0' "$CARD_372" "row372 must consume perf owner refresh output"
guard_expect_fixed_in_file "$TAG" 'selected_boundary=arraybox_public_semantics_and_directarray_split_ssot' "$CARD_372" "row372 must point to the split SSOT before implementation"
guard_expect_fixed_in_file "$TAG" 'selected_next=arraybox_public_semantics_and_directarray_split_ssot' "$CARD_372" "row372 must point to the split SSOT before implementation"
guard_expect_fixed_in_file "$TAG" 'legacy_helper_cache_retirement_open=0|1' "$CARD_372" "row372 must expose retirement open flag"
guard_expect_fixed_in_file "$TAG" 'selected_retirement_candidate=single_thread_exact_array_helper_backend|array_slot_handle_entry_cache|array_slot_public_helper_fast_lane' "$CARD_372" "row372 must choose a retirement candidate"
guard_expect_fixed_in_file "$TAG" 'selected_next=array_slot_nativedirect_legacy_helper_cache_retirement_selection' "$CARD_371" "row371 must point to the retirement selection"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "ARRAYBOX-PUBLIC-SEMANTICS-AND-DIRECTARRAY-SPLIT-SSOT-296X-001"' "$STATE" "current state must point to the split SSOT row"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-372-ARRAY-SLOT-NATIVEDIRECT-LEGACY-HELPER-CACHE-RETIREMENT-SELECTION"' "$STATE" "current state must land row372"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_array_nativedirect_retirement_selection.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
perf_report="$tmp_dir/perf.report"
report="$tmp_dir/report.out"

cat >"$perf_report" <<'REPORT'
    38.21%  app.exe  app.exe               [.] nyash_kernel::plugin::array_slot_backend::single_thread_store_i64
    21.74%  app.exe  app.exe               [.] nyash_kernel::plugin::array_slot_backend::single_thread_load_encoded_i64
     6.50%  app.exe  app.exe               [.] nyash.array.slot_load_store_i64_hihi
     5.42%  app.exe  app.exe               [.] HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
     4.30%  app.exe  app.exe               [.] HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
     3.15%  app.exe  app.exe               [.] nyash.object.field_set_hii
     2.27%  app.exe  app.exe               [.] nyash.object.field_get_u64_hii
     1.72%  app.exe  app.exe               [.] core::hash::BuildHasher::hash_one
     1.28%  app.exe  app.exe               [.] <core::hash::sip::Hasher<S> as core::hash::Hasher>::write
     0.90%  app.exe  app.exe               [.] nyash.array.slot_store_hii
REPORT

python3 "$TOOL" --perf-report "$perf_report" --out "$report" >/dev/null

guard_expect_fixed_in_file "$TAG" 'output_contract=array-slot-nativedirect-legacy-helper-cache-retirement-selection-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=array-slot-nativedirect-post-semantic-perf-owner-refresh-v0' "$report" "tool must record perf refresh input contract"
guard_expect_fixed_in_file "$TAG" 'workload_id=representative-object-lifecycle-small-block-v0' "$report" "tool must preserve workload id"
guard_expect_fixed_in_file "$TAG" 'selected_method=HakoAllocPageModel.acquire_usize/1' "$report" "tool must preserve selected method"
guard_expect_fixed_in_file "$TAG" 'direct_array_backend_store_pct=38.21' "$report" "tool must report direct store pct"
guard_expect_fixed_in_file "$TAG" 'direct_array_backend_load_pct=21.74' "$report" "tool must report direct load pct"
guard_expect_fixed_in_file "$TAG" 'direct_array_backend_direct_op_pct=6.50' "$report" "tool must report direct fused pct"
guard_expect_fixed_in_file "$TAG" 'direct_array_backend_total_pct=66.45' "$report" "tool must report direct total pct"
guard_expect_fixed_in_file "$TAG" 'legacy_helper_cache_total_pct=9.32' "$report" "tool must report legacy helper/cache pct"
guard_expect_fixed_in_file "$TAG" 'direct_array_dominates_legacy_helper_cache=1' "$report" "tool must report direct dominance"
guard_expect_fixed_in_file "$TAG" 'legacy_helper_cache_retirement_open=1' "$report" "tool must open legacy retirement selection"
guard_expect_fixed_in_file "$TAG" 'selected_retirement_candidate=single_thread_exact_array_helper_backend' "$report" "tool must choose the exact-array helper backend first"
guard_expect_fixed_in_file "$TAG" 'selected_retirement_reason=direct_array_path_dominates_legacy_helper_cache_after_semantic_smoke' "$report" "tool must explain the selection"
guard_expect_fixed_in_file "$TAG" 'selected_boundary=arraybox_public_semantics_and_directarray_split_ssot' "$report" "tool must point to the split SSOT"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=arraybox_public_semantics_and_directarray_split_ssot' "$report" "tool must point to the split SSOT"
guard_expect_fixed_in_file "$TAG" 'selected_next=arraybox_public_semantics_and_directarray_split_ssot' "$report" "tool must emit the split SSOT row"
guard_expect_fixed_in_file "$TAG" 'legacy_retirement_candidate_0=single_thread_exact_array_helper_backend' "$report" "tool must keep legacy candidate list"
guard_expect_fixed_in_file "$TAG" 'legacy_retirement_now=0' "$report" "tool must not delete legacy code in this row"
guard_expect_fixed_in_file "$TAG" 'optimization_open=0' "$report" "tool must keep optimization closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$report" "tool must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$report" "tool must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
