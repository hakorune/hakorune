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

run_fastmem_route_cfg_manifest_seed() {
  python3 "$ROOT/tools/hako_check/fastmem_source_manifest_runner.py" \
    --manifest "$ROOT/tools/hako_check/manifests/fastmem_route_cfg_smoke.toml"
}


run_fastmem_source_manifest_seed
run_fastmem_route_cfg_manifest_seed

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

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$REFILL_THEN_FREE_HEAD_ALLOC_AST" "$REFILL_THEN_FREE_HEAD_ALLOC_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$REFILL_THEN_FREE_HEAD_ALLOC_MIR" "$REFILL_THEN_FREE_HEAD_ALLOC_SRC" >/dev/null

NYASH_FEATURES="$FEATURES" "$BIN" --emit-ast-json "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_AST" "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_SRC" >/dev/null
NYASH_FEATURES="$FEATURES" "$BIN" --backend mir --emit-mir-json "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_MIR" "$PAGE_LOCAL_ALLOC_ROUTE_CFG_PREFLIGHT_SRC" >/dev/null

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
