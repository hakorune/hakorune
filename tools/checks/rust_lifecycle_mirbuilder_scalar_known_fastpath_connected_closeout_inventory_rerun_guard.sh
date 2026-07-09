#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_connected_closeout_inventory_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3366-MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
MAPSTORE_ANY_HAKO="$ROOT/lang/src/compiler/lib/write_set_mapstore_any_policy_classifier.hako"
MAPSTORE_I64_ARTIFACT="$ROOT/src/mir/generic_method_route_plan/generated/write_set_mapstore_i64_hako_policy.rs"
WRITE_ROUTES="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" "$MAPSTORE_ANY_HAKO" "$MAPSTORE_I64_ARTIFACT" "$WRITE_ROUTES"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$MAPSTORE_ANY_HAKO" "$MAPSTORE_I64_ARTIFACT" "$WRITE_ROUTES" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))
mapstore_any_hako = Path(sys.argv[5]).read_text(encoding="utf-8")
mapstore_i64_artifact = Path(sys.argv[6]).read_text(encoding="utf-8")
write_routes = Path(sys.argv[7]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-SET-MAPSTORE-ANY-BASIS-001"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathConnectedCloseoutInventoryRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("basis_decision") == "SelectFastpathConnectedCloseoutInventoryRerun", "basis decision drift")
need(inputs.get("basis_selected_next_card") == token, "basis next drift")
need(inputs.get("fastpath_connected_closeout_basis_hash"), "missing basis hash")

inventory = fixture.get("inventory") or {}
need(inventory.get("connected_surface_row_count") == 1, "connected count drift")
need(inventory.get("known_unconnected_surface_row_count") == 5, "unconnected count drift")
need(inventory.get("selection_eligible_candidate_count") == 1, "eligible count drift")
selected = inventory.get("selected_candidate") or {}
need(selected.get("surface_id") == "WriteScalarI64Routes", "selected surface drift")
need(selected.get("subsurface_id") == "SetSurfacePolicy/MapStoreAny", "selected subsurface drift")
need(selected.get("route_kind") == "MapStoreAny", "selected route drift")
need(selected.get("selection_kind") == "SameSetSurfacePolicyGeneratedTypedArtifactShadowConsume", "selection kind drift")
rule = inventory.get("selection_rule") or {}
need(rule.get("name") == "PriorGeneratedTypedArtifactSameSetSurfacePolicyMinimalDeltaV1", "rule drift")
for key in ["route_count_as_proof", "manual_surface_selection", "hako_runtime_authority_switch"]:
    need(rule.get(key) is False, f"forbidden rule drift: {key}")

need("map_store_any_set_surface|SetSurfacePolicy/MapStoreAny|MapStoreAny" in mapstore_any_hako, "MapStoreAny hako row missing")
need("WRITE_SET_MAPSTORE_I64_HAKO_POLICY" in mapstore_i64_artifact, "prior generated artifact missing")
need("scalar_known_hako_shadow::mapstore_i64_shadow_consumed_decision()" in write_routes, "prior fastpath connection missing")

summary = fixture.get("summary") or {}
for key in ["fastpath_connected_closeout_inventory_rerun"]:
    need(summary.get(key) == 1, f"summary positive drift: {key}")
need(summary.get("connected_surface_row_count") == 1, "summary connected count drift")
need(summary.get("known_unconnected_surface_row_count") == 5, "summary unconnected count drift")
need(summary.get("selection_eligible_candidate_count") == 1, "summary eligible count drift")
need(summary.get("selected_surface") == "WriteScalarI64Routes/SetSurfacePolicy/MapStoreAny", "summary selected drift")
for key in ["fastpath_connected_closeout", "hako_runtime_route_authority", "rust_fastpath_rewired", "source_selfhost_claim"]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectMapStoreAnyGeneratedTypedArtifactShadowConsumeBasis", "decision kind drift")
need(decision.get("reason_token") == "PriorMapStoreI64GeneratedTypedArtifactSameSetSurfacePolicyMinimalDelta", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("fastpath_connected_closeout_inventory_rerun") == 1, "missing inventory claim")
need(claims.get("selection_eligible_candidate_count") == 1, "claim eligible drift")
for key in [
    "fastpath_connected_closeout",
    "hako_runtime_route_authority",
    "rust_fastpath_rewired",
    "source_selfhost_claim",
    "hako_generation",
    "new_route_authority",
    "behavior_change",
    "runtime_mutation_authority",
    "publication_execution",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "native_seed_materialization",
    "new_python_semantic_projector",
    "manual_axis_selection",
    "manual_carrier_selection",
    "manual_subsurface_selection",
    "row_count_as_proof",
    "route_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("3366-MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_scalar_known_fastpath_connected_closeout_inventory_rerun_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun")
print("connected_surface_row_count=1")
print("known_unconnected_surface_row_count=5")
print("selection_eligible_candidate_count=1")
print("selected_surface=WriteScalarI64Routes/SetSurfacePolicy/MapStoreAny")
print("fastpath_connected_closeout=0")
print("hako_runtime_route_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY
