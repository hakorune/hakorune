#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/generate_variable_context_carrier_snapshot_artifact.py"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/variable_context_carrier_snapshot.hako"
EXE="/tmp/hako_variable_context_carrier_snapshot_artifact"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"

python3 "$GENERATOR" --check
bash tools/checks/rust_lifecycle_variable_context_carrier_snapshot_guard.sh

python3 - <<'PY'
import json
import sys
from pathlib import Path

sys.path.insert(0, "tools/rust_lifecycle")
from mirbuilder_carrier_snapshot_artifacts import carrier_snapshot_spec

base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")
manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/variable_context_carrier_snapshot.artifact.json").read_text())
recipe = json.loads((base / "variable-context-carrier-snapshot-behavior-recipe-v0.json").read_text())
verifier = json.loads((base / "variable-context-carrier-snapshot-derived-artifact-verifier-result-v0.json").read_text())
hako = Path("lang/generated/rust_derived/hakorune_mir_builder/variable_context_carrier_snapshot.hako").read_text()

assert manifest["kind"] == "RustDerivedHakoArtifact"
assert manifest["family_id"] == "hakorune_mir_builder::variable_context"
assert manifest["pilot_scope"] == "VariableContext_carrier_snapshot_only"
assert manifest["state"] == "DerivedShadow"
assert manifest["claims"]["generated_hako_manual_edit"] == 0
assert manifest["claims"]["mainline_selected"] == 0
assert manifest["claims"]["full_variable_context_claim"] == 0
assert manifest["claims"]["rust_bootstrap_retained"] == 1
assert manifest["claims"]["backend_behavior_changed"] == 0
assert manifest["claims"]["source_selfhost_claim"] == 0
for method in [
    "VariableContext::variable_map_mut",
    "VariableContext::variable_map",
    "VariableContext::restore",
    "CarrierInfo::with_explicit_carriers",
    "join_id lifecycle",
    "promoted_body_locals lifecycle",
    "trim_helper lifecycle",
    "PHI planner integration",
]:
    assert method in set(manifest["excluded_methods"])

inputs = manifest["inputs"]
assert inputs["facts"]["path"].endswith("variable-context-carrier-snapshot-facts-v0.json")
assert inputs["plan"]["path"].endswith("variable-context-carrier-snapshot-plan-v0.json")
assert inputs["oracle"]["path"].endswith("variable-context-carrier-snapshot-oracle-vectors-v0.json")
assert inputs["recipe"]["path"].endswith("variable-context-carrier-snapshot-behavior-recipe-v0.json")
assert inputs["verifier"]["path"].endswith("variable-context-carrier-snapshot-derived-artifact-verifier-result-v0.json")

output = manifest["output"]
assert output["hako_path"].endswith("variable_context_carrier_snapshot.hako")

assert recipe["kind"] == "HakoBehaviorRecipe"
assert recipe["pilot_scope"] == "VariableContext_carrier_snapshot_only"
assert recipe["selected_body_count"] == "carrier_snapshot_methods_only"
assert recipe["methods"][0]["id"] == "CarrierInfo::from_variable_map"
assert "CarrierInfo::with_explicit_carriers" in set(recipe["excluded_methods"])

assert verifier["kind"] == "DerivedHakoArtifactVerifierResult"
assert verifier["result"] == "VerifiedHakoFamilyIR"
checks = verifier["checks"]
assert checks["selected_body_count"] == "carrier_snapshot_methods_only"
assert checks["carrier_behavior_generated"] == 1
assert checks["full_variable_context_claim"] == 0
assert checks["rust_bootstrap_retained"] == 1
assert checks["backend_behavior_changed"] == 0
assert "CarrierSnapshotFromOwnedMap" in verifier["verified_operations"]
assert "CloneOwnedMap" in verifier["verified_operations"]
assert "OrderedMapBox.key_at" in verifier["verified_operations"]
assert "ArrayBox.push" in verifier["verified_operations"]

assert "static box CarrierInfoApi" in hako
assert "from_snapshot(carrier_data: OrderedMapBox, loop_var_name, snapshot: OrderedMapBox): i64" in hako
assert 'CarrierInfoApi.from_snapshot(info, "i", snapshot)' in hako
assert 'local carrier_names = info.get("carrier_names")' in hako
assert 'local carrier_host_ids = info.get("carrier_host_ids")' in hako
assert 'carrier_names.get(0)' in hako
assert 'carrier_host_ids.get(0)' in hako
assert "VariableContextApi.snapshot" in hako
assert "return ctx.variable_map\n" not in hako
assert "from_variable_map(loop_var_name, variable_map)" not in hako
assert "variable_map_mut" not in hako
assert "CarrierInfo::with_explicit_carriers" not in hako
assert "carrier_names_init" not in hako
assert "init_index" not in hako
assert "carrier_snapshot_output_arg_mutation=fail" in hako

spec = carrier_snapshot_spec()
assert all(
    method.body_lines is None and method.operations
    for static_box in spec.static_boxes
    for method in static_box.methods
)
assert not any(
    method.signature == "variable_map(ctx)"
    for static_box in spec.static_boxes
    for method in static_box.methods
)
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED"
./target/release/hakorune --emit-mir-json /tmp/hako_variable_context_carrier_snapshot_artifact.mir.json "$ARTIFACT" >/tmp/hako_variable_context_carrier_snapshot_artifact.mir.log 2>&1
./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_variable_context_carrier_snapshot_artifact.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_variable_context_carrier_snapshot_artifact.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
variable_context_carrier_snapshot_derived_artifact=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-variable-context-carrier-snapshot-derived-artifact-v0
family_id=hakorune_mir_builder::variable_context
pilot_scope=VariableContext_carrier_snapshot_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe=green
owned_snapshot_alias_isolation=green
typed_output_arg_mutation=green
main_inlined_duplicate_carrier_projection=0
orderedmap_get_result_type_origin=green
runtime_data_get_for_carrier_arrays=0
route_selected=0
full_variable_context_claim=0
variable_map_mut_generated=0
carrier_behavior_generated=1
rust_bootstrap_retained=1
runtime_try_hako_then_rust_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
