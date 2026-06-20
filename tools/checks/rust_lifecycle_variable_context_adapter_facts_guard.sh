#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

FACTS="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-adapter-facts-v0.json"
CARD="docs/development/current/main/phases/phase-296x/296x-1472-RUST-LIFECYCLE-FACTS-ADAPTER-VARIABLE-CONTEXT-FIXTURE-001.md"

test -f "$FACTS"
test -f "$CARD"

python3 - <<'PY'
import json
from pathlib import Path

path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-adapter-facts-v0.json")
text = path.read_text()

for forbidden in [
    "OrderedMapBox",
    "BorrowView",
    "ReturnedMutableBorrow",
    "HakoLifecyclePlan",
    "LocalBox",
    "TransferOwned",
]:
    assert forbidden not in text, forbidden

facts = json.loads(text)
assert facts["schema_version"] == 0
assert facts["kind"] == "RustLifecycleAdapterFacts"
assert facts["subject"] == "hakorune_mir_builder::variable_context::VariableContext"

target = facts["target_neutral"]
assert target["hako_policy_owner"] is False
assert target["hako_plan_kind_spelling_allowed"] is False
assert target["rendering_instruction_allowed"] is False
assert target["rustc_toolchain_invoked"] is False

types = {row["id"]: row for row in facts["types"]}
assert types["VariableContext"]["copy_class"] == "NonCopyOwned"
assert types["VariableContext"]["drop_class"] == "TrivialMemory"
assert types["VariableContext"]["identity_observed"] is False
assert types["ValueId"]["copy_class"] == "ImmediateValue"
assert types["ValueId"]["drop_class"] == "TrivialMemory"

fields = {row["id"]: row for row in facts["fields"]}
field = fields["VariableContext.variable_map"]
assert field["rust_type"] == "BTreeMap<String, ValueId>"
assert field["deterministic_order_required"] is True
assert field["drop_class"] == "TrivialMemory"
assert field["thread_atomic_observed"] is False

methods = {row["id"]: row for row in facts["methods"]}

for method in ["VariableContext::lookup", "VariableContext::contains"]:
    receiver = methods[method]["receiver"]
    assert receiver["borrow_kind"] == "SharedRead"
    assert receiver["borrow_escape"] == "CallOnly"
    assert receiver["mutation"] is False

for method in ["VariableContext::insert", "VariableContext::remove"]:
    receiver = methods[method]["receiver"]
    assert receiver["borrow_kind"] == "UniqueWrite"
    assert receiver["borrow_escape"] == "CallOnly"
    assert receiver["mutation"] is True

immutable = methods["VariableContext::variable_map"]
assert immutable["receiver"]["borrow_kind"] == "SharedRead"
assert immutable["receiver"]["borrow_escape"] == "Returned"
assert immutable["returned_reference"]["mutation_allowed"] is False

mutable = methods["VariableContext::variable_map_mut"]
assert mutable["receiver"]["borrow_kind"] == "UniqueWrite"
assert mutable["receiver"]["borrow_escape"] == "Returned"
assert mutable["returned_reference"]["mutation_allowed"] is True

snapshot = methods["VariableContext::snapshot"]
assert snapshot["ownership_effect"] == "CloneOwnedMap"
assert snapshot["returns"]["deterministic_order_required"] is True
assert snapshot["returns"]["drop_class"] == "TrivialMemory"

restore = methods["VariableContext::restore"]
assert restore["ownership_effect"] == "ReplaceOwned"
assert restore["arguments"][0]["move_kind"] == "ConsumeArgument"
assert restore["arguments"][0]["deterministic_order_required"] is True
assert restore["replaced_field_drop_class"] == "TrivialMemory"

consumers = {row["id"]: row for row in facts["consumers"]}
assert consumers["CarrierInfo::from_variable_map"]["required_access"] == "ReadOnly"
assert consumers["CarrierInfo::from_variable_map"]["requires_deterministic_order"] is True
assert consumers["CarrierInfo::with_explicit_carriers"]["missing_carrier_policy"] == "FailFast"

negative = {row["id"]: row["required_fact"] for row in facts["negative_requirements"]}
assert negative["returned_mutable_map_reference"] == "borrow_escape=Returned,borrow_kind=UniqueWrite"
assert negative["missing_deterministic_order"] == "deterministic_order_required"
assert negative["missing_trivial_memory_drop"] == "drop_class=TrivialMemory"
PY

grep -q "variable_context_adapter_facts_fixture_exists=1" "$CARD"
grep -q "adapter_facts_are_target_neutral=1" "$CARD"
grep -q "returned_immutable_borrow_fact_present=1" "$CARD"
grep -q "returned_mutable_borrow_fact_present=1" "$CARD"
grep -q "snapshot_restore_ownership_facts_present=1" "$CARD"
grep -q "carrier_read_requirements_present=1" "$CARD"
grep -q "hako_policy_spellings_absent=1" "$CARD"
grep -q "rustc_toolchain_integration_started=0" "$CARD"
grep -q "resolver_implementation_started=0" "$CARD"
grep -q "verifier_implementation_started=0" "$CARD"
grep -q "emitter_implementation_started=0" "$CARD"
grep -q "converter_core_changed=0" "$CARD"
grep -q "backend_behavior_changed=0" "$CARD"

cat <<'REPORT'
output_contract=rust-lifecycle-variable-context-adapter-facts-v0
variable_context_adapter_facts_fixture_exists=1
adapter_facts_are_target_neutral=1
deterministic_order_fact_present=1
returned_immutable_borrow_fact_present=1
returned_mutable_borrow_fact_present=1
snapshot_restore_ownership_facts_present=1
carrier_read_requirements_present=1
hako_policy_spellings_absent=1
rustc_toolchain_integration_started=0
resolver_implementation_started=0
verifier_implementation_started=0
emitter_implementation_started=0
converter_core_changed=0
backend_behavior_changed=0
summary=ok
REPORT
