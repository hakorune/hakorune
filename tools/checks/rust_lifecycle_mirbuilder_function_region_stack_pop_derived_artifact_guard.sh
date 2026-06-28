#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-function-region-stack-pop"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_function_region_stack_pop.hako"
MIR_JSON="/tmp/hako_mirbuilder_function_region_stack_pop_artifact.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import hashlib
import json
from pathlib import Path

artifact_path = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_function_region_stack_pop.hako")
manifest_path = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_function_region_stack_pop.artifact.json")
recipe_path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-function-region-stack-pop-derived-hako-recipe-v0.json")
verifier_path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-function-region-stack-pop-derived-hako-verifier-result-v0.json")

hako = artifact_path.read_text()
required = [
    "box PreparedRegionTraceStateBox",
    "trace_enabled: i64",
    "current_region_stack: ArrayBox",
    "stack_size_before: i64",
    "stack_size_after: i64",
    "pop_attempted: i64",
    "static box FunctionRegionStackPopApi",
    "pop_option(stack): Option<i64>",
    "apply(state): i64",
    "state.stack_size_before = before",
    "state.stack_size_after = after",
    "state.pop_attempted = 0",
    "state.pop_attempted = 1",
    "FunctionRegionStackPopApi.pop_option(state.current_region_stack)",
    "mirbuilder_function_region_stack_pop_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing function region stack pop artifact text: {missing}")
for forbidden in [
    "NYASH_REGION_TRACE",
    "std::env",
    "host env",
    "current_slot_registry",
    "slot_registry_release",
    "module_metadata_publication",
    "semantic_refresh",
    "full_finalize_module",
    "backend_behavior_changed",
    "runtime_fallback",
]:
    if forbidden in hako:
        raise SystemExit(f"function region stack pop artifact opened forbidden text: {forbidden}")

manifest = json.loads(manifest_path.read_text())
if manifest.get("kind") != "RustDerivedHakoArtifact":
    raise SystemExit("function region stack pop manifest kind drift")
if manifest.get("family_id") != "hakorune_mir_builder::function_region_stack_pop":
    raise SystemExit("function region stack pop manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("function region stack pop artifact must remain DerivedShadow")
if manifest.get("pilot_scope") != "FunctionRegionStackPop_prepared_region_trace_only":
    raise SystemExit("function region stack pop pilot scope drift")
generator = manifest.get("generator") or {}
if generator.get("tool") != "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-function-region-stack-pop":
    raise SystemExit("function region stack pop generator drift")
if manifest.get("output", {}).get("hako_path") != "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_function_region_stack_pop.hako":
    raise SystemExit("function region stack pop output path drift")
if manifest.get("output", {}).get("hako_sha256") != hashlib.sha256(artifact_path.read_bytes()).hexdigest():
    raise SystemExit("function region stack pop artifact hash stale")
claims = manifest.get("claims") or {}
expected_claims = {
    "function_region_stack_pop": 1,
    "slot_registry_release": 0,
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
        raise SystemExit(f"function region stack pop claim drift: {key}={claims.get(key)}")

recipe = json.loads(recipe_path.read_text())
if recipe.get("family_id") != "hakorune_mir_builder::function_region_stack_pop":
    raise SystemExit("function region stack pop recipe family drift")
if recipe.get("pilot_scope") != "FunctionRegionStackPop_prepared_region_trace_only":
    raise SystemExit("function region stack pop recipe scope drift")
if recipe.get("selected_body_count") != "function_region_stack_pop_prepared_region_trace_only":
    raise SystemExit("function region stack pop selected body count drift")
methods = {method["id"]: method for method in recipe.get("methods", [])}
for method_id, hako_operation in {
    "FunctionRegionStackPopApi::pop_option": "SequencePopOption",
    "FunctionRegionStackPopApi::apply": "MethodCall + SetField + IfElse + StaticCall + ReturnI64",
}.items():
    method = methods.get(method_id)
    if method is None:
        raise SystemExit(f"missing function region stack pop recipe method: {method_id}")
    if method.get("hako_operation") != hako_operation:
        raise SystemExit(f"function region stack pop recipe hako operation drift: {method_id}")

verifier = json.loads(verifier_path.read_text())
if verifier.get("kind") != "DerivedHakoArtifactVerifierResult":
    raise SystemExit("function region stack pop verifier kind drift")
if verifier.get("family_id") != "hakorune_mir_builder::function_region_stack_pop":
    raise SystemExit("function region stack pop verifier family drift")
if verifier.get("result") != "VerifiedHakoFamilyIR":
    raise SystemExit("function region stack pop verifier result drift")
if verifier.get("pilot_scope") != "FunctionRegionStackPop_prepared_region_trace_only":
    raise SystemExit("function region stack pop verifier scope drift")
checks = verifier.get("checks") or {}
expected_checks = {
    "function_region_stack_pop_only": 1,
    "trace_flag_transport": "RegionTraceEnabledI64BoolV0",
    "stack_transport": "ArrayBox",
    "stack_element_transport": "RegionIdAsI64",
    "apply_result_transport": "ScalarI64",
    "apply_result_semantics": "Unit",
    "trace_disabled_noop": 1,
    "trace_enabled_pop_once": 1,
    "trace_enabled_empty_safe": 1,
    "result_discarded": 1,
    "host_env_lookup": 0,
    "slot_registry_release": 0,
    "metadata_publication": 0,
    "semantic_refresh": 0,
    "all_functions_phi_materialization": 0,
    "full_finalize_module": 0,
    "runtime_fallback": 0,
}
for key, expected in expected_checks.items():
    if checks.get(key) != expected:
        raise SystemExit(f"function region stack pop verifier check drift: {key}={checks.get(key)}")
PY

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_function_region_stack_pop_artifact.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_function_region_stack_pop_artifact.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in function region stack pop MIR")
metadata = main.get("metadata") or {}
routes = metadata.get("global_call_routes") or []
if not routes:
    raise SystemExit("missing function region stack pop MIR routes")
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
required_defs = {
    "FunctionRegionStackPopApi.pop_option/1",
    "FunctionRegionStackPopApi.apply/1",
}
missing_defs = sorted(required_defs - symbols)
if missing_defs:
    raise SystemExit(f"missing function region stack pop same-module definitions: {missing_defs}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-function-region-stack-pop-derived-artifact-v0
family_id=hakorune_mir_builder::function_region_stack_pop
function_region_stack_pop_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
state=DerivedShadow
mainline_selected=0
function_region_stack_pop=1
slot_registry_release=0
metadata_publication=0
semantic_refresh=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
