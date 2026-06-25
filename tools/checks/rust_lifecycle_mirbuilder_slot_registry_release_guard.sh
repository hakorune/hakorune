#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

SCRIPT="tools/rust_lifecycle/mirbuilder_slot_registry_release.py"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-slot-registry-release-plan-v0.json"

python3 "$SCRIPT" --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-slot-registry-release-plan-v0.json")
plan = json.loads(path.read_text())

assert plan["kind"] == "MirBuilderSlotRegistryReleasePlanV1"
assert "SlotRegistryRelease" in plan["available_capabilities"]
assert plan["execution_profile"]["context"] == "finalize_module"
assert plan["execution_profile"]["prepared_slot_registry"] == "Some(FunctionSlotRegistry)"
release = plan["release_policy"]
assert release["lifecycle_owner"] == "CompilationContext.current_slot_registry"
assert release["init_operation"] == "Some(FunctionSlotRegistry::new())"
assert release["release_operation"] == "current_slot_registry = None"
assert release["release_timing"] == "AfterFunctionRegionStackPopBeforeModuleMetadataPublication"
assert release["released_value"] == "FunctionSlotRegistry"
result = plan["result_contract"]
assert result["mutates"] == ["builder.comp_ctx.current_slot_registry"]
assert result["entrypoint"] == "MirBuilder::finalize_module current_slot_registry release"
assert result["minimal_path_expected_result"] == "NoErrorReturn"
for key in [
    "slot_metadata_classification",
    "module_metadata_publication",
    "metadata_publication",
    "semantic_refresh",
    "all_functions_phi_materialization",
    "full_finalize_module",
    "generated_hako_artifact",
    "runtime_fallback",
]:
    assert plan["non_claims"][key] == 0, key

print("output_contract=rust-lifecycle-mirbuilder-slot-registry-release-guard-v0")
print("slot_registry_release_guard=green")
print("capability=SlotRegistryRelease")
print(f"entrypoint={result['entrypoint']}")
print("module_metadata_publication_claim=0")
print("generated_hako_change=0")
print("runtime_fallback=0")
print("summary=ok")
PY
