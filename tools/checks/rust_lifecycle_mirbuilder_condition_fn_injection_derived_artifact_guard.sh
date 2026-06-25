#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-condition-fn-injection"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_condition_fn_injection.hako"
EXE="/tmp/hako_mirbuilder_condition_fn_injection"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"
MIR_JSON="$EXE.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

hako = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_condition_fn_injection.hako").read_text()
required = [
    "using apps.lib.collections.ordered_map as OrderedMap",
    "box ConditionFnInjectionModuleShellBox",
    "box ConditionFnInjectionFunctionShellBox",
    "box ConditionFnInjectionResultBox",
    "ConditionFnInjectionApi",
    "inject_if_missing(module_state): ConditionFnInjectionResultBox",
    "module_state.functions.has(stub.name)",
    "module_state.functions.set(stub.name, stub)",
    "ConditionFnInjectionStubApi.create()",
    "ConditionFnInjectionFunctionShellApi.create(\"condition_fn\", 1, 1, 1, 0, 1, 1)",
    "mirbuilder_condition_fn_injection_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing condition_fn injection artifact text: {missing}")
for forbidden in [
    "pop_function_region",
    "current_slot_registry",
    "metadata_publication",
    "semantic_refresh",
    "all_functions_phi_materialization",
    "runtime_fallback",
]:
    if forbidden in hako:
        raise SystemExit(f"condition_fn injection artifact opened non-selected behavior: {forbidden}")

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_condition_fn_injection.artifact.json").read_text())
if manifest.get("family_id") != "hakorune_mir_builder::condition_fn_injection":
    raise SystemExit("condition_fn injection manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("condition_fn injection artifact must remain DerivedShadow")
claims = manifest.get("claims") or {}
expected_claims = {
    "condition_fn_injection": 1,
    "condition_fn_policy_generalization": 0,
    "region_stack_pop": 0,
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
        raise SystemExit(f"condition_fn injection claim drift: {key}={claims.get(key)}")

verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-condition-fn-injection-derived-hako-verifier-result-v0.json").read_text())
checks = verifier.get("checks") or {}
expected_checks = {
    "condition_fn_injection_only": 1,
    "entrypoint": "MirBuilder::finalize_module condition_fn injection block",
    "module_transport": "MirModuleMinimalShell",
    "context": "finalize_module",
    "predicate": "module.functions.get(\"condition_fn\").is_none()",
    "function_name": "condition_fn",
    "param_count": 1,
    "return_type": "MirType::Integer",
    "effects": "EffectMask::PURE",
    "entry_block": 0,
    "body": ["ConstInteger(1)", "ReturnValue(one)"],
    "hako_operation": "OrderedMapBox.has + OrderedMapBox.set",
    "mutation_frame": ["module.functions"],
    "region_stack_pop": 0,
    "slot_registry_release": 0,
    "full_finalize_module": 0,
    "runtime_fallback": 0,
}
for key, expected in expected_checks.items():
    if checks.get(key) != expected:
        raise SystemExit(f"condition_fn injection verifier check drift: {key}={checks.get(key)}")
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_condition_fn_injection.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_condition_fn_injection.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in condition_fn injection MIR")
metadata = main.get("metadata") or {}
routes = metadata.get("global_call_routes") or []
matches = [route for route in routes if route.get("callee_name") == "ConditionFnInjectionApi.inject_if_missing/1"]
if len(matches) < 1:
    raise SystemExit("missing condition_fn injection route")
for route in matches:
    if route.get("reason") is not None:
        raise SystemExit(f"condition_fn injection route was not direct: {route}")
    if route.get("proof") != "typed_global_call_same_module_object_handle":
        raise SystemExit(f"condition_fn injection route proof drift: {route}")
    if route.get("definition_owner") != "uniform_mir":
        raise SystemExit(f"condition_fn injection route should use uniform_mir definition: {route}")
    if route.get("target_result_box_name") != "ConditionFnInjectionResultBox":
        raise SystemExit(f"condition_fn injection result box drift: {route}")
    if route.get("value_demand") != "runtime_i64_or_handle":
        raise SystemExit(f"condition_fn injection value demand drift: {route}")
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
if "ConditionFnInjectionApi.inject_if_missing/1" not in symbols:
    raise SystemExit("missing condition_fn injection same-module definition")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mirbuilder_condition_fn_injection.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mirbuilder_condition_fn_injection.err
sed '/^Result: /d' "$RAW" >"$OUT"

printf '%s\n' "mirbuilder_condition_fn_injection_derived_hako=ok" >"$EXPECTED"
diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-condition-fn-injection-derived-artifact-v0
family_id=hakorune_mir_builder::condition_fn_injection
condition_fn_injection_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
condition_fn_injection=1
region_stack_pop=0
slot_registry_release=0
full_finalize_module=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
