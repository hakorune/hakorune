#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-variable-context-route-matrix-closeout-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

ROUTE_MANIFEST="$ROOT_DIR/lang/generated/rust_derived/hakorune_mir_builder/family_routes.json"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-route-matrix-closeout-v0.json"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$ROUTE_MANIFEST" "$FIXTURE" "$0"

python3 - <<'PY'
import json
from pathlib import Path

route_manifest_path = Path("lang/generated/rust_derived/hakorune_mir_builder/family_routes.json")
fixture_path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-route-matrix-closeout-v0.json")

routes = json.loads(route_manifest_path.read_text())
fixture = json.loads(fixture_path.read_text())

assert routes["kind"] == "RustDerivedHakoFamilyRouteManifest"
assert routes["crate"] == "hakorune_mir_builder"
assert routes["schema_version"] == 0

claims = routes["claims"]
assert claims["source_selfhost_claim"] == 0
assert claims["backend_behavior_changed"] == 0
assert claims["runtime_try_hako_then_rust_fallback"] == 0
assert claims["mirbuilder_wide_claim"] == 0
assert claims["variable_context_selected"] == 0
assert claims["variable_context_simple_map_selected"] == 1
assert claims["variable_context_immutable_borrow_selected"] == 0
assert claims["variable_context_snapshot_restore_selected"] == 1
assert claims["variable_context_carrier_snapshot_selected"] == 1
assert claims["variable_context_explicit_carrier_snapshot_selected"] == 1
assert claims["full_variable_context_claim"] == 0

rows = [
    row for row in routes["routes"]
    if row["family_id"] == "hakorune_mir_builder::variable_context"
]
assert len(rows) == 5

selected = [row for row in rows if row["selected_on_mainline"] is True]
denied = [row for row in rows if row["state"] == "Denied"]

assert len(selected) == 4
assert len(denied) == 1

selected_scopes = sorted(row["pilot_scope"] for row in selected)
assert selected_scopes == sorted([
    "VariableContext_simple_map_only",
    "VariableContext_snapshot_restore_only",
    "VariableContext_carrier_snapshot_only",
    "VariableContext_explicit_carrier_snapshot_only",
])

denied_row = denied[0]
assert denied_row["pilot_scope"] == "VariableContext_immutable_borrow_only"
assert denied_row["route"] == "denied"
assert denied_row["selected_on_mainline"] is False
assert denied_row["deny_reason"] == "ReturnedReadBorrow"
assert denied_row["replacement_policy"] == "OwnedReadSnapshotProjection"
assert denied_row["fallback_policy"] == "forbidden"

expected = {
    "candidate_pool_state": "Parked",
    "denied_routes": ["VariableContext_immutable_borrow_only"],
    "family_id": "hakorune_mir_builder::variable_context",
    "family_state": "Parked",
    "kind": "VariableContextRouteMatrixCloseoutV1",
    "manual_HakoAdopted_candidate_selection": 0,
    "next_action": "MIRBUILDER-NEXT-HAKO-ADOPTION-CANDIDATE-SELECTION-001",
    "new_backend_route": 0,
    "new_abi": 0,
    "output_contract": "rust-lifecycle-variable-context-route-matrix-closeout-v0",
    "parked_reason": "ReturnedReadBorrow",
    "replacement_policy": "OwnedReadSnapshotProjection",
    "route_manifest": "lang/generated/rust_derived/hakorune_mir_builder/family_routes.json",
    "runtime_fallback": 0,
    "selected_mainline_routes": [
        "VariableContext_simple_map_only",
        "VariableContext_snapshot_restore_only",
        "VariableContext_carrier_snapshot_only",
        "VariableContext_explicit_carrier_snapshot_only",
    ],
    "selected_on_mainline_count": 4,
    "denied_route_count": 1,
    "source_selfhost_claim": 0,
}

for key, value in expected.items():
    assert fixture[key] == value, f"{key}: expected {value!r}, got {fixture[key]!r}"
PY

cat <<'REPORT'
output_contract=rust-lifecycle-variable-context-route-matrix-closeout-v0
family_id=hakorune_mir_builder::variable_context
family_state=Parked
parked_reason=ReturnedReadBorrow
replacement_policy=OwnedReadSnapshotProjection
selected_mainline_routes=4
denied_routes=1
candidate_pool_state=Parked
next_action=MIRBUILDER-NEXT-HAKO-ADOPTION-CANDIDATE-SELECTION-001
manual_HakoAdopted_candidate_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
