#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

SCRIPT="tools/rust_lifecycle/mirbuilder_all_functions_phi_materialization.py"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-all-functions-phi-materialization-plan-v0.json"

python3 "$SCRIPT" --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-all-functions-phi-materialization-plan-v0.json")
plan = json.loads(path.read_text())

assert plan["kind"] == "MirBuilderAllFunctionsPhiMaterializationPlanV1"
assert "AllFunctionsPhiMaterialization" in plan["available_capabilities"]
sweep = plan["sweep_policy"]
assert sweep["iteration"] == "for function in module.functions.values_mut()"
assert sweep["delegate"] == "phi_input_materializer::materialize_all_phi_inputs"
assert sweep["delegate_context"] == "finalize_module_all_functions"
assert sweep["delegate_capability"] == "PhiInputMaterialization"
assert sweep["error_transport"] == "ResultPropagatedByQuestionMark"
result = plan["result_contract"]
assert result["mutates"] == [
    "module.functions[*].blocks",
    "module.functions[*].next_value_id",
]
assert result["entrypoint"] == "MirBuilder::finalize_module all-functions PHI materialization sweep"
for key in [
    "full_finalize_module",
    "generated_hako_artifact",
    "backend_route_changed",
    "abi_changed",
    "runtime_fallback",
    "mainline_selected",
    "source_selfhost_claim",
]:
    assert plan["non_claims"][key] == 0, key

print("output_contract=rust-lifecycle-mirbuilder-all-functions-phi-materialization-guard-v0")
print("all_functions_phi_materialization_guard=green")
print("capability=AllFunctionsPhiMaterialization")
print(f"entrypoint={result['entrypoint']}")
print("generated_hako_change=0")
print("runtime_fallback=0")
print("summary=ok")
PY
