#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-240-SELECTED-PAGE-QUEUE-SAME-BLOCK-GET-SET-KEEPER.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-239-SELECTED-PAGE-QUEUE-SAME-BLOCK-GET-SET-GUARD-SURFACE.md"
LOWER="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_same_module_body_emit.inc"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TMP_DIR="$(mktemp -d /tmp/hakorune_row240_page_queue_keeper.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR="$TMP_DIR/app.mir.json"
EXE="$TMP_DIR/app.exe"
OUT="$TMP_DIR/app.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row240-page-queue-keeper] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_contains() {
  local file="$1"
  local expected="$2"
  if ! grep -q "$expected" "$file"; then
    echo "[row240-page-queue-keeper] missing content in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=selected-page-queue-same-block-get-set-keeper-v0"
require_line "$DOC" "input_contract=selected-page-queue-same-block-get-set-guard-surface-v0"
require_line "$DOC" "implementation_owner=c_abi_same_module_typed_field_rmw_fusion"
require_line "$DOC" "candidate_count=21"
require_line "$DOC" "candidate_usize_count=21"
require_line "$DOC" "fused_runtime_symbol=nyash.object.exact_slot_rmw_add_u64_hiii"
require_line "$DOC" "status_continue_label_contract=exact_status_continue"
require_line "$DOC" "planned_erased_get_set_helper_calls=42"
require_line "$DOC" "planned_added_fused_helper_calls=21"
require_line "$DOC" "planned_net_helper_call_delta=21"
require_line "$DOC" "rmw_plan_capacity=32"
require_line "$DOC" "runtime_storage_owner_preserved=1"
require_line "$DOC" "helper_free_direct_op_rejected=1"
require_line "$DOC" "generic_residence_open=0"
require_line "$DOC" "source_rewrite=0"
require_line "$DOC" "by_name_hako_alloc_special_case=0"
require_line "$DOC" "summary=ok"

require_contains "$LOWER" "same_module_function_name_is_selected_page_queue_get_set_fusion_target"
require_contains "$LOWER" "HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3"
require_contains "$LOWER" "HakoAllocObjectLifecyclePageQueue.addPage/1"
require_contains "$LOWER" "HakoAllocObjectLifecyclePageQueue.selectPage/0"
require_contains "$LOWER" "HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0"
require_contains "$LOWER" "struct SameModuleTypedFieldRmwFusionPlan typed_field_rmw_plans\\[32\\]"
require_contains "$LOWER" "nyash.object.exact_slot_rmw_add_u64_hiii"
require_contains "$LOWER" "exact_status_continue_%lld_%zu"

bash "$ROOT_DIR/tools/build_hako_llvmc_ffi.sh" >/dev/null
cargo build --release -p nyash_kernel >/dev/null

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" --backend mir --emit-mir-json "$MIR" "$APP" >/dev/null
python3 "$ROOT_DIR/tools/checks/pure_first_route_preflight.py" "$MIR" >/dev/null

HAKO_TYPED_OBJECT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1 \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/tools/selfhost/selfhost_build.sh" --mir-in "$MIR" --exe "$EXE" >/dev/null

fused_symbol_count="$(strings "$EXE" | grep -c 'nyash.object.exact_slot_rmw_add_u64_hiii' || true)"
if [ "$fused_symbol_count" -lt 1 ]; then
  echo "[row240-page-queue-keeper] exact-EXE does not reference fused runtime symbol" >&2
  exit 1
fi

HAKO_ARRAY_SLOT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1 \
NYASH_DISABLE_PLUGINS=1 \
  "$EXE" >"$OUT"

require_contains "$OUT" '^summary=ok$'

cat <<REPORT
output_contract=selected-page-queue-same-block-get-set-keeper-v0
input_contract=selected-page-queue-same-block-get-set-guard-surface-v0
implementation_owner=c_abi_same_module_typed_field_rmw_fusion
target_family=page_queue_helpers
candidate_count=21
candidate_usize_count=21
fused_runtime_symbol=nyash.object.exact_slot_rmw_add_u64_hiii
rmw_plan_capacity=32
exact_exe_fused_symbol_count=${fused_symbol_count}
semantic_proof_summary=ok
single_thread_backend_smoke=ok
runtime_storage_owner_preserved=1
helper_free_direct_op_rejected=1
generic_residence_open=0
source_rewrite=0
by_name_hako_alloc_special_case=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT
