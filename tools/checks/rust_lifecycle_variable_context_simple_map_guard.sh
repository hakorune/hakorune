#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import json
from pathlib import Path

base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")
facts = json.loads((base / "variable-context-simple-map-facts-v0.json").read_text())
plan = json.loads((base / "variable-context-simple-map-plan-v0.json").read_text())
oracle = json.loads((base / "variable-context-simple-map-oracle-vectors-v0.json").read_text())

assert facts["schema_version"] == 0
assert facts["kind"] == "RustLifecycleFacts"
assert facts["subject"].endswith("VariableContext.simple_map")

field = next(row for row in facts["field_facts"] if row["id"] == "VariableContext.variable_map")
assert field["rust_type"] == "BTreeMap<String, ValueId>"
assert field["deterministic_order_required"] is True
assert field["drop_fact"] == "TrivialMemory"

method_ids = {row["id"] for row in facts["method_facts"]}
for method in [
    "VariableContext::lookup",
    "VariableContext::contains",
    "VariableContext::len",
    "VariableContext::is_empty",
    "VariableContext::insert",
    "VariableContext::remove",
]:
    assert method in method_ids

excluded_methods = {row["id"] for row in facts["excluded_methods"]}
for method in [
    "VariableContext::variable_map",
    "VariableContext::variable_map_mut",
    "VariableContext::snapshot",
    "VariableContext::restore",
]:
    assert method in excluded_methods

assert plan["schema_version"] == 0
assert plan["kind"] == "HakoLifecyclePlan"
assert plan["source_facts"] == "variable-context-simple-map-facts-v0.json"

plans = {row["id"]: row for row in plan["plans"]}
assert plans["VariableContext.variable_map"]["plan_kind"] == "OrderedMapBox"
assert "VariableContext.variable_map.deterministic_order_required=true" in plans["VariableContext.variable_map"]["required_facts"]
assert plans["VariableContext::insert"]["plan_kind"] == "TransferOwned"
assert plans["VariableContext::insert"]["overwrite_policy"] == "allowed_when_previous_value_drop_is_TrivialMemory"
assert plans["VariableContext::lookup"]["return_plan"] == "Immediate"

excluded = set(plan["excluded"])
for item in [
    "VariableContext::variable_map",
    "VariableContext::variable_map_mut",
    "VariableContext::snapshot",
    "VariableContext::restore",
    "CarrierInfo::from_variable_map",
    "PHI planner integration",
]:
    assert item in excluded

behavior = plan["behavior"]
assert behavior["general_resolver_implemented"] is False
assert behavior["converter_emission_added"] is False
assert behavior["rust_lifetime_syntax_added"] is False

assert oracle["schema_version"] == 0
assert oracle["kind"] == "RustOracleVectors"
assert oracle["subject"] == facts["subject"]

oracle_ops = {
    op["op"]
    for vector in oracle["vectors"]
    for op in vector["operations"]
}
for op in [
    "new",
    "lookup",
    "contains",
    "len",
    "is_empty",
    "insert",
    "remove",
    "iteration_order",
]:
    assert op in oracle_ops

assert oracle["drop_oracle"]["drop_observable"] is False
assert oracle["drop_oracle"]["required_fact"] == "VariableContext.drop_fact=TrivialMemory"
assert oracle["promotion_scope"]["hako_authority"] == "VariableContext simple map only"
assert oracle["promotion_scope"]["full_variable_context_claim"] is False
assert oracle["promotion_scope"]["mirbuilder_wide_claim"] is False

excluded_vectors = set(oracle["excluded_vectors"])
for item in [
    "variable_map_returned_borrow",
    "variable_map_mut_returned_borrow",
    "snapshot",
    "restore",
    "carrier_extraction",
    "phi_planner_integration",
]:
    assert item in excluded_vectors
PY

cat <<'REPORT'
output_contract=rust-lifecycle-variable-context-simple-map-v0
variable_context_simple_map_facts_fixture=green
variable_context_simple_map_plan_fixture=green
variable_context_simple_map_oracle_vectors=green
returned_map_methods_excluded=green
snapshot_restore_excluded=green
carrier_consumers_excluded=green
ordered_map_projection_requires_deterministic_order_fact=green
memory_drop_erased_only_with_TrivialMemory=green
variable_context_simple_map_plan_matches_oracle=green
hako_authority_promoted_for_VariableContext_simple_map_only=green
general_resolver_implemented=0
converter_emission_added=0
rust_lifetime_syntax_added=0
summary=ok
REPORT
