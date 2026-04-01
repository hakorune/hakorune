#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TESTS=(
  runtime_data_invalid_handle_returns_zero
  runtime_data_array_round_trip_keeps_rawarray_contract
  runtime_data_array_has_keeps_runtime_facade_fail_safe_contract
  runtime_data_array_non_i64_keys_keep_fail_safe_fallback_contract
  runtime_data_scalar_handle_keeps_facade_only_contract
  legacy_set_h_returns_zero_but_applies_value
  hi_hii_aliases_keep_fail_safe_contract
  slot_load_store_raw_aliases_keep_contract
  slot_append_raw_alias_keeps_contract
  slot_reserve_and_grow_raw_aliases_keep_length_and_expand_capacity
)

echo "[k2-core-rawarray-acceptance] running explicit RawArray acceptance tests"

for test_name in "${TESTS[@]}"; do
  echo "[k2-core-rawarray-acceptance] >>> ${test_name}"
  cargo test -q -p nyash_kernel "${test_name}" -- --nocapture
done

echo "[k2-core-rawarray-acceptance] ok"
