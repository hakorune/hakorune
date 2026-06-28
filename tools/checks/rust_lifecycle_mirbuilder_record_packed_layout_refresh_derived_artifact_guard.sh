#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-record-packed-layout-refresh"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_record_packed_layout_refresh.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_record_packed_layout_refresh.artifact.json"
RECIPE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-record-packed-layout-refresh-derived-hako-recipe-v0.json"
VERIFIER="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-record-packed-layout-refresh-derived-hako-verifier-result-v0.json"
EXE="/tmp/hako_mirbuilder_record_packed_layout_refresh"
EXPECTED="/tmp/hako_mirbuilder_record_packed_layout_refresh.expected"
RAW="/tmp/hako_mirbuilder_record_packed_layout_refresh.out.raw"
OUT="/tmp/hako_mirbuilder_record_packed_layout_refresh.out"
MIR_JSON="/tmp/hako_mirbuilder_record_packed_layout_refresh.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import hashlib
import json
from pathlib import Path

artifact_path = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_record_packed_layout_refresh.hako")
manifest_path = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_record_packed_layout_refresh.artifact.json")
recipe_path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-record-packed-layout-refresh-derived-hako-recipe-v0.json")
verifier_path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-record-packed-layout-refresh-derived-hako-verifier-result-v0.json")

hako = artifact_path.read_text()
required = [
    "box RecordPackedLayoutRefreshPayloadBox",
    "shadow_json: StringBox",
    "RecordPackedLayoutRefreshFixtureApi",
    "RecordPackedLayoutRefreshApi",
    "RecordPackedLayoutRefreshResultBox",
    "RecordPackedLayoutRefreshShadowCandidateV1",
    "module.metadata.record_layout_plans",
    "module.metadata.hako_alloc_huge_page_packed_store_pilot_plans",
    "semantic_refresh::refresh_module_record_and_packed_layout_plans",
    "AfterModuleMetadataPublicationBeforeTypedObjectRefresh",
    "result.shadow_record = hako_shadow.payload",
    "result.shadow_json = hako_shadow.shadow_json",
    "mirbuilder_record_packed_layout_refresh_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing record-packed-layout artifact text: {missing}")

manifest = json.loads(manifest_path.read_text())
if manifest.get("kind") != "RustDerivedHakoArtifact":
    raise SystemExit("record-packed-layout manifest kind drift")
