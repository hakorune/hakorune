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

run_fastmem_source_manifest_seed() {
  python3 "$ROOT/tools/hako_check/fastmem_source_manifest_runner.py" \
    --manifest "$ROOT/tools/hako_check/manifests/fastmem_source_syntax_smoke.toml"
}

run_fastmem_source_manifest_seed

emit_fastmem_producer_report() {
  local profile="$1"
  local mir_json="$2"
  local out="$3"

  bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
    --profile "$profile" \
    --mir-json "$mir_json" \
    --out "$out"
}

assert_fastmem_report_check_ok() {
  local report="$1"
  local check="$2"

  bash "$ROOT/tools/hako_check.sh" fastmem-check \
    --inventory "$report" \
    --format kv \
    --out "$check"

  grep -q '^summary=ok$' "$check"
  grep -q '^failure_count=0$' "$check"
}

GOOD_SRC="$TMPDIR/good.hako"
GOOD_AST="$TMPDIR/good.ast.json"
GOOD_INV="$TMPDIR/good.inventory.kv"
GOOD_CHECK="$TMPDIR/good.check.kv"
BAD_SRC="$TMPDIR/bad.hako"
BAD_AST="$TMPDIR/bad.ast.json"
BAD_INV="$TMPDIR/bad.inventory.kv"
BAD_CHECK="$TMPDIR/bad.check.kv"
BAD_BRANCH_SRC="$TMPDIR/bad_branch.hako"
BAD_BRANCH_MIR="$TMPDIR/bad_branch.mir.json"
BAD_BRANCH_LOG="$TMPDIR/bad_branch.log"
BRANCH_RETURN_SCOPE_SRC="$ROOT/lang/src/hako_alloc/memory/page_meta_fastmem_branch_return_scope_box.hako"
BRANCH_RETURN_SCOPE_AST="$TMPDIR/page_meta_fastmem_branch_return_scope.ast.json"
BRANCH_RETURN_SCOPE_MIR="$TMPDIR/page_meta_fastmem_branch_return_scope.mir.json"
BRANCH_RETURN_SCOPE_INV="$TMPDIR/page_meta_fastmem_branch_return_scope.inventory.kv"
ATOMIC_REMOTE_HEAD_PUSH_SRC="$ROOT/lang/src/hako_alloc/memory/page_meta_atomic_remote_head_push_vocabulary_box.hako"
ATOMIC_REMOTE_HEAD_PUSH_AST="$TMPDIR/page_meta_atomic_remote_head_push.ast.json"
ATOMIC_REMOTE_HEAD_PUSH_MIR="$TMPDIR/page_meta_atomic_remote_head_push.mir.json"
ATOMIC_REMOTE_HEAD_PUSH_INV="$TMPDIR/page_meta_atomic_remote_head_push.inventory.kv"
ATOMIC_REMOTE_HEAD_PUSH_MIR_INV="$TMPDIR/page_meta_atomic_remote_head_push.mir.inventory.kv"
ATOMIC_REMOTE_HEAD_PUSH_LLVM_REPORT="$TMPDIR/page_meta_atomic_remote_head_push.llvm.report.kv"
ATOMIC_REMOTE_HEAD_PUSH_LLVM_CHECK="$TMPDIR/page_meta_atomic_remote_head_push.llvm.check.kv"
ATOMIC_REMOTE_HEAD_PUSH_RETRY_REPORT="$TMPDIR/page_meta_atomic_remote_head_push.retry.report.kv"
ATOMIC_REMOTE_HEAD_PUSH_RETRY_CHECK="$TMPDIR/page_meta_atomic_remote_head_push.retry.check.kv"
ATOMIC_REMOTE_HEAD_PUSH_RETRY_PRODUCER_REPORT="$TMPDIR/page_meta_atomic_remote_head_push.retry_producer.report.kv"
ATOMIC_REMOTE_HEAD_PUSH_RETRY_PRODUCER_CHECK="$TMPDIR/page_meta_atomic_remote_head_push.retry_producer.check.kv"
ATOMIC_REMOTE_HEAD_PUSH_LLVM_STDERR="$TMPDIR/page_meta_atomic_remote_head_push.llvm.stderr"
ATOMIC_REMOTE_HEAD_PUSH_DIRECT_OBJ="$TMPDIR/page_meta_atomic_remote_head_push.direct.o"
ATOMIC_REMOTE_HEAD_DRAIN_SRC="$ROOT/lang/src/hako_alloc/memory/page_meta_atomic_remote_head_drain_vocabulary_box.hako"
ATOMIC_REMOTE_HEAD_DRAIN_AST="$TMPDIR/page_meta_atomic_remote_head_drain.ast.json"
ATOMIC_REMOTE_HEAD_DRAIN_MIR="$TMPDIR/page_meta_atomic_remote_head_drain.mir.json"
ATOMIC_REMOTE_HEAD_DRAIN_INV="$TMPDIR/page_meta_atomic_remote_head_drain.inventory.kv"
ATOMIC_REMOTE_HEAD_DRAIN_MIR_INV="$TMPDIR/page_meta_atomic_remote_head_drain.mir.inventory.kv"
ATOMIC_REMOTE_HEAD_DRAIN_LLVM_REPORT="$TMPDIR/page_meta_atomic_remote_head_drain.llvm.report.kv"
ATOMIC_REMOTE_HEAD_DRAIN_LLVM_STDERR="$TMPDIR/page_meta_atomic_remote_head_drain.llvm.stderr"
ATOMIC_REMOTE_HEAD_DRAIN_DIRECT_OBJ="$TMPDIR/page_meta_atomic_remote_head_drain.direct.o"
ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_REPORT="$TMPDIR/page_meta_atomic_remote_head_drain.preflight.report.kv"
ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_CHECK="$TMPDIR/page_meta_atomic_remote_head_drain.preflight.check.kv"
ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_REPORT="$TMPDIR/page_meta_atomic_remote_head_drain.exchange.report.kv"
ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_CHECK="$TMPDIR/page_meta_atomic_remote_head_drain.exchange.check.kv"
ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT="$TMPDIR/page_meta_atomic_remote_head_drain.to_local.report.kv"
ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_CHECK="$TMPDIR/page_meta_atomic_remote_head_drain.to_local.check.kv"
ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_REPORT="$TMPDIR/page_meta_atomic_remote_head_drain.local_list_mutation_preflight.report.kv"
ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_CHECK="$TMPDIR/page_meta_atomic_remote_head_drain.local_list_mutation_preflight.check.kv"
ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_REPORT="$TMPDIR/page_meta_atomic_remote_head_drain.local_list_mutation_proof.report.kv"
ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_CHECK="$TMPDIR/page_meta_atomic_remote_head_drain.local_list_mutation_proof.check.kv"
DRAIN_REMOTE_LIST_TO_LOCAL_SRC="$ROOT/lang/src/hako_alloc/memory/page_meta_drain_remote_list_to_local_vocabulary_box.hako"
DRAIN_REMOTE_LIST_TO_LOCAL_AST="$TMPDIR/page_meta_drain_remote_list_to_local.ast.json"
DRAIN_REMOTE_LIST_TO_LOCAL_MIR="$TMPDIR/page_meta_drain_remote_list_to_local.mir.json"
DRAIN_REMOTE_LIST_TO_LOCAL_INV="$TMPDIR/page_meta_drain_remote_list_to_local.inventory.kv"
DRAIN_REMOTE_LIST_TO_LOCAL_MIR_INV="$TMPDIR/page_meta_drain_remote_list_to_local.mir.inventory.kv"
DRAIN_REMOTE_LIST_TO_LOCAL_REPORT="$TMPDIR/page_meta_drain_remote_list_to_local.report.kv"
DRAIN_REMOTE_LIST_TO_LOCAL_CHECK="$TMPDIR/page_meta_drain_remote_list_to_local.check.kv"
DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_REPORT="$TMPDIR/page_meta_drain_remote_list_to_local.lowering.report.kv"
DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_CHECK="$TMPDIR/page_meta_drain_remote_list_to_local.lowering.check.kv"
DRAIN_REMOTE_LIST_TO_LOCAL_LLVM_STDERR="$TMPDIR/page_meta_drain_remote_list_to_local.llvm.stderr"
REMOTE_OWNER_BRANCH_ROUTING_LOWERING_SRC="$ROOT/lang/src/hako_alloc/memory/page_meta_remote_owner_branch_routing_lowering_box.hako"
REMOTE_OWNER_BRANCH_ROUTING_LOWERING_AST="$TMPDIR/page_meta_remote_owner_branch_routing_lowering.ast.json"
REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR="$TMPDIR/page_meta_remote_owner_branch_routing_lowering.mir.json"
REMOTE_OWNER_BRANCH_ROUTING_LOWERING_INV="$TMPDIR/page_meta_remote_owner_branch_routing_lowering.inventory.kv"
REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR_INV="$TMPDIR/page_meta_remote_owner_branch_routing_lowering.mir.inventory.kv"
REMOTE_OWNER_BRANCH_ROUTING_LOWERING_REPORT="$TMPDIR/page_meta_remote_owner_branch_routing_lowering.report.kv"
REMOTE_OWNER_BRANCH_ROUTING_LOWERING_CHECK="$TMPDIR/page_meta_remote_owner_branch_routing_lowering.check.kv"
REMOTE_OWNER_BRANCH_ROUTING_LOWERING_LLVM_STDERR="$TMPDIR/page_meta_remote_owner_branch_routing_lowering.llvm.stderr"
REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_REPORT="$TMPDIR/page_meta_remote_owner_branch_route_body_preflight.report.kv"
REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_CHECK="$TMPDIR/page_meta_remote_owner_branch_route_body_preflight.check.kv"
FASTMEM_BRANCH_CFG_PREFLIGHT_REPORT="$TMPDIR/page_meta_fastmem_branch_cfg_preflight.report.kv"
FASTMEM_BRANCH_CFG_PREFLIGHT_CHECK="$TMPDIR/page_meta_fastmem_branch_cfg_preflight.check.kv"
FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_REPORT="$TMPDIR/page_meta_fastmem_branch_cfg_lowering_preflight.report.kv"
FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_CHECK="$TMPDIR/page_meta_fastmem_branch_cfg_lowering_preflight.check.kv"
FASTMEM_BRANCH_CFG_LOWERING_SRC="$ROOT/lang/src/hako_alloc/memory/page_meta_fastmem_branch_cfg_lowering_box.hako"
FASTMEM_BRANCH_CFG_LOWERING_AST="$TMPDIR/page_meta_fastmem_branch_cfg_lowering.ast.json"
FASTMEM_BRANCH_CFG_LOWERING_MIR="$TMPDIR/page_meta_fastmem_branch_cfg_lowering.mir.json"
FASTMEM_BRANCH_CFG_LOWERING_INV="$TMPDIR/page_meta_fastmem_branch_cfg_lowering.inventory.kv"
FASTMEM_BRANCH_CFG_LOWERING_MIR_INV="$TMPDIR/page_meta_fastmem_branch_cfg_lowering.mir.inventory.kv"
FASTMEM_BRANCH_CFG_LOWERING_REPORT="$TMPDIR/page_meta_fastmem_branch_cfg_lowering.report.kv"
FASTMEM_BRANCH_CFG_LOWERING_CHECK="$TMPDIR/page_meta_fastmem_branch_cfg_lowering.check.kv"
FASTMEM_BRANCH_CFG_LOWERING_LLVM_STDERR="$TMPDIR/page_meta_fastmem_branch_cfg_lowering.llvm.stderr"
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
FREE_HEAD_POP_SRC="$ROOT/lang/src/hako_alloc/memory/page_meta_free_head_pop_vocabulary_box.hako"
FREE_HEAD_POP_AST="$TMPDIR/page_meta_free_head_pop.ast.json"
FREE_HEAD_POP_MIR="$TMPDIR/page_meta_free_head_pop.mir.json"
FREE_HEAD_POP_INV="$TMPDIR/page_meta_free_head_pop.inventory.kv"
FREE_HEAD_POP_MIR_INV="$TMPDIR/page_meta_free_head_pop.mir.inventory.kv"
FREE_HEAD_POP_LLVM_REPORT="$TMPDIR/page_meta_free_head_pop.llvm.report.kv"
FREE_HEAD_POP_LLVM_STDERR="$TMPDIR/page_meta_free_head_pop.llvm.stderr"
FREE_HEAD_POP_PRECONDITION_SRC="$ROOT/lang/src/hako_alloc/memory/page_meta_free_head_pop_precondition_box.hako"
FREE_HEAD_POP_PRECONDITION_AST="$TMPDIR/page_meta_free_head_pop_precondition.ast.json"
FREE_HEAD_POP_PRECONDITION_MIR="$TMPDIR/page_meta_free_head_pop_precondition.mir.json"
FREE_HEAD_POP_PRECONDITION_INV="$TMPDIR/page_meta_free_head_pop_precondition.inventory.kv"
FREE_HEAD_POP_PRECONDITION_MIR_INV="$TMPDIR/page_meta_free_head_pop_precondition.mir.inventory.kv"
FREE_HEAD_POP_PRECONDITION_LLVM_REPORT="$TMPDIR/page_meta_free_head_pop_precondition.llvm.report.kv"
FREE_HEAD_POP_PRECONDITION_LLVM_CHECK="$TMPDIR/page_meta_free_head_pop_precondition.llvm.check.kv"
FREE_HEAD_POP_PRECONDITION_LLVM_STDERR="$TMPDIR/page_meta_free_head_pop_precondition.llvm.stderr"
FREE_HEAD_PUSH_SRC="$ROOT/lang/src/hako_alloc/memory/page_meta_free_head_push_vocabulary_box.hako"
FREE_HEAD_PUSH_AST="$TMPDIR/page_meta_free_head_push.ast.json"
FREE_HEAD_PUSH_MIR="$TMPDIR/page_meta_free_head_push.mir.json"
FREE_HEAD_PUSH_INV="$TMPDIR/page_meta_free_head_push.inventory.kv"
FREE_HEAD_PUSH_MIR_INV="$TMPDIR/page_meta_free_head_push.mir.inventory.kv"
FREE_HEAD_PUSH_LLVM_REPORT="$TMPDIR/page_meta_free_head_push.llvm.report.kv"
FREE_HEAD_PUSH_LLVM_STDERR="$TMPDIR/page_meta_free_head_push.llvm.stderr"
FREE_HEAD_PUSH_PRECONDITION_SRC="$ROOT/lang/src/hako_alloc/memory/page_meta_free_head_push_precondition_box.hako"
FREE_HEAD_PUSH_PRECONDITION_AST="$TMPDIR/page_meta_free_head_push_precondition.ast.json"
FREE_HEAD_PUSH_PRECONDITION_MIR="$TMPDIR/page_meta_free_head_push_precondition.mir.json"
FREE_HEAD_PUSH_PRECONDITION_INV="$TMPDIR/page_meta_free_head_push_precondition.inventory.kv"
FREE_HEAD_PUSH_PRECONDITION_MIR_INV="$TMPDIR/page_meta_free_head_push_precondition.mir.inventory.kv"
FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT="$TMPDIR/page_meta_free_head_push_precondition.llvm.report.kv"
FREE_HEAD_PUSH_PRECONDITION_LLVM_STDERR="$TMPDIR/page_meta_free_head_push_precondition.llvm.stderr"
REFILL_THEN_FREE_HEAD_ALLOC_SRC="$ROOT/lang/src/hako_alloc/memory/page_meta_refill_then_free_head_alloc_body_box.hako"
REFILL_THEN_FREE_HEAD_ALLOC_AST="$TMPDIR/page_meta_refill_then_free_head_alloc.ast.json"
REFILL_THEN_FREE_HEAD_ALLOC_MIR="$TMPDIR/page_meta_refill_then_free_head_alloc.mir.json"
PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_SRC="$ROOT/lang/src/hako_alloc/memory/page_meta_page_local_alloc_route_cfg_preflight_box.hako"
PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_AST="$TMPDIR/page_meta_page_local_alloc_route_cfg_preflight.ast.json"
PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_MIR="$TMPDIR/page_meta_page_local_alloc_route_cfg_preflight.mir.json"
PAGE_LOCAL_ALLOC_ROUTE_CFG_PRODUCER_REPORT="$TMPDIR/page_meta_page_local_alloc_route_cfg_producer.report.kv"
PAGE_LOCAL_ALLOC_ROUTE_CFG_PRODUCER_CHECK="$TMPDIR/page_meta_page_local_alloc_route_cfg_producer.check.kv"
PAGE_LOCAL_ROUTE_BODY_JOIN_REPORT="$TMPDIR/page_meta_page_local_route_body_join.report.kv"
PAGE_LOCAL_ROUTE_BODY_JOIN_CHECK="$TMPDIR/page_meta_page_local_route_body_join.check.kv"
PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER_REPORT="$TMPDIR/page_meta_page_local_route_body_join_producer.report.kv"
PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER_CHECK="$TMPDIR/page_meta_page_local_route_body_join_producer.check.kv"
TERMINAL_LADDER_REFRESH_REPORT="$TMPDIR/terminal_ladder_refresh.report.kv"
TERMINAL_LADDER_REFRESH_CHECK="$TMPDIR/terminal_ladder_refresh.check.kv"
TLS_BACKING_TRANSFER_REFRESH_REPORT="$TMPDIR/tls_backing_transfer_refresh.report.kv"
TLS_BACKING_TRANSFER_REFRESH_CHECK="$TMPDIR/tls_backing_transfer_refresh.check.kv"
TLS_BACKING_TRANSFER_PRODUCER_REFRESH_REPORT="$TMPDIR/tls_backing_transfer_producer_refresh.report.kv"
TLS_BACKING_TRANSFER_PRODUCER_REFRESH_CHECK="$TMPDIR/tls_backing_transfer_producer_refresh.check.kv"
OWNER_SLOT_REUSE_REFRESH_REPORT="$TMPDIR/owner_slot_reuse_refresh.report.kv"
OWNER_SLOT_REUSE_REFRESH_CHECK="$TMPDIR/owner_slot_reuse_refresh.check.kv"
OWNER_SLOT_REUSE_PRODUCER_REFRESH_REPORT="$TMPDIR/owner_slot_reuse_producer_refresh.report.kv"
OWNER_SLOT_REUSE_PRODUCER_REFRESH_CHECK="$TMPDIR/owner_slot_reuse_producer_refresh.check.kv"
ABANDONED_RECLAIM_REFRESH_REPORT="$TMPDIR/abandoned_reclaim_refresh.report.kv"
ABANDONED_RECLAIM_REFRESH_CHECK="$TMPDIR/abandoned_reclaim_refresh.check.kv"
ABANDONED_RECLAIM_PRODUCER_REFRESH_REPORT="$TMPDIR/abandoned_reclaim_producer_refresh.report.kv"
ABANDONED_RECLAIM_PRODUCER_REFRESH_CHECK="$TMPDIR/abandoned_reclaim_producer_refresh.check.kv"
PRODUCT_ACTIVATION_REFRESH_REPORT="$TMPDIR/product_activation_refresh.report.kv"
PRODUCT_ACTIVATION_REFRESH_CHECK="$TMPDIR/product_activation_refresh.check.kv"
PRODUCT_ACTIVATION_PRODUCER_REFRESH_REPORT="$TMPDIR/product_activation_producer_refresh.report.kv"
PRODUCT_ACTIVATION_PRODUCER_REFRESH_CHECK="$TMPDIR/product_activation_producer_refresh.check.kv"
HOOK_INSTALL_REFRESH_REPORT="$TMPDIR/hook_install_refresh.report.kv"
HOOK_INSTALL_REFRESH_CHECK="$TMPDIR/hook_install_refresh.check.kv"
HOOK_INSTALL_PRODUCER_REFRESH_REPORT="$TMPDIR/hook_install_producer_refresh.report.kv"
HOOK_INSTALL_PRODUCER_REFRESH_CHECK="$TMPDIR/hook_install_producer_refresh.check.kv"
GLOBAL_ALLOCATOR_CLAIM_REFRESH_REPORT="$TMPDIR/global_allocator_claim_refresh.report.kv"
GLOBAL_ALLOCATOR_CLAIM_REFRESH_CHECK="$TMPDIR/global_allocator_claim_refresh.check.kv"
GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_REPORT="$TMPDIR/global_allocator_claim_producer_refresh.report.kv"
GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_CHECK="$TMPDIR/global_allocator_claim_producer_refresh.check.kv"
WINNER_CLAIM_REFRESH_REPORT="$TMPDIR/winner_claim_refresh.report.kv"
WINNER_CLAIM_REFRESH_CHECK="$TMPDIR/winner_claim_refresh.check.kv"
WINNER_CLAIM_PRODUCER_REFRESH_REPORT="$TMPDIR/winner_claim_producer_refresh.report.kv"
WINNER_CLAIM_PRODUCER_REFRESH_CHECK="$TMPDIR/winner_claim_producer_refresh.check.kv"
WINNER_CLAIM_PREFLIGHT_REPORT="$TMPDIR/winner_claim_preflight.report.kv"
WINNER_CLAIM_PREFLIGHT_CHECK="$TMPDIR/winner_claim_preflight.check.kv"
WINNER_CLAIM_PRODUCER_REPORT="$TMPDIR/winner_claim_producer.report.kv"
WINNER_CLAIM_PRODUCER_CHECK="$TMPDIR/winner_claim_producer.check.kv"

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

