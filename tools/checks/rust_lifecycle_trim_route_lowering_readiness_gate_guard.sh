#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
from pathlib import Path

root = Path(".")
trim = (root / "src/mir/loop_route_detection/support/trim.rs").read_text()
card = (root / "docs/development/current/main/phases/phase-296x/296x-1458-TRIM-ROUTE-LOWERING-READINESS-GATE-001.md").read_text()
design = (root / "docs/development/current/main/design/executable-trim-route-lowering-implementation-design.md").read_text()

assert "pub enum TrimRouteLoweringReadinessDeny" in trim
assert "pub enum TrimRouteLoweringReadinessDecision" in trim
assert "pub fn decide_trim_route_lowering_readiness" in trim
assert "TrimRouteLoweringReadinessDeny::NoTrimHelper" in trim
assert "TrimRouteLoweringReadinessDeny::InvalidTrimMetadata" in trim
assert "TrimRouteLoweringReadinessDeny::MissingConditionBindingIdentity" in trim
assert ".resolve_promoted_condition_binding_identity(&helper.original_var, condition_bindings)" in trim

assert "trim_route_lowering_readiness_allows_identity_ready_inputs" in trim
assert "trim_route_lowering_readiness_denies_missing_trim_helper" in trim
assert "trim_route_lowering_readiness_denies_invalid_trim_metadata" in trim
assert "trim_route_lowering_readiness_denies_missing_condition_binding_identity" in trim

assert "readiness_gate_exists=1" in card
assert "backend_lowering_implementation_started=0" in card
assert "do_not_emit_trim_route_lowering=1" in card

assert "implementation_shape=readiness_gate_before_backend_lowering" in design
PY

cat <<'REPORT'
output_contract=rust-lifecycle-trim-route-lowering-readiness-gate-v0
readiness_gate_exists=1
readiness_allows_identity_ready_inputs=1
readiness_denies_missing_trim_helper=1
readiness_denies_invalid_trim_metadata=1
readiness_denies_missing_condition_binding_identity=1
backend_lowering_implementation_started=0
generated_program_execution_claim=0
summary=ok
REPORT
