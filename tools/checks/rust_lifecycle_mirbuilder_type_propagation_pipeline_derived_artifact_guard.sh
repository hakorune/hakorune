#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-type-propagation-pipeline"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_type_propagation_pipeline.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_type_propagation_pipeline.artifact.json"
VERIFIER="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-type-propagation-pipeline-derived-hako-verifier-result-v0.json"
EXE="/tmp/hako_mirbuilder_type_propagation_pipeline"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"
MIR_JSON="$EXE.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

hako = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_type_propagation_pipeline.hako").read_text()
required = [
    "box TypePropagationFunctionShellBox",
    "seed_declared_field_types: i64",
    "copy_propagation_initial: i64",
    "binop_repropagation: i64",
    "copy_propagation_after_binop: i64",
    "phi_type_inference: i64",
    "box TypePropagationValueTypesShellBox",
    "box TypePropagationPipelineResultBox",
    "TypePropagationPipelineApi",
    "run(fn_state, value_types): TypePropagationPipelineResultBox",
    "fn_state.seed_declared_field_types = 1",
    "fn_state.copy_propagation_initial = 1",
    "fn_state.binop_repropagation = 1",
    "fn_state.copy_propagation_after_binop = 1",
    "fn_state.phi_type_inference = 1",
    "value_types.mutated = 1",
    "result.steps_run = 5",
    "mirbuilder_type_propagation_pipeline_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing type propagation artifact text: {missing}")
for forbidden in [
    "type_hint_provision",
    "metadata_value_type_publication",
    "phi_return_type_inference",
    "phi_input_materialization",
    "module_function_insertion",
    "full_finalize_module",
    "runtime_fallback",
]:
    if forbidden in hako:
        raise SystemExit(f"type propagation artifact opened non-selected behavior: {forbidden}")

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_type_propagation_pipeline.artifact.json").read_text())
if manifest.get("family_id") != "hakorune_mir_builder::type_propagation_pipeline":
    raise SystemExit("type propagation manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("type propagation artifact must remain DerivedShadow")
claims = manifest.get("claims") or {}
expected_claims = {
    "type_propagation": 1,
    "type_hint_provision": 0,
    "metadata_value_type_publication": 0,
    "phi_return_type_inference": 0,
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
        raise SystemExit(f"type propagation claim drift: {key}={claims.get(key)}")

verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-type-propagation-pipeline-derived-hako-verifier-result-v0.json").read_text())
checks = verifier.get("checks") or {}
expected_checks = {
    "type_propagation_only": 1,
    "entrypoint": "TypePropagationPipeline::run",
    "function_transport": "MirFunctionPreparedMain",
    "value_types": "self.type_ctx.value_types",
    "minimal_path_expected_result": "Ok",
    "type_hint_provision": 0,
    "full_finalize_module": 0,
    "runtime_fallback": 0,
}
for key, expected in expected_checks.items():
    if checks.get(key) != expected:
        raise SystemExit(f"type propagation verifier check drift: {key}={checks.get(key)}")
if checks.get("pipeline_steps") != [
    "seed_declared_field_types",
    "copy_propagation_initial",
    "binop_repropagation",
    "copy_propagation_after_binop",
    "phi_type_inference",
]:
    raise SystemExit(f"type propagation pipeline steps drift: {checks.get('pipeline_steps')}")
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_type_propagation_pipeline.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_type_propagation_pipeline.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in type propagation MIR")
metadata = main.get("metadata") or {}
routes = metadata.get("global_call_routes") or []
matches = [route for route in routes if route.get("callee_name") == "TypePropagationPipelineApi.run/2"]
if len(matches) != 1:
    raise SystemExit(f"expected one type propagation route, got {len(matches)}")
route = matches[0]
if route.get("reason") is not None:
    raise SystemExit(f"type propagation route was not direct: {route}")
if route.get("proof") != "typed_global_call_same_module_object_handle":
    raise SystemExit(f"type propagation route proof drift: {route}")
if route.get("definition_owner") != "uniform_mir":
    raise SystemExit(f"type propagation route should use uniform_mir definition: {route}")
if route.get("target_result_box_name") != "TypePropagationPipelineResultBox":
    raise SystemExit(f"type propagation result box drift: {route}")
if route.get("value_demand") != "runtime_i64_or_handle":
    raise SystemExit(f"type propagation value demand drift: {route}")
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
if "TypePropagationPipelineApi.run/2" not in symbols:
    raise SystemExit("missing type propagation same-module definition")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mirbuilder_type_propagation_pipeline.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mirbuilder_type_propagation_pipeline.err
sed '/^Result: /d' "$RAW" >"$OUT"

printf '%s\n' "mirbuilder_type_propagation_pipeline_derived_hako=ok" >"$EXPECTED"
diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-type-propagation-pipeline-derived-artifact-v0
family_id=hakorune_mir_builder::type_propagation_pipeline
type_propagation_pipeline_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
type_propagation=1
type_hint_provision=0
metadata_value_type_publication=0
full_finalize_module=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
