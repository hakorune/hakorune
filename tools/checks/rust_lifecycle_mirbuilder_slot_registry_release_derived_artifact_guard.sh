#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-slot-registry-release"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_slot_registry_release.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_slot_registry_release.artifact.json"
RECIPE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-slot-registry-release-derived-hako-recipe-v0.json"
VERIFIER="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-slot-registry-release-derived-hako-verifier-result-v0.json"
MIR_JSON="/tmp/hako_mirbuilder_slot_registry_release.mir.json"
EXE="/tmp/hako_mirbuilder_slot_registry_release"
RAW="/tmp/hako_mirbuilder_slot_registry_release.out.raw"
OUT="/tmp/hako_mirbuilder_slot_registry_release.out"
EXPECTED="/tmp/hako_mirbuilder_slot_registry_release.expected"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import hashlib
import json
from pathlib import Path

artifact_path = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_slot_registry_release.hako")
manifest_path = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_slot_registry_release.artifact.json")
recipe_path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-slot-registry-release-derived-hako-recipe-v0.json")
verifier_path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-slot-registry-release-derived-hako-verifier-result-v0.json")

hako = artifact_path.read_text()
required = [
    "box PreparedSlotRegistryStateShellBox",
    "current_slot_registry: FunctionSlotRegistryPreparedBox",
    "current_slot_registry_present: i64",
    "released_registry_present: i64",
    "slot_registry_released: i64",
    "static box SlotRegistryReleaseApi",
    "release(state): FunctionSlotRegistryPreparedBox",
    "apply(state): i64",
    "local registry = state.current_slot_registry",
    "state.current_slot_registry = null",
    "state.current_slot_registry_present = 0",
    "state.released_registry_present = 1",
    "state.slot_registry_released = 1",
    "SlotRegistryReleaseApi.release(direct_state)",
    "SlotRegistryReleaseApi.apply(apply_state)",
    "mirbuilder_slot_registry_release_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing slot registry release artifact text: {missing}")
for forbidden in [
    "module_metadata_publication",
    "semantic_refresh",
    "full_finalize_module",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
]:
    if forbidden in hako:
        raise SystemExit(f"slot registry release artifact opened forbidden text: {forbidden}")

manifest = json.loads(manifest_path.read_text())
if manifest.get("kind") != "RustDerivedHakoArtifact":
    raise SystemExit("slot registry release manifest kind drift")
