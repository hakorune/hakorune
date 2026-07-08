#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-rust-oracle-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_set_mapstore_any_rust_oracle_parity_fixture.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2141-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-RUST-ORACLE-PARITY-FIXTURE-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
WRITE_SOURCE="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-rust-oracle-parity-fixture"

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


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-RUST-ORACLE-PARITY-FIXTURE-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-HAKO-PARITY-PILOT-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownWriteSetMapStoreAnyRustOracleParityFixtureV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("basis_decision") == "SelectMapStoreAnyRustOracleParityFixture", "basis decision drift")
need(inputs.get("basis_selected_next_card") == token, "basis next drift")

rows = fixture.get("oracle_fixture", {}).get("rows") or []
need(len(rows) == 1, "row count drift")
row = rows[0]
expected = {
    "case_id": "map_store_any_set_surface",
    "subsurface_id": "SetSurfacePolicy/MapStoreAny",
    "route_kind": "MapStoreAny",
    "proof_or_policy_source": "SetSurfacePolicy",
    "core_method_op": "MapSet",
    "core_method_lowering_tier": "ColdFallback",
    "result_class": "NoneResult",
    "return_shape": "None",
    "value_demand": "WriteAny",
    "write_value_boundary": "Any",
    "publication_policy": "NonePublication",
    "effect_class": "mutate",
    "mutation_class": "MutatesReceiverOrContainer",
    "any_write_boundary": "DeclaredMetadataOnly",
    "hako_role": "classifier_policy_mirror_only",
}
for key, value in expected.items():
    need(row.get(key) == value, f"oracle row drift: {key}")

for token_text in [
    "GenericMethodRouteKind::MapStoreAny",
    "GenericMethodRouteProof::SetSurfacePolicy",
    "GenericMethodValueDemand::WriteAny",
]:
    need(token_text in write_source, f"missing write source token: {token_text}")

metadata = fixture.get("metadata_boundary") or {}
need(metadata.get("any_write_boundary_declared") is True, "declared drift")
need(metadata.get("any_write_boundary_opened") is False, "opened drift")
need(metadata.get("runtime_mutation_authority") is False, "runtime mutation drift")
need(metadata.get("publication_execution") is False, "publication execution drift")

summary = fixture.get("summary") or {}
for key in [
    "write_set_mapstore_any_hako_implementation_candidate",
    "set_surface_policy_scope",
    "mapstore_any_scope",
    "any_write_boundary_declared",
    "rust_oracle_fixture_defined",
    "next_hako_parity_pilot_selected",
]:
    need(summary.get(key) == 1, f"missing summary claim: {key}")
for key in [
    "any_write_boundary_opened",
    "write_direct_closeout_materialized",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "runtime_mutation_authority",
    "publication_execution",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"forbidden summary drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectWriteSetMapStoreAnyHakoParityPilot", "decision kind drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "write_set_mapstore_any_hako_implementation_candidate",
    "any_write_boundary_declared",
    "rust_oracle_fixture_defined",
    "next_hako_parity_pilot_selected",
]:
    need(claims.get(key) == 1, f"missing claim: {key}")
for key in [
    "any_write_boundary_opened",
    "write_direct_closeout_materialized",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "hako_adoption",
    "source_selfhost_claim",
    "new_route_authority",
    "behavior_change",
    "runtime_mutation_authority",
    "publication_execution",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "manual_subsurface_selection",
    "route_count_as_proof",
    "apparent_simplicity_as_proof",
    "accepted_read_contract_similarity_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2141-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-RUST-ORACLE-PARITY-FIXTURE-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-rust-oracle-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_set_mapstore_any_rust_oracle_parity_fixture_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-rust-oracle")
print("write_set_mapstore_any_hako_implementation_candidate=1")
print("rust_oracle_fixture_defined=1")
print("any_write_boundary_declared=1")
print("any_write_boundary_opened=0")
print("write_direct_closeout_materialized=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY
