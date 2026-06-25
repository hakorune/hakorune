#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-metadata-value-type-publication"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_metadata_value_type_publication.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_metadata_value_type_publication.artifact.json"
VERIFIER="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-metadata-value-type-publication-derived-hako-verifier-result-v0.json"
EXE="/tmp/hako_mirbuilder_metadata_value_type_publication"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"
MIR_JSON="$EXE.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

hako = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_metadata_value_type_publication.hako").read_text()
required = [
    "box MetadataValueTypeFunctionShellBox",
    "metadata_value_types_entries: i64",
    "metadata_value_types_mutated: i64",
    "metadata_origin_callers_mutated: i64",
    "box MetadataValueTypeContextShellBox",
    "value_types_source_entries: i64",
    "box MetadataValueTypePublicationResultBox",
    "MetadataValueTypePublicationApi",
    "publish(fn_state, type_ctx): MetadataValueTypePublicationResultBox",
    "fn_state.metadata_value_types_entries = type_ctx.value_types_source_entries",
    "fn_state.metadata_value_types_mutated = 1",
    "fn_state.metadata_origin_callers_mutated = 0",
    "result.clone_owned = 1",
    "result.origin_caller_merge = 0",
    "mirbuilder_metadata_value_type_publication_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing metadata value-type publication artifact text: {missing}")
for forbidden in [
    "metadata_origin_caller_merge",
    "phi_return_type_inference",
    "phi_input_materialization",
    "module_function_insertion",
    "full_finalize_module",
    "runtime_fallback",
]:
    if forbidden in hako:
        raise SystemExit(f"metadata value-type artifact opened non-selected behavior: {forbidden}")

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_metadata_value_type_publication.artifact.json").read_text())
if manifest.get("family_id") != "hakorune_mir_builder::metadata_value_type_publication":
    raise SystemExit("metadata value-type manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("metadata value-type artifact must remain DerivedShadow")
claims = manifest.get("claims") or {}
expected_claims = {
    "metadata_value_type_publication": 1,
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
        raise SystemExit(f"metadata value-type claim drift: {key}={claims.get(key)}")

verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-metadata-value-type-publication-derived-hako-verifier-result-v0.json").read_text())
checks = verifier.get("checks") or {}
expected_checks = {
    "metadata_value_type_publication_only": 1,
    "entrypoint": "function.metadata.value_types = self.type_ctx.value_types.clone()",
    "publication_operation": "CloneOwnedMap",
    "function_transport": "MirFunctionPreparedMain",
    "value_types_source": "self.type_ctx.value_types",
    "metadata_target": "function.metadata.value_types",
    "minimal_path_expected_result": "OkImplicitUnit",
    "metadata_origin_caller_merge": 0,
    "full_finalize_module": 0,
    "runtime_fallback": 0,
}
for key, expected in expected_checks.items():
    if checks.get(key) != expected:
        raise SystemExit(f"metadata value-type verifier check drift: {key}={checks.get(key)}")
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_metadata_value_type_publication.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_metadata_value_type_publication.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in metadata value-type MIR")
metadata = main.get("metadata") or {}
routes = metadata.get("global_call_routes") or []
matches = [route for route in routes if route.get("callee_name") == "MetadataValueTypePublicationApi.publish/2"]
if len(matches) != 1:
    raise SystemExit(f"expected one metadata value-type route, got {len(matches)}")
route = matches[0]
if route.get("reason") is not None:
    raise SystemExit(f"metadata value-type route was not direct: {route}")
if route.get("proof") != "typed_global_call_same_module_object_handle":
    raise SystemExit(f"metadata value-type route proof drift: {route}")
if route.get("definition_owner") != "uniform_mir":
    raise SystemExit(f"metadata value-type route should use uniform_mir definition: {route}")
if route.get("target_result_box_name") != "MetadataValueTypePublicationResultBox":
    raise SystemExit(f"metadata value-type result box drift: {route}")
if route.get("value_demand") != "runtime_i64_or_handle":
    raise SystemExit(f"metadata value-type value demand drift: {route}")
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
if "MetadataValueTypePublicationApi.publish/2" not in symbols:
    raise SystemExit("missing metadata value-type same-module definition")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mirbuilder_metadata_value_type_publication.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mirbuilder_metadata_value_type_publication.err
sed '/^Result: /d' "$RAW" >"$OUT"

printf '%s\n' "mirbuilder_metadata_value_type_publication_derived_hako=ok" >"$EXPECTED"
diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-metadata-value-type-publication-derived-artifact-v0
family_id=hakorune_mir_builder::metadata_value_type_publication
metadata_value_type_publication_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
metadata_value_type_publication=1
metadata_origin_caller_merge=0
full_finalize_module=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
