#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import json
from pathlib import Path

base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")
facts = json.loads((base / "variable-context-carrier-snapshot-facts-v0.json").read_text())
plan = json.loads((base / "variable-context-carrier-snapshot-plan-v0.json").read_text())
oracle = json.loads((base / "variable-context-carrier-snapshot-oracle-vectors-v0.json").read_text())

assert facts["schema_version"] == 0
assert facts["kind"] == "RustLifecycleFacts"
assert facts["subject"].endswith("CarrierInfo.from_variable_map")
assert "variable-context-snapshot-restore-facts-v0.json" in facts["base_facts"]

method = facts["method_fact"]
assert method["id"] == "CarrierInfo::from_variable_map"
assert method["operation"] == "CarrierSnapshotFromOwnedMap"
assert method["input_snapshot"]["ownership"] == "OwnedReadSnapshotProjection"
assert method["input_snapshot"]["access"] == "read"
assert method["input_snapshot"]["escapes"] is False
assert method["map_requirements"]["deterministic_order_required"] is True
assert method["map_requirements"]["value_drop_fact"] == "TrivialMemory"
assert method["output"]["owns_carrier_names"] is True
assert method["output"]["copies_value_ids"] is True
assert method["output"]["value_id_copy_kind"] == "ImmediateValue"
assert method["output"]["join_id_initialized"] is False
denied_methods = {row["id"]: row for row in facts["denied_methods"]}
assert denied_methods["VariableContext::variable_map"]["deny_reason"] == "ReturnedReadBorrow"

for item in [
    "CarrierInfo::with_explicit_carriers",
    "join_id lifecycle",
    "PHI planner integration",
]:
    assert item in set(facts["denied_followups"])

assert plan["schema_version"] == 0
assert plan["kind"] == "HakoLifecyclePlan"
entry = plan["plans"][0]
assert entry["id"] == "CarrierInfo::from_variable_map"
assert entry["plan_kind"] == "CarrierSnapshotFromOwnedMap"
assert entry["mutation_policy"] == "none"
assert entry["publication_policy"] == "does_not_publish_variable_map"
assert entry["output_policy"]["carrier_names"] == "owned_strings"
assert entry["output_policy"]["host_id"] == "copied_ValueId"
assert entry["output_policy"]["join_id"] == "None_uninitialized"
assert "input_snapshot.ownership=OwnedReadSnapshotProjection" in entry["required_facts"]
assert "map_requirements.deterministic_order_required=true" in entry["required_facts"]
assert "output.value_id_copy_kind=ImmediateValue" in entry["required_facts"]

behavior = plan["behavior"]
assert behavior["general_resolver_implemented"] is False
assert behavior["converter_emission_added"] is False
assert behavior["phi_join_id_claim"] is False
assert behavior["full_variable_context_claim"] is False

vector = oracle["vectors"][0]
assert vector["loop_var_name"] == "i"
assert vector["expect"]["loop_var_id"] == 5
assert vector["expect"]["carrier_count"] == 2
carrier_names = [row["name"] for row in vector["expect"]["carriers"]]
assert carrier_names == ["count", "sum"]
assert all(row["join_id"] is None for row in vector["expect"]["carriers"])
assert "owned_read_snapshot_projection" in vector["requires"]
assert "deterministic_order_required=true" in vector["requires"]

denied = set(oracle["denied_vectors"])
for item in ["with_explicit_carriers", "join_id_assignment", "phi_planner_integration"]:
    assert item in denied

scope = oracle["promotion_scope"]
assert scope["hako_authority"] == "CarrierInfo::from_variable_map snapshot only"
assert scope["phi_join_id_claim"] is False
assert scope["full_variable_context_claim"] is False
assert scope["mirbuilder_wide_claim"] is False
PY

cat <<'REPORT'
output_contract=rust-lifecycle-variable-context-carrier-snapshot-v0
carrier_snapshot_facts_fixture=green
carrier_snapshot_plan_fixture=green
carrier_snapshot_oracle_vectors=green
requires_owned_read_snapshot_projection=green
returned_read_borrow_deny=green
requires_deterministic_order=green
requires_ValueId_TrivialMemory=green
mutates_VariableContext=0
publishes_variable_map=0
PHI_join_id_claim=0
general_resolver_implemented=0
summary=ok
REPORT
