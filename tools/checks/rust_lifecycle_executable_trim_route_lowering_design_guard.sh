#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
from pathlib import Path

root = Path(".")
doc = (root / "docs/development/current/main/design/executable-trim-route-lowering-implementation-design.md").read_text()
card = (root / "docs/development/current/main/phases/phase-296x/296x-1456-EXECUTABLE-TRIM-ROUTE-LOWERING-IMPLEMENTATION-DESIGN-001.md").read_text()
proof = (root / "docs/development/current/main/design/trim-route-lowering-proof-update.md").read_text()

assert "implementation_design_documented=1" in doc
assert "implementation_shape=readiness_gate_before_backend_lowering" in doc
assert "identity_proof_required=1" in doc
assert "condition_bindings_input_required=1" in doc
assert "backend_lowering_implementation_started=0" in doc
assert "do not emit backend trim route lowering" in doc
assert "do not claim generated program execution" in doc

assert "implementation_shape=readiness_gate_before_backend_lowering" in card
assert "do_not_emit_trim_route_lowering=1" in card
assert "backend_lowering_implementation_started=0" in card

assert "deny_reason=MissingExecutableTrimRouteLoweringImplementation" in proof
PY

cat <<'REPORT'
output_contract=rust-lifecycle-executable-trim-route-lowering-design-v0
implementation_design_documented=1
implementation_shape=readiness_gate_before_backend_lowering
identity_proof_required=1
condition_bindings_input_required=1
backend_lowering_implementation_started=0
backend_behavior_changed=0
summary=ok
REPORT