cat >"$BAD_BRANCH_SRC" <<'HK'
static box Main {
  main(ptr) {
    fastmem PageMapV0 {
      if true {
        local addr = mem.addr(ptr)
      } else {
        local addr = mem.addr(ptr)
      }
    }
    return 0
  }
}
HK

if NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$BAD_BRANCH_MIR" "$BAD_BRANCH_SRC" >"$BAD_BRANCH_LOG" 2>&1; then
  echo "[TEST/FAIL] fastmem branch CFG was accepted" >&2
  cat "$BAD_BRANCH_LOG" >&2 || true
  exit 1
fi
# The MIRBuilder unit test pins the precise FastMemory rejection tag. The CLI
# path can still mask builder Err with the existing lexical-scope cleanup
# fail-fast; this smoke only requires that unsupported branch CFG does not pass.
grep -Eq '\[freeze:contract\]\[(fastmem/branch_cfg_requires_owner_eq_condition|lexical_scope/unbalanced_pop)\]' "$BAD_BRANCH_LOG"

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$BRANCH_RETURN_SCOPE_AST" "$BRANCH_RETURN_SCOPE_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$BRANCH_RETURN_SCOPE_MIR" "$BRANCH_RETURN_SCOPE_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --mir-json "$BRANCH_RETURN_SCOPE_MIR" \
  --out "$BRANCH_RETURN_SCOPE_INV"

grep -q '^input_kind=mir_json_metadata$' "$BRANCH_RETURN_SCOPE_INV"
grep -q '^fastmem_region_count=1$' "$BRANCH_RETURN_SCOPE_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$BRANCH_RETURN_SCOPE_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$BRANCH_RETURN_SCOPE_INV"
grep -q '^fastmem_memop_field_load_count=3$' "$BRANCH_RETURN_SCOPE_INV"
grep -q '^fastmem_memop_field_store_count=0$' "$BRANCH_RETURN_SCOPE_INV"
grep -q '^fastmem_memop_current_alloc_owner_id_count=1$' "$BRANCH_RETURN_SCOPE_INV"
grep -q '^fastmem_memop_owner_eq_count=1$' "$BRANCH_RETURN_SCOPE_INV"
grep -q '^fastmem_memop_unbalanced_region_count=0$' "$BRANCH_RETURN_SCOPE_INV"
grep -q '^summary=ok$' "$BRANCH_RETURN_SCOPE_INV"

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$ATOMIC_REMOTE_HEAD_PUSH_AST" "$ATOMIC_REMOTE_HEAD_PUSH_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$ATOMIC_REMOTE_HEAD_PUSH_MIR" "$ATOMIC_REMOTE_HEAD_PUSH_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --ast-json "$ATOMIC_REMOTE_HEAD_PUSH_AST" \
  --out "$ATOMIC_REMOTE_HEAD_PUSH_INV"

grep -q '^input_kind=ast_json$' "$ATOMIC_REMOTE_HEAD_PUSH_INV"
grep -q '^fastmem_region_count=1$' "$ATOMIC_REMOTE_HEAD_PUSH_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$ATOMIC_REMOTE_HEAD_PUSH_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$ATOMIC_REMOTE_HEAD_PUSH_INV"
grep -q '^fastmem_memop_field_load_count=1$' "$ATOMIC_REMOTE_HEAD_PUSH_INV"
grep -q '^fastmem_memop_atomic_remote_head_push_count=1$' "$ATOMIC_REMOTE_HEAD_PUSH_INV"
grep -q '^fastmem_forbidden_call_count=0$' "$ATOMIC_REMOTE_HEAD_PUSH_INV"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_HEAD_PUSH_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --mir-json "$ATOMIC_REMOTE_HEAD_PUSH_MIR" \
  --out "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"

grep -q '^input_kind=mir_json_metadata$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_region_count=1$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_memop_field_load_count=1$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_memop_atomic_remote_head_push_count=1$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^atomic_remote_head_push_plan_count=1$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^atomic_remote_head_push_lowerable_count=1$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^atomic_remote_head_remote_owner_required=1$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^atomic_remote_head_remote_owner_missing_count=0$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^atomic_remote_head_block_next_required=1$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^atomic_remote_head_block_next_missing_count=0$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^atomic_remote_head_access_resolved_count=1$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^atomic_remote_head_memory_order_policy=acq_rel$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_remote_owner_fact_count=1$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_remote_owner_source_assume_count=1$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_remote_free_block_next_source_assume_count=1$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_verified_mem_access_plan_count=3$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_verified_field_access_count=1$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_verified_table_access_count=1$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_table_index_unchecked_count=0$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_table_access_proof_incomplete_count=0$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_table_overflow_proof_missing_count=0$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_HEAD_PUSH_MIR_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile remote-free \
  --mir-json "$ATOMIC_REMOTE_HEAD_PUSH_MIR" \
  --out "$ATOMIC_REMOTE_HEAD_PUSH_LLVM_REPORT"

grep -q '^replacement_front_selected_memop_family=remote_free$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_LLVM_REPORT"
grep -q '^replacement_front_selected_memop_kinds=AtomicRemoteHeadPush$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_LLVM_REPORT"
grep -q '^fastmem_atomic_remote_head_cas_producer_pilot=1$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_LLVM_REPORT"
grep -q '^atomic_remote_head_cas_lowering_selected=1$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_LLVM_REPORT"
grep -q '^atomic_remote_head_cas_lowering_open=1$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_LLVM_REPORT"
grep -q '^atomic_remote_head_push_plan_count=1$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_LLVM_REPORT"
grep -q '^atomic_remote_head_push_lowerable_count=1$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_LLVM_REPORT"
grep -q '^atomic_remote_head_remote_owner_missing_count=0$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_LLVM_REPORT"
grep -q '^atomic_remote_head_block_next_missing_count=0$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_LLVM_REPORT"
grep -q '^atomic_remote_head_memory_order_policy=acq_rel$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_LLVM_REPORT"
grep -q '^memop_atomic_remote_head_lowered_count=1$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_LLVM_REPORT"
grep -q '^fastmem_remote_owner_source_assume_count=1$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_LLVM_REPORT"
grep -q '^fastmem_remote_free_block_next_source_assume_count=1$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_LLVM_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$ATOMIC_REMOTE_HEAD_PUSH_LLVM_REPORT" \
  --format kv \
  --out "$ATOMIC_REMOTE_HEAD_PUSH_LLVM_CHECK"

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_HEAD_PUSH_LLVM_CHECK"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_HEAD_PUSH_LLVM_CHECK"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile remote-free-retry-preflight \
  --mir-json "$ATOMIC_REMOTE_HEAD_PUSH_MIR" \
  --out "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_REPORT"

grep -q '^replacement_front_selected_memop_family=remote_free$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_REPORT"
grep -q '^replacement_front_selected_memop_kinds=AtomicRemoteHeadPush$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_REPORT"
grep -q '^replacement_front_next_producer_slice=atomic_remote_head_retry_policy_preflight$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_REPORT"
grep -q '^fastmem_atomic_remote_head_retry_preflight=1$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_REPORT"
grep -q '^atomic_remote_head_retry_policy_selected=1$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_REPORT"
grep -q '^atomic_remote_head_retry_policy_open=0$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_REPORT"
grep -q '^atomic_remote_head_retry_attempt_limit=3$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_REPORT"
grep -q '^atomic_remote_head_retry_lowered_count=0$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_REPORT"
grep -q '^atomic_remote_head_drain_open=0$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_REPORT"
grep -q '^remote_owner_branch_routing_open=0$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_REPORT"
grep -q '^atomic_remote_head_cas_lowering_open=1$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_REPORT"
grep -q '^memop_atomic_remote_head_lowered_count=1$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_REPORT"
grep -q '^product_activation=0$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_REPORT" \
  --format kv \
  --out "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_CHECK"

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_CHECK"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_CHECK"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile remote-free-retry \
  --mir-json "$ATOMIC_REMOTE_HEAD_PUSH_MIR" \
  --out "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_PRODUCER_REPORT"

grep -q '^replacement_front_selected_memop_family=remote_free$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_PRODUCER_REPORT"
grep -q '^replacement_front_selected_memop_kinds=AtomicRemoteHeadPush$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_PRODUCER_REPORT"
grep -q '^replacement_front_next_producer_slice=atomic_remote_head_retry_lowering_producer_pilot$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_PRODUCER_REPORT"
grep -q '^fastmem_atomic_remote_head_retry_producer_pilot=1$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_PRODUCER_REPORT"
grep -q '^atomic_remote_head_retry_policy_selected=1$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_PRODUCER_REPORT"
grep -q '^atomic_remote_head_retry_policy_open=1$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_PRODUCER_REPORT"
grep -q '^atomic_remote_head_retry_attempt_limit=3$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_PRODUCER_REPORT"
grep -q '^atomic_remote_head_retry_lowered_count=1$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_PRODUCER_REPORT"
grep -q '^atomic_remote_head_drain_open=0$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_PRODUCER_REPORT"
grep -q '^remote_owner_branch_routing_open=0$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_PRODUCER_REPORT"
grep -q '^atomic_remote_head_cas_lowering_open=1$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_PRODUCER_REPORT"
grep -q '^memop_atomic_remote_head_lowered_count=1$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_PRODUCER_REPORT"
grep -q '^product_activation=0$' \
  "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_PRODUCER_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_PRODUCER_REPORT" \
  --format kv \
  --out "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_PRODUCER_CHECK"

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_PRODUCER_CHECK"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_HEAD_PUSH_RETRY_PRODUCER_CHECK"

python3 "$ROOT/src/llvm_py/llvm_builder.py" \
  "$ATOMIC_REMOTE_HEAD_PUSH_MIR" \
  -o "$ATOMIC_REMOTE_HEAD_PUSH_DIRECT_OBJ" \
  2>"$ATOMIC_REMOTE_HEAD_PUSH_LLVM_STDERR"
test -s "$ATOMIC_REMOTE_HEAD_PUSH_DIRECT_OBJ"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile remote-free-drain-preflight \
  --mir-json "$ATOMIC_REMOTE_HEAD_PUSH_MIR" \
  --out "$ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_REPORT"

grep -q '^replacement_front_selected_memop_family=remote_free$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_kinds=AtomicRemoteHeadDrain$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_REPORT"
grep -q '^replacement_front_next_producer_slice=atomic_remote_head_drain_preflight$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_REPORT"
grep -q '^fastmem_atomic_remote_head_drain_preflight=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_REPORT"
grep -q '^atomic_remote_head_drain_selected=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_REPORT"
grep -q '^atomic_remote_head_drain_open=0$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_REPORT"
grep -q '^atomic_remote_head_drain_lowered_count=0$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_REPORT"
grep -q '^remote_owner_branch_routing_open=0$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_REPORT"
grep -q '^atomic_remote_head_retry_policy_open=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_REPORT"
grep -q '^atomic_remote_head_retry_lowered_count=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_REPORT"
grep -q '^product_activation=0$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_REPORT" \
  --format kv \
  --out "$ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_CHECK"

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_CHECK"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_CHECK"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile remote-free-drain-exchange-selection \
  --mir-json "$ATOMIC_REMOTE_HEAD_PUSH_MIR" \
  --out "$ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_REPORT"

grep -q '^replacement_front_selected_memop_family=remote_free$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_REPORT"
grep -q '^replacement_front_selected_memop_kinds=AtomicRemoteHeadDrain$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_REPORT"
grep -q '^replacement_front_next_producer_slice=atomic_remote_head_drain_exchange_lowering_producer_pilot$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_REPORT"
grep -q '^fastmem_atomic_remote_head_drain_exchange_selection=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_REPORT"
grep -q '^atomic_remote_head_drain_exchange_selected=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_REPORT"
grep -q '^atomic_remote_head_drain_exchange_order=acquire$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_REPORT"
grep -q '^atomic_remote_head_drain_result_kind=remote_free_list_token$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_REPORT"
grep -q '^atomic_remote_head_drain_open=0$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_REPORT"
grep -q '^atomic_remote_head_drain_lowered_count=0$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_REPORT"
grep -q '^atomic_remote_head_drain_to_local_route_open=0$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_REPORT"
grep -q '^remote_owner_branch_routing_open=0$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_REPORT"
grep -q '^atomic_remote_head_retry_policy_open=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_REPORT"
grep -q '^atomic_remote_head_retry_lowered_count=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_REPORT"
grep -q '^product_activation=0$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_REPORT" \
  --format kv \
  --out "$ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_CHECK"

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_CHECK"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_HEAD_DRAIN_EXCHANGE_CHECK"

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$ATOMIC_REMOTE_HEAD_DRAIN_AST" "$ATOMIC_REMOTE_HEAD_DRAIN_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$ATOMIC_REMOTE_HEAD_DRAIN_MIR" "$ATOMIC_REMOTE_HEAD_DRAIN_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --ast-json "$ATOMIC_REMOTE_HEAD_DRAIN_AST" \
  --out "$ATOMIC_REMOTE_HEAD_DRAIN_INV"

grep -q '^input_kind=ast_json$' "$ATOMIC_REMOTE_HEAD_DRAIN_INV"
grep -q '^fastmem_region_count=1$' "$ATOMIC_REMOTE_HEAD_DRAIN_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$ATOMIC_REMOTE_HEAD_DRAIN_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$ATOMIC_REMOTE_HEAD_DRAIN_INV"
grep -q '^fastmem_memop_field_load_count=1$' "$ATOMIC_REMOTE_HEAD_DRAIN_INV"
grep -q '^fastmem_memop_atomic_remote_head_drain_count=1$' "$ATOMIC_REMOTE_HEAD_DRAIN_INV"
grep -q '^fastmem_forbidden_call_count=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_INV"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_HEAD_DRAIN_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --mir-json "$ATOMIC_REMOTE_HEAD_DRAIN_MIR" \
  --out "$ATOMIC_REMOTE_HEAD_DRAIN_MIR_INV"

grep -q '^input_kind=mir_json_metadata$' "$ATOMIC_REMOTE_HEAD_DRAIN_MIR_INV"
grep -q '^fastmem_region_count=1$' "$ATOMIC_REMOTE_HEAD_DRAIN_MIR_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$ATOMIC_REMOTE_HEAD_DRAIN_MIR_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$ATOMIC_REMOTE_HEAD_DRAIN_MIR_INV"
grep -q '^fastmem_memop_field_load_count=1$' "$ATOMIC_REMOTE_HEAD_DRAIN_MIR_INV"
grep -q '^fastmem_memop_atomic_remote_head_drain_count=1$' "$ATOMIC_REMOTE_HEAD_DRAIN_MIR_INV"
grep -q '^atomic_remote_head_drain_plan_count=1$' "$ATOMIC_REMOTE_HEAD_DRAIN_MIR_INV"
grep -q '^atomic_remote_head_drain_lowerable_count=1$' "$ATOMIC_REMOTE_HEAD_DRAIN_MIR_INV"
grep -q '^atomic_remote_head_access_resolved_count=1$' "$ATOMIC_REMOTE_HEAD_DRAIN_MIR_INV"
grep -q '^atomic_remote_head_memory_order_policy=acquire_exchange$' "$ATOMIC_REMOTE_HEAD_DRAIN_MIR_INV"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_HEAD_DRAIN_MIR_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile remote-free-drain-exchange \
  --mir-json "$ATOMIC_REMOTE_HEAD_DRAIN_MIR" \
  --object-out "$ATOMIC_REMOTE_HEAD_DRAIN_DIRECT_OBJ" \
  --out "$ATOMIC_REMOTE_HEAD_DRAIN_LLVM_REPORT" \
  2>"$ATOMIC_REMOTE_HEAD_DRAIN_LLVM_STDERR"

grep -q '^fastmem_atomic_remote_head_drain_exchange_producer_pilot=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_LLVM_REPORT"
grep -q '^replacement_front_next_producer_slice=atomic_remote_head_drain_to_local_route_selection$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_LLVM_REPORT"
grep -q '^atomic_remote_head_drain_open=1$' "$ATOMIC_REMOTE_HEAD_DRAIN_LLVM_REPORT"
grep -q '^atomic_remote_head_drain_lowered_count=1$' "$ATOMIC_REMOTE_HEAD_DRAIN_LLVM_REPORT"
grep -q '^atomic_remote_head_drain_exchange_order=acquire$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_LLVM_REPORT"
grep -q '^atomic_remote_head_drain_result_kind=remote_free_list_token$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_LLVM_REPORT"
grep -q '^atomic_remote_head_drain_to_local_route_open=0$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_LLVM_REPORT"
grep -q '^remote_owner_branch_routing_open=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_LLVM_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_LLVM_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_LLVM_REPORT"
grep -q '^product_activation=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_LLVM_REPORT"
grep -q '^global_allocator_claim=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_LLVM_REPORT"
grep -q '^winner_claim=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_LLVM_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$ATOMIC_REMOTE_HEAD_DRAIN_LLVM_REPORT" \
  --format kv \
  --out "$ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_CHECK"

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_CHECK"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_HEAD_DRAIN_PREFLIGHT_CHECK"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile remote-free-drain-to-local-selection \
  --mir-json "$ATOMIC_REMOTE_HEAD_DRAIN_MIR" \
  --object-out "$ATOMIC_REMOTE_HEAD_DRAIN_DIRECT_OBJ" \
  --out "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT" \
  2>"$ATOMIC_REMOTE_HEAD_DRAIN_LLVM_STDERR"

grep -q '^fastmem_atomic_remote_head_drain_to_local_route_selection=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^replacement_front_next_producer_slice=atomic_remote_head_drain_to_local_route_producer_pilot$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^atomic_remote_head_drain_open=1$' "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^atomic_remote_head_drain_lowered_count=1$' "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^atomic_remote_head_drain_to_local_route_selected=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^atomic_remote_head_drain_to_local_route_open=0$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^remote_owner_branch_routing_open=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^product_activation=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^global_allocator_claim=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^winner_claim=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT" \
  --format kv \
  --out "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_CHECK"

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_CHECK"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_CHECK"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile remote-free-drain-to-local \
  --mir-json "$ATOMIC_REMOTE_HEAD_DRAIN_MIR" \
  --object-out "$ATOMIC_REMOTE_HEAD_DRAIN_DIRECT_OBJ" \
  --out "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT" \
  2>"$ATOMIC_REMOTE_HEAD_DRAIN_LLVM_STDERR"

grep -q '^fastmem_atomic_remote_head_drain_to_local_route_producer_pilot=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^replacement_front_next_producer_slice=atomic_remote_head_drain_local_list_mutation_preflight$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^atomic_remote_head_drain_open=1$' "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^atomic_remote_head_drain_lowered_count=1$' "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^atomic_remote_head_drain_to_local_route_selected=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^atomic_remote_head_drain_to_local_route_producer_pilot=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^atomic_remote_head_drain_to_local_route_open=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^remote_owner_branch_routing_open=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^product_activation=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^global_allocator_claim=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"
grep -q '^winner_claim=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_REPORT" \
  --format kv \
  --out "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_CHECK"

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_CHECK"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_HEAD_DRAIN_TO_LOCAL_CHECK"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile remote-free-drain-local-list-mutation-preflight \
  --mir-json "$ATOMIC_REMOTE_HEAD_DRAIN_MIR" \
  --object-out "$ATOMIC_REMOTE_HEAD_DRAIN_DIRECT_OBJ" \
  --out "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_REPORT" \
  2>"$ATOMIC_REMOTE_HEAD_DRAIN_LLVM_STDERR"

