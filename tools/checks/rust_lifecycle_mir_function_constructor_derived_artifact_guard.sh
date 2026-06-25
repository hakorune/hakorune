#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mir-function-constructor-shell"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mir_function_constructor_shell.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/mir_function_constructor_shell.artifact.json"
EXE="/tmp/hako_mir_function_constructor_shell"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"
MIR_JSON="$EXE.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

hako = Path("lang/generated/rust_derived/hakorune_mir_builder/mir_function_constructor_shell.hako").read_text()
required = [
    "box FunctionSignaturePrepared",
    "box BasicBlockConstructorShellBox",
    "box FunctionBlockTableShell",
    "box FunctionMetadataDefaultShell",
    "box MirFunctionConstructorShellBox",
    "FunctionSignaturePreparedApi.create",
    "BasicBlockConstructorShellApi.create",
    "FunctionBlockTableShellApi.create",
    "MirFunctionConstructorShellApi.create",
    "func.params.push(param_index)",
    "if func.next_value_id < 1",
    "mir_function_constructor_shell_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing MirFunction constructor artifact text: {missing}")
for forbidden in [
    "add_instruction",
    "add_block",
    "reserve_parameter_value_ids",
    "finalize",
    "MirFunctionConstructorShellApi.next_value_id",
    "MirFunctionConstructorShellApi.add",
]:
    if forbidden in hako:
        raise SystemExit(f"MirFunction constructor artifact opened non-selected API: {forbidden}")

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mir_function_constructor_shell.artifact.json").read_text())
if manifest.get("family_id") != "hakorune_mir::MirFunctionConstructorShell":
    raise SystemExit("MirFunction constructor manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("MirFunction constructor artifact must remain DerivedShadow")
claims = manifest.get("claims") or {}
expected_claims = {
    "mir_function_constructor_shell": 1,
    "basic_block_constructor_shell": 1,
    "separate_block_only_claim": 0,
    "function_body_lowering": 0,
    "instruction_emission": 0,
    "parameter_setup_compatibility_fallback": 0,
    "reserve_parameter_value_ids_call": 0,
    "function_finalization": 0,
    "prepared_state_install": 0,
    "full_mir_function_conversion": 0,
    "mainline_selected": 0,
    "source_selfhost_claim": 0,
    "backend_behavior_changed": 0,
    "runtime_fallback": 0,
    "new_backend_route": 0,
    "new_abi": 0,
}
for key, expected in expected_claims.items():
    if claims.get(key) != expected:
        raise SystemExit(f"MirFunction constructor claim drift: {key}={claims.get(key)}")

verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mir-function-constructor-derived-hako-verifier-result-v0.json").read_text())
checks = verifier.get("checks") or {}
for key in [
    "constructor_shell_only",
    "basic_block_child_constructor",
    "entry_block_only_table",
    "params_prepopulated",
    "next_value_id_seed_max_param_count_1",
    "fresh_params_identity",
    "fresh_entry_block_instruction_identity",
]:
    if checks.get(key) != 1:
        raise SystemExit(f"MirFunction constructor verifier check missing: {key}")
for key in ["function_body_lowering", "instruction_emission", "backend_behavior_changed", "runtime_fallback"]:
    if checks.get(key) != 0:
        raise SystemExit(f"MirFunction constructor verifier non-claim drift: {key}")
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mir_function_constructor_shell.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mir_function_constructor_shell.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in MirFunction constructor MIR")
metadata = main.get("metadata") or {}
routes = metadata.get("global_call_routes") or []
expected_routes = {
    "FunctionSignaturePreparedApi.create/2": "FunctionSignaturePrepared",
    "MirFunctionConstructorShellApi.create/2": "MirFunctionConstructorShellBox",
}
for callee, result_box in expected_routes.items():
    matches = [route for route in routes if route.get("callee_name") == callee]
    if not matches:
        raise SystemExit(f"missing route for {callee}")
    for route in matches:
        if route.get("reason") is not None:
            raise SystemExit(f"{callee} route was not direct: {route}")
        if route.get("definition_owner") != "uniform_mir":
            raise SystemExit(f"{callee} route should use uniform_mir definition: {route}")
        if route.get("target_result_box_name") != result_box:
            raise SystemExit(f"{callee} result box drift: {route}")
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
required_defs = {
    "FunctionSignaturePreparedApi.create/2",
    "MirFunctionConstructorShellApi.create/2",
    "FunctionBlockTableShellApi.create/1",
    "BasicBlockConstructorShellApi.create/1",
}
missing_defs = sorted(required_defs - symbols)
if missing_defs:
    raise SystemExit(f"missing same-module definitions: {missing_defs}")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mir_function_constructor_shell.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mir_function_constructor_shell.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
mir_function_constructor_shell_derived_hako=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mir-function-constructor-derived-artifact-v0
family_id=hakorune_mir::MirFunctionConstructorShell
mir_function_constructor_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
constructor_shell_only=1
basic_block_child_constructor=1
entry_block_only_table=1
params_prepopulated=1
next_value_id_seed_max_param_count_1=1
prepared_state_install=0
function_body_lowering=0
instruction_emission=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
