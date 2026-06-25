#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mir-module-minimal-shell"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mir_module_minimal_shell.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/mir_module_minimal_shell.artifact.json"
EXE="/tmp/hako_mir_module_minimal_shell"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"
MIR_JSON="$EXE.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

hako = Path("lang/generated/rust_derived/hakorune_mir_builder/mir_module_minimal_shell.hako").read_text()
required = [
    "using apps.lib.collections.ordered_map as OrderedMap",
    "box ModuleMetadataDefaultShell",
    "source_file: Option<StringBox>",
    "box MirModuleMinimalShellBox",
    "name: StringBox",
    "functions: OrderedMapBox",
    "globals: OrderedMapBox",
    "metadata: ModuleMetadataDefaultShell",
    "me.functions = OrderedMap.create()",
    "me.globals = OrderedMap.create()",
    "MirModuleMinimalShellApi.create",
    "mir_module_minimal_shell_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing MirModule shell artifact text: {missing}")
for forbidden in [
    "MirModuleMinimalShellApi.name",
    "MirModuleMinimalShellApi.source_file",
    "MirModuleMinimalShellApi.insert",
    "add_function",
    "set_source_file",
]:
    if forbidden in hako:
        raise SystemExit(f"MirModule shell artifact opened non-selected API: {forbidden}")

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mir_module_minimal_shell.artifact.json").read_text())
if manifest.get("family_id") != "hakorune_mir::MirModuleMinimalShell":
    raise SystemExit("MirModule shell manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("MirModule shell must remain DerivedShadow")
claims = manifest.get("claims") or {}
expected_claims = {
    "mir_module_minimal_shell": 1,
    "source_file_assignment": 0,
    "function_insertion": 0,
    "global_publication": 0,
    "metadata_publication": 0,
    "finalize_module": 0,
    "full_mir_module_conversion": 0,
    "full_mirbuilder_new": 0,
    "mainline_selected": 0,
    "source_selfhost_claim": 0,
    "backend_behavior_changed": 0,
    "runtime_fallback": 0,
    "new_backend_route": 0,
    "new_abi": 0,
}
for key, expected in expected_claims.items():
    if claims.get(key) != expected:
        raise SystemExit(f"MirModule shell claim drift: {key}={claims.get(key)}")

verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mir-module-minimal-shell-derived-hako-verifier-result-v0.json").read_text())
checks = verifier.get("checks") or {}
for key in [
    "constructor_shell_only",
    "module_name_preserved",
    "functions_initially_empty",
    "globals_initially_empty",
    "source_file_absent",
    "fresh_function_table_identity",
    "fresh_global_table_identity",
]:
    if checks.get(key) != 1:
        raise SystemExit(f"MirModule shell verifier check missing: {key}")
for key in ["function_insertion", "global_publication", "backend_behavior_changed", "runtime_fallback"]:
    if checks.get(key) != 0:
        raise SystemExit(f"MirModule shell verifier non-claim drift: {key}")
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mir_module_minimal_shell.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mir_module_minimal_shell.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in MirModule shell MIR")
metadata = main.get("metadata") or {}
routes = metadata.get("global_call_routes") or []
create_routes = [
    route for route in routes
    if route.get("callee_name") == "MirModuleMinimalShellApi.create/1"
]
if not create_routes:
    raise SystemExit("missing MirModuleMinimalShellApi.create route")
for route in create_routes:
    if route.get("reason") is not None:
        raise SystemExit(f"MirModule create route was not direct: {route}")
    if route.get("definition_owner") != "uniform_mir":
        raise SystemExit(f"MirModule create route should use uniform_mir definition: {route}")
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
if "MirModuleMinimalShellApi.create/1" not in symbols:
    raise SystemExit("missing same-module definition for MirModuleMinimalShellApi.create/1")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mir_module_minimal_shell.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mir_module_minimal_shell.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
mir_module_minimal_shell_derived_hako=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mir-module-minimal-shell-derived-artifact-v0
family_id=hakorune_mir::MirModuleMinimalShell
mir_module_minimal_shell_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
function_insertion=0
global_publication=0
metadata_publication=0
source_file_assignment=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
