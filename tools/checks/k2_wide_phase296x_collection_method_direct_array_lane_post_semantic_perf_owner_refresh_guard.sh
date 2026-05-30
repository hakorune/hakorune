#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="k2-wide-phase296x-collection-method-direct-array-lane-post-semantic-perf-owner-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_407="docs/development/current/main/phases/phase-296x/296x-407-COLLECTION-METHOD-DIRECT-ARRAY-LANE-SEMANTIC-SMOKE.md"
CARD_408="docs/development/current/main/phases/phase-296x/296x-408-COLLECTION-METHOD-DIRECT-ARRAY-LANE-POST-SEMANTIC-PERF-OWNER-REFRESH.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/collection_method_call_direct_array_lane_post_semantic_perf_owner_refresh.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_collection_method_direct_array_lane_post_semantic_perf_owner_refresh_guard.sh"

echo "[$TAG] checking collection-method direct-array lane post-semantic perf owner refresh"

guard_require_files "$TAG" "$CARD_407" "$CARD_408" "$STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_407" "row407 must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_408" "row408 must be landed"
guard_expect_fixed_in_file "$TAG" 'output_contract=collection-method-direct-array-lane-post-semantic-perf-owner-refresh-v0' "$CARD_408" "row408 must define output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=collection-method-direct-array-lane-semantic-smoke-v0' "$CARD_408" "row408 must consume row407"
guard_expect_fixed_in_file "$TAG" 'selected_method=HakoAllocPageModel.acquire_usize/1' "$CARD_408" "row408 must keep the selected method pinned"
guard_expect_fixed_in_file "$TAG" 'selected_boundary=collection_method_call_direct_array_lane_legacy_helper_cache_retirement_selection|collection_method_call_direct_array_lane_post_semantic_perf_owner_refresh' "$CARD_408" "row408 must define both branch outcomes"
guard_expect_fixed_in_file "$TAG" 'legacy_helper_cache_retirement_open=0|1' "$CARD_408" "row408 must expose the retirement-open flag"
guard_expect_fixed_in_file "$TAG" 'selected_next=collection_method_call_direct_array_lane_legacy_helper_cache_retirement_selection|collection_method_call_direct_array_lane_post_semantic_perf_owner_refresh' "$CARD_408" "row408 must define the next-row outcomes"
guard_expect_fixed_in_file "$TAG" 'selected_reason=direct_array_path_dominates_legacy_helper_cache_after_semantic_smoke|legacy_helper_cache_still_dominant_after_semantic_smoke' "$CARD_408" "row408 must explain both dominance outcomes"
guard_expect_fixed_in_file "$TAG" 'selected_next=collection_method_call_direct_array_lane_post_semantic_perf_owner_refresh' "$CARD_407" "row407 must still point to the perf owner refresh"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "COLLECTION-METHOD-DIRECT-ARRAY-LANE-LEGACY-HELPER-CACHE-RETIREMENT-SELECTION-296X-001"' "$STATE" "current state must point to row409"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-408-COLLECTION-METHOD-DIRECT-ARRAY-LANE-POST-SEMANTIC-PERF-OWNER-REFRESH"' "$STATE" "current state must land row408"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_collection_method_direct_array_lane_post_semantic_perf_owner_refresh.XXXXXX)"
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

guard_expect_fixed_in_file "$TAG" 'output_contract=collection-method-direct-array-lane-post-semantic-perf-owner-refresh-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=collection-method-direct-array-lane-semantic-smoke-v0' "$report" "tool must record smoke input contract"
guard_expect_fixed_in_file "$TAG" 'workload_id=representative-object-lifecycle-small-block-v0' "$report" "tool must preserve workload id"
guard_expect_fixed_in_file "$TAG" 'selected_method=HakoAllocPageModel.acquire_usize/1' "$report" "tool must preserve selected method"
guard_expect_fixed_in_file "$TAG" 'direct_array_backend_store_pct=38.21' "$report" "tool must report direct store pct"
guard_expect_fixed_in_file "$TAG" 'direct_array_backend_load_pct=21.74' "$report" "tool must report direct load pct"
guard_expect_fixed_in_file "$TAG" 'direct_array_backend_direct_op_pct=6.50' "$report" "tool must report direct fused pct"
guard_expect_fixed_in_file "$TAG" 'direct_array_backend_total_pct=66.45' "$report" "tool must report direct total pct"
guard_expect_fixed_in_file "$TAG" 'legacy_helper_cache_total_pct=9.32' "$report" "tool must report legacy helper/cache pct"
guard_expect_fixed_in_file "$TAG" 'arraybox_runtime_total_pct=1.50' "$report" "tool must report the public ArrayBox runtime surface"
guard_expect_fixed_in_file "$TAG" 'direct_array_dominates_legacy_helper_cache=1' "$report" "tool must report direct dominance"
guard_expect_fixed_in_file "$TAG" 'legacy_helper_cache_retirement_open=1' "$report" "tool must open legacy retirement selection"
guard_expect_fixed_in_file "$TAG" 'selected_boundary=collection_method_call_direct_array_lane_legacy_helper_cache_retirement_selection' "$report" "tool must select retirement boundary"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=collection_method_call_direct_array_lane_legacy_helper_cache_retirement_selection' "$report" "tool must select retirement next diagnostic"
guard_expect_fixed_in_file "$TAG" 'selected_next=collection_method_call_direct_array_lane_legacy_helper_cache_retirement_selection' "$report" "tool must emit retirement next row"
guard_expect_fixed_in_file "$TAG" 'selected_reason=direct_array_path_dominates_legacy_helper_cache_after_semantic_smoke' "$report" "tool must explain selection"
guard_expect_fixed_in_file "$TAG" 'legacy_retirement_candidate_0=single_thread_exact_array_helper_backend' "$report" "tool must keep legacy candidate list"
guard_expect_fixed_in_file "$TAG" 'legacy_retirement_now=0' "$report" "tool must not delete legacy code in this row"
guard_expect_fixed_in_file "$TAG" 'optimization_open=0' "$report" "tool must keep optimization closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$report" "tool must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$report" "tool must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

