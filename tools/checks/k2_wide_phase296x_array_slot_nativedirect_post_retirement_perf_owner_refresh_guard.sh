#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="k2-wide-phase296x-array-slot-nativedirect-post-retirement-perf-owner-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_376="docs/development/current/main/phases/phase-296x/296x-376-ARRAY-SLOT-NATIVEDIRECT-LEGACY-HELPER-CACHE-RETIREMENT-SEMANTIC-SMOKE.md"
CARD_377="docs/development/current/main/phases/phase-296x/296x-377-ARRAY-SLOT-NATIVEDIRECT-POST-RETIREMENT-PERF-OWNER-REFRESH.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/array_slot_nativedirect_post_retirement_perf_owner_refresh.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_array_slot_nativedirect_post_retirement_perf_owner_refresh_guard.sh"

echo "[$TAG] checking ArraySlot NativeDirect post-retirement perf owner refresh"

guard_require_files "$TAG" "$CARD_376" "$CARD_377" "$STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$TOOL"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_376" "row376 smoke must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_377" "row377 perf owner refresh must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=array-slot-nativedirect-post-retirement-perf-owner-refresh-v0' "$CARD_377" "row377 must define output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=array-slot-nativedirect-legacy-helper-cache-retirement-semantic-smoke-v0' "$CARD_377" "row377 must consume row376"
guard_expect_fixed_in_file "$TAG" 'selected_next=array_repr_design_row' "$CARD_377" "row377 must point to the ArrayRepr design row"
guard_expect_fixed_in_file "$TAG" 'array_repr_design_open=1' "$CARD_377" "row377 must open the ArrayRepr design row"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "ARRAY-SLOT-NATIVEDIRECT-POST-RETIREMENT-PERF-OWNER-REFRESH-296X-001"' "$STATE" "current state must point to row377"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-376-ARRAY-SLOT-NATIVEDIRECT-LEGACY-HELPER-CACHE-RETIREMENT-SEMANTIC-SMOKE"' "$STATE" "current state must land row376"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_array_nativedirect_post_retirement_perf_refresh.XXXXXX)"
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
     0.80%  app.exe  app.exe               [.] nyash.runtime_data.get_hh
     0.70%  app.exe  app.exe               [.] nyash.runtime_data.set_hhh
REPORT

python3 "$TOOL" --perf-report "$perf_report" --out "$report" >/dev/null

guard_expect_fixed_in_file "$TAG" 'output_contract=array-slot-nativedirect-post-retirement-perf-owner-refresh-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=array-slot-nativedirect-legacy-helper-cache-retirement-semantic-smoke-v0' "$report" "tool must record smoke input contract"
guard_expect_fixed_in_file "$TAG" 'workload_id=representative-object-lifecycle-small-block-v0' "$report" "tool must preserve workload id"
guard_expect_fixed_in_file "$TAG" 'selected_method=HakoAllocPageModel.acquire_usize/1' "$report" "tool must preserve selected method"
guard_expect_fixed_in_file "$TAG" 'direct_array_backend_store_pct=38.21' "$report" "tool must report direct store pct"
guard_expect_fixed_in_file "$TAG" 'direct_array_backend_load_pct=21.74' "$report" "tool must report direct load pct"
guard_expect_fixed_in_file "$TAG" 'direct_array_backend_direct_op_pct=6.50' "$report" "tool must report direct fused pct"
guard_expect_fixed_in_file "$TAG" 'direct_array_backend_total_pct=66.45' "$report" "tool must report direct total pct"
guard_expect_fixed_in_file "$TAG" 'arraybox_public_helper_pct=1.50' "$report" "tool must report ArrayBox/public helper pct"
guard_expect_fixed_in_file "$TAG" 'legacy_helper_cache_total_pct=9.32' "$report" "tool must report legacy helper/cache pct"
guard_expect_fixed_in_file "$TAG" 'direct_array_dominates_legacy_helper_cache=1' "$report" "tool must report direct dominance"
guard_expect_fixed_in_file "$TAG" 'array_repr_design_open=1' "$report" "tool must open the ArrayRepr design row"
guard_expect_fixed_in_file "$TAG" 'selected_boundary=array_repr_design_row' "$report" "tool must select the ArrayRepr design row"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=array_repr_design_row' "$report" "tool must select the ArrayRepr design row as next diagnostic"
guard_expect_fixed_in_file "$TAG" 'selected_next=array_repr_design_row' "$report" "tool must emit the ArrayRepr design row as next"
guard_expect_fixed_in_file "$TAG" 'selected_reason=direct_array_path_ready_for_arrayrepr_design_after_retirement_smoke' "$report" "tool must explain selection"
guard_expect_fixed_in_file "$TAG" 'optimization_open=0' "$report" "tool must keep optimization closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$report" "tool must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$report" "tool must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
