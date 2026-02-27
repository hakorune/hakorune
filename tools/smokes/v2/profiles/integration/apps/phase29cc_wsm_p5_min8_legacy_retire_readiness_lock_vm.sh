#!/bin/bash
# phase29cc_wsm_p5_min8_legacy_retire_readiness_lock_vm.sh
# Contract pin:
# - WSM-P5-min8: legacy lane retire readiness criteria are docs-first fixed.
# - Route trace evidence for default(native) and legacy lane is reproducible.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-167-wsm-p5-min8-legacy-retire-readiness-lock-ssot.md"
if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_p5_min8_legacy_retire_readiness_lock_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-P5-min8" \
  "retire readiness" \
  "NYASH_WASM_ROUTE_TRACE" \
  "wasm-boundary-lite" \
  "legacy-wasm-rust"; do
  if ! grep -q "$needle" "$doc"; then
    test_fail "phase29cc_wsm_p5_min8_legacy_retire_readiness_lock_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

cargo test --features wasm-backend wasm_route_policy_name_contract -- --nocapture
cargo test --features wasm-backend wasm_demo_route_trace_reports_shape_id_for_native_default_contract -- --nocapture
cargo test --features wasm-backend wasm_demo_route_trace_reports_bridge_and_legacy_contract -- --nocapture

test_pass "phase29cc_wsm_p5_min8_legacy_retire_readiness_lock_vm: PASS (WSM-P5-min8 legacy retire readiness lock)"
