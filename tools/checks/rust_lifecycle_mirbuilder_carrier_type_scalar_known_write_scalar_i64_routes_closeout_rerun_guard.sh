#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-scalar-i64-routes-closeout-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_scalar_i64_routes_closeout_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3363-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SCALAR-I64-ROUTES-CLOSEOUT-RERUN-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-scalar-i64-routes-closeout-rerun"

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


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SCALAR-I64-ROUTES-CLOSEOUT-RERUN-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-RERUN-002"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownWriteScalarI64RoutesCloseoutRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("basis_decision") == "SelectWriteScalarI64RoutesCloseoutRerun", "basis decision drift")
need(inputs.get("basis_selected_next_card") == token, "basis next drift")
need(inputs.get("basis_ready_for_rerun") == 1, "basis readiness drift")
need(inputs.get("write_scalar_i64_routes_closeout_basis_hash"), "missing basis hash")

closeout = fixture.get("materialized_closeout") or {}
need(closeout.get("surface_id") == "WriteScalarI64Routes", "surface drift")
need(closeout.get("closeout_kind") == "ScopedWriteSurfaceCloseout", "closeout kind drift")
need(closeout.get("scoped_direct_closeout_contract_count") == 3, "scoped count drift")
contracts = closeout.get("scoped_direct_closeout_contracts") or []
need({c.get("subsurface_id") for c in contracts} == {
    "PushSurfacePolicy",
    "SetSurfacePolicy/MapStoreI64",
    "SetSurfacePolicy/MapStoreAny",
}, "contract set drift")
delete = closeout.get("delete_surface_treatment") or {}
need(delete.get("closeout_treatment") == "RetiredUnconnectedMirrorLiveRustRoutePreserved", "delete treatment drift")
need(closeout.get("delete_surface_counts_as_hako_direct_closeout") is False, "delete counted as hako closeout")
need(closeout.get("delete_surface_live_rust_route_preserved") is True, "delete Rust route drift")
need(closeout.get("runtime_mutation_authority") is False, "runtime mutation drift")
need(closeout.get("publication_execution") is False, "publication execution drift")
need(closeout.get("route_authority_switch") is False, "route authority drift")

summary = fixture.get("summary") or {}
for key in [
    "write_scalar_i64_routes_closeout",
    "write_scalar_i64_routes_scoped_closeout_materialized",
    "delete_surface_hako_mirror_retired",
    "delete_surface_live_rust_route_preserved",
]:
    need(summary.get(key) == 1, f"summary positive drift: {key}")
need(summary.get("scoped_direct_closeout_contract_count") == 3, "summary scoped count drift")
for key in [
    "delete_surface_direct_closeout_materialized",
    "scalar_known_transport_axis_closeout",
    "runtime_mutation_authority",
    "publication_execution",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectScalarKnownTransportCloseoutRerunAfterWriteCloseout", "decision kind drift")
need(decision.get("reason_token") == "WriteScalarI64RoutesScopedCloseoutMaterialized", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "write_scalar_i64_routes_closeout",
    "write_scalar_i64_routes_scoped_closeout_materialized",
    "delete_surface_hako_mirror_retired",
    "delete_surface_live_rust_route_preserved",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
need(claims.get("scoped_direct_closeout_contract_count") == 3, "claim scoped count drift")
for key in [
    "delete_surface_direct_closeout_materialized",
    "scalar_known_transport_axis_closeout",
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
need(manifest_row.get("card", "").endswith("3363-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SCALAR-I64-ROUTES-CLOSEOUT-RERUN-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-write-scalar-i64-routes-closeout-rerun-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_scalar_i64_routes_closeout_rerun_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-scalar-i64-routes-closeout-rerun")
print("write_scalar_i64_routes_closeout=1")
print("write_scalar_i64_routes_scoped_closeout_materialized=1")
print("scoped_direct_closeout_contract_count=3")
print("delete_surface_hako_mirror_retired=1")
print("delete_surface_direct_closeout_materialized=0")
print("scalar_known_transport_axis_closeout=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY
