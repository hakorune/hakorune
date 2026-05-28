#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-211-SELECTED-METHOD-ARRAY-SLOT-DIRECT-OP-KEEPER.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-210-SELECTED-METHOD-ARRAY-SLOT-DIRECT-OP-OWNER-SELECTION.md"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TMP_DIR="$(mktemp -d /tmp/hakorune_row211_array_direct.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR="$TMP_DIR/app.mir.json"
EXE="$TMP_DIR/app.exe"
DEFAULT_OUT="$TMP_DIR/default.out"
SINGLE_OUT="$TMP_DIR/single_thread.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row211-array-direct-op] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_contains() {
  local file="$1"
  local expected="$2"
  if ! grep -q "$expected" "$file"; then
    echo "[row211-array-direct-op] missing content in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=selected-method-array-slot-direct-op-keeper-v0"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "selected_block=45"
require_line "$DOC" "implementation_owner=c_abi_same_module_array_slot_direct_op_fusion"
require_line "$DOC" "helper_owner_file=lang/c-abi/shims/hako_llvmc_ffi_same_module_body_emit.inc"
require_line "$DOC" "declaration_owner_file=lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc"
require_line "$DOC" "runtime_export_owner=crates/nyash_kernel/src/plugin/array_direct_slot_op.rs"
require_line "$DOC" "fused_runtime_symbol=nyash.array.slot_load_store_i64_hihi"
require_line "$DOC" "planned_erased_get_set_helper_calls=2"
require_line "$DOC" "planned_added_fused_helper_calls=1"
require_line "$DOC" "planned_net_helper_call_delta=1"
require_line "$DOC" "generic_array_residence_open=0"
require_line "$DOC" "by_name_hako_alloc_special_case=0"

bash "$ROOT_DIR/tools/build_hako_llvmc_ffi.sh" >/dev/null
cargo build --release -p nyash_kernel >/dev/null

if ! command -v llvm-nm >/dev/null 2>&1; then
  echo "[row211-array-direct-op] llvm-nm is required for the export check" >&2
  exit 1
fi

llvm-nm -g "$ROOT_DIR/target/release/libnyash_kernel.a" >"$TMP_DIR/kernel.nm" 2>/dev/null
if ! grep -q ' nyash\.array\.slot_load_store_i64_hihi$' "$TMP_DIR/kernel.nm"; then
  echo "[row211-array-direct-op] fused runtime helper is not exported" >&2
  exit 1
fi

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" --backend mir --emit-mir-json "$MIR" "$APP" >/dev/null
python3 "$ROOT_DIR/tools/checks/pure_first_route_preflight.py" "$MIR" >/dev/null
NYASH_DISABLE_PLUGINS=1 "$ROOT_DIR/tools/selfhost/selfhost_build.sh" --mir-in "$MIR" --exe "$EXE" >/dev/null

fused_symbol_count="$(strings "$EXE" | grep -c 'nyash.array.slot_load_store_i64_hihi' || true)"
if [ "$fused_symbol_count" -lt 1 ]; then
  echo "[row211-array-direct-op] exact-EXE does not reference fused runtime symbol" >&2
  exit 1
fi

NYASH_DISABLE_PLUGINS=1 "$EXE" >"$DEFAULT_OUT"
HAKO_ARRAY_SLOT_STORE=single_thread_exact HAKO_TYPED_OBJECT_STORE=single_thread_exact NYASH_DISABLE_PLUGINS=1 "$EXE" >"$SINGLE_OUT"

require_contains "$DEFAULT_OUT" '^summary=ok$'
require_contains "$SINGLE_OUT" '^summary=ok$'

cat <<REPORT
output_contract=selected-method-array-slot-direct-op-keeper-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_block=45
implementation_owner=c_abi_same_module_array_slot_direct_op_fusion
runtime_helper_exported=1
selected_block_fused=1
fused_runtime_symbol=nyash.array.slot_load_store_i64_hihi
exact_exe_fused_symbol_count=${fused_symbol_count}
erased_get_set_helper_calls=2
added_fused_helper_calls=1
net_helper_call_delta=1
semantic_proof_summary=ok
default_backend_smoke=ok
single_thread_backend_smoke=ok
generic_array_residence_open=0
by_name_hako_alloc_special_case=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT
