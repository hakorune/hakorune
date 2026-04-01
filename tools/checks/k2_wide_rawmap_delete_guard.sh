#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

RUST_TESTS=(
  delete_raw_alias_keeps_contract
)

PYTHON_TESTS=(
  src.llvm_py.tests.test_collection_method_call
  src.llvm_py.tests.test_boxcall_collection_policy
  src.llvm_py.tests.test_rawmap_first_slice_lock
)

MAP_RUNTIME_FACADE_FILE="crates/nyash_kernel/src/plugin/map_runtime_facade.rs"
MAP_SUBSTRATE_FILE="crates/nyash_kernel/src/plugin/map_substrate.rs"
MIR_COLLECTION_FILE="src/llvm_py/instructions/mir_call/collection_method_call.py"
BOXCALL_COLLECTION_FILE="src/llvm_py/instructions/boxcall_runtime_data.py"
METHOD_CALL_FILE="src/llvm_py/instructions/mir_call/method_call.py"

echo "[k2-wide-rawmap-delete] running narrow RawMap delete acceptance pack"
echo "[k2-wide-rawmap-delete] --- Rust/kernel RawMap delete acceptance ---"

for test_name in "${RUST_TESTS[@]}"; do
  echo "[k2-wide-rawmap-delete] >>> ${test_name}"
  cargo test -q -p nyash_kernel "${test_name}" -- --nocapture
done

echo "[k2-wide-rawmap-delete] --- Python lowering lock ---"
env PYTHONPATH=src/llvm_py:src python3 -m unittest "${PYTHON_TESTS[@]}"

echo "[k2-wide-rawmap-delete] --- route lock ---"
rg -F -q 'pub(super) fn map_runtime_delete_any(handle: i64, key_any: i64) -> i64' "$MAP_RUNTIME_FACADE_FILE"
rg -F -q 'nyash.map.delete_hh' "$MAP_SUBSTRATE_FILE"
rg -F -q 'method_name == "delete"' "$MIR_COLLECTION_FILE"
rg -F -q 'method in {"get", "push", "set", "has", "clear", "delete"}' "$METHOD_CALL_FILE"
rg -F -q 'method_name == "delete"' "$BOXCALL_COLLECTION_FILE"

echo "[k2-wide-rawmap-delete] ok"
