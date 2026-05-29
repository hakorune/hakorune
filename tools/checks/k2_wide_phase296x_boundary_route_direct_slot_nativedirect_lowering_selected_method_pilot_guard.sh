#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-340-BOUNDARY-ROUTE-DIRECT-SLOT-NATIVEDIRECT-LOWERING-SELECTED-METHOD-PILOT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-339-DIRECT-SLOT-NATIVEDIRECT-LOWERING-DAILY-OWNER-GAP-DIAGNOSTIC.md"
SRC="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_object_emit.inc"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TMP_DIR="$(mktemp -d /tmp/hakorune_row340_boundary_direct_slot.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

IR_DUMP="$TMP_DIR/generated.ll"
MIR_JSON="$TMP_DIR/app.mir.json"
EXE="$TMP_DIR/app.exe"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row340-boundary-direct-slot-lowering] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row340-boundary-direct-slot-lowering] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=boundary-route-direct-slot-nativedirect-lowering-selected-method-pilot-v0"
require_line "$DOC" "input_contract=direct-slot-nativedirect-lowering-daily-owner-gap-diagnostic-v0"
require_line "$DOC" "implemented_owner=ny_llvmc_boundary_same_module_typed_object_emit"
require_line "$DOC" "implemented_owner_file=lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_object_emit.inc"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "selected_backend=direct_slot_exact"
require_line "$DOC" "initial_selected_method_only=1"
require_line "$DOC" "selected_method_pilot_superseded_by_supported_storage_nativedirect=1"
require_line "$DOC" "direct_slot_exact_only=1"
require_line "$DOC" "llvmlite_keep_lane_changes_allowed=0"
require_line "$DOC" "generic_direct_slot_rewrite_allowed=0"
require_line "$DOC" "direct_slot_payload_formula=object_base_plus_24_plus_slot_times_16_plus_8"
require_line "$DOC" "implemented_get_lowering=payload_load_i64"
require_line "$DOC" "implemented_set_lowering=payload_store_i64"
require_line "$DOC" "supported_storage=i64,u64,usize,handle"
require_line "$DOC" "unsigned_set_nonnegative_trap_preserved=1"
require_line "$DOC" "direct_set_status_continue_branch_preserved=1"
require_line "$DOC" "exact_status_continue_label_preserved=1"
require_line "$DOC" "non_selected_method_policy=existing_helper_path"
require_line "$DOC" "silent_fallback_allowed=0"
require_line "$DOC" "ffi_shim_rebuild_required=1"
require_line "$DOC" "direct_slot_ir_shape_smoke=ok"
require_line "$DOC" "exact_exe_semantic_smoke=blocked_by_direct_slot_bootstrap_materialization_boundary"
require_line "$DOC" "blocked_owner=direct_slot_positive_handle_bootstrap_materialization"
require_line "$DOC" "body_elapsed_positive=0"
require_line "$DOC" "summary=ok"

require_pattern "$SRC" "same_module_function_direct_slot_nativedirect_backend_enabled"
require_pattern "$SRC" "HAKO_TYPED_OBJECT_STORE"
require_pattern "$SRC" "direct_slot_exact"
require_pattern "$SRC" "same_module_function_emit_direct_slot_payload_ptr"
require_pattern "$SRC" "direct_slot_payload_addr"
require_pattern "$SRC" "inttoptr i64"
require_pattern "$SRC" "load i64, ptr %"
require_pattern "$SRC" "store i64 %"
require_pattern "$SRC" "br label %%exact_status_continue_%lld_%zu"
require_pattern "$SRC" "exact_status_continue_%lld_%zu"
require_pattern "$SRC" "note_exact_status_continue_label"

cargo build --release --bin hakorune >/dev/null
cargo build --release -p nyash-llvm-compiler --bin ny-llvmc >/dev/null
cargo build --release -p nyash_kernel >/dev/null
bash "$ROOT_DIR/tools/build_hako_llvmc_ffi.sh" >/dev/null

NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >/dev/null

HAKO_TYPED_OBJECT_STORE=direct_slot_exact \
NYASH_DISABLE_PLUGINS=1 \
NYASH_LLVM_DUMP_IR="$IR_DUMP" \
  "$ROOT_DIR/tools/selfhost/selfhost_build.sh" --mir-in "$MIR_JSON" --exe "$EXE" >/dev/null

require_pattern "$IR_DUMP" "define i64 @\"HakoAllocPageModel.acquire_usize/1\""
require_pattern "$IR_DUMP" "direct_slot_payload_addr"
require_pattern "$IR_DUMP" "direct_slot_payload_ptr"
require_pattern "$IR_DUMP" "load i64, ptr %direct_slot_payload_ptr"
require_pattern "$IR_DUMP" "store i64 %"

cat <<REPORT_TEXT
output_contract=boundary-route-direct-slot-nativedirect-lowering-selected-method-pilot-v0
input_contract=direct-slot-nativedirect-lowering-daily-owner-gap-diagnostic-v0
workload_id=representative-object-lifecycle-small-block-v0
measurement_scope=ir_shape_only
direct_slot_ir_shape_smoke=ok
semantic_proof_summary=blocked_by_direct_slot_bootstrap_materialization_boundary
provider_activation=0
host_replacement=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
REPORT_TEXT
