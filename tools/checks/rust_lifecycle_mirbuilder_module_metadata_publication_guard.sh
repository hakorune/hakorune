#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

SCRIPT="tools/rust_lifecycle/mirbuilder_module_metadata_publication.py"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-module-metadata-publication-plan-v0.json"

python3 "$SCRIPT" --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-module-metadata-publication-plan-v0.json")
plan = json.loads(path.read_text())

assert plan["kind"] == "MirBuilderModuleMetadataPublicationPlanV1"
assert "ModuleMetadataPublication" in plan["available_capabilities"]
assert plan["execution_profile"]["context"] == "finalize_module"
assert plan["execution_profile"]["module_transport"] == "MirModuleMinimalShell"
publication = plan["publication"]
assert publication["timing"] == "AfterSlotRegistryReleaseBeforeSemanticRefresh"
assert [field["target"] for field in publication["fields"]] == [
    "module.metadata.user_box_decls",
    "module.metadata.user_box_field_decls",
    "module.metadata.record_decls",
    "module.metadata.enum_decls",
]
assert publication["fields"][1]["projected_fields"] == ["name", "declared_type_name", "is_weak"]
result = plan["result_contract"]
assert result["mutates"] == [
    "module.metadata.user_box_decls",
    "module.metadata.user_box_field_decls",
    "module.metadata.record_decls",
    "module.metadata.enum_decls",
]
assert result["entrypoint"] == "MirBuilder::finalize_module module metadata publication"
for key in [
    "semantic_refresh",
    "record_and_packed_layout_refresh",
    "typed_object_plan_refresh",
    "direct_state_plan_refresh",
    "all_functions_phi_materialization",
    "full_finalize_module",
    "generated_hako_artifact",
    "runtime_fallback",
]:
    assert plan["non_claims"][key] == 0, key

print("output_contract=rust-lifecycle-mirbuilder-module-metadata-publication-guard-v0")
print("module_metadata_publication_guard=green")
print("capability=ModuleMetadataPublication")
print(f"entrypoint={result['entrypoint']}")
print("semantic_refresh_claim=0")
print("generated_hako_change=0")
print("runtime_fallback=0")
print("summary=ok")
PY
