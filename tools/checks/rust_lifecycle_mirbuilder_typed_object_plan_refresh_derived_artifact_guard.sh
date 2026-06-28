#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-typed-object-plan-refresh"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_typed_object_plan_refresh.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_typed_object_plan_refresh.artifact.json"
RECIPE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-typed-object-plan-refresh-derived-hako-recipe-v0.json"
VERIFIER="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-typed-object-plan-refresh-derived-hako-verifier-result-v0.json"
EXE="/tmp/hako_mirbuilder_typed_object_plan_refresh"
EXPECTED="/tmp/hako_mirbuilder_typed_object_plan_refresh.expected"
RAW="/tmp/hako_mirbuilder_typed_object_plan_refresh.out.raw"
OUT="/tmp/hako_mirbuilder_typed_object_plan_refresh.out"
MIR_JSON="/tmp/hako_mirbuilder_typed_object_plan_refresh.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import hashlib
import json
from pathlib import Path

artifact_path = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_typed_object_plan_refresh.hako")
manifest_path = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_typed_object_plan_refresh.artifact.json")
recipe_path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-typed-object-plan-refresh-derived-hako-recipe-v0.json")
verifier_path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-typed-object-plan-refresh-derived-hako-verifier-result-v0.json")

hako = artifact_path.read_text()
required = [
    "box TypedObjectPlanRefreshPayloadBox",
    "shadow_json: StringBox",
    "TypedObjectPlanRefreshFixtureApi",
    "TypedObjectPlanRefreshApi",
    "TypedObjectPlanRefreshResultBox",
    "TypedObjectPlanRefreshShadowCandidateV1",
    "module.metadata.typed_object_plans",
    "AfterRecordPackedLayoutRefreshBeforeDirectStateRefresh",
    "result.shadow_record = hako_shadow.payload",
    "result.shadow_json = hako_shadow.shadow_json",
    "mirbuilder_typed_object_plan_refresh_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing typed-object artifact text: {missing}")

manifest = json.loads(manifest_path.read_text())
if manifest.get("kind") != "RustDerivedHakoArtifact":
    raise SystemExit("typed-object manifest kind drift")
if manifest.get("family_id") != "hakorune_mir_builder::typed_object_plan_refresh":
    raise SystemExit("typed-object manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("typed-object artifact must remain DerivedShadow")
if manifest.get("pilot_scope") != "TypedObjectPlanRefresh_prepared_typed_object_plan_refresh_only":
    raise SystemExit("typed-object pilot scope drift")
generator = manifest.get("generator") or {}
if generator.get("tool") != "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-typed-object-plan-refresh":
    raise SystemExit("typed-object generator drift")
if manifest.get("output", {}).get("hako_path") != "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_typed_object_plan_refresh.hako":
    raise SystemExit("typed-object output path drift")
if manifest.get("output", {}).get("hako_sha256") != hashlib.sha256(artifact_path.read_bytes()).hexdigest():
    raise SystemExit("typed-object artifact hash stale")
claims = manifest.get("claims") or {}
expected_claims = {
    "typed_object_plan_refresh": 1,
    "typed_object_field_value_type_refresh": 0,
    "typed_object_collection_field_element_refresh": 0,
    "module_metadata_publication": 0,
    "direct_state_plan_refresh": 0,
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
        raise SystemExit(f"typed-object claim drift: {key}={claims.get(key)}")

recipe = json.loads(recipe_path.read_text())
if recipe.get("family_id") != "hakorune_mir_builder::typed_object_plan_refresh":
    raise SystemExit("typed-object recipe family drift")
if recipe.get("pilot_scope") != "TypedObjectPlanRefresh_prepared_typed_object_plan_refresh_only":
    raise SystemExit("typed-object recipe scope drift")
if recipe.get("selected_body_count") != "typed_object_plan_refresh_prepared_typed_object_plan_refresh_only":
    raise SystemExit("typed-object selected body count drift")
methods = {method["id"]: method for method in recipe.get("methods", [])}
method = methods.get("TypedObjectPlanRefreshApi::project_shadow_record")
if method is None:
    raise SystemExit("missing typed-object recipe method")
if method.get("hako_operation") != "StaticCall + SetField + ReturnValue":
    raise SystemExit("typed-object recipe hako operation drift")

verifier = json.loads(verifier_path.read_text())
if verifier.get("kind") != "DerivedHakoArtifactVerifierResult":
    raise SystemExit("typed-object verifier kind drift")
if verifier.get("family_id") != "hakorune_mir_builder::typed_object_plan_refresh":
    raise SystemExit("typed-object verifier family drift")
if verifier.get("result") != "VerifiedHakoFamilyIR":
    raise SystemExit("typed-object verifier result drift")
if verifier.get("pilot_scope") != "TypedObjectPlanRefresh_prepared_typed_object_plan_refresh_only":
    raise SystemExit("typed-object verifier scope drift")
checks = verifier.get("checks") or {}
expected_checks = {
    "typed_object_plan_refresh_only": 1,
    "canonical_json_parity": 1,
    "plan_kind": "MirBuilderTypedObjectPlanRefreshPlanV1",
    "plan_subject": "MirBuilder::finalize_module typed object plan refresh",
    "publication_target_count": 1,
    "projected_field_count": 5,
    "mutation_target_count": 1,
    "entrypoint": "typed_object_plan::refresh_module_typed_object_plans",
    "refresh_timing": "AfterRecordPackedLayoutRefreshBeforeDirectStateRefresh",
    "direct_state_plan_refresh": 0,
    "full_finalize_module": 0,
    "runtime_fallback": 0,
}
for key, expected in expected_checks.items():
    if checks.get(key) != expected:
        raise SystemExit(f"typed-object verifier check drift: {key}={checks.get(key)}")
PY

rm -f "$EXPECTED" "$RAW" "$OUT" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_typed_object_plan_refresh.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_typed_object_plan_refresh.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in typed-object MIR")
metadata = main.get("metadata") or {}
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
required_defs = {
    "TypedObjectPlanRefreshFixtureApi.build_plan/0",
    "TypedObjectPlanRefreshFixtureApi.build_shadow_candidate/0",
    "TypedObjectPlanRefreshApi.project_shadow_record/6",
}
missing = sorted(required_defs - symbols)
if missing:
    raise SystemExit(f"missing typed-object same-module definitions: {missing}")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mirbuilder_typed_object_plan_refresh.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mirbuilder_typed_object_plan_refresh.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
mirbuilder_typed_object_plan_refresh_derived_hako=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-typed-object-plan-refresh-derived-artifact-v0
family_id=hakorune_mir_builder::typed_object_plan_refresh
typed_object_plan_refresh_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
typed_object_plan_refresh=1
typed_object_field_value_type_refresh=0
typed_object_collection_field_element_refresh=0
module_metadata_publication=0
direct_state_plan_refresh=0
semantic_refresh=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
