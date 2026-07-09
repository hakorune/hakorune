#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-004-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_connected_closeout_inventory_rerun_004.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3376-MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-004.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
MAPLOAD="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-mapload-scalar-i64-v0.json"
STRING_ROUTES="$ROOT/src/mir/generic_method_route_plan/string_routes.rs"
COLLECTION_READ_ROUTES="$ROOT/src/mir/generic_method_route_plan/collection_read_routes.rs"
SCALAR_CONTRACT="$ROOT/src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-004"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" \
  "$MAPLOAD" "$STRING_ROUTES" "$COLLECTION_READ_ROUTES" "$SCALAR_CONTRACT"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$MAPLOAD" "$STRING_ROUTES" "$COLLECTION_READ_ROUTES" "$SCALAR_CONTRACT" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))
mapload = json.load(open(sys.argv[5], encoding="utf-8"))
string_routes = Path(sys.argv[6]).read_text(encoding="utf-8")
collection_read_routes = Path(sys.argv[7]).read_text(encoding="utf-8")
scalar_contract = Path(sys.argv[8]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-004"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-BASIS-STRING-SCALAR-I64-001"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathConnectedCloseoutInventoryRerun004V1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need((mapload.get("claims") or {}).get("generated_typed_hako_artifact_shadow_consumed") == 1, "MapLoad shadow consume not proven")

inventory = fixture.get("inventory") or {}
need(inventory.get("connected_surface_row_count") == 4, "connected count drift")
need(inventory.get("known_unconnected_surface_row_count") == 2, "unconnected count drift")
need(inventory.get("write_surface_connection_complete") is True, "write complete drift")
need(inventory.get("read_mapload_connection_complete") is True, "mapload complete drift")
need(inventory.get("read_surface_connection_complete") is False, "read complete drift")
need(inventory.get("selection_eligible_candidate_count") == 1, "eligible count drift")

connected = inventory.get("connected_surface_rows") or []
connected_names = {
    "/".join(filter(None, [row.get("surface_id"), row.get("subsurface_id"), row.get("route_kind")]))
    for row in connected
}
for expected in [
    "WriteScalarI64Routes/SetSurfacePolicy/MapStoreI64/MapStoreI64",
    "WriteScalarI64Routes/SetSurfacePolicy/MapStoreAny/MapStoreAny",
    "WriteScalarI64Routes/PushSurfacePolicy/ArrayAppendAny",
    "MapLoadScalarI64Routes/MapLoadScalarI64",
]:
    need(expected in connected_names, f"missing connected row: {expected}")

selected = inventory.get("selected_candidate") or {}
need(selected.get("surface_id") == "StringScalarI64Routes", "selected surface drift")
need(selected.get("selection_kind") == "ReadSurfaceGeneratedArtifactMinimalityAfterMapLoad", "selection kind drift")
need(selected.get("route_kind_family") == ["StringIndexOf", "StringLastIndexOf", "StringContains"], "string route family drift")

unconnected = inventory.get("known_unconnected_surface_rows") or []
need([row.get("surface_id") for row in unconnected] == [
    "StringScalarI64Routes",
    "CollectionScalarI64Routes",
], "unconnected surface order drift")
need((unconnected[0].get("blocked_by") or []) == ["NoCheckedInGeneratedTypedHakoPolicyArtifact"], "string blocker drift")
need("MixedReceiverDomainFamiliesAfterStringReadCandidate" in (unconnected[1].get("blocked_by") or []), "collection blocker drift")

rule = inventory.get("selection_rule") or {}
need(rule.get("name") == "ReadSurfaceGeneratedArtifactMinimalityAfterMapLoadV1", "rule drift")
need(rule.get("prior_mapload_shadow_consumed") is True, "prior mapload drift")
for key in [
    "route_count_as_proof",
    "manual_surface_selection",
    "hako_runtime_authority_switch",
    "owner_name_as_proof",
    "source_path_as_authority",
]:
    need(rule.get(key) is False, f"forbidden rule drift: {key}")

provenance = fixture.get("provenance") or {}
need(len(provenance.get("absent_read_policy_sources") or []) == 2, "read policy absence drift")
need(len(provenance.get("absent_read_generated_artifacts") or []) == 2, "read artifact absence drift")

for token_text in [
    "StringIndexOf",
    "StringLastIndexOf",
    "StringContains",
]:
    need(f"GenericMethodRouteKind::{token_text}" in string_routes, f"string route missing {token_text}")
need("mapload_scalar_i64_shadow_consumed_decision" in collection_read_routes, "collection_read_routes missing MapLoad shadow consumer")
need("ScalarKnownSurfaceId::StringScalarI64Routes" in scalar_contract, "contract missing string surface")
need("ScalarKnownSurfaceId::CollectionScalarI64Routes" in scalar_contract, "contract missing collection surface")

summary = fixture.get("summary") or {}
need(summary.get("fastpath_connected_closeout_inventory_rerun_004") == 1, "summary missing rerun")
need(summary.get("connected_surface_row_count") == 4, "summary connected drift")
need(summary.get("known_unconnected_surface_row_count") == 2, "summary unconnected drift")
need(summary.get("selected_surface") == "StringScalarI64Routes", "summary selected drift")
for key in ["fastpath_connected_closeout", "hako_runtime_route_authority", "rust_fastpath_rewired", "source_selfhost_claim"]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectStringScalarI64GeneratedTypedArtifactBasis", "decision kind drift")
need(decision.get("reason_token") == "ReadSurfaceGeneratedArtifactMinimalityAfterMapLoad", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("fastpath_connected_closeout_inventory_rerun_004") == 1, "missing inventory claim")
need(claims.get("read_mapload_connection_complete") == 1, "mapload claim drift")
need(claims.get("read_surface_connection_complete") == 0, "read claim drift")
need(claims.get("selection_eligible_candidate_count") == 1, "eligible claim drift")
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
need(manifest_row.get("card", "").endswith("3376-MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-004.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-004-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_scalar_known_fastpath_connected_closeout_inventory_rerun_004_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-004")
print("connected_surface_row_count=4")
print("known_unconnected_surface_row_count=2")
print("read_mapload_connection_complete=1")
print("read_surface_connection_complete=0")
print("selection_eligible_candidate_count=1")
print("selected_surface=StringScalarI64Routes")
print("fastpath_connected_closeout=0")
print("hako_runtime_route_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY
