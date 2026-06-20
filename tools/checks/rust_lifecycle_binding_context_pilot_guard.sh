#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

FACTS="docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-facts-v0.json"
PLAN="docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-plan-v0.json"
ORACLE="docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-oracle-vectors-v0.json"

python3 - <<'PY'
import json
from pathlib import Path

facts = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-facts-v0.json").read_text())
plan = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-plan-v0.json").read_text())
oracle = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-oracle-vectors-v0.json").read_text())

assert facts["schema_version"] == 0
assert facts["kind"] == "RustLifecycleFacts"
assert facts["subject"] == "hakorune_mir_builder::binding_context::BindingContext"

field = next(row for row in facts["field_facts"] if row["id"] == "BindingContext.binding_map")
assert field["rust_type"] == "BTreeMap<String, BindingId>"
assert field["deterministic_order_required"] is True
assert field["drop_fact"] == "TrivialMemory"

method_ids = {row["id"] for row in facts["method_facts"]}
for method in [
    "BindingContext::is_empty",
    "BindingContext::len",
    "BindingContext::contains",
    "BindingContext::lookup",
    "BindingContext::insert",
    "BindingContext::remove",
    "BindingContext::clear_for_function_entry",
]:
    assert method in method_ids

assert plan["schema_version"] == 0
assert plan["kind"] == "HakoLifecyclePlan"
assert plan["source_facts"] == "binding-context-facts-v0.json"

plans = {row["id"]: row for row in plan["plans"]}
assert plans["BindingContext.binding_map"]["plan_kind"] == "OrderedMapBox"
assert "BindingContext.binding_map.deterministic_order_required=true" in plans["BindingContext.binding_map"]["required_facts"]
assert plans["BindingContext"]["cleanup_policy"] == "erase"
assert "BindingContext.drop_fact=TrivialMemory" in plans["BindingContext"]["required_facts"]
assert plans["BindingContext::insert"]["plan_kind"] == "TransferOwned"
assert plans["BindingContext::lookup"]["return_plan"] == "Immediate"

denied = {row["deny_reason"] for row in plan["denied"]}
assert "BorrowEscapeUnknown" in denied
assert "MissingDeterministicOrderFact" in denied
assert "MissingTrivialMemoryDropFact" in denied

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
    "is_empty",
    "len",
    "contains",
    "lookup",
    "insert",
    "remove",
    "clear_for_function_entry",
    "iteration_order",
]:
    assert op in oracle_ops

assert oracle["drop_oracle"]["drop_observable"] is False
assert oracle["drop_oracle"]["required_fact"] == "BindingContext.drop_fact=TrivialMemory"
assert oracle["promotion_scope"]["hako_authority"] == "BindingContext only"
assert oracle["promotion_scope"]["mirbuilder_wide_claim"] is False
assert oracle["promotion_scope"]["variable_context_claim"] is False

plan_ids = set(plans)
for required in [
    "BindingContext",
    "BindingContext.binding_map",
    "BindingContext::is_empty",
    "BindingContext::len",
    "BindingContext::contains",
    "BindingContext::lookup",
    "BindingContext::insert",
    "BindingContext::remove",
    "BindingContext::clear_for_function_entry",
]:
    assert required in plan_ids
PY

cat <<'REPORT'
output_contract=rust-lifecycle-binding-context-pilot-v0
binding_context_lifecycle_facts_fixture=green
binding_context_lifecycle_plan_fixture=green
binding_context_oracle_vectors=green
ordered_map_projection_requires_deterministic_order_fact=green
memory_drop_erased_only_with_TrivialMemory=green
borrow_escape_unknown_denied=green
binding_context_plan_matches_oracle=green
hako_authority_promoted_for_BindingContext_only=green
general_resolver_implemented=0
converter_emission_added=0
rust_lifetime_syntax_added=0
summary=ok
REPORT
