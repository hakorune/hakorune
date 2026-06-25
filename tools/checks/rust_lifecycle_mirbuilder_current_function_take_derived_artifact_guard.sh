#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-current-function-take"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_current_function_take.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_current_function_take.artifact.json"
VERIFIER="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-current-function-take-derived-hako-verifier-result-v0.json"
EXE="/tmp/hako_mirbuilder_current_function_take"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"
MIR_JSON="$EXE.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

hako = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_current_function_take.hako").read_text()
required = [
    "box PreparedCurrentFunctionStateShellBox",
    "current_function: MirFunctionPreparedMainBox",
    "current_function_present: i64",
    "taken_function_present: i64",
    "box MirFunctionPreparedMainBox",
    "signature: FunctionSignaturePrepared",
    "entry_block: i64",
    "next_value_id: i64",
    "CurrentFunctionTakeApi",
    "take(state): MirFunctionPreparedMainBox",
    "state.current_function_present = 0",
    "state.taken_function_present = 1",
    "return state.current_function",
    "mirbuilder_current_function_take_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing current function take artifact text: {missing}")
for forbidden in [
    "TypePropagationPipeline",
    "type_propagation",
    "type_hint_provision",
    "metadata_value_type_publication",
    "phi_return_type_inference",
    "phi_input_materialization",
    "module_function_insertion",
    "full_finalize_module",
    "runtime_fallback",
]:
    if forbidden in hako:
        raise SystemExit(f"current function take artifact opened non-selected behavior: {forbidden}")

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_current_function_take.artifact.json").read_text())
if manifest.get("family_id") != "hakorune_mir_builder::current_function_take":
    raise SystemExit("current function take manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("current function take artifact must remain DerivedShadow")
claims = manifest.get("claims") or {}
expected_claims = {
    "current_function_take": 1,
    "type_propagation": 0,
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
        raise SystemExit(f"current function take claim drift: {key}={claims.get(key)}")

verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-current-function-take-derived-hako-verifier-result-v0.json").read_text())
checks = verifier.get("checks") or {}
expected_checks = {
    "current_function_take_only": 1,
    "function_transport": "MirFunctionPreparedMain",
    "post_take_state": "None",
    "taken_function_present": 1,
    "type_propagation": 0,
    "type_hint_provision": 0,
    "full_finalize_module": 0,
    "runtime_fallback": 0,
}
for key, expected in expected_checks.items():
    if checks.get(key) != expected:
        raise SystemExit(f"current function take verifier check drift: {key}={checks.get(key)}")
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_current_function_take.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_current_function_take.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in current function take MIR")
metadata = main.get("metadata") or {}
routes = metadata.get("global_call_routes") or []
matches = [route for route in routes if route.get("callee_name") == "CurrentFunctionTakeApi.take/1"]
if len(matches) != 1:
    raise SystemExit(f"expected one current function take route, got {len(matches)}")
route = matches[0]
if route.get("reason") is not None:
    raise SystemExit(f"current function take route was not direct: {route}")
if route.get("proof") != "typed_global_call_same_module_object_handle":
    raise SystemExit(f"current function take route proof drift: {route}")
if route.get("definition_owner") != "uniform_mir":
    raise SystemExit(f"current function take route should use uniform_mir definition: {route}")
if route.get("target_result_box_name") != "MirFunctionPreparedMainBox":
    raise SystemExit(f"current function take result box drift: {route}")
if route.get("value_demand") != "runtime_i64_or_handle":
    raise SystemExit(f"current function take value demand drift: {route}")
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
if "CurrentFunctionTakeApi.take/1" not in symbols:
    raise SystemExit("missing current function take same-module definition")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mirbuilder_current_function_take.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mirbuilder_current_function_take.err
sed '/^Result: /d' "$RAW" >"$OUT"

printf '%s\n' "mirbuilder_current_function_take_derived_hako=ok" >"$EXPECTED"
diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-current-function-take-derived-artifact-v0
family_id=hakorune_mir_builder::current_function_take
current_function_take_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
current_function_take=1
type_propagation=0
type_hint_provision=0
full_finalize_module=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
