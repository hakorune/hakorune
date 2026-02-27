#!/bin/bash
# phase29cc_wsm_g4_min6_gate_promotion_closeout_vm.sh
# Contract pin:
# - WSM-G4-min6: G4 closeout gate promotion lock

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-176-wsm-g4-min6-gate-promotion-closeout-lock-ssot.md"
gate="$NYASH_ROOT/tools/checks/dev_gate.sh"

if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_g4_min6_gate_promotion_closeout_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-G4-min6" \
  "wasm-demo-g2" \
  "wasm-boundary-lite" \
  "monitor-only"; do
  if ! grep -Fq "$needle" "$doc"; then
    test_fail "phase29cc_wsm_g4_min6_gate_promotion_closeout_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

if ! grep -Fq "phase29cc_wsm_g4_min5_headless_two_examples_vm.sh" "$gate"; then
  test_fail "phase29cc_wsm_g4_min6_gate_promotion_closeout_vm: dev_gate missing min5 step"
  exit 1
fi
if ! grep -Fq "phase29cc_wsm_g4_min6_gate_promotion_closeout_vm.sh" "$gate"; then
  test_fail "phase29cc_wsm_g4_min6_gate_promotion_closeout_vm: dev_gate missing min6 step"
  exit 1
fi

bash "$NYASH_ROOT/tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g4_min5_headless_two_examples_vm.sh"

test_pass "phase29cc_wsm_g4_min6_gate_promotion_closeout_vm: PASS (WSM-G4-min6 gate promotion closeout lock)"