grep -q '^fastmem_atomic_remote_head_drain_local_list_mutation_preflight=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_REPORT"
grep -q '^replacement_front_next_producer_slice=atomic_remote_head_drain_local_list_mutation_proof$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_REPORT"
grep -q '^atomic_remote_head_drain_open=1$' "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_REPORT"
grep -q '^atomic_remote_head_drain_lowered_count=1$' "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_REPORT"
grep -q '^atomic_remote_head_drain_to_local_route_selected=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_REPORT"
grep -q '^atomic_remote_head_drain_to_local_route_open=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_REPORT"
grep -q '^atomic_remote_head_drain_local_list_mutation_selected=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_REPORT"
grep -q '^atomic_remote_head_drain_local_list_mutation_open=0$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_REPORT"
grep -q '^remote_owner_branch_routing_open=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_REPORT"
grep -q '^product_activation=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_REPORT"
grep -q '^global_allocator_claim=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_REPORT"
grep -q '^winner_claim=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_REPORT" \
  --format kv \
  --out "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_CHECK"

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_CHECK"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_CHECK"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile remote-free-drain-local-list-mutation-proof \
  --mir-json "$ATOMIC_REMOTE_HEAD_DRAIN_MIR" \
  --object-out "$ATOMIC_REMOTE_HEAD_DRAIN_DIRECT_OBJ" \
  --out "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_REPORT" \
  2>"$ATOMIC_REMOTE_HEAD_DRAIN_LLVM_STDERR"

grep -q '^fastmem_atomic_remote_head_drain_local_list_mutation_proof=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_REPORT"
grep -q '^replacement_front_next_producer_slice=atomic_remote_head_drain_local_list_mutation_vocabulary_preflight$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_REPORT"
grep -q '^atomic_remote_head_drain_to_local_route_open=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_REPORT"
grep -q '^atomic_remote_head_drain_local_list_mutation_selected=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_REPORT"
grep -q '^atomic_remote_head_drain_local_list_mutation_open=0$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_REPORT"
grep -q '^atomic_remote_head_drain_local_list_token_escape_count=0$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_REPORT"
grep -q '^atomic_remote_head_drain_local_list_head_class_resolved=1$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_REPORT"
grep -q '^atomic_remote_head_drain_local_list_head_class=owner_local_free_or_free_head$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_REPORT"
grep -q '^atomic_remote_head_drain_local_list_publication_order=verifier_owned_acquire_then_owner_local$' \
  "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_REPORT"
grep -q '^remote_owner_branch_routing_open=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_REPORT"
grep -q '^product_activation=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_REPORT"
grep -q '^global_allocator_claim=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_REPORT"
grep -q '^winner_claim=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_REPORT" \
  --format kv \
  --out "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_CHECK"

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_CHECK"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_HEAD_DRAIN_LOCAL_LIST_MUTATION_PROOF_CHECK"

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$DRAIN_REMOTE_LIST_TO_LOCAL_AST" "$DRAIN_REMOTE_LIST_TO_LOCAL_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR" "$DRAIN_REMOTE_LIST_TO_LOCAL_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --ast-json "$DRAIN_REMOTE_LIST_TO_LOCAL_AST" \
  --out "$DRAIN_REMOTE_LIST_TO_LOCAL_INV"

grep -q '^input_kind=ast_json$' "$DRAIN_REMOTE_LIST_TO_LOCAL_INV"
grep -q '^fastmem_region_count=1$' "$DRAIN_REMOTE_LIST_TO_LOCAL_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$DRAIN_REMOTE_LIST_TO_LOCAL_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$DRAIN_REMOTE_LIST_TO_LOCAL_INV"
grep -q '^fastmem_memop_field_load_count=1$' "$DRAIN_REMOTE_LIST_TO_LOCAL_INV"
grep -q '^fastmem_memop_atomic_remote_head_drain_count=1$' "$DRAIN_REMOTE_LIST_TO_LOCAL_INV"
grep -q '^fastmem_memop_drain_remote_list_to_local_count=1$' "$DRAIN_REMOTE_LIST_TO_LOCAL_INV"
grep -q '^fastmem_forbidden_call_count=0$' "$DRAIN_REMOTE_LIST_TO_LOCAL_INV"
grep -q '^summary=ok$' "$DRAIN_REMOTE_LIST_TO_LOCAL_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --mir-json "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR" \
  --out "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR_INV"

grep -q '^input_kind=mir_json_metadata$' "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR_INV"
grep -q '^fastmem_region_count=1$' "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR_INV"
grep -q '^fastmem_memop_field_load_count=1$' "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR_INV"
grep -q '^fastmem_memop_atomic_remote_head_drain_count=1$' "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR_INV"
grep -q '^fastmem_memop_drain_remote_list_to_local_count=1$' "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR_INV"
grep -q '^atomic_remote_head_drain_plan_count=1$' "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR_INV"
grep -q '^atomic_remote_head_drain_lowerable_count=1$' "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR_INV"
grep -q '^drain_remote_list_to_local_plan_count=1$' "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR_INV"
grep -q '^drain_remote_list_to_local_token_provenance_valid=1$' "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR_INV"
grep -q '^drain_remote_list_to_local_page_operand_valid=1$' "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR_INV"
grep -q '^drain_remote_list_to_local_head_class_resolved=1$' "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR_INV"
grep -q '^drain_remote_list_to_local_lowerable_count=1$' "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR_INV"
grep -q '^atomic_remote_head_drain_local_list_mutation_lowerable_count=1$' "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR_INV"
grep -q '^summary=ok$' "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile remote-free-drain-local-list-mutation-verifier-preconditions \
  --mir-json "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR" \
  --out "$DRAIN_REMOTE_LIST_TO_LOCAL_REPORT"

grep -q '^fastmem_atomic_remote_head_drain_local_list_mutation_verifier_preconditions=1$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_REPORT"
grep -q '^replacement_front_selected_memop_kinds=DrainRemoteListToLocal$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_REPORT"
grep -q '^replacement_front_next_producer_slice=atomic_remote_head_drain_local_list_mutation_lowering_producer_pilot$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_REPORT"
grep -q '^fastmem_memop_drain_remote_list_to_local_count=1$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_REPORT"
grep -q '^drain_remote_list_to_local_plan_count=1$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_REPORT"
grep -q '^drain_remote_list_to_local_token_provenance_valid=1$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_REPORT"
grep -q '^drain_remote_list_to_local_page_operand_valid=1$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_REPORT"
grep -q '^drain_remote_list_to_local_head_class_resolved=1$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_REPORT"
grep -q '^drain_remote_list_to_local_lowerable_count=1$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_REPORT"
grep -q '^atomic_remote_head_drain_local_list_mutation_selected=1$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_REPORT"
grep -q '^atomic_remote_head_drain_local_list_mutation_open=0$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_REPORT"
grep -q '^atomic_remote_head_drain_local_list_mutation_lowerable_count=1$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_REPORT"
grep -q '^remote_owner_branch_routing_open=0$' "$DRAIN_REMOTE_LIST_TO_LOCAL_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$DRAIN_REMOTE_LIST_TO_LOCAL_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' "$DRAIN_REMOTE_LIST_TO_LOCAL_REPORT"
grep -q '^product_activation=0$' "$DRAIN_REMOTE_LIST_TO_LOCAL_REPORT"
grep -q '^global_allocator_claim=0$' "$DRAIN_REMOTE_LIST_TO_LOCAL_REPORT"
grep -q '^winner_claim=0$' "$DRAIN_REMOTE_LIST_TO_LOCAL_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$DRAIN_REMOTE_LIST_TO_LOCAL_REPORT" \
  --format kv \
  --out "$DRAIN_REMOTE_LIST_TO_LOCAL_CHECK"

grep -q '^failure_count=0$' "$DRAIN_REMOTE_LIST_TO_LOCAL_CHECK"
grep -q '^summary=ok$' "$DRAIN_REMOTE_LIST_TO_LOCAL_CHECK"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile remote-free-drain-local-list-mutation-lowering \
  --mir-json "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR" \
  --out "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_REPORT"

grep -q '^fastmem_atomic_remote_head_drain_local_list_mutation_lowering_producer_pilot=1$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_REPORT"
grep -q '^replacement_front_selected_memop_kinds=DrainRemoteListToLocal$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_REPORT"
grep -q '^replacement_front_next_producer_slice=remote_owner_branch_routing_preflight$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_REPORT"
grep -q '^fastmem_memop_drain_remote_list_to_local_count=1$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_REPORT"
grep -q '^drain_remote_list_to_local_plan_count=1$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_REPORT"
grep -q '^drain_remote_list_to_local_token_provenance_valid=1$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_REPORT"
grep -q '^drain_remote_list_to_local_page_operand_valid=1$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_REPORT"
grep -q '^drain_remote_list_to_local_head_class_resolved=1$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_REPORT"
grep -q '^drain_remote_list_to_local_lowerable_count=1$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_REPORT"
grep -q '^atomic_remote_head_drain_local_list_mutation_selected=1$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_REPORT"
grep -q '^atomic_remote_head_drain_local_list_mutation_open=1$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_REPORT"
grep -q '^atomic_remote_head_drain_local_list_mutation_lowerable_count=1$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_REPORT"
grep -q '^atomic_remote_head_drain_local_list_mutation_lowered_count=1$' \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_REPORT"
grep -q '^remote_owner_branch_routing_open=0$' "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_REPORT"
grep -q '^product_activation=0$' "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_REPORT"
grep -q '^global_allocator_claim=0$' "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_REPORT"
grep -q '^winner_claim=0$' "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_REPORT" \
  --format kv \
  --out "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_CHECK"

grep -q '^failure_count=0$' "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_CHECK"
grep -q '^summary=ok$' "$DRAIN_REMOTE_LIST_TO_LOCAL_LOWERING_CHECK"

REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_REPORT="$TMPDIR/remote_owner_branch_routing_preflight_report.kv"
REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_CHECK="$TMPDIR/remote_owner_branch_routing_preflight_check.kv"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile remote-owner-branch-routing-preflight \
  --mir-json "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR" \
  --out "$REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_REPORT"

grep -q '^fastmem_remote_owner_branch_routing_preflight=1$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_route=remote_owner_branch_routing_preflight$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_kinds=RemoteOwnerBranchRouting$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_REPORT"
grep -q '^replacement_front_next_producer_slice=remote_owner_branch_routing_lowering_preflight$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_REPORT"
grep -q '^atomic_remote_head_drain_local_list_mutation_lowered_count=1$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_REPORT"
grep -q '^remote_owner_branch_routing_selected=1$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_REPORT"
grep -q '^remote_owner_branch_routing_open=0$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_REPORT"
grep -q '^remote_owner_branch_routing_lowered_count=0$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' "$REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_REPORT"
grep -q '^product_activation=0$' "$REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_REPORT"
grep -q '^global_allocator_claim=0$' "$REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_REPORT"
grep -q '^winner_claim=0$' "$REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_REPORT" \
  --format kv \
  --out "$REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_CHECK"

grep -q '^failure_count=0$' "$REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_CHECK"
grep -q '^summary=ok$' "$REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_CHECK"

REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_REPORT="$TMPDIR/remote_owner_branch_routing_lowering_preflight_report.kv"
REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_CHECK="$TMPDIR/remote_owner_branch_routing_lowering_preflight_check.kv"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile remote-owner-branch-routing-lowering-preflight \
  --mir-json "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR" \
  --out "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_REPORT"

grep -q '^fastmem_remote_owner_branch_routing_lowering_preflight=1$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_route=remote_owner_branch_routing_lowering_preflight$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_kinds=RemoteOwnerBranchRouting$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_REPORT"
grep -q '^replacement_front_next_producer_slice=remote_owner_branch_routing_lowering_producer_pilot$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_REPORT"
grep -q '^atomic_remote_head_drain_local_list_mutation_lowered_count=1$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_REPORT"
grep -q '^remote_owner_branch_routing_selected=1$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_REPORT"
grep -q '^remote_owner_branch_routing_lowering_selected=1$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_REPORT"
grep -q '^remote_owner_branch_routing_open=0$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_REPORT"
grep -q '^remote_owner_branch_routing_lowered_count=0$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_REPORT"
grep -q '^product_activation=0$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_REPORT"
grep -q '^global_allocator_claim=0$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_REPORT"
grep -q '^winner_claim=0$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_REPORT" \
  --format kv \
  --out "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_CHECK"

grep -q '^failure_count=0$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_CHECK"
grep -q '^summary=ok$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_CHECK"

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_AST" "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR" "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --ast-json "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_AST" \
  --out "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_INV"

grep -q '^input_kind=ast_json$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_INV"
grep -q '^fastmem_region_count=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_INV"
grep -q '^fastmem_memop_field_load_count=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_INV"
grep -q '^fastmem_memop_field_store_count=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_INV"
grep -q '^fastmem_memop_current_alloc_owner_id_count=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_INV"
grep -q '^fastmem_memop_owner_eq_count=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_INV"
grep -q '^fastmem_memop_atomic_remote_head_drain_count=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_INV"
grep -q '^fastmem_memop_drain_remote_list_to_local_count=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_INV"
grep -q '^fastmem_forbidden_call_count=0$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_INV"
grep -q '^summary=ok$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --mir-json "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR" \
  --out "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR_INV"

grep -q '^input_kind=mir_json_metadata$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR_INV"
grep -q '^fastmem_region_count=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR_INV"
grep -q '^fastmem_memop_field_load_count=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR_INV"
grep -q '^fastmem_memop_field_store_count=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR_INV"
grep -q '^fastmem_memop_current_alloc_owner_id_count=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR_INV"
grep -q '^fastmem_memop_owner_eq_count=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR_INV"
grep -q '^fastmem_memop_atomic_remote_head_drain_count=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR_INV"
grep -q '^fastmem_memop_drain_remote_list_to_local_count=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR_INV"
grep -q '^atomic_remote_head_drain_plan_count=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR_INV"
grep -q '^atomic_remote_head_drain_lowerable_count=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR_INV"
grep -q '^drain_remote_list_to_local_plan_count=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR_INV"
grep -q '^drain_remote_list_to_local_token_provenance_valid=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR_INV"
grep -q '^drain_remote_list_to_local_page_operand_valid=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR_INV"
grep -q '^drain_remote_list_to_local_head_class_resolved=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR_INV"
grep -q '^drain_remote_list_to_local_lowerable_count=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR_INV"
grep -q '^atomic_remote_head_drain_local_list_mutation_lowerable_count=1$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR_INV"
grep -q '^summary=ok$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile remote-owner-branch-routing-lowering \
  --mir-json "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR" \
  --out "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_REPORT"

grep -q '^fastmem_remote_owner_branch_routing_lowering_producer_pilot=1$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_REPORT"
grep -q '^replacement_front_selected_route=remote_owner_branch_routing_lowering_producer_pilot$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_REPORT"
grep -q '^replacement_front_selected_memop_kinds=RemoteOwnerBranchRouting$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_REPORT"
grep -q '^replacement_front_next_producer_slice=remote_owner_branch_route_body_preflight$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_REPORT"
grep -q '^memop_current_alloc_owner_id_lowered_count=1$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_REPORT"
grep -q '^memop_owner_eq_lowered_count=1$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_REPORT"
grep -q '^atomic_remote_head_drain_local_list_mutation_lowered_count=1$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_REPORT"
grep -q '^remote_owner_branch_routing_selected=1$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_REPORT"
grep -q '^remote_owner_branch_routing_lowering_selected=1$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_REPORT"
grep -q '^remote_owner_branch_routing_open=1$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_REPORT"
grep -q '^remote_owner_branch_routing_lowered_count=1$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_REPORT"
grep -q '^remote_owner_branch_routing_preflight_requires_branch_cfg_row=0$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_REPORT"
grep -q '^page_local_free_route_cfg_lowering_enabled=0$' \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_REPORT"
grep -q '^product_activation=0$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_REPORT"
grep -q '^global_allocator_claim=0$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_REPORT"
grep -q '^winner_claim=0$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_REPORT" \
  --format kv \
  --out "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_CHECK"

grep -q '^failure_count=0$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_CHECK"
grep -q '^summary=ok$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_CHECK"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile remote-owner-branch-route-body-preflight \
  --mir-json "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR" \
  --out "$REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_REPORT"

grep -q '^fastmem_remote_owner_branch_route_body_preflight=1$' \
  "$REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_route=remote_owner_branch_route_body_preflight$' \
  "$REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_REPORT"
grep -q '^replacement_front_next_producer_slice=fastmem_branch_cfg_preflight$' \
  "$REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_REPORT"
grep -q '^remote_owner_branch_routing_open=1$' \
  "$REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_REPORT"
grep -q '^remote_owner_branch_routing_lowered_count=1$' \
  "$REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_REPORT"
grep -q '^remote_owner_branch_route_body_selected=1$' \
  "$REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_REPORT"
grep -q '^remote_owner_branch_route_body_open=0$' \
  "$REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_REPORT"
grep -q '^page_local_free_route_cfg_lowering_enabled=0$' \
  "$REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' "$REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_REPORT"
grep -q '^product_activation=0$' "$REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_REPORT"
grep -q '^global_allocator_claim=0$' "$REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_REPORT"
grep -q '^winner_claim=0$' "$REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_REPORT" \
  --format kv \
  --out "$REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_CHECK"

grep -q '^failure_count=0$' "$REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_CHECK"
grep -q '^summary=ok$' "$REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_CHECK"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile fastmem-branch-cfg-preflight \
  --mir-json "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR" \
  --out "$FASTMEM_BRANCH_CFG_PREFLIGHT_REPORT"

grep -q '^fastmem_branch_cfg_preflight=1$' "$FASTMEM_BRANCH_CFG_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_route=fastmem_branch_cfg_preflight$' \
  "$FASTMEM_BRANCH_CFG_PREFLIGHT_REPORT"
grep -q '^replacement_front_next_producer_slice=fastmem_branch_cfg_lowering_preflight$' \
  "$FASTMEM_BRANCH_CFG_PREFLIGHT_REPORT"
grep -q '^remote_owner_branch_routing_open=1$' \
  "$FASTMEM_BRANCH_CFG_PREFLIGHT_REPORT"
grep -q '^remote_owner_branch_routing_lowered_count=1$' \
  "$FASTMEM_BRANCH_CFG_PREFLIGHT_REPORT"
grep -q '^remote_owner_branch_route_body_selected=1$' \
  "$FASTMEM_BRANCH_CFG_PREFLIGHT_REPORT"
grep -q '^remote_owner_branch_route_body_open=0$' \
  "$FASTMEM_BRANCH_CFG_PREFLIGHT_REPORT"
grep -q '^fastmem_branch_cfg_selected=1$' "$FASTMEM_BRANCH_CFG_PREFLIGHT_REPORT"
grep -q '^fastmem_branch_cfg_open=0$' "$FASTMEM_BRANCH_CFG_PREFLIGHT_REPORT"
grep -q '^fastmem_branch_cfg_closed_guard=1$' "$FASTMEM_BRANCH_CFG_PREFLIGHT_REPORT"
grep -q '^page_local_free_route_cfg_lowering_enabled=0$' \
  "$FASTMEM_BRANCH_CFG_PREFLIGHT_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$FASTMEM_BRANCH_CFG_PREFLIGHT_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' "$FASTMEM_BRANCH_CFG_PREFLIGHT_REPORT"
grep -q '^product_activation=0$' "$FASTMEM_BRANCH_CFG_PREFLIGHT_REPORT"
grep -q '^global_allocator_claim=0$' "$FASTMEM_BRANCH_CFG_PREFLIGHT_REPORT"
grep -q '^winner_claim=0$' "$FASTMEM_BRANCH_CFG_PREFLIGHT_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FASTMEM_BRANCH_CFG_PREFLIGHT_REPORT" \
  --format kv \
  --out "$FASTMEM_BRANCH_CFG_PREFLIGHT_CHECK"

grep -q '^failure_count=0$' "$FASTMEM_BRANCH_CFG_PREFLIGHT_CHECK"
grep -q '^summary=ok$' "$FASTMEM_BRANCH_CFG_PREFLIGHT_CHECK"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile fastmem-branch-cfg-lowering-preflight \
  --mir-json "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR" \
  --out "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_REPORT"

grep -q '^fastmem_branch_cfg_lowering_preflight=1$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_route=fastmem_branch_cfg_lowering_preflight$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_family=branch_cfg$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_kinds=FastMemBranchCfg$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_REPORT"
grep -q '^replacement_front_next_producer_slice=fastmem_branch_cfg_lowering_producer_pilot$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_REPORT"
grep -q '^remote_owner_branch_route_body_selected=1$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_REPORT"
grep -q '^remote_owner_branch_route_body_open=0$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_REPORT"
grep -q '^fastmem_branch_cfg_selected=1$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_REPORT"
grep -q '^fastmem_branch_cfg_open=0$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_REPORT"
grep -q '^fastmem_branch_cfg_closed_guard=1$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_REPORT"
grep -q '^fastmem_branch_cfg_lowered_count=0$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_REPORT"
grep -q '^page_local_free_route_cfg_lowering_enabled=0$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_REPORT"
grep -q '^product_activation=0$' "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_REPORT"
grep -q '^global_allocator_claim=0$' "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_REPORT"
grep -q '^winner_claim=0$' "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_REPORT" \
  --format kv \
  --out "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_CHECK"

grep -q '^failure_count=0$' "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_CHECK"
grep -q '^summary=ok$' "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_CHECK"

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$FASTMEM_BRANCH_CFG_LOWERING_AST" "$FASTMEM_BRANCH_CFG_LOWERING_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$FASTMEM_BRANCH_CFG_LOWERING_MIR" "$FASTMEM_BRANCH_CFG_LOWERING_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --ast-json "$FASTMEM_BRANCH_CFG_LOWERING_AST" \
  --out "$FASTMEM_BRANCH_CFG_LOWERING_INV"

grep -q '^input_kind=ast_json$' "$FASTMEM_BRANCH_CFG_LOWERING_INV"
grep -q '^fastmem_region_count=1$' "$FASTMEM_BRANCH_CFG_LOWERING_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$FASTMEM_BRANCH_CFG_LOWERING_INV"
grep -q '^fastmem_forbidden_call_count=0$' "$FASTMEM_BRANCH_CFG_LOWERING_INV"
grep -q '^summary=ok$' "$FASTMEM_BRANCH_CFG_LOWERING_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --mir-json "$FASTMEM_BRANCH_CFG_LOWERING_MIR" \
  --out "$FASTMEM_BRANCH_CFG_LOWERING_MIR_INV"

grep -q '^input_kind=mir_json_metadata$' "$FASTMEM_BRANCH_CFG_LOWERING_MIR_INV"
grep -q '^fastmem_region_count=1$' "$FASTMEM_BRANCH_CFG_LOWERING_MIR_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$FASTMEM_BRANCH_CFG_LOWERING_MIR_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$FASTMEM_BRANCH_CFG_LOWERING_MIR_INV"
grep -q '^fastmem_memop_current_alloc_owner_id_count=1$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_MIR_INV"
grep -q '^fastmem_memop_owner_eq_count=1$' "$FASTMEM_BRANCH_CFG_LOWERING_MIR_INV"
grep -q '^fastmem_memop_atomic_remote_head_drain_count=1$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_MIR_INV"
grep -q '^fastmem_memop_drain_remote_list_to_local_count=1$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_MIR_INV"
grep -q '^summary=ok$' "$FASTMEM_BRANCH_CFG_LOWERING_MIR_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile fastmem-branch-cfg-lowering \
  --mir-json "$FASTMEM_BRANCH_CFG_LOWERING_MIR" \
  --out "$FASTMEM_BRANCH_CFG_LOWERING_REPORT" \
  2>"$FASTMEM_BRANCH_CFG_LOWERING_LLVM_STDERR"

grep -q '^fastmem_branch_cfg_lowering_producer_pilot=1$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_REPORT"
grep -q '^replacement_front_selected_route=fastmem_branch_cfg_lowering_producer_pilot$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_REPORT"
grep -q '^replacement_front_selected_memop_family=branch_cfg$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_REPORT"
grep -q '^replacement_front_selected_memop_kinds=FastMemBranchCfg$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_REPORT"
grep -q '^replacement_front_next_producer_slice=same_remote_free_body_preflight$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_REPORT"
grep -q '^remote_owner_branch_route_body_selected=1$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_REPORT"
grep -q '^remote_owner_branch_route_body_open=0$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_REPORT"
grep -q '^fastmem_branch_cfg_selected=1$' "$FASTMEM_BRANCH_CFG_LOWERING_REPORT"
grep -q '^fastmem_branch_cfg_open=1$' "$FASTMEM_BRANCH_CFG_LOWERING_REPORT"
grep -q '^fastmem_branch_cfg_closed_guard=0$' "$FASTMEM_BRANCH_CFG_LOWERING_REPORT"
grep -q '^fastmem_branch_cfg_lowered_count=1$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_REPORT"
grep -q '^fastmem_branch_cfg_source_guard=branch_cfg_open$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_REPORT"
grep -q '^page_local_free_route_cfg_lowering_enabled=0$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$FASTMEM_BRANCH_CFG_LOWERING_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' \
  "$FASTMEM_BRANCH_CFG_LOWERING_REPORT"
grep -q '^product_activation=0$' "$FASTMEM_BRANCH_CFG_LOWERING_REPORT"
grep -q '^global_allocator_claim=0$' "$FASTMEM_BRANCH_CFG_LOWERING_REPORT"
grep -q '^winner_claim=0$' "$FASTMEM_BRANCH_CFG_LOWERING_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FASTMEM_BRANCH_CFG_LOWERING_REPORT" \
  --format kv \
  --out "$FASTMEM_BRANCH_CFG_LOWERING_CHECK"

grep -q '^failure_count=0$' "$FASTMEM_BRANCH_CFG_LOWERING_CHECK"
grep -q '^summary=ok$' "$FASTMEM_BRANCH_CFG_LOWERING_CHECK"

SAME_REMOTE_FREE_BODY_PREFLIGHT_REPORT="$TMPDIR/page_meta_same_remote_free_body_preflight.report.kv"
SAME_REMOTE_FREE_BODY_PREFLIGHT_CHECK="$TMPDIR/page_meta_same_remote_free_body_preflight.check.kv"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile same-remote-free-body-preflight \
  --mir-json "$FASTMEM_BRANCH_CFG_LOWERING_MIR" \
  --out "$SAME_REMOTE_FREE_BODY_PREFLIGHT_REPORT"

grep -q '^fastmem_same_remote_free_body_preflight=1$' \
  "$SAME_REMOTE_FREE_BODY_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_route=same_remote_free_body_preflight$' \
  "$SAME_REMOTE_FREE_BODY_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_family=same_remote_free_body$' \
  "$SAME_REMOTE_FREE_BODY_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_kinds=SameRemoteFreeBody$' \
  "$SAME_REMOTE_FREE_BODY_PREFLIGHT_REPORT"
grep -q '^replacement_front_next_producer_slice=same_remote_free_body_producer_pilot$' \
  "$SAME_REMOTE_FREE_BODY_PREFLIGHT_REPORT"
grep -q '^same_remote_free_body_selected=1$' \
  "$SAME_REMOTE_FREE_BODY_PREFLIGHT_REPORT"
grep -q '^same_remote_free_body_open=0$' \
  "$SAME_REMOTE_FREE_BODY_PREFLIGHT_REPORT"
grep -q '^same_remote_free_body_lowered_count=0$' \
  "$SAME_REMOTE_FREE_BODY_PREFLIGHT_REPORT"
grep -q '^fastmem_branch_cfg_open=1$' "$SAME_REMOTE_FREE_BODY_PREFLIGHT_REPORT"
grep -q '^fastmem_branch_cfg_lowered_count=1$' \
  "$SAME_REMOTE_FREE_BODY_PREFLIGHT_REPORT"
grep -q '^page_local_free_route_cfg_lowering_enabled=0$' \
  "$SAME_REMOTE_FREE_BODY_PREFLIGHT_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$SAME_REMOTE_FREE_BODY_PREFLIGHT_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' \
  "$SAME_REMOTE_FREE_BODY_PREFLIGHT_REPORT"
grep -q '^product_activation=0$' "$SAME_REMOTE_FREE_BODY_PREFLIGHT_REPORT"
grep -q '^global_allocator_claim=0$' "$SAME_REMOTE_FREE_BODY_PREFLIGHT_REPORT"
grep -q '^winner_claim=0$' "$SAME_REMOTE_FREE_BODY_PREFLIGHT_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$SAME_REMOTE_FREE_BODY_PREFLIGHT_REPORT" \
  --format kv \
  --out "$SAME_REMOTE_FREE_BODY_PREFLIGHT_CHECK"

grep -q '^failure_count=0$' "$SAME_REMOTE_FREE_BODY_PREFLIGHT_CHECK"
grep -q '^summary=ok$' "$SAME_REMOTE_FREE_BODY_PREFLIGHT_CHECK"

SAME_REMOTE_FREE_BODY_PRODUCER_REPORT="$TMPDIR/page_meta_same_remote_free_body_producer.report.kv"
SAME_REMOTE_FREE_BODY_PRODUCER_CHECK="$TMPDIR/page_meta_same_remote_free_body_producer.check.kv"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile same-remote-free-body \
  --mir-json "$FASTMEM_BRANCH_CFG_LOWERING_MIR" \
  --out "$SAME_REMOTE_FREE_BODY_PRODUCER_REPORT"

grep -q '^fastmem_same_remote_free_body_producer_pilot=1$' \
  "$SAME_REMOTE_FREE_BODY_PRODUCER_REPORT"
grep -q '^replacement_front_selected_route=same_remote_free_body_producer_pilot$' \
  "$SAME_REMOTE_FREE_BODY_PRODUCER_REPORT"
grep -q '^replacement_front_selected_memop_family=same_remote_free_body$' \
  "$SAME_REMOTE_FREE_BODY_PRODUCER_REPORT"
grep -q '^replacement_front_selected_memop_kinds=SameRemoteFreeBody$' \
  "$SAME_REMOTE_FREE_BODY_PRODUCER_REPORT"
grep -q '^replacement_front_next_producer_slice=page_local_free_route_cfg_preflight$' \
  "$SAME_REMOTE_FREE_BODY_PRODUCER_REPORT"
grep -q '^same_remote_free_body_selected=1$' \
  "$SAME_REMOTE_FREE_BODY_PRODUCER_REPORT"
grep -q '^same_remote_free_body_open=1$' \
  "$SAME_REMOTE_FREE_BODY_PRODUCER_REPORT"
grep -q '^same_remote_free_body_lowered_count=1$' \
  "$SAME_REMOTE_FREE_BODY_PRODUCER_REPORT"
grep -q '^fastmem_branch_cfg_open=1$' "$SAME_REMOTE_FREE_BODY_PRODUCER_REPORT"
grep -q '^fastmem_branch_cfg_lowered_count=1$' \
  "$SAME_REMOTE_FREE_BODY_PRODUCER_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$SAME_REMOTE_FREE_BODY_PRODUCER_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' \
  "$SAME_REMOTE_FREE_BODY_PRODUCER_REPORT"
grep -q '^product_activation=0$' "$SAME_REMOTE_FREE_BODY_PRODUCER_REPORT"
grep -q '^global_allocator_claim=0$' "$SAME_REMOTE_FREE_BODY_PRODUCER_REPORT"
grep -q '^winner_claim=0$' "$SAME_REMOTE_FREE_BODY_PRODUCER_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$SAME_REMOTE_FREE_BODY_PRODUCER_REPORT" \
  --format kv \
  --out "$SAME_REMOTE_FREE_BODY_PRODUCER_CHECK"

grep -q '^failure_count=0$' "$SAME_REMOTE_FREE_BODY_PRODUCER_CHECK"
grep -q '^summary=ok$' "$SAME_REMOTE_FREE_BODY_PRODUCER_CHECK"

PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_REPORT="$TMPDIR/page_meta_page_local_free_route_cfg_preflight.report.kv"
PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_CHECK="$TMPDIR/page_meta_page_local_free_route_cfg_preflight.check.kv"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile page-local-free-route-cfg-preflight \
  --mir-json "$FASTMEM_BRANCH_CFG_LOWERING_MIR" \
  --out "$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_REPORT"

grep -q '^fastmem_page_local_free_route_cfg_preflight=1$' \
  "$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_route=page_local_free_route_cfg_preflight$' \
  "$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_family=page_local_route_cfg$' \
  "$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_kinds=PageLocalFreeRouteCfg$' \
  "$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_REPORT"
grep -q '^replacement_front_next_producer_slice=page_local_free_route_cfg_producer_pilot$' \
  "$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_REPORT"
grep -q '^page_local_free_route_cfg_selected=1$' \
  "$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_REPORT"
grep -q '^page_local_free_route_cfg_lowering_enabled=0$' \
  "$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_REPORT"
grep -q '^same_remote_free_body_open=1$' \
  "$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_REPORT"
grep -q '^same_remote_free_body_lowered_count=1$' \
  "$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' \
  "$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_REPORT"
grep -q '^product_activation=0$' "$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_REPORT"
grep -q '^global_allocator_claim=0$' "$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_REPORT"
grep -q '^winner_claim=0$' "$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_REPORT" \
  --format kv \
  --out "$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_CHECK"

grep -q '^failure_count=0$' "$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_CHECK"
grep -q '^summary=ok$' "$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_CHECK"

PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_REPORT="$TMPDIR/page_meta_page_local_free_route_cfg_producer.report.kv"
PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_CHECK="$TMPDIR/page_meta_page_local_free_route_cfg_producer.check.kv"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile page-local-free-route-cfg \
  --mir-json "$FASTMEM_BRANCH_CFG_LOWERING_MIR" \
  --out "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_REPORT"

grep -q '^fastmem_page_local_free_route_cfg_producer_pilot=1$' \
  "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_REPORT"
grep -q '^replacement_front_selected_route=page_local_free_route_cfg_producer_pilot$' \
  "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_REPORT"
grep -q '^replacement_front_selected_memop_family=page_local_route_cfg$' \
  "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_REPORT"
grep -q '^replacement_front_selected_memop_kinds=PageLocalFreeRouteCfg$' \
  "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_REPORT"
grep -q '^replacement_front_next_producer_slice=tls_backing_transfer_preflight$' \
  "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_REPORT"
grep -q '^page_local_free_route_cfg_selected=1$' \
  "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_REPORT"
grep -q '^page_local_free_route_cfg_lowering_enabled=1$' \
  "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_REPORT"
grep -q '^same_remote_free_body_open=1$' \
  "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_REPORT"
grep -q '^same_remote_free_body_lowered_count=1$' \
  "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_REPORT"
grep -q '^tls_backing_transfer_enabled=0$' \
  "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' \
  "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_REPORT"
grep -q '^product_activation=0$' "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_REPORT"
grep -q '^global_allocator_claim=0$' "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_REPORT"
grep -q '^winner_claim=0$' "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_REPORT" \
  --format kv \
  --out "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_CHECK"

grep -q '^failure_count=0$' "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_CHECK"
grep -q '^summary=ok$' "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_CHECK"

TLS_BACKING_TRANSFER_PREFLIGHT_REPORT="$TMPDIR/page_meta_tls_backing_transfer_preflight.report.kv"
TLS_BACKING_TRANSFER_PREFLIGHT_CHECK="$TMPDIR/page_meta_tls_backing_transfer_preflight.check.kv"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile tls-backing-transfer-preflight \
  --mir-json "$FASTMEM_BRANCH_CFG_LOWERING_MIR" \
  --out "$TLS_BACKING_TRANSFER_PREFLIGHT_REPORT"

grep -q '^fastmem_tls_backing_transfer_preflight=1$' \
  "$TLS_BACKING_TRANSFER_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_route=tls_backing_transfer_preflight$' \
  "$TLS_BACKING_TRANSFER_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_family=page_local_route_cfg$' \
  "$TLS_BACKING_TRANSFER_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_kinds=PageLocalFreeRouteCfg$' \
  "$TLS_BACKING_TRANSFER_PREFLIGHT_REPORT"
grep -q '^replacement_front_next_producer_slice=tls_backing_transfer_producer_pilot$' \
  "$TLS_BACKING_TRANSFER_PREFLIGHT_REPORT"
grep -q '^tls_backing_transfer_selected=1$' \
  "$TLS_BACKING_TRANSFER_PREFLIGHT_REPORT"
grep -q '^tls_backing_transfer_enabled=0$' \
  "$TLS_BACKING_TRANSFER_PREFLIGHT_REPORT"
grep -q '^page_local_free_route_cfg_selected=1$' \
  "$TLS_BACKING_TRANSFER_PREFLIGHT_REPORT"
grep -q '^page_local_free_route_cfg_lowering_enabled=1$' \
  "$TLS_BACKING_TRANSFER_PREFLIGHT_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$TLS_BACKING_TRANSFER_PREFLIGHT_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' \
  "$TLS_BACKING_TRANSFER_PREFLIGHT_REPORT"
grep -q '^product_activation=0$' "$TLS_BACKING_TRANSFER_PREFLIGHT_REPORT"
grep -q '^global_allocator_claim=0$' "$TLS_BACKING_TRANSFER_PREFLIGHT_REPORT"
grep -q '^winner_claim=0$' "$TLS_BACKING_TRANSFER_PREFLIGHT_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$TLS_BACKING_TRANSFER_PREFLIGHT_REPORT" \
  --format kv \
  --out "$TLS_BACKING_TRANSFER_PREFLIGHT_CHECK"

grep -q '^failure_count=0$' "$TLS_BACKING_TRANSFER_PREFLIGHT_CHECK"
grep -q '^summary=ok$' "$TLS_BACKING_TRANSFER_PREFLIGHT_CHECK"

TLS_BACKING_TRANSFER_PRODUCER_REPORT="$TMPDIR/page_meta_tls_backing_transfer_producer.report.kv"
TLS_BACKING_TRANSFER_PRODUCER_CHECK="$TMPDIR/page_meta_tls_backing_transfer_producer.check.kv"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile tls-backing-transfer-producer-pilot \
  --mir-json "$FASTMEM_BRANCH_CFG_LOWERING_MIR" \
  --out "$TLS_BACKING_TRANSFER_PRODUCER_REPORT"

grep -q '^fastmem_tls_backing_transfer_producer_pilot=1$' \
  "$TLS_BACKING_TRANSFER_PRODUCER_REPORT"
grep -q '^replacement_front_selected_route=tls_backing_transfer_producer_pilot$' \
  "$TLS_BACKING_TRANSFER_PRODUCER_REPORT"
grep -q '^replacement_front_selected_memop_family=tls_backing_transfer$' \
  "$TLS_BACKING_TRANSFER_PRODUCER_REPORT"
grep -q '^replacement_front_selected_memop_kinds=TlsBackingTransfer$' \
  "$TLS_BACKING_TRANSFER_PRODUCER_REPORT"
grep -q '^replacement_front_next_producer_slice=owner_slot_reuse_preflight$' \
  "$TLS_BACKING_TRANSFER_PRODUCER_REPORT"
grep -q '^tls_backing_transfer_selected=1$' \
  "$TLS_BACKING_TRANSFER_PRODUCER_REPORT"
grep -q '^tls_backing_transfer_enabled=1$' \
  "$TLS_BACKING_TRANSFER_PRODUCER_REPORT"
grep -q '^page_local_free_route_cfg_selected=1$' \
  "$TLS_BACKING_TRANSFER_PRODUCER_REPORT"
grep -q '^page_local_free_route_cfg_lowering_enabled=1$' \
  "$TLS_BACKING_TRANSFER_PRODUCER_REPORT"
