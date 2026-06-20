#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import json
from pathlib import Path

base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")
result = json.loads((base / "carrier-info-merge-from-emitter-verifier-result-v0.json").read_text())
surface = (base / "carrier-info-merge-from-emitter-surface-v0.hako").read_text()

assert result["schema_version"] == 0
assert result["kind"] == "HakoLifecycleVerifierResult"
assert result["mode"] == "emitter_probe_fixture"
assert result["result"] == "VerifiedPlan"

facts_path = base / result["source_facts"]
plan_path = base / result["source_plan"]
assert facts_path.exists(), result["source_facts"]
assert plan_path.exists(), result["source_plan"]

plan = json.loads(plan_path.read_text())
assert plan["kind"] == "HakoLifecyclePlan"
assert plan["subject"] == result["subject"]
assert plan["plans"][0]["plan_kind"] == "OwnedCarrierInfoMerge"

claims = result["claims"]
assert claims["emission_allowed"] is True
assert claims["emission_scope"] == "CarrierInfo::merge_from only"
assert claims["backend_behavior_changed"] is False
assert claims["resolver_selection_owner"] is False
assert claims["full_variable_context_parity"] is False
assert claims["mirbuilder_wide_lifecycle"] is False

for token in [
    "lifecycle-emitter-probe-v0",
    "subject: CarrierInfo::merge_from",
    "plan_kind: OwnedCarrierInfoMerge",
    "source_plan: carrier-info-merge-from-plan-v0.json",
    "verifier_result: carrier-info-merge-from-emitter-verifier-result-v0.json",
    "function CarrierInfo_merge_from_lifecycle_surface",
    "return receiver",
]:
    assert token in surface, token

for forbidden in [
    "Verified boundary: join_id producer",
    "full_variable_context_parity: 1",
    "mirbuilder_wide_lifecycle: 1",
    "backend_behavior_changed: 1",
]:
    assert forbidden not in surface, forbidden

assert "Denied boundary: no join_id producer is emitted here." in surface
assert "Denied boundary: no trim_helper lifecycle owner is claimed here." in surface
assert "Denied boundary: no general converter rewrite is claimed here." in surface
PY

cat <<'REPORT'
output_contract=rust-lifecycle-emitter-probe-v0
emitter_probe_surface=green
verified_result_required=green
emission_scope=CarrierInfo::merge_from_only
backend_behavior_changed=0
resolver_selection_owner=0
full_variable_context_parity=0
mirbuilder_wide_lifecycle=0
join_id_producer_emitted=0
general_converter_rewrite=0
summary=ok
REPORT
