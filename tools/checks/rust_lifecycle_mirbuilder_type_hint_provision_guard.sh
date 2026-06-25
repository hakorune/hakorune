#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

SCRIPT="tools/rust_lifecycle/mirbuilder_type_hint_provision.py"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-type-hint-provision-plan-v0.json"

python3 "$SCRIPT" --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-type-hint-provision-plan-v0.json")
plan = json.loads(path.read_text())

assert plan["kind"] == "MirBuilderTypeHintProvisionPlanV1"
assert "TypeHintProvision" in plan["available_capabilities"]
assert plan["execution_profile"]["function_transport"] == "MirFunctionPreparedMain"
assert plan["execution_profile"]["module_transport"] == "MirModulePreparedMain"
assert plan["execution_profile"]["value_types"] == "self.type_ctx.value_types"
assert plan["result_contract"]["entrypoint"] == "type_hint_providers::annotate_missing_result_types_from_calls_and_await"
assert plan["result_contract"]["minimal_path_expected_result"] == "OkImplicitUnit"
assert [case["instruction"] for case in plan["provider_cases"]] == [
    "Await",
    "Call(Global)",
    "Call(Constructor)",
    "Call(OtherOrMissingCallee)",
]
for key in [
    "metadata_value_type_publication",
    "metadata_origin_caller_merge",
    "phi_return_type_inference",
    "phi_input_materialization",
    "full_finalize_module",
    "generated_hako_artifact",
    "runtime_fallback",
]:
    assert plan["non_claims"][key] == 0, key

print("output_contract=rust-lifecycle-mirbuilder-type-hint-provision-guard-v0")
print("type_hint_provision_guard=green")
print("capability=TypeHintProvision")
print(f"entrypoint={plan['result_contract']['entrypoint']}")
print("metadata_value_type_publication_claim=0")
print("generated_hako_change=0")
print("runtime_fallback=0")
print("summary=ok")
PY