grep -q '^allocator_owner_slot_reuse_enabled=0$' \
  "$TLS_BACKING_TRANSFER_PRODUCER_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$TLS_BACKING_TRANSFER_PRODUCER_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' \
  "$TLS_BACKING_TRANSFER_PRODUCER_REPORT"
grep -q '^product_activation=0$' "$TLS_BACKING_TRANSFER_PRODUCER_REPORT"
grep -q '^global_allocator_claim=0$' "$TLS_BACKING_TRANSFER_PRODUCER_REPORT"
grep -q '^winner_claim=0$' "$TLS_BACKING_TRANSFER_PRODUCER_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$TLS_BACKING_TRANSFER_PRODUCER_REPORT" \
  --format kv \
  --out "$TLS_BACKING_TRANSFER_PRODUCER_CHECK"

grep -q '^failure_count=0$' "$TLS_BACKING_TRANSFER_PRODUCER_CHECK"
grep -q '^summary=ok$' "$TLS_BACKING_TRANSFER_PRODUCER_CHECK"

OWNER_SLOT_REUSE_PREFLIGHT_REPORT="$TMPDIR/page_meta_owner_slot_reuse_preflight.report.kv"
OWNER_SLOT_REUSE_PREFLIGHT_CHECK="$TMPDIR/page_meta_owner_slot_reuse_preflight.check.kv"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile owner-slot-reuse-preflight \
  --mir-json "$FASTMEM_BRANCH_CFG_LOWERING_MIR" \
  --out "$OWNER_SLOT_REUSE_PREFLIGHT_REPORT"

grep -q '^fastmem_allocator_owner_slot_reuse_preflight=1$' \
  "$OWNER_SLOT_REUSE_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_route=owner_slot_reuse_preflight$' \
  "$OWNER_SLOT_REUSE_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_family=owner_slot_reuse$' \
  "$OWNER_SLOT_REUSE_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_kinds=OwnerSlotReuse$' \
  "$OWNER_SLOT_REUSE_PREFLIGHT_REPORT"
grep -q '^replacement_front_next_producer_slice=owner_slot_reuse_producer_pilot$' \
  "$OWNER_SLOT_REUSE_PREFLIGHT_REPORT"
grep -q '^allocator_owner_slot_reuse_selected=1$' \
  "$OWNER_SLOT_REUSE_PREFLIGHT_REPORT"
grep -q '^allocator_owner_slot_reuse_enabled=0$' \
  "$OWNER_SLOT_REUSE_PREFLIGHT_REPORT"
grep -q '^allocator_owner_reuse_without_generation_bump_count=0$' \
  "$OWNER_SLOT_REUSE_PREFLIGHT_REPORT"
grep -q '^tls_backing_transfer_selected=1$' \
  "$OWNER_SLOT_REUSE_PREFLIGHT_REPORT"
grep -q '^tls_backing_transfer_enabled=1$' \
  "$OWNER_SLOT_REUSE_PREFLIGHT_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$OWNER_SLOT_REUSE_PREFLIGHT_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' \
  "$OWNER_SLOT_REUSE_PREFLIGHT_REPORT"
grep -q '^product_activation=0$' "$OWNER_SLOT_REUSE_PREFLIGHT_REPORT"
grep -q '^global_allocator_claim=0$' "$OWNER_SLOT_REUSE_PREFLIGHT_REPORT"
grep -q '^winner_claim=0$' "$OWNER_SLOT_REUSE_PREFLIGHT_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$OWNER_SLOT_REUSE_PREFLIGHT_REPORT" \
  --format kv \
  --out "$OWNER_SLOT_REUSE_PREFLIGHT_CHECK"

grep -q '^failure_count=0$' "$OWNER_SLOT_REUSE_PREFLIGHT_CHECK"
grep -q '^summary=ok$' "$OWNER_SLOT_REUSE_PREFLIGHT_CHECK"

OWNER_SLOT_REUSE_PRODUCER_REPORT="$TMPDIR/page_meta_owner_slot_reuse_producer.report.kv"
OWNER_SLOT_REUSE_PRODUCER_CHECK="$TMPDIR/page_meta_owner_slot_reuse_producer.check.kv"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile owner-slot-reuse-producer-pilot \
  --mir-json "$FASTMEM_BRANCH_CFG_LOWERING_MIR" \
  --out "$OWNER_SLOT_REUSE_PRODUCER_REPORT"

grep -q '^fastmem_allocator_owner_slot_reuse_producer_pilot=1$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REPORT"
grep -q '^replacement_front_selected_route=owner_slot_reuse_producer_pilot$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REPORT"
grep -q '^replacement_front_selected_memop_family=owner_slot_reuse$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REPORT"
grep -q '^replacement_front_selected_memop_kinds=OwnerSlotReuse$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REPORT"
grep -q '^replacement_front_next_producer_slice=abandoned_reclaim_preflight$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REPORT"
grep -q '^allocator_owner_slot_reuse_selected=1$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REPORT"
grep -q '^allocator_owner_slot_reuse_enabled=1$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REPORT"
grep -q '^allocator_owner_generation_bump_count=1$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REPORT"
grep -q '^allocator_owner_reuse_without_generation_bump_count=0$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REPORT"
grep -q '^tls_backing_transfer_selected=1$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REPORT"
grep -q '^tls_backing_transfer_enabled=1$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$OWNER_SLOT_REUSE_PRODUCER_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REPORT"
grep -q '^product_activation=0$' "$OWNER_SLOT_REUSE_PRODUCER_REPORT"
grep -q '^global_allocator_claim=0$' "$OWNER_SLOT_REUSE_PRODUCER_REPORT"
grep -q '^winner_claim=0$' "$OWNER_SLOT_REUSE_PRODUCER_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$OWNER_SLOT_REUSE_PRODUCER_REPORT" \
  --format kv \
  --out "$OWNER_SLOT_REUSE_PRODUCER_CHECK"

grep -q '^failure_count=0$' "$OWNER_SLOT_REUSE_PRODUCER_CHECK"
grep -q '^summary=ok$' "$OWNER_SLOT_REUSE_PRODUCER_CHECK"

ABANDONED_RECLAIM_PREFLIGHT_REPORT="$TMPDIR/page_meta_abandoned_reclaim_preflight.report.kv"
ABANDONED_RECLAIM_PREFLIGHT_CHECK="$TMPDIR/page_meta_abandoned_reclaim_preflight.check.kv"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile abandoned-reclaim-preflight \
  --mir-json "$FASTMEM_BRANCH_CFG_LOWERING_MIR" \
  --out "$ABANDONED_RECLAIM_PREFLIGHT_REPORT"

grep -q '^fastmem_abandoned_reclaim_preflight=1$' \
  "$ABANDONED_RECLAIM_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_route=abandoned_reclaim_preflight$' \
  "$ABANDONED_RECLAIM_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_family=abandoned_reclaim$' \
  "$ABANDONED_RECLAIM_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_kinds=AbandonedReclaim$' \
  "$ABANDONED_RECLAIM_PREFLIGHT_REPORT"
grep -q '^replacement_front_next_producer_slice=abandoned_reclaim_producer_pilot$' \
  "$ABANDONED_RECLAIM_PREFLIGHT_REPORT"
grep -q '^abandoned_reclaim_selected=1$' \
  "$ABANDONED_RECLAIM_PREFLIGHT_REPORT"
grep -q '^abandoned_reclaim_enabled=0$' \
  "$ABANDONED_RECLAIM_PREFLIGHT_REPORT"
grep -q '^page_reclaimed_with_remote_candidates=0$' \
  "$ABANDONED_RECLAIM_PREFLIGHT_REPORT"
grep -q '^allocator_owner_slot_reuse_enabled=1$' \
  "$ABANDONED_RECLAIM_PREFLIGHT_REPORT"
grep -q '^allocator_owner_generation_bump_count=1$' \
  "$ABANDONED_RECLAIM_PREFLIGHT_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$ABANDONED_RECLAIM_PREFLIGHT_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' \
  "$ABANDONED_RECLAIM_PREFLIGHT_REPORT"
grep -q '^product_activation=0$' "$ABANDONED_RECLAIM_PREFLIGHT_REPORT"
grep -q '^global_allocator_claim=0$' "$ABANDONED_RECLAIM_PREFLIGHT_REPORT"
grep -q '^winner_claim=0$' "$ABANDONED_RECLAIM_PREFLIGHT_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$ABANDONED_RECLAIM_PREFLIGHT_REPORT" \
  --format kv \
  --out "$ABANDONED_RECLAIM_PREFLIGHT_CHECK"

grep -q '^failure_count=0$' "$ABANDONED_RECLAIM_PREFLIGHT_CHECK"
grep -q '^summary=ok$' "$ABANDONED_RECLAIM_PREFLIGHT_CHECK"

ABANDONED_RECLAIM_PRODUCER_REPORT="$TMPDIR/page_meta_abandoned_reclaim_producer.report.kv"
ABANDONED_RECLAIM_PRODUCER_CHECK="$TMPDIR/page_meta_abandoned_reclaim_producer.check.kv"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile abandoned-reclaim-producer-pilot \
  --mir-json "$FASTMEM_BRANCH_CFG_LOWERING_MIR" \
  --out "$ABANDONED_RECLAIM_PRODUCER_REPORT"

grep -q '^fastmem_abandoned_reclaim_producer_pilot=1$' \
  "$ABANDONED_RECLAIM_PRODUCER_REPORT"
grep -q '^replacement_front_selected_route=abandoned_reclaim_producer_pilot$' \
  "$ABANDONED_RECLAIM_PRODUCER_REPORT"
grep -q '^replacement_front_selected_memop_family=abandoned_reclaim$' \
  "$ABANDONED_RECLAIM_PRODUCER_REPORT"
grep -q '^replacement_front_selected_memop_kinds=AbandonedReclaim$' \
  "$ABANDONED_RECLAIM_PRODUCER_REPORT"
grep -q '^replacement_front_next_producer_slice=product_activation_preflight$' \
  "$ABANDONED_RECLAIM_PRODUCER_REPORT"
grep -q '^abandoned_reclaim_selected=1$' \
  "$ABANDONED_RECLAIM_PRODUCER_REPORT"
grep -q '^abandoned_reclaim_enabled=1$' \
  "$ABANDONED_RECLAIM_PRODUCER_REPORT"
grep -q '^page_reclaimed_with_remote_candidates=0$' \
  "$ABANDONED_RECLAIM_PRODUCER_REPORT"
grep -q '^allocator_owner_slot_reuse_enabled=1$' \
  "$ABANDONED_RECLAIM_PRODUCER_REPORT"
grep -q '^allocator_owner_generation_bump_count=1$' \
  "$ABANDONED_RECLAIM_PRODUCER_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$ABANDONED_RECLAIM_PRODUCER_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' \
  "$ABANDONED_RECLAIM_PRODUCER_REPORT"
grep -q '^product_activation=0$' "$ABANDONED_RECLAIM_PRODUCER_REPORT"
grep -q '^global_allocator_claim=0$' "$ABANDONED_RECLAIM_PRODUCER_REPORT"
grep -q '^winner_claim=0$' "$ABANDONED_RECLAIM_PRODUCER_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$ABANDONED_RECLAIM_PRODUCER_REPORT" \
  --format kv \
  --out "$ABANDONED_RECLAIM_PRODUCER_CHECK"

grep -q '^failure_count=0$' "$ABANDONED_RECLAIM_PRODUCER_CHECK"
grep -q '^summary=ok$' "$ABANDONED_RECLAIM_PRODUCER_CHECK"

PRODUCT_ACTIVATION_PREFLIGHT_REPORT="$TMPDIR/page_meta_product_activation_preflight.report.kv"
PRODUCT_ACTIVATION_PREFLIGHT_CHECK="$TMPDIR/page_meta_product_activation_preflight.check.kv"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile product-activation-preflight \
  --mir-json "$FASTMEM_BRANCH_CFG_LOWERING_MIR" \
  --out "$PRODUCT_ACTIVATION_PREFLIGHT_REPORT"

grep -q '^fastmem_product_activation_preflight=1$' \
  "$PRODUCT_ACTIVATION_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_route=product_activation_preflight$' \
  "$PRODUCT_ACTIVATION_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_family=product_activation$' \
  "$PRODUCT_ACTIVATION_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_kinds=ProductActivation$' \
  "$PRODUCT_ACTIVATION_PREFLIGHT_REPORT"
grep -q '^replacement_front_next_producer_slice=product_activation_producer_pilot$' \
  "$PRODUCT_ACTIVATION_PREFLIGHT_REPORT"
grep -q '^product_activation_selected=1$' \
  "$PRODUCT_ACTIVATION_PREFLIGHT_REPORT"
grep -q '^abandoned_reclaim_selected=1$' \
  "$PRODUCT_ACTIVATION_PREFLIGHT_REPORT"
grep -q '^abandoned_reclaim_enabled=1$' \
  "$PRODUCT_ACTIVATION_PREFLIGHT_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$PRODUCT_ACTIVATION_PREFLIGHT_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' \
  "$PRODUCT_ACTIVATION_PREFLIGHT_REPORT"
grep -q '^product_activation=0$' "$PRODUCT_ACTIVATION_PREFLIGHT_REPORT"
grep -q '^hook_install=0$' "$PRODUCT_ACTIVATION_PREFLIGHT_REPORT"
grep -q '^global_allocator_claim=0$' "$PRODUCT_ACTIVATION_PREFLIGHT_REPORT"
grep -q '^winner_claim=0$' "$PRODUCT_ACTIVATION_PREFLIGHT_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$PRODUCT_ACTIVATION_PREFLIGHT_REPORT" \
  --format kv \
  --out "$PRODUCT_ACTIVATION_PREFLIGHT_CHECK"

grep -q '^failure_count=0$' "$PRODUCT_ACTIVATION_PREFLIGHT_CHECK"
grep -q '^summary=ok$' "$PRODUCT_ACTIVATION_PREFLIGHT_CHECK"

PRODUCT_ACTIVATION_PRODUCER_REPORT="$TMPDIR/page_meta_product_activation_producer.report.kv"
PRODUCT_ACTIVATION_PRODUCER_CHECK="$TMPDIR/page_meta_product_activation_producer.check.kv"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile product-activation-producer-pilot \
  --mir-json "$FASTMEM_BRANCH_CFG_LOWERING_MIR" \
  --out "$PRODUCT_ACTIVATION_PRODUCER_REPORT"

grep -q '^fastmem_product_activation_producer_pilot=1$' \
  "$PRODUCT_ACTIVATION_PRODUCER_REPORT"
grep -q '^replacement_front_selected_route=product_activation_producer_pilot$' \
  "$PRODUCT_ACTIVATION_PRODUCER_REPORT"
grep -q '^replacement_front_selected_memop_family=product_activation$' \
  "$PRODUCT_ACTIVATION_PRODUCER_REPORT"
grep -q '^replacement_front_selected_memop_kinds=ProductActivation$' \
  "$PRODUCT_ACTIVATION_PRODUCER_REPORT"
grep -q '^replacement_front_next_producer_slice=hook_install_preflight$' \
  "$PRODUCT_ACTIVATION_PRODUCER_REPORT"
grep -q '^product_activation_selected=1$' \
  "$PRODUCT_ACTIVATION_PRODUCER_REPORT"
grep -q '^product_activation=1$' "$PRODUCT_ACTIVATION_PRODUCER_REPORT"
grep -q '^hook_install=0$' "$PRODUCT_ACTIVATION_PRODUCER_REPORT"
grep -q '^global_allocator_claim=0$' "$PRODUCT_ACTIVATION_PRODUCER_REPORT"
grep -q '^winner_claim=0$' "$PRODUCT_ACTIVATION_PRODUCER_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$PRODUCT_ACTIVATION_PRODUCER_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' \
  "$PRODUCT_ACTIVATION_PRODUCER_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$PRODUCT_ACTIVATION_PRODUCER_REPORT" \
  --format kv \
  --out "$PRODUCT_ACTIVATION_PRODUCER_CHECK"

grep -q '^failure_count=0$' "$PRODUCT_ACTIVATION_PRODUCER_CHECK"
grep -q '^summary=ok$' "$PRODUCT_ACTIVATION_PRODUCER_CHECK"

HOOK_INSTALL_PREFLIGHT_REPORT="$TMPDIR/page_meta_hook_install_preflight.report.kv"
HOOK_INSTALL_PREFLIGHT_CHECK="$TMPDIR/page_meta_hook_install_preflight.check.kv"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile hook-install-preflight \
  --mir-json "$FASTMEM_BRANCH_CFG_LOWERING_MIR" \
  --out "$HOOK_INSTALL_PREFLIGHT_REPORT"

grep -q '^fastmem_hook_install_preflight=1$' \
  "$HOOK_INSTALL_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_route=hook_install_preflight$' \
  "$HOOK_INSTALL_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_family=hook_install$' \
  "$HOOK_INSTALL_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_kinds=HookInstall$' \
  "$HOOK_INSTALL_PREFLIGHT_REPORT"
grep -q '^replacement_front_next_producer_slice=hook_install_producer_pilot$' \
  "$HOOK_INSTALL_PREFLIGHT_REPORT"
grep -q '^hook_install_selected=1$' "$HOOK_INSTALL_PREFLIGHT_REPORT"
grep -q '^product_activation_selected=1$' "$HOOK_INSTALL_PREFLIGHT_REPORT"
grep -q '^product_activation=1$' "$HOOK_INSTALL_PREFLIGHT_REPORT"
grep -q '^hook_install=0$' "$HOOK_INSTALL_PREFLIGHT_REPORT"
grep -q '^global_allocator_claim=0$' "$HOOK_INSTALL_PREFLIGHT_REPORT"
grep -q '^winner_claim=0$' "$HOOK_INSTALL_PREFLIGHT_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$HOOK_INSTALL_PREFLIGHT_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' \
  "$HOOK_INSTALL_PREFLIGHT_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$HOOK_INSTALL_PREFLIGHT_REPORT" \
  --format kv \
  --out "$HOOK_INSTALL_PREFLIGHT_CHECK"

grep -q '^failure_count=0$' "$HOOK_INSTALL_PREFLIGHT_CHECK"
grep -q '^summary=ok$' "$HOOK_INSTALL_PREFLIGHT_CHECK"

HOOK_INSTALL_PRODUCER_REPORT="$TMPDIR/page_meta_hook_install_producer.report.kv"
HOOK_INSTALL_PRODUCER_CHECK="$TMPDIR/page_meta_hook_install_producer.check.kv"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile hook-install-producer-pilot \
  --mir-json "$FASTMEM_BRANCH_CFG_LOWERING_MIR" \
  --out "$HOOK_INSTALL_PRODUCER_REPORT"

grep -q '^fastmem_hook_install_producer_pilot=1$' \
  "$HOOK_INSTALL_PRODUCER_REPORT"
grep -q '^replacement_front_selected_route=hook_install_producer_pilot$' \
  "$HOOK_INSTALL_PRODUCER_REPORT"
grep -q '^replacement_front_selected_memop_family=hook_install$' \
  "$HOOK_INSTALL_PRODUCER_REPORT"
grep -q '^replacement_front_selected_memop_kinds=HookInstall$' \
  "$HOOK_INSTALL_PRODUCER_REPORT"
grep -q '^replacement_front_next_producer_slice=global_allocator_claim_preflight$' \
  "$HOOK_INSTALL_PRODUCER_REPORT"
grep -q '^hook_install_selected=1$' "$HOOK_INSTALL_PRODUCER_REPORT"
grep -q '^hook_install=1$' "$HOOK_INSTALL_PRODUCER_REPORT"
grep -q '^product_activation_selected=1$' "$HOOK_INSTALL_PRODUCER_REPORT"
grep -q '^product_activation=1$' "$HOOK_INSTALL_PRODUCER_REPORT"
grep -q '^global_allocator_claim=0$' "$HOOK_INSTALL_PRODUCER_REPORT"
grep -q '^winner_claim=0$' "$HOOK_INSTALL_PRODUCER_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$HOOK_INSTALL_PRODUCER_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' \
  "$HOOK_INSTALL_PRODUCER_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$HOOK_INSTALL_PRODUCER_REPORT" \
  --format kv \
  --out "$HOOK_INSTALL_PRODUCER_CHECK"

