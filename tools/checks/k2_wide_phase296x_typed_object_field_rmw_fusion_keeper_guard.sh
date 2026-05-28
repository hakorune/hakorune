#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-222-TYPED-OBJECT-FIELD-RMW-FUSION-KEEPER.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-221-TYPED-OBJECT-FIELD-RMW-FUSION-SELECTION.md"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TMP_DIR="$(mktemp -d /tmp/hakorune_row222_rmw_keeper.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR="$TMP_DIR/app.mir.json"
EXE="$TMP_DIR/app.exe"
OUT="$TMP_DIR/app.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row222-rmw-keeper] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_contains() {
  local file="$1"
  local expected="$2"
  if ! grep -q "$expected" "$file"; then
    echo "[row222-rmw-keeper] missing content in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=typed-object-field-rmw-fusion-keeper-v0"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "implementation_owner=c_abi_same_module_typed_field_rmw_fusion"
require_line "$DOC" "fused_runtime_symbol=nyash.object.exact_slot_rmw_add_u64_hiii"
require_line "$DOC" "planned_erased_get_set_helper_calls=10"
require_line "$DOC" "planned_added_fused_helper_calls=5"
require_line "$DOC" "planned_net_helper_call_delta=5"
require_line "$DOC" "rejected_extra_get_use_count=1"
require_line "$DOC" "runtime_storage_owner_preserved=1"
require_line "$DOC" "helper_free_direct_op_rejected=1"
require_line "$DOC" "generic_residence_open=0"
require_line "$DOC" "source_rewrite=0"
require_line "$DOC" "by_name_hako_alloc_special_case=0"
require_line "$DOC" "summary=ok"

bash "$ROOT_DIR/tools/build_hako_llvmc_ffi.sh" >/dev/null
cargo build --release -p nyash_kernel >/dev/null

if ! command -v llvm-nm >/dev/null 2>&1; then
  echo "[row222-rmw-keeper] llvm-nm is required for the export check" >&2
  exit 1
fi

llvm-nm -g "$ROOT_DIR/target/release/libnyash_kernel.a" >"$TMP_DIR/kernel.nm" 2>/dev/null
if ! grep -q ' nyash\.object\.exact_slot_rmw_add_u64_hiii$' "$TMP_DIR/kernel.nm"; then
  echo "[row222-rmw-keeper] fused runtime helper is not exported" >&2
  exit 1
fi

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" --backend mir --emit-mir-json "$MIR" "$APP" >/dev/null
python3 "$ROOT_DIR/tools/checks/pure_first_route_preflight.py" "$MIR" >/dev/null

HAKO_TYPED_OBJECT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1 \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/tools/selfhost/selfhost_build.sh" --mir-in "$MIR" --exe "$EXE" >/dev/null

fused_symbol_count="$(strings "$EXE" | grep -c 'nyash.object.exact_slot_rmw_add_u64_hiii' || true)"
if [ "$fused_symbol_count" -lt 1 ]; then
  echo "[row222-rmw-keeper] exact-EXE does not reference fused runtime symbol" >&2
  exit 1
fi

HAKO_ARRAY_SLOT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1 \
NYASH_DISABLE_PLUGINS=1 \
  "$EXE" >"$OUT"

require_contains "$OUT" '^summary=ok$'

cat <<REPORT
output_contract=typed-object-field-rmw-fusion-keeper-v0
selected_method=HakoAllocPageModel.acquire_usize/1
implementation_owner=c_abi_same_module_typed_field_rmw_fusion
runtime_helper_exported=1
selected_method_fused=1
fused_runtime_symbol=nyash.object.exact_slot_rmw_add_u64_hiii
exact_exe_fused_symbol_count=${fused_symbol_count}
erased_get_set_helper_calls=10
added_fused_helper_calls=5
net_helper_call_delta=5
semantic_proof_summary=ok
single_thread_backend_smoke=ok
runtime_storage_owner_preserved=1
helper_free_direct_op_rejected=1
generic_residence_open=0
by_name_hako_alloc_special_case=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT
