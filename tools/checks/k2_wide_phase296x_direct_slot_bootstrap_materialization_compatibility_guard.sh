#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-341-DIRECT-SLOT-BOOTSTRAP-MATERIALIZATION-COMPATIBILITY.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-340-BOUNDARY-ROUTE-DIRECT-SLOT-NATIVEDIRECT-LOWERING-SELECTED-METHOD-PILOT.md"
STORE="$ROOT_DIR/crates/nyash_kernel/src/exports/typed_object_store.rs"
ARENA="$ROOT_DIR/crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TMP_DIR="$(mktemp -d /tmp/hakorune_row341_direct_slot_bootstrap.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

REPORT="$TMP_DIR/report.out"
IR_DUMP="$TMP_DIR/generated.ll"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row341-direct-slot-bootstrap] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row341-direct-slot-bootstrap] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_positive_key() {
  local file="$1"
  local key="$2"
  if ! awk -F= -v key="$key" '$1 == key { found=1; exit !($2 + 0 > 0) } END { if (!found) exit 1 }' "$file"; then
    echo "[row341-direct-slot-bootstrap] expected positive ${key} in ${file#$ROOT_DIR/}" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-bootstrap-materialization-compatibility-v0"
require_line "$DOC" "input_contract=boundary-route-direct-slot-nativedirect-lowering-selected-method-pilot-v0"
require_line "$DOC" "implemented_owner=typed_object_store_direct_slot_positive_handle_compatibility"
require_line "$DOC" "selected_backend=direct_slot_exact"
require_line "$DOC" "selected_hot_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "hot_selected_method_native_direct=preserved"
require_line "$DOC" "compatibility_scope=bootstrap_materialization_and_non_native_regions"
require_line "$DOC" "direct_slot_cell_primary_storage=1"
require_line "$DOC" "positive_direct_handle_generic_helper_get_supported=1"
require_line "$DOC" "positive_direct_handle_generic_helper_set_supported=1"
require_line "$DOC" "positive_direct_handle_exact_slot_helper_get_supported=1"
require_line "$DOC" "positive_direct_handle_exact_slot_helper_set_supported=1"
require_line "$DOC" "helper_fallback_hot_path_owner=0"
require_line "$DOC" "runtime_helper_compatibility_change=explicit"
require_line "$DOC" "materialized_snapshot_reads_primary_direct_cells=1"
require_line "$DOC" "direct_slot_ir_shape_smoke=ok"
require_line "$DOC" "exact_exe_semantic_smoke=ok"
require_line "$DOC" "summary=ok"

require_pattern "$STORE" "with_direct_slot_object"
require_pattern "$STORE" "with_direct_slot_object_mut"
require_pattern "$STORE" "object.get_legacy_i64(slot)"
require_pattern "$STORE" "object.set_legacy_i64(slot, value)"
require_pattern "$STORE" "object.exact_slot_record_alloc_success"
require_pattern "$ARENA" "pub(crate) fn get_legacy_i64"
require_pattern "$ARENA" "pub(crate) fn set_legacy_i64"
require_pattern "$ARENA" "pub(crate) fn exact_slot_rmw_add_u64"
require_pattern "$ARENA" "pub(crate) fn exact_slot_record_release_success"

HAKO_TYPED_OBJECT_STORE=direct_slot_exact cargo test -p nyash_kernel direct_slot_exact --lib >/dev/null
cargo build --release --bin hakorune >/dev/null
cargo build --release -p nyash-llvm-compiler --bin ny-llvmc >/dev/null
cargo build --release -p nyash_kernel >/dev/null
bash "$ROOT_DIR/tools/build_hako_llvmc_ffi.sh" >/dev/null

HAKO_ARRAY_SLOT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_STORE=direct_slot_exact \
NYASH_DISABLE_PLUGINS=1 \
NYASH_LLVM_DUMP_IR="$IR_DUMP" \
  "$ROOT_DIR/tools/allocator/hako_exe_memory_runner.sh" \
    --app "$APP" \
    --workload representative-object-lifecycle-small-block-v0 \
    --runtime-config empty \
    --operation-repeat 1 \
    --out "$REPORT" >/dev/null

require_pattern "$IR_DUMP" "define i64 @\"HakoAllocPageModel.acquire_usize/1\""
require_pattern "$IR_DUMP" "direct_slot_payload_addr"
require_pattern "$IR_DUMP" "direct_slot_payload_ptr"
require_pattern "$IR_DUMP" "load i64, ptr %direct_slot_payload_ptr"
require_pattern "$IR_DUMP" "store i64 %"

require_line "$REPORT" "output_contract=hako-exe-memory-evidence-v0"
require_line "$REPORT" "workload=representative-object-lifecycle-small-block-v0"
require_line "$REPORT" "runtime_config_profile=empty"
require_line "$REPORT" "result_code=0"
require_line "$REPORT" "operation_repeat=1"
require_line "$REPORT" "hako_body_timing_available=1"
require_line "$REPORT" "allocation_count=524288"
require_line "$REPORT" "free_count=524288"
require_line "$REPORT" "select_page_single_fast_path_count=524288"
require_line "$REPORT" "select_page_single_fallback_count=0"
require_line "$REPORT" "release_known_page_fast_path_count=524288"
require_line "$REPORT" "release_known_page_fallback_count=0"
require_line "$REPORT" "output_summary_ok=1"
require_line "$REPORT" "provider_activation=0"
require_line "$REPORT" "host_replacement=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator_installed=0"
require_line "$REPORT" "summary=ok"
require_positive_key "$REPORT" "body_elapsed_ns"
require_positive_key "$REPORT" "external_elapsed_ms"
require_positive_key "$REPORT" "peak_rss_bytes"

cat <<REPORT_TEXT
output_contract=direct-slot-bootstrap-materialization-compatibility-v0
input_contract=boundary-route-direct-slot-nativedirect-lowering-selected-method-pilot-v0
workload_id=representative-object-lifecycle-small-block-v0
measurement_scope=exact_exe_object_lifecycle_direct_slot_exact_backend
body_elapsed_ns=$(awk -F= '$1 == "body_elapsed_ns" { print $2 }' "$REPORT")
external_elapsed_ms=$(awk -F= '$1 == "external_elapsed_ms" { print $2 }' "$REPORT")
peak_rss_bytes=$(awk -F= '$1 == "peak_rss_bytes" { print $2 }' "$REPORT")
semantic_proof_summary=ok
provider_activation=0
host_replacement=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
REPORT_TEXT
