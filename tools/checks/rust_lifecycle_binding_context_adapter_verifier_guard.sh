#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

FACTS="docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-adapter-facts-v0.json"
PLAN="docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-plan-v0.json"
RESULT="docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-adapter-verifier-result-v0.json"
CARD="docs/development/current/main/phases/phase-296x/296x-1470-HAKO-LIFECYCLE-VERIFIER-BINDING-CONTEXT-ADAPTER-FACTS-FIXTURE-001.md"

test -f "$FACTS"
test -f "$PLAN"
test -f "$RESULT"
test -f "$CARD"

python3 - <<'PY'
import json
from pathlib import Path

facts = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-adapter-facts-v0.json").read_text())
plan = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-plan-v0.json").read_text())
result = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-adapter-verifier-result-v0.json").read_text())

assert result["schema_version"] == 0
assert result["kind"] == "HakoLifecycleVerifierResult"
assert result["mode"] == "passive_adapter_fixture"
assert result["subject"] == facts["subject"] == plan["subject"]
assert result["source_facts"] == "binding-context-adapter-facts-v0.json"
assert result["source_plan"] == "binding-context-plan-v0.json"
assert result["result"] == "VerifiedPlan"

field = next(row for row in facts["fields"] if row["id"] == "BindingContext.binding_map")
assert field["deterministic_order_required"] is True
assert field["drop_class"] == "TrivialMemory"

plans = {row["id"]: row for row in plan["plans"]}
map_plan = plans["BindingContext.binding_map"]
assert map_plan["plan_kind"] == "OrderedMapBox"
assert "BindingContext.binding_map.deterministic_order_required=true" in map_plan["required_facts"]

verified_facts = set(result["verified_facts"])
for required in [
    "BindingContext.binding_map.deterministic_order_required=true",
    "BindingContext.binding_map.drop_class=TrivialMemory",
    "BindingContext::is_empty.receiver.borrow_kind=SharedRead",
    "BindingContext::insert.receiver.borrow_kind=UniqueWrite",
    "BindingContext::clear_for_function_entry.receiver.borrow_escape=CallOnly",
]:
    assert required in verified_facts

verified_boundaries = set(result["verified_boundaries"])
for required in [
    "adapter_facts_are_target_neutral",
    "deterministic_order_fact_available_for_plan",
    "shared_read_borrows_are_callonly",
    "unique_write_borrows_are_callonly",
    "drop_erase_backed_by_trivial_memory",
]:
    assert required in verified_boundaries

denied = set(result["denied_boundaries"])
for required in [
    "rustc toolchain integration",
    "general verifier implementation",
    "lifecycle-aware converter emission",
    "backend behavior change",
]:
    assert required in denied

claims = result["claims"]
assert claims["emission_allowed"] is False
assert claims["verifier_implementation_started"] is False
assert claims["emitter_implementation_started"] is False
assert claims["converter_core_changed"] is False
assert claims["backend_behavior_changed"] is False
assert claims["mirbuilder_wide_lifecycle"] is False

assert facts["target_neutral"]["hako_policy_owner"] is False
assert facts["target_neutral"]["hako_plan_kind_spelling_allowed"] is False
assert facts["target_neutral"]["rendering_instruction_allowed"] is False
PY

grep -q "binding_context_adapter_verifier_fixture_exists=1" "$CARD"
grep -q "verifier_result_kind=VerifiedPlan" "$CARD"
grep -q "source_adapter_facts=binding-context-adapter-facts-v0.json" "$CARD"
grep -q "source_plan=binding-context-plan-v0.json" "$CARD"
grep -q "emission_allowed=0" "$CARD"
grep -q "verifier_implementation_started=0" "$CARD"
grep -q "emitter_implementation_started=0" "$CARD"
grep -q "converter_core_changed=0" "$CARD"
grep -q "backend_behavior_changed=0" "$CARD"

cat <<'REPORT'
output_contract=rust-lifecycle-binding-context-adapter-verifier-v0
binding_context_adapter_verifier_fixture_exists=1
verifier_result_kind=VerifiedPlan
source_adapter_facts=binding-context-adapter-facts-v0.json
source_plan=binding-context-plan-v0.json
deterministic_order_verified=1
borrow_escape_verified=1
drop_erase_verified=1
emission_allowed=0
verifier_implementation_started=0
emitter_implementation_started=0
converter_core_changed=0
backend_behavior_changed=0
summary=ok
REPORT
