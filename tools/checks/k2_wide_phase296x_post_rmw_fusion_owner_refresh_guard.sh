#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-224-POST-RMW-FUSION-OWNER-REFRESH.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-223-TYPED-OBJECT-FIELD-RMW-FUSION-MEASUREMENT.md"
TOOL="$ROOT_DIR/tools/allocator/typed_object_post_rmw_fusion_owner_refresh.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row224_owner_refresh.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

PERF_REPORT="$TMP_DIR/perf.report"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row224-owner-refresh] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=typed-object-post-rmw-fusion-owner-refresh-v0"
require_line "$DOC" "input_contract=typed-object-field-rmw-fusion-measurement-v0"
require_line "$DOC" "perf_exact_slot_helper_pct=62.27"
require_line "$DOC" "perf_exact_slot_get_set_pct=59.97"
require_line "$DOC" "perf_exact_slot_rmw_helper_pct=2.30"
require_line "$DOC" "perf_array_total_pct=28.62"
require_line "$DOC" "selected_boundary=typed_object_exact_slot_callsite_attribution"
require_line "$DOC" "next_diagnostic=typed_object_exact_slot_callsite_attribution"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat >"$PERF_REPORT" <<'REPORT'
    16.00%  app.exe  app.exe               [.] nyash.object.exact_slot_set_i64_hii
    13.98%  app.exe  app.exe               [.] core::hash::BuildHasher::hash_one
    11.42%  app.exe  app.exe               [.] nyash.object.exact_slot_get_u64_hii
     9.53%  app.exe  app.exe               [.] nyash.object.exact_slot_get_handle_hii
     9.20%  app.exe  app.exe               [.] nyash.object.exact_slot_set_u64_hiu
     8.47%  app.exe  app.exe               [.] nyash_kernel::plugin::array_slot_backend::single_thread_store_i64
     6.94%  app.exe  app.exe               [.] nyash.object.exact_slot_get_i64_hii
     6.88%  app.exe  app.exe               [.] nyash.object.exact_slot_set_handle_hii
     3.84%  app.exe  app.exe               [.] <core::hash::sip::Hasher<S> as core::hash::Hasher>::write
     2.33%  app.exe  app.exe               [.] nyash_kernel::plugin::array_slot_backend::single_thread_load_encoded_i64
     2.30%  app.exe  app.exe               [.] nyash.object.exact_slot_rmw_add_u64_hiii
     2.26%  app.exe  app.exe               [.] HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
     1.50%  app.exe  app.exe               [.] HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
     1.47%  app.exe  app.exe               [.] HakoAllocObjectLifecycleFacade.resetReleaseResult/0
     0.79%  app.exe  app.exe               [.] HakoAllocObjectLifecycleReleaseResult.reset/0
     0.77%  app.exe  app.exe               [.] HakoAllocPageModel.releaseLocalKnownLive/1
     0.76%  app.exe  app.exe               [.] HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
     0.76%  app.exe  app.exe               [.] HakoAllocPageModel.acquire_usize/1
     0.74%  app.exe  app.exe               [.] HakoAllocObjectLifecycleAllocResult.recordSuccess/1
REPORT

python3 "$TOOL" --perf-report "$PERF_REPORT" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=typed-object-post-rmw-fusion-owner-refresh-v0"
require_line "$REPORT" "input_contract=typed-object-field-rmw-fusion-measurement-v0"
require_line "$REPORT" "workload_id=representative-object-lifecycle-small-block-v0"
require_line "$REPORT" "perf_exact_slot_helper_pct=62.27"
require_line "$REPORT" "perf_exact_slot_get_set_pct=59.97"
require_line "$REPORT" "perf_exact_slot_rmw_helper_pct=2.30"
require_line "$REPORT" "perf_array_total_pct=28.62"
require_line "$REPORT" "selected_boundary=typed_object_exact_slot_callsite_attribution"
require_line "$REPORT" "next_diagnostic=typed_object_exact_slot_callsite_attribution"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
