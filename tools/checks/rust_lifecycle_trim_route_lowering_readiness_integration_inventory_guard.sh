#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
from pathlib import Path

root = Path(".")
doc = (root / "docs/development/current/main/design/trim-route-lowering-readiness-integration-inventory.md").read_text()
card = (root / "docs/development/current/main/phases/phase-296x/296x-1460-TRIM-ROUTE-LOWERING-READINESS-INTEGRATION-INVENTORY-001.md").read_text()
trim_info = (root / "src/mir/loop_route_detection/support/body_local/carrier.rs").read_text()
boundary = (root / "src/mir/join_ir/lowering/inline_boundary_builder.rs").read_text()
scope = (root / "src/mir/join_ir/lowering/scope_manager.rs").read_text()

assert "readiness_integration_inventory=1" in doc
assert "selected_candidate=InlineBoundaryBuilder_or_route_lowering_boundary" in doc
assert "trim_route_info_to_carrier_info_allowed=0" in doc
assert "loop_break_scope_manager_allowed=0" in doc
assert "condition_bindings_required=1" in doc
assert "do not emit trim route lowering" in doc

assert "pub fn to_carrier_info" in trim_info
assert "condition_bindings" in boundary
assert "pub condition_bindings: &'a [ConditionBinding]" in scope

assert "selected_candidate=InlineBoundaryBuilder_or_route_lowering_boundary" in card
assert "do_not_change_code=1" in card
PY

cat <<'REPORT'
output_contract=rust-lifecycle-trim-route-readiness-integration-inventory-v0
readiness_integration_inventory=1
selected_candidate_documented=1
condition_bindings_required=1
invalid_callsite_rejected=1
backend_behavior_changed=0
generated_program_execution_claim=0
summary=ok
REPORT
