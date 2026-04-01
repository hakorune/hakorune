#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

RUST_TESTS=(
  runtime_data_map_get_keeps_mixed_runtime_i64_contract
  runtime_data_map_any_key_keeps_shared_facade_contract
  slot_probe_raw_aliases_keep_hh_contract
  slot_probe_raw_aliases_keep_hi_contract
  raw_aliases_keep_fail_safe_contract
  entry_count_raw_alias_keeps_contract
  capacity_raw_alias_keeps_observer_contract
)

PYTHON_TESTS=(
  src.llvm_py.tests.test_rawmap_first_slice_lock
)

GENERATED_DEFAULTS_FILE="lang/src/vm/boxes/generated/abi_adapter_registry_defaults.hako"
RAW_MAP_CORE_FILE="lang/src/runtime/substrate/raw_map/raw_map_core_box.hako"

echo "[k2-wide-rawmap-first-slice] running narrow RawMap first-slice acceptance pack"
echo "[k2-wide-rawmap-first-slice] --- Rust/kernel RawMap acceptance ---"

for test_name in "${RUST_TESTS[@]}"; do
  echo "[k2-wide-rawmap-first-slice] >>> ${test_name}"
  cargo test -q -p nyash_kernel "${test_name}" -- --nocapture
done

echo "[k2-wide-rawmap-first-slice] --- Python lowering lock ---"
env PYTHONPATH=src/llvm_py:src python3 -m unittest "${PYTHON_TESTS[@]}"

echo "[k2-wide-rawmap-first-slice] --- ABI/substrate route lock ---"
rg -zq 'MapBox",\s*"get".*nyash\.map\.slot_load_hh' "$GENERATED_DEFAULTS_FILE"
rg -zq 'MapBox",\s*"set".*nyash\.map\.slot_store_hhh' "$GENERATED_DEFAULTS_FILE"
rg -zq 'MapBox",\s*"has".*nyash\.map\.probe_hh' "$GENERATED_DEFAULTS_FILE"
rg -zq 'MapBox",\s*"size".*nyash\.map\.entry_count_i64' "$GENERATED_DEFAULTS_FILE"

rg -F -q 'externcall "nyash.map.entry_count_i64"' "$RAW_MAP_CORE_FILE"
rg -F -q 'externcall "nyash.map.cap_h"' "$RAW_MAP_CORE_FILE"
rg -F -q 'externcall "nyash.map.slot_load_hh"' "$RAW_MAP_CORE_FILE"
rg -F -q 'externcall "nyash.map.slot_store_hhh"' "$RAW_MAP_CORE_FILE"
rg -F -q 'externcall "nyash.map.probe_hh"' "$RAW_MAP_CORE_FILE"

echo "[k2-wide-rawmap-first-slice] ok"
