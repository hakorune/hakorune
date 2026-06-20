#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

FACTS="docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-adapter-facts-v0.json"
CARD="docs/development/current/main/phases/phase-296x/296x-1468-RUST-LIFECYCLE-FACTS-ADAPTER-BINDING-CONTEXT-FIXTURE-001.md"

test -f "$FACTS"
test -f "$CARD"

python3 - <<'PY'
import json
from pathlib import Path

path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-adapter-facts-v0.json")
text = path.read_text()

for forbidden in [
    "OrderedMapBox",
    "BorrowView",
    "TransferOwned",
    "LocalBox",
    "HakoLifecyclePlan",
    "converter_render",
]:
    assert forbidden not in text, forbidden

facts = json.loads(text)
assert facts["schema_version"] == 0
assert facts["kind"] == "RustLifecycleAdapterFacts"
assert facts["subject"] == "hakorune_mir_builder::binding_context::BindingContext"
assert facts["target_neutral"]["hako_policy_owner"] is False
assert facts["target_neutral"]["hako_plan_kind_spelling_allowed"] is False
assert facts["target_neutral"]["rendering_instruction_allowed"] is False
assert facts["target_neutral"]["rustc_toolchain_invoked"] is False

types = {row["id"]: row for row in facts["types"]}
assert types["BindingContext"]["copy_class"] == "NonCopyOwned"
assert types["BindingContext"]["drop_class"] == "TrivialMemory"
assert types["BindingContext"]["identity_observed"] is False
assert types["BindingContext"]["thread_atomic_observed"] is False
assert types["BindingId"]["copy_class"] == "ImmediateValue"
assert types["BindingId"]["drop_class"] == "TrivialMemory"

fields = {row["id"]: row for row in facts["fields"]}
field = fields["BindingContext.binding_map"]
assert field["rust_type"] == "BTreeMap<String, BindingId>"
assert field["key_type"] == "String"
assert field["value_type"] == "BindingId"
assert field["deterministic_order_required"] is True
assert field["drop_class"] == "TrivialMemory"
assert field["identity_observed"] is False
assert field["thread_atomic_observed"] is False

methods = {row["id"]: row for row in facts["methods"]}
for method in [
    "BindingContext::is_empty",
    "BindingContext::len",
    "BindingContext::contains",
    "BindingContext::lookup",
]:
    receiver = methods[method]["receiver"]
    assert receiver["borrow_kind"] == "SharedRead"
    assert receiver["borrow_escape"] == "CallOnly"
    assert receiver["mutation"] is False

for method in [
    "BindingContext::insert",
    "BindingContext::remove",
    "BindingContext::clear_for_function_entry",
]:
    receiver = methods[method]["receiver"]
    assert receiver["borrow_kind"] == "UniqueWrite"
    assert receiver["borrow_escape"] == "CallOnly"
    assert receiver["mutation"] is True

assert methods["BindingContext::insert"]["ownership_effect"] == "ConsumeArgument"
assert methods["BindingContext::lookup"]["returns"]["copy_class"] == "ImmediateValue"
assert methods["BindingContext::remove"]["returns"]["drop_class"] == "TrivialMemory"

negative = {row["id"]: row["required_fact"] for row in facts["negative_requirements"]}
assert negative["borrow_escape_unknown"] == "borrow_escape"
assert negative["missing_deterministic_order"] == "deterministic_order_required"
assert negative["missing_trivial_memory_drop"] == "drop_class=TrivialMemory"
PY

grep -q "binding_context_adapter_facts_fixture_exists=1" "$CARD"
grep -q "adapter_facts_are_target_neutral=1" "$CARD"
grep -q "hako_policy_spellings_absent=1" "$CARD"
grep -q "rustc_toolchain_integration_started=0" "$CARD"
grep -q "resolver_implementation_started=0" "$CARD"
grep -q "verifier_implementation_started=0" "$CARD"
grep -q "emitter_implementation_started=0" "$CARD"
grep -q "converter_core_changed=0" "$CARD"

cat <<'REPORT'
output_contract=rust-lifecycle-binding-context-adapter-facts-v0
binding_context_adapter_facts_fixture_exists=1
adapter_facts_are_target_neutral=1
deterministic_order_fact_present=1
shared_read_callonly_facts_present=1
unique_write_callonly_facts_present=1
trivial_memory_drop_fact_present=1
hako_policy_spellings_absent=1
rustc_toolchain_integration_started=0
resolver_implementation_started=0
verifier_implementation_started=0
emitter_implementation_started=0
converter_core_changed=0
backend_behavior_changed=0
summary=ok
REPORT
