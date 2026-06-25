#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-return-type-publication"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_return_type_publication.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_return_type_publication.artifact.json"
VERIFIER="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-return-type-publication-derived-hako-verifier-result-v0.json"
EXE="/tmp/hako_mirbuilder_return_type_publication"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"
MIR_JSON="$EXE.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

hako = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_return_type_publication.hako").read_text()
required = [
    "box ReturnTypeFunctionSignatureShellBox",
    "return_type_is_integer: i64",
    "published_from_value_id: i64",
    "publication_present: i64",
    "box ReturnTypeValueTypeRecordShellBox",
    "value_id: i64",
    "is_integer: i64",
    "ReturnTypePublicationApi",
    "publish(signature, value_type, result_value): ReturnTypeFunctionSignatureShellBox",
    "signature.return_type_is_integer = 1",
    "signature.published_from_value_id = result_value",
    "signature.publication_present = 1",
    "mirbuilder_return_type_publication_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing return type publication artifact text: {missing}")
for forbidden in [
    "current_module.take",
    "verify_typed_values",
    "full_finalize_module",
    "phi_return_type_inference",
    "runtime_fallback",
]:
    if forbidden in hako:
        raise SystemExit(f"return type publication artifact opened non-selected behavior: {forbidden}")

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_return_type_publication.artifact.json").read_text())
if manifest.get("family_id") != "hakorune_mir_builder::return_type_publication":
    raise SystemExit("return type publication manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("return type publication artifact must remain DerivedShadow")
claims = manifest.get("claims") or {}
expected_claims = {
    "return_type_publication": 1,
    "module_take": 0,
    "verify_typed_values": 0,
    "full_finalize_module": 0,
    "phi_return_type_inference": 0,
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
        raise SystemExit(f"return type publication claim drift: {key}={claims.get(key)}")

verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-return-type-publication-derived-hako-verifier-result-v0.json").read_text())
checks = verifier.get("checks") or {}
expected_checks = {
    "return_type_publication_only": 1,
    "source_value_type_owner": "LiteralIntegerLowering",
    "result_value_transport": "ValueIdAsI64",
    "signature_return_type": "MirType::Integer",
    "module_take": 0,
    "verify_typed_values": 0,
    "full_finalize_module": 0,
    "runtime_fallback": 0,
}
for key, expected in expected_checks.items():
    if checks.get(key) != expected:
        raise SystemExit(f"return type publication verifier check drift: {key}={checks.get(key)}")
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_return_type_publication.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_return_type_publication.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in return type publication MIR")
metadata = main.get("metadata") or {}
routes = metadata.get("global_call_routes") or []
matches = [route for route in routes if route.get("callee_name") == "ReturnTypePublicationApi.publish/3"]
if len(matches) != 1:
    raise SystemExit(f"expected one return type publication route, got {len(matches)}")
route = matches[0]
if route.get("reason") is not None:
    raise SystemExit(f"return type publication route was not direct: {route}")
if route.get("definition_owner") != "uniform_mir":
    raise SystemExit(f"return type publication route should use uniform_mir definition: {route}")
if route.get("target_result_box_name") != "ReturnTypeFunctionSignatureShellBox":
    raise SystemExit(f"return type publication result box drift: {route}")
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
if "ReturnTypePublicationApi.publish/3" not in symbols:
    raise SystemExit("missing return type publication same-module definition")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mirbuilder_return_type_publication.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mirbuilder_return_type_publication.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
mirbuilder_return_type_publication_derived_hako=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-return-type-publication-derived-artifact-v0
family_id=hakorune_mir_builder::return_type_publication
return_type_publication_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
return_type_publication=1
module_take=0
verify_typed_values=0
full_finalize_module=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
