#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

SCRIPT="tools/rust_lifecycle/mirbuilder_record_packed_layout_refresh.py"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-record-packed-layout-refresh-plan-v0.json"

python3 "$SCRIPT" --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-record-packed-layout-refresh-plan-v0.json")
plan = json.loads(path.read_text())

assert plan["kind"] == "MirBuilderRecordPackedLayoutRefreshPlanV1"
assert "RecordAndPackedLayoutRefresh" in plan["available_capabilities"]
assert plan["execution_profile"]["context"] == "finalize_module"
refresh = plan["refresh_policy"]
assert refresh["entrypoint"] == "refresh_module_record_and_packed_layout_plans"
assert refresh["timing"] == "AfterModuleMetadataPublicationBeforeTypedObjectRefresh"
assert refresh["module_arg"] == "&mut MirModule"
assert len(refresh["steps"]) == 9
result = plan["result_contract"]
assert result["entrypoint"] == "semantic_refresh::refresh_module_record_and_packed_layout_plans"
assert len(result["mutates"]) == len(refresh["steps"])
for key in [
    "typed_object_plan_refresh",
    "direct_state_plan_refresh",
    "full_semantic_refresh",
    "all_functions_phi_materialization",
    "full_finalize_module",
    "generated_hako_artifact",
    "runtime_fallback",
]:
    assert plan["non_claims"][key] == 0, key

print("output_contract=rust-lifecycle-mirbuilder-record-packed-layout-refresh-guard-v0")
print("record_packed_layout_refresh_guard=green")
print("capability=RecordAndPackedLayoutRefresh")
print(f"entrypoint={result['entrypoint']}")
print("typed_object_plan_refresh_claim=0")
print("generated_hako_change=0")
print("runtime_fallback=0")
print("summary=ok")
PY
