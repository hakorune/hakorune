#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
from pathlib import Path

root = Path(".")
constructors = (root / "src/mir/join_ir/lowering/inline_boundary/constructors.rs").read_text()
tests = (root / "src/mir/join_ir/lowering/inline_boundary/tests.rs").read_text()
card = (root / "docs/development/current/main/phases/phase-296x/296x-1462-ROUTE-BOUNDARY-TRIM-READINESS-INTEGRATION-PROBE-001.md").read_text()

assert "pub fn trim_route_lowering_readiness" in constructors
assert "decide_trim_route_lowering_readiness" in constructors
assert "self.condition_bindings.as_slice()" in constructors
assert "route_boundary_trim_readiness_allows_ready_inputs" in tests
assert "route_boundary_trim_readiness_denies_missing_identity" in tests
assert "TrimRouteLoweringReadinessDecision::Ready" in tests
assert "MissingConditionBindingIdentity" in tests

assert "route_boundary_readiness_probe_exists=1" in card
assert "probe_consumes_carrier_info_and_condition_bindings=1" in card
assert "backend_lowering_implementation_started=0" in card
assert "do_not_emit_trim_route_lowering=1" in card
PY

cat <<'REPORT'
output_contract=rust-lifecycle-route-boundary-trim-readiness-probe-v0
route_boundary_readiness_probe_exists=1
probe_consumes_carrier_info_and_condition_bindings=1
probe_calls_trim_readiness_gate=1
probe_has_ready_and_deny_tests=1
backend_lowering_implementation_started=0
generated_program_execution_claim=0
summary=ok
REPORT
