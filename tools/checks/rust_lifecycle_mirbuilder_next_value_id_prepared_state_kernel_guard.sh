#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="mirbuilder-next-value-id-prepared-state-kernel"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_next_value_id_prepared_state_kernel.hako"
EXE="/tmp/hako_mirbuilder_next_value_id_prepared_state_kernel"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"
MIR_JSON="$EXE.mir.json"

python3 "$GENERATOR" --family "$FAMILY" --check

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED" "$MIR_JSON"

./target/release/hakorune --emit-mir-json "$MIR_JSON" "$ARTIFACT" >/tmp/hako_mirbuilder_next_value_id_prepared_state_kernel.mir.log 2>&1
python3 - <<'PY'
import json
from pathlib import Path

mir = json.loads(Path("/tmp/hako_mirbuilder_next_value_id_prepared_state_kernel.mir.json").read_text())
main = next((fn for fn in mir.get("functions", []) if fn.get("name") == "main"), None)
if main is None:
    raise SystemExit("missing main function in prepared-state kernel MIR")

metadata = main.get("metadata") or {}
routes = metadata.get("global_call_routes", [])
policy_routes = [
    route for route in routes
    if route.get("callee_name") == "MirBuilderAllocationPolicyApi.next_value_id/4"
]
if not policy_routes:
    raise SystemExit("missing MirBuilderAllocationPolicyApi.next_value_id route")
for route in policy_routes:
    expected = {
        "reason": None,
        "tier": "DirectAbi",
        "emit_kind": "direct_function_call",
        "return_shape": "ScalarI64",
        "value_demand": "scalar_i64",
        "definition_owner": "generic_i64_or_leaf",
        "proof": "typed_global_call_generic_i64",
    }
    for key, value in expected.items():
        if route.get(key) != value:
            raise SystemExit(f"policy route expected {key}={value}, got {route.get(key)}")

definitions = metadata.get("same_module_function_definitions") or []
definition_symbols = {row.get("target_symbol") for row in definitions}
required = {
    "MirBuilderAllocationPolicyApi.next_value_id/4",
    "FunctionValueIdCounterStateApi.next/1",
    "CoreContextApi.next_value/1",
    "ReservedValueIdMembershipViewApi.has/2",
}
missing = required - definition_symbols
if missing:
    raise SystemExit(f"missing same-module definitions: {sorted(missing)}")
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_mirbuilder_next_value_id_prepared_state_kernel.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_mirbuilder_next_value_id_prepared_state_kernel.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
mirbuilder_next_value_id_prepared_state_kernel=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-next-value-id-prepared-state-kernel-v0
family_id=hakorune_mir_builder::next_value_id_prepared_state_kernel
prepared_state_policy_kernel=green
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
policy_route=generic_i64_or_leaf_direct
generated_hako_exe_aot=green
full_mirbuilder_object_method=0
scope_context_conversion=0
compilation_context_conversion=0
runtime_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
