#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import json
from pathlib import Path

root = Path(".")
base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")

doc = (root / "docs/development/current/main/design/trim-route-lowering-proof-update.md").read_text()
card = (root / "docs/development/current/main/phases/phase-296x/296x-1454-TRIM-ROUTE-LOWERING-PROOF-UPDATE-001.md").read_text()
carrier = (root / "src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs").read_text()
scope = (root / "src/mir/join_ir/lowering/scope_manager.rs").read_text()

assert "trim_route_proof_updated=1" in doc
assert "missing_promoted_carrier_identity_retired_or_reclassified=1" in doc
assert "deny_reason=MissingExecutableTrimRouteLoweringImplementation" in doc
assert "backend_behavior_changed=0" in doc
assert "do not emit trim route lowering" in doc

assert "resolve_promoted_condition_binding_identity" in carrier
assert "pub condition_bindings: &'a [ConditionBinding]" in scope
assert ".resolve_promoted_condition_binding_identity(name, self.condition_bindings)" in scope

facts = json.loads((base / "trim-route-lowering-proof-update-facts-v1.json").read_text())
plan = json.loads((base / "trim-route-lowering-proof-update-plan-v1.json").read_text())
oracle = json.loads((base / "trim-route-lowering-proof-update-oracle-vectors-v1.json").read_text())

assert facts["schema_version"] == 1
candidate = facts["candidate_facts"][0]
assert candidate["condition_binding_identity_available"] is True
assert candidate["scope_manager_lookup_consumes_adapter"] is True
assert candidate["backend_trim_lowering_implementation"] is False
assert facts["claims"]["missing_promoted_carrier_identity_reclassified"] is True
assert facts["claims"]["executable_lowering_allow"] is False
assert facts["claims"]["backend_behavior_changed"] is False

row = plan["plans"][0]
assert row["plan_kind"] == "TrimRouteLoweringProofUpdate"
assert row["identity_decision"] == "AllowConditionBindingIdentity"
assert row["executable_decision"] == "Deny"
assert row["deny_reason"] == "MissingExecutableTrimRouteLoweringImplementation"
assert row["allowed_output"]["backend_lowering"] is False
assert plan["behavior"]["executable_lowering_allow"] is False
assert plan["behavior"]["backend_behavior_changed"] is False

vectors = {item["id"]: item for item in oracle["vectors"]}
ready = vectors["identity_ready_backend_missing"]
assert ready["expect"]["identity_decision"] == "AllowConditionBindingIdentity"
assert ready["expect"]["deny_reason"] == "MissingExecutableTrimRouteLoweringImplementation"
assert vectors["identity_missing"]["expect"]["deny_reason"] == "MissingPromotedCarrierIdentity"
assert oracle["claims"]["executable_lowering_allow"] is False

assert "trim_route_proof_updated=1" in card
assert "do_not_emit_trim_route_lowering=1" in card
PY

cat <<'REPORT'
output_contract=rust-lifecycle-trim-route-lowering-proof-update-v1
trim_route_proof_updated=1
missing_promoted_carrier_identity_retired_or_reclassified=1
scope_manager_condition_binding_input_consumed_as_proof=1
executable_lowering_allow=0
deny_reason=MissingExecutableTrimRouteLoweringImplementation
backend_behavior_changed=0
generated_program_execution_claim=0
summary=ok
REPORT
