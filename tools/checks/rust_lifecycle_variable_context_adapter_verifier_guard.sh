#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

FACTS="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-adapter-facts-v0.json"
RESULT="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-adapter-verifier-result-v0.json"
CARD="docs/development/current/main/phases/phase-296x/296x-1474-HAKO-LIFECYCLE-VERIFIER-VARIABLE-CONTEXT-ADAPTER-FACTS-FIXTURE-001.md"

for path in \
  "$FACTS" \
  "$RESULT" \
  "$CARD" \
  docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-simple-map-plan-v0.json \
  docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-immutable-borrow-plan-v0.json \
  docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-snapshot-restore-plan-v0.json \
  docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-carrier-snapshot-plan-v0.json
do
  test -f "$path"
done

python3 - <<'PY'
import json
from pathlib import Path

base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")
facts = json.loads((base / "variable-context-adapter-facts-v0.json").read_text())
result = json.loads((base / "variable-context-adapter-verifier-result-v0.json").read_text())
plans = {
    path.name: json.loads(path.read_text())
    for path in [
        base / "variable-context-simple-map-plan-v0.json",
        base / "variable-context-immutable-borrow-plan-v0.json",
        base / "variable-context-snapshot-restore-plan-v0.json",
        base / "variable-context-carrier-snapshot-plan-v0.json",
    ]
}

assert result["schema_version"] == 0
assert result["kind"] == "HakoLifecycleVerifierResult"
assert result["mode"] == "passive_adapter_fixture"
assert result["subject"] == facts["subject"]
assert result["source_facts"] == "variable-context-adapter-facts-v0.json"
assert set(result["source_plans"]) == set(plans)
assert result["result"] == "VerifiedPlan"

field = next(row for row in facts["fields"] if row["id"] == "VariableContext.variable_map")
assert field["deterministic_order_required"] is True
assert field["drop_class"] == "TrivialMemory"

methods = {row["id"]: row for row in facts["methods"]}
assert methods["VariableContext::variable_map"]["receiver"]["borrow_escape"] == "Returned"
assert methods["VariableContext::variable_map"]["returned_reference"]["mutation_allowed"] is False
assert methods["VariableContext::variable_map_mut"]["receiver"]["borrow_kind"] == "UniqueWrite"
assert methods["VariableContext::variable_map_mut"]["returned_reference"]["mutation_allowed"] is True
assert methods["VariableContext::snapshot"]["ownership_effect"] == "CloneOwnedMap"
assert methods["VariableContext::restore"]["ownership_effect"] == "ReplaceOwned"

verified_facts = set(result["verified_facts"])
for required in [
    "VariableContext.variable_map.deterministic_order_required=true",
    "VariableContext.variable_map.drop_class=TrivialMemory",
    "VariableContext::variable_map.receiver.borrow_escape=Returned",
    "VariableContext::variable_map.returned_reference.mutation_allowed=false",
    "VariableContext::variable_map_mut.returned_reference.mutation_allowed=true",
    "VariableContext::snapshot.ownership_effect=CloneOwnedMap",
    "VariableContext::restore.ownership_effect=ReplaceOwned",
    "CarrierInfo::from_variable_map.required_access=ReadOnly",
]:
    assert required in verified_facts

surfaces = {row["surface"]: row for row in result["verified_plan_surfaces"]}
for surface in ["simple_map", "immutable_map_borrow", "snapshot_restore", "carrier_snapshot"]:
    assert surface in surfaces

assert plans["variable-context-simple-map-plan-v0.json"]["kind"] == "HakoLifecyclePlan"
assert plans["variable-context-immutable-borrow-plan-v0.json"]["kind"] == "HakoLifecyclePlan"
assert plans["variable-context-snapshot-restore-plan-v0.json"]["kind"] == "HakoLifecyclePlan"
assert plans["variable-context-carrier-snapshot-plan-v0.json"]["kind"] == "HakoLifecyclePlan"

denied = set(result["denied_boundaries"])
for required in [
    "VariableContext::variable_map_mut emitted as naked alias",
    "general verifier implementation",
    "lifecycle-aware converter emission",
    "full VariableContext parity",
    "MirBuilder-wide lifecycle parity",
]:
    assert required in denied

claims = result["claims"]
assert claims["emission_allowed"] is False
assert claims["verifier_implementation_started"] is False
assert claims["emitter_implementation_started"] is False
assert claims["converter_core_changed"] is False
assert claims["backend_behavior_changed"] is False
assert claims["full_variable_context_parity"] is False
assert claims["mirbuilder_wide_lifecycle"] is False
PY

grep -q "variable_context_adapter_verifier_fixture_exists=1" "$CARD"
grep -q "verifier_result_kind=VerifiedPlan" "$CARD"
grep -q "source_adapter_facts=variable-context-adapter-facts-v0.json" "$CARD"
grep -q "simple_map_plan_verified=1" "$CARD"
grep -q "immutable_borrow_plan_verified=1" "$CARD"
grep -q "snapshot_restore_plan_verified=1" "$CARD"
grep -q "carrier_snapshot_plan_verified=1" "$CARD"
grep -q "returned_mutable_borrow_denied=1" "$CARD"
grep -q "emission_allowed=0" "$CARD"
grep -q "verifier_implementation_started=0" "$CARD"
grep -q "emitter_implementation_started=0" "$CARD"
grep -q "converter_core_changed=0" "$CARD"
grep -q "backend_behavior_changed=0" "$CARD"

cat <<'REPORT'
output_contract=rust-lifecycle-variable-context-adapter-verifier-v0
variable_context_adapter_verifier_fixture_exists=1
verifier_result_kind=VerifiedPlan
source_adapter_facts=variable-context-adapter-facts-v0.json
simple_map_plan_verified=1
immutable_borrow_plan_verified=1
snapshot_restore_plan_verified=1
carrier_snapshot_plan_verified=1
returned_mutable_borrow_denied=1
emission_allowed=0
verifier_implementation_started=0
emitter_implementation_started=0
converter_core_changed=0
backend_behavior_changed=0
summary=ok
REPORT
