#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-boundary-selection-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_set_mapstore_any_boundary_selection.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2139-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-BOUNDARY-SELECTION-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-boundary-selection"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-BOUNDARY-SELECTION-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-WRITE-BOUNDARY-BASIS-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownWriteSetMapStoreAnyBoundarySelectionV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("accepted_scoped_closeout_count") == 6, "closeout count drift")
need(inputs.get("previous_selected_next_card") == token, "previous next drift")
need(inputs.get("remaining_write_scoped_surfaces") == ["SetSurfacePolicy/MapStoreAny"], "remaining surface drift")
need((inputs.get("remaining_write_surface_blockers") or {}).get("SetSurfacePolicy/MapStoreAny") == "AnyWriteBoundaryRequired", "blocker drift")

consultation = fixture.get("consultation_result") or {}
need(consultation.get("selected_option") == "B", "consultation option drift")
need(consultation.get("selected_basis") == "AnyWriteBoundaryBasis", "basis drift")
need(consultation.get("mapstore_any_within_scalar_known_closeout_chain") is True, "lane drift")
need(consultation.get("immediate_hako_pilot_allowed") is False, "immediate pilot drift")
need(consultation.get("scalar_known_lane_escape_selected") is False, "lane escape drift")

rule = fixture.get("selection_rule") or {}
need(rule.get("basis_first") is True, "basis first drift")
need(rule.get("hako_pilot_requires_boundary_basis") is True, "pilot requires basis drift")
need(rule.get("any_write_boundary_declared_at_next_basis") is True, "declared-at-next drift")
need(rule.get("any_write_boundary_opened_at_selection") is False, "opened-at-selection drift")
for key in [
    "route_count_as_proof",
    "apparent_simplicity_as_proof",
    "manual_subsurface_selection",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
]:
    need(rule.get(key) is False, f"forbidden rule drift: {key}")

summary = fixture.get("summary") or {}
for key in [
    "mapstore_any_boundary_selection",
    "selected_option_b",
    "selected_next_is_boundary_basis",
    "mapstore_any_remaining",
    "mapstore_i64_already_scoped_closeout",
]:
    need(summary.get(key) == 1, f"missing summary claim: {key}")
for key in [
    "any_write_boundary_declared",
    "any_write_boundary_opened",
    "mapstore_any_hako_pilot_selected",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"forbidden summary drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectMapStoreAnyWriteBoundaryBasis", "decision kind drift")
need(decision.get("reason_token") == "ConsultationSelectedBasisFirstForAnyWriteBoundary", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in ["mapstore_any_boundary_selection", "selected_option_b", "selected_next_is_boundary_basis"]:
    need(claims.get(key) == 1, f"missing claim: {key}")
for key in [
    "any_write_boundary_declared",
    "any_write_boundary_opened",
    "mapstore_any_hako_pilot_selected",
    "mapstore_any_direct_closeout_materialized",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "runtime_mutation_authority",
    "publication_execution",
    "source_selfhost_claim",
    "new_route_authority",
    "new_backend_route",
    "new_abi",
    "runtime_fallback",
    "behavior_change",
    "hako_generation",
    "native_seed_materialization",
    "route_count_as_proof",
    "apparent_simplicity_as_proof",
    "manual_subsurface_selection",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2139-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-BOUNDARY-SELECTION-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-boundary-selection-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_set_mapstore_any_boundary_selection_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-boundary-selection")
print("selected_option_b=1")
print("selected_next_is_boundary_basis=1")
print("any_write_boundary_declared=0")
print("any_write_boundary_opened=0")
print("mapstore_any_hako_pilot_selected=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY
