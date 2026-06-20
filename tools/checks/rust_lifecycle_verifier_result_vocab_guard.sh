#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import json
from pathlib import Path

base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")
result = json.loads((base / "carrier-info-merge-from-verifier-result-v0.json").read_text())

assert result["schema_version"] == 0
assert result["kind"] == "HakoLifecycleVerifierResult"
assert result["mode"] == "passive_fixture"
assert result["result"] == "VerifiedPlan"

facts_path = base / result["source_facts"]
plan_path = base / result["source_plan"]
assert facts_path.exists(), result["source_facts"]
assert plan_path.exists(), result["source_plan"]

facts = json.loads(facts_path.read_text())
plan = json.loads(plan_path.read_text())
assert facts["kind"] == "RustLifecycleFacts"
assert plan["kind"] == "HakoLifecyclePlan"
assert facts["subject"] == result["subject"]
assert plan["subject"] == result["subject"]

verified_facts = set(result["verified_facts"])
plan_required = set(plan["plans"][0]["required_facts"])
assert verified_facts == plan_required

verified_boundaries = set(result["verified_boundaries"])
for item in [
    "OwnedCarrierInfoMerge",
    "mutate_receiver_only",
    "other_not_mutated",
    "does_not_publish_variable_map",
]:
    assert item in verified_boundaries

denied = set(result["denied_boundaries"])
for item in [
    "join_id producer",
    "trim_helper lifecycle owner",
    "promoted_body_locals lifecycle owner",
    "general resolver selection owner",
]:
    assert item in denied

claims = result["claims"]
assert claims["emission_allowed"] is False
assert claims["backend_behavior_changed"] is False
assert claims["resolver_selection_owner"] is False
assert claims["full_variable_context_parity"] is False
assert claims["mirbuilder_wide_lifecycle"] is False
PY

cat <<'REPORT'
output_contract=rust-lifecycle-verifier-result-vocab-v0
verifier_result_fixture=green
result_kind=VerifiedPlan
source_facts_exists=green
source_plan_exists=green
verified_facts_match_plan_required_facts=green
emission_allowed=0
backend_behavior_changed=0
resolver_selection_owner=0
full_variable_context_parity=0
mirbuilder_wide_lifecycle=0
summary=ok
REPORT
