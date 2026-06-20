#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/generate_variable_context_explicit_carrier_snapshot_artifact.py"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/variable_context_explicit_carrier_snapshot.hako"

python3 "$GENERATOR" --check
bash tools/checks/rust_lifecycle_variable_context_explicit_carrier_snapshot_guard.sh

python3 - <<'PY'
import json
from pathlib import Path

base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")
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
    "VariableContext::snapshot",
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
assert "CarrierInfo::from_variable_map" in set(recipe["excluded_methods"])

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
assert "ExplicitCarrierSnapshotFromBorrowView" in verifier["verified_operations"]
assert "OrderedMapBox.keys" in verifier["verified_operations"]
assert "ArrayBox.get" in verifier["verified_operations"]

assert "CarrierInfoApi.with_explicit_carriers" in hako
assert "VariableContextApi.variable_map" in hako
assert "CarrierInfo::from_variable_map" not in hako
assert "variable_map_mut" not in hako
assert "carrier_snapshot_missing_requested_carrier=fail" in hako
PY

./target/release/hakorune --emit-mir-json /tmp/hako_variable_context_explicit_carrier_snapshot_artifact.mir.json "$ARTIFACT" >/tmp/hako_variable_context_explicit_carrier_snapshot_artifact.mir.log 2>&1

cat <<'REPORT'
output_contract=rust-lifecycle-variable-context-explicit-carrier-snapshot-derived-artifact-v0
family_id=hakorune_mir_builder::variable_context
pilot_scope=VariableContext_explicit_carrier_snapshot_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
route_selected=0
full_variable_context_claim=0
variable_map_mut_generated=0
carrier_behavior_generated=1
rust_bootstrap_retained=1
runtime_try_hako_then_rust_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
