#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

SCRIPT="tools/rust_lifecycle/mirbuilder_phi_input_materialization.py"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-phi-input-materialization-plan-v0.json"

python3 "$SCRIPT" --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-phi-input-materialization-plan-v0.json")
plan = json.loads(path.read_text())

assert plan["kind"] == "MirBuilderPhiInputMaterializationPlanV1"
assert "PhiInputMaterialization" in plan["available_capabilities"]
assert plan["execution_profile"]["function_transport"] == "MirFunctionPreparedMain"
assert plan["execution_profile"]["context"] == "finalize_module"
assert plan["materialization_steps"] == [
    "PruneUnusedPhiInstructions",
    "CompleteMissingSelfCarriedPhiInputs",
    "CollectPhiInputWorklist",
    "BuildDefBlocksAndDominators",
    "RematerializeIncomingPerPredWithMemo",
    "RewritePhiInputSlots",
    "ReturnChangedCount",
]
assert plan["result_contract"]["entrypoint"] == "phi_input_materializer::materialize_all_phi_inputs"
assert plan["result_contract"]["minimal_path_expected_result"] == "Result<usize, String>"
for key in [
    "dev_birth_verification",
    "module_function_insertion",
    "all_functions_phi_materialization",
    "semantic_refresh",
    "full_finalize_module",
    "generated_hako_artifact",
    "runtime_fallback",
]:
    assert plan["non_claims"][key] == 0, key

print("output_contract=rust-lifecycle-mirbuilder-phi-input-materialization-guard-v0")
print("phi_input_materialization_guard=green")
print("capability=PhiInputMaterialization")
print(f"entrypoint={plan['result_contract']['entrypoint']}")
print("dev_birth_verification_claim=0")
print("generated_hako_change=0")
print("runtime_fallback=0")
print("summary=ok")
PY
