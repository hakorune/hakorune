#!/bin/bash
# phase29cc_wsm_g4_min5_headless_two_examples_vm.sh
# Contract pin:
# - WSM-G4-min5: headless parity lock for webcanvas + canvas_advanced

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-175-wsm-g4-min5-headless-two-example-parity-lock-ssot.md"
build_script="$NYASH_ROOT/projects/nyash-wasm/build.sh"
playground_dir="$NYASH_ROOT/projects/nyash-wasm"
base_url="http://127.0.0.1:18080/nyash_playground.html?autorun=1"

if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_g4_min5_headless_two_examples_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-G4-min5" \
  "webcanvas" \
  "canvas_advanced" \
  "[autorun-example]"; do
  if ! grep -Fq "$needle" "$doc"; then
    test_fail "phase29cc_wsm_g4_min5_headless_two_examples_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

if ! command -v chromium-browser >/dev/null 2>&1; then
  test_fail "phase29cc_wsm_g4_min5_headless_two_examples_vm: chromium-browser not found"
  exit 2
fi

bash "$NYASH_ROOT/tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g4_min4_canvas_advanced_fixture_parity_vm.sh"

set +e
build_out=$(cd "$NYASH_ROOT" && bash "$build_script" 2>&1)
build_rc=$?
set -e
if [ "$build_rc" -ne 0 ]; then
  test_fail "phase29cc_wsm_g4_min5_headless_two_examples_vm: wasm build failed (rc=$build_rc)"
  printf '%s\n' "$build_out" | sed -n '1,200p'
  exit 1
fi

srv_log="$(mktemp)"
dom_webcanvas="$(mktemp)"
dom_advanced="$(mktemp)"
cleanup() {
  if [ -n "${srv_pid:-}" ]; then
    kill "$srv_pid" >/dev/null 2>&1 || true
    wait "$srv_pid" 2>/dev/null || true
  fi
  rm -f "$srv_log" "$dom_webcanvas" "$dom_advanced"
}
trap cleanup EXIT

(cd "$playground_dir" && python3 -m http.server 18080 >"$srv_log" 2>&1) &
srv_pid=$!
sleep 1

set +e
chromium-browser \
  --headless \
  --disable-gpu \
  --no-sandbox \
  --virtual-time-budget=10000 \
  --dump-dom \
  "${base_url}&example=webcanvas" >"$dom_webcanvas" 2>&1
rc_webcanvas=$?
set -e
if [ "$rc_webcanvas" -ne 0 ]; then
  test_fail "phase29cc_wsm_g4_min5_headless_two_examples_vm: webcanvas headless run failed (rc=$rc_webcanvas)"
  sed -n '1,220p' "$dom_webcanvas"
  exit 1
fi

for marker in \
  "[autorun-example] webcanvas" \
  "wsm_g4_min3_webcanvas_marker_1" \
  "wsm_g4_min3_webcanvas_marker_2" \
  "[autorun] done"; do
  if ! grep -F -q "$marker" "$dom_webcanvas"; then
    test_fail "phase29cc_wsm_g4_min5_headless_two_examples_vm: webcanvas missing marker: $marker"
    sed -n '1,220p' "$dom_webcanvas"
    exit 1
  fi
done

set +e
chromium-browser \
  --headless \
  --disable-gpu \
  --no-sandbox \
  --virtual-time-budget=10000 \
  --dump-dom \
  "${base_url}&example=canvas_advanced" >"$dom_advanced" 2>&1
rc_advanced=$?
set -e
if [ "$rc_advanced" -ne 0 ]; then
  test_fail "phase29cc_wsm_g4_min5_headless_two_examples_vm: canvas_advanced headless run failed (rc=$rc_advanced)"
  sed -n '1,220p' "$dom_advanced"
  exit 1
fi

for marker in \
  "[autorun-example] canvas_advanced" \
  "wsm_g4_min4_canvas_advanced_marker_1" \
  "wsm_g4_min4_canvas_advanced_marker_2" \
  "[autorun] done"; do
  if ! grep -F -q "$marker" "$dom_advanced"; then
    test_fail "phase29cc_wsm_g4_min5_headless_two_examples_vm: canvas_advanced missing marker: $marker"
    sed -n '1,220p' "$dom_advanced"
    exit 1
  fi
done

test_pass "phase29cc_wsm_g4_min5_headless_two_examples_vm: PASS (WSM-G4-min5 headless two-example parity lock)"
