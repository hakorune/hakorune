#!/bin/bash
# phase29cc_wsm_g4_min1_playground_console_baseline_vm.sh
# Contract pin:
# - WSM-G4-min1: nyash_playground console baseline migration lock
# - run loop + 1 fixture parity (demo-min markers)

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-171-wsm-g4-min1-nyash-playground-console-baseline-lock-ssot.md"
if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_g4_min1_playground_console_baseline_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-G4-min1" \
  "nyash_playground.html" \
  "Console loop only" \
  "wsm02d_demo_min_log" \
  "wasm-demo-g2" \
  "wasm-boundary-lite"; do
  if ! grep -Fq "$needle" "$doc"; then
    test_fail "phase29cc_wsm_g4_min1_playground_console_baseline_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

bash "$NYASH_ROOT/tools/smokes/v2/profiles/integration/phase29cc_wsm/g2_browser/phase29cc_wsm_g2_min1_bridge_build_vm.sh"
bash "$NYASH_ROOT/tools/smokes/v2/profiles/integration/phase29cc_wsm/g2_browser/phase29cc_wsm_g2_browser_run_vm.sh"
bash "$NYASH_ROOT/tools/smokes/v2/profiles/integration/apps/phase29cc_wsm02d_demo_min_boundary_vm.sh"

test_pass "phase29cc_wsm_g4_min1_playground_console_baseline_vm: PASS (WSM-G4-min1 playground console baseline lock)"
