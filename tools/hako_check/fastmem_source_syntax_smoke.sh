#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
BIN="${NYASH_BIN:-$ROOT/target/release/hakorune}"
if [ ! -x "$BIN" ]; then
  BIN="$ROOT/target/release/nyash"
fi

if [ ! -x "$BIN" ]; then
  echo "[TEST/FAIL] hakorune/nyash binary not found: $BIN" >&2
  exit 2
fi

FEATURES="${FASTMEM_SOURCE_FEATURES:-stage3,rune}"
TMPDIR="$(mktemp -d "${TMPDIR:-/tmp}/hako_fastmem_source.XXXXXX")"
cleanup() {
  rm -rf "$TMPDIR"
}
trap cleanup EXIT

GOOD_SRC="$TMPDIR/good.hako"
GOOD_AST="$TMPDIR/good.ast.json"
GOOD_INV="$TMPDIR/good.inventory.kv"
GOOD_CHECK="$TMPDIR/good.check.kv"
BAD_SRC="$TMPDIR/bad.hako"
BAD_AST="$TMPDIR/bad.ast.json"
BAD_INV="$TMPDIR/bad.inventory.kv"
BAD_CHECK="$TMPDIR/bad.check.kv"
PILOT_SRC="$ROOT/lang/src/hako_alloc/memory/page_meta_fastmem_pilot_box.hako"
PILOT_AST="$TMPDIR/page_meta_pilot.ast.json"
PILOT_MIR="$TMPDIR/page_meta_pilot.mir.json"
PILOT_INV="$TMPDIR/page_meta_pilot.inventory.kv"
PILOT_MIR_INV="$TMPDIR/page_meta_pilot.mir.inventory.kv"
PILOT_LLVM_REPORT="$TMPDIR/page_meta_pilot.llvm.report.kv"
PILOT_LLVM_CHECK="$TMPDIR/page_meta_pilot.llvm.check.kv"
OWNER_SRC="$ROOT/lang/src/hako_alloc/memory/page_meta_owner_read_fastmem_pilot_box.hako"
OWNER_AST="$TMPDIR/page_meta_owner.ast.json"
OWNER_MIR="$TMPDIR/page_meta_owner.mir.json"
OWNER_INV="$TMPDIR/page_meta_owner.inventory.kv"
OWNER_MIR_INV="$TMPDIR/page_meta_owner.mir.inventory.kv"
OWNER_LLVM_REPORT="$TMPDIR/page_meta_owner.llvm.report.kv"
OWNER_LLVM_CHECK="$TMPDIR/page_meta_owner.llvm.check.kv"
FREE_HEAD_SRC="$ROOT/lang/src/hako_alloc/memory/page_meta_free_head_read_fastmem_pilot_box.hako"
FREE_HEAD_AST="$TMPDIR/page_meta_free_head.ast.json"
FREE_HEAD_MIR="$TMPDIR/page_meta_free_head.mir.json"
FREE_HEAD_INV="$TMPDIR/page_meta_free_head.inventory.kv"
FREE_HEAD_MIR_INV="$TMPDIR/page_meta_free_head.mir.inventory.kv"
FREE_HEAD_LLVM_REPORT="$TMPDIR/page_meta_free_head.llvm.report.kv"
FREE_HEAD_LLVM_CHECK="$TMPDIR/page_meta_free_head.llvm.check.kv"
OWNER_EQ_SRC="$ROOT/lang/src/hako_alloc/memory/page_meta_owner_eq_fastmem_pilot_box.hako"
OWNER_EQ_AST="$TMPDIR/page_meta_owner_eq.ast.json"
OWNER_EQ_MIR="$TMPDIR/page_meta_owner_eq.mir.json"
OWNER_EQ_INV="$TMPDIR/page_meta_owner_eq.inventory.kv"
OWNER_EQ_MIR_INV="$TMPDIR/page_meta_owner_eq.mir.inventory.kv"
OWNER_EQ_LLVM_REPORT="$TMPDIR/page_meta_owner_eq.llvm.report.kv"
OWNER_EQ_LLVM_CHECK="$TMPDIR/page_meta_owner_eq.llvm.check.kv"
LOCAL_FREE_HEAD_SRC="$ROOT/lang/src/hako_alloc/memory/page_meta_local_free_head_preflight_box.hako"
LOCAL_FREE_HEAD_AST="$TMPDIR/page_meta_local_free_head.ast.json"
LOCAL_FREE_HEAD_MIR="$TMPDIR/page_meta_local_free_head.mir.json"
LOCAL_FREE_HEAD_INV="$TMPDIR/page_meta_local_free_head.inventory.kv"
LOCAL_FREE_HEAD_MIR_INV="$TMPDIR/page_meta_local_free_head.mir.inventory.kv"
LOCAL_FREE_HEAD_LLVM_REPORT="$TMPDIR/page_meta_local_free_head.llvm.report.kv"
LOCAL_FREE_HEAD_LLVM_STDERR="$TMPDIR/page_meta_local_free_head.llvm.stderr"
LOCAL_FREE_MEMOP_SRC="$ROOT/lang/src/hako_alloc/memory/page_meta_local_free_memop_vocabulary_box.hako"
LOCAL_FREE_MEMOP_AST="$TMPDIR/page_meta_local_free_memop.ast.json"
LOCAL_FREE_MEMOP_MIR="$TMPDIR/page_meta_local_free_memop.mir.json"
LOCAL_FREE_MEMOP_INV="$TMPDIR/page_meta_local_free_memop.inventory.kv"
LOCAL_FREE_MEMOP_MIR_INV="$TMPDIR/page_meta_local_free_memop.mir.inventory.kv"
LOCAL_FREE_MEMOP_LLVM_REPORT="$TMPDIR/page_meta_local_free_memop.llvm.report.kv"
LOCAL_FREE_MEMOP_LLVM_STDERR="$TMPDIR/page_meta_local_free_memop.llvm.stderr"
LOCAL_FREE_PUSH_PRECONDITION_SRC="$ROOT/lang/src/hako_alloc/memory/page_meta_local_free_push_precondition_box.hako"
LOCAL_FREE_PUSH_PRECONDITION_AST="$TMPDIR/page_meta_local_free_push_precondition.ast.json"
LOCAL_FREE_PUSH_PRECONDITION_MIR="$TMPDIR/page_meta_local_free_push_precondition.mir.json"
LOCAL_FREE_PUSH_PRECONDITION_INV="$TMPDIR/page_meta_local_free_push_precondition.inventory.kv"
LOCAL_FREE_PUSH_PRECONDITION_MIR_INV="$TMPDIR/page_meta_local_free_push_precondition.mir.inventory.kv"
LOCAL_FREE_PUSH_PRECONDITION_LLVM_REPORT="$TMPDIR/page_meta_local_free_push_precondition.llvm.report.kv"
LOCAL_FREE_PUSH_PRECONDITION_LLVM_CHECK="$TMPDIR/page_meta_local_free_push_precondition.llvm.check.kv"
LOCAL_FREE_PUSH_PRECONDITION_LLVM_STDERR="$TMPDIR/page_meta_local_free_push_precondition.llvm.stderr"
LOCAL_FREE_POP_PRECONDITION_SRC="$ROOT/lang/src/hako_alloc/memory/page_meta_local_free_pop_precondition_box.hako"
LOCAL_FREE_POP_PRECONDITION_AST="$TMPDIR/page_meta_local_free_pop_precondition.ast.json"
LOCAL_FREE_POP_PRECONDITION_MIR="$TMPDIR/page_meta_local_free_pop_precondition.mir.json"
LOCAL_FREE_POP_PRECONDITION_INV="$TMPDIR/page_meta_local_free_pop_precondition.inventory.kv"
LOCAL_FREE_POP_PRECONDITION_MIR_INV="$TMPDIR/page_meta_local_free_pop_precondition.mir.inventory.kv"
LOCAL_FREE_POP_PRECONDITION_LLVM_REPORT="$TMPDIR/page_meta_local_free_pop_precondition.llvm.report.kv"
LOCAL_FREE_POP_PRECONDITION_LLVM_CHECK="$TMPDIR/page_meta_local_free_pop_precondition.llvm.check.kv"
LOCAL_FREE_POP_PRECONDITION_LLVM_STDERR="$TMPDIR/page_meta_local_free_pop_precondition.llvm.stderr"

