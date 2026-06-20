#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import json
from pathlib import Path

root = Path(".")
base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")

doc = (root / "docs/development/current/main/design/trim-route-lowering-decision-probe.md").read_text()
trim_src = (root / "src/mir/loop_route_detection/support/trim.rs").read_text()
carrier_src = (root / "src/mir/loop_route_detection/support/body_local/carrier.rs").read_text()
info_src = (root / "src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs").read_text()

assert "metadata_candidate_allow=1" in doc
assert "executable_lowering_allow=0" in doc
assert "deny_reason=MissingPromotedCarrierIdentity" in doc
assert "do not emit trim route lowering" in doc

assert "pub fn has_valid_structure" in trim_src
assert "pub fn carrier_type" in trim_src
assert "pub fn initial_value" in trim_src
assert "carrier_info.trim_helper = Some" in carrier_src
assert ".promoted_body_locals" in carrier_src
assert "pub fn resolve_promoted_join_id" in info_src
assert "if let Some(join_id) = carrier.join_id" in info_src

facts = json.loads((base / "trim-route-lowering-decision-facts-v0.json").read_text())
plan = json.loads((base / "trim-route-lowering-decision-plan-v0.json").read_text())
oracle = json.loads((base / "trim-route-lowering-decision-oracle-vectors-v0.json").read_text())

assert facts["schema_version"] == 0
assert facts["kind"] == "RustLifecycleFacts"
candidate = facts["candidate_facts"][0]
assert candidate["candidate_kind"] == "TrimRouteMetadataCandidate"
assert candidate["trim_helper_present"] is True
assert candidate["metadata_candidate_allow"] is True
assert facts["claims"]["metadata_candidate_allow"] is True
assert facts["claims"]["executable_lowering_allow"] is False
assert facts["claims"]["backend_behavior_changed"] is False
deny = facts["denied_dependencies"][0]
assert deny["reason"] == "MissingPromotedCarrierIdentity"
assert deny["join_id_producer"] is False

row = plan["plans"][0]
assert row["plan_kind"] == "TrimRouteLoweringDecisionProbe"
assert row["metadata_decision"] == "AllowMetadataCandidate"
assert row["executable_decision"] == "Deny"
assert row["deny_reason"] == "MissingPromotedCarrierIdentity"
assert row["allowed_output"]["backend_lowering"] is False
assert plan["behavior"]["executable_lowering_allow"] is False
assert plan["behavior"]["join_id_producer"] is False
assert plan["behavior"]["backend_behavior_changed"] is False

vectors = {row["id"]: row for row in oracle["vectors"]}
valid = vectors["valid_metadata_missing_identity"]
assert valid["expect"]["metadata_decision"] == "AllowMetadataCandidate"
assert valid["expect"]["executable_decision"] == "Deny"
assert valid["expect"]["deny_reason"] == "MissingPromotedCarrierIdentity"
assert vectors["invalid_metadata_empty_whitespace"]["expect"]["deny_reason"] == "InvalidTrimMetadata"
assert vectors["metadata_absent"]["expect"]["deny_reason"] == "NoTrimHelper"
assert oracle["claims"]["executable_lowering_allow"] is False
PY

cat <<'REPORT'
output_contract=rust-lifecycle-trim-route-lowering-decision-v0
metadata_candidate_allow=1
executable_lowering_allow=0
deny_reason=MissingPromotedCarrierIdentity
join_id_producer=0
backend_behavior_changed=0
generated_program_execution_claim=0
summary=ok
REPORT
