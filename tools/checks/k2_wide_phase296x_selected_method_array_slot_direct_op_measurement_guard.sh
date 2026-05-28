#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-212-SELECTED-METHOD-ARRAY-SLOT-DIRECT-OP-MEASUREMENT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-211-SELECTED-METHOD-ARRAY-SLOT-DIRECT-OP-KEEPER.md"
TOOL="$ROOT_DIR/tools/allocator/selected_method_array_slot_direct_op_post_fusion_owner_refresh.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row212_array_direct_measure.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

PERF_REPORT="$TMP_DIR/perf.report"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row212-array-direct-measure] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=array-runtime-single-thread-store-backend-keeper-measurement-v0"
require_line "$DOC" "sample_count=3"
require_line "$DOC" "single_thread_exact_body_elapsed_ns=123000000"
require_line "$DOC" "keeper_effect=accepted"
require_line "$DOC" "output_contract=selected-method-array-slot-direct-op-post-fusion-owner-refresh-v0"
require_line "$DOC" "perf_field_helper_pct=51.04"
require_line "$DOC" "perf_array_slot_backend_pct=18.14"
require_line "$DOC" "perf_fused_direct_op_pct=0.89"
require_line "$DOC" "perf_array_backend_hash_pct=19.96"
require_line "$DOC" "perf_array_total_pct=38.99"
require_line "$DOC" "selected_boundary=typed_object_field_helper_lowering"
require_line "$DOC" "secondary_boundary=array_slot_backend_handle_map_hash"
require_line "$DOC" "next_diagnostic=typed_object_field_helper_subowner_refresh"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat >"$PERF_REPORT" <<'REPORT'
    17.79%  app.exe  app.exe               [.] nyash.object.field_set_hii
    16.43%  app.exe  app.exe               [.] core::hash::BuildHasher::hash_one
    14.23%  app.exe  app.exe               [.] nyash.object.field_set_u64_hiu
    10.45%  app.exe  app.exe               [.] nyash_kernel::plugin::array_slot_backend::single_thread_store_i64
     9.91%  app.exe  app.exe               [.] nyash.object.field_get_hii
     9.11%  app.exe  app.exe               [.] nyash.object.field_get_u64_hii
     7.69%  app.exe  app.exe               [.] nyash_kernel::plugin::array_slot_backend::single_thread_load_encoded_i64
     3.53%  app.exe  app.exe               [.] <core::hash::sip::Hasher<S> as core::hash::Hasher>::write
     0.89%  app.exe  app.exe               [.] nyash.array.slot_load_store_i64_hihi
REPORT

"$TOOL" --perf-report "$PERF_REPORT" --out "$REPORT"

require_line "$REPORT" "output_contract=selected-method-array-slot-direct-op-post-fusion-owner-refresh-v0"
require_line "$REPORT" "input_contract=selected-method-array-slot-direct-op-keeper-v0"
require_line "$REPORT" "workload_id=representative-object-lifecycle-small-block-v0"
require_line "$REPORT" "perf_field_helper_pct=51.04"
require_line "$REPORT" "perf_array_slot_backend_pct=18.14"
require_line "$REPORT" "perf_fused_direct_op_pct=0.89"
require_line "$REPORT" "perf_array_backend_hash_pct=19.96"
require_line "$REPORT" "perf_array_total_pct=38.99"
require_line "$REPORT" "selected_boundary=typed_object_field_helper_lowering"
require_line "$REPORT" "secondary_boundary=array_slot_backend_handle_map_hash"
require_line "$REPORT" "next_diagnostic=typed_object_field_helper_subowner_refresh"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