cat >"$GOOD_SRC" <<'HK'
static box Main {
  main(ptr) {
    local page_table = ptr
    local page_index = 0
    fastmem PageMapV0 {
      local addr = mem.addr(ptr)
      local key = (addr >> 12) & 255
      local page = page_table[page_index]
      local capacity = page.capacity
      page.used = capacity
    }
    return 0
  }
}
HK

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$GOOD_AST" "$GOOD_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --ast-json "$GOOD_AST" \
  --out "$GOOD_INV"

grep -q '^input_kind=ast_json$' "$GOOD_INV"
grep -q '^measured_hot_path_owner=hako_source$' "$GOOD_INV"
grep -q '^fastmem_region_count=1$' "$GOOD_INV"
grep -q '^fastmem_contract_count=1$' "$GOOD_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$GOOD_INV"
grep -q '^fastmem_contract_family=allocator.page_map$' "$GOOD_INV"
grep -q '^fastmem_memop_region_begin_count=1$' "$GOOD_INV"
grep -q '^fastmem_memop_region_end_count=1$' "$GOOD_INV"
grep -q '^fastmem_memop_unbalanced_region_count=0$' "$GOOD_INV"
grep -q '^fastmem_memop_addr_of_count=1$' "$GOOD_INV"
grep -q '^fastmem_memop_logical_shr_count=1$' "$GOOD_INV"
grep -q '^fastmem_memop_and_count=1$' "$GOOD_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$GOOD_INV"
grep -q '^fastmem_memop_field_load_count=1$' "$GOOD_INV"
grep -q '^fastmem_memop_field_store_count=1$' "$GOOD_INV"
grep -q '^fastmem_forbidden_call_count=0$' "$GOOD_INV"
grep -q '^fastmem_type_abi_hot_lookup_count=0$' "$GOOD_INV"
grep -q '^fastmem_provider_abi_crossing_count=0$' "$GOOD_INV"
grep -q '^summary=ok$' "$GOOD_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --ast-json "$GOOD_AST" \
  --format kv \
  --out "$GOOD_CHECK"
grep -q '^summary=ok$' "$GOOD_CHECK"
grep -q '^failure_count=0$' "$GOOD_CHECK"

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$PILOT_AST" "$PILOT_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$PILOT_MIR" "$PILOT_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --ast-json "$PILOT_AST" \
  --out "$PILOT_INV"

grep -q '^input_kind=ast_json$' "$PILOT_INV"
grep -q '^fastmem_region_count=1$' "$PILOT_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$PILOT_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$PILOT_INV"
grep -q '^fastmem_memop_field_load_count=3$' "$PILOT_INV"
grep -q '^fastmem_memop_field_store_count=1$' "$PILOT_INV"
grep -q '^fastmem_memop_add_count=2$' "$PILOT_INV"
grep -q '^fastmem_forbidden_call_count=0$' "$PILOT_INV"
grep -q '^summary=ok$' "$PILOT_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --mir-json "$PILOT_MIR" \
  --out "$PILOT_MIR_INV"

grep -q '^input_kind=mir_json_metadata$' "$PILOT_MIR_INV"
grep -q '^fastmem_region_count=1$' "$PILOT_MIR_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$PILOT_MIR_INV"
grep -q '^replacement_front_mir_memop_enabled=1$' "$PILOT_MIR_INV"
grep -q '^replacement_front_mir_fastmem_region_enabled=1$' "$PILOT_MIR_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$PILOT_MIR_INV"
grep -q '^fastmem_memop_field_load_count=3$' "$PILOT_MIR_INV"
grep -q '^fastmem_memop_field_store_count=1$' "$PILOT_MIR_INV"
grep -q '^fastmem_verified_mem_access_plan_count=5$' "$PILOT_MIR_INV"
grep -q '^fastmem_verified_field_access_count=4$' "$PILOT_MIR_INV"
grep -q '^fastmem_verified_table_access_count=1$' "$PILOT_MIR_INV"
grep -q '^fastmem_table_index_unchecked_count=0$' "$PILOT_MIR_INV"
grep -q '^fastmem_table_access_proof_incomplete_count=0$' "$PILOT_MIR_INV"
grep -q '^fastmem_table_overflow_proof_missing_count=0$' "$PILOT_MIR_INV"
grep -q '^fastmem_field_id_missing_count=0$' "$PILOT_MIR_INV"
grep -q '^fastmem_table_id_missing_count=0$' "$PILOT_MIR_INV"
grep -q '^fastmem_unknown_alignment_count=0$' "$PILOT_MIR_INV"
grep -q '^summary=ok$' "$PILOT_MIR_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --mir-json "$PILOT_MIR" \
  --out "$PILOT_LLVM_REPORT"

grep -q '^output_contract=hako-check-fastmem-mir-to-llvm-producer-report-v0$' "$PILOT_LLVM_REPORT"
grep -q '^replacement_front_producer=mir_to_llvm_lowering$' "$PILOT_LLVM_REPORT"
grep -q '^replacement_front_backend_artifact=object$' "$PILOT_LLVM_REPORT"
grep -q '^mir_fmem_008b_layout_table_producer_pilot=1$' "$PILOT_LLVM_REPORT"
grep -q '^replacement_front_selected_memop_kinds=TableIndex,FieldLoad,FieldStore$' "$PILOT_LLVM_REPORT"
grep -q '^replacement_front_deferred_memop_kinds=CurrentAllocOwnerId,OwnerEq$' "$PILOT_LLVM_REPORT"
grep -q '^memop_table_index_lowered_count=1$' "$PILOT_LLVM_REPORT"
grep -q '^memop_field_load_lowered_count=3$' "$PILOT_LLVM_REPORT"
grep -q '^memop_field_store_lowered_count=1$' "$PILOT_LLVM_REPORT"
grep -q '^memop_current_alloc_owner_id_lowered_count=0$' "$PILOT_LLVM_REPORT"
grep -q '^memop_owner_eq_lowered_count=0$' "$PILOT_LLVM_REPORT"
grep -q '^memop_atomic_remote_head_lowered_count=0$' "$PILOT_LLVM_REPORT"
grep -q '^fastmem_raw_pointer_in_ordinary_vmap_count=0$' "$PILOT_LLVM_REPORT"
grep -q '^fastmem_layout_ref_escape_count=0$' "$PILOT_LLVM_REPORT"
grep -q '^fastmem_lowering_recomputed_layout_offset_count=0$' "$PILOT_LLVM_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$PILOT_LLVM_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' "$PILOT_LLVM_REPORT"
grep -q '^product_activation=0$' "$PILOT_LLVM_REPORT"
grep -q '^summary=ok$' "$PILOT_LLVM_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$PILOT_LLVM_REPORT" \
  --format kv \
  --out "$PILOT_LLVM_CHECK"
grep -q '^summary=ok$' "$PILOT_LLVM_CHECK"
grep -q '^failure_count=0$' "$PILOT_LLVM_CHECK"

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$OWNER_AST" "$OWNER_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$OWNER_MIR" "$OWNER_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --ast-json "$OWNER_AST" \
  --out "$OWNER_INV"

grep -q '^input_kind=ast_json$' "$OWNER_INV"
grep -q '^fastmem_region_count=1$' "$OWNER_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$OWNER_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$OWNER_INV"
grep -q '^fastmem_memop_field_load_count=4$' "$OWNER_INV"
grep -q '^fastmem_memop_field_store_count=1$' "$OWNER_INV"
grep -q '^fastmem_memop_add_count=3$' "$OWNER_INV"
grep -q '^fastmem_forbidden_call_count=0$' "$OWNER_INV"
grep -q '^summary=ok$' "$OWNER_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --mir-json "$OWNER_MIR" \
  --out "$OWNER_MIR_INV"

grep -q '^input_kind=mir_json_metadata$' "$OWNER_MIR_INV"
grep -q '^fastmem_region_count=1$' "$OWNER_MIR_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$OWNER_MIR_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$OWNER_MIR_INV"
grep -q '^fastmem_memop_field_load_count=4$' "$OWNER_MIR_INV"
grep -q '^fastmem_memop_field_store_count=1$' "$OWNER_MIR_INV"
grep -q '^fastmem_verified_mem_access_plan_count=6$' "$OWNER_MIR_INV"
grep -q '^fastmem_verified_field_access_count=5$' "$OWNER_MIR_INV"
grep -q '^fastmem_verified_table_access_count=1$' "$OWNER_MIR_INV"
grep -q '^fastmem_table_index_unchecked_count=0$' "$OWNER_MIR_INV"
grep -q '^fastmem_table_access_proof_incomplete_count=0$' "$OWNER_MIR_INV"
grep -q '^fastmem_table_overflow_proof_missing_count=0$' "$OWNER_MIR_INV"
grep -q '^fastmem_field_id_missing_count=0$' "$OWNER_MIR_INV"
grep -q '^fastmem_table_id_missing_count=0$' "$OWNER_MIR_INV"
grep -q '^fastmem_unknown_alignment_count=0$' "$OWNER_MIR_INV"
grep -q '^summary=ok$' "$OWNER_MIR_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --mir-json "$OWNER_MIR" \
  --out "$OWNER_LLVM_REPORT"

grep -q '^replacement_front_producer=mir_to_llvm_lowering$' "$OWNER_LLVM_REPORT"
grep -q '^replacement_front_selected_memop_kinds=TableIndex,FieldLoad,FieldStore$' "$OWNER_LLVM_REPORT"
grep -q '^replacement_front_deferred_memop_kinds=CurrentAllocOwnerId,OwnerEq$' "$OWNER_LLVM_REPORT"
grep -q '^memop_table_index_lowered_count=1$' "$OWNER_LLVM_REPORT"
grep -q '^memop_field_load_lowered_count=4$' "$OWNER_LLVM_REPORT"
grep -q '^memop_field_store_lowered_count=1$' "$OWNER_LLVM_REPORT"
grep -q '^memop_current_alloc_owner_id_lowered_count=0$' "$OWNER_LLVM_REPORT"
grep -q '^memop_owner_eq_lowered_count=0$' "$OWNER_LLVM_REPORT"
grep -q '^memop_atomic_remote_head_lowered_count=0$' "$OWNER_LLVM_REPORT"
grep -q '^fastmem_layout_ref_escape_count=0$' "$OWNER_LLVM_REPORT"
grep -q '^fastmem_lowering_recomputed_layout_offset_count=0$' "$OWNER_LLVM_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$OWNER_LLVM_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' "$OWNER_LLVM_REPORT"
grep -q '^product_activation=0$' "$OWNER_LLVM_REPORT"
grep -q '^summary=ok$' "$OWNER_LLVM_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$OWNER_LLVM_REPORT" \
  --format kv \
  --out "$OWNER_LLVM_CHECK"
grep -q '^summary=ok$' "$OWNER_LLVM_CHECK"
grep -q '^failure_count=0$' "$OWNER_LLVM_CHECK"

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$FREE_HEAD_AST" "$FREE_HEAD_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$FREE_HEAD_MIR" "$FREE_HEAD_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --ast-json "$FREE_HEAD_AST" \
  --out "$FREE_HEAD_INV"

grep -q '^input_kind=ast_json$' "$FREE_HEAD_INV"
grep -q '^fastmem_region_count=1$' "$FREE_HEAD_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$FREE_HEAD_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$FREE_HEAD_INV"
grep -q '^fastmem_memop_field_load_count=5$' "$FREE_HEAD_INV"
grep -q '^fastmem_memop_field_store_count=1$' "$FREE_HEAD_INV"
grep -q '^fastmem_memop_add_count=3$' "$FREE_HEAD_INV"
grep -q '^fastmem_forbidden_call_count=0$' "$FREE_HEAD_INV"
grep -q '^summary=ok$' "$FREE_HEAD_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --mir-json "$FREE_HEAD_MIR" \
  --out "$FREE_HEAD_MIR_INV"

grep -q '^input_kind=mir_json_metadata$' "$FREE_HEAD_MIR_INV"
grep -q '^fastmem_region_count=1$' "$FREE_HEAD_MIR_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$FREE_HEAD_MIR_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$FREE_HEAD_MIR_INV"
grep -q '^fastmem_memop_field_load_count=5$' "$FREE_HEAD_MIR_INV"
grep -q '^fastmem_memop_field_store_count=1$' "$FREE_HEAD_MIR_INV"
grep -q '^fastmem_verified_mem_access_plan_count=7$' "$FREE_HEAD_MIR_INV"
grep -q '^fastmem_verified_field_access_count=6$' "$FREE_HEAD_MIR_INV"
grep -q '^fastmem_verified_table_access_count=1$' "$FREE_HEAD_MIR_INV"
grep -q '^fastmem_table_index_unchecked_count=0$' "$FREE_HEAD_MIR_INV"
grep -q '^fastmem_table_access_proof_incomplete_count=0$' "$FREE_HEAD_MIR_INV"
grep -q '^fastmem_table_overflow_proof_missing_count=0$' "$FREE_HEAD_MIR_INV"
grep -q '^fastmem_field_id_missing_count=0$' "$FREE_HEAD_MIR_INV"
grep -q '^fastmem_table_id_missing_count=0$' "$FREE_HEAD_MIR_INV"
grep -q '^fastmem_unknown_alignment_count=0$' "$FREE_HEAD_MIR_INV"
grep -q '^summary=ok$' "$FREE_HEAD_MIR_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --mir-json "$FREE_HEAD_MIR" \
  --out "$FREE_HEAD_LLVM_REPORT"

grep -q '^replacement_front_producer=mir_to_llvm_lowering$' "$FREE_HEAD_LLVM_REPORT"
grep -q '^replacement_front_selected_memop_kinds=TableIndex,FieldLoad,FieldStore$' "$FREE_HEAD_LLVM_REPORT"
grep -q '^replacement_front_deferred_memop_kinds=CurrentAllocOwnerId,OwnerEq$' "$FREE_HEAD_LLVM_REPORT"
grep -q '^memop_table_index_lowered_count=1$' "$FREE_HEAD_LLVM_REPORT"
grep -q '^memop_field_load_lowered_count=5$' "$FREE_HEAD_LLVM_REPORT"
grep -q '^memop_field_store_lowered_count=1$' "$FREE_HEAD_LLVM_REPORT"
grep -q '^memop_current_alloc_owner_id_lowered_count=0$' "$FREE_HEAD_LLVM_REPORT"
grep -q '^memop_owner_eq_lowered_count=0$' "$FREE_HEAD_LLVM_REPORT"
grep -q '^memop_atomic_remote_head_lowered_count=0$' "$FREE_HEAD_LLVM_REPORT"
grep -q '^fastmem_raw_pointer_in_ordinary_vmap_count=0$' "$FREE_HEAD_LLVM_REPORT"
grep -q '^fastmem_layout_ref_escape_count=0$' "$FREE_HEAD_LLVM_REPORT"
grep -q '^fastmem_lowering_recomputed_layout_offset_count=0$' "$FREE_HEAD_LLVM_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$FREE_HEAD_LLVM_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' "$FREE_HEAD_LLVM_REPORT"
grep -q '^product_activation=0$' "$FREE_HEAD_LLVM_REPORT"
grep -q '^summary=ok$' "$FREE_HEAD_LLVM_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FREE_HEAD_LLVM_REPORT" \
  --format kv \
  --out "$FREE_HEAD_LLVM_CHECK"
grep -q '^summary=ok$' "$FREE_HEAD_LLVM_CHECK"
grep -q '^failure_count=0$' "$FREE_HEAD_LLVM_CHECK"

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$OWNER_EQ_AST" "$OWNER_EQ_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$OWNER_EQ_MIR" "$OWNER_EQ_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --ast-json "$OWNER_EQ_AST" \
  --out "$OWNER_EQ_INV"

grep -q '^input_kind=ast_json$' "$OWNER_EQ_INV"
grep -q '^fastmem_region_count=1$' "$OWNER_EQ_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$OWNER_EQ_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$OWNER_EQ_INV"
grep -q '^fastmem_memop_field_load_count=1$' "$OWNER_EQ_INV"
grep -q '^fastmem_memop_field_store_count=1$' "$OWNER_EQ_INV"
grep -q '^fastmem_memop_current_alloc_owner_id_count=1$' "$OWNER_EQ_INV"
grep -q '^fastmem_memop_owner_eq_count=1$' "$OWNER_EQ_INV"
grep -q '^fastmem_forbidden_call_count=0$' "$OWNER_EQ_INV"
grep -q '^summary=ok$' "$OWNER_EQ_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --mir-json "$OWNER_EQ_MIR" \
  --out "$OWNER_EQ_MIR_INV"

grep -q '^input_kind=mir_json_metadata$' "$OWNER_EQ_MIR_INV"
grep -q '^fastmem_region_count=1$' "$OWNER_EQ_MIR_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$OWNER_EQ_MIR_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$OWNER_EQ_MIR_INV"
grep -q '^fastmem_memop_field_load_count=1$' "$OWNER_EQ_MIR_INV"
grep -q '^fastmem_memop_field_store_count=1$' "$OWNER_EQ_MIR_INV"
grep -q '^fastmem_memop_current_alloc_owner_id_count=1$' "$OWNER_EQ_MIR_INV"
grep -q '^fastmem_memop_owner_eq_count=1$' "$OWNER_EQ_MIR_INV"
grep -q '^fastmem_verified_mem_access_plan_count=3$' "$OWNER_EQ_MIR_INV"
grep -q '^fastmem_verified_field_access_count=2$' "$OWNER_EQ_MIR_INV"
grep -q '^fastmem_verified_table_access_count=1$' "$OWNER_EQ_MIR_INV"
grep -q '^fastmem_table_index_unchecked_count=0$' "$OWNER_EQ_MIR_INV"
grep -q '^fastmem_table_access_proof_incomplete_count=0$' "$OWNER_EQ_MIR_INV"
grep -q '^fastmem_table_overflow_proof_missing_count=0$' "$OWNER_EQ_MIR_INV"
grep -q '^fastmem_field_id_missing_count=0$' "$OWNER_EQ_MIR_INV"
grep -q '^fastmem_table_id_missing_count=0$' "$OWNER_EQ_MIR_INV"
grep -q '^fastmem_unknown_alignment_count=0$' "$OWNER_EQ_MIR_INV"
grep -q '^summary=ok$' "$OWNER_EQ_MIR_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile owner-runtime \
  --mir-json "$OWNER_EQ_MIR" \
  --out "$OWNER_EQ_LLVM_REPORT"

grep -q '^replacement_front_producer=mir_to_llvm_lowering$' "$OWNER_EQ_LLVM_REPORT"
grep -q '^replacement_front_selected_memop_family=owner_runtime$' "$OWNER_EQ_LLVM_REPORT"
grep -q '^replacement_front_selected_memop_kinds=CurrentAllocOwnerId,OwnerEq$' "$OWNER_EQ_LLVM_REPORT"
grep -q '^replacement_front_deferred_memop_kinds=AtomicRemoteHead$' "$OWNER_EQ_LLVM_REPORT"
grep -q '^fastmem_owner_runtime_producer_pilot=1$' "$OWNER_EQ_LLVM_REPORT"
grep -q '^fastmem_owner_runtime_current_owner_source=llvm_producer_intrinsic$' "$OWNER_EQ_LLVM_REPORT"
grep -q '^memop_table_index_lowered_count=1$' "$OWNER_EQ_LLVM_REPORT"
grep -q '^memop_field_load_lowered_count=1$' "$OWNER_EQ_LLVM_REPORT"
grep -q '^memop_field_store_lowered_count=1$' "$OWNER_EQ_LLVM_REPORT"
grep -q '^memop_current_alloc_owner_id_lowered_count=1$' "$OWNER_EQ_LLVM_REPORT"
grep -q '^memop_owner_eq_lowered_count=1$' "$OWNER_EQ_LLVM_REPORT"
grep -q '^memop_atomic_remote_head_lowered_count=0$' "$OWNER_EQ_LLVM_REPORT"
grep -q '^tls_backing_transfer_enabled=0$' "$OWNER_EQ_LLVM_REPORT"
grep -q '^allocator_owner_slot_reuse_enabled=0$' "$OWNER_EQ_LLVM_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$OWNER_EQ_LLVM_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' "$OWNER_EQ_LLVM_REPORT"
grep -q '^product_activation=0$' "$OWNER_EQ_LLVM_REPORT"
grep -q '^hook_install=0$' "$OWNER_EQ_LLVM_REPORT"
grep -q '^global_allocator_claim=0$' "$OWNER_EQ_LLVM_REPORT"
grep -q '^winner_claim=0$' "$OWNER_EQ_LLVM_REPORT"
grep -q '^summary=ok$' "$OWNER_EQ_LLVM_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$OWNER_EQ_LLVM_REPORT" \
  --format kv \
  --out "$OWNER_EQ_LLVM_CHECK"
