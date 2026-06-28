#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-296x/1783-VARIABLE-CONTEXT-NATIVE-SURFACE-HAKO-ADOPTION-DECISION-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-native-surface-hako-adoption-decision-v0.json"
SELECTION="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-native-surface-adoption-selection-v0.json"
ROUTE_MANIFEST="lang/generated/rust_derived/hakorune_mir_builder/family_routes.json"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

bash tools/checks/rust_mirbuilder_variable_context_native_simple_map_guard.sh >/tmp/hako_variable_context_native_simple_map_guard.out
bash tools/checks/rust_mirbuilder_variable_context_native_snapshot_restore_guard.sh >/tmp/hako_variable_context_native_snapshot_restore_guard.out
bash tools/checks/rust_mirbuilder_carrier_info_native_snapshot_guard.sh >/tmp/hako_carrier_info_native_snapshot_guard.out

python3 - "$CARD" "$FIXTURE" "$SELECTION" "$ROUTE_MANIFEST" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
selection_path = Path(sys.argv[3])
route_manifest_path = Path(sys.argv[4])
state_path = Path(sys.argv[5])
task_order_path = Path(sys.argv[6])
index_path = Path(sys.argv[7])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
selection = json.loads(selection_path.read_text(encoding="utf-8"))
route_manifest = json.loads(route_manifest_path.read_text(encoding="utf-8"))
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "VARIABLE-CONTEXT-NATIVE-SURFACE-HAKO-ADOPTION-DECISION-001"
output_contract = "rust-lifecycle-variable-context-native-surface-hako-adoption-decision-v0"
surface_id = "VariableContextNativeSurfaceNoReturnedBorrowV1"
included_scopes = [
    "VariableContext_simple_map_only",
    "VariableContext_snapshot_restore_only",
    "VariableContext_carrier_snapshot_only",
    "VariableContext_explicit_carrier_snapshot_only",
]
native_sources = [
    "apps/lib/hakorune_mir_builder/variable_context.hako",
    "apps/lib/hakorune_mir_builder/carrier_info.hako",
]
native_guards = [
    "tools/checks/rust_mirbuilder_variable_context_native_simple_map_guard.sh",
    "tools/checks/rust_mirbuilder_variable_context_native_snapshot_restore_guard.sh",
    "tools/checks/rust_mirbuilder_carrier_info_native_snapshot_guard.sh",
]

require(f"# {token}" in card, "card token drift")
require(f"output_contract={output_contract}" in card, "card output contract drift")
require("decision = Adopt" in card, "card decision drift")
require("full_variable_context_claim = 0" in card, "card full claim drift")
require("returned_borrow_selected = 0" in card, "card returned borrow drift")

require(fixture.get("kind") == "VariableContextNativeSurfaceHakoAdoptionDecisionV1", "fixture kind drift")
require(fixture.get("output_contract") == output_contract, "fixture output contract drift")
require((fixture.get("selection_evidence") or {}).get("surface_id") == surface_id, "fixture selected surface drift")

fixture_state = fixture.get("current_state") or {}
require(fixture_state.get("latest_card") == token, "fixture latest card drift")
require(fixture_state.get("current_blocker_token") == token, "fixture blocker drift")

target = fixture.get("target") or {}
require(target.get("family_id") == "hakorune_mir_builder::variable_context", "fixture target family drift")
require(target.get("surface_id") == surface_id, "fixture surface drift")
require(target.get("full_family_state") == "Parked", "fixture full family state drift")
require(target.get("full_variable_context_claim") == 0, "fixture full claim drift")

require(fixture.get("included_scopes") == included_scopes, "fixture included scopes drift")
excluded = {item.get("scope"): item.get("reason") for item in fixture.get("excluded_scopes", [])}
require(excluded.get("VariableContext_immutable_borrow_only") == "ReturnedReadBorrow", "fixture immutable borrow exclusion drift")
require(excluded.get("VariableContext_mutable_returned_borrow") == "ReturnedMutableBorrow", "fixture mutable borrow exclusion drift")

