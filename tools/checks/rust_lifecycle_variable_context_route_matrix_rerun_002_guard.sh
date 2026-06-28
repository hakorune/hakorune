#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-296x/1792-MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-002.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-route-matrix-rerun-002-v0.json"
PRIOR_RERUN="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-route-matrix-rerun-v0.json"
SELECTION="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-explicit-mutation-surface-selection-v0.json"
PROJECTION="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-explicit-mutation-api-projection-v0.json"
ROUTE_MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/family_routes.json"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

bash tools/checks/rust_lifecycle_variable_context_explicit_mutation_api_projection_guard.sh >/tmp/hako_variable_context_explicit_mutation_api_projection_guard.out

python3 - "$CARD" "$FIXTURE" "$PRIOR_RERUN" "$SELECTION" "$PROJECTION" "$ROUTE_MANIFEST" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
prior_rerun_path = Path(sys.argv[3])
selection_path = Path(sys.argv[4])
projection_path = Path(sys.argv[5])
route_manifest_path = Path(sys.argv[6])
state_path = Path(sys.argv[7])
task_order_path = Path(sys.argv[8])
index_path = Path(sys.argv[9])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
prior_rerun = json.loads(prior_rerun_path.read_text(encoding="utf-8"))
selection = json.loads(selection_path.read_text(encoding="utf-8"))
projection = json.loads(projection_path.read_text(encoding="utf-8"))
route_manifest = json.loads(route_manifest_path.read_text(encoding="utf-8"))
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-002"
output_contract = "rust-lifecycle-variable-context-route-matrix-rerun-v1"
selected_surface = "VariableContextNativeSurfaceExplicitMutationApiOnlyV1"
next_action = "VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-HAKO-ADOPTION-DECISION-001"

require(f"# {token}" in card, "card token drift")
require(f"output_contract={output_contract}" in card, "card output contract drift")
require(f"selected_surface_id={selected_surface}" in card, "card selected surface drift")
require(f"next_action={next_action}" in card, "card next action drift")

require(fixture.get("kind") == "VariableContextRouteMatrixRerunV2", "fixture kind drift")
require(fixture.get("output_contract") == output_contract, "fixture output contract drift")

fixture_state = fixture.get("current_state") or {}
require(fixture_state.get("latest_card") == token, "fixture latest card drift")
require(fixture_state.get("current_blocker_token") == token, "fixture blocker drift")

require(prior_rerun.get("kind") == "VariableContextRouteMatrixRerunV1", "prior rerun kind drift")
prior_rerun_result = prior_rerun.get("rerun_result") or {}
require(prior_rerun_result.get("candidate_pool_state") == "CandidateEligible", "prior rerun candidate drift")
require(prior_rerun_result.get("selected_surface_id") == "VariableContextNativeSurfaceOwnedReadSnapshotV1", "prior rerun surface drift")
require(prior_rerun_result.get("next_action") == "VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-HAKO-ADOPTION-DECISION-001", "prior rerun next action drift")

prior_state = fixture.get("prior_state") or {}
require(prior_state.get("candidate_pool_state") == "Blocked", "fixture prior pool drift")
require(prior_state.get("parked_reason") == "ReturnedMutableBorrow", "fixture prior parked reason drift")
require(prior_state.get("replacement_policy") == "ExplicitMutationApiOnly", "fixture prior replacement drift")
require(prior_state.get("denied_route") == "VariableContext_mutable_returned_borrow", "fixture prior denied route drift")

repair = fixture.get("repair_evidence") or {}
require(repair.get("projection") == "ExplicitMutationApiOnly", "fixture repair projection drift")
require(repair.get("replace_owned_map_native_api") == 1, "fixture replace_owned_map drift")
require(repair.get("raw_variable_map_mut_alias_emitted") == 0, "fixture raw alias drift")
require(repair.get("variable_map_mut_selected") == 0, "fixture mutable selected drift")

rerun = fixture.get("rerun_result") or {}
require(rerun.get("candidate_pool_state") == "CandidateEligible", "fixture rerun candidate drift")
require(rerun.get("selected_surface_id") == selected_surface, "fixture rerun surface drift")
require(rerun.get("reason_token") == "ReturnedMutableBorrowRepairedByExplicitMutationApiOnly", "fixture rerun reason drift")
require(rerun.get("next_action") == next_action, "fixture rerun next action drift")

for scope in [
    "VariableContext_simple_map_only",
    "VariableContext_snapshot_restore_only",
    "VariableContext_carrier_snapshot_only",
    "VariableContext_explicit_carrier_snapshot_only",
    "VariableContext_immutable_borrow_repaired_as_owned_snapshot",
    "VariableContext_mutable_returned_borrow_repaired_as_explicit_mutation",
]:
    require(scope in fixture.get("included_scopes", []), f"fixture missing included scope: {scope}")

excluded = {row.get("scope"): row.get("reason") for row in fixture.get("excluded_scopes", [])}
require(excluded.get("VariableContext_mutable_returned_borrow") == "ReturnedMutableBorrow", "fixture mutable exclusion drift")

claims = fixture.get("claims") or {}
for key in [
    "route_matrix_rerun",
    "manual_family_selection",
    "raw_variable_map_alias_selected",
    "variable_map_mut_selected",
    "full_variable_context_claim",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "hako_adopted",
]:
    require(claims.get(key) == (1 if key == "route_matrix_rerun" else 0), f"fixture claim drift: {key}")

require(selection.get("kind") == "VariableContextExplicitMutationSurfaceSelectionV1", "selection kind drift")
require((selection.get("candidate_recovery") or {}).get("next_action") == "MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-PROJECTION-001", "selection next action drift")

require(projection.get("kind") == "VariableContextExplicitMutationApiProjectionV1", "projection kind drift")
projection_state = projection.get("current_state") or {}
require(projection_state.get("latest_card") == "MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-PROJECTION-001", "projection latest card drift")
require((projection.get("candidate_recovery") or {}).get("next_action") == token, "projection next action drift")

rows = route_manifest.get("routes") or []
rows_by_scope = {row.get("pilot_scope") or row.get("mainline_selection_scope"): row for row in rows}
for scope in [
    "VariableContext_simple_map_only",
    "VariableContext_snapshot_restore_only",
    "VariableContext_carrier_snapshot_only",
    "VariableContext_explicit_carrier_snapshot_only",
]:
    row = rows_by_scope.get(scope)
    require(row is not None, f"route manifest missing selected scope: {scope}")
    require(row.get("family_id") == "hakorune_mir_builder::variable_context", f"route manifest family drift: {scope}")
    require(row.get("state") == "DerivedMainline", f"route manifest state drift: {scope}")
    require(row.get("selected_on_mainline") is True, f"route manifest selection drift: {scope}")
    require(row.get("fallback_policy") == "forbidden", f"route manifest fallback drift: {scope}")

denied = rows_by_scope.get("VariableContext_immutable_borrow_only")
require(denied is not None, "route manifest missing immutable borrow row")
require(denied.get("state") == "Denied", "route manifest immutable borrow state drift")
require(denied.get("deny_reason") == "ReturnedReadBorrow", "route manifest immutable borrow reason drift")
require(denied.get("replacement_policy") == "OwnedReadSnapshotProjection", "route manifest immutable borrow replacement drift")

claims_manifest = route_manifest.get("claims") or {}
require(claims_manifest.get("variable_context_selected") == 0, "route manifest selected claim drift")
require(claims_manifest.get("variable_context_simple_map_selected") == 1, "route manifest simple map claim drift")
require(claims_manifest.get("variable_context_snapshot_restore_selected") == 1, "route manifest snapshot claim drift")
require(claims_manifest.get("variable_context_carrier_snapshot_selected") == 1, "route manifest carrier snapshot claim drift")
require(claims_manifest.get("variable_context_explicit_carrier_snapshot_selected") == 1, "route manifest explicit carrier claim drift")
require(claims_manifest.get("variable_context_immutable_borrow_selected") == 0, "route manifest immutable borrow claim drift")
require(claims_manifest.get("full_variable_context_claim") == 0, "route manifest full claim drift")

allowed_current_tokens = {
    token,
    "VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-HAKO-ADOPTION-DECISION-001",
    "SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-EXPLICIT-MUTATION-RESOLUTION-001",
}
require(state.get("latest_card") in allowed_current_tokens, "current-state latest card drift")
require(state.get("current_blocker_token") in allowed_current_tokens, "current-state blocker drift")
allowed_current_paths = {
    "docs/development/current/main/phases/phase-296x/1792-MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-002.md",
    "docs/development/current/main/phases/phase-296x/1793-VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-HAKO-ADOPTION-DECISION-001.md",
    "docs/development/current/main/phases/phase-296x/1794-SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-EXPLICIT-MUTATION-RESOLUTION-001.md",
}
require(state.get("latest_card_path") in allowed_current_paths, "current-state latest card path drift")

for needle in [
    token,
    selected_surface,
    next_action,
]:
    require(needle in task_order, f"task-order missing {needle}")

require("tools/checks/rust_lifecycle_variable_context_route_matrix_rerun_002_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print("candidate_pool_state=CandidateEligible")
print(f"selected_surface_id={selected_surface}")
print("explicit_mutation_projection=green")
print("raw_variable_map_mut_alias_selected=0")
print("variable_map_mut_selected=0")
print("full_variable_context_claim=0")
print("manual_family_selection=0")
print("runtime_fallback=0")
print("new_backend_route=0")
print("new_abi=0")
print("new_python_semantic_projector=0")
print("source_selfhost_claim=0")
print(f"next_action={next_action}")
print("summary=ok")
PY
