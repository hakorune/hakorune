#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import json
from pathlib import Path

base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")
facts = json.loads((base / "variable-context-snapshot-restore-facts-v0.json").read_text())
plan = json.loads((base / "variable-context-snapshot-restore-plan-v0.json").read_text())
oracle = json.loads((base / "variable-context-snapshot-restore-oracle-vectors-v0.json").read_text())

assert facts["schema_version"] == 0
assert facts["kind"] == "RustLifecycleFacts"
assert facts["subject"].endswith("VariableContext.snapshot_restore")
assert facts["base_facts"] == "variable-context-simple-map-facts-v0.json"

methods = {row["id"]: row for row in facts["method_facts"]}
snapshot = methods["VariableContext::snapshot"]
assert snapshot["operation"] == "CloneOwnedMap"
assert snapshot["receiver_borrow"]["kind"] == "SharedRead"
assert snapshot["receiver_borrow"]["escapes"] is False
assert snapshot["returns"]["deterministic_order_required"] is True
assert snapshot["returns"]["drop_fact"] == "TrivialMemory"

restore = methods["VariableContext::restore"]
assert restore["operation"] == "ReplaceOwned"
assert restore["receiver_borrow"]["kind"] == "UniqueWrite"
assert restore["receiver_borrow"]["escapes"] is False
arg = restore["argument_moves"][0]
assert arg["move_kind"] == "ConsumeArgument"
assert arg["deterministic_order_required"] is True
assert restore["old_value_cleanup"]["required_fact"] == "VariableContext.variable_map.drop_fact=TrivialMemory"

denied = {row["id"]: row["deny_reason"] for row in facts["denied_methods"]}
assert denied["VariableContext::variable_map_mut"] == "ReturnedMutableBorrow"
assert "CarrierInfo::from_variable_map" in set(facts["excluded_consumers"])

assert plan["schema_version"] == 0
assert plan["kind"] == "HakoLifecyclePlan"
assert plan["source_facts"] == "variable-context-snapshot-restore-facts-v0.json"
plans = {row["id"]: row for row in plan["plans"]}
assert plans["VariableContext::snapshot"]["plan_kind"] == "CloneOwnedMap"
assert plans["VariableContext::snapshot"]["result_plan"] == "OwnedOrderedMap"
assert "returns.deterministic_order_required=true" in plans["VariableContext::snapshot"]["required_facts"]
assert plans["VariableContext::restore"]["plan_kind"] == "ReplaceOwned"
assert plans["VariableContext::restore"]["old_value_cleanup"] == "erase"
assert "VariableContext.variable_map.drop_fact=TrivialMemory" in plans["VariableContext::restore"]["required_facts"]

for item in [
    "VariableContext::variable_map_mut",
    "CarrierInfo::from_variable_map",
    "PHI planner integration",
]:
    assert item in set(plan["denied"])

behavior = plan["behavior"]
assert behavior["general_resolver_implemented"] is False
assert behavior["converter_emission_added"] is False
assert behavior["rust_lifetime_syntax_added"] is False
assert behavior["carrier_phi_claim"] is False
assert behavior["full_variable_context_claim"] is False

assert oracle["schema_version"] == 0
assert oracle["kind"] == "RustOracleVectors"
ops = [op for vector in oracle["vectors"] for op in vector["operations"]]
assert any(op.get("op") == "snapshot" for op in ops)
restore_ops = [op for op in ops if op.get("op") == "restore"]
assert restore_ops
assert "ReplaceOwned" in restore_ops[0]["requires"]
assert "old_map_cleanup=TrivialMemory" in restore_ops[0]["requires"]

scope = oracle["promotion_scope"]
assert scope["hako_authority"] == "VariableContext snapshot/restore only"
assert scope["carrier_phi_claim"] is False
assert scope["full_variable_context_claim"] is False
assert scope["mirbuilder_wide_claim"] is False
PY

cat <<'REPORT'
output_contract=rust-lifecycle-variable-context-snapshot-restore-v0
snapshot_restore_facts_fixture=green
snapshot_restore_plan_fixture=green
snapshot_restore_oracle_vectors=green
snapshot_clone_requires_deterministic_order=green
restore_requires_ReplaceOwned=green
old_map_cleanup_requires_TrivialMemory=green
mutable_map_claim=0
carrier_PHI_claim=0
full_VariableContext_parity_claim=0
general_resolver_implemented=0
converter_emission_added=0
rust_lifetime_syntax_added=0
summary=ok
REPORT
