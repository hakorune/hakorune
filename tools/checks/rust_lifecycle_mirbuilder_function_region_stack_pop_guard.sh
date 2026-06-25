#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

SCRIPT="tools/rust_lifecycle/mirbuilder_function_region_stack_pop.py"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-function-region-stack-pop-plan-v0.json"

python3 "$SCRIPT" --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-function-region-stack-pop-plan-v0.json")
plan = json.loads(path.read_text())

assert plan["kind"] == "MirBuilderFunctionRegionStackPopPlanV1"
assert "FunctionRegionStackPop" in plan["available_capabilities"]
assert plan["execution_profile"]["context"] == "finalize_module"
assert plan["execution_profile"]["region_trace"] == "NYASH_REGION_TRACE=1"
pop_policy = plan["pop_policy"]
assert pop_policy["callsite"] == "region::observer::pop_function_region(self)"
assert pop_policy["guard"] == "NYASH_REGION_TRACE == 1"
assert pop_policy["operation"] == "metadata_ctx.pop_region"
assert pop_policy["result_ignored"] is True
assert pop_policy["tracing_disabled_effect"] == "NoOp"
assert pop_policy["push_counterpart_observed"] is True
result = plan["result_contract"]
assert result["mutates_when_guard_enabled"] == ["builder.metadata_ctx.current_region_stack"]
assert result["entrypoint"] == "region::observer::pop_function_region"
assert result["minimal_path_expected_result"] == "NoErrorReturn"
for key in [
    "observe_function_region_claim",
    "slot_registry_release",
    "metadata_publication",
    "semantic_refresh",
    "all_functions_phi_materialization",
    "full_finalize_module",
    "generated_hako_artifact",
    "runtime_fallback",
]:
    assert plan["non_claims"][key] == 0, key

print("output_contract=rust-lifecycle-mirbuilder-function-region-stack-pop-guard-v0")
print("function_region_stack_pop_guard=green")
print("capability=FunctionRegionStackPop")
print(f"entrypoint={result['entrypoint']}")
print("slot_registry_release_claim=0")
print("generated_hako_change=0")
print("runtime_fallback=0")
print("summary=ok")
PY
