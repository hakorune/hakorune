#!/bin/bash
# phase29cc_wsm_p5_min10_legacy_hard_remove_lock_vm.sh
# Contract pin:
# - WSM-P5-min10: NYASH_WASM_ROUTE_POLICY hard-removes legacy values and accepts default only.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/phase29cc_wsm_p5_route_trace_common.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-169-wsm-p5-min10-legacy-hard-remove-lock-ssot.md"
if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_p5_min10_legacy_hard_remove_lock_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-P5-min10" \
  "hard-remove" \
  "NYASH_WASM_ROUTE_POLICY" \
  "allowed: default|rust_native" \
  "legacy-wasm-rust" \
  "[freeze:contract][wasm/route-policy]" \
  "wasm-boundary-lite"; do
  if ! grep -Fq "$needle" "$doc"; then
    test_fail "phase29cc_wsm_p5_min10_legacy_hard_remove_lock_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

run_wsm_p5_legacy_hard_remove_contract_tests

test_pass "phase29cc_wsm_p5_min10_legacy_hard_remove_lock_vm: PASS (WSM-P5-min10 legacy hard-remove lock)"
