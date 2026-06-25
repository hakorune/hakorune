#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

SCRIPT="tools/rust_lifecycle/mirbuilder_condition_fn_injection.py"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-condition-fn-injection-plan-v0.json"

python3 "$SCRIPT" --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-condition-fn-injection-plan-v0.json")
plan = json.loads(path.read_text())

assert plan["kind"] == "MirBuilderConditionFnInjectionPlanV1"
assert "ConditionFnInjection" in plan["available_capabilities"]
assert plan["execution_profile"]["module_transport"] == "MirModuleMinimalShell"
assert plan["execution_profile"]["condition_fn_initially_missing"] is True
assert plan["execution_profile"]["context"] == "finalize_module"
injection = plan["injection"]
assert injection["predicate"] == 'module.functions.get("condition_fn").is_none()'
assert injection["function_name"] == "condition_fn"
assert injection["params"] == ["MirType::Integer"]
assert injection["return_type"] == "MirType::Integer"
assert injection["effects"] == "EffectMask::PURE"
assert injection["entry_block"] == "BasicBlockId(0)"
assert injection["body"] == ["ConstInteger(1)", "ReturnValue(one)"]
assert injection["insert_operation"] == "module.add_function(f)"
assert injection["required_by_source"] is True
assert plan["result_contract"]["mutates"] == ["module.functions"]
assert plan["result_contract"]["entrypoint"] == "MirBuilder::finalize_module condition_fn injection block"
assert plan["result_contract"]["minimal_path_expected_result"] == "NoErrorReturn"
for key in [
    "condition_fn_policy_generalization",
    "region_stack_pop",
    "slot_registry_release",
    "metadata_publication",
    "semantic_refresh",
    "all_functions_phi_materialization",
    "full_finalize_module",
    "generated_hako_artifact",
    "runtime_fallback",
]:
    assert plan["non_claims"][key] == 0, key

print("output_contract=rust-lifecycle-mirbuilder-condition-fn-injection-guard-v0")
print("condition_fn_injection_guard=green")
print("capability=ConditionFnInjection")
print(f"entrypoint={plan['result_contract']['entrypoint']}")
print("region_stack_pop_claim=0")
print("generated_hako_change=0")
print("runtime_fallback=0")
print("summary=ok")
PY
