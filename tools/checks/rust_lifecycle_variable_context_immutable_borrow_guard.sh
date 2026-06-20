#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import json
from pathlib import Path

base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")
facts = json.loads((base / "variable-context-immutable-borrow-facts-v0.json").read_text())
plan = json.loads((base / "variable-context-immutable-borrow-plan-v0.json").read_text())
oracle = json.loads((base / "variable-context-immutable-borrow-oracle-vectors-v0.json").read_text())

assert facts["schema_version"] == 0
assert facts["kind"] == "RustLifecycleFacts"
assert facts["subject"].endswith("VariableContext.immutable_map_borrow")
assert facts["base_facts"] == "variable-context-simple-map-facts-v0.json"

method = facts["method_facts"][0]
assert method["id"] == "VariableContext::variable_map"
borrow = method["receiver_borrow"]
assert borrow["kind"] == "SharedRead"
assert borrow["scope"] == "ReturnedBorrow"
assert borrow["escapes"] is False
assert borrow["owner_carrying_required"] is True
assert method["returns"]["borrow_view"] == "OwnerCarryingBorrowView"
assert method["returns"]["access"] == "read"

excluded = {row["id"] for row in facts["excluded_consumers"]}
assert "CarrierInfo::from_variable_map" in excluded
assert "CarrierInfo::with_explicit_carriers" in excluded

denied = {row["id"]: row["deny_reason"] for row in facts["denied_methods"]}
assert denied["VariableContext::variable_map_mut"] == "ReturnedMutableBorrow"
assert denied["VariableContext::snapshot"] == "SnapshotOwnedMapOutOfScope"
assert denied["VariableContext::restore"] == "ReplaceOwnedOutOfScope"

assert plan["schema_version"] == 0
assert plan["kind"] == "HakoLifecyclePlan"
assert plan["source_facts"] == "variable-context-immutable-borrow-facts-v0.json"
entry = plan["plans"][0]
assert entry["id"] == "VariableContext::variable_map"
assert entry["plan_kind"] == "BorrowView"
assert entry["access"] == "read"
assert entry["owner_carrying"] is True
assert entry["escape_policy"] == "deny_if_escapes"
assert entry["return_alias_policy"] == "owner_carrying_view_only"
assert "receiver_borrow.owner_carrying_required=true" in entry["required_facts"]
assert "receiver_borrow.escapes=false" in entry["required_facts"]

for item in [
    "VariableContext::variable_map_mut",
    "VariableContext::snapshot",
    "VariableContext::restore",
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
assert any(op.get("op") == "borrow_view" and op.get("owner_carrying") is True for op in ops)
assert any(op.get("op") == "borrow_iteration_order" for op in ops)

denied_vectors = set(oracle["denied_vectors"])
for item in [
    "variable_map_mut_returned_borrow",
    "snapshot",
    "restore",
    "carrier_extraction",
    "phi_planner_integration",
]:
    assert item in denied_vectors

scope = oracle["promotion_scope"]
assert scope["hako_authority"] == "VariableContext immutable map borrow only"
assert scope["carrier_phi_claim"] is False
assert scope["full_variable_context_claim"] is False
assert scope["mirbuilder_wide_claim"] is False
PY

cat <<'REPORT'
output_contract=rust-lifecycle-variable-context-immutable-borrow-v0
immutable_map_borrow_facts_fixture=green
immutable_map_borrow_plan_fixture=green
immutable_map_borrow_oracle_vectors=green
owner_carrying_borrowview_required=green
borrow_escape_denied=green
mutable_map_claim=0
snapshot_restore_claim=0
carrier_PHI_claim=0
full_VariableContext_parity_claim=0
general_resolver_implemented=0
converter_emission_added=0
rust_lifetime_syntax_added=0
summary=ok
REPORT
