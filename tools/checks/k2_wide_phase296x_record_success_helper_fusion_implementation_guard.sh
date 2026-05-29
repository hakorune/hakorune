#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-282-RECORD-SUCCESS-HELPER-FUSION-IMPLEMENTATION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-281-RECORD-SUCCESS-HELPER-FUSION-GUARD-SURFACE.md"
LOWER="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_same_module_body_emit.inc"
PRESCAN="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc"
RUNTIME="$ROOT_DIR/crates/nyash_kernel/src/exports/typed_object.rs"
STORE="$ROOT_DIR/crates/nyash_kernel/src/exports/typed_object_store.rs"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TMP_DIR="$(mktemp -d /tmp/hakorune_row282_record_success.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR="$TMP_DIR/app.mir.json"
DEFAULT_EXE="$TMP_DIR/default.exe"
EXACT_EXE="$TMP_DIR/exact.exe"
OUT="$TMP_DIR/exact.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row282-record-success-helper-fusion] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_contains() {
  local file="$1"
  local expected="$2"
  if ! grep -q "$expected" "$file"; then
    echo "[row282-record-success-helper-fusion] missing content in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=record-success-helper-fusion-implementation-v0"
require_line "$DOC" "input_contract=record-success-helper-fusion-guard-surface-v0"
require_line "$DOC" "implementation_owner=c_abi_same_module_record_success_helper_fusion"
require_line "$DOC" "runtime_helper_symbol_0=nyash.object.exact_slot_record_alloc_success_hii"
require_line "$DOC" "runtime_helper_symbol_1=nyash.object.exact_slot_record_release_success_hiii"
require_line "$DOC" "runtime_helper_exported_count=2"
require_line "$DOC" "same_module_emit_selected_method_count=2"
require_line "$DOC" "same_module_emit_target_0=HakoAllocObjectLifecycleAllocResult.recordSuccess/1"
require_line "$DOC" "same_module_emit_target_1=HakoAllocObjectLifecycleReleaseResult.recordSuccess/2"
require_line "$DOC" "default_exact_helper_emission=0"
require_line "$DOC" "exact_exe_record_success_alloc_symbol_present=1"
require_line "$DOC" "exact_exe_record_success_release_symbol_present=1"
require_line "$DOC" "semantic_proof_summary=ok"
require_line "$DOC" "planned_net_helper_call_delta=12"
require_line "$DOC" "requires_hako_source_change=0"
require_line "$DOC" "generic_typed_field_residence_open=0"
require_line "$DOC" "generic_cse_open=0"
require_line "$DOC" "capsule_value_aggregate_open=0"
require_line "$DOC" "source_rewrite=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

require_contains "$RUNTIME" 'nyash.object.exact_slot_record_alloc_success_hii'
require_contains "$RUNTIME" 'nyash.object.exact_slot_record_release_success_hiii'
require_contains "$STORE" 'pub(crate) fn exact_slot_record_alloc_success'
require_contains "$STORE" 'pub(crate) fn exact_slot_record_release_success'
require_contains "$PRESCAN" 'nyash.object.exact_slot_record_alloc_success_hii'
require_contains "$PRESCAN" 'nyash.object.exact_slot_record_release_success_hiii'
require_contains "$LOWER" 'same_module_function_selected_record_success_helper_kind'
require_contains "$LOWER" 'HakoAllocObjectLifecycleAllocResult.recordSuccess/1'
require_contains "$LOWER" 'HakoAllocObjectLifecycleReleaseResult.recordSuccess/2'
require_contains "$LOWER" 'nyash.object.exact_slot_record_alloc_success_hii'
require_contains "$LOWER" 'nyash.object.exact_slot_record_release_success_hiii'

bash "$ROOT_DIR/tools/build_hako_llvmc_ffi.sh" >/dev/null
cargo build --release --bin hakorune >/dev/null
cargo build --release -p nyash_kernel >/dev/null

if command -v llvm-nm >/dev/null 2>&1; then
  llvm-nm -g "$ROOT_DIR/target/release/libnyash_kernel.a" >"$TMP_DIR/kernel.nm" 2>/dev/null
  require_contains "$TMP_DIR/kernel.nm" 'nyash.object.exact_slot_record_alloc_success_hii'
  require_contains "$TMP_DIR/kernel.nm" 'nyash.object.exact_slot_record_release_success_hiii'
fi

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" --backend mir --emit-mir-json "$MIR" "$APP" >/dev/null
python3 "$ROOT_DIR/tools/checks/pure_first_route_preflight.py" "$MIR" >/dev/null

NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/tools/selfhost/selfhost_build.sh" --mir-in "$MIR" --exe "$DEFAULT_EXE" >/dev/null
if grep -a -q 'nyash.object.exact_slot_record_' "$DEFAULT_EXE"; then
  echo "[row282-record-success-helper-fusion] default EXE unexpectedly references recordSuccess helper" >&2
  exit 1
fi

HAKO_TYPED_OBJECT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1 \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/tools/selfhost/selfhost_build.sh" --mir-in "$MIR" --exe "$EXACT_EXE" >/dev/null

grep -a -q 'nyash.object.exact_slot_record_alloc_success_hii' "$EXACT_EXE"
grep -a -q 'nyash.object.exact_slot_record_release_success_hiii' "$EXACT_EXE"

HAKO_ARRAY_SLOT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1 \
NYASH_DISABLE_PLUGINS=1 \
  "$EXACT_EXE" >"$OUT"

require_contains "$OUT" '^summary=ok$'

cat <<REPORT
output_contract=record-success-helper-fusion-implementation-v0
runtime_helper_symbol_0=nyash.object.exact_slot_record_alloc_success_hii
runtime_helper_symbol_1=nyash.object.exact_slot_record_release_success_hiii
runtime_helper_exported_count=2
default_exact_helper_emission=0
exact_exe_record_success_alloc_symbol_present=1
exact_exe_record_success_release_symbol_present=1
semantic_proof_summary=ok
planned_net_helper_call_delta=12
requires_hako_source_change=0
generic_typed_field_residence_open=0
generic_cse_open=0
capsule_value_aggregate_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT
