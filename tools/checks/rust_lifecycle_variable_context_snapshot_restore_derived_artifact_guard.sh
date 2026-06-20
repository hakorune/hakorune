#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/generate_variable_context_snapshot_restore_artifact.py"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/variable_context_snapshot_restore.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/variable_context_snapshot_restore.artifact.json"

python3 "$GENERATOR" --check
bash tools/checks/rust_lifecycle_variable_context_snapshot_restore_guard.sh

python3 - <<'PY'
import json
from pathlib import Path

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/variable_context_snapshot_restore.artifact.json").read_text())
hako = Path("lang/generated/rust_derived/hakorune_mir_builder/variable_context_snapshot_restore.hako").read_text()

assert manifest["kind"] == "RustDerivedHakoArtifact"
assert manifest["family_id"] == "hakorune_mir_builder::variable_context"
assert manifest["pilot_scope"] == "VariableContext_snapshot_restore_only"
assert manifest["state"] == "DerivedShadow"
assert manifest["claims"]["generated_hako_manual_edit"] == 0
assert manifest["claims"]["mainline_selected"] == 0
assert manifest["claims"]["full_variable_context_claim"] == 0
assert manifest["claims"]["rust_bootstrap_retained"] == 1
assert manifest["claims"]["backend_behavior_changed"] == 0
assert manifest["claims"]["source_selfhost_claim"] == 0
assert manifest["excluded_methods"] == [
    "VariableContext::variable_map",
    "VariableContext::variable_map_mut",
    "VariableContext::lookup",
    "VariableContext::require",
    "VariableContext::insert",
    "VariableContext::remove",
    "VariableContext::contains",
    "VariableContext::len",
    "VariableContext::is_empty",
    "CarrierInfo::from_variable_map",
    "CarrierInfo::with_explicit_carriers",
    "PHI planner integration",
]

inputs = manifest["inputs"]
assert inputs["facts"]["path"].endswith("variable-context-snapshot-restore-facts-v0.json")
assert inputs["plan"]["path"].endswith("variable-context-snapshot-restore-plan-v0.json")
assert inputs["oracle"]["path"].endswith("variable-context-snapshot-restore-oracle-vectors-v0.json")

output = manifest["output"]
assert output["hako_path"].endswith("variable_context_snapshot_restore.hako")

assert "VariableContextApi.snapshot" in hako
assert "VariableContextApi.restore" in hako
assert "variable_map_mut" not in hako
assert "CarrierInfo" not in hako
PY

./target/release/hakorune --emit-mir-json "$ARTIFACT" >/tmp/hako_variable_context_snapshot_restore_artifact.mir.log 2>&1

cat <<'REPORT'
output_contract=rust-lifecycle-variable-context-snapshot-restore-derived-artifact-v0
family_id=hakorune_mir_builder::variable_context
pilot_scope=VariableContext_snapshot_restore_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
route_selected=0
full_variable_context_claim=0
variable_map_mut_generated=0
carrier_behavior_generated=0
rust_bootstrap_retained=1
runtime_try_hako_then_rust_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