grep -q '^failure_count=0$' "$HOOK_INSTALL_PRODUCER_CHECK"
grep -q '^summary=ok$' "$HOOK_INSTALL_PRODUCER_CHECK"

GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_REPORT="$TMPDIR/page_meta_global_allocator_claim_preflight.report.kv"
GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_CHECK="$TMPDIR/page_meta_global_allocator_claim_preflight.check.kv"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile global-allocator-claim-preflight \
  --mir-json "$FASTMEM_BRANCH_CFG_LOWERING_MIR" \
  --out "$GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_REPORT"

grep -q '^fastmem_global_allocator_claim_preflight=1$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_route=global_allocator_claim_preflight$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_family=global_allocator_claim$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_kinds=GlobalAllocatorClaim$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_REPORT"
grep -q '^replacement_front_next_producer_slice=global_allocator_claim_producer_pilot$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_REPORT"
grep -q '^global_allocator_claim_selected=1$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_REPORT"
grep -q '^hook_install_selected=1$' "$GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_REPORT"
grep -q '^hook_install=1$' "$GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_REPORT"
grep -q '^product_activation_selected=1$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_REPORT"
grep -q '^product_activation=1$' "$GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_REPORT"
grep -q '^global_allocator_claim=0$' "$GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_REPORT"
grep -q '^winner_claim=0$' "$GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_REPORT" \
  --format kv \
  --out "$GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_CHECK"

grep -q '^failure_count=0$' "$GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_CHECK"
grep -q '^summary=ok$' "$GLOBAL_ALLOCATOR_CLAIM_PREFLIGHT_CHECK"

GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REPORT="$TMPDIR/page_meta_global_allocator_claim_producer.report.kv"
GLOBAL_ALLOCATOR_CLAIM_PRODUCER_CHECK="$TMPDIR/page_meta_global_allocator_claim_producer.check.kv"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile global-allocator-claim-producer-pilot \
  --mir-json "$FASTMEM_BRANCH_CFG_LOWERING_MIR" \
  --out "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REPORT"

grep -q '^fastmem_global_allocator_claim_producer_pilot=1$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REPORT"
grep -q '^replacement_front_selected_route=global_allocator_claim_producer_pilot$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REPORT"
grep -q '^replacement_front_selected_memop_family=global_allocator_claim$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REPORT"
grep -q '^replacement_front_selected_memop_kinds=GlobalAllocatorClaim$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REPORT"
grep -q '^replacement_front_next_producer_slice=winner_claim_preflight$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REPORT"
grep -q '^global_allocator_claim_selected=1$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REPORT"
grep -q '^global_allocator_claim=1$' "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REPORT"
grep -q '^hook_install_selected=1$' "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REPORT"
grep -q '^hook_install=1$' "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REPORT"
grep -q '^product_activation_selected=1$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REPORT"
grep -q '^product_activation=1$' "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REPORT"
grep -q '^winner_claim=0$' "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REPORT" \
  --format kv \
  --out "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_CHECK"

grep -q '^failure_count=0$' "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_CHECK"
grep -q '^summary=ok$' "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_CHECK"

python3 "$ROOT/src/llvm_py/llvm_builder.py" \
  "$FASTMEM_BRANCH_CFG_LOWERING_MIR" \
  -o "$TMPDIR/page_meta_fastmem_branch_cfg_lowering.direct.o" \
  2>"$FASTMEM_BRANCH_CFG_LOWERING_LLVM_STDERR"
test -f "$TMPDIR/page_meta_fastmem_branch_cfg_lowering.direct.o"

python3 "$ROOT/src/llvm_py/llvm_builder.py" \
  "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_MIR" \
  -o "$TMPDIR/page_meta_remote_owner_branch_routing_lowering.direct.o" \
  2>"$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_LLVM_STDERR"
test -f "$TMPDIR/page_meta_remote_owner_branch_routing_lowering.direct.o"

python3 "$ROOT/src/llvm_py/llvm_builder.py" \
  "$DRAIN_REMOTE_LIST_TO_LOCAL_MIR" \
  -o "$TMPDIR/page_meta_drain_remote_list_to_local.direct.o" \
  2>"$DRAIN_REMOTE_LIST_TO_LOCAL_LLVM_STDERR"
test -f "$TMPDIR/page_meta_drain_remote_list_to_local.direct.o"

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
grep -q '^replacement_front_deferred_memop_kinds=LocalFreePop,FreeHeadPush,FreeHeadPop,AtomicRemoteHead$' "$LOCAL_FREE_PUSH_PRECONDITION_LLVM_REPORT"
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

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$FREE_HEAD_POP_AST" "$FREE_HEAD_POP_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$FREE_HEAD_POP_MIR" "$FREE_HEAD_POP_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --ast-json "$FREE_HEAD_POP_AST" \
  --out "$FREE_HEAD_POP_INV"

grep -q '^input_kind=ast_json$' "$FREE_HEAD_POP_INV"
grep -q '^fastmem_region_count=1$' "$FREE_HEAD_POP_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$FREE_HEAD_POP_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$FREE_HEAD_POP_INV"
grep -q '^fastmem_memop_field_load_count=0$' "$FREE_HEAD_POP_INV"
grep -q '^fastmem_memop_field_store_count=1$' "$FREE_HEAD_POP_INV"
grep -q '^fastmem_memop_free_head_pop_count=1$' "$FREE_HEAD_POP_INV"
grep -q '^fastmem_forbidden_call_count=0$' "$FREE_HEAD_POP_INV"
grep -q '^summary=ok$' "$FREE_HEAD_POP_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --mir-json "$FREE_HEAD_POP_MIR" \
  --out "$FREE_HEAD_POP_MIR_INV"

grep -q '^input_kind=mir_json_metadata$' "$FREE_HEAD_POP_MIR_INV"
grep -q '^fastmem_region_count=1$' "$FREE_HEAD_POP_MIR_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$FREE_HEAD_POP_MIR_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$FREE_HEAD_POP_MIR_INV"
grep -q '^fastmem_memop_field_load_count=0$' "$FREE_HEAD_POP_MIR_INV"
grep -q '^fastmem_memop_field_store_count=1$' "$FREE_HEAD_POP_MIR_INV"
grep -q '^fastmem_memop_free_head_pop_count=1$' "$FREE_HEAD_POP_MIR_INV"
grep -q '^fastmem_verified_mem_access_plan_count=2$' "$FREE_HEAD_POP_MIR_INV"
grep -q '^fastmem_verified_field_access_count=1$' "$FREE_HEAD_POP_MIR_INV"
grep -q '^fastmem_verified_table_access_count=1$' "$FREE_HEAD_POP_MIR_INV"
grep -q '^summary=ok$' "$FREE_HEAD_POP_MIR_INV"

if bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile local-free \
  --mir-json "$FREE_HEAD_POP_MIR" \
  --out "$FREE_HEAD_POP_LLVM_REPORT" \
  2>"$FREE_HEAD_POP_LLVM_STDERR"; then
  echo "[TEST/FAIL] FreeHeadPop vocabulary unexpectedly lowered" >&2
  cat "$FREE_HEAD_POP_LLVM_REPORT" >&2 || true
  exit 1
fi
grep -q '\[llvm/fastmem:missing-verified-free-head-pop-plan\]' \
  "$FREE_HEAD_POP_LLVM_STDERR"

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$FREE_HEAD_POP_PRECONDITION_AST" "$FREE_HEAD_POP_PRECONDITION_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$FREE_HEAD_POP_PRECONDITION_MIR" "$FREE_HEAD_POP_PRECONDITION_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --ast-json "$FREE_HEAD_POP_PRECONDITION_AST" \
  --out "$FREE_HEAD_POP_PRECONDITION_INV"

grep -q '^input_kind=ast_json$' "$FREE_HEAD_POP_PRECONDITION_INV"
grep -q '^fastmem_region_count=1$' "$FREE_HEAD_POP_PRECONDITION_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$FREE_HEAD_POP_PRECONDITION_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$FREE_HEAD_POP_PRECONDITION_INV"
grep -q '^fastmem_memop_field_load_count=2$' "$FREE_HEAD_POP_PRECONDITION_INV"
grep -q '^fastmem_memop_field_store_count=1$' "$FREE_HEAD_POP_PRECONDITION_INV"
grep -q '^fastmem_memop_current_alloc_owner_id_count=1$' "$FREE_HEAD_POP_PRECONDITION_INV"
grep -q '^fastmem_memop_owner_eq_count=1$' "$FREE_HEAD_POP_PRECONDITION_INV"
grep -q '^fastmem_memop_free_head_pop_count=1$' "$FREE_HEAD_POP_PRECONDITION_INV"
grep -q '^fastmem_forbidden_call_count=0$' "$FREE_HEAD_POP_PRECONDITION_INV"
grep -q '^summary=ok$' "$FREE_HEAD_POP_PRECONDITION_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --mir-json "$FREE_HEAD_POP_PRECONDITION_MIR" \
  --out "$FREE_HEAD_POP_PRECONDITION_MIR_INV"

grep -q '^input_kind=mir_json_metadata$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_region_count=1$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_field_load_count=2$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_field_store_count=1$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_current_alloc_owner_id_count=0$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_owner_eq_count=0$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_free_head_pop_count=1$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_verified_mem_access_plan_count=5$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_verified_field_access_count=3$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_verified_table_access_count=1$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_same_owner_fact_count=1$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_list_plan=1$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_pop_plan_count=1$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_nonlowerable_count=0$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_pop_lowerable_count=1$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_access_resolved_count=1$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_block_next_access_resolved_count=1$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_access_plan_incomplete_count=0$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_non_empty_fact_count=1$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_same_owner_required=1$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_same_owner_missing_count=0$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_non_empty_required=1$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_non_empty_missing_count=0$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_remote_owner_rejected_count=1$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"
grep -q '^summary=ok$' "$FREE_HEAD_POP_PRECONDITION_MIR_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FREE_HEAD_POP_PRECONDITION_MIR_INV" \
  --format kv \
  --out "$FREE_HEAD_POP_PRECONDITION_LLVM_CHECK"
grep -q '^summary=ok$' "$FREE_HEAD_POP_PRECONDITION_LLVM_CHECK"
grep -q '^failure_count=0$' "$FREE_HEAD_POP_PRECONDITION_LLVM_CHECK"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile local-free \
  --mir-json "$FREE_HEAD_POP_PRECONDITION_MIR" \
  --out "$FREE_HEAD_POP_PRECONDITION_LLVM_REPORT" \
  2>"$FREE_HEAD_POP_PRECONDITION_LLVM_STDERR"

grep -q '^replacement_front_producer=mir_to_llvm_lowering$' "$FREE_HEAD_POP_PRECONDITION_LLVM_REPORT"
grep -q '^replacement_front_selected_memop_family=local_free$' "$FREE_HEAD_POP_PRECONDITION_LLVM_REPORT"
grep -q '^replacement_front_selected_memop_kinds=FreeHeadPop$' "$FREE_HEAD_POP_PRECONDITION_LLVM_REPORT"
grep -q '^replacement_front_deferred_memop_kinds=LocalFreePush,LocalFreePop,FreeHeadPush,AtomicRemoteHead$' "$FREE_HEAD_POP_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_local_free_producer_pilot=1$' "$FREE_HEAD_POP_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_free_head_pop_plan_count=1$' "$FREE_HEAD_POP_PRECONDITION_LLVM_REPORT"
grep -q '^memop_free_head_pop_lowered_count=1$' "$FREE_HEAD_POP_PRECONDITION_LLVM_REPORT"
grep -q '^memop_free_head_pop_layout_ref_consumed_count=1$' "$FREE_HEAD_POP_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_free_head_access_plan_incomplete_count=0$' "$FREE_HEAD_POP_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_free_head_plain_store_lowered_count=0$' "$FREE_HEAD_POP_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_free_head_pop_lowering_uses_verified_plan=1$' "$FREE_HEAD_POP_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_free_head_pop_lowering_enabled=1$' "$FREE_HEAD_POP_PRECONDITION_LLVM_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$FREE_HEAD_POP_PRECONDITION_LLVM_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' "$FREE_HEAD_POP_PRECONDITION_LLVM_REPORT"
grep -q '^product_activation=0$' "$FREE_HEAD_POP_PRECONDITION_LLVM_REPORT"
grep -q '^global_allocator_claim=0$' "$FREE_HEAD_POP_PRECONDITION_LLVM_REPORT"
grep -q '^winner_claim=0$' "$FREE_HEAD_POP_PRECONDITION_LLVM_REPORT"
grep -q '^summary=ok$' "$FREE_HEAD_POP_PRECONDITION_LLVM_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FREE_HEAD_POP_PRECONDITION_LLVM_REPORT" \
  --format kv \
  --out "$FREE_HEAD_POP_PRECONDITION_LLVM_CHECK"
grep -q '^summary=ok$' "$FREE_HEAD_POP_PRECONDITION_LLVM_CHECK"
grep -q '^failure_count=0$' "$FREE_HEAD_POP_PRECONDITION_LLVM_CHECK"

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$FREE_HEAD_PUSH_AST" "$FREE_HEAD_PUSH_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$FREE_HEAD_PUSH_MIR" "$FREE_HEAD_PUSH_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --ast-json "$FREE_HEAD_PUSH_AST" \
  --out "$FREE_HEAD_PUSH_INV"

grep -q '^input_kind=ast_json$' "$FREE_HEAD_PUSH_INV"
grep -q '^fastmem_region_count=1$' "$FREE_HEAD_PUSH_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$FREE_HEAD_PUSH_INV"
grep -q '^fastmem_memop_table_index_count=0$' "$FREE_HEAD_PUSH_INV"
grep -q '^fastmem_memop_field_load_count=0$' "$FREE_HEAD_PUSH_INV"
grep -q '^fastmem_memop_field_store_count=0$' "$FREE_HEAD_PUSH_INV"
grep -q '^fastmem_memop_free_head_push_count=1$' "$FREE_HEAD_PUSH_INV"
grep -q '^fastmem_memop_free_head_pop_count=0$' "$FREE_HEAD_PUSH_INV"
grep -q '^fastmem_forbidden_call_count=0$' "$FREE_HEAD_PUSH_INV"
grep -q '^summary=ok$' "$FREE_HEAD_PUSH_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --mir-json "$FREE_HEAD_PUSH_MIR" \
  --out "$FREE_HEAD_PUSH_MIR_INV"

grep -q '^input_kind=mir_json_metadata$' "$FREE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_region_count=1$' "$FREE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$FREE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_memop_table_index_count=0$' "$FREE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_memop_field_load_count=0$' "$FREE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_memop_field_store_count=0$' "$FREE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_memop_free_head_push_count=1$' "$FREE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_memop_free_head_pop_count=0$' "$FREE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_verified_mem_access_plan_count=0$' "$FREE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_verified_field_access_count=0$' "$FREE_HEAD_PUSH_MIR_INV"
grep -q '^fastmem_verified_table_access_count=0$' "$FREE_HEAD_PUSH_MIR_INV"
grep -q '^summary=ok$' "$FREE_HEAD_PUSH_MIR_INV"

if bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile local-free \
  --mir-json "$FREE_HEAD_PUSH_MIR" \
  --out "$FREE_HEAD_PUSH_LLVM_REPORT" \
  2>"$FREE_HEAD_PUSH_LLVM_STDERR"; then
  echo "[TEST/FAIL] FreeHeadPush vocabulary unexpectedly lowered" >&2
  cat "$FREE_HEAD_PUSH_LLVM_REPORT" >&2 || true
  exit 1
fi
grep -q '\[llvm/fastmem:missing-verified-free-head-push-plan\]' \
  "$FREE_HEAD_PUSH_LLVM_STDERR"

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$FREE_HEAD_PUSH_PRECONDITION_AST" "$FREE_HEAD_PUSH_PRECONDITION_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$FREE_HEAD_PUSH_PRECONDITION_MIR" "$FREE_HEAD_PUSH_PRECONDITION_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --ast-json "$FREE_HEAD_PUSH_PRECONDITION_AST" \
  --out "$FREE_HEAD_PUSH_PRECONDITION_INV"

grep -q '^input_kind=ast_json$' "$FREE_HEAD_PUSH_PRECONDITION_INV"
grep -q '^fastmem_region_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$FREE_HEAD_PUSH_PRECONDITION_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_INV"
grep -q '^fastmem_memop_field_load_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_INV"
grep -q '^fastmem_memop_field_store_count=0$' "$FREE_HEAD_PUSH_PRECONDITION_INV"
grep -q '^fastmem_memop_current_alloc_owner_id_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_INV"
grep -q '^fastmem_memop_owner_eq_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_INV"
grep -q '^fastmem_memop_free_head_push_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_INV"
grep -q '^fastmem_memop_free_head_pop_count=0$' "$FREE_HEAD_PUSH_PRECONDITION_INV"
grep -q '^fastmem_forbidden_call_count=0$' "$FREE_HEAD_PUSH_PRECONDITION_INV"
grep -q '^summary=ok$' "$FREE_HEAD_PUSH_PRECONDITION_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --mir-json "$FREE_HEAD_PUSH_PRECONDITION_MIR" \
  --out "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"

grep -q '^input_kind=mir_json_metadata$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_region_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_contract_id=PageMapV0$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_table_index_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_field_load_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_field_store_count=0$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_current_alloc_owner_id_count=0$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_owner_eq_count=0$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_free_head_push_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_memop_free_head_pop_count=0$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_verified_mem_access_plan_count=3$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_verified_field_access_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_verified_table_access_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_same_owner_fact_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_block_next_fact_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_list_plan=1$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_push_plan_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_pop_plan_count=0$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_nonlowerable_count=0$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_push_lowerable_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_pop_lowerable_count=0$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_access_resolved_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_block_next_access_resolved_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_access_plan_incomplete_count=0$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_non_empty_fact_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_same_owner_required=1$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_same_owner_missing_count=0$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_non_empty_required=0$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_non_empty_missing_count=0$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_remote_owner_rejected_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^fastmem_free_head_block_next_proof_missing_count=0$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"
grep -q '^summary=ok$' "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FREE_HEAD_PUSH_PRECONDITION_MIR_INV" \
  --format kv \
  --out "$TMPDIR/page_meta_free_head_push_precondition.check.kv"
grep -q '^summary=ok$' "$TMPDIR/page_meta_free_head_push_precondition.check.kv"
grep -q '^failure_count=0$' "$TMPDIR/page_meta_free_head_push_precondition.check.kv"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile local-free \
  --mir-json "$FREE_HEAD_PUSH_PRECONDITION_MIR" \
  --out "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT" \
  2>"$FREE_HEAD_PUSH_PRECONDITION_LLVM_STDERR"

grep -q '^replacement_front_producer=mir_to_llvm_lowering$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^replacement_front_selected_memop_family=local_free$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^replacement_front_selected_memop_kinds=FreeHeadPush$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^replacement_front_deferred_memop_kinds=LocalFreePush,LocalFreePop,FreeHeadPop,AtomicRemoteHead$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_local_free_producer_pilot=1$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_verified_mem_access_plan_count=3$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_field_load_plan_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_field_store_plan_count=0$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_free_head_push_plan_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^memop_table_index_lowered_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^memop_field_load_lowered_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^memop_field_store_lowered_count=0$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^memop_free_head_push_lowered_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^memop_free_head_push_layout_ref_consumed_count=1$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_free_head_access_plan_incomplete_count=0$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_free_head_plain_store_lowered_count=0$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_free_head_push_lowering_uses_verified_plan=1$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^fastmem_free_head_push_lowering_enabled=1$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^product_activation=0$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^global_allocator_claim=0$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^winner_claim=0$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"
grep -q '^summary=ok$' "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FREE_HEAD_PUSH_PRECONDITION_LLVM_REPORT" \
  --format kv \
  --out "$TMPDIR/page_meta_free_head_push_precondition.llvm.check.kv"
