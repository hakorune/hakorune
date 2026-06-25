#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-phi-input-materialization"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_phi_input_materialization.hako"
EXE="/tmp/hako_mirbuilder_phi_input_materialization"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"
MIR_JSON="$EXE.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

hako = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_phi_input_materialization.hako").read_text()
required = [
    "box PhiInputFunctionShellBox",
    "box PhiInputMaterializationResultBox",
    "PhiInputMaterializationApi",
    "run(fn_state): PhiInputMaterializationResultBox",
    "fn_state.prune_unused_phi_instructions = 1",
    "fn_state.complete_missing_self_carried_phi_inputs = 1",
    "fn_state.collect_phi_input_worklist = 1",
    "fn_state.build_def_blocks_and_dominators = 1",
    "fn_state.rematerialize_incoming_per_pred_with_memo = 1",
    "fn_state.rewrite_phi_input_slots = 1",
    "fn_state.return_changed_count = 1",
    "result.materialization_steps = 7",
    "result.dev_birth_verification = 0",
    "result.full_finalize_module = 0",
    "mirbuilder_phi_input_materialization_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing PHI input materialization artifact text: {missing}")
for forbidden in [
    "using_is_dev",
    "module.add_function",
    "condition_fn",
    "runtime_fallback",
]:
    if forbidden in hako:
        raise SystemExit(f"PHI input artifact opened non-selected behavior: {forbidden}")

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_phi_input_materialization.artifact.json").read_text())
if manifest.get("family_id") != "hakorune_mir_builder::phi_input_materialization":
    raise SystemExit("PHI input materialization manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("PHI input materialization artifact must remain DerivedShadow")
claims = manifest.get("claims") or {}
expected_claims = {
    "phi_input_materialization": 1,
    "dev_birth_verification": 0,
    "module_function_insertion": 0,
    "condition_fn_injection": 0,
    "all_functions_phi_materialization": 0,
    "semantic_refresh": 0,
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
        raise SystemExit(f"PHI input materialization claim drift: {key}={claims.get(key)}")

verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-phi-input-materialization-derived-hako-verifier-result-v0.json").read_text())
checks = verifier.get("checks") or {}
expected_checks = {
    "phi_input_materialization_only": 1,
    "entrypoint": "phi_input_materializer::materialize_all_phi_inputs",
    "function_transport": "MirFunctionPreparedMain",
    "context": "finalize_module",
    "minimal_path_expected_result": "Result<usize, String>",
    "dev_birth_verification": 0,
    "full_finalize_module": 0,
    "runtime_fallback": 0,
}
for key, expected in expected_checks.items():
    if checks.get(key) != expected:
        raise SystemExit(f"PHI input materialization verifier check drift: {key}={checks.get(key)}")
if checks.get("materialization_steps") != [
    "PruneUnusedPhiInstructions",
    "CompleteMissingSelfCarriedPhiInputs",
    "CollectPhiInputWorklist",
    "BuildDefBlocksAndDominators",
    "RematerializeIncomingPerPredWithMemo",
    "RewritePhiInputSlots",
    "ReturnChangedCount",
]:
    raise SystemExit("PHI input materialization step drift")
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_phi_input_materialization.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_phi_input_materialization.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in PHI input MIR")
metadata = main.get("metadata") or {}
routes = metadata.get("global_call_routes") or []
matches = [route for route in routes if route.get("callee_name") == "PhiInputMaterializationApi.run/1"]
if len(matches) != 1:
    raise SystemExit(f"expected one PHI input route, got {len(matches)}")
route = matches[0]
if route.get("reason") is not None:
    raise SystemExit(f"PHI input route was not direct: {route}")
if route.get("proof") != "typed_global_call_same_module_object_handle":
    raise SystemExit(f"PHI input route proof drift: {route}")
if route.get("definition_owner") != "uniform_mir":
    raise SystemExit(f"PHI input route should use uniform_mir definition: {route}")
if route.get("target_result_box_name") != "PhiInputMaterializationResultBox":
    raise SystemExit(f"PHI input result box drift: {route}")
if route.get("value_demand") != "runtime_i64_or_handle":
    raise SystemExit(f"PHI input value demand drift: {route}")
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
if "PhiInputMaterializationApi.run/1" not in symbols:
    raise SystemExit("missing PHI input same-module definition")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mirbuilder_phi_input_materialization.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mirbuilder_phi_input_materialization.err
sed '/^Result: /d' "$RAW" >"$OUT"

printf '%s\n' "mirbuilder_phi_input_materialization_derived_hako=ok" >"$EXPECTED"
diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-phi-input-materialization-derived-artifact-v0
family_id=hakorune_mir_builder::phi_input_materialization
phi_input_materialization_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
phi_input_materialization=1
dev_birth_verification=0
full_finalize_module=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
