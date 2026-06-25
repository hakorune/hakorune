#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

SCRIPT="tools/rust_lifecycle/mirbuilder_typed_object_plan_refresh.py"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-typed-object-plan-refresh-plan-v0.json"

python3 "$SCRIPT" --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-typed-object-plan-refresh-plan-v0.json")
plan = json.loads(path.read_text())

assert plan["kind"] == "MirBuilderTypedObjectPlanRefreshPlanV1"
assert "TypedObjectPlanRefresh" in plan["available_capabilities"]
refresh = plan["refresh_policy"]
assert refresh["entrypoint"] == "refresh_module_typed_object_plans"
assert refresh["timing"] == "AfterRecordPackedLayoutRefreshBeforeDirectStateRefresh"
assert refresh["operation"] == "AssignTypedObjectPlans"
assert refresh["source"] == "build_typed_object_plans(module)"
assert refresh["build_provider"] == "storage_inference::build_typed_object_plans"
assert refresh["target"] == "module.metadata.typed_object_plans"
result = plan["result_contract"]
assert result["mutates"] == ["module.metadata.typed_object_plans"]
assert result["entrypoint"] == "typed_object_plan::refresh_module_typed_object_plans"
for key in [
    "typed_object_field_value_type_refresh",
    "typed_object_collection_field_element_refresh",
    "direct_state_plan_refresh",
    "full_semantic_refresh",
    "all_functions_phi_materialization",
    "full_finalize_module",
    "generated_hako_artifact",
    "runtime_fallback",
]:
    assert plan["non_claims"][key] == 0, key

print("output_contract=rust-lifecycle-mirbuilder-typed-object-plan-refresh-guard-v0")
print("typed_object_plan_refresh_guard=green")
print("capability=TypedObjectPlanRefresh")
print(f"entrypoint={result['entrypoint']}")
print("direct_state_plan_refresh_claim=0")
print("generated_hako_change=0")
print("runtime_fallback=0")
print("summary=ok")
PY
