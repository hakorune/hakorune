#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-typed-value-verification"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_typed_value_verification.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_typed_value_verification.artifact.json"
VERIFIER="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-typed-value-verification-derived-hako-verifier-result-v0.json"
EXE="/tmp/hako_mirbuilder_typed_value_verification"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"
MIR_JSON="$EXE.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

hako = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_typed_value_verification.hako").read_text()
required = [
    "box TypedValueVerificationResultBox",
    "verified: i64",
    "missing_count: i64",
    "fatal_missing: i64",
    "stale_cleanup_count: i64",
    "TypedValueVerificationApi",
    "verify(result_value, typed_value, defined_value, param_value): TypedValueVerificationResultBox",
    "result.verified = 1",
    "result.missing_count = 0",
    "result.fatal_missing = 0",
    "result.stale_cleanup_count = 0",
    "mirbuilder_typed_value_verification_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing typed value verification artifact text: {missing}")
for forbidden in [
    "current_function_take",
    "type_propagation",
    "type_hint_provision",
    "phi_return_type_inference",
    "phi_input_materialization",
    "module_metadata_publication",
    "full_finalize_module",
    "runtime_fallback",
]:
    if forbidden in hako:
        raise SystemExit(f"typed value verification artifact opened non-selected behavior: {forbidden}")

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_typed_value_verification.artifact.json").read_text())
if manifest.get("family_id") != "hakorune_mir_builder::typed_value_verification":
    raise SystemExit("typed value verification manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("typed value verification artifact must remain DerivedShadow")
claims = manifest.get("claims") or {}
expected_claims = {
    "typed_value_verification": 1,
    "current_function_take": 0,
    "type_propagation": 0,
    "type_hint_provision": 0,
    "phi_return_type_inference": 0,
    "phi_input_materialization": 0,
    "module_metadata_publication": 0,
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
        raise SystemExit(f"typed value verification claim drift: {key}={claims.get(key)}")

verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-typed-value-verification-derived-hako-verifier-result-v0.json").read_text())
checks = verifier.get("checks") or {}
expected_checks = {
    "typed_value_verification_only": 1,
    "typed_values": "builder.type_ctx.value_types",
    "excluded_value": "ValueId::INVALID",
    "fail_fast_tag": "[freeze:contract][value_lifecycle/typed_without_def]",
    "minimal_path_expected_result": "Ok",
    "current_function_take": 0,
    "type_propagation": 0,
    "full_finalize_module": 0,
    "runtime_fallback": 0,
}
for key, expected in expected_checks.items():
    if checks.get(key) != expected:
        raise SystemExit(f"typed value verification verifier check drift: {key}={checks.get(key)}")
if checks.get("definition_sources") != ["compute_def_blocks(func)", "func.params"]:
    raise SystemExit(f"typed value verification definition sources drift: {checks.get('definition_sources')}")
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_typed_value_verification.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_typed_value_verification.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in typed value verification MIR")
metadata = main.get("metadata") or {}
routes = metadata.get("global_call_routes") or []
matches = [route for route in routes if route.get("callee_name") == "TypedValueVerificationApi.verify/4"]
if len(matches) != 1:
    raise SystemExit(f"expected one typed value verification route, got {len(matches)}")
route = matches[0]
if route.get("reason") is not None:
    raise SystemExit(f"typed value verification route was not direct: {route}")
if route.get("proof") != "typed_global_call_same_module_object_handle":
    raise SystemExit(f"typed value verification route proof drift: {route}")
if route.get("definition_owner") != "uniform_mir":
    raise SystemExit(f"typed value verification route should use uniform_mir definition: {route}")
if route.get("target_result_box_name") != "TypedValueVerificationResultBox":
    raise SystemExit(f"typed value verification result box drift: {route}")
if route.get("value_demand") != "runtime_i64_or_handle":
    raise SystemExit(f"typed value verification value demand drift: {route}")
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
if "TypedValueVerificationApi.verify/4" not in symbols:
    raise SystemExit("missing typed value verification same-module definition")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mirbuilder_typed_value_verification.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mirbuilder_typed_value_verification.err
sed '/^Result: /d' "$RAW" >"$OUT"

printf '%s\n' "mirbuilder_typed_value_verification_derived_hako=ok" >"$EXPECTED"
diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-typed-value-verification-derived-artifact-v0
family_id=hakorune_mir_builder::typed_value_verification
typed_value_verification_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
typed_value_verification=1
current_function_take=0
type_propagation=0
full_finalize_module=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
