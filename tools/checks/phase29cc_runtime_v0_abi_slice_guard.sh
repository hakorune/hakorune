#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

source "$ROOT_DIR/tools/checks/lib/runtime_v0_abi_slice_sections.sh"

RUNTIME_V0_ABI_SLICE_TAG="runtime-v0-abi-slice-guard"

LOCK_DOC="docs/development/current/main/phases/phase-29cc/29cc-216-runtime-v0-abi-slice-lock-ssot.md"
CUTOVER_SSOT="docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md"
ABI_MATRIX="docs/reference/abi/ABI_BOUNDARY_MATRIX.md"
DEV_GATE="tools/checks/dev_gate.sh"
MANIFEST_FILE="docs/development/current/main/design/abi-export-manifest-v0.toml"
ABI_MANIFEST_CODEGEN="tools/abi_manifest_codegen.py"
GENERATED_DEFAULTS_FILE="lang/src/vm/boxes/generated/abi_adapter_registry_defaults.hako"
REGISTRY_FILE="lang/src/vm/boxes/abi_adapter_registry.hako"
HANDLER_FILE="lang/src/vm/boxes/mir_call_v1_handler.hako"
ARRAY_CORE_FILE="lang/src/runtime/collections/array_core_box.hako"
ARRAY_STATE_CORE_FILE="lang/src/runtime/collections/array_state_core_box.hako"
RAW_ARRAY_CORE_FILE="lang/src/runtime/substrate/raw_array/raw_array_core_box.hako"
RAW_MAP_CORE_FILE="lang/src/runtime/substrate/raw_map/raw_map_core_box.hako"
ATOMIC_CORE_FILE="lang/src/runtime/substrate/atomic/atomic_core_box.hako"
TLS_CORE_FILE="lang/src/runtime/substrate/tls/tls_core_box.hako"
GC_CORE_FILE="lang/src/runtime/substrate/gc/gc_core_box.hako"
OSVM_CORE_FILE="lang/src/runtime/substrate/osvm/osvm_core_box.hako"
INTRIN_CORE_FILE="lang/src/runtime/substrate/intrin/intrin_core_box.hako"
INITIALIZED_RANGE_CORE_FILE="lang/src/runtime/substrate/verifier/initialized_range/initialized_range_core_box.hako"
BOUNDS_CORE_FILE="lang/src/runtime/substrate/verifier/bounds/bounds_core_box.hako"
OWNERSHIP_CORE_FILE="lang/src/runtime/substrate/verifier/ownership/ownership_core_box.hako"
BUF_CORE_FILE="lang/src/runtime/substrate/buf/buf_core_box.hako"
PTR_CORE_FILE="lang/src/runtime/substrate/ptr/ptr_core_box.hako"
VALUE_REPR_CORE_FILE="lang/src/runtime/substrate/value_repr/current_lane_box.hako"
STRING_CORE_FILE="lang/src/runtime/collections/string_core_box.hako"
MAP_CORE_FILE="lang/src/runtime/collections/map_core_box.hako"
COLLECTIONS_HOT_FILE="lang/src/llvm_ir/boxes/aot_prep/passes/collections_hot.hako"

for file in \
  "$LOCK_DOC" \
  "$CUTOVER_SSOT" \
  "$ABI_MATRIX" \
  "$DEV_GATE" \
  "$MANIFEST_FILE" \
  "$ABI_MANIFEST_CODEGEN" \
  "$GENERATED_DEFAULTS_FILE" \
  "$REGISTRY_FILE" \
  "$HANDLER_FILE" \
  "$ARRAY_CORE_FILE" \
  "$ARRAY_STATE_CORE_FILE" \
  "$RAW_ARRAY_CORE_FILE" \
  "$RAW_MAP_CORE_FILE" \
  "$ATOMIC_CORE_FILE" \
  "$TLS_CORE_FILE" \
  "$GC_CORE_FILE" \
  "$OSVM_CORE_FILE" \
  "$INTRIN_CORE_FILE" \
  "$BOUNDS_CORE_FILE" \
  "$INITIALIZED_RANGE_CORE_FILE" \
  "$OWNERSHIP_CORE_FILE" \
  "$BUF_CORE_FILE" \
  "$PTR_CORE_FILE" \
  "$VALUE_REPR_CORE_FILE" \
  "$STRING_CORE_FILE" \
  "$MAP_CORE_FILE" \
  "$COLLECTIONS_HOT_FILE"; do
  if [ ! -f "$file" ]; then
    runtime_v0_abi_slice_fail "missing file: $file"
  fi
done

runtime_v0_abi_slice_check_collections_hot
if ! python3 "$ABI_MANIFEST_CODEGEN" --check; then
  runtime_v0_abi_slice_fail "abi manifest codegen --check failed"
fi
runtime_v0_abi_slice_check_lock_docs_and_manifest
runtime_v0_abi_slice_check_adapter_defaults
runtime_v0_abi_slice_check_raw_array
runtime_v0_abi_slice_check_raw_map
runtime_v0_abi_slice_check_substrates

echo "[${RUNTIME_V0_ABI_SLICE_TAG}] ok"
