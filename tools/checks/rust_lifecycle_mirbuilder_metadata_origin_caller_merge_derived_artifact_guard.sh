#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-metadata-origin-caller-merge"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_metadata_origin_caller_merge.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_metadata_origin_caller_merge.artifact.json"
VERIFIER="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-metadata-origin-caller-merge-derived-hako-verifier-result-v0.json"
EXE="/tmp/hako_mirbuilder_metadata_origin_caller_merge"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"
MIR_JSON="$EXE.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

hako = Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_metadata_origin_caller_merge.hako").read_text()
required = [
    "using apps.lib.collections.value_id_ordered_map as ValueIdOrderedMap",
    "box MetadataOriginCallerFunctionShellBox",
    "value_origin_callers: ValueIdOrderedMapBox",
    "box MetadataOriginCallerContextShellBox",
    "box MetadataOriginCallerMergeResultBox",
    "MetadataOriginCallerMergeApi",
    "merge(fn_state, metadata_ctx): MetadataOriginCallerMergeResultBox",
    "local merged = ValueIdOrderedMap.create()",
    "fn_state.value_origin_callers.key_at",
    "metadata_ctx.value_origin_callers.key_at",
    "merged.set(key, value)",
    "fn_state.value_origin_callers = merged",
    "result.merged_entries = merged.length()",
    "result.source_wins = 1",
    "result.phi_return_type_inference = 0",
    "mirbuilder_metadata_origin_caller_merge_derived_hako=ok",
]
missing = [needle for needle in required if needle not in hako]
if missing:
    raise SystemExit(f"missing metadata origin-caller merge artifact text: {missing}")
for forbidden in [
    "phi_return_type_inference::infer_return_type_from_phi",
    "phi_input_materialization",
    "module_function_insertion",
    "full_finalize_module",
    "runtime_fallback",
]:
    if forbidden in hako:
        raise SystemExit(f"metadata origin-caller artifact opened non-selected behavior: {forbidden}")

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_metadata_origin_caller_merge.artifact.json").read_text())
if manifest.get("family_id") != "hakorune_mir_builder::metadata_origin_caller_merge":
    raise SystemExit("metadata origin-caller manifest family drift")
if manifest.get("state") != "DerivedShadow":
    raise SystemExit("metadata origin-caller artifact must remain DerivedShadow")
claims = manifest.get("claims") or {}
expected_claims = {
    "metadata_origin_caller_merge": 1,
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
        raise SystemExit(f"metadata origin-caller claim drift: {key}={claims.get(key)}")

verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-metadata-origin-caller-merge-derived-hako-verifier-result-v0.json").read_text())
checks = verifier.get("checks") or {}
expected_checks = {
    "metadata_origin_caller_merge_only": 1,
    "entrypoint": "function.metadata.value_origin_callers = origin_callers",
    "collision_policy": "SourceWins",
    "function_transport": "MirFunctionPreparedMain",
    "source": "self.metadata_ctx.value_origin_callers()",
    "target": "function.metadata.value_origin_callers",
    "minimal_path_expected_result": "OkImplicitUnit",
    "phi_return_type_inference": 0,
    "full_finalize_module": 0,
    "runtime_fallback": 0,
}
for key, expected in expected_checks.items():
    if checks.get(key) != expected:
        raise SystemExit(f"metadata origin-caller verifier check drift: {key}={checks.get(key)}")
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_metadata_origin_caller_merge.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_metadata_origin_caller_merge.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in metadata origin-caller MIR")
metadata = main.get("metadata") or {}
routes = metadata.get("global_call_routes") or []
matches = [route for route in routes if route.get("callee_name") == "MetadataOriginCallerMergeApi.merge/2"]
if len(matches) != 1:
    raise SystemExit(f"expected one metadata origin-caller route, got {len(matches)}")
route = matches[0]
if route.get("reason") is not None:
    raise SystemExit(f"metadata origin-caller route was not direct: {route}")
if route.get("proof") != "typed_global_call_same_module_object_handle":
    raise SystemExit(f"metadata origin-caller route proof drift: {route}")
if route.get("definition_owner") != "uniform_mir":
    raise SystemExit(f"metadata origin-caller route should use uniform_mir definition: {route}")
if route.get("target_result_box_name") != "MetadataOriginCallerMergeResultBox":
    raise SystemExit(f"metadata origin-caller result box drift: {route}")
if route.get("value_demand") != "runtime_i64_or_handle":
    raise SystemExit(f"metadata origin-caller value demand drift: {route}")
definitions = metadata.get("same_module_function_definitions") or []
symbols = {row.get("target_symbol") for row in definitions}
if "MetadataOriginCallerMergeApi.merge/2" not in symbols:
    raise SystemExit("missing metadata origin-caller same-module definition")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mirbuilder_metadata_origin_caller_merge.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mirbuilder_metadata_origin_caller_merge.err
sed '/^Result: /d' "$RAW" >"$OUT"

printf '%s\n' "mirbuilder_metadata_origin_caller_merge_derived_hako=ok" >"$EXPECTED"
diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-metadata-origin-caller-merge-derived-artifact-v0
family_id=hakorune_mir_builder::metadata_origin_caller_merge
metadata_origin_caller_merge_artifact=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
state=DerivedShadow
mainline_selected=0
metadata_origin_caller_merge=1
phi_return_type_inference=0
full_finalize_module=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
