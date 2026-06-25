#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-return-emission"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_return_emission.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_return_emission.artifact.json"
VERIFIER="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-return-emission-derived-hako-verifier-result-v0.json"
EXE="/tmp/hako_mirbuilder_return_emission"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"
MIR_JSON="$EXE.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

hako = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_return_emission.hako").read_text()
required = [
    "box ReturnEmissionBasicBlockShellBox",
    "terminated: i64",
    "return_value: i64",
    "return_value_present: i64",
    "successors_empty: i64",
    "ReturnEmissionApi",
    "emit(block, result_value): ReturnEmissionBasicBlockShellBox",
    "block.terminated = 1",
    "block.return_value = result_value",
    "block.return_value_present = 1",
    "block.successors_empty = 1",
    "mirbuilder_return_emission_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing return emission artifact text: {missing}")
for forbidden in [
    "return_type_publication",
    "full_finalize_module",
    "already_terminated_block_behavior",
    "runtime_fallback",
]:
    if forbidden in hako:
        raise SystemExit(f"return emission artifact opened non-selected behavior: {forbidden}")

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_return_emission.artifact.json").read_text())
if manifest.get("family_id") != "hakorune_mir_builder::return_emission":
    raise SystemExit("return emission manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("return emission artifact must remain DerivedShadow")
claims = manifest.get("claims") or {}
expected_claims = {
    "return_emission": 1,
    "return_type_publication": 0,
    "full_finalize_module": 0,
    "other_terminator_shapes": 0,
    "already_terminated_block_behavior": 0,
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
        raise SystemExit(f"return emission claim drift: {key}={claims.get(key)}")

verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-return-emission-derived-hako-verifier-result-v0.json").read_text())
checks = verifier.get("checks") or {}
expected_checks = {
    "return_emission_only": 1,
    "terminator_is_return": 1,
    "return_value_some": 1,
    "value_transport": "ValueIdAsI64",
    "successors_empty": 1,
    "return_type_publication": 0,
    "full_finalize_module": 0,
    "runtime_fallback": 0,
}
for key, expected in expected_checks.items():
    if checks.get(key) != expected:
        raise SystemExit(f"return emission verifier check drift: {key}={checks.get(key)}")
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_return_emission.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_return_emission.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in return emission MIR")
metadata = main.get("metadata") or {}
routes = metadata.get("global_call_routes") or []
matches = [route for route in routes if route.get("callee_name") == "ReturnEmissionApi.emit/2"]
if len(matches) != 1:
    raise SystemExit(f"expected one return emission route, got {len(matches)}")
route = matches[0]
if route.get("reason") is not None:
    raise SystemExit(f"return emission route was not direct: {route}")
if route.get("definition_owner") != "uniform_mir":
    raise SystemExit(f"return emission route should use uniform_mir definition: {route}")
if route.get("target_result_box_name") != "ReturnEmissionBasicBlockShellBox":
    raise SystemExit(f"return emission result box drift: {route}")
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
if "ReturnEmissionApi.emit/2" not in symbols:
    raise SystemExit("missing return emission same-module definition")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mirbuilder_return_emission.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mirbuilder_return_emission.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
mirbuilder_return_emission_derived_hako=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-return-emission-derived-artifact-v0
family_id=hakorune_mir_builder::return_emission
return_emission_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
return_emission=1
return_type_publication=0
full_finalize_module=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