grep -q '^summary=ok$' "$OWNER_EQ_LLVM_CHECK"
grep -q '^failure_count=0$' "$OWNER_EQ_LLVM_CHECK"

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$LOCAL_FREE_HEAD_AST" "$LOCAL_FREE_HEAD_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$LOCAL_FREE_HEAD_MIR" "$LOCAL_FREE_HEAD_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --ast-json "$LOCAL_FREE_HEAD_AST" \
  --out "$LOCAL_FREE_HEAD_INV"

grep -q '^input_kind=ast_json$' "$LOCAL_FREE_HEAD_INV"
grep -q '^fastmem_region_count=1$' "$LOCAL_FREE_HEAD_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$LOCAL_FREE_HEAD_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$LOCAL_FREE_HEAD_INV"
grep -q '^fastmem_memop_field_load_count=1$' "$LOCAL_FREE_HEAD_INV"
grep -q '^fastmem_memop_field_store_count=1$' "$LOCAL_FREE_HEAD_INV"
grep -q '^fastmem_forbidden_call_count=0$' "$LOCAL_FREE_HEAD_INV"
grep -q '^summary=ok$' "$LOCAL_FREE_HEAD_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --mir-json "$LOCAL_FREE_HEAD_MIR" \
  --out "$LOCAL_FREE_HEAD_MIR_INV"

grep -q '^input_kind=mir_json_metadata$' "$LOCAL_FREE_HEAD_MIR_INV"
grep -q '^fastmem_region_count=1$' "$LOCAL_FREE_HEAD_MIR_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$LOCAL_FREE_HEAD_MIR_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$LOCAL_FREE_HEAD_MIR_INV"
grep -q '^fastmem_memop_field_load_count=1$' "$LOCAL_FREE_HEAD_MIR_INV"
grep -q '^fastmem_memop_field_store_count=1$' "$LOCAL_FREE_HEAD_MIR_INV"
grep -q '^fastmem_verified_mem_access_plan_count=3$' "$LOCAL_FREE_HEAD_MIR_INV"
grep -q '^fastmem_verified_field_access_count=2$' "$LOCAL_FREE_HEAD_MIR_INV"
grep -q '^fastmem_verified_table_access_count=1$' "$LOCAL_FREE_HEAD_MIR_INV"
grep -q '^fastmem_table_index_unchecked_count=0$' "$LOCAL_FREE_HEAD_MIR_INV"
grep -q '^fastmem_table_access_proof_incomplete_count=0$' "$LOCAL_FREE_HEAD_MIR_INV"
grep -q '^fastmem_table_overflow_proof_missing_count=0$' "$LOCAL_FREE_HEAD_MIR_INV"
grep -q '^fastmem_field_id_missing_count=0$' "$LOCAL_FREE_HEAD_MIR_INV"
grep -q '^fastmem_table_id_missing_count=0$' "$LOCAL_FREE_HEAD_MIR_INV"
grep -q '^fastmem_unknown_alignment_count=0$' "$LOCAL_FREE_HEAD_MIR_INV"
grep -q '^summary=ok$' "$LOCAL_FREE_HEAD_MIR_INV"

if bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --mir-json "$LOCAL_FREE_HEAD_MIR" \
  --out "$LOCAL_FREE_HEAD_LLVM_REPORT" \
  2>"$LOCAL_FREE_HEAD_LLVM_STDERR"; then
  echo "[TEST/FAIL] local_free_head preflight unexpectedly lowered as ordinary FieldLoad" >&2
  cat "$LOCAL_FREE_HEAD_LLVM_REPORT" >&2 || true
  exit 1
fi
grep -q '\[llvm/fastmem:unsupported-field-load-class\] local_free_head' \
  "$LOCAL_FREE_HEAD_LLVM_STDERR"

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$LOCAL_FREE_MEMOP_AST" "$LOCAL_FREE_MEMOP_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$LOCAL_FREE_MEMOP_MIR" "$LOCAL_FREE_MEMOP_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --ast-json "$LOCAL_FREE_MEMOP_AST" \
  --out "$LOCAL_FREE_MEMOP_INV"

grep -q '^input_kind=ast_json$' "$LOCAL_FREE_MEMOP_INV"
grep -q '^fastmem_region_count=1$' "$LOCAL_FREE_MEMOP_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$LOCAL_FREE_MEMOP_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$LOCAL_FREE_MEMOP_INV"
grep -q '^fastmem_memop_field_store_count=1$' "$LOCAL_FREE_MEMOP_INV"
grep -q '^fastmem_memop_local_free_push_count=1$' "$LOCAL_FREE_MEMOP_INV"
grep -q '^fastmem_memop_local_free_pop_count=1$' "$LOCAL_FREE_MEMOP_INV"
grep -q '^fastmem_forbidden_call_count=0$' "$LOCAL_FREE_MEMOP_INV"
grep -q '^summary=ok$' "$LOCAL_FREE_MEMOP_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --mir-json "$LOCAL_FREE_MEMOP_MIR" \
  --out "$LOCAL_FREE_MEMOP_MIR_INV"

grep -q '^input_kind=mir_json_metadata$' "$LOCAL_FREE_MEMOP_MIR_INV"
grep -q '^fastmem_region_count=1$' "$LOCAL_FREE_MEMOP_MIR_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$LOCAL_FREE_MEMOP_MIR_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$LOCAL_FREE_MEMOP_MIR_INV"
grep -q '^fastmem_memop_field_store_count=1$' "$LOCAL_FREE_MEMOP_MIR_INV"
grep -q '^fastmem_memop_local_free_push_count=1$' "$LOCAL_FREE_MEMOP_MIR_INV"
grep -q '^fastmem_memop_local_free_pop_count=1$' "$LOCAL_FREE_MEMOP_MIR_INV"
grep -q '^fastmem_verified_mem_access_plan_count=2$' "$LOCAL_FREE_MEMOP_MIR_INV"
grep -q '^fastmem_verified_field_access_count=1$' "$LOCAL_FREE_MEMOP_MIR_INV"
grep -q '^fastmem_verified_table_access_count=1$' "$LOCAL_FREE_MEMOP_MIR_INV"
grep -q '^fastmem_local_free_list_plan=1$' "$LOCAL_FREE_MEMOP_MIR_INV"
grep -q '^fastmem_local_free_push_plan_count=1$' "$LOCAL_FREE_MEMOP_MIR_INV"
grep -q '^fastmem_local_free_pop_plan_count=1$' "$LOCAL_FREE_MEMOP_MIR_INV"
grep -q '^fastmem_local_free_nonlowerable_count=2$' "$LOCAL_FREE_MEMOP_MIR_INV"
grep -q '^fastmem_local_free_same_owner_required=1$' "$LOCAL_FREE_MEMOP_MIR_INV"
grep -q '^fastmem_local_free_same_owner_missing_count=2$' "$LOCAL_FREE_MEMOP_MIR_INV"
grep -q '^fastmem_local_free_non_empty_required=1$' "$LOCAL_FREE_MEMOP_MIR_INV"
grep -q '^fastmem_local_free_non_empty_missing_count=1$' "$LOCAL_FREE_MEMOP_MIR_INV"
grep -q '^fastmem_local_free_block_next_proof_missing_count=1$' "$LOCAL_FREE_MEMOP_MIR_INV"
grep -q '^summary=ok$' "$LOCAL_FREE_MEMOP_MIR_INV"

if bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --mir-json "$LOCAL_FREE_MEMOP_MIR" \
  --out "$LOCAL_FREE_MEMOP_LLVM_REPORT" \
  2>"$LOCAL_FREE_MEMOP_LLVM_STDERR"; then
  echo "[TEST/FAIL] local free-list MemOp vocabulary unexpectedly lowered" >&2
  cat "$LOCAL_FREE_MEMOP_LLVM_REPORT" >&2 || true
  exit 1
fi
grep -q '\[llvm/fastmem:missing-verified-local-free-push-plan\]' \
  "$LOCAL_FREE_MEMOP_LLVM_STDERR"

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$LOCAL_FREE_PUSH_PRECONDITION_AST" "$LOCAL_FREE_PUSH_PRECONDITION_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$LOCAL_FREE_PUSH_PRECONDITION_MIR" "$LOCAL_FREE_PUSH_PRECONDITION_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --ast-json "$LOCAL_FREE_PUSH_PRECONDITION_AST" \
  --out "$LOCAL_FREE_PUSH_PRECONDITION_INV"

grep -q '^input_kind=ast_json$' "$LOCAL_FREE_PUSH_PRECONDITION_INV"
grep -q '^fastmem_region_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$LOCAL_FREE_PUSH_PRECONDITION_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_INV"
grep -q '^fastmem_memop_field_load_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_INV"
grep -q '^fastmem_memop_field_store_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_INV"
grep -q '^fastmem_memop_current_alloc_owner_id_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_INV"
grep -q '^fastmem_memop_owner_eq_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_INV"
grep -q '^fastmem_memop_local_free_push_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_INV"
grep -q '^fastmem_forbidden_call_count=0$' "$LOCAL_FREE_PUSH_PRECONDITION_INV"
grep -q '^summary=ok$' "$LOCAL_FREE_PUSH_PRECONDITION_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --mir-json "$LOCAL_FREE_PUSH_PRECONDITION_MIR" \
  --out "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"

grep -q '^input_kind=mir_json_metadata$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_region_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_field_load_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_field_store_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_current_alloc_owner_id_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_owner_eq_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_local_free_push_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_list_plan=1$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_push_plan_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_pop_plan_count=0$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_nonlowerable_count=0$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_push_lowerable_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_head_access_resolved_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_block_next_access_resolved_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_access_plan_incomplete_count=0$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_same_owner_fact_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_block_next_fact_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_same_owner_required=1$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_same_owner_missing_count=0$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_remote_owner_rejected_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_block_next_proof_missing_count=0$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"
grep -q '^summary=ok$' "$LOCAL_FREE_PUSH_PRECONDITION_MIR_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile local-free \
  --mir-json "$LOCAL_FREE_PUSH_PRECONDITION_MIR" \
  --out "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_REPORT" \
  2>"$LOCAL_FREE_PUSH_PRECONDITION_LLVM_STDERR"

grep -q '^replacement_front_producer=mir_to_llvm_lowering$' "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^replacement_front_selected_memop_family=local_free$' "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^replacement_front_selected_memop_kinds=LocalFreePush$' "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^replacement_front_deferred_memop_kinds=LocalFreePop,AtomicRemoteHead$' "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_local_free_producer_pilot=1$' "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_local_free_push_plan_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_local_free_pop_plan_count=0$' "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^memop_local_free_push_lowered_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^memop_local_free_pop_lowered_count=0$' "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^memop_local_free_push_layout_ref_consumed_count=1$' "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_local_free_access_plan_incomplete_count=0$' "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_local_free_head_plain_store_lowered_count=0$' "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_local_free_push_lowering_uses_verified_plan=1$' "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_local_free_pop_lowering_enabled=0$' "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^product_activation=0$' "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^global_allocator_claim=0$' "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^winner_claim=0$' "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^summary=ok$' "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_REPORT" \
  --format kv \
  --out "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_CHECK"
grep -q '^summary=ok$' "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_CHECK"
grep -q '^failure_count=0$' "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_CHECK"

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$LOCAL_FREE_POP_PRECONDITION_AST" "$LOCAL_FREE_POP_PRECONDITION_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$LOCAL_FREE_POP_PRECONDITION_MIR" "$LOCAL_FREE_POP_PRECONDITION_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --ast-json "$LOCAL_FREE_POP_PRECONDITION_AST" \
  --out "$LOCAL_FREE_POP_PRECONDITION_INV"