grep -q '^summary=ok$' "$TMPDIR/page_meta_free_head_push_precondition.llvm.check.kv"
grep -q '^failure_count=0$' "$TMPDIR/page_meta_free_head_push_precondition.llvm.check.kv"

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$REFILL_THEN_FREE_HEAD_ALLOC_AST" "$REFILL_THEN_FREE_HEAD_ALLOC_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$REFILL_THEN_FREE_HEAD_ALLOC_MIR" "$REFILL_THEN_FREE_HEAD_ALLOC_SRC" >/dev/null

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_AST" "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_MIR" "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_SRC" >/dev/null

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile page-local-alloc-route-cfg \
  --mir-json "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_MIR" \
  --out "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PRODUCER_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile page-local-route-body-join-preflight \
  --mir-json "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_MIR" \
  --out "$PAGE_LOCAL_ROUTE_BODY_JOIN_REPORT"

grep -q '^fastmem_page_local_route_body_join_preflight=1$' \
  "$PAGE_LOCAL_ROUTE_BODY_JOIN_REPORT"
grep -q '^replacement_front_selected_route=page_local_route_body_join_preflight$' \
  "$PAGE_LOCAL_ROUTE_BODY_JOIN_REPORT"
grep -q '^replacement_front_selected_memop_family=page_local_route_body_join$' \
  "$PAGE_LOCAL_ROUTE_BODY_JOIN_REPORT"
grep -q '^replacement_front_selected_memop_kinds=PageLocalRouteBodyJoin$' \
  "$PAGE_LOCAL_ROUTE_BODY_JOIN_REPORT"
grep -q '^replacement_front_next_producer_slice=page_local_route_body_join_producer_pilot$' \
  "$PAGE_LOCAL_ROUTE_BODY_JOIN_REPORT"
grep -q '^page_local_route_body_join_selected=1$' \
  "$PAGE_LOCAL_ROUTE_BODY_JOIN_REPORT"
grep -q '^page_local_route_body_join_open=0$' \
  "$PAGE_LOCAL_ROUTE_BODY_JOIN_REPORT"
grep -q '^page_local_alloc_route_cfg_selected=1$' \
  "$PAGE_LOCAL_ROUTE_BODY_JOIN_REPORT"
grep -q '^page_local_alloc_route_cfg_lowering_enabled=1$' \
  "$PAGE_LOCAL_ROUTE_BODY_JOIN_REPORT"
grep -q '^page_local_free_route_cfg_selected=1$' \
  "$PAGE_LOCAL_ROUTE_BODY_JOIN_REPORT"
grep -q '^page_local_free_route_cfg_lowering_enabled=1$' \
  "$PAGE_LOCAL_ROUTE_BODY_JOIN_REPORT"
grep -q '^fastmem_branch_cfg_source_guard=branch_cfg_open$' \
  "$PAGE_LOCAL_ROUTE_BODY_JOIN_REPORT"
grep -q '^product_activation=0$' "$PAGE_LOCAL_ROUTE_BODY_JOIN_REPORT"
grep -q '^hook_install=0$' "$PAGE_LOCAL_ROUTE_BODY_JOIN_REPORT"
grep -q '^global_allocator_claim=0$' "$PAGE_LOCAL_ROUTE_BODY_JOIN_REPORT"
grep -q '^winner_claim=0$' "$PAGE_LOCAL_ROUTE_BODY_JOIN_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$PAGE_LOCAL_ROUTE_BODY_JOIN_REPORT" \
  --format kv \
  --out "$PAGE_LOCAL_ROUTE_BODY_JOIN_CHECK"

grep -q '^summary=ok$' "$PAGE_LOCAL_ROUTE_BODY_JOIN_CHECK"
grep -q '^failure_count=0$' "$PAGE_LOCAL_ROUTE_BODY_JOIN_CHECK"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile page-local-route-body-join \
  --mir-json "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_MIR" \
  --out "$PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER_REPORT"

grep -q '^fastmem_page_local_route_body_join_producer_pilot=1$' \
  "$PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER_REPORT"
grep -q '^replacement_front_selected_route=page_local_route_body_join_producer_pilot$' \
  "$PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER_REPORT"
grep -q '^replacement_front_selected_memop_family=page_local_route_body_join$' \
  "$PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER_REPORT"
grep -q '^replacement_front_selected_memop_kinds=PageLocalRouteBodyJoinProducer$' \
  "$PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER_REPORT"
grep -q '^replacement_front_next_producer_slice=terminal_ladder_refresh_preflight$' \
  "$PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER_REPORT"
grep -q '^page_local_route_body_join_selected=1$' \
  "$PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER_REPORT"
grep -q '^page_local_route_body_join_open=1$' \
  "$PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER_REPORT"
grep -q '^page_local_alloc_route_cfg_lowering_enabled=1$' \
  "$PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER_REPORT"
grep -q '^page_local_free_route_cfg_lowering_enabled=1$' \
  "$PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER_REPORT"
grep -q '^product_activation=0$' "$PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER_REPORT"
grep -q '^hook_install=0$' "$PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER_REPORT"
grep -q '^global_allocator_claim=0$' "$PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER_REPORT"
grep -q '^winner_claim=0$' "$PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER_REPORT" \
  --format kv \
  --out "$PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER_CHECK"

grep -q '^summary=ok$' "$PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER_CHECK"
grep -q '^failure_count=0$' "$PAGE_LOCAL_ROUTE_BODY_JOIN_PRODUCER_CHECK"

emit_fastmem_producer_report \
  terminal-ladder-refresh-preflight \
  "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_MIR" \
  "$TERMINAL_LADDER_REFRESH_REPORT"

grep -q '^fastmem_terminal_ladder_refresh_preflight=1$' \
  "$TERMINAL_LADDER_REFRESH_REPORT"
grep -q '^replacement_front_selected_route=terminal_ladder_refresh_preflight$' \
  "$TERMINAL_LADDER_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_family=terminal_ladder_refresh$' \
  "$TERMINAL_LADDER_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_kinds=TerminalLadderRefresh$' \
  "$TERMINAL_LADDER_REFRESH_REPORT"
grep -q '^replacement_front_next_producer_slice=tls_backing_transfer_preflight_refresh$' \
  "$TERMINAL_LADDER_REFRESH_REPORT"
grep -q '^page_local_route_body_join_selected=1$' \
  "$TERMINAL_LADDER_REFRESH_REPORT"
grep -q '^page_local_route_body_join_open=1$' \
  "$TERMINAL_LADDER_REFRESH_REPORT"
grep -q '^page_local_alloc_route_cfg_lowering_enabled=1$' \
  "$TERMINAL_LADDER_REFRESH_REPORT"
grep -q '^page_local_free_route_cfg_lowering_enabled=1$' \
  "$TERMINAL_LADDER_REFRESH_REPORT"
grep -q '^terminal_ladder_refresh_selected=1$' \
  "$TERMINAL_LADDER_REFRESH_REPORT"
grep -q '^terminal_ladder_refresh_open=0$' \
  "$TERMINAL_LADDER_REFRESH_REPORT"
grep -q '^tls_backing_transfer_enabled=0$' "$TERMINAL_LADDER_REFRESH_REPORT"
grep -q '^product_activation=0$' "$TERMINAL_LADDER_REFRESH_REPORT"
grep -q '^hook_install=0$' "$TERMINAL_LADDER_REFRESH_REPORT"
grep -q '^global_allocator_claim=0$' "$TERMINAL_LADDER_REFRESH_REPORT"
grep -q '^winner_claim=0$' "$TERMINAL_LADDER_REFRESH_REPORT"

assert_fastmem_report_check_ok \
  "$TERMINAL_LADDER_REFRESH_REPORT" \
  "$TERMINAL_LADDER_REFRESH_CHECK"

emit_fastmem_producer_report \
  tls-backing-transfer-preflight-refresh \
  "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_MIR" \
  "$TLS_BACKING_TRANSFER_REFRESH_REPORT"

grep -q '^fastmem_tls_backing_transfer_preflight_refresh=1$' \
  "$TLS_BACKING_TRANSFER_REFRESH_REPORT"
grep -q '^replacement_front_selected_route=tls_backing_transfer_preflight_refresh$' \
  "$TLS_BACKING_TRANSFER_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_family=tls_backing_transfer$' \
  "$TLS_BACKING_TRANSFER_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_kinds=TlsBackingTransfer$' \
  "$TLS_BACKING_TRANSFER_REFRESH_REPORT"
grep -q '^replacement_front_next_producer_slice=tls_backing_transfer_producer_refresh$' \
  "$TLS_BACKING_TRANSFER_REFRESH_REPORT"
grep -q '^terminal_ladder_refresh_selected=1$' \
  "$TLS_BACKING_TRANSFER_REFRESH_REPORT"
grep -q '^terminal_ladder_refresh_open=1$' \
  "$TLS_BACKING_TRANSFER_REFRESH_REPORT"
grep -q '^page_local_route_body_join_open=1$' \
  "$TLS_BACKING_TRANSFER_REFRESH_REPORT"
grep -q '^page_local_alloc_route_cfg_lowering_enabled=1$' \
  "$TLS_BACKING_TRANSFER_REFRESH_REPORT"
grep -q '^page_local_free_route_cfg_lowering_enabled=1$' \
  "$TLS_BACKING_TRANSFER_REFRESH_REPORT"
grep -q '^tls_backing_transfer_selected=1$' \
  "$TLS_BACKING_TRANSFER_REFRESH_REPORT"
grep -q '^tls_backing_transfer_enabled=0$' "$TLS_BACKING_TRANSFER_REFRESH_REPORT"
grep -q '^product_activation=0$' "$TLS_BACKING_TRANSFER_REFRESH_REPORT"
grep -q '^hook_install=0$' "$TLS_BACKING_TRANSFER_REFRESH_REPORT"
grep -q '^global_allocator_claim=0$' "$TLS_BACKING_TRANSFER_REFRESH_REPORT"
grep -q '^winner_claim=0$' "$TLS_BACKING_TRANSFER_REFRESH_REPORT"

assert_fastmem_report_check_ok \
  "$TLS_BACKING_TRANSFER_REFRESH_REPORT" \
  "$TLS_BACKING_TRANSFER_REFRESH_CHECK"

emit_fastmem_producer_report \
  tls-backing-transfer-producer-refresh \
  "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_MIR" \
  "$TLS_BACKING_TRANSFER_PRODUCER_REFRESH_REPORT"

grep -q '^fastmem_tls_backing_transfer_producer_refresh=1$' \
  "$TLS_BACKING_TRANSFER_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_selected_route=tls_backing_transfer_producer_refresh$' \
  "$TLS_BACKING_TRANSFER_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_family=tls_backing_transfer$' \
  "$TLS_BACKING_TRANSFER_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_kinds=TlsBackingTransfer$' \
  "$TLS_BACKING_TRANSFER_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_next_producer_slice=owner_slot_reuse_preflight_refresh$' \
  "$TLS_BACKING_TRANSFER_PRODUCER_REFRESH_REPORT"
grep -q '^terminal_ladder_refresh_open=1$' \
  "$TLS_BACKING_TRANSFER_PRODUCER_REFRESH_REPORT"
grep -q '^page_local_route_body_join_open=1$' \
  "$TLS_BACKING_TRANSFER_PRODUCER_REFRESH_REPORT"
grep -q '^tls_backing_transfer_selected=1$' \
  "$TLS_BACKING_TRANSFER_PRODUCER_REFRESH_REPORT"
grep -q '^tls_backing_transfer_enabled=1$' \
  "$TLS_BACKING_TRANSFER_PRODUCER_REFRESH_REPORT"
grep -q '^allocator_owner_slot_reuse_enabled=0$' \
  "$TLS_BACKING_TRANSFER_PRODUCER_REFRESH_REPORT"
grep -q '^product_activation=0$' "$TLS_BACKING_TRANSFER_PRODUCER_REFRESH_REPORT"
grep -q '^hook_install=0$' "$TLS_BACKING_TRANSFER_PRODUCER_REFRESH_REPORT"
grep -q '^global_allocator_claim=0$' "$TLS_BACKING_TRANSFER_PRODUCER_REFRESH_REPORT"
grep -q '^winner_claim=0$' "$TLS_BACKING_TRANSFER_PRODUCER_REFRESH_REPORT"

assert_fastmem_report_check_ok \
  "$TLS_BACKING_TRANSFER_PRODUCER_REFRESH_REPORT" \
  "$TLS_BACKING_TRANSFER_PRODUCER_REFRESH_CHECK"

emit_fastmem_producer_report \
  owner-slot-reuse-preflight-refresh \
  "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_MIR" \
  "$OWNER_SLOT_REUSE_REFRESH_REPORT"

grep -q '^fastmem_allocator_owner_slot_reuse_preflight_refresh=1$' \
  "$OWNER_SLOT_REUSE_REFRESH_REPORT"
grep -q '^replacement_front_selected_route=owner_slot_reuse_preflight_refresh$' \
  "$OWNER_SLOT_REUSE_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_family=owner_slot_reuse$' \
  "$OWNER_SLOT_REUSE_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_kinds=OwnerSlotReuse$' \
  "$OWNER_SLOT_REUSE_REFRESH_REPORT"
grep -q '^replacement_front_next_producer_slice=owner_slot_reuse_producer_refresh$' \
  "$OWNER_SLOT_REUSE_REFRESH_REPORT"
grep -q '^terminal_ladder_refresh_open=1$' \
  "$OWNER_SLOT_REUSE_REFRESH_REPORT"
grep -q '^page_local_route_body_join_open=1$' \
  "$OWNER_SLOT_REUSE_REFRESH_REPORT"
grep -q '^tls_backing_transfer_selected=1$' \
  "$OWNER_SLOT_REUSE_REFRESH_REPORT"
grep -q '^tls_backing_transfer_enabled=1$' \
  "$OWNER_SLOT_REUSE_REFRESH_REPORT"
grep -q '^allocator_owner_slot_reuse_selected=1$' \
  "$OWNER_SLOT_REUSE_REFRESH_REPORT"
grep -q '^allocator_owner_slot_reuse_enabled=0$' \
  "$OWNER_SLOT_REUSE_REFRESH_REPORT"
grep -q '^product_activation=0$' "$OWNER_SLOT_REUSE_REFRESH_REPORT"
grep -q '^hook_install=0$' "$OWNER_SLOT_REUSE_REFRESH_REPORT"
grep -q '^global_allocator_claim=0$' "$OWNER_SLOT_REUSE_REFRESH_REPORT"
grep -q '^winner_claim=0$' "$OWNER_SLOT_REUSE_REFRESH_REPORT"

assert_fastmem_report_check_ok \
  "$OWNER_SLOT_REUSE_REFRESH_REPORT" \
  "$OWNER_SLOT_REUSE_REFRESH_CHECK"

emit_fastmem_producer_report \
  owner-slot-reuse-producer-refresh \
  "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_MIR" \
  "$OWNER_SLOT_REUSE_PRODUCER_REFRESH_REPORT"

grep -q '^fastmem_allocator_owner_slot_reuse_producer_refresh=1$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_selected_route=owner_slot_reuse_producer_refresh$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_family=owner_slot_reuse$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_kinds=OwnerSlotReuse$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_next_producer_slice=abandoned_reclaim_preflight_refresh$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REFRESH_REPORT"
grep -q '^terminal_ladder_refresh_open=1$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REFRESH_REPORT"
grep -q '^page_local_route_body_join_open=1$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REFRESH_REPORT"
grep -q '^tls_backing_transfer_enabled=1$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REFRESH_REPORT"
grep -q '^allocator_owner_slot_reuse_selected=1$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REFRESH_REPORT"
grep -q '^allocator_owner_slot_reuse_enabled=1$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REFRESH_REPORT"
grep -q '^allocator_owner_generation_bump_count=1$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REFRESH_REPORT"
grep -q '^allocator_owner_reuse_without_generation_bump_count=0$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REFRESH_REPORT"
grep -q '^abandoned_reclaim_enabled=0$' \
  "$OWNER_SLOT_REUSE_PRODUCER_REFRESH_REPORT"
grep -q '^product_activation=0$' "$OWNER_SLOT_REUSE_PRODUCER_REFRESH_REPORT"
grep -q '^hook_install=0$' "$OWNER_SLOT_REUSE_PRODUCER_REFRESH_REPORT"
grep -q '^global_allocator_claim=0$' "$OWNER_SLOT_REUSE_PRODUCER_REFRESH_REPORT"
grep -q '^winner_claim=0$' "$OWNER_SLOT_REUSE_PRODUCER_REFRESH_REPORT"

assert_fastmem_report_check_ok \
  "$OWNER_SLOT_REUSE_PRODUCER_REFRESH_REPORT" \
  "$OWNER_SLOT_REUSE_PRODUCER_REFRESH_CHECK"

emit_fastmem_producer_report \
  abandoned-reclaim-preflight-refresh \
  "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_MIR" \
  "$ABANDONED_RECLAIM_REFRESH_REPORT"

grep -q '^fastmem_abandoned_reclaim_preflight_refresh=1$' \
  "$ABANDONED_RECLAIM_REFRESH_REPORT"
grep -q '^replacement_front_selected_route=abandoned_reclaim_preflight_refresh$' \
  "$ABANDONED_RECLAIM_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_family=abandoned_reclaim$' \
  "$ABANDONED_RECLAIM_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_kinds=AbandonedReclaim$' \
  "$ABANDONED_RECLAIM_REFRESH_REPORT"
grep -q '^replacement_front_next_producer_slice=abandoned_reclaim_producer_refresh$' \
  "$ABANDONED_RECLAIM_REFRESH_REPORT"
grep -q '^terminal_ladder_refresh_open=1$' \
  "$ABANDONED_RECLAIM_REFRESH_REPORT"
grep -q '^page_local_route_body_join_open=1$' \
  "$ABANDONED_RECLAIM_REFRESH_REPORT"
grep -q '^tls_backing_transfer_enabled=1$' \
  "$ABANDONED_RECLAIM_REFRESH_REPORT"
grep -q '^allocator_owner_slot_reuse_enabled=1$' \
  "$ABANDONED_RECLAIM_REFRESH_REPORT"
grep -q '^allocator_owner_generation_bump_count=1$' \
  "$ABANDONED_RECLAIM_REFRESH_REPORT"
grep -q '^abandoned_reclaim_selected=1$' \
  "$ABANDONED_RECLAIM_REFRESH_REPORT"
grep -q '^abandoned_reclaim_enabled=0$' \
  "$ABANDONED_RECLAIM_REFRESH_REPORT"
grep -q '^page_reclaimed_with_remote_candidates=0$' \
  "$ABANDONED_RECLAIM_REFRESH_REPORT"
grep -q '^product_activation=0$' "$ABANDONED_RECLAIM_REFRESH_REPORT"
grep -q '^hook_install=0$' "$ABANDONED_RECLAIM_REFRESH_REPORT"
grep -q '^global_allocator_claim=0$' "$ABANDONED_RECLAIM_REFRESH_REPORT"
grep -q '^winner_claim=0$' "$ABANDONED_RECLAIM_REFRESH_REPORT"

assert_fastmem_report_check_ok \
  "$ABANDONED_RECLAIM_REFRESH_REPORT" \
  "$ABANDONED_RECLAIM_REFRESH_CHECK"

emit_fastmem_producer_report \
  abandoned-reclaim-producer-refresh \
  "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_MIR" \
  "$ABANDONED_RECLAIM_PRODUCER_REFRESH_REPORT"

grep -q '^fastmem_abandoned_reclaim_producer_refresh=1$' \
  "$ABANDONED_RECLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_selected_route=abandoned_reclaim_producer_refresh$' \
  "$ABANDONED_RECLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_family=abandoned_reclaim$' \
  "$ABANDONED_RECLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_kinds=AbandonedReclaim$' \
  "$ABANDONED_RECLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_next_producer_slice=product_activation_preflight_refresh$' \
  "$ABANDONED_RECLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^terminal_ladder_refresh_open=1$' \
  "$ABANDONED_RECLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^page_local_route_body_join_open=1$' \
  "$ABANDONED_RECLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^tls_backing_transfer_enabled=1$' \
  "$ABANDONED_RECLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^allocator_owner_slot_reuse_enabled=1$' \
  "$ABANDONED_RECLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^allocator_owner_generation_bump_count=1$' \
  "$ABANDONED_RECLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^abandoned_reclaim_selected=1$' \
  "$ABANDONED_RECLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^abandoned_reclaim_enabled=1$' \
  "$ABANDONED_RECLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^page_reclaimed_with_remote_candidates=0$' \
  "$ABANDONED_RECLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^product_activation=0$' "$ABANDONED_RECLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^hook_install=0$' "$ABANDONED_RECLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^global_allocator_claim=0$' "$ABANDONED_RECLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^winner_claim=0$' "$ABANDONED_RECLAIM_PRODUCER_REFRESH_REPORT"

