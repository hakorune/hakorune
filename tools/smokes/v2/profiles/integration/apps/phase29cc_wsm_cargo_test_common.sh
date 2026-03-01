#!/bin/bash
# phase29cc_wsm_cargo_test_common.sh
# Shared targeted cargo-test runner for phase29cc wasm smoke contracts.
#
# Why:
# - `cargo test <filter>` without target selection scans many unrelated test
#   binaries, which is expensive for portability gates.
# - This helper narrows execution to `--lib` or `--test wasm_demo_min_fixture`
#   when the filter prefix guarantees the test location.

set -euo pipefail

run_wsm_targeted_contract_test() {
  local filter="$1"
  if [ -z "$filter" ]; then
    return 1
  fi

  # Escape hatch: keep legacy broad search when needed for diagnostics.
  if [ "${NYASH_WASM_TARGETED_CARGO_TEST:-1}" != "1" ]; then
    cargo test --features wasm-backend "$filter" -- --nocapture
    return 0
  fi

  case "$filter" in
    wasm_demo_*)
      cargo test --features wasm-backend --test wasm_demo_min_fixture "$filter" -- --nocapture
      ;;
    wasm_shape_table_*|wasm_binary_writer_*|wasm_hako_default_lane_*|wasm_route_policy_*)
      cargo test --features wasm-backend --lib "$filter" -- --nocapture
      ;;
    *)
      cargo test --features wasm-backend "$filter" -- --nocapture
      ;;
  esac
}

