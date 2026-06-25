#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-bounded-finalize-composition"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_bounded_finalize_composition.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_bounded_finalize_composition.artifact.json"
EXE="/tmp/hako_mirbuilder_bounded_finalize_composition"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"
MIR_JSON="$EXE.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

hako = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_bounded_finalize_composition.hako").read_text()
required = [
    "box FinalizedMirModuleShellBox",
    "box FinalizedMirFunctionShellBox",
    "box FinalizedBasicBlockShellBox",
    "box PreparedFinalizeStateShellBox",
    "BoundedFinalizeCompositionApi",
    "finalize(state, module, func, block, result_value, published_type_is_integer)",
    "block.terminated = 1",
    "block.return_value = result_value",
    "func.return_type_is_integer = published_type_is_integer",
    "module.condition_fn_present = 1",
    "module.record_packed_layout_refreshed = 1",
    "module.typed_object_plan_refreshed = 1",
    "module.direct_state_plan_refreshed = 1",
    "module.all_functions_phi_materialized = 1",
    "state.current_module_present = 0",
    "mirbuilder_bounded_finalize_composition_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing bounded finalize artifact text: {missing}")
for forbidden in [
    "full_finalize_module",
    "build_module",
    "runtime_fallback",
    "Option::Some(module)",
    "Option::Some(func)",
]:
    if forbidden in hako:
        raise SystemExit(f"bounded finalize artifact opened non-selected behavior: {forbidden}")

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_bounded_finalize_composition.artifact.json").read_text())
if manifest.get("family_id") != "hakorune_mir_builder::bounded_finalize_composition":
    raise SystemExit("bounded finalize manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("bounded finalize artifact must remain DerivedShadow")
claims = manifest.get("claims") or {}
expected_claims = {
    "bounded_finalize_composition": 1,
    "full_finalize_module": 0,
    "full_build_module_execution": 0,
    "reusable_return_emission": 0,
    "reusable_type_publication": 0,
    "current_module_take_artifact": 0,
    "current_function_take_artifact": 0,
    "mainline_selected": 0,
    "source_selfhost_claim": 0,
    "backend_behavior_changed": 0,
    "runtime_fallback": 0,
    "new_backend_route": 0,
    "new_abi": 0,
}
for key, expected in expected_claims.items():
    if claims.get(key) != expected:
        raise SystemExit(f"bounded finalize claim drift: {key}={claims.get(key)}")

verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-bounded-finalize-derived-hako-verifier-result-v0.json").read_text())
checks = verifier.get("checks") or {}
for key in [
    "bounded_finalize_composition_only",
    "return_instruction_shell_connected",
    "return_type_integer_published",
    "module_function_count_includes_condition_fn",
    "condition_fn_injection_source_required",
    "metadata_publication_shell",
    "semantic_refresh_subset_shell",
    "state_take_presence_tags_cleared",
]:
    if checks.get(key) != 1:
        raise SystemExit(f"bounded finalize verifier check missing: {key}")
for key in ["full_finalize_module", "full_build_module_execution", "mainline_selected", "runtime_fallback"]:
    if checks.get(key) != 0:
        raise SystemExit(f"bounded finalize verifier non-claim drift: {key}")
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_bounded_finalize_composition.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_bounded_finalize_composition.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in bounded finalize MIR")
metadata = main.get("metadata") or {}
routes = metadata.get("global_call_routes") or []
matches = [route for route in routes if route.get("callee_name") == "BoundedFinalizeCompositionApi.finalize/6"]
if len(matches) != 1:
    raise SystemExit(f"expected one bounded finalize route, got {len(matches)}")
route = matches[0]
if route.get("reason") is not None:
    raise SystemExit(f"bounded finalize route was not direct: {route}")
if route.get("definition_owner") != "uniform_mir":
    raise SystemExit(f"bounded finalize route should use uniform_mir definition: {route}")
if route.get("target_result_box_name") != "FinalizedMirModuleShellBox":
    raise SystemExit(f"bounded finalize result box drift: {route}")
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
if "BoundedFinalizeCompositionApi.finalize/6" not in symbols:
    raise SystemExit("missing bounded finalize same-module definition")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mirbuilder_bounded_finalize_composition.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mirbuilder_bounded_finalize_composition.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
mirbuilder_bounded_finalize_composition_derived_hako=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-bounded-finalize-derived-artifact-v0
family_id=hakorune_mir_builder::bounded_finalize_composition
bounded_finalize_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
bounded_finalize_composition=1
full_finalize_module=0
full_build_module_execution=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
