#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-literal-integer-lowering"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_literal_integer_lowering.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_literal_integer_lowering.artifact.json"
EXE="/tmp/hako_mirbuilder_literal_integer_lowering"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"
MIR_JSON="$EXE.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

hako = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_literal_integer_lowering.hako").read_text()
required = [
    "box ConstIntegerInstructionShellBox",
    "dst: i64",
    "value: i64",
    "box PublishedIntegerTypeShellBox",
    "is_integer: i64",
    "box LiteralIntegerLoweringResultBox",
    "LiteralIntegerLoweringApi",
    "MirBuilderAllocationPolicyApi.next_value_id",
    "instruction.value = literal_value",
    "published.is_integer = 1",
    "mirbuilder_literal_integer_lowering_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing literal integer artifact text: {missing}")
for forbidden in [
    "finalize_module",
    "return_emission",
    "build_string",
    "LiteralValue::String",
    "Float",
    "Bool",
    "Null",
]:
    if forbidden in hako:
        raise SystemExit(f"literal integer artifact opened non-selected behavior: {forbidden}")

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_literal_integer_lowering.artifact.json").read_text())
if manifest.get("family_id") != "hakorune_mir_builder::literal_integer_lowering":
    raise SystemExit("literal integer manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("literal integer artifact must remain DerivedShadow")
claims = manifest.get("claims") or {}
expected_claims = {
    "literal_integer_lowering": 1,
    "allocation_policy_prepared_state_dependency": 1,
    "typed_integer_literal": 0,
    "float_literal": 0,
    "bool_literal": 0,
    "string_literal": 0,
    "null_literal": 0,
    "void_literal": 0,
    "full_expression_lowering": 0,
    "return_emission": 0,
    "finalize_module": 0,
    "mainline_selected": 0,
    "source_selfhost_claim": 0,
    "backend_behavior_changed": 0,
    "runtime_fallback": 0,
    "new_backend_route": 0,
    "new_abi": 0,
}
for key, expected in expected_claims.items():
    if claims.get(key) != expected:
        raise SystemExit(f"literal integer claim drift: {key}={claims.get(key)}")

dependency = (manifest.get("dependency_artifacts") or {}).get("allocation_policy") or {}
if dependency.get("path") != "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_next_value_id_prepared_state_kernel.artifact.json":
    raise SystemExit("literal integer allocation dependency drift")

verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-literal-integer-derived-hako-verifier-result-v0.json").read_text())
checks = verifier.get("checks") or {}
for key in [
    "literal_integer_lowering_only",
    "allocation_policy_dependency_verified",
    "allocates_result_value",
    "emits_const_integer_instruction_shell",
    "publishes_mir_type_integer_shell",
    "returns_value_id",
    "reserved_candidate_consumed",
]:
    if checks.get(key) != 1:
        raise SystemExit(f"literal integer verifier check missing: {key}")
for key in ["return_emission", "finalize_module", "backend_behavior_changed", "runtime_fallback"]:
    if checks.get(key) != 0:
        raise SystemExit(f"literal integer verifier non-claim drift: {key}")
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_literal_integer_lowering.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_literal_integer_lowering.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in literal integer MIR")
metadata = main.get("metadata") or {}
routes = metadata.get("global_call_routes") or []
matches = [route for route in routes if route.get("callee_name") == "LiteralIntegerLoweringApi.lower/5"]
if len(matches) != 2:
    raise SystemExit(f"expected two literal lowering routes, got {len(matches)}")
for route in matches:
    if route.get("reason") is not None:
        raise SystemExit(f"literal lowering route was not direct: {route}")
    if route.get("definition_owner") != "uniform_mir":
        raise SystemExit(f"literal lowering route should use uniform_mir definition: {route}")
    if route.get("target_result_box_name") != "LiteralIntegerLoweringResultBox":
        raise SystemExit(f"literal lowering result box drift: {route}")
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
required_defs = {
    "LiteralIntegerLoweringApi.lower/5",
    "MirBuilderAllocationPolicyApi.next_value_id/4",
    "FunctionValueIdCounterStateApi.next/1",
    "CoreContextApi.next_value/1",
    "ReservedValueIdMembershipViewApi.has/2",
}
missing_defs = sorted(required_defs - symbols)
if missing_defs:
    raise SystemExit(f"missing same-module definitions: {missing_defs}")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mirbuilder_literal_integer_lowering.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mirbuilder_literal_integer_lowering.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
mirbuilder_literal_integer_lowering_derived_hako=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-literal-integer-derived-artifact-v0
family_id=hakorune_mir_builder::literal_integer_lowering
literal_integer_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
literal_integer_lowering=1
allocation_policy_prepared_state_dependency=1
return_emission=0
finalize_module=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
