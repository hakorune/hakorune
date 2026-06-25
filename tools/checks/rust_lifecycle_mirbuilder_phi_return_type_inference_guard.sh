#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

SCRIPT="tools/rust_lifecycle/mirbuilder_phi_return_type_inference.py"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-phi-return-type-inference-plan-v0.json"

python3 "$SCRIPT" --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-phi-return-type-inference-plan-v0.json")
plan = json.loads(path.read_text())

assert plan["kind"] == "MirBuilderPhiReturnTypeInferencePlanV1"
assert "PhiReturnTypeInference" in plan["available_capabilities"]
assert plan["execution_profile"]["function_transport"] == "MirFunctionPreparedMain"
assert plan["execution_profile"]["builder_type_context"] == "self.type_ctx.value_types"
assert plan["resolver_chain"] == [
    "SkipConcreteReturnType",
    "TerminatorReturnOnly",
    "DirectValueTypesLookup",
    "TypeHintPolicyExtract",
    "MethodReturnHintBox",
    "PhiTypeResolver",
    "GenericTypeResolver",
    "UnknownFallbackOutsideDebug",
]
assert plan["result_contract"]["entrypoint"] == "phi_type_inference::infer_return_type_from_phi"
assert plan["result_contract"]["minimal_path_expected_result"] == "Option<MirType>"
for key in [
    "phi_input_materialization",
    "module_function_insertion",
    "full_finalize_module",
    "generated_hako_artifact",
    "runtime_fallback",
]:
    assert plan["non_claims"][key] == 0, key

print("output_contract=rust-lifecycle-mirbuilder-phi-return-type-inference-guard-v0")
print("phi_return_type_inference_guard=green")
print("capability=PhiReturnTypeInference")
print(f"entrypoint={plan['result_contract']['entrypoint']}")
print("phi_input_materialization_claim=0")
print("generated_hako_change=0")
print("runtime_fallback=0")
print("summary=ok")
PY
