#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-prepared-state-install"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_prepared_state_install.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_prepared_state_install.artifact.json"
EXE="/tmp/hako_mirbuilder_prepared_state_install"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"
MIR_JSON="$EXE.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

hako = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_prepared_state_install.hako").read_text()
required = [
    "box PreparedScopeContextShellBox",
    "current_function_present: i64",
    "box PreparedMirBuilderStateShellBox",
    "current_module_present: i64",
    "current_block_present: i64",
    "PreparedMirBuilderStateShellApi.empty",
    "PreparedMirBuilderStateShellApi.install",
    "state.current_module = module",
    "state.current_module_present = 1",
    "state.scope_ctx.current_function = func",
    "state.scope_ctx.current_function_present = 1",
    "state.current_block = entry_block",
    "state.current_block_present = 1",
    "mirbuilder_prepared_state_install_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing prepared-state install artifact text: {missing}")
for forbidden in [
    "current_module.take",
    "current_function.take",
    "lower_root",
    "build_literal",
    "finalize_module",
    "Option::Some(module)",
    "Option::Some(func)",
]:
    if forbidden in hako:
        raise SystemExit(f"prepared-state install artifact opened non-selected behavior: {forbidden}")

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_prepared_state_install.artifact.json").read_text())
if manifest.get("family_id") != "hakorune_mir_builder::prepared_state_install":
    raise SystemExit("prepared-state install manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("prepared-state install artifact must remain DerivedShadow")
claims = manifest.get("claims") or {}
expected_claims = {
    "prepared_state_install": 1,
    "current_module_take": 0,
    "current_function_take": 0,
    "lower_root": 0,
    "literal_integer_lowering": 0,
    "return_emission": 0,
    "finalize_module": 0,
    "full_mirbuilder_object_transport": 0,
    "mainline_selected": 0,
    "source_selfhost_claim": 0,
    "backend_behavior_changed": 0,
    "runtime_fallback": 0,
    "new_backend_route": 0,
    "new_abi": 0,
}
for key, expected in expected_claims.items():
    if claims.get(key) != expected:
        raise SystemExit(f"prepared-state install claim drift: {key}={claims.get(key)}")

verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-prepared-state-install-derived-hako-verifier-result-v0.json").read_text())
checks = verifier.get("checks") or {}
for key in [
    "prepared_state_install_only",
    "current_module_installed",
    "current_function_installed",
    "current_block_installed",
    "fresh_state_identity",
]:
    if checks.get(key) != 1:
        raise SystemExit(f"prepared-state install verifier check missing: {key}")
for key in ["current_module_take", "current_function_take", "lower_root", "finalize_module", "backend_behavior_changed", "runtime_fallback"]:
    if checks.get(key) != 0:
        raise SystemExit(f"prepared-state install verifier non-claim drift: {key}")
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_prepared_state_install.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_prepared_state_install.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in prepared-state install MIR")
metadata = main.get("metadata") or {}
routes = metadata.get("global_call_routes") or []
expected_routes = {
    "PreparedMirBuilderStateShellApi.empty/0": "PreparedMirBuilderStateShellBox",
    "PreparedMirBuilderStateShellApi.install/4": "PreparedMirBuilderStateShellBox",
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
    "PreparedMirBuilderStateShellApi.empty/0",
    "PreparedMirBuilderStateShellApi.install/4",
    "PreparedScopeContextShellApi.create/0",
    "MirModuleMinimalShellApi.create/1",
    "MirFunctionConstructorShellApi.create/2",
}
missing_defs = sorted(required_defs - symbols)
if missing_defs:
    raise SystemExit(f"missing same-module definitions: {missing_defs}")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mirbuilder_prepared_state_install.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mirbuilder_prepared_state_install.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
mirbuilder_prepared_state_install_derived_hako=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-prepared-state-install-derived-artifact-v0
family_id=hakorune_mir_builder::prepared_state_install
prepared_state_install_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
prepared_state_install=1
current_module_take=0
current_function_take=0
lower_root=0
literal_integer_lowering=0
finalize_module=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
