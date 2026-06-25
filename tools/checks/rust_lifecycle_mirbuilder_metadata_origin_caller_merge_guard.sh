#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

SCRIPT="tools/rust_lifecycle/mirbuilder_metadata_origin_caller_merge.py"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-metadata-origin-caller-merge-plan-v0.json"

python3 "$SCRIPT" --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-metadata-origin-caller-merge-plan-v0.json")
plan = json.loads(path.read_text())

assert plan["kind"] == "MirBuilderMetadataOriginCallerMergePlanV1"
assert "MetadataOriginCallerMerge" in plan["available_capabilities"]
assert plan["execution_profile"]["function_transport"] == "MirFunctionPreparedMain"
assert plan["execution_profile"]["source"] == "self.metadata_ctx.value_origin_callers()"
assert plan["execution_profile"]["target"] == "function.metadata.value_origin_callers"
assert plan["merge"]["base_operation"] == "CloneExistingFunctionMap"
assert plan["merge"]["entry_operation"] == "InsertClonedValue"
assert plan["merge"]["collision_policy"] == "SourceWins"
assert plan["result_contract"]["entrypoint"] == "function.metadata.value_origin_callers = origin_callers"
for key in [
    "phi_return_type_inference",
    "phi_input_materialization",
    "module_function_insertion",
    "full_finalize_module",
    "generated_hako_artifact",
    "runtime_fallback",
]:
    assert plan["non_claims"][key] == 0, key

print("output_contract=rust-lifecycle-mirbuilder-metadata-origin-caller-merge-guard-v0")
print("metadata_origin_caller_merge_guard=green")
print("capability=MetadataOriginCallerMerge")
print(f"entrypoint={plan['result_contract']['entrypoint']}")
print("phi_return_type_inference_claim=0")
print("generated_hako_change=0")
print("runtime_fallback=0")
print("summary=ok")
PY