if manifest.get("family_id") != "hakorune_mir_builder::record_packed_layout_refresh":
    raise SystemExit("record-packed-layout manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("record-packed-layout artifact must remain DerivedShadow")
if manifest.get("pilot_scope") != "RecordPackedLayoutRefresh_prepared_record_packed_layout_refresh_only":
    raise SystemExit("record-packed-layout pilot scope drift")
generator = manifest.get("generator") or {}
if generator.get("tool") != "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-record-packed-layout-refresh":
    raise SystemExit("record-packed-layout generator drift")
if manifest.get("output", {}).get("hako_path") != "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_record_packed_layout_refresh.hako":
    raise SystemExit("record-packed-layout output path drift")
if manifest.get("output", {}).get("hako_sha256") != hashlib.sha256(artifact_path.read_bytes()).hexdigest():
    raise SystemExit("record-packed-layout artifact hash stale")
claims = manifest.get("claims") or {}
expected_claims = {
    "generated_hako_manual_edit": 0,
    "record_packed_layout_refresh": 1,
    "record_packed_layout_field_value_type_refresh": 0,
    "record_packed_layout_collection_field_element_refresh": 0,
    "module_metadata_publication": 0,
    "semantic_refresh": 0,
    "all_functions_phi_materialization": 0,
    "full_finalize_module": 0,
    "mainline_selected": 0,
    "source_selfhost_claim": 0,
    "backend_behavior_changed": 0,
    "runtime_fallback": 0,
    "new_backend_route": 0,
    "new_abi": 0,
    "new_canonical_mir_instruction": 0,
}
for key, expected in expected_claims.items():
    if claims.get(key) != expected:
        raise SystemExit(f"record-packed-layout claim drift: {key}={claims.get(key)}")

recipe = json.loads(recipe_path.read_text())
if recipe.get("family_id") != "hakorune_mir_builder::record_packed_layout_refresh":
    raise SystemExit("record-packed-layout recipe family drift")
if recipe.get("pilot_scope") != "RecordPackedLayoutRefresh_prepared_record_packed_layout_refresh_only":
    raise SystemExit("record-packed-layout recipe scope drift")
if recipe.get("selected_body_count") != "record_packed_layout_refresh_prepared_record_packed_layout_refresh_only":
    raise SystemExit("record-packed-layout selected body count drift")
methods = {method["id"]: method for method in recipe.get("methods", [])}
method = methods.get("RecordPackedLayoutRefreshApi::project_shadow_record")
if method is None:
    raise SystemExit("missing record-packed-layout recipe method")
if method.get("hako_operation") != "StaticCall + SetField + ReturnValue":
    raise SystemExit("record-packed-layout recipe hako operation drift")

verifier = json.loads(verifier_path.read_text())
if verifier.get("kind") != "DerivedHakoArtifactVerifierResult":
    raise SystemExit("record-packed-layout verifier kind drift")
if verifier.get("family_id") != "hakorune_mir_builder::record_packed_layout_refresh":
    raise SystemExit("record-packed-layout verifier family drift")
if verifier.get("result") != "VerifiedHakoFamilyIR":
    raise SystemExit("record-packed-layout verifier result drift")
if verifier.get("pilot_scope") != "RecordPackedLayoutRefresh_prepared_record_packed_layout_refresh_only":
    raise SystemExit("record-packed-layout verifier scope drift")
checks = verifier.get("checks") or {}
expected_checks = {
    "record_packed_layout_refresh_only": 1,
    "canonical_json_parity": 1,
    "plan_kind": "MirBuilderRecordPackedLayoutRefreshPlanV1",
    "plan_subject": "MirBuilder::finalize_module record/packed layout refresh",
    "publication_target_count": 9,
    "projected_field_count": 9,
    "mutation_target_count": 9,
    "entrypoint": "semantic_refresh::refresh_module_record_and_packed_layout_plans",
    "refresh_timing": "AfterModuleMetadataPublicationBeforeTypedObjectRefresh",
    "record_packed_layout_refresh": 1,
    "full_finalize_module": 0,
    "runtime_fallback": 0,
}
for key, expected in expected_checks.items():
    if checks.get(key) != expected:
        raise SystemExit(f"record-packed-layout verifier check drift: {key}={checks.get(key)}")
PY

rm -f "$EXPECTED" "$RAW" "$OUT" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_record_packed_layout_refresh.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_record_packed_layout_refresh.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in record-packed-layout MIR")
metadata = main.get("metadata") or {}
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
required_defs = {
    "RecordPackedLayoutRefreshFixtureApi.build_plan/0",
    "RecordPackedLayoutRefreshFixtureApi.build_shadow_candidate/0",
    "RecordPackedLayoutRefreshApi.project_shadow_record/6",
}
missing = sorted(required_defs - symbols)
if missing:
    raise SystemExit(f"missing record-packed-layout same-module definitions: {missing}")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mirbuilder_record_packed_layout_refresh.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mirbuilder_record_packed_layout_refresh.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
mirbuilder_record_packed_layout_refresh_derived_hako=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-record-packed-layout-refresh-derived-artifact-v0
family_id=hakorune_mir_builder::record_packed_layout_refresh
record_packed_layout_refresh_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
generated_hako_manual_edit=0
record_packed_layout_refresh=1
record_packed_layout_field_value_type_refresh=0
record_packed_layout_collection_field_element_refresh=0
module_metadata_publication=0
semantic_refresh=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
