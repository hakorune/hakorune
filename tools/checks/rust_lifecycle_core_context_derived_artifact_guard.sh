#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="core-context"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/core_context.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/core_context.artifact.json"
EXE="/tmp/hako_core_context_artifact"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"

python3 "$GENERATOR" --family "$FAMILY" --check
python3 tools/rust_lifecycle/verify_core_context_artifact_contract.py --drift-probes

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED"

./target/release/hakorune --emit-mir-json /tmp/hako_core_context_artifact.mir.json "$ARTIFACT" >/tmp/hako_core_context_artifact.mir.log 2>&1
python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_core_context_artifact.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in core_context MIR")

metadata = main.get("metadata") or {}
routes = [
    route
    for route in metadata.get("global_call_routes", [])
    if str(route.get("callee_name", "")).startswith("CoreContextApi.")
]
if not routes:
    raise SystemExit("missing CoreContextApi global-call routes")

required = {
    "CoreContextApi.next_binding/1",
    "CoreContextApi.next_temp_slot/1",
    "CoreContextApi.next_debug_join/1",
    "CoreContextApi.next_value/1",
    "CoreContextApi.peek_next_value/1",
    "CoreContextApi.next_block/1",
    "CoreContextApi.peek_next_block/1",
}
seen = {route.get("callee_name") for route in routes}
missing = required - seen
if missing:
    raise SystemExit(f"missing CoreContextApi routes: {sorted(missing)}")

for route in routes:
    callee = route.get("callee_name")
    if route.get("reason") is not None:
        raise SystemExit(f"{callee}: expected direct route, got reason={route.get('reason')}")
    if callee in {"CoreContextApi.peek_next_value/1", "CoreContextApi.peek_next_block/1"}:
        expected = {
            "tier": "DirectAbi",
            "emit_kind": "direct_function_call",
            "return_shape": "ScalarI64",
            "value_demand": "scalar_i64",
            "definition_owner": "generic_i64_or_leaf",
            "proof": "typed_global_call_generic_i64",
        }
    else:
        expected = {
            "tier": "DirectAbi",
            "emit_kind": "direct_function_call",
            "return_shape": "ScalarI64",
            "value_demand": "scalar_i64",
            "definition_owner": "uniform_mir",
            "proof": "typed_global_call_same_module_scalar_i64",
        }
    for key, value in expected.items():
        if route.get(key) != value:
            raise SystemExit(f"{callee}: expected {key}={value}, got {route.get(key)}")

definitions = (
    metadata.get("same_module_definition_plans")
    or metadata.get("same_module_function_definitions")
    or []
)
definition_symbols = {row.get("target_symbol") for row in definitions}
missing_definitions = required - definition_symbols
if missing_definitions:
    raise SystemExit(f"missing same-module definitions: {sorted(missing_definitions)}")

for row in definitions:
    if row.get("target_symbol") in required:
        if row.get("definition_kind") != "same_module_function":
            raise SystemExit(f"{row.get('target_symbol')}: unexpected definition_kind={row.get('definition_kind')}")
        target_symbol = row.get("target_symbol")
        expected_owner = (
            "generic_i64_or_leaf"
            if target_symbol in {"CoreContextApi.peek_next_value/1", "CoreContextApi.peek_next_block/1"}
            else "uniform_mir"
        )
        if row.get("definition_owner") != expected_owner:
            raise SystemExit(f"{target_symbol}: unexpected definition_owner={row.get('definition_owner')}")
PY
./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_core_context_artifact.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_core_context_artifact.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
core_context_scalar_counters_and_id_generators_derived_artifact=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-core-context-derived-artifact-v0
family_id=hakorune_mir_builder::core_context
pilot_scope=CoreContext_scalar_counters_and_id_generators
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
same_module_scalar_counter_routes=green
same_module_scalar_counter_definitions=green
newtype_id_generator_routes=green
newtype_id_generator_definitions=green
generated_hako_exe_aot=green
core_context_full_claim=0
mirbuilder_wide_claim=0
generator_object_transport=0
value_id_transport=ValueIdAsI64
basic_block_id_transport=BasicBlockIdAsI64
runtime_try_hako_then_rust_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
