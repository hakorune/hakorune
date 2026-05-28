#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-232-POST-SELECTED-FACADE-GET-SET-OWNER-REFRESH.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-231-SELECTED-FACADE-SAME-BLOCK-GET-SET-MEASUREMENT.md"
TOOL="$ROOT_DIR/tools/allocator/post_selected_facade_get_set_owner_refresh.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row232_owner_refresh.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

PERF_REPORT="$TMP_DIR/perf.report"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row232-owner-refresh] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=post-selected-facade-get-set-owner-refresh-v0"
require_line "$DOC" "input_contract=selected-facade-same-block-get-set-measurement-v0"
require_line "$DOC" "perf_exact_slot_helper_pct=64.99"
require_line "$DOC" "perf_exact_slot_get_set_pct=61.59"
require_line "$DOC" "perf_exact_slot_rmw_helper_pct=3.40"
require_line "$DOC" "perf_array_total_pct=28.55"
require_line "$DOC" "selected_boundary=post_facade_exact_slot_callsite_attribution_refresh"
require_line "$DOC" "next_diagnostic=post_facade_exact_slot_callsite_attribution_refresh"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat >"$PERF_REPORT" <<'REPORT'
    15.19%  app.exe  app.exe               [.] nyash.object.exact_slot_get_handle_hii
    14.54%  app.exe  app.exe               [.] core::hash::BuildHasher::hash_one
    12.09%  app.exe  app.exe               [.] nyash.object.exact_slot_get_i64_hii
    12.03%  app.exe  app.exe               [.] nyash.object.exact_slot_set_i64_hii
    11.12%  app.exe  app.exe               [.] nyash.object.exact_slot_get_u64_hii
    10.84%  app.exe  app.exe               [.] nyash_kernel::plugin::array_slot_backend::single_thread_store_i64
     7.13%  app.exe  app.exe               [.] nyash.object.exact_slot_set_u64_hiu
     4.03%  app.exe  app.exe               [.] nyash.object.exact_slot_set_handle_hii
     3.40%  app.exe  app.exe               [.] nyash.object.exact_slot_rmw_add_u64_hiii
     1.59%  app.exe  app.exe               [.] <core::hash::sip::Hasher<S> as core::hash::Hasher>::write
     1.58%  app.exe  app.exe               [.] nyash_kernel::plugin::array_slot_backend::single_thread_load_encoded_i64
     1.57%  app.exe  app.exe               [.] HakoAllocObjectLifecycleReleaseResult.recordRequest/2
     0.80%  app.exe  app.exe               [.] HakoAllocPageModel.resetToFresh/0
     0.80%  app.exe  app.exe               [.] Main.runOne/2
     0.80%  app.exe  app.exe               [.] HakoAllocObjectLifecycleAllocResult.recordSuccess/1
     0.80%  app.exe  app.exe               [.] HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
     0.79%  app.exe  app.exe               [.] HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
     0.78%  app.exe  app.exe               [.] HakoAllocPageModel.acquire_usize/1
REPORT

python3 "$TOOL" --perf-report "$PERF_REPORT" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=post-selected-facade-get-set-owner-refresh-v0"
require_line "$REPORT" "input_contract=selected-facade-same-block-get-set-measurement-v0"
require_line "$REPORT" "workload_id=representative-object-lifecycle-small-block-v0"
require_line "$REPORT" "perf_exact_slot_helper_pct=64.99"
require_line "$REPORT" "perf_exact_slot_get_set_pct=61.59"
require_line "$REPORT" "perf_exact_slot_rmw_helper_pct=3.40"
require_line "$REPORT" "perf_array_total_pct=28.55"
require_line "$REPORT" "selected_boundary=post_facade_exact_slot_callsite_attribution_refresh"
require_line "$REPORT" "next_diagnostic=post_facade_exact_slot_callsite_attribution_refresh"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
