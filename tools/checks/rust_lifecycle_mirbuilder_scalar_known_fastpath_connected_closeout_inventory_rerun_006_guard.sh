#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-006-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_connected_closeout_inventory_rerun_006.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3382-MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-006.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-006"

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


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-006"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-ALL-SURFACES-BASIS-001"
surface_ids = [
    "WriteScalarI64Routes",
    "WriteScalarI64Routes",
    "WriteScalarI64Routes",
    "MapLoadScalarI64Routes",
    "StringScalarI64Routes",
    "CollectionScalarI64Routes",
]

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathConnectedCloseoutInventoryRerun006V1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inventory = fixture.get("inventory") or {}
rows = inventory.get("connected_surface_rows") or []
need(inventory.get("connected_surface_row_count") == 6, "connected count drift")
need(inventory.get("known_unconnected_surface_row_count") == 0, "unconnected count drift")
need([row.get("surface_id") for row in rows] == surface_ids, "surface order drift")
need(inventory.get("known_unconnected_surface_rows") == [], "unconnected rows not empty")
need(inventory.get("write_surface_connection_complete") is True, "write complete drift")
need(inventory.get("read_surface_connection_complete") is True, "read complete drift")
need(inventory.get("all_known_scalar_known_surfaces_shadow_consumed") is True, "all-known shadow drift")
need(inventory.get("selection_eligible_candidate_count") == 1, "eligible count drift")

selected = inventory.get("selected_candidate") or {}
need(selected.get("selection_kind") == "AllKnownScalarKnownFastpathSurfacesConnectedCloseoutBasis", "selection kind drift")
need(selected.get("selected_next_card") == next_card, "selected next drift")

rule = inventory.get("selection_rule") or {}
need(rule.get("name") == "AllKnownScalarKnownFastpathSurfacesConnectedV1", "rule drift")
for key in [
    "requires_zero_unconnected_surfaces",
    "requires_write_surface_connection_complete",
    "requires_read_surface_connection_complete",
]:
    need(rule.get(key) is True, f"required rule drift: {key}")
for key in [
    "row_count_as_proof",
    "route_count_as_proof",
    "manual_surface_selection",
    "hako_runtime_authority_switch",
    "owner_name_as_proof",
    "source_path_as_authority",
]:
    need(rule.get(key) is False, f"forbidden rule drift: {key}")

input_state = fixture.get("input_state") or {}
for key in ["mapload", "string", "collection", "mapstore_i64", "mapstore_any", "write_push"]:
    claims = (input_state.get(key) or {}).get("claims") or {}
    need(claims.get("generated_typed_hako_artifact_shadow_consumed") == 1, f"shadow claim missing: {key}")

summary = fixture.get("summary") or {}
need(summary.get("fastpath_connected_closeout_inventory_rerun_006") == 1, "summary missing rerun")
need(summary.get("connected_surface_row_count") == 6, "summary connected drift")
need(summary.get("known_unconnected_surface_row_count") == 0, "summary unconnected drift")
need(summary.get("all_known_scalar_known_surfaces_shadow_consumed") == 1, "summary all-known drift")
need(summary.get("selected_next_card") == next_card, "summary next drift")
for key in ["fastpath_connected_closeout", "hako_runtime_route_authority", "rust_fastpath_rewired", "source_selfhost_claim"]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectAllKnownScalarKnownFastpathConnectedCloseoutBasis", "decision kind drift")
need(decision.get("reason_token") == "AllKnownScalarKnownFastpathSurfacesShadowConsumed", "reason drift")
need(decision.get("selected_next_card") == next_card, "decision next drift")

claims = fixture.get("claims") or {}
for key in [
    "fastpath_connected_closeout_inventory_rerun_006",
    "write_surface_connection_complete",
    "read_surface_connection_complete",
    "all_known_scalar_known_surfaces_shadow_consumed",
    "selection_eligible_candidate_count",
]:
    need(claims.get(key) == 1, f"positive claim drift: {key}")
for key in [
    "fastpath_connected_closeout",
    "hako_runtime_route_authority",
    "rust_fastpath_rewired",
    "route_selection_authority_switch",
    "backend_lowering_authority",
    "runtime_mutation_authority",
    "publication_execution",
    "build_rs_hako_compiler_invocation",
    "live_hako_authority",
    "caller_orientation_runtime_path",
    "source_selfhost_claim",
    "hako_generation",
    "new_route_authority",
    "behavior_change",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "native_seed_materialization",
    "new_python_semantic_projector",
    "manual_axis_selection",
    "manual_carrier_selection",
    "manual_subsurface_selection",
    "manual_surface_selection",
    "row_count_as_proof",
    "route_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("3382-MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-006.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-006-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_scalar_known_fastpath_connected_closeout_inventory_rerun_006_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-006")
print("connected_surface_row_count=6")
print("known_unconnected_surface_row_count=0")
print("write_surface_connection_complete=1")
print("read_surface_connection_complete=1")
print("all_known_scalar_known_surfaces_shadow_consumed=1")
print("fastpath_connected_closeout=0")
print("hako_runtime_route_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY
