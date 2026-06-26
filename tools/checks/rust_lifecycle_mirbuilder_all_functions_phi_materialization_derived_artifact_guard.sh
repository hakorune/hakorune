#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-all-functions-phi-materialization"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_all_functions_phi_materialization.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_all_functions_phi_materialization.artifact.json"
RECIPE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-all-functions-phi-materialization-derived-hako-recipe-v0.json"
VERIFIER="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-all-functions-phi-materialization-derived-hako-verifier-result-v0.json"
EXE="/tmp/hako_mirbuilder_all_functions_phi_materialization"
EXPECTED="/tmp/hako_mirbuilder_all_functions_phi_materialization.expected"
RAW="/tmp/hako_mirbuilder_all_functions_phi_materialization.out.raw"
OUT="/tmp/hako_mirbuilder_all_functions_phi_materialization.out"
MIR_JSON="/tmp/hako_mirbuilder_all_functions_phi_materialization.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import hashlib
import json
from pathlib import Path

artifact_path = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_all_functions_phi_materialization.hako")
manifest_path = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_all_functions_phi_materialization.artifact.json")
recipe_path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-all-functions-phi-materialization-derived-hako-recipe-v0.json")
verifier_path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-all-functions-phi-materialization-derived-hako-verifier-result-v0.json")

hako = artifact_path.read_text()
required = [
    "box AllFunctionsPhiMaterializationPayloadBox",
    "shadow_json: StringBox",
    "AllFunctionsPhiMaterializationFixtureApi",
    "AllFunctionsPhiMaterializationApi",
    "AllFunctionsPhiMaterializationResultBox",
    "AllFunctionsPhiMaterializationShadowCandidateV1",
    "module.functions.values_mut()",
    "phi_input_materializer::materialize_all_phi_inputs",
    "finalize_module_all_functions",
    "AfterTypedObjectPlanRefreshBeforeAllFunctionsPhiMaterialization",
    "result.shadow_record = hako_shadow.payload",
    "result.shadow_json = hako_shadow.shadow_json",
    "mirbuilder_all_functions_phi_materialization_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing all-functions PHI artifact text: {missing}")

manifest = json.loads(manifest_path.read_text())
if manifest.get("kind") != "RustDerivedHakoArtifact":
    raise SystemExit("all-functions PHI manifest kind drift")
if manifest.get("family_id") != "hakorune_mir_builder::all_functions_phi_materialization":
    raise SystemExit("all-functions PHI manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("all-functions PHI artifact must remain DerivedShadow")
if manifest.get("pilot_scope") != "AllFunctionsPhiMaterialization_prepared_all_functions_phi_materialization_only":
    raise SystemExit("all-functions PHI pilot scope drift")
generator = manifest.get("generator") or {}
if generator.get("tool") != "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-all-functions-phi-materialization":
    raise SystemExit("all-functions PHI generator drift")
if manifest.get("output", {}).get("hako_path") != "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_all_functions_phi_materialization.hako":
    raise SystemExit("all-functions PHI output path drift")
if manifest.get("output", {}).get("hako_sha256") != hashlib.sha256(artifact_path.read_bytes()).hexdigest():
    raise SystemExit("all-functions PHI artifact hash stale")
claims = manifest.get("claims") or {}
expected_claims = {
    "generated_hako_manual_edit": 0,
    "all_functions_phi_materialization": 1,
    "full_finalize_module": 0,
    "generated_hako_artifact": 0,
    "backend_route_changed": 0,
    "abi_changed": 0,
    "runtime_fallback": 0,
    "mainline_selected": 0,
    "source_selfhost_claim": 0,
}
for key, expected in expected_claims.items():
    if claims.get(key) != expected:
        raise SystemExit(f"all-functions PHI claim drift: {key}={claims.get(key)}")

recipe = json.loads(recipe_path.read_text())
if recipe.get("family_id") != "hakorune_mir_builder::all_functions_phi_materialization":
    raise SystemExit("all-functions PHI recipe family drift")
if recipe.get("pilot_scope") != "AllFunctionsPhiMaterialization_prepared_all_functions_phi_materialization_only":
    raise SystemExit("all-functions PHI recipe scope drift")
if recipe.get("selected_body_count") != "all_functions_phi_materialization_prepared_all_functions_phi_materialization_only":
    raise SystemExit("all-functions PHI selected body count drift")
methods = {method["id"]: method for method in recipe.get("methods", [])}
method = methods.get("AllFunctionsPhiMaterializationApi::project_shadow_record")
if method is None:
    raise SystemExit("missing all-functions PHI recipe method")
if method.get("hako_operation") != "StaticCall + SetField + ReturnValue":
    raise SystemExit("all-functions PHI recipe hako operation drift")

verifier = json.loads(verifier_path.read_text())
if verifier.get("kind") != "DerivedHakoArtifactVerifierResult":
    raise SystemExit("all-functions PHI verifier kind drift")
if verifier.get("family_id") != "hakorune_mir_builder::all_functions_phi_materialization":
    raise SystemExit("all-functions PHI verifier family drift")
if verifier.get("result") != "VerifiedHakoFamilyIR":
    raise SystemExit("all-functions PHI verifier result drift")
if verifier.get("pilot_scope") != "AllFunctionsPhiMaterialization_prepared_all_functions_phi_materialization_only":
    raise SystemExit("all-functions PHI verifier scope drift")
checks = verifier.get("checks") or {}
expected_checks = {
    "all_functions_phi_materialization_only": 1,
    "canonical_json_parity": 1,
    "plan_kind": "MirBuilderAllFunctionsPhiMaterializationPlanV1",
    "plan_subject": "MirBuilder::finalize_module all-functions PHI materialization",
    "publication_target_count": 1,
    "projected_field_count": 8,
    "mutation_target_count": 1,
    "entrypoint": "MirBuilder::finalize_module all-functions PHI materialization sweep",
    "refresh_timing": "AfterTypedObjectPlanRefreshBeforeAllFunctionsPhiMaterialization",
    "delegate": "phi_input_materializer::materialize_all_phi_inputs",
    "delegate_context": "finalize_module_all_functions",
    "all_functions_phi_materialization": 1,
    "full_finalize_module": 0,
    "runtime_fallback": 0,
}
for key, expected in expected_checks.items():
    if checks.get(key) != expected:
        raise SystemExit(f"all-functions PHI verifier check drift: {key}={checks.get(key)}")
PY

rm -f "$EXPECTED" "$RAW" "$OUT" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_all_functions_phi_materialization.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_all_functions_phi_materialization.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in all-functions PHI MIR")
metadata = main.get("metadata") or {}
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
required_defs = {
    "AllFunctionsPhiMaterializationFixtureApi.build_plan/0",
    "AllFunctionsPhiMaterializationFixtureApi.build_shadow_candidate/0",
    "AllFunctionsPhiMaterializationApi.project_shadow_record/6",
}
missing = sorted(required_defs - symbols)
if missing:
    raise SystemExit(f"missing all-functions PHI same-module definitions: {missing}")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mirbuilder_all_functions_phi_materialization.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mirbuilder_all_functions_phi_materialization.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
mirbuilder_all_functions_phi_materialization_derived_hako=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-all-functions-phi-materialization-derived-artifact-v0
family_id=hakorune_mir_builder::all_functions_phi_materialization
all_functions_phi_materialization_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
generated_hako_manual_edit=0
all_functions_phi_materialization=1
runtime_fallback=0
backend_route_changed=0
summary=ok
REPORT
