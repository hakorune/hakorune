#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import json
from pathlib import Path

root = Path(".")
base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")

doc = (root / "docs/development/current/main/design/condition-binding-promoted-identity-proof-probe.md").read_text()
policy = (root / "docs/development/current/main/design/promoted-carrier-identity-policy-decision.md").read_text()
condition_env = (root / "src/mir/join_ir/lowering/condition_env.rs").read_text()
boundary = (root / "src/mir/join_ir/lowering/inline_boundary_builder.rs").read_text()
carrier_impl = (root / "src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs").read_text()

assert "condition_binding_identity_proof_probe=1" in doc
assert "allow_identity_candidate=1" in doc
assert "resolution_rewrite_added=0" in doc
assert "trim_route_lowering_added=0" in doc
assert "selected_policy=condition_binding_identity" in policy
assert "pub struct ConditionBinding" in condition_env
assert "pub join_value: ValueId" in condition_env
assert "ParamRole::Condition" in boundary
assert "get_condition_binding" in boundary
assert "pub fn resolve_promoted_join_id" in carrier_impl

facts = json.loads((base / "condition-binding-promoted-identity-facts-v0.json").read_text())
plan = json.loads((base / "condition-binding-promoted-identity-plan-v0.json").read_text())
oracle = json.loads((base / "condition-binding-promoted-identity-oracle-vectors-v0.json").read_text())

assert facts["schema_version"] == 0
assert facts["kind"] == "RustLifecycleFacts"
candidate = facts["candidate_facts"][0]
assert candidate["id"] == "ConditionBindingPromotedIdentity::trim_ch"
assert candidate["identity_candidate"] == "ValueId(200)"
assert facts["claims"]["allow_identity_candidate"] is True
assert facts["claims"]["resolution_rewrite_added"] is False
assert facts["claims"]["join_id_producer_added"] is False
assert facts["claims"]["trim_route_lowering_added"] is False

row = plan["plans"][0]
assert row["plan_kind"] == "ConditionBindingPromotedIdentityProof"
assert row["decision"] == "AllowIdentityCandidate"
assert row["identity_source"] == "ConditionBinding.join_value"
assert row["allowed_output"]["resolution_rewrite"] is False
assert row["allowed_output"]["trim_route_lowering"] is False
assert plan["behavior"]["allow_identity_candidate"] is True
assert plan["behavior"]["backend_behavior_changed"] is False

vectors = {row["id"]: row for row in oracle["vectors"]}
assert vectors["allow_identity_candidate"]["expect"]["decision"] == "AllowIdentityCandidate"
assert vectors["allow_identity_candidate"]["expect"]["identity_value"] == "ValueId(200)"
assert vectors["deny_missing_condition_binding"]["expect"]["decision"] == "DenyMissingConditionBindingIdentity"
assert vectors["deny_promoted_name_mismatch"]["expect"]["decision"] == "DenyPromotedNameMismatch"
assert oracle["claims"]["resolution_rewrite_added"] is False
assert oracle["claims"]["trim_route_lowering_added"] is False
PY

cat <<'REPORT'
output_contract=rust-lifecycle-condition-binding-promoted-identity-v0
condition_binding_identity_proof_probe=1
allow_identity_candidate=1
deny_missing_condition_binding_identity=1
deny_promoted_name_mismatch=1
resolution_rewrite_added=0
trim_route_lowering_added=0
backend_behavior_changed=0
summary=ok
REPORT
