#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-write-boundary-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_set_mapstore_any_write_boundary_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2140-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-WRITE-BOUNDARY-BASIS-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
WRITE_SOURCE="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-write-boundary-basis"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" "$WRITE_SOURCE"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$WRITE_SOURCE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))
write_source = Path(sys.argv[5]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-WRITE-BOUNDARY-BASIS-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-RUST-ORACLE-PARITY-FIXTURE-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownWriteSetMapStoreAnyWriteBoundaryBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("boundary_selection_decision") == "SelectMapStoreAnyWriteBoundaryBasis", "selection drift")
need(inputs.get("boundary_selection_selected_next_card") == token, "selection next drift")
need(inputs.get("remaining_write_scoped_surface") == "SetSurfacePolicy/MapStoreAny", "remaining surface drift")

basis = fixture.get("boundary_basis") or {}
need(basis.get("basis_id") == "MapStoreAnyWriteBoundaryBasis", "basis id drift")
need(basis.get("route_kind") == "MapStoreAny", "route drift")
need(basis.get("surface_id") == "WriteScalarI64Routes", "surface drift")
need(basis.get("subsurface_id") == "SetSurfacePolicy/MapStoreAny", "subsurface drift")
need(basis.get("write_value_boundary") == "Any", "Any boundary drift")
need(basis.get("relationship_to_scalar_known") == "RemainingScopedWriteSurfaceInScalarKnownCloseoutChain", "relationship drift")
need(basis.get("mapstore_i64_already_scoped_closeout") is True, "MapStoreI64 closeout drift")
need(basis.get("mapstore_any_deferred_until_boundary") is True, "defer drift")
need(basis.get("runtime_mutation_authority") is False, "runtime mutation drift")
need(basis.get("publication_execution") is False, "publication execution drift")
need(basis.get("any_write_boundary_opened") is False, "opened drift")

for token_text in [
    "GenericMethodRouteKind::MapStoreAny",
    "GenericMethodRouteProof::SetSurfacePolicy",
    "GenericMethodValueDemand::WriteAny",
]:
    need(token_text in write_source, f"missing write source token: {token_text}")

rule = fixture.get("selection_rule") or {}
need(rule.get("basis_only") is True, "basis-only drift")
need(rule.get("hako_pilot_required_before_adoption") is True, "pilot rule drift")
need(rule.get("rust_oracle_fixture_required_next") is True, "oracle next drift")
for key in [
    "direct_closeout_materialization_allowed",
    "write_scalar_i64_routes_closeout_allowed",
    "scalar_known_transport_axis_closeout_allowed",
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
    "mapstore_any_write_boundary_basis",
    "any_write_boundary_declared",
    "set_surface_policy_remaining",
    "mapstore_i64_already_scoped_closeout",
    "mapstore_any_deferred_until_boundary",
    "basis_only",
    "hako_pilot_required_before_adoption",
]:
    need(summary.get(key) == 1, f"missing summary claim: {key}")
for key in [
    "any_write_boundary_opened",
    "mapstore_any_hako_pilot_selected",
    "mapstore_any_direct_closeout_materialized",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"forbidden summary drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectMapStoreAnyRustOracleParityFixture", "decision kind drift")
need(decision.get("reason_token") == "MapStoreAnyAnyWriteBoundaryBasisDeclared", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "mapstore_any_write_boundary_basis",
    "any_write_boundary_declared",
    "basis_only",
    "hako_pilot_required_before_adoption",
]:
    need(claims.get(key) == 1, f"missing claim: {key}")
for key in [
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
need(manifest_row.get("card", "").endswith("2140-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-WRITE-BOUNDARY-BASIS-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-write-boundary-basis-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_set_mapstore_any_write_boundary_basis_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-write-boundary-basis")
print("mapstore_any_write_boundary_basis=1")
print("any_write_boundary_declared=1")
print("any_write_boundary_opened=0")
print("mapstore_any_hako_pilot_selected=0")
print("write_scalar_i64_routes_closeout=0")
print("scalar_known_transport_axis_closeout=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY
