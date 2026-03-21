#!/bin/bash
# phase29cc_wsm_p7_min3_two_demo_lock_vm.sh
# Contract pin:
# - WSM-P7-min3: two demo fixtures from projects/nyash-wasm remain green on default route.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-186-wsm-p7-min3-two-demo-lock-ssot.md"
if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_p7_min3_two_demo_lock_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-P7-min3" \
  "projects/nyash-wasm" \
  "webcanvas" \
  "canvas_advanced" \
  "phase29cc_wsm_g4_min5_headless_two_examples_vm.sh"; do
  if ! grep -Fq "$needle" "$doc"; then
    test_fail "phase29cc_wsm_p7_min3_two_demo_lock_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

bash "$NYASH_ROOT/tools/smokes/v2/profiles/integration/phase29cc_wsm/g4/phase29cc_wsm_g4_min5_headless_two_examples_vm.sh"

test_pass "phase29cc_wsm_p7_min3_two_demo_lock_vm: PASS (WSM-P7-min3 two-demo lock)"
