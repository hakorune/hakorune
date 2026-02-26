#!/bin/bash
# phase29cc_wsm_p4_min2_binary_writer_lock_vm.sh
# Contract pin:
# - WSM-P4-min2: wasm binary writer skeleton lock
#   (magic/version + section order + LEB128 + main export marker).

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

set +e
output=$(cd "$NYASH_ROOT" && cargo test --features wasm-backend wasm_binary_writer_ -- --nocapture 2>&1)
rc=$?
set -e

if [ "$rc" -ne 0 ]; then
  test_fail "phase29cc_wsm_p4_min2_binary_writer_lock_vm: binary writer contract tests failed (rc=$rc)"
  printf '%s\n' "$output" | sed -n '1,220p'
  exit 1
fi

for marker in \
  "wasm_binary_writer_magic_version_contract" \
  "wasm_binary_writer_section_order_contract" \
  "wasm_binary_writer_leb128_contract" \
  "wasm_binary_writer_main_export_contract" \
  "wasm_binary_writer_minimal_module_contract"; do
  if ! printf '%s\n' "$output" | grep -q "$marker"; then
    test_fail "phase29cc_wsm_p4_min2_binary_writer_lock_vm: expected marker missing: $marker"
    printf '%s\n' "$output" | sed -n '1,220p'
    exit 1
  fi
done

test_pass "phase29cc_wsm_p4_min2_binary_writer_lock_vm: PASS (WSM-P4-min2 binary writer skeleton lock)"
