#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-current-module-take"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_current_module_take.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_current_module_take.artifact.json"
VERIFIER="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-current-module-take-derived-hako-verifier-result-v0.json"
EXE="/tmp/hako_mirbuilder_current_module_take"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"
MIR_JSON="$EXE.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

hako = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_current_module_take.hako").read_text()
required = [
    "box PreparedCurrentModuleStateShellBox",
    "current_module_present: i64",
    "taken_module_present: i64",
    "box CurrentModuleTakeModuleShellBox",
    "name_is_main: i64",
    "functions_empty: i64",
    "globals_empty: i64",
    "CurrentModuleTakeApi",
    "take(state, module): CurrentModuleTakeModuleShellBox",
    "state.current_module_present = 0",
    "state.taken_module_present = 1",
    "mirbuilder_current_module_take_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing current module take artifact text: {missing}")
for forbidden in [
    "verify_typed_values",
    "current_function_take",
    "full_finalize_module",
    "module_metadata_publication",
    "runtime_fallback",
]:
    if forbidden in hako:
        raise SystemExit(f"current module take artifact opened non-selected behavior: {forbidden}")

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_current_module_take.artifact.json").read_text())
if manifest.get("family_id") != "hakorune_mir_builder::current_module_take":
    raise SystemExit("current module take manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("current module take artifact must remain DerivedShadow")
claims = manifest.get("claims") or {}
expected_claims = {
    "current_module_take": 1,
    "verify_typed_values": 0,
    "current_function_take": 0,
    "full_finalize_module": 0,
    "module_metadata_publication": 0,
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
        raise SystemExit(f"current module take claim drift: {key}={claims.get(key)}")

verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-current-module-take-derived-hako-verifier-result-v0.json").read_text())
checks = verifier.get("checks") or {}
expected_checks = {
    "current_module_take_only": 1,
    "module_transport": "MirModuleMinimalShell",
    "post_take_state": "None",
    "taken_module_present": 1,
    "verify_typed_values": 0,
    "current_function_take": 0,
    "full_finalize_module": 0,
    "runtime_fallback": 0,
}
for key, expected in expected_checks.items():
    if checks.get(key) != expected:
        raise SystemExit(f"current module take verifier check drift: {key}={checks.get(key)}")
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_current_module_take.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_current_module_take.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in current module take MIR")
metadata = main.get("metadata") or {}
routes = metadata.get("global_call_routes") or []
matches = [route for route in routes if route.get("callee_name") == "CurrentModuleTakeApi.take/2"]
if len(matches) != 1:
    raise SystemExit(f"expected one current module take route, got {len(matches)}")
route = matches[0]
if route.get("reason") is not None:
    raise SystemExit(f"current module take route was not direct: {route}")
if route.get("definition_owner") != "uniform_mir":
    raise SystemExit(f"current module take route should use uniform_mir definition: {route}")
if route.get("target_result_box_name") != "CurrentModuleTakeModuleShellBox":
    raise SystemExit(f"current module take result box drift: {route}")
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
if "CurrentModuleTakeApi.take/2" not in symbols:
    raise SystemExit("missing current module take same-module definition")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mirbuilder_current_module_take.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mirbuilder_current_module_take.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
mirbuilder_current_module_take_derived_hako=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-current-module-take-derived-artifact-v0
family_id=hakorune_mir_builder::current_module_take
current_module_take_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
current_module_take=1
verify_typed_values=0
current_function_take=0
full_finalize_module=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