legacy_perf_report="$tmp_dir/legacy.perf.report"
legacy_report="$tmp_dir/legacy.report.out"
cat >"$legacy_perf_report" <<'REPORT'
    24.82%  app.exe  app.exe               [.] nyash.object.field_set_hii
    19.43%  app.exe  app.exe               [.] nyash.object.field_get_u64_hii
    18.45%  app.exe  app.exe               [.] nyash.object.field_get_hii
    13.64%  app.exe  app.exe               [.] nyash.object.field_set_u64_hiu
    14.11%  app.exe  app.exe               [.] nyash_kernel::plugin::array_slot_backend::safe_store_i64
     5.56%  app.exe  app.exe               [.] nyash_kernel::plugin::array_slot_backend::safe_store_i64::_$u7b$$u7b$closure$u7d$$u7d$::h282338177f964fcc
     1.63%  app.exe  app.exe               [.] nyash_kernel::plugin::array_slot_backend::safe_load_encoded_i64
     0.91%  app.exe  app.exe               [.] nyash_kernel::plugin::array_handle_cache::array_get_index_encoded_i64::_$u7b$$u7b$closure$u7d$$u7d$::hfdba97e4b1495642
     0.18%  app.exe  app.exe               [.] HakoAllocPageModel.acquire_usize/1
REPORT

python3 "$TOOL" --perf-report "$legacy_perf_report" --out "$legacy_report" >/dev/null

guard_expect_fixed_in_file "$TAG" 'direct_array_backend_total_pct=0.00' "$legacy_report" "legacy fixture must report zero direct array total"
guard_expect_fixed_in_file "$TAG" 'legacy_helper_cache_total_pct=76.34' "$legacy_report" "legacy fixture must report legacy helper/cache total"
guard_expect_fixed_in_file "$TAG" 'array_slot_backend_safe_pct=21.30' "$legacy_report" "legacy fixture must report array slot backend safe pct"
guard_expect_fixed_in_file "$TAG" 'array_handle_cache_pct=0.91' "$legacy_report" "legacy fixture must report array handle-cache pct"
guard_expect_fixed_in_file "$TAG" 'arraybox_runtime_total_pct=22.21' "$legacy_report" "legacy fixture must report the combined public ArrayBox runtime cost"
guard_expect_fixed_in_file "$TAG" 'direct_array_dominates_legacy_helper_cache=0' "$legacy_report" "legacy fixture must keep optional member closed"
guard_expect_fixed_in_file "$TAG" 'legacy_helper_cache_retirement_open=0' "$legacy_report" "legacy fixture must keep retirement closed"
guard_expect_fixed_in_file "$TAG" 'selected_boundary=collection_method_call_direct_array_lane_post_semantic_perf_owner_refresh' "$legacy_report" "legacy fixture must stay on the perf owner refresh boundary"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=collection_method_call_direct_array_lane_post_semantic_perf_owner_refresh' "$legacy_report" "legacy fixture must stay on the perf owner refresh diagnostic"
guard_expect_fixed_in_file "$TAG" 'selected_next=collection_method_call_direct_array_lane_post_semantic_perf_owner_refresh' "$legacy_report" "legacy fixture must stay on the perf owner refresh row"
guard_expect_fixed_in_file "$TAG" 'selected_reason=legacy_helper_cache_still_dominant_after_semantic_smoke' "$legacy_report" "legacy fixture must explain legacy helper/cache dominance"

echo "[$TAG] ok"
