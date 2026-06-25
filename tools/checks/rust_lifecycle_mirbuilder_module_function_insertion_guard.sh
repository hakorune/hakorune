#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

SCRIPT="tools/rust_lifecycle/mirbuilder_module_function_insertion.py"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-module-function-insertion-plan-v0.json"

python3 "$SCRIPT" --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-module-function-insertion-plan-v0.json")
plan = json.loads(path.read_text())

assert plan["kind"] == "MirBuilderModuleFunctionInsertionPlanV1"
assert "ModuleFunctionInsertion" in plan["available_capabilities"]
assert plan["execution_profile"]["module_transport"] == "MirModuleMinimalShell"
assert plan["execution_profile"]["function_transport"] == "MirFunctionPreparedMain"
assert plan["execution_profile"]["context"] == "finalize_module"
insertion = plan["insertion"]
assert insertion["callsite"] == "module.add_function(function)"
assert insertion["inserted_function"] == "MirFunctionPreparedMain"
assert insertion["key_source"] == "function.signature.name.clone()"
assert insertion["container"] == "MirModule.functions"
assert insertion["container_operation"] == "BTreeMap::insert"
assert insertion["collision_policy"] == "ReplaceExistingByName"
assert plan["result_contract"]["mutates"] == ["module.functions"]
assert plan["result_contract"]["entrypoint"] == "MirModule::add_function"
assert plan["result_contract"]["minimal_path_expected_result"] == "NoErrorReturn"
for key in [
    "condition_fn_injection",
    "all_functions_phi_materialization",
    "region_stack_pop",
    "slot_registry_release",
    "metadata_publication",
    "semantic_refresh",
    "full_finalize_module",
    "generated_hako_artifact",
    "runtime_fallback",
]:
    assert plan["non_claims"][key] == 0, key

print("output_contract=rust-lifecycle-mirbuilder-module-function-insertion-guard-v0")
print("module_function_insertion_guard=green")
print("capability=ModuleFunctionInsertion")
print(f"entrypoint={plan['result_contract']['entrypoint']}")
print("condition_fn_injection_claim=0")
print("generated_hako_change=0")
print("runtime_fallback=0")
print("summary=ok")
PY
