#!/bin/bash
# phase29cc_wsm_p5_min6_shape_expand_lock_vm.sh
# Contract pin:
# - WSM-P5-min6: default(hako-lane) extends native shape-table by one additional shape.
# - const->copy->return i32 path is emitted via native helper (no bridge fallback).

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-165-wsm-p5-min6-shape-expand-lock-ssot.md"
if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_p5_min6_shape_expand_lock_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-P5-min6" \
  "compile_hako_native_shape_bytes" \
  "const->copy->return" \
  "default(hako-lane)" \
  "bridge fallback"; do
  if ! grep -q "$needle" "$doc"; then
    test_fail "phase29cc_wsm_p5_min6_shape_expand_lock_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

cargo test --features wasm-backend wasm_hako_native_shape_bytes_ -- --nocapture
cargo test --features wasm-backend wasm_demo_default_hako_lane_native_const_copy_shape_contract -- --nocapture
cargo test --features wasm-backend wasm_demo_default_route_const_copy_uses_native_helper_contract -- --nocapture

test_pass "phase29cc_wsm_p5_min6_shape_expand_lock_vm: PASS (WSM-P5-min6 shape expand lock)"
