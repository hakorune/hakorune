#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-258-RESULT-CAPSULE-RESET-FIELD-BATCHING-IMPLEMENTATION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-257-RESULT-CAPSULE-RESET-FIELD-BATCHING-GUARD-SURFACE.md"
LOWER="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_same_module_body_emit.inc"
PRESCAN="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc"
RUNTIME="$ROOT_DIR/crates/nyash_kernel/src/exports/typed_object.rs"
STORE="$ROOT_DIR/crates/nyash_kernel/src/exports/typed_object_store.rs"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TMP_DIR="$(mktemp -d /tmp/hakorune_row258_result_capsule_reset.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR="$TMP_DIR/app.mir.json"
EXE="$TMP_DIR/app.exe"
OUT="$TMP_DIR/app.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row258-result-capsule-reset-impl] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_contains() {
  local file="$1"
  local expected="$2"
  if ! grep -q "$expected" "$file"; then
    echo "[row258-result-capsule-reset-impl] missing content in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=result-capsule-reset-field-batching-implementation-v0"
require_line "$DOC" "input_contract=result-capsule-reset-field-batching-guard-surface-v0"
require_line "$DOC" "implementation_owner=c_abi_same_module_result_capsule_reset_batching"
require_line "$DOC" "runtime_helper_symbol=nyash.object.exact_slot_set4_i64_hiiiii"
require_line "$DOC" "runtime_helper_exported=1"
require_line "$DOC" "same_module_emit_selected_method_count=2"
require_line "$DOC" "same_module_emit_target_0=HakoAllocObjectLifecycleAllocResult.reset/0"
require_line "$DOC" "same_module_emit_target_1=HakoAllocObjectLifecycleReleaseResult.reset/0"
require_line "$DOC" "same_module_emit_target_slots=0,1,2,3"
require_line "$DOC" "same_module_emit_target_values=-1,-1,0,0"
require_line "$DOC" "semantic_proof_summary=ok"
require_line "$DOC" "planned_net_helper_call_delta=6"
require_line "$DOC" "requires_hako_source_change=0"
require_line "$DOC" "generic_typed_field_residence_open=0"
require_line "$DOC" "generic_cse_open=0"
require_line "$DOC" "capsule_flattening_open=0"
require_line "$DOC" "summary=ok"

require_contains "$RUNTIME" 'nyash.object.exact_slot_set4_i64_hiiiii'
require_contains "$STORE" 'pub(crate) fn exact_slot_set4_i64'
require_contains "$PRESCAN" 'nyash.object.exact_slot_set4_i64_hiiiii'
require_contains "$LOWER" 'same_module_function_is_selected_result_capsule_reset_batch_target'
require_contains "$LOWER" 'HakoAllocObjectLifecycleAllocResult.reset/0'
require_contains "$LOWER" 'HakoAllocObjectLifecycleReleaseResult.reset/0'
require_contains "$LOWER" 'same_module_function_match_result_capsule_reset_batch_plan'
require_contains "$LOWER" 'nyash.object.exact_slot_set4_i64_hiiiii'

bash "$ROOT_DIR/tools/build_hako_llvmc_ffi.sh" >/dev/null
cargo build --release --bin hakorune -p nyash-rust >/dev/null
cargo build --release -p nyash_kernel >/dev/null

if command -v llvm-nm >/dev/null 2>&1; then
  llvm-nm -g "$ROOT_DIR/target/release/libnyash_kernel.a" >"$TMP_DIR/kernel.nm" 2>/dev/null
  require_contains "$TMP_DIR/kernel.nm" 'nyash.object.exact_slot_set4_i64_hiiiii'
fi

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" --backend mir --emit-mir-json "$MIR" "$APP" >/dev/null
python3 "$ROOT_DIR/tools/checks/pure_first_route_preflight.py" "$MIR" >/dev/null

HAKO_TYPED_OBJECT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1 \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/tools/selfhost/selfhost_build.sh" --mir-in "$MIR" --exe "$EXE" >/dev/null

set4_symbol_count="$(strings "$EXE" | grep -c 'nyash.object.exact_slot_set4_i64_hiiiii' || true)"
if [ "$set4_symbol_count" -lt 1 ]; then
  echo "[row258-result-capsule-reset-impl] exact-EXE does not reference set4 helper" >&2
  exit 1
fi

HAKO_ARRAY_SLOT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1 \
NYASH_DISABLE_PLUGINS=1 \
  "$EXE" >"$OUT"

require_contains "$OUT" '^summary=ok$'

cat <<REPORT
output_contract=result-capsule-reset-field-batching-implementation-v0
runtime_helper_symbol=nyash.object.exact_slot_set4_i64_hiiiii
runtime_helper_exported=1
exact_exe_set4_symbol_present=1
semantic_proof_summary=ok
planned_net_helper_call_delta=6
requires_hako_source_change=0
generic_typed_field_residence_open=0
generic_cse_open=0
capsule_flattening_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT
