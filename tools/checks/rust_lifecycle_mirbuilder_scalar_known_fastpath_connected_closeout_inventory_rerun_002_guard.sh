#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-002-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_connected_closeout_inventory_rerun_002.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3369-MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-002.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
MAPSTORE_I64="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-set-mapstore-i64-v0.json"
MAPSTORE_ANY="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-set-mapstore-any-v0.json"
PUSH_HAKO="$ROOT/lang/src/compiler/lib/write_push_surface_policy_classifier.hako"
WRITE_ROUTES="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-002"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" "$MAPSTORE_I64" "$MAPSTORE_ANY" "$PUSH_HAKO" "$WRITE_ROUTES"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$MAPSTORE_I64" "$MAPSTORE_ANY" "$PUSH_HAKO" "$WRITE_ROUTES" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))
mapstore_i64 = json.load(open(sys.argv[5], encoding="utf-8"))
mapstore_any = json.load(open(sys.argv[6], encoding="utf-8"))
push_hako = Path(sys.argv[7]).read_text(encoding="utf-8")
write_routes = Path(sys.argv[8]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-002"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-WRITE-PUSH-BASIS-001"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathConnectedCloseoutInventoryRerun002V1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

need((mapstore_i64.get("claims") or {}).get("generated_typed_hako_artifact_shadow_consumed") == 1, "MapStoreI64 connection not proven")
need((mapstore_any.get("claims") or {}).get("generated_typed_hako_artifact_shadow_consumed") == 1, "MapStoreAny connection not proven")

inventory = fixture.get("inventory") or {}
need(inventory.get("connected_surface_row_count") == 2, "connected count drift")
need(inventory.get("known_unconnected_surface_row_count") == 4, "unconnected count drift")
need(inventory.get("selection_eligible_candidate_count") == 1, "eligible count drift")
selected = inventory.get("selected_candidate") or {}
need(selected.get("surface_id") == "WriteScalarI64Routes", "selected surface drift")
need(selected.get("subsurface_id") == "PushSurfacePolicy", "selected subsurface drift")
need(selected.get("route_kind") == "ArrayAppendAny", "selected route drift")
need(selected.get("selection_kind") == "PriorWriteRouteGeneratedTypedArtifactContinuation", "selection kind drift")
rule = inventory.get("selection_rule") or {}
need(rule.get("name") == "PriorWriteRouteGeneratedTypedArtifactContinuationV1", "rule drift")
for key in ["route_count_as_proof", "manual_surface_selection", "hako_runtime_authority_switch", "read_surface_selection"]:
    need(rule.get(key) is False, f"forbidden rule drift: {key}")

need("ArrayAppendAny" in push_hako and "PushSurfacePolicy" in push_hako, "push hako source missing policy row")
need("GenericMethodRouteKind::ArrayAppendAny" in write_routes, "write_routes missing push route kind")
need("GenericMethodRouteProof::PushSurfacePolicy" in write_routes, "write_routes missing push proof")

summary = fixture.get("summary") or {}
need(summary.get("fastpath_connected_closeout_inventory_rerun_002") == 1, "missing rerun summary")
need(summary.get("connected_surface_row_count") == 2, "summary connected count drift")
need(summary.get("known_unconnected_surface_row_count") == 4, "summary unconnected count drift")
need(summary.get("selection_eligible_candidate_count") == 1, "summary eligible count drift")
need(summary.get("selected_surface") == "WriteScalarI64Routes/PushSurfacePolicy", "summary selected drift")
for key in ["fastpath_connected_closeout", "hako_runtime_route_authority", "rust_fastpath_rewired", "source_selfhost_claim"]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectWritePushGeneratedTypedArtifactShadowConsumeBasis", "decision kind drift")
need(decision.get("reason_token") == "PriorWriteRouteGeneratedTypedArtifactContinuation", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("fastpath_connected_closeout_inventory_rerun_002") == 1, "missing inventory claim")
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
need(manifest_row.get("card", "").endswith("3369-MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-002.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-002-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_scalar_known_fastpath_connected_closeout_inventory_rerun_002_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-002")
print("connected_surface_row_count=2")
print("known_unconnected_surface_row_count=4")
print("selection_eligible_candidate_count=1")
print("selected_surface=WriteScalarI64Routes/PushSurfacePolicy")
print("fastpath_connected_closeout=0")
print("hako_runtime_route_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY
