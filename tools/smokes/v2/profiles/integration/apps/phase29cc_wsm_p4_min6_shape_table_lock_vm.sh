#!/bin/bash
# phase29cc_wsm_p4_min6_shape_table_lock_vm.sh
# Contract pin:
# - WSM-P4-min6: pilot shape routing is governed by wasm shape table (boxed matcher) only.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

set +e
output=$(cd "$NYASH_ROOT" && cargo test --features wasm-backend wasm_shape_table_ -- --nocapture 2>&1)
rc=$?
set -e

if [ "$rc" -ne 0 ]; then
  test_fail "phase29cc_wsm_p4_min6_shape_table_lock_vm: shape table contract tests failed (rc=$rc)"
  printf '%s\n' "$output" | sed -n '1,220p'
  exit 1
fi

for marker in \
  "wasm_shape_table_matches_min_const_return_contract" \
  "wasm_shape_table_rejects_non_const_return_contract"; do
  if ! printf '%s\n' "$output" | grep -q "$marker"; then
    test_fail "phase29cc_wsm_p4_min6_shape_table_lock_vm: expected marker missing: $marker"
    printf '%s\n' "$output" | sed -n '1,220p'
    exit 1
  fi
done

test_pass "phase29cc_wsm_p4_min6_shape_table_lock_vm: PASS (WSM-P4-min6 shape table lock)"
