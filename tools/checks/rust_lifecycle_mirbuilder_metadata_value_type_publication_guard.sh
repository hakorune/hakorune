#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

SCRIPT="tools/rust_lifecycle/mirbuilder_metadata_value_type_publication.py"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-metadata-value-type-publication-plan-v0.json"

python3 "$SCRIPT" --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-metadata-value-type-publication-plan-v0.json")
plan = json.loads(path.read_text())

assert plan["kind"] == "MirBuilderMetadataValueTypePublicationPlanV1"
assert "MetadataValueTypePublication" in plan["available_capabilities"]
assert plan["execution_profile"]["function_transport"] == "MirFunctionPreparedMain"
assert plan["execution_profile"]["value_types_source"] == "self.type_ctx.value_types"
assert plan["execution_profile"]["metadata_target"] == "function.metadata.value_types"
assert plan["publication"]["operation"] == "CloneOwnedMap"
assert plan["publication"]["timing"] == "AfterTypeHintProvisionBeforeOriginCallerMerge"
assert plan["result_contract"]["entrypoint"] == "function.metadata.value_types = self.type_ctx.value_types.clone()"
assert plan["result_contract"]["minimal_path_expected_result"] == "OkImplicitUnit"
for key in [
    "metadata_origin_caller_merge",
    "phi_return_type_inference",
    "phi_input_materialization",
    "full_finalize_module",
    "generated_hako_artifact",
    "runtime_fallback",
]:
    assert plan["non_claims"][key] == 0, key

print("output_contract=rust-lifecycle-mirbuilder-metadata-value-type-publication-guard-v0")
print("metadata_value_type_publication_guard=green")
print("capability=MetadataValueTypePublication")
print(f"entrypoint={plan['result_contract']['entrypoint']}")
print("metadata_origin_caller_merge_claim=0")
print("generated_hako_change=0")
print("runtime_fallback=0")
print("summary=ok")
PY
