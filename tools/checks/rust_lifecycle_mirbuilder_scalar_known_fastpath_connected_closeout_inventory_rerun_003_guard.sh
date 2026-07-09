#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-003-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_connected_closeout_inventory_rerun_003.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3372-MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-003.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
MAPSTORE_I64="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-set-mapstore-i64-v0.json"
MAPSTORE_ANY="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-set-mapstore-any-v0.json"
WRITE_PUSH="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-write-push-v0.json"
SCALAR_CONTRACT="$ROOT/src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs"
COLLECTION_READ_ROUTES="$ROOT/src/mir/generic_method_route_plan/collection_read_routes.rs"
STRING_ROUTES="$ROOT/src/mir/generic_method_route_plan/string_routes.rs"
WRITE_ROUTES="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-003"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" \
  "$MAPSTORE_I64" "$MAPSTORE_ANY" "$WRITE_PUSH" "$SCALAR_CONTRACT" \
  "$COLLECTION_READ_ROUTES" "$STRING_ROUTES" "$WRITE_ROUTES"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$MAPSTORE_I64" "$MAPSTORE_ANY" "$WRITE_PUSH" "$SCALAR_CONTRACT" "$COLLECTION_READ_ROUTES" "$STRING_ROUTES" "$WRITE_ROUTES" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))
mapstore_i64 = json.load(open(sys.argv[5], encoding="utf-8"))
mapstore_any = json.load(open(sys.argv[6], encoding="utf-8"))
write_push = json.load(open(sys.argv[7], encoding="utf-8"))
scalar_contract = Path(sys.argv[8]).read_text(encoding="utf-8")
collection_read_routes = Path(sys.argv[9]).read_text(encoding="utf-8")
string_routes = Path(sys.argv[10]).read_text(encoding="utf-8")
write_routes = Path(sys.argv[11]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-003"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-SURFACE-GENERATED-TYPED-ARTIFACT-SELECTION-DESIGN-CONSULTATION-001"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathConnectedCloseoutInventoryRerun003V1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

for source, label in [
    (mapstore_i64, "MapStoreI64"),
    (mapstore_any, "MapStoreAny"),
    (write_push, "WritePush"),
]:
    need((source.get("claims") or {}).get("generated_typed_hako_artifact_shadow_consumed") == 1, f"{label} connection not proven")

inventory = fixture.get("inventory") or {}
need(inventory.get("connected_surface_row_count") == 3, "connected count drift")
need(inventory.get("known_unconnected_surface_row_count") == 3, "unconnected count drift")
need(inventory.get("write_surface_connection_complete") is True, "write connection drift")
need(inventory.get("read_surface_connection_complete") is False, "read connection drift")
need(inventory.get("selection_eligible_candidate_count") == 0, "eligible count drift")
need(inventory.get("selected_candidate") is None, "unexpected selected candidate")

connected = inventory.get("connected_surface_rows") or []
connected_names = {
    "/".join(filter(None, [row.get("surface_id"), row.get("subsurface_id"), row.get("route_kind")]))
    for row in connected
}
for expected in [
    "WriteScalarI64Routes/SetSurfacePolicy/MapStoreI64/MapStoreI64",
    "WriteScalarI64Routes/SetSurfacePolicy/MapStoreAny/MapStoreAny",
    "WriteScalarI64Routes/PushSurfacePolicy/ArrayAppendAny",
]:
    need(expected in connected_names, f"missing connected row: {expected}")

unconnected = inventory.get("known_unconnected_surface_rows") or []
need([row.get("surface_id") for row in unconnected] == [
    "MapLoadScalarI64Routes",
    "StringScalarI64Routes",
    "CollectionScalarI64Routes",
], "unconnected read surface order drift")
for row in unconnected:
    blocked = row.get("blocked_by") or []
    need("NoCheckedInGeneratedTypedHakoPolicyArtifact" in blocked, "missing no artifact blocker")
    need("NoMechanicalReadSurfacePriorityAfterWriteContinuationConsumed" in blocked, "missing priority blocker")

rule = inventory.get("selection_rule") or {}
need(rule.get("name") == "RequireConsultationApprovedReadSurfaceGeneratedTypedArtifactPriorityV1", "rule drift")
for key in [
    "write_continuation_consumed",
]:
    need(rule.get(key) is True, f"required rule drift: {key}")
for key in [
    "route_count_as_proof",
    "manual_surface_selection",
    "hako_runtime_authority_switch",
    "read_surface_selection",
    "owner_name_as_proof",
    "source_path_as_authority",
]:
    need(rule.get(key) is False, f"forbidden rule drift: {key}")

provenance = fixture.get("provenance") or {}
need(len(provenance.get("absent_read_policy_sources") or []) == 3, "read policy source absence drift")
need(len(provenance.get("absent_read_generated_artifacts") or []) == 3, "read artifact absence drift")

for token_text in [
    "MapLoadScalarI64Routes",
    "StringScalarI64Routes",
    "CollectionScalarI64Routes",
    "WriteScalarI64Routes",
]:
    need(token_text in scalar_contract, f"contract missing {token_text}")
need("GenericMethodRouteKind::MapLoadScalarI64" in collection_read_routes, "collection_read_routes missing MapLoadScalarI64")
need("GenericMethodRouteKind::StringIndexOf" in string_routes, "string_routes missing StringIndexOf")
need("write_push_shadow_consumed_decision" in write_routes, "write_routes missing Push shadow consumer")

summary = fixture.get("summary") or {}
need(summary.get("fastpath_connected_closeout_inventory_rerun_003") == 1, "missing rerun summary")
need(summary.get("connected_surface_row_count") == 3, "summary connected count drift")
need(summary.get("known_unconnected_surface_row_count") == 3, "summary unconnected count drift")
need(summary.get("write_surface_connection_complete") == 1, "summary write drift")
need(summary.get("read_surface_connection_complete") == 0, "summary read drift")
need(summary.get("selection_eligible_candidate_count") == 0, "summary eligible drift")
for key in ["fastpath_connected_closeout", "hako_runtime_route_authority", "rust_fastpath_rewired", "source_selfhost_claim"]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStoppedDesignConsultationRequired", "decision kind drift")
need(decision.get("reason_token") == "NoMechanicalReadSurfaceGeneratedTypedArtifactPriority", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("fastpath_connected_closeout_inventory_rerun_003") == 1, "missing inventory claim")
need(claims.get("write_surface_connection_complete") == 1, "claim write drift")
need(claims.get("read_surface_connection_complete") == 0, "claim read drift")
need(claims.get("selection_eligible_candidate_count") == 0, "claim eligible drift")
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
need(manifest_row.get("card", "").endswith("3372-MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-003.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-003-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_scalar_known_fastpath_connected_closeout_inventory_rerun_003_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-003")
print("connected_surface_row_count=3")
print("known_unconnected_surface_row_count=3")
print("write_surface_connection_complete=1")
print("read_surface_connection_complete=0")
print("selection_eligible_candidate_count=0")
print("decision=KeepStoppedDesignConsultationRequired")
print("fastpath_connected_closeout=0")
print("hako_runtime_route_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY
