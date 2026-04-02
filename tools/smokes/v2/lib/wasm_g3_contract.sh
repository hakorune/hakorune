#!/bin/bash
# wasm_g3_contract.sh
# Shared runner for WSM-G3 canvas contract smokes.
# This helper belongs to the `wasm` experimental lane and is not a product-mainline gate.

run_wasm_g3_contract_smoke() {
  local smoke_name="$1"
  local extern_test="$2"
  local runtime_test="$3"
  local pass_msg="$4"

  set +e
  local output1
  output1=$(cd "$NYASH_ROOT" && cargo test --features wasm-backend "$extern_test" -- --nocapture 2>&1)
  local rc1=$?
  set -e

  if [ "$rc1" -ne 0 ]; then
    test_fail "${smoke_name}: extern contract test failed (rc=$rc1)"
    printf '%s\n' "$output1" | sed -n '1,200p'
    exit 1
  fi

  set +e
  local output2
  output2=$(cd "$NYASH_ROOT" && cargo test --features wasm-backend "$runtime_test" -- --nocapture 2>&1)
  local rc2=$?
  set -e

  if [ "$rc2" -ne 0 ]; then
    test_fail "${smoke_name}: runtime JS import test failed (rc=$rc2)"
    printf '%s\n' "$output2" | sed -n '1,200p'
    exit 1
  fi

  test_pass "$pass_msg"
}