grep -q '^input_kind=ast_json$' "$LOCAL_FREE_POP_PRECONDITION_INV"
grep -q '^fastmem_region_count=1$' "$LOCAL_FREE_POP_PRECONDITION_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$LOCAL_FREE_POP_PRECONDITION_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$LOCAL_FREE_POP_PRECONDITION_INV"
grep -q '^fastmem_memop_field_load_count=1$' "$LOCAL_FREE_POP_PRECONDITION_INV"
grep -q '^fastmem_memop_field_store_count=2$' "$LOCAL_FREE_POP_PRECONDITION_INV"
grep -q '^fastmem_memop_current_alloc_owner_id_count=1$' "$LOCAL_FREE_POP_PRECONDITION_INV"
grep -q '^fastmem_memop_owner_eq_count=1$' "$LOCAL_FREE_POP_PRECONDITION_INV"
grep -q '^fastmem_memop_local_free_push_count=0$' "$LOCAL_FREE_POP_PRECONDITION_INV"
grep -q '^fastmem_memop_local_free_pop_count=1$' "$LOCAL_FREE_POP_PRECONDITION_INV"
grep -q '^fastmem_forbidden_call_count=0$' "$LOCAL_FREE_POP_PRECONDITION_INV"
grep -q '^summary=ok$' "$LOCAL_FREE_POP_PRECONDITION_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --mir-json "$LOCAL_FREE_POP_PRECONDITION_MIR" \
  --out "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"

grep -q '^input_kind=mir_json_metadata$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_region_count=1$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_field_load_count=1$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_field_store_count=2$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_current_alloc_owner_id_count=1$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_owner_eq_count=1$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_local_free_push_count=0$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_local_free_pop_count=1$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_list_plan=1$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_push_plan_count=0$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_pop_plan_count=1$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_nonlowerable_count=0$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_pop_lowerable_count=1$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_head_access_resolved_count=1$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_block_next_access_resolved_count=1$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_access_plan_incomplete_count=0$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_same_owner_fact_count=1$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_non_empty_fact_count=1$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_same_owner_required=1$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_same_owner_missing_count=0$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_non_empty_required=1$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_non_empty_missing_count=0$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_remote_owner_rejected_count=1$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_local_free_block_next_proof_missing_count=0$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"
grep -q '^summary=ok$' "$LOCAL_FREE_POP_PRECONDITION_MIR_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile local-free \
  --mir-json "$LOCAL_FREE_POP_PRECONDITION_MIR" \
  --out "$LOCAL_FREE_POP_PRECONDITION_LLVM_REPORT" \
  2>"$LOCAL_FREE_POP_PRECONDITION_LLVM_STDERR"

grep -q '^replacement_front_producer=mir_to_llvm_lowering$' "$LOCAL_FREE_POP_PRECONDITION_LLVM_REPORT"
grep -q '^replacement_front_selected_memop_family=local_free$' "$LOCAL_FREE_POP_PRECONDITION_LLVM_REPORT"
grep -q '^replacement_front_selected_memop_kinds=LocalFreePop$' "$LOCAL_FREE_POP_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_local_free_producer_pilot=1$' "$LOCAL_FREE_POP_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_local_free_push_plan_count=0$' "$LOCAL_FREE_POP_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_local_free_pop_plan_count=1$' "$LOCAL_FREE_POP_PRECONDITION_LLVM_REPORT"
grep -q '^memop_local_free_push_lowered_count=0$' "$LOCAL_FREE_POP_PRECONDITION_LLVM_REPORT"
grep -q '^memop_local_free_pop_lowered_count=1$' "$LOCAL_FREE_POP_PRECONDITION_LLVM_REPORT"
grep -q '^memop_local_free_pop_layout_ref_consumed_count=1$' "$LOCAL_FREE_POP_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_local_free_access_plan_incomplete_count=0$' "$LOCAL_FREE_POP_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_local_free_head_plain_store_lowered_count=0$' "$LOCAL_FREE_POP_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_local_free_push_lowering_uses_verified_plan=0$' "$LOCAL_FREE_POP_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_local_free_pop_lowering_uses_verified_plan=1$' "$LOCAL_FREE_POP_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_local_free_pop_lowering_enabled=1$' "$LOCAL_FREE_POP_PRECONDITION_LLVM_REPORT"
grep -q '^product_activation=0$' "$LOCAL_FREE_POP_PRECONDITION_LLVM_REPORT"
grep -q '^global_allocator_claim=0$' "$LOCAL_FREE_POP_PRECONDITION_LLVM_REPORT"
grep -q '^winner_claim=0$' "$LOCAL_FREE_POP_PRECONDITION_LLVM_REPORT"
grep -q '^summary=ok$' "$LOCAL_FREE_POP_PRECONDITION_LLVM_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$LOCAL_FREE_POP_PRECONDITION_LLVM_REPORT" \
  --format kv \
  --out "$LOCAL_FREE_POP_PRECONDITION_LLVM_CHECK"
grep -q '^summary=ok$' "$LOCAL_FREE_POP_PRECONDITION_LLVM_CHECK"
grep -q '^failure_count=0$' "$LOCAL_FREE_POP_PRECONDITION_LLVM_CHECK"

cat >"$BAD_SRC" <<'HK'
static box Main {
  main(ptr) {
    fastmem PageMapV0 {
      local addr = mem.addr(ptr)
      local bad = arbitrary(ptr)
    }
    return 0
  }
}
HK

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$BAD_AST" "$BAD_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --ast-json "$BAD_AST" \
  --out "$BAD_INV"
grep -q '^fastmem_forbidden_call_count=1$' "$BAD_INV"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --ast-json "$BAD_AST" \
  --format kv \
  --out "$BAD_CHECK"; then
  echo "[TEST/FAIL] fastmem-check accepted forbidden call inside fastmem" >&2
  cat "$BAD_CHECK" >&2 || true
  exit 1
fi
grep -q '^summary=failed$' "$BAD_CHECK"
grep -q '^failure_0_reason=fastmem_forbidden_call_count$' "$BAD_CHECK"

echo "[TEST/OK] fastmem_source_syntax"
