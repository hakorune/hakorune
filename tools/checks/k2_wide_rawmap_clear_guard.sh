#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

RUST_TESTS=(
  clear_raw_alias_keeps_contract
)

PYTHON_TESTS=(
  src.llvm_py.tests.test_collection_method_call
  src.llvm_py.tests.test_boxcall_collection_policy
  src.llvm_py.tests.test_method_call_collection_birth
  src.llvm_py.tests.test_rawmap_first_slice_lock
)

MAP_RUNTIME_FACADE_FILE="crates/nyash_kernel/src/plugin/map_runtime_facade.rs"
MAP_SUBSTRATE_FILE="crates/nyash_kernel/src/plugin/map_substrate.rs"
MIR_COLLECTION_FILE="src/llvm_py/instructions/mir_call/collection_method_call.py"
BOXCALL_COLLECTION_FILE="src/llvm_py/instructions/boxcall_runtime_data.py"
METHOD_CALL_FILE="src/llvm_py/instructions/mir_call/method_call.py"
METHOD_CALL_LEGACY_FILE="src/llvm_py/instructions/mir_call_legacy.py"
MANIFEST_FILE="docs/development/current/main/design/abi-export-manifest-v0.toml"
GENERATED_DEFAULTS_FILE="lang/src/vm/boxes/generated/abi_adapter_registry_defaults.hako"

echo "[k2-wide-rawmap-clear] running narrow RawMap clear acceptance pack"
echo "[k2-wide-rawmap-clear] --- Rust/kernel RawMap clear acceptance ---"

for test_name in "${RUST_TESTS[@]}"; do
  echo "[k2-wide-rawmap-clear] >>> ${test_name}"
  cargo test -q -p nyash_kernel "${test_name}" -- --nocapture
done

echo "[k2-wide-rawmap-clear] --- Python lowering lock ---"
env PYTHONPATH=src/llvm_py:src python3 -m unittest "${PYTHON_TESTS[@]}"

echo "[k2-wide-rawmap-clear] --- route lock ---"
rg -F -q 'pub(super) fn map_runtime_clear(handle: i64) -> i64' "$MAP_RUNTIME_FACADE_FILE"
rg -F -q 'nyash.map.clear_h' "$MAP_SUBSTRATE_FILE"
rg -F -q 'method_name == "clear"' "$MIR_COLLECTION_FILE"
rg -F -q 'method_name == "clear"' "$BOXCALL_COLLECTION_FILE"
rg -F -q 'method in {"get", "push", "set", "has", "clear", "delete"}' "$METHOD_CALL_FILE"
rg -F -q 'method in {"get", "push", "set", "has", "clear"}' "$METHOD_CALL_LEGACY_FILE"
rg -F -q 'method = "clear"' "$MANIFEST_FILE"
rg -F -q 'symbol = "nyash.map.clear_h"' "$MANIFEST_FILE"
rg -F -q 'AbiAdapterRegistryBox._put(reg, "MapBox", "clear", "nyash.map.clear_h"' "$GENERATED_DEFAULTS_FILE"

echo "[k2-wide-rawmap-clear] ok"
