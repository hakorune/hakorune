#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

FACTS="docs/development/current/main/design/fixtures/rust-lifecycle/binding-context-adapter-facts-v0.json"
DESIGN="docs/development/current/main/design/rustc-semir-binding-context-adapter-harness-design.md"

test -f "$FACTS"
test -f "$DESIGN"

python3 - <<'PY' "$FACTS" "$DESIGN"
import json
import sys
from pathlib import Path

facts_path = Path(sys.argv[1])
design_path = Path(sys.argv[2])
facts_text = facts_path.read_text()
design = design_path.read_text()

for forbidden in [
    "OrderedMapBox",
    "BorrowView",
    "TransferOwned",
    "LocalBox",
    "HakoLifecyclePlan",
    ".hako source",
    "backend lowering",
]:
    assert forbidden not in facts_text, forbidden

facts = json.loads(facts_text)
assert facts["schema_version"] == 0
assert facts["kind"] == "RustLifecycleAdapterFacts"
assert facts["subject"] == "hakorune_mir_builder::binding_context::BindingContext"

target = facts["target_neutral"]
assert target["hako_policy_owner"] is False
assert target["hako_plan_kind_spelling_allowed"] is False
assert target["rendering_instruction_allowed"] is False
assert target["rustc_toolchain_invoked"] is False

fields = {row["id"]: row for row in facts["fields"]}
binding_map = fields["BindingContext.binding_map"]
assert binding_map["rust_type"] == "BTreeMap<String, BindingId>"
assert binding_map["deterministic_order_required"] is True
assert binding_map["drop_class"] == "TrivialMemory"

methods = {row["id"]: row for row in facts["methods"]}
assert methods["BindingContext::lookup"]["receiver"]["borrow_kind"] == "SharedRead"
assert methods["BindingContext::lookup"]["receiver"]["borrow_escape"] == "CallOnly"
assert methods["BindingContext::insert"]["receiver"]["borrow_kind"] == "UniqueWrite"
assert methods["BindingContext::insert"]["receiver"]["borrow_escape"] == "CallOnly"
assert methods["BindingContext::insert"]["ownership_effect"] == "ConsumeArgument"

for token in [
    "output:",
    "RustLifecycleFacts-v0 JSON only",
    "raw pretty MIR text",
    "do_not_emit_HakoLifecyclePlan_from_adapter=1",
    "do_not_choose_OrderedMapBox_in_adapter=1",
]:
    assert token in design, token
PY

cat <<'REPORT'
output_contract=rustc-semir-binding-context-adapter-harness-probe-v0
harness_probe_green=1
output_kind=RustLifecycleAdapterFacts
subject=BindingContext
target_neutral_adapter=1
adapter_policy_owner=0
raw_rustc_dump_as_schema=0
backend_behavior_changed=0
summary=ok
REPORT
