#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-344-DIRECT-SLOT-SUPPORTED-STORAGE-NATIVEDIRECT-IMPLEMENTATION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-343-DIRECT-SLOT-SUPPORTED-STORAGE-NATIVEDIRECT-GUARD-SURFACE.md"
SRC="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_object_emit.inc"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TMP_DIR="$(mktemp -d /tmp/hakorune_row344_direct_slot_supported.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

REPORT="$TMP_DIR/report.out"
IR_DUMP="$TMP_DIR/generated.ll"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row344-direct-slot-supported-storage] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row344-direct-slot-supported-storage] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

forbid_pattern() {
  local file="$1"
  local pattern="$2"
  if grep -Fq "$pattern" "$file"; then
    echo "[row344-direct-slot-supported-storage] forbidden pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_positive_key() {
  local file="$1"
  local key="$2"
  if ! awk -F= -v key="$key" '$1 == key { found=1; exit !($2 + 0 > 0) } END { if (!found) exit 1 }' "$file"; then
    echo "[row344-direct-slot-supported-storage] expected positive ${key} in ${file#$ROOT_DIR/}" >&2
    exit 1
  fi
}

require_grep_count_ge() {
  local file="$1"
  local pattern="$2"
  local min_count="$3"
  local count
  count="$(grep -F "$pattern" "$file" | wc -l | tr -d ' ')"
  if (( count < min_count )); then
    echo "[row344-direct-slot-supported-storage] expected at least ${min_count} occurrences of ${pattern} in ${file#$ROOT_DIR/}, got ${count}" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-supported-storage-nativedirect-implementation-v0"
require_line "$DOC" "input_contract=direct-slot-supported-storage-nativedirect-guard-surface-v0"
require_line "$DOC" "implemented_owner=ny_llvmc_boundary_same_module_typed_object_emit"
require_line "$DOC" "implemented_owner_file=lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_object_emit.inc"
require_line "$DOC" "selected_backend=direct_slot_exact"
require_line "$DOC" "selection_kind=fact_driven_supported_storage"
require_line "$DOC" "selected_method_only=0"
require_line "$DOC" "selected_method_name_gate_removed=1"
require_line "$DOC" "by_name_hako_alloc_special_case=0"
require_line "$DOC" "required_receiver_fact=typed_object_binding"
require_line "$DOC" "required_slot_fact=typed_object_plan_runtime_slot"
require_line "$DOC" "required_storage_fact=typed_object_plan_storage"
require_line "$DOC" "supported_storage=i64,u64,usize,handle"
require_line "$DOC" "implemented_get_lowering=payload_load_i64"
require_line "$DOC" "implemented_set_lowering=payload_store_i64"
require_line "$DOC" "unsupported_storage_policy=existing_helper_route"
require_line "$DOC" "unsupported_narrow_integer_direct_store=0"
require_line "$DOC" "legacy_field_helper_internal_fast_lane=0"
require_line "$DOC" "runtime_helper_semantics_change=0"
require_line "$DOC" "mirbuilder_changes_allowed=0"
require_line "$DOC" "hako_source_changes_allowed=0"
require_line "$DOC" "direct_slot_payload_formula=object_base_plus_24_plus_slot_times_16_plus_8"
require_line "$DOC" "unsigned_set_nonnegative_trap_preserved=1"
require_line "$DOC" "exact_status_continue_label_preserved=1"
require_line "$DOC" "silent_fallback_allowed=0"
require_line "$DOC" "ffi_shim_rebuild_required=1"
require_line "$DOC" "direct_slot_ir_shape_smoke=ok"
require_line "$DOC" "exact_exe_semantic_smoke=ok"
require_line "$DOC" "summary=ok"

require_pattern "$SRC" "same_module_function_direct_slot_nativedirect_backend_enabled"
require_pattern "$SRC" "getenv(\"HAKO_TYPED_OBJECT_STORE\")"
require_pattern "$SRC" "typed_object_plan_field_runtime_slot_with_storage"
require_pattern "$SRC" "same_module_function_direct_slot_nativedirect_storage_supported(storage)"
require_pattern "$SRC" "same_module_function_emit_direct_slot_payload_ptr"
require_pattern "$SRC" "load i64, ptr %%%s"
require_pattern "$SRC" "store i64 %s, ptr %%%s"
require_pattern "$SRC" "same_module_function_emit_unsigned_nonnegative_trap"
require_pattern "$SRC" "note_exact_status_continue_label"
forbid_pattern "$SRC" "same_module_function_direct_slot_nativedirect_selected_method_enabled"
forbid_pattern "$SRC" "HakoAllocPageModel.acquire_usize/1"

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

require_pattern "$IR_DUMP" "direct_slot_payload_addr"
require_pattern "$IR_DUMP" "direct_slot_payload_ptr"
require_grep_count_ge "$IR_DUMP" "direct_slot_payload_addr" 20

if grep -Eq 'call .*@"nyash\.object\.field_(get|set)' "$IR_DUMP"; then
  echo "[row344-direct-slot-supported-storage] legacy typed-object field helper call remained in IR" >&2
  grep -En 'call .*@"nyash\.object\.field_(get|set)' "$IR_DUMP" >&2 || true
  exit 1
fi

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
output_contract=direct-slot-supported-storage-nativedirect-implementation-v0
input_contract=direct-slot-supported-storage-nativedirect-guard-surface-v0
workload_id=representative-object-lifecycle-small-block-v0
measurement_scope=exact_exe_object_lifecycle_direct_slot_exact_backend
direct_slot_payload_addr_count=$(grep -F "direct_slot_payload_addr" "$IR_DUMP" | wc -l | tr -d ' ')
legacy_field_helper_call_count=0
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
