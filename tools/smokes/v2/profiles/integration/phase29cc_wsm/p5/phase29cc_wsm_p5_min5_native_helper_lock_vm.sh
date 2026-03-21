#!/bin/bash
# phase29cc_wsm_p5_min5_native_helper_lock_vm.sh
# Contract pin:
# - WSM-P5-min5: default(hako-lane) pilot shape uses native helper path.
# - non-pilot shapes stay explicit bridge fallback.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-164-wsm-p5-min5-native-helper-lock-ssot.md"
if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_p5_min5_native_helper_lock_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-P5-min5" \
  "compile_hako_native_pilot_bytes" \
  "default(hako-lane)" \
  "bridge fallback" \
  "pilot shape"; do
  if ! grep -q "$needle" "$doc"; then
    test_fail "phase29cc_wsm_p5_min5_native_helper_lock_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

cargo test --features wasm-backend wasm_hako_native_pilot_bytes_ -- --nocapture
cargo test --features wasm-backend wasm_demo_default_route_ -- --nocapture

test_pass "phase29cc_wsm_p5_min5_native_helper_lock_vm: PASS (WSM-P5-min5 native helper lock)"
