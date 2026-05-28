#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-217-POST-EXACT-SLOT-DIRECT-HELPER-OWNER-REFRESH.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-216-TYPED-OBJECT-EXACT-SLOT-DIRECT-HELPER-MEASUREMENT.md"
TOOL="$ROOT_DIR/tools/allocator/typed_object_exact_slot_owner_refresh.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row217_owner_refresh.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

PERF_REPORT="$TMP_DIR/perf.report"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row217-owner-refresh] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=typed-object-exact-slot-owner-refresh-v0"
require_line "$DOC" "perf_exact_slot_helper_pct=55.04"
require_line "$DOC" "perf_array_total_pct=38.39"
require_line "$DOC" "selected_boundary=mir_typed_field_direct_op_inventory"
require_line "$DOC" "next_diagnostic=mir_typed_field_direct_op_net_inventory"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat >"$PERF_REPORT" <<'REPORT'
    19.73%  app.exe  app.exe               [.] nyash.object.exact_slot_get_u64_hii
    17.28%  app.exe  app.exe               [.] nyash_kernel::plugin::array_slot_backend::single_thread_store_i64
    14.95%  app.exe  app.exe               [.] nyash.object.exact_slot_set_i64_hii
    14.59%  app.exe  app.exe               [.] core::hash::BuildHasher::hash_one
     7.04%  app.exe  app.exe               [.] nyash.object.exact_slot_get_handle_hii
     4.90%  app.exe  app.exe               [.] nyash.object.exact_slot_set_u64_hiu
     4.88%  app.exe  app.exe               [.] nyash.object.exact_slot_get_i64_hii
     4.10%  app.exe  app.exe               [.] <core::hash::sip::Hasher<S> as core::hash::Hasher>::write
     3.54%  app.exe  app.exe               [.] nyash.object.exact_slot_set_handle_hii
     2.42%  app.exe  app.exe               [.] nyash_kernel::plugin::array_slot_backend::single_thread_load_encoded_i64
     1.62%  app.exe  app.exe               [.] HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
     1.61%  app.exe  app.exe               [.] HakoAllocPageModel.acquire_usize/1
REPORT

"$TOOL" --perf-report "$PERF_REPORT" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=typed-object-exact-slot-owner-refresh-v0"
require_line "$REPORT" "input_contract=typed-object-exact-slot-direct-helper-measurement-v0"
require_line "$REPORT" "workload_id=representative-object-lifecycle-small-block-v0"
require_line "$REPORT" "perf_exact_slot_helper_pct=55.04"
require_line "$REPORT" "perf_array_total_pct=38.39"
require_line "$REPORT" "selected_boundary=mir_typed_field_direct_op_inventory"
require_line "$REPORT" "next_diagnostic=mir_typed_field_direct_op_net_inventory"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
