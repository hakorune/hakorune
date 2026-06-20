#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

GENERATOR="tools/rust_lifecycle/generate_variable_context_immutable_borrow_artifact.py"
ARTIFACT="lang/generated/rust_derived/hakorune_mir_builder/variable_context_immutable_borrow.hako"
MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/variable_context_immutable_borrow.artifact.json"

python3 "$GENERATOR" --check

python3 - <<'PY'
import json
from pathlib import Path

manifest = json.loads(Path("lang/generated/rust_derived/hakorune_mir_builder/variable_context_immutable_borrow.artifact.json").read_text())

assert manifest["kind"] == "RustDerivedHakoArtifact"
assert manifest["family_id"] == "hakorune_mir_builder::variable_context"
assert manifest["pilot_scope"] == "VariableContext_immutable_borrow_only"
assert manifest["state"] == "DerivedShadow"
assert manifest["claims"]["generated_hako_manual_edit"] == 0
assert manifest["claims"]["mainline_selected"] == 0
assert manifest["claims"]["full_variable_context_claim"] == 0
assert manifest["claims"]["rust_bootstrap_retained"] == 1
assert manifest["claims"]["backend_behavior_changed"] == 0
assert manifest["claims"]["source_selfhost_claim"] == 0
assert manifest["excluded_methods"] == [
    "VariableContext::variable_map_mut",
    "VariableContext::snapshot",
    "VariableContext::restore",
]

inputs = manifest["inputs"]
assert inputs["facts"]["path"].endswith("variable-context-immutable-borrow-facts-v0.json")
assert inputs["plan"]["path"].endswith("variable-context-immutable-borrow-plan-v0.json")
assert inputs["oracle"]["path"].endswith("variable-context-immutable-borrow-oracle-vectors-v0.json")

output = manifest["output"]
assert output["hako_path"].endswith("variable_context_immutable_borrow.hako")
PY

./target/release/hakorune --emit-mir-json /tmp/hako_variable_context_immutable_borrow_artifact.mir.json "$ARTIFACT" >/tmp/hako_variable_context_immutable_borrow_artifact.mir.log 2>&1

cat <<'REPORT'
output_contract=rust-lifecycle-variable-context-immutable-borrow-derived-artifact-v0
family_id=hakorune_mir_builder::variable_context
pilot_scope=VariableContext_immutable_borrow_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
route_selected=0
full_variable_context_claim=0
variable_map_mut_generated=0
snapshot_restore_generated=0
carrier_behavior_generated=0
rust_bootstrap_retained=1
runtime_try_hako_then_rust_fallback=0
backend_behavior_changed=0
summary=ok
REPORT
