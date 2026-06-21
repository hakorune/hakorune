#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

ARTIFACT_GUARD_LOG="/tmp/hako_variable_context_explicit_carrier_snapshot_derived_artifact_guard.out"
SEAM_GUARD_LOG="/tmp/hako_family_artifact_route_seam_guard.out"

bash tools/checks/rust_lifecycle_variable_context_explicit_carrier_snapshot_derived_artifact_guard.sh >"$ARTIFACT_GUARD_LOG" 2>&1
bash tools/checks/selfhost_family_artifact_route_seam_ssot_guard.sh >"$SEAM_GUARD_LOG" 2>&1

python3 - <<'PY'
import json
from pathlib import Path

route_path = Path("lang/generated/rust_derived/hakorune_mir_builder/family_routes.json")
artifact_path = Path("lang/generated/rust_derived/hakorune_mir_builder/variable_context_explicit_carrier_snapshot.artifact.json")

routes = json.loads(route_path.read_text())
artifact = json.loads(artifact_path.read_text())

assert routes["schema_version"] == 0
assert routes["kind"] == "RustDerivedHakoFamilyRouteManifest"
assert routes["crate"] == "hakorune_mir_builder"

claims = routes["claims"]
assert claims["source_selfhost_claim"] == 0
assert claims["backend_behavior_changed"] == 0
assert claims.get("runtime_try_hako_then_rust_fallback", 0) == 0
assert claims["mirbuilder_wide_claim"] == 0
assert claims["variable_context_selected"] == 0
assert claims["variable_context_simple_map_selected"] == 1
assert claims["variable_context_immutable_borrow_selected"] == 0
assert claims["variable_context_snapshot_restore_selected"] == 1
assert claims["variable_context_carrier_snapshot_selected"] == 1
assert claims["variable_context_explicit_carrier_snapshot_selected"] == 1
assert claims["full_variable_context_claim"] == 0

route_entries = [
    route
    for route in routes["routes"]
    if route["artifact_manifest"] == str(artifact_path)
]
assert len(route_entries) == 1
route = route_entries[0]

assert route["family_id"] == "hakorune_mir_builder::variable_context"
assert route["pilot_scope"] == "VariableContext_explicit_carrier_snapshot_only"
assert route["route"] == "derived_hako"
assert route["state"] == "DerivedMainline"
assert route["selected_on_mainline"] is True
assert route["artifact_manifest"] == str(artifact_path)
assert route["guard_command"] == "bash tools/checks/rust_lifecycle_variable_context_explicit_carrier_snapshot_derived_route_selection_guard.sh"
assert route["fallback_policy"] == "forbidden"
assert route["rust_bootstrap_route"] == "retained"
assert route["rust_oracle_route"] == "retained"
assert "not_selected_reason" not in route

assert artifact["kind"] == "RustDerivedHakoArtifact"
assert artifact["family_id"] == route["family_id"]
assert artifact["pilot_scope"] == route["pilot_scope"]
assert artifact["state"] == "DerivedShadow"
assert artifact["claims"]["generated_hako_manual_edit"] == 0
assert artifact["claims"]["mainline_selected"] == 0
assert artifact["claims"]["full_variable_context_claim"] == 0
assert artifact["claims"]["rust_bootstrap_retained"] == 1
assert artifact["claims"]["source_selfhost_claim"] == 0
assert artifact["claims"]["backend_behavior_changed"] == 0

excluded = set(artifact["excluded_methods"])
for method in [
    "VariableContext::variable_map_mut",
    "VariableContext::variable_map",
    "VariableContext::restore",
    "CarrierInfo::from_variable_map",
]:
    assert method in excluded
PY

cat <<'REPORT'
output_contract=rust-lifecycle-variable-context-explicit-carrier-snapshot-derived-route-selection-v0
family_id=hakorune_mir_builder::variable_context
pilot_scope=VariableContext_explicit_carrier_snapshot_only
selected_route=derived_hako
route_state=DerivedMainline
selected_on_mainline=1
route_seam_ssot_verified=1
artifact_manifest_verified=1
full_variable_context_claim=0
runtime_try_hako_then_rust_fallback=0
rust_bootstrap_retained=1
rust_oracle_retained=1
generated_artifact_manual_edit=0
source_selfhost_claim=0
backend_behavior_changed=0
summary=ok
REPORT
