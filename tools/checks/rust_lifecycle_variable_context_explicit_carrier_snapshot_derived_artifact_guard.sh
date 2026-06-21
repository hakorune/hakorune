#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/generate_variable_context_explicit_carrier_snapshot_artifact.py"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/variable_context_explicit_carrier_snapshot.hako"
EXE="/tmp/hako_variable_context_explicit_carrier_snapshot_artifact"
RAW="$EXE.out.raw"
OUT="$EXE.out"
EXPECTED="$EXE.expected"

python3 "$GENERATOR" --check
bash tools/checks/rust_lifecycle_variable_context_explicit_carrier_snapshot_guard.sh

python3 - <<'PY'
import json
from pathlib import Path

import sys

sys.path.insert(0, "tools/rust_lifecycle")
from mirbuilder_carrier_snapshot_artifacts import _api_methods_from_compiled, explicit_carrier_snapshot_spec
from mirbuilder_carrier_snapshot_converter import compile_explicit_carrier_snapshot_methods

base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")
facts = json.loads((base / "variable-context-explicit-carrier-snapshot-facts-v0.json").read_text())
plan = json.loads((base / "variable-context-explicit-carrier-snapshot-plan-v0.json").read_text())
manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/variable_context_explicit_carrier_snapshot.artifact.json").read_text())
recipe = json.loads((base / "variable-context-explicit-carrier-snapshot-behavior-recipe-v0.json").read_text())
verifier = json.loads((base / "variable-context-explicit-carrier-snapshot-derived-artifact-verifier-result-v0.json").read_text())
hako = Path("lang/generated/rust_derived/hakorune_mir_builder/variable_context_explicit_carrier_snapshot.hako").read_text()

assert manifest["kind"] == "RustDerivedHakoArtifact"
assert manifest["family_id"] == "hakorune_mir_builder::variable_context"
assert manifest["pilot_scope"] == "VariableContext_explicit_carrier_snapshot_only"
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
    "CarrierInfo::from_variable_map",
    "join_id lifecycle",
    "promoted_body_locals lifecycle",
    "trim_helper lifecycle",
    "PHI planner integration",
]:
    assert method in set(manifest["excluded_methods"])

inputs = manifest["inputs"]
assert inputs["facts"]["path"].endswith("variable-context-explicit-carrier-snapshot-facts-v0.json")
assert inputs["plan"]["path"].endswith("variable-context-explicit-carrier-snapshot-plan-v0.json")
assert inputs["oracle"]["path"].endswith("variable-context-explicit-carrier-snapshot-oracle-vectors-v0.json")
assert inputs["recipe"]["path"].endswith("variable-context-explicit-carrier-snapshot-behavior-recipe-v0.json")
assert inputs["verifier"]["path"].endswith("variable-context-explicit-carrier-snapshot-derived-artifact-verifier-result-v0.json")

output = manifest["output"]
assert output["hako_path"].endswith("variable_context_explicit_carrier_snapshot.hako")

assert recipe["kind"] == "HakoBehaviorRecipe"
assert recipe["pilot_scope"] == "VariableContext_explicit_carrier_snapshot_only"
assert recipe["selected_body_count"] == "explicit_carrier_snapshot_methods_only"
assert recipe["methods"][0]["id"] == "CarrierInfo::with_explicit_carriers"
assert recipe["methods"][0]["rust_operation"] == "ExplicitCarrierSnapshotFromOwnedMap"
assert "CarrierInfo::from_variable_map" in set(recipe["excluded_methods"])
assert "VariableContext::variable_map" in set(recipe["excluded_methods"])

assert verifier["kind"] == "DerivedHakoArtifactVerifierResult"
assert verifier["result"] == "VerifiedHakoFamilyIR"
checks = verifier["checks"]
assert checks["selected_body_count"] == "explicit_carrier_snapshot_methods_only"
assert checks["carrier_behavior_generated"] == 1
assert checks["requested_names_owned"] == 1
assert checks["missing_carrier_fail_fast"] == 1
assert checks["full_variable_context_claim"] == 0
assert checks["rust_bootstrap_retained"] == 1
assert checks["backend_behavior_changed"] == 0
assert "ExplicitCarrierSnapshotFromOwnedMap" in verifier["verified_operations"]
assert "CloneOwnedMap" in verifier["verified_operations"]
assert "OrderedMapBox.key_at" in verifier["verified_operations"]
assert "ArrayBox.get" in verifier["verified_operations"]

assert "static box CarrierInfoApi" in hako
assert "with_explicit_carriers_from_snapshot(carrier_data: OrderedMapBox, loop_var_name, loop_var_id, requested_names, snapshot: OrderedMapBox): i64" in hako
assert 'CarrierInfoApi.with_explicit_carriers_from_snapshot(info, "i", 5, requested_names, snapshot)' in hako
assert 'local requested_name_copy = info.get("requested_names")' in hako
assert 'local carrier_names = info.get("carrier_names")' in hako
assert 'local carrier_host_ids = info.get("carrier_host_ids")' in hako
assert 'requested_name_copy.get(0)' in hako
assert 'carrier_names.get(0)' in hako
assert 'carrier_host_ids.get(0)' in hako
assert "VariableContextApi.snapshot" in hako
assert "VariableContextApi.variable_map" not in hako
assert "return ctx.variable_map\n" not in hako
assert "CarrierInfo::from_variable_map" not in hako
assert "variable_map_mut" not in hako
assert "carrier_names_init" not in hako
assert "missing_scan_index" not in hako
assert "explicit_carrier_snapshot_output_arg_mutation=fail" in hako
assert "explicit_carrier_snapshot_ctx_alias=fail" in hako

spec = explicit_carrier_snapshot_spec(_api_methods_from_compiled(compile_explicit_carrier_snapshot_methods(facts, plan)))
assert all(
    method.operations
    for static_box in spec.static_boxes
    for method in static_box.methods
)
PY

rm -f "$EXE" "$RAW" "$OUT" "$EXPECTED"
./target/release/hakorune --emit-mir-json /tmp/hako_variable_context_explicit_carrier_snapshot_artifact.mir.json "$ARTIFACT" >/tmp/hako_variable_context_explicit_carrier_snapshot_artifact.mir.log 2>&1
./target/release/hakorune --emit-exe "$EXE" "$ARTIFACT" >/tmp/hako_variable_context_explicit_carrier_snapshot_artifact.build.log 2>&1
"$EXE" >"$RAW" 2>/tmp/hako_variable_context_explicit_carrier_snapshot_artifact.err
sed '/^Result: /d' "$RAW" >"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
variable_context_explicit_carrier_snapshot_derived_artifact=ok
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-lifecycle-variable-context-explicit-carrier-snapshot-derived-artifact-v0
family_id=hakorune_mir_builder::variable_context
pilot_scope=VariableContext_explicit_carrier_snapshot_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe=green
typed_output_arg_mutation=green
main_inlined_duplicate_carrier_projection=0
orderedmap_get_result_type_origin=green
runtime_data_get_for_carrier_arrays=0
route_selected=0
full_variable_context_claim=0
variable_map_mut_generated=0
returned_read_borrow_deny=green
owned_snapshot_alias_isolation=green
missing_carrier_fail_fast=green
carrier_behavior_generated=1
rust_bootstrap_retained=1
runtime_try_hako_then_rust_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
