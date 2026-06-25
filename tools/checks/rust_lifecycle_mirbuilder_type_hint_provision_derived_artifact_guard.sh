#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-type-hint-provision"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_type_hint_provision.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_type_hint_provision.artifact.json"
VERIFIER="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-type-hint-provision-derived-hako-verifier-result-v0.json"
EXE="/tmp/hako_mirbuilder_type_hint_provision"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"
MIR_JSON="$EXE.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

hako = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_type_hint_provision.hako").read_text()
required = [
    "box TypeHintFunctionShellBox",
    "await_case_seen: i64",
    "call_global_case_seen: i64",
    "call_constructor_case_seen: i64",
    "call_unknown_case_seen: i64",
    "box TypeHintModuleShellBox",
    "box TypeHintTypeContextShellBox",
    "box TypeHintProvisionResultBox",
    "TypeHintProvisionApi",
    "run(fn_state, module_state, type_ctx): TypeHintProvisionResultBox",
    "fn_state.await_case_seen = 1",
    "fn_state.call_global_case_seen = 1",
    "fn_state.call_constructor_case_seen = 1",
    "fn_state.call_unknown_case_seen = 1",
    "type_ctx.value_types_inserted = 4",
    "type_ctx.value_origin_newbox_inserted = 1",
    "result.metadata_publication = 0",
    "mirbuilder_type_hint_provision_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing type hint artifact text: {missing}")
for forbidden in [
    "metadata_value_type_publication",
    "metadata_origin_caller_merge",
    "phi_return_type_inference",
    "phi_input_materialization",
    "module_function_insertion",
    "full_finalize_module",
    "runtime_fallback",
]:
    if forbidden in hako:
        raise SystemExit(f"type hint artifact opened non-selected behavior: {forbidden}")

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_type_hint_provision.artifact.json").read_text())
if manifest.get("family_id") != "hakorune_mir_builder::type_hint_provision":
    raise SystemExit("type hint manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("type hint artifact must remain DerivedShadow")
claims = manifest.get("claims") or {}
expected_claims = {
    "type_hint_provision": 1,
    "metadata_value_type_publication": 0,
    "metadata_origin_caller_merge": 0,
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
        raise SystemExit(f"type hint claim drift: {key}={claims.get(key)}")

verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-type-hint-provision-derived-hako-verifier-result-v0.json").read_text())
checks = verifier.get("checks") or {}
expected_checks = {
    "type_hint_provision_only": 1,
    "entrypoint": "type_hint_providers::annotate_missing_result_types_from_calls_and_await",
    "function_transport": "MirFunctionPreparedMain",
    "module_transport": "MirModulePreparedMain",
    "value_types": "self.type_ctx.value_types",
    "minimal_path_expected_result": "OkImplicitUnit",
    "metadata_value_type_publication": 0,
    "full_finalize_module": 0,
    "runtime_fallback": 0,
}
for key, expected in expected_checks.items():
    if checks.get(key) != expected:
        raise SystemExit(f"type hint verifier check drift: {key}={checks.get(key)}")
if checks.get("provider_cases") != [
    "Await",
    "Call(Global)",
    "Call(Constructor)",
    "Call(OtherOrMissingCallee)",
]:
    raise SystemExit(f"type hint provider cases drift: {checks.get('provider_cases')}")
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_type_hint_provision.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_type_hint_provision.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in type hint MIR")
metadata = main.get("metadata") or {}
routes = metadata.get("global_call_routes") or []
matches = [route for route in routes if route.get("callee_name") == "TypeHintProvisionApi.run/3"]
if len(matches) != 1:
    raise SystemExit(f"expected one type hint route, got {len(matches)}")
route = matches[0]
if route.get("reason") is not None:
    raise SystemExit(f"type hint route was not direct: {route}")
if route.get("proof") != "typed_global_call_same_module_object_handle":
    raise SystemExit(f"type hint route proof drift: {route}")
if route.get("definition_owner") != "uniform_mir":
    raise SystemExit(f"type hint route should use uniform_mir definition: {route}")
if route.get("target_result_box_name") != "TypeHintProvisionResultBox":
    raise SystemExit(f"type hint result box drift: {route}")
if route.get("value_demand") != "runtime_i64_or_handle":
    raise SystemExit(f"type hint value demand drift: {route}")
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
if "TypeHintProvisionApi.run/3" not in symbols:
    raise SystemExit("missing type hint same-module definition")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mirbuilder_type_hint_provision.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mirbuilder_type_hint_provision.err
sed '/^Result: /d' "$RAW" >"$OUT"

printf '%s\n' "mirbuilder_type_hint_provision_derived_hako=ok" >"$EXPECTED"
diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-type-hint-provision-derived-artifact-v0
family_id=hakorune_mir_builder::type_hint_provision
type_hint_provision_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
type_hint_provision=1
metadata_value_type_publication=0
metadata_origin_caller_merge=0
full_finalize_module=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
