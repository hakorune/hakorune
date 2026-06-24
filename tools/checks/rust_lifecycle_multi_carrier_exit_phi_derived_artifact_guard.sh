#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py"
FAMILY="multi-carrier-exit-phi"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/multi_carrier_exit_phi.hako"
EXE="/tmp/hako_multi_carrier_exit_phi_artifact"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"

python3 "$GENERATOR" --family "$FAMILY" --check

python3 - <<'PY'
import json
from pathlib import Path

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/multi_carrier_exit_phi.artifact.json").read_text())
hako = Path("lang/generated/rust_derived/hakorune_mir_builder/multi_carrier_exit_phi.hako").read_text()

assert manifest["kind"] == "RustDerivedHakoArtifact"
assert manifest["family_id"] == "hakorune_mir_builder::multi_carrier_exit_phi"
assert manifest["pilot_scope"] == "MultiCarrierExitPhi_only"
assert manifest["claims"]["generated_hako_manual_edit"] == 0
assert manifest["claims"]["mainline_selected"] == 0
assert manifest["claims"]["runtime_fallback"] == 0
assert manifest["claims"]["inferred_phi_claim"] == 0
assert "if exit_kind == 0" in hako
assert "else if exit_kind == 1" in hako
assert "else if exit_kind == 2" in hako
assert "carriers.push(0)" in hako
assert "local exit_99 = MultiCarrierExitPhiPilotApi.project_exit_carriers(99)" in hako
assert "multi_exit_phi_unknown_exit=fail" not in hako
assert "return 7" not in hako
assert "TODO" not in hako
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED"

./target/release/hakorune --emit-mir-json /tmp/hako_multi_carrier_exit_phi_artifact.mir.json "$ARTIFACT" >/tmp/hako_multi_carrier_exit_phi_artifact.mir.log 2>&1

python3 - <<'PY'
import json
from pathlib import Path

data = json.loads(Path("/tmp/hako_multi_carrier_exit_phi_artifact.mir.json").read_text())
main = next(fn for fn in data["functions"] if fn.get("name") == "main")
metadata = main.get("metadata", {})
target = "MultiCarrierExitPhiPilotApi.project_exit_carriers/1"

routes = [
    row
    for row in metadata.get("global_call_routes", [])
    if row.get("target_symbol") == target
]
assert len(routes) == 4, f"expected four multi-exit call routes, got {len(routes)}"
for row in routes:
    assert row.get("reason") is None
    assert row.get("tier") == "DirectAbi"
    assert row.get("emit_kind") == "direct_function_call"
    assert row.get("proof") == "typed_global_call_same_module_object_handle"
    assert row.get("return_shape") == "object_handle"
    assert row.get("value_demand") == "runtime_i64_or_handle"
    assert row.get("target_result_box_name") == "ArrayBox"
    assert row.get("definition_owner") == "uniform_mir"
    assert row.get("target_exists") is True
    assert row.get("arity_matches") is True

definitions = [
    row
    for row in metadata.get("same_module_function_definitions", [])
    if row.get("target_symbol") == target
]
assert len(definitions) == 1, f"expected one same-module definition, got {len(definitions)}"
definition = definitions[0]
assert definition.get("definition_kind") == "same_module_function"
assert definition.get("definition_owner") == "uniform_mir"
PY

./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_multi_carrier_exit_phi_artifact.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_multi_carrier_exit_phi_artifact.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
multi_carrier_exit_phi_direct_artifact=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-multi-carrier-exit-phi-derived-artifact-v0
family_id=hakorune_mir_builder::multi_carrier_exit_phi
pilot_scope=MultiCarrierExitPhi_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
raw_hako_body=0
exit_kinds=break,continue,early_return
default_exit=0,0
carrier_count=2
same_module_arraybox_return_contract=green
same_module_definition_plan=green
runtime_try_hako_then_rust_fallback=0
summary=ok
REPORT
