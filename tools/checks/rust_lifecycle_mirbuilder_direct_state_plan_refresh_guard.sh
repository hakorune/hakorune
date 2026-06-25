#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

SCRIPT="tools/rust_lifecycle/mirbuilder_direct_state_plan_refresh.py"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-direct-state-plan-refresh-plan-v0.json"

python3 "$SCRIPT" --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-direct-state-plan-refresh-plan-v0.json")
plan = json.loads(path.read_text())

assert plan["kind"] == "MirBuilderDirectStatePlanRefreshPlanV1"
assert "DirectStatePlanRefresh" in plan["available_capabilities"]
refresh = plan["refresh_policy"]
assert refresh["entrypoint"] == "refresh_module_direct_state_plans"
assert refresh["timing"] == "AfterTypedObjectPlanRefreshBeforeAllFunctionsPhiMaterialization"
assert refresh["operation"] == "AssignDirectStatePlans"
assert refresh["source"] == "build_direct_state_plans(module)"
assert refresh["build_provider"] == "direct_state_plan::build_direct_state_plans"
assert refresh["target"] == "module.metadata.direct_state_plans"
builder = plan["plan_builder_contract"]
assert builder["input_authority"] == "module.metadata.user_box_field_decls"
assert builder["ordering"] == "SortBoxNames"
assert builder["field_selection"] == "TypedObjectFieldStorageUsesIntegerLaneAndNotWeak"
assert builder["state_repr"] == "direct_v0"
assert builder["runtime_layout_created"] == 0
assert builder["lowering_enabled"] == 0
result = plan["result_contract"]
assert result["mutates"] == ["module.metadata.direct_state_plans"]
assert result["entrypoint"] == "direct_state_plan::refresh_module_direct_state_plans"
for key in [
    "all_functions_phi_materialization",
    "direct_state_lowering",
    "route_selection",
    "native_direct_guard",
    "full_semantic_refresh",
    "full_finalize_module",
    "generated_hako_artifact",
    "runtime_fallback",
]:
    assert plan["non_claims"][key] == 0, key

print("output_contract=rust-lifecycle-mirbuilder-direct-state-plan-refresh-guard-v0")
print("direct_state_plan_refresh_guard=green")
print("capability=DirectStatePlanRefresh")
print(f"entrypoint={result['entrypoint']}")
print("all_functions_phi_materialization_claim=0")
print("generated_hako_change=0")
print("runtime_fallback=0")
print("summary=ok")
PY
