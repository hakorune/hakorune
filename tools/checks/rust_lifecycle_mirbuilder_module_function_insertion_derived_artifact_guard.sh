#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-module-function-insertion"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_module_function_insertion.hako"
EXE="/tmp/hako_mirbuilder_module_function_insertion"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"
MIR_JSON="$EXE.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

hako = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_module_function_insertion.hako").read_text()
required = [
    "using apps.lib.collections.ordered_map as OrderedMap",
    "box ModuleFunctionInsertionModuleShellBox",
    "box ModuleFunctionInsertionFunctionShellBox",
    "box ModuleFunctionInsertionResultBox",
    "ModuleFunctionInsertionApi",
    "insert(module_state, function_state): ModuleFunctionInsertionResultBox",
    "module_state.functions.set(function_state.name, function_state)",
    "module_state.function_count = 1",
    "function_state.inserted = 1",
    "result.collision_policy_replace = 1",
    "result.condition_fn_injection = 0",
    "result.full_finalize_module = 0",
    "module_state.functions.length()",
    "mirbuilder_module_function_insertion_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing module function insertion artifact text: {missing}")
for forbidden in [
    "condition_fn_missing",
    "all_functions_phi_materialization",
    "runtime_fallback",
]:
    if forbidden in hako:
        raise SystemExit(f"module function insertion artifact opened non-selected behavior: {forbidden}")

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_module_function_insertion.artifact.json").read_text())
if manifest.get("family_id") != "hakorune_mir_builder::module_function_insertion":
    raise SystemExit("module function insertion manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("module function insertion artifact must remain DerivedShadow")
claims = manifest.get("claims") or {}
expected_claims = {
    "module_function_insertion": 1,
    "condition_fn_injection": 0,
    "all_functions_phi_materialization": 0,
    "region_stack_pop": 0,
    "slot_registry_release": 0,
    "metadata_publication": 0,
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
        raise SystemExit(f"module function insertion claim drift: {key}={claims.get(key)}")

verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-module-function-insertion-derived-hako-verifier-result-v0.json").read_text())
checks = verifier.get("checks") or {}
expected_checks = {
    "module_function_insertion_only": 1,
    "entrypoint": "MirModule::add_function",
    "module_transport": "MirModuleMinimalShell",
    "function_transport": "MirFunctionPreparedMain",
    "context": "finalize_module",
    "container": "MirModule.functions",
    "container_operation": "BTreeMap::insert",
    "hako_operation": "OrderedMapBox.set",
    "collision_policy": "ReplaceExistingByName",
    "mutation_frame": ["module.functions"],
    "condition_fn_injection": 0,
    "full_finalize_module": 0,
    "runtime_fallback": 0,
}
for key, expected in expected_checks.items():
    if checks.get(key) != expected:
        raise SystemExit(f"module function insertion verifier check drift: {key}={checks.get(key)}")
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_module_function_insertion.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_module_function_insertion.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in module function insertion MIR")
metadata = main.get("metadata") or {}
routes = metadata.get("global_call_routes") or []
matches = [route for route in routes if route.get("callee_name") == "ModuleFunctionInsertionApi.insert/2"]
if len(matches) != 1:
    raise SystemExit(f"expected one module function insertion route, got {len(matches)}")
route = matches[0]
if route.get("reason") is not None:
    raise SystemExit(f"module function insertion route was not direct: {route}")
if route.get("proof") != "typed_global_call_same_module_object_handle":
    raise SystemExit(f"module function insertion route proof drift: {route}")
if route.get("definition_owner") != "uniform_mir":
    raise SystemExit(f"module function insertion route should use uniform_mir definition: {route}")
if route.get("target_result_box_name") != "ModuleFunctionInsertionResultBox":
    raise SystemExit(f"module function insertion result box drift: {route}")
if route.get("value_demand") != "runtime_i64_or_handle":
    raise SystemExit(f"module function insertion value demand drift: {route}")
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
if "ModuleFunctionInsertionApi.insert/2" not in symbols:
    raise SystemExit("missing module function insertion same-module definition")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mirbuilder_module_function_insertion.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mirbuilder_module_function_insertion.err
sed '/^Result: /d' "$RAW" >"$OUT"

printf '%s\n' "mirbuilder_module_function_insertion_derived_hako=ok" >"$EXPECTED"
diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-module-function-insertion-derived-artifact-v0
family_id=hakorune_mir_builder::module_function_insertion
module_function_insertion_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
module_function_insertion=1
condition_fn_injection=0
full_finalize_module=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
