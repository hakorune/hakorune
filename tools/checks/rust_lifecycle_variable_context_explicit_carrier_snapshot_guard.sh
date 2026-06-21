#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import json
from pathlib import Path

base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")
facts = json.loads((base / "variable-context-explicit-carrier-snapshot-facts-v0.json").read_text())
plan = json.loads((base / "variable-context-explicit-carrier-snapshot-plan-v0.json").read_text())
oracle = json.loads((base / "variable-context-explicit-carrier-snapshot-oracle-vectors-v0.json").read_text())

assert facts["schema_version"] == 0
assert facts["kind"] == "RustLifecycleFacts"
assert facts["subject"].endswith("CarrierInfo.with_explicit_carriers")
assert "variable-context-carrier-snapshot-facts-v0.json" in facts["base_facts"]

method = facts["method_fact"]
assert method["id"] == "CarrierInfo::with_explicit_carriers"
assert method["operation"] == "ExplicitCarrierSnapshotFromOwnedMap"
assert method["input_snapshot"]["ownership"] == "OwnedReadSnapshotProjection"
assert method["input_snapshot"]["escapes"] is False
assert method["loop_var_id"]["copy_kind"] == "ImmediateValue"
assert method["carrier_names"]["ownership"] == "owned_strings"
assert method["carrier_names"]["missing_carrier_policy"] == "fail_fast"
assert method["map_requirements"]["value_drop_fact"] == "TrivialMemory"
assert method["output"]["join_id_initialized"] is False

for item in ["join_id lifecycle", "promoted_body_locals lifecycle", "PHI planner integration"]:
    assert item in set(facts["denied_followups"])
denied_methods = {row["id"]: row for row in facts["denied_methods"]}
assert denied_methods["VariableContext::variable_map"]["deny_reason"] == "ReturnedReadBorrow"

entry = plan["plans"][0]
assert entry["id"] == "CarrierInfo::with_explicit_carriers"
assert entry["plan_kind"] == "ExplicitCarrierSnapshotFromOwnedMap"
assert entry["mutation_policy"] == "none"
assert entry["publication_policy"] == "does_not_publish_variable_map"
assert entry["missing_carrier_policy"] == "fail_fast"
assert entry["output_policy"]["carrier_names"] == "owned_strings"
assert entry["output_policy"]["join_id"] == "None_uninitialized"
assert "input_snapshot.ownership=OwnedReadSnapshotProjection" in entry["required_facts"]
assert "carrier_names.ownership=owned_strings" in entry["required_facts"]
assert "carrier_names.missing_carrier_policy=fail_fast" in entry["required_facts"]

behavior = plan["behavior"]
assert behavior["general_resolver_implemented"] is False
assert behavior["converter_emission_added"] is False
assert behavior["phi_join_id_claim"] is False
assert behavior["full_variable_context_claim"] is False

vectors = {row["id"]: row for row in oracle["vectors"]}
ok = vectors["loop_var_i_with_requested_carriers"]
assert ok["carrier_names"] == ["sum", "count"]
assert [row["name"] for row in ok["expect"]["carriers"]] == ["count", "sum"]
assert all(row["join_id"] is None for row in ok["expect"]["carriers"])
assert "owned_read_snapshot_projection" in ok["requires"]
assert "requested_names_owned" in ok["requires"]
assert "missing_carrier_fail_fast" in ok["requires"]

missing = vectors["missing_requested_carrier_fails"]
assert missing["expect_error"] == "Carrier variable 'missing' not found in variable_map"

scope = oracle["promotion_scope"]
assert scope["hako_authority"] == "CarrierInfo::with_explicit_carriers snapshot only"
assert scope["phi_join_id_claim"] is False
assert scope["full_variable_context_claim"] is False
assert scope["mirbuilder_wide_claim"] is False
PY

cat <<'REPORT'
output_contract=rust-lifecycle-variable-context-explicit-carrier-snapshot-v0
explicit_carrier_snapshot_facts_fixture=green
explicit_carrier_snapshot_plan_fixture=green
explicit_carrier_snapshot_oracle_vectors=green
requires_owned_read_snapshot_projection=green
returned_read_borrow_deny=green
requires_requested_names_owned=green
missing_carrier_fail_fast_preserved=green
mutates_VariableContext=0
publishes_variable_map=0
PHI_join_id_claim=0
general_resolver_implemented=0
summary=ok
REPORT
