#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-242-ROLLBACK-SELECTED-PAGE-QUEUE-SAME-BLOCK-GET-SET.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-241-SELECTED-PAGE-QUEUE-SAME-BLOCK-GET-SET-MEASUREMENT.md"
LOWER="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_same_module_body_emit.inc"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TMP_DIR="$(mktemp -d /tmp/hakorune_row242_page_queue_rollback.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR="$TMP_DIR/app.mir.json"
EXE="$TMP_DIR/app.exe"
OUT="$TMP_DIR/app.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row242-page-queue-rollback] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_contains() {
  local file="$1"
  local expected="$2"
  if ! grep -q "$expected" "$file"; then
    echo "[row242-page-queue-rollback] missing content in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

reject_contains() {
  local file="$1"
  local unexpected="$2"
  if grep -q "$unexpected" "$file"; then
    echo "[row242-page-queue-rollback] unexpected content in ${file#$ROOT_DIR/}: $unexpected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=rollback-selected-page-queue-same-block-get-set-v0"
require_line "$DOC" "rollback_reason=selected_page_queue_get_set_keeper_no_effect"
require_line "$DOC" "facade_fusion_preserved=1"
require_line "$DOC" "page_model_acquire_fusion_preserved=1"
require_line "$DOC" "page_queue_fusion_target_removed=1"
require_line "$DOC" "rmw_plan_capacity=16"
require_line "$DOC" "semantic_proof_summary=ok"
require_line "$DOC" "summary=ok"

require_contains "$LOWER" "same_module_function_name_is_selected_facade_get_set_fusion_target"
require_contains "$LOWER" "HakoAllocPageModel.acquire_usize/1"
require_contains "$LOWER" "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_contains "$LOWER" "struct SameModuleTypedFieldRmwFusionPlan typed_field_rmw_plans\\[16\\]"
reject_contains "$LOWER" "same_module_function_name_is_selected_page_queue_get_set_fusion_target"
reject_contains "$LOWER" "HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3"
reject_contains "$LOWER" "struct SameModuleTypedFieldRmwFusionPlan typed_field_rmw_plans\\[32\\]"

bash "$ROOT_DIR/tools/build_hako_llvmc_ffi.sh" >/dev/null
cargo build --release -p nyash_kernel >/dev/null

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" --backend mir --emit-mir-json "$MIR" "$APP" >/dev/null
python3 "$ROOT_DIR/tools/checks/pure_first_route_preflight.py" "$MIR" >/dev/null

HAKO_TYPED_OBJECT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1 \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/tools/selfhost/selfhost_build.sh" --mir-in "$MIR" --exe "$EXE" >/dev/null

HAKO_ARRAY_SLOT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1 \
NYASH_DISABLE_PLUGINS=1 \
  "$EXE" >"$OUT"

require_contains "$OUT" '^summary=ok$'

cat <<REPORT
output_contract=rollback-selected-page-queue-same-block-get-set-v0
input_contract=selected-page-queue-same-block-get-set-measurement-v0
rollback_reason=selected_page_queue_get_set_keeper_no_effect
facade_fusion_preserved=1
page_model_acquire_fusion_preserved=1
page_queue_fusion_target_removed=1
rmw_plan_capacity=16
semantic_proof_summary=ok
single_thread_backend_smoke=ok
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT
