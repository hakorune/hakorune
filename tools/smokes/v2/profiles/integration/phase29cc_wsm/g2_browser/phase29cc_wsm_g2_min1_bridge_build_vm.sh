#!/bin/bash
# phase29cc_wsm_g2_min1_bridge_build_vm.sh
# Contract pin:
# - WSM-G2-min1 builds nyash-wasm bridge crate and emits pkg/nyash_rust.js.
# - Browser playground default snippet keeps ConsoleBox 5-method minimum.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

BUILD_SCRIPT="$NYASH_ROOT/projects/nyash-wasm/build.sh"
PLAYGROUND="$NYASH_ROOT/projects/nyash-wasm/nyash_playground.html"
PKG_JS="$NYASH_ROOT/projects/nyash-wasm/pkg/nyash_rust.js"
PKG_WASM="$NYASH_ROOT/projects/nyash-wasm/pkg/nyash_rust_bg.wasm"

if [ ! -f "$BUILD_SCRIPT" ]; then
  test_fail "phase29cc_wsm_g2_min1_bridge_build_vm: missing build script: $BUILD_SCRIPT"
  exit 2
fi

set +e
output=$(cd "$NYASH_ROOT" && bash "$BUILD_SCRIPT" 2>&1)
rc=$?
set -e

if [ "$rc" -ne 0 ]; then
  test_fail "phase29cc_wsm_g2_min1_bridge_build_vm: build failed (rc=$rc)"
  printf '%s\n' "$output" | sed -n '1,200p'
  exit 1
fi

if [ ! -f "$PKG_JS" ] || [ ! -f "$PKG_WASM" ]; then
  test_fail "phase29cc_wsm_g2_min1_bridge_build_vm: pkg artifacts missing"
  exit 1
fi

if ! grep -q "class NyashWasm" "$PKG_JS"; then
  test_fail "phase29cc_wsm_g2_min1_bridge_build_vm: NyashWasm export missing in pkg js"
  exit 1
fi

for method in log warn error info debug
do
  if ! grep -q "console\\.${method}" "$PLAYGROUND"; then
    test_fail "phase29cc_wsm_g2_min1_bridge_build_vm: playground method marker missing: console.${method}"
    exit 1
  fi
done

for message in \
  "wsm02d_demo_min_log" \
  "wsm02d_demo_min_warn" \
  "wsm02d_demo_min_error" \
  "wsm02d_demo_min_info" \
  "wsm02d_demo_min_debug"
do
  if ! grep -q "$message" "$PLAYGROUND"; then
    test_fail "phase29cc_wsm_g2_min1_bridge_build_vm: playground message marker missing: $message"
    exit 1
  fi
done

test_pass "phase29cc_wsm_g2_min1_bridge_build_vm: PASS (WSM-G2-min1 bridge build/run baseline)"