assert_fastmem_report_check_ok \
  "$ABANDONED_RECLAIM_PRODUCER_REFRESH_REPORT" \
  "$ABANDONED_RECLAIM_PRODUCER_REFRESH_CHECK"

emit_fastmem_producer_report \
  product-activation-preflight-refresh \
  "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_MIR" \
  "$PRODUCT_ACTIVATION_REFRESH_REPORT"

grep -q '^fastmem_product_activation_preflight_refresh=1$' \
  "$PRODUCT_ACTIVATION_REFRESH_REPORT"
grep -q '^replacement_front_selected_route=product_activation_preflight_refresh$' \
  "$PRODUCT_ACTIVATION_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_family=product_activation$' \
  "$PRODUCT_ACTIVATION_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_kinds=ProductActivation$' \
  "$PRODUCT_ACTIVATION_REFRESH_REPORT"
grep -q '^replacement_front_next_producer_slice=product_activation_producer_refresh$' \
  "$PRODUCT_ACTIVATION_REFRESH_REPORT"
grep -q '^terminal_ladder_refresh_open=1$' \
  "$PRODUCT_ACTIVATION_REFRESH_REPORT"
grep -q '^page_local_route_body_join_open=1$' \
  "$PRODUCT_ACTIVATION_REFRESH_REPORT"
grep -q '^tls_backing_transfer_enabled=1$' \
  "$PRODUCT_ACTIVATION_REFRESH_REPORT"
grep -q '^allocator_owner_slot_reuse_enabled=1$' \
  "$PRODUCT_ACTIVATION_REFRESH_REPORT"
grep -q '^abandoned_reclaim_enabled=1$' \
  "$PRODUCT_ACTIVATION_REFRESH_REPORT"
grep -q '^product_activation_selected=1$' \
  "$PRODUCT_ACTIVATION_REFRESH_REPORT"
grep -q '^product_activation=0$' "$PRODUCT_ACTIVATION_REFRESH_REPORT"
grep -q '^hook_install=0$' "$PRODUCT_ACTIVATION_REFRESH_REPORT"
grep -q '^global_allocator_claim=0$' "$PRODUCT_ACTIVATION_REFRESH_REPORT"
grep -q '^winner_claim=0$' "$PRODUCT_ACTIVATION_REFRESH_REPORT"

assert_fastmem_report_check_ok \
  "$PRODUCT_ACTIVATION_REFRESH_REPORT" \
  "$PRODUCT_ACTIVATION_REFRESH_CHECK"

emit_fastmem_producer_report \
  product-activation-producer-refresh \
  "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_MIR" \
  "$PRODUCT_ACTIVATION_PRODUCER_REFRESH_REPORT"

grep -q '^fastmem_product_activation_producer_refresh=1$' \
  "$PRODUCT_ACTIVATION_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_selected_route=product_activation_producer_refresh$' \
  "$PRODUCT_ACTIVATION_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_family=product_activation$' \
  "$PRODUCT_ACTIVATION_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_kinds=ProductActivation$' \
  "$PRODUCT_ACTIVATION_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_next_producer_slice=hook_install_preflight_refresh$' \
  "$PRODUCT_ACTIVATION_PRODUCER_REFRESH_REPORT"
grep -q '^terminal_ladder_refresh_open=1$' \
  "$PRODUCT_ACTIVATION_PRODUCER_REFRESH_REPORT"
grep -q '^page_local_route_body_join_open=1$' \
  "$PRODUCT_ACTIVATION_PRODUCER_REFRESH_REPORT"
grep -q '^tls_backing_transfer_enabled=1$' \
  "$PRODUCT_ACTIVATION_PRODUCER_REFRESH_REPORT"
grep -q '^allocator_owner_slot_reuse_enabled=1$' \
  "$PRODUCT_ACTIVATION_PRODUCER_REFRESH_REPORT"
grep -q '^abandoned_reclaim_enabled=1$' \
  "$PRODUCT_ACTIVATION_PRODUCER_REFRESH_REPORT"
grep -q '^product_activation_selected=1$' \
  "$PRODUCT_ACTIVATION_PRODUCER_REFRESH_REPORT"
grep -q '^product_activation=1$' "$PRODUCT_ACTIVATION_PRODUCER_REFRESH_REPORT"
grep -q '^hook_install=0$' "$PRODUCT_ACTIVATION_PRODUCER_REFRESH_REPORT"
grep -q '^global_allocator_claim=0$' "$PRODUCT_ACTIVATION_PRODUCER_REFRESH_REPORT"
grep -q '^winner_claim=0$' "$PRODUCT_ACTIVATION_PRODUCER_REFRESH_REPORT"

assert_fastmem_report_check_ok \
  "$PRODUCT_ACTIVATION_PRODUCER_REFRESH_REPORT" \
  "$PRODUCT_ACTIVATION_PRODUCER_REFRESH_CHECK"

emit_fastmem_producer_report \
  hook-install-preflight-refresh \
  "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_MIR" \
  "$HOOK_INSTALL_REFRESH_REPORT"

grep -q '^fastmem_hook_install_preflight_refresh=1$' \
  "$HOOK_INSTALL_REFRESH_REPORT"
grep -q '^replacement_front_selected_route=hook_install_preflight_refresh$' \
  "$HOOK_INSTALL_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_family=hook_install$' \
  "$HOOK_INSTALL_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_kinds=HookInstall$' \
  "$HOOK_INSTALL_REFRESH_REPORT"
grep -q '^replacement_front_next_producer_slice=hook_install_producer_refresh$' \
  "$HOOK_INSTALL_REFRESH_REPORT"
grep -q '^terminal_ladder_refresh_open=1$' \
  "$HOOK_INSTALL_REFRESH_REPORT"
grep -q '^page_local_route_body_join_open=1$' \
  "$HOOK_INSTALL_REFRESH_REPORT"
grep -q '^product_activation_selected=1$' \
  "$HOOK_INSTALL_REFRESH_REPORT"
grep -q '^product_activation=1$' "$HOOK_INSTALL_REFRESH_REPORT"
grep -q '^hook_install_selected=1$' "$HOOK_INSTALL_REFRESH_REPORT"
grep -q '^hook_install=0$' "$HOOK_INSTALL_REFRESH_REPORT"
grep -q '^global_allocator_claim=0$' "$HOOK_INSTALL_REFRESH_REPORT"
grep -q '^winner_claim=0$' "$HOOK_INSTALL_REFRESH_REPORT"

assert_fastmem_report_check_ok \
  "$HOOK_INSTALL_REFRESH_REPORT" \
  "$HOOK_INSTALL_REFRESH_CHECK"

emit_fastmem_producer_report \
  hook-install-producer-refresh \
  "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_MIR" \
  "$HOOK_INSTALL_PRODUCER_REFRESH_REPORT"

grep -q '^fastmem_hook_install_producer_refresh=1$' \
  "$HOOK_INSTALL_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_selected_route=hook_install_producer_refresh$' \
  "$HOOK_INSTALL_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_family=hook_install$' \
  "$HOOK_INSTALL_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_kinds=HookInstall$' \
  "$HOOK_INSTALL_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_next_producer_slice=global_allocator_claim_preflight_refresh$' \
  "$HOOK_INSTALL_PRODUCER_REFRESH_REPORT"
grep -q '^terminal_ladder_refresh_open=1$' \
  "$HOOK_INSTALL_PRODUCER_REFRESH_REPORT"
grep -q '^page_local_route_body_join_open=1$' \
  "$HOOK_INSTALL_PRODUCER_REFRESH_REPORT"
grep -q '^product_activation_selected=1$' \
  "$HOOK_INSTALL_PRODUCER_REFRESH_REPORT"
grep -q '^product_activation=1$' "$HOOK_INSTALL_PRODUCER_REFRESH_REPORT"
grep -q '^hook_install_selected=1$' "$HOOK_INSTALL_PRODUCER_REFRESH_REPORT"
grep -q '^hook_install=1$' "$HOOK_INSTALL_PRODUCER_REFRESH_REPORT"
grep -q '^hook_installed=0$' "$HOOK_INSTALL_PRODUCER_REFRESH_REPORT"
grep -q '^global_allocator_claim=0$' "$HOOK_INSTALL_PRODUCER_REFRESH_REPORT"
grep -q '^winner_claim=0$' "$HOOK_INSTALL_PRODUCER_REFRESH_REPORT"

assert_fastmem_report_check_ok \
  "$HOOK_INSTALL_PRODUCER_REFRESH_REPORT" \
  "$HOOK_INSTALL_PRODUCER_REFRESH_CHECK"

emit_fastmem_producer_report \
  global-allocator-claim-preflight-refresh \
  "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_MIR" \
  "$GLOBAL_ALLOCATOR_CLAIM_REFRESH_REPORT"

grep -q '^fastmem_global_allocator_claim_preflight_refresh=1$' \
  "$GLOBAL_ALLOCATOR_CLAIM_REFRESH_REPORT"
grep -q '^replacement_front_selected_route=global_allocator_claim_preflight_refresh$' \
  "$GLOBAL_ALLOCATOR_CLAIM_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_family=global_allocator_claim$' \
  "$GLOBAL_ALLOCATOR_CLAIM_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_kinds=GlobalAllocatorClaim$' \
  "$GLOBAL_ALLOCATOR_CLAIM_REFRESH_REPORT"
grep -q '^replacement_front_next_producer_slice=global_allocator_claim_producer_refresh$' \
  "$GLOBAL_ALLOCATOR_CLAIM_REFRESH_REPORT"
grep -q '^terminal_ladder_refresh_open=1$' \
  "$GLOBAL_ALLOCATOR_CLAIM_REFRESH_REPORT"
grep -q '^page_local_route_body_join_open=1$' \
  "$GLOBAL_ALLOCATOR_CLAIM_REFRESH_REPORT"
grep -q '^product_activation=1$' "$GLOBAL_ALLOCATOR_CLAIM_REFRESH_REPORT"
grep -q '^hook_install_selected=1$' "$GLOBAL_ALLOCATOR_CLAIM_REFRESH_REPORT"
grep -q '^hook_install=1$' "$GLOBAL_ALLOCATOR_CLAIM_REFRESH_REPORT"
grep -q '^hook_installed=0$' "$GLOBAL_ALLOCATOR_CLAIM_REFRESH_REPORT"
grep -q '^global_allocator_claim_selected=1$' "$GLOBAL_ALLOCATOR_CLAIM_REFRESH_REPORT"
grep -q '^global_allocator_claim=0$' "$GLOBAL_ALLOCATOR_CLAIM_REFRESH_REPORT"
grep -q '^global_allocator_product_claim=0$' "$GLOBAL_ALLOCATOR_CLAIM_REFRESH_REPORT"
grep -q '^winner_claim=0$' "$GLOBAL_ALLOCATOR_CLAIM_REFRESH_REPORT"

assert_fastmem_report_check_ok \
  "$GLOBAL_ALLOCATOR_CLAIM_REFRESH_REPORT" \
  "$GLOBAL_ALLOCATOR_CLAIM_REFRESH_CHECK"

emit_fastmem_producer_report \
  global-allocator-claim-producer-refresh \
  "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_MIR" \
  "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_REPORT"

grep -q '^fastmem_global_allocator_claim_producer_refresh=1$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_selected_route=global_allocator_claim_producer_refresh$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_family=global_allocator_claim$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_kinds=GlobalAllocatorClaim$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_next_producer_slice=winner_claim_preflight_refresh$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^terminal_ladder_refresh_open=1$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^page_local_route_body_join_open=1$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^product_activation=1$' "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^hook_install=1$' "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^hook_installed=0$' "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^global_allocator_claim_selected=1$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^global_allocator_claim=1$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^global_allocator_product_claim=0$' \
  "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^winner_claim=0$' "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_REPORT"

assert_fastmem_report_check_ok \
  "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_REPORT" \
  "$GLOBAL_ALLOCATOR_CLAIM_PRODUCER_REFRESH_CHECK"

emit_fastmem_producer_report \
  winner-claim-preflight-refresh \
  "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_MIR" \
  "$WINNER_CLAIM_REFRESH_REPORT"

grep -q '^fastmem_winner_claim_preflight_refresh=1$' \
  "$WINNER_CLAIM_REFRESH_REPORT"
grep -q '^replacement_front_selected_route=winner_claim_preflight_refresh$' \
  "$WINNER_CLAIM_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_family=winner_claim$' \
  "$WINNER_CLAIM_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_kinds=WinnerClaim$' \
  "$WINNER_CLAIM_REFRESH_REPORT"
grep -q '^replacement_front_next_producer_slice=winner_claim_producer_refresh$' \
  "$WINNER_CLAIM_REFRESH_REPORT"
grep -q '^terminal_ladder_refresh_open=1$' "$WINNER_CLAIM_REFRESH_REPORT"
grep -q '^page_local_route_body_join_open=1$' "$WINNER_CLAIM_REFRESH_REPORT"
grep -q '^product_activation=1$' "$WINNER_CLAIM_REFRESH_REPORT"
grep -q '^hook_install=1$' "$WINNER_CLAIM_REFRESH_REPORT"
grep -q '^global_allocator_claim=1$' "$WINNER_CLAIM_REFRESH_REPORT"
grep -q '^global_allocator_product_claim=0$' "$WINNER_CLAIM_REFRESH_REPORT"
grep -q '^winner_claim_selected=1$' "$WINNER_CLAIM_REFRESH_REPORT"
grep -q '^winner_claim=0$' "$WINNER_CLAIM_REFRESH_REPORT"

assert_fastmem_report_check_ok \
  "$WINNER_CLAIM_REFRESH_REPORT" \
  "$WINNER_CLAIM_REFRESH_CHECK"

emit_fastmem_producer_report \
  winner-claim-producer-refresh \
  "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_MIR" \
  "$WINNER_CLAIM_PRODUCER_REFRESH_REPORT"

grep -q '^fastmem_winner_claim_producer_refresh=1$' \
  "$WINNER_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_selected_route=winner_claim_producer_refresh$' \
  "$WINNER_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_family=winner_claim$' \
  "$WINNER_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_selected_memop_kinds=WinnerClaim$' \
  "$WINNER_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_next_producer_slice=complete$' \
  "$WINNER_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^replacement_front_deferred_memop_kinds=none$' \
  "$WINNER_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^terminal_ladder_refresh_open=1$' "$WINNER_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^page_local_route_body_join_open=1$' "$WINNER_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^product_activation=1$' "$WINNER_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^hook_install=1$' "$WINNER_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^global_allocator_claim=1$' "$WINNER_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^global_allocator_product_claim=0$' \
  "$WINNER_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^winner_claim_selected=1$' "$WINNER_CLAIM_PRODUCER_REFRESH_REPORT"
grep -q '^winner_claim=1$' "$WINNER_CLAIM_PRODUCER_REFRESH_REPORT"

assert_fastmem_report_check_ok \
  "$WINNER_CLAIM_PRODUCER_REFRESH_REPORT" \
  "$WINNER_CLAIM_PRODUCER_REFRESH_CHECK"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile winner-claim-preflight \
  --mir-json "$REFILL_THEN_FREE_HEAD_ALLOC_MIR" \
  --out "$WINNER_CLAIM_PREFLIGHT_REPORT"

grep -q '^replacement_front_producer=mir_to_llvm_lowering$' "$WINNER_CLAIM_PREFLIGHT_REPORT"
grep -q '^fastmem_winner_claim_preflight=1$' "$WINNER_CLAIM_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_route=winner_claim_preflight$' "$WINNER_CLAIM_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_family=winner_claim$' "$WINNER_CLAIM_PREFLIGHT_REPORT"
grep -q '^replacement_front_selected_memop_kinds=WinnerClaim$' "$WINNER_CLAIM_PREFLIGHT_REPORT"
grep -q '^replacement_front_next_producer_slice=winner_claim_producer_pilot$' "$WINNER_CLAIM_PREFLIGHT_REPORT"
grep -q '^replacement_front_deferred_memop_kinds=WinnerClaimProducer$' "$WINNER_CLAIM_PREFLIGHT_REPORT"
grep -q '^winner_claim_selected=1$' "$WINNER_CLAIM_PREFLIGHT_REPORT"
grep -q '^global_allocator_claim_selected=1$' "$WINNER_CLAIM_PREFLIGHT_REPORT"
grep -q '^global_allocator_claim=1$' "$WINNER_CLAIM_PREFLIGHT_REPORT"
grep -q '^hook_install=1$' "$WINNER_CLAIM_PREFLIGHT_REPORT"
grep -q '^product_activation=1$' "$WINNER_CLAIM_PREFLIGHT_REPORT"
grep -q '^winner_claim=0$' "$WINNER_CLAIM_PREFLIGHT_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$WINNER_CLAIM_PREFLIGHT_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' "$WINNER_CLAIM_PREFLIGHT_REPORT"
grep -q '^summary=ok$' "$WINNER_CLAIM_PREFLIGHT_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$WINNER_CLAIM_PREFLIGHT_REPORT" \
  --format kv \
  --out "$WINNER_CLAIM_PREFLIGHT_CHECK"
grep -q '^summary=ok$' "$WINNER_CLAIM_PREFLIGHT_CHECK"
grep -q '^failure_count=0$' "$WINNER_CLAIM_PREFLIGHT_CHECK"

bash "$ROOT/tools/hako_check.sh" fastmem-mir-to-llvm-producer-report \
  --profile winner-claim-producer-pilot \
  --mir-json "$REFILL_THEN_FREE_HEAD_ALLOC_MIR" \
  --out "$WINNER_CLAIM_PRODUCER_REPORT"

grep -q '^replacement_front_producer=mir_to_llvm_lowering$' "$WINNER_CLAIM_PRODUCER_REPORT"
grep -q '^fastmem_winner_claim_producer_pilot=1$' "$WINNER_CLAIM_PRODUCER_REPORT"
grep -q '^replacement_front_selected_route=winner_claim_producer_pilot$' "$WINNER_CLAIM_PRODUCER_REPORT"
grep -q '^replacement_front_selected_memop_family=winner_claim$' "$WINNER_CLAIM_PRODUCER_REPORT"
grep -q '^replacement_front_selected_memop_kinds=WinnerClaim$' "$WINNER_CLAIM_PRODUCER_REPORT"
grep -q '^replacement_front_next_producer_slice=complete$' "$WINNER_CLAIM_PRODUCER_REPORT"
grep -q '^replacement_front_deferred_memop_kinds=none$' "$WINNER_CLAIM_PRODUCER_REPORT"
grep -q '^winner_claim_selected=1$' "$WINNER_CLAIM_PRODUCER_REPORT"
grep -q '^global_allocator_claim_selected=1$' "$WINNER_CLAIM_PRODUCER_REPORT"
grep -q '^global_allocator_claim=1$' "$WINNER_CLAIM_PRODUCER_REPORT"
grep -q '^hook_install=1$' "$WINNER_CLAIM_PRODUCER_REPORT"
grep -q '^product_activation=1$' "$WINNER_CLAIM_PRODUCER_REPORT"
grep -q '^winner_claim=1$' "$WINNER_CLAIM_PRODUCER_REPORT"
grep -q '^type_abi_hot_lookup_count=0$' "$WINNER_CLAIM_PRODUCER_REPORT"
grep -q '^provider_abi_hot_dispatch_count=0$' "$WINNER_CLAIM_PRODUCER_REPORT"
grep -q '^summary=ok$' "$WINNER_CLAIM_PRODUCER_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$WINNER_CLAIM_PRODUCER_REPORT" \
  --format kv \
  --out "$WINNER_CLAIM_PRODUCER_CHECK"
grep -q '^summary=ok$' "$WINNER_CLAIM_PRODUCER_CHECK"
grep -q '^failure_count=0$' "$WINNER_CLAIM_PRODUCER_CHECK"

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
