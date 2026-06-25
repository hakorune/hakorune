#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-phi-return-type-inference"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_phi_return_type_inference.hako"
EXE="/tmp/hako_mirbuilder_phi_return_type_inference"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"
MIR_JSON="$EXE.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

hako = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_phi_return_type_inference.hako").read_text()
required = [
    "box PhiReturnBuilderShellBox",
    "box PhiReturnFunctionShellBox",
    "box PhiReturnTypeInferenceResultBox",
    "PhiReturnTypeInferenceApi",
    "infer(builder_state, fn_state): PhiReturnTypeInferenceResultBox",
    "builder_state.direct_value_type_lookup = 1",
    "builder_state.type_hint_policy_checked = 1",
    "builder_state.method_return_hint_checked = 1",
    "builder_state.phi_type_resolver_checked = 1",
    "builder_state.generic_type_resolver_checked = 1",
    "fn_state.signature_return_type_is_integer = 1",
    "fn_state.inferred_return_type_present = 1",
    "result.resolver_steps = 8",
    "result.phi_input_materialization = 0",
    "result.full_finalize_module = 0",
    "mirbuilder_phi_return_type_inference_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing PHI return-type artifact text: {missing}")
for forbidden in [
    "phi_input_materializer::materialize_all_phi_inputs",
    "module.add_function",
    "runtime_fallback",
]:
    if forbidden in hako:
        raise SystemExit(f"PHI return-type artifact opened non-selected behavior: {forbidden}")

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_phi_return_type_inference.artifact.json").read_text())
if manifest.get("family_id") != "hakorune_mir_builder::phi_return_type_inference":
    raise SystemExit("PHI return-type manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("PHI return-type artifact must remain DerivedShadow")
claims = manifest.get("claims") or {}
expected_claims = {
    "phi_return_type_inference": 1,
    "phi_input_materialization": 0,
    "module_function_insertion": 0,
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
        raise SystemExit(f"PHI return-type claim drift: {key}={claims.get(key)}")

verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-phi-return-type-inference-derived-hako-verifier-result-v0.json").read_text())
checks = verifier.get("checks") or {}
expected_checks = {
    "phi_return_type_inference_only": 1,
    "entrypoint": "phi_type_inference::infer_return_type_from_phi",
    "function_transport": "MirFunctionPreparedMain",
    "builder_type_context": "self.type_ctx.value_types",
    "minimal_path_expected_result": "Option<MirType>",
    "phi_input_materialization": 0,
    "full_finalize_module": 0,
    "runtime_fallback": 0,
}
for key, expected in expected_checks.items():
    if checks.get(key) != expected:
        raise SystemExit(f"PHI return-type verifier check drift: {key}={checks.get(key)}")
if checks.get("resolver_chain") != [
    "SkipConcreteReturnType",
    "TerminatorReturnOnly",
    "DirectValueTypesLookup",
    "TypeHintPolicyExtract",
    "MethodReturnHintBox",
    "PhiTypeResolver",
    "GenericTypeResolver",
    "UnknownFallbackOutsideDebug",
]:
    raise SystemExit("PHI return-type resolver chain drift")
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_phi_return_type_inference.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_phi_return_type_inference.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in PHI return-type MIR")
metadata = main.get("metadata") or {}
routes = metadata.get("global_call_routes") or []
matches = [route for route in routes if route.get("callee_name") == "PhiReturnTypeInferenceApi.infer/2"]
if len(matches) != 1:
    raise SystemExit(f"expected one PHI return-type route, got {len(matches)}")
route = matches[0]
if route.get("reason") is not None:
    raise SystemExit(f"PHI return-type route was not direct: {route}")
if route.get("proof") != "typed_global_call_same_module_object_handle":
    raise SystemExit(f"PHI return-type route proof drift: {route}")
if route.get("definition_owner") != "uniform_mir":
    raise SystemExit(f"PHI return-type route should use uniform_mir definition: {route}")
if route.get("target_result_box_name") != "PhiReturnTypeInferenceResultBox":
    raise SystemExit(f"PHI return-type result box drift: {route}")
if route.get("value_demand") != "runtime_i64_or_handle":
    raise SystemExit(f"PHI return-type value demand drift: {route}")
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
if "PhiReturnTypeInferenceApi.infer/2" not in symbols:
    raise SystemExit("missing PHI return-type same-module definition")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mirbuilder_phi_return_type_inference.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mirbuilder_phi_return_type_inference.err
sed '/^Result: /d' "$RAW" >"$OUT"

printf '%s\n' "mirbuilder_phi_return_type_inference_derived_hako=ok" >"$EXPECTED"
diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-phi-return-type-inference-derived-artifact-v0
family_id=hakorune_mir_builder::phi_return_type_inference
phi_return_type_inference_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
phi_return_type_inference=1
phi_input_materialization=0
full_finalize_module=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
