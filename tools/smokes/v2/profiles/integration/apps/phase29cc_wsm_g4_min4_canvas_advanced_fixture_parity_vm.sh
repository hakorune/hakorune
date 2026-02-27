#!/bin/bash
# phase29cc_wsm_g4_min4_canvas_advanced_fixture_parity_vm.sh
# Contract pin:
# - WSM-G4-min4: canvas_advanced fixture parity lock
# - source marker lock + fixture compile parity

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-174-wsm-g4-min4-canvas-advanced-fixture-parity-lock-ssot.md"
html="$NYASH_ROOT/projects/nyash-wasm/nyash_playground.html"
fixture="$NYASH_ROOT/apps/tests/phase29cc_wsm_g4_min4_canvas_advanced_fixture_min.hako"

if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_g4_min4_canvas_advanced_fixture_parity_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-G4-min4" \
  "canvas_advanced" \
  "wsm_g4_min4_canvas_advanced_marker_1" \
  "wsm_g4_min4_canvas_advanced_marker_2"; do
  if ! grep -Fq "$needle" "$doc"; then
    test_fail "phase29cc_wsm_g4_min4_canvas_advanced_fixture_parity_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

for needle in \
  "wsm_g4_min4_canvas_advanced_marker_1" \
  "wsm_g4_min4_canvas_advanced_marker_2"; do
  if ! grep -Fq "$needle" "$fixture"; then
    test_fail "phase29cc_wsm_g4_min4_canvas_advanced_fixture_parity_vm: missing keyword in fixture: $needle"
    exit 1
  fi
done

for needle in \
  "wsm_g4_min4_canvas_advanced_source_lock" \
  "wsm_g4_min4_canvas_advanced_marker_1" \
  "wsm_g4_min4_canvas_advanced_marker_2"; do
  if ! grep -Fq "$needle" "$html"; then
    test_fail "phase29cc_wsm_g4_min4_canvas_advanced_fixture_parity_vm: missing keyword in nyash_playground.html: $needle"
    exit 1
  fi
done

bash "$NYASH_ROOT/tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g4_min3_webcanvas_fixture_parity_vm.sh"

set +e
output=$(cd "$NYASH_ROOT" && cargo test --features wasm-backend wasm_demo_g4_min4_canvas_advanced_fixture_compile_to_wat_contract -- --nocapture 2>&1)
rc=$?
set -e
if [ "$rc" -ne 0 ]; then
  test_fail "phase29cc_wsm_g4_min4_canvas_advanced_fixture_parity_vm: cargo test failed (rc=$rc)"
  printf '%s\n' "$output" | sed -n '1,200p'
  exit 1
fi
if ! printf '%s\n' "$output" | grep -q "wasm_demo_g4_min4_canvas_advanced_fixture_compile_to_wat_contract"; then
  test_fail "phase29cc_wsm_g4_min4_canvas_advanced_fixture_parity_vm: expected test marker missing"
  printf '%s\n' "$output" | sed -n '1,200p'
  exit 1
fi

test_pass "phase29cc_wsm_g4_min4_canvas_advanced_fixture_parity_vm: PASS (WSM-G4-min4 canvas_advanced fixture parity lock)"
