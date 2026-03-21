#!/bin/bash
# phase29cc_wsm_g2_browser_run_vm.sh
# Contract pin:
# - WSM-G2-min2 runs nyash_playground in headless browser with autorun=1.
# - Output must include ConsoleBox 5-method demo markers from WSM-02d-min2 fixture.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

BUILD_SCRIPT="$NYASH_ROOT/projects/nyash-wasm/build.sh"
PLAYGROUND_DIR="$NYASH_ROOT/projects/nyash-wasm"
URL="http://127.0.0.1:18080/nyash_playground.html?autorun=1"

if ! command -v chromium-browser >/dev/null 2>&1; then
  test_fail "phase29cc_wsm_g2_browser_run_vm: chromium-browser not found"
  exit 2
fi

set +e
build_out=$(cd "$NYASH_ROOT" && bash "$BUILD_SCRIPT" 2>&1)
build_rc=$?
set -e
if [ "$build_rc" -ne 0 ]; then
  test_fail "phase29cc_wsm_g2_browser_run_vm: wasm build failed (rc=$build_rc)"
  printf '%s\n' "$build_out" | sed -n '1,200p'
  exit 1
fi

srv_log="$(mktemp)"
dom_out="$(mktemp)"
cleanup() {
  if [ -n "${srv_pid:-}" ]; then
    kill "$srv_pid" >/dev/null 2>&1 || true
    wait "$srv_pid" 2>/dev/null || true
  fi
  rm -f "$srv_log" "$dom_out"
}
trap cleanup EXIT

(cd "$PLAYGROUND_DIR" && python3 -m http.server 18080 >"$srv_log" 2>&1) &
srv_pid=$!
sleep 1

set +e
chromium-browser \
  --headless \
  --disable-gpu \
  --no-sandbox \
  --virtual-time-budget=10000 \
  --dump-dom \
  "$URL" >"$dom_out" 2>&1
chrome_rc=$?
set -e
if [ "$chrome_rc" -ne 0 ]; then
  test_fail "phase29cc_wsm_g2_browser_run_vm: chromium headless failed (rc=$chrome_rc)"
  sed -n '1,200p' "$dom_out"
  exit 1
fi

for marker in \
  "wsm02d_demo_min_log" \
  "wsm02d_demo_min_warn" \
  "wsm02d_demo_min_error" \
  "wsm02d_demo_min_info" \
  "wsm02d_demo_min_debug" \
  "[autorun] done"
do
  if ! grep -F -q "$marker" "$dom_out"; then
    test_fail "phase29cc_wsm_g2_browser_run_vm: missing marker in DOM: $marker"
    sed -n '1,220p' "$dom_out"
    exit 1
  fi
done

test_pass "phase29cc_wsm_g2_browser_run_vm: PASS (WSM-G2-min2 headless browser run contract)"