for path in native_sources:
    require(path in fixture.get("native_source_owners", []), f"fixture missing native source: {path}")
    require(Path(path).exists(), f"native source missing: {path}")

for guard in native_guards:
    require(guard in fixture.get("native_behavior_guards", []), f"fixture missing native guard: {guard}")
    require(Path(guard).exists(), f"native guard missing: {guard}")

decision = fixture.get("decision") or {}
require(decision.get("value") == "Adopt", "fixture decision drift")
require(decision.get("reason_token") == "NativeSurfaceOwnerPresentAndGreen", "fixture reason drift")
require(decision.get("selected_next_route") == "native_hako_source_owner", "fixture next route drift")

claims = fixture.get("claims") or {}
for key in [
    "native_hako_source_owner_present",
    "native_behavior_guard_green",
    "generator_overwrite_guard",
    "rust_bootstrap_retained",
    "rust_oracle_retained",
]:
    require(claims.get(key) == 1, f"fixture positive claim drift: {key}")
for key in [
    "generated_artifact_manual_edit",
    "full_variable_context_claim",
    "returned_borrow_selected",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
]:
    require(claims.get(key) == 0, f"fixture non-claim drift: {key}")

selection_surface = selection.get("native_surface") or {}
require(selection_surface.get("surface_id") == surface_id, "selection fixture surface drift")
require(selection_surface.get("candidate_state") == "CandidateEligible", "selection fixture candidate drift")
require(selection_surface.get("included_scopes") == included_scopes, "selection fixture included scopes drift")

rows = route_manifest.get("routes") or []
rows_by_scope = {row.get("pilot_scope") or row.get("mainline_selection_scope"): row for row in rows}
for scope in included_scopes:
    row = rows_by_scope.get(scope)
    require(row is not None, f"route manifest missing included scope: {scope}")
    require(row.get("family_id") == "hakorune_mir_builder::variable_context", f"route manifest family drift: {scope}")
    require(row.get("state") == "DerivedMainline", f"route manifest state drift: {scope}")
    require(row.get("selected_on_mainline") is True, f"route manifest selection drift: {scope}")
    require(row.get("fallback_policy") == "forbidden", f"route manifest fallback drift: {scope}")
    require(row.get("rust_bootstrap_route") == "retained", f"route manifest bootstrap drift: {scope}")
    require(row.get("rust_oracle_route") == "retained", f"route manifest oracle drift: {scope}")

denied = rows_by_scope.get("VariableContext_immutable_borrow_only")
require(denied is not None, "route manifest missing immutable borrow row")
require(denied.get("state") == "Denied", "route manifest immutable borrow state drift")
require(denied.get("deny_reason") == "ReturnedReadBorrow", "route manifest immutable borrow reason drift")

current_latest = state.get("latest_card")
current_blocker = state.get("current_blocker_token")
allowed_current_tokens = {
    token,
    "SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-SURFACE-RESOLUTION-001",
    "MIRBUILDER-VARIABLE-CONTEXT-RETURNED-READ-SNAPSHOT-ROUTE-001",
    "MIRBUILDER-VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-PROJECTION-001",
    "MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-001",
}
require(current_latest in allowed_current_tokens, "current-state latest card drift")
require(current_blocker in allowed_current_tokens, "current-state blocker drift")
require(Path(state.get("latest_card_path", "")).exists(), "current-state latest card path missing")

for needle in [
    token,
    "VariableContextNativeSurfaceNoReturnedBorrowV1",
    "decision = Adopt",
]:
    require(needle in task_order, f"task-order missing {needle}")

require("tools/checks/rust_lifecycle_variable_context_native_surface_hako_adoption_decision_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print(f"surface_id={surface_id}")
print("decision=Adopt")
print("included_scope_count=4")
print("native_hako_source_owner_present=1")
print("native_behavior_guard_green=1")
print("generator_overwrite_guard=1")
print("rust_bootstrap_retained=1")
print("rust_oracle_retained=1")
print("full_variable_context_claim=0")
print("returned_borrow_selected=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("new_backend_route=0")
print("new_abi=0")
print("summary=ok")
PY
