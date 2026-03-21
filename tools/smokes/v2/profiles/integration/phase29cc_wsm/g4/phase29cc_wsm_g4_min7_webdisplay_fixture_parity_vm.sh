#!/bin/bash
# phase29cc_wsm_g4_min7_webdisplay_fixture_parity_vm.sh
# Contract pin:
# - WSM-G4-min7: webdisplay fixture parity lock

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-204-wsm-g4-min7-webdisplay-fixture-parity-lock-ssot.md"
fixture="$NYASH_ROOT/apps/tests/phase29cc_wsm_g4_min7_webdisplay_fixture_min.hako"
playground_dir="$NYASH_ROOT/projects/nyash-wasm"
playground_url="http://127.0.0.1:18080/nyash_playground.html?autorun=1&example=webdisplay"

if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_g4_min7_webdisplay_fixture_parity_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-G4-min7" \
  "webdisplay" \
  "wsm_g4_min7_webdisplay_marker_1" \
  "wsm_g4_min7_webdisplay_marker_2"; do
  if ! grep -Fq "$needle" "$doc"; then
    test_fail "phase29cc_wsm_g4_min7_webdisplay_fixture_parity_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

for needle in \
  "wsm_g4_min7_webdisplay_marker_1" \
  "wsm_g4_min7_webdisplay_marker_2"; do
  if ! grep -Fq "$needle" "$fixture"; then
    test_fail "phase29cc_wsm_g4_min7_webdisplay_fixture_parity_vm: missing keyword in fixture: $needle"
    exit 1
  fi
done

bash "$NYASH_ROOT/tools/smokes/v2/profiles/integration/phase29cc_wsm/g2_browser/phase29cc_wsm_g2_min1_bridge_build_vm.sh"
bash "$NYASH_ROOT/projects/nyash-wasm/build.sh"

if ! command -v chromium-browser >/dev/null 2>&1; then
  test_fail "phase29cc_wsm_g4_min7_webdisplay_fixture_parity_vm: chromium-browser not found"
  exit 2
fi

dom_out="$(mktemp)"
srv_log="$(mktemp)"
cleanup() {
  if [ -n "${srv_pid:-}" ]; then
    kill "$srv_pid" >/dev/null 2>&1 || true
    wait "$srv_pid" 2>/dev/null || true
  fi
  rm -f "$dom_out" "$srv_log"
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
  "$playground_url" >"$dom_out" 2>&1
rc=$?
set -e
if [ "$rc" -ne 0 ]; then
  test_fail "phase29cc_wsm_g4_min7_webdisplay_fixture_parity_vm: headless run failed (rc=$rc)"
  sed -n '1,220p' "$dom_out"
  exit 1
fi

for marker in \
  "[autorun-example] webdisplay" \
  "wsm_g4_min7_webdisplay_marker_1" \
  "wsm_g4_min7_webdisplay_marker_2" \
  "[autorun] done"; do
  if ! grep -Fq "$marker" "$dom_out"; then
    test_fail "phase29cc_wsm_g4_min7_webdisplay_fixture_parity_vm: missing marker: $marker"
    sed -n '1,220p' "$dom_out"
    exit 1
  fi
done

test_pass "phase29cc_wsm_g4_min7_webdisplay_fixture_parity_vm: PASS (WSM-G4-min7 webdisplay fixture parity lock)"