if manifest.get("family_id") != "hakorune_mir_builder::slot_registry_release":
    raise SystemExit("slot registry release manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("slot registry release artifact must remain DerivedShadow")
if manifest.get("pilot_scope") != "SlotRegistryRelease_prepared_slot_registry_only":
    raise SystemExit("slot registry release pilot scope drift")
generator = manifest.get("generator") or {}
if generator.get("tool") != "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-slot-registry-release":
    raise SystemExit("slot registry release generator drift")
if manifest.get("output", {}).get("hako_path") != "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_slot_registry_release.hako":
    raise SystemExit("slot registry release output path drift")
if manifest.get("output", {}).get("hako_sha256") != hashlib.sha256(artifact_path.read_bytes()).hexdigest():
    raise SystemExit("slot registry release artifact hash stale")
claims = manifest.get("claims") or {}
expected_claims = {
    "slot_registry_release": 1,
    "generated_hako_manual_edit": 0,
    "module_metadata_publication": 0,
    "metadata_publication": 0,
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
        raise SystemExit(f"slot registry release claim drift: {key}={claims.get(key)}")

recipe = json.loads(recipe_path.read_text())
if recipe.get("family_id") != "hakorune_mir_builder::slot_registry_release":
    raise SystemExit("slot registry release recipe family drift")
if recipe.get("pilot_scope") != "SlotRegistryRelease_prepared_slot_registry_only":
    raise SystemExit("slot registry release recipe scope drift")
if recipe.get("selected_body_count") != "slot_registry_release_prepared_slot_registry_only":
    raise SystemExit("slot registry release selected body count drift")
methods = {method["id"]: method for method in recipe.get("methods", [])}
for method_id, hako_operation in {
    "SlotRegistryReleaseApi::release": "Assign + SetField + ReturnValue",
    "SlotRegistryReleaseApi::apply": "StaticCall + ReturnI64",
}.items():
    method = methods.get(method_id)
    if method is None:
        raise SystemExit(f"missing slot registry release recipe method: {method_id}")
    if method.get("hako_operation") != hako_operation:
        raise SystemExit(f"slot registry release recipe hako operation drift: {method_id}")

verifier = json.loads(verifier_path.read_text())
if verifier.get("kind") != "DerivedHakoArtifactVerifierResult":
    raise SystemExit("slot registry release verifier kind drift")
if verifier.get("family_id") != "hakorune_mir_builder::slot_registry_release":
    raise SystemExit("slot registry release verifier family drift")
if verifier.get("result") != "VerifiedHakoFamilyIR":
    raise SystemExit("slot registry release verifier result drift")
if verifier.get("pilot_scope") != "SlotRegistryRelease_prepared_slot_registry_only":
    raise SystemExit("slot registry release verifier scope drift")
checks = verifier.get("checks") or {}
expected_checks = {
    "slot_registry_release_only": 1,
    "current_slot_registry_transport": "FunctionSlotRegistryPreparedBox",
    "release_result_transport": "FunctionSlotRegistryPreparedBox",
    "apply_result_transport": "ScalarI64",
    "apply_result_semantics": "Unit",
    "current_slot_registry_cleared": 1,
    "released_registry_present": 1,
    "slot_registry_released": 1,
    "module_metadata_publication": 0,
    "semantic_refresh": 0,
    "all_functions_phi_materialization": 0,
    "full_finalize_module": 0,
    "runtime_fallback": 0,
}
for key, expected in expected_checks.items():
    if checks.get(key) != expected:
        raise SystemExit(f"slot registry release verifier check drift: {key}={checks.get(key)}")
denied = verifier.get("denied_boundaries") or []
for boundary in [
    "module_metadata_publication",
    "metadata_publication",
    "semantic_refresh",
    "all_functions_phi_materialization",
    "full_finalize_module",
    "mainline_selected",
    "runtime_fallback",
]:
    if boundary not in denied:
        raise SystemExit(f"slot registry release denied boundary missing: {boundary}")
PY

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_slot_registry_release.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_slot_registry_release.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in slot registry release MIR")
metadata = main.get("metadata") or {}
routes = metadata.get("global_call_routes") or []
if not routes:
    raise SystemExit("missing slot registry release MIR routes")
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
required_defs = {
    "SlotRegistryReleaseApi.release/1",
    "SlotRegistryReleaseApi.apply/1",
}
missing_defs = sorted(required_defs - symbols)
if missing_defs:
    raise SystemExit(f"missing slot registry release same-module definitions: {missing_defs}")
release_route = next((route for route in routes if route.get("callee_name") == "SlotRegistryReleaseApi.release/1"), None)
if release_route is None:
    raise SystemExit("missing slot registry release direct route")
if release_route.get("reason") is not None:
    raise SystemExit(f"slot registry release route was not direct: {release_route}")
if release_route.get("definition_owner") != "uniform_mir":
    raise SystemExit(f"slot registry release route should use uniform_mir definition: {release_route}")
if release_route.get("target_result_box_name") != "FunctionSlotRegistryPreparedBox":
    raise SystemExit(f"slot registry release result box drift: {release_route}")
apply_route = next((route for route in routes if route.get("callee_name") == "SlotRegistryReleaseApi.apply/1"), None)
if apply_route is None:
    raise SystemExit("missing slot registry release apply route")
if apply_route.get("reason") is not None:
    raise SystemExit(f"slot registry release apply route was not direct: {apply_route}")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mirbuilder_slot_registry_release.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mirbuilder_slot_registry_release.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
mirbuilder_slot_registry_release_derived_hako=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-slot-registry-release-derived-artifact-v0
family_id=hakorune_mir_builder::slot_registry_release
slot_registry_release_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
slot_registry_release=1
module_metadata_publication=0
semantic_refresh=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
