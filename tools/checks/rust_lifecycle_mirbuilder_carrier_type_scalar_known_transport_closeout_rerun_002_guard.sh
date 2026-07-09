#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-transport-closeout-rerun-002-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_transport_closeout_rerun_002.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3364-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-RERUN-002.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
RUST_BOUNDARY="$ROOT/src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs"
SHADOW="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-carrier-type-scalar-known-transport-closeout-rerun-002"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" "$RUST_BOUNDARY" "$SHADOW"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$RUST_BOUNDARY" "$SHADOW" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))
rust_boundary = Path(sys.argv[5]).read_text(encoding="utf-8")
shadow = Path(sys.argv[6]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-RERUN-002"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-BASIS-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownTransportCloseoutRerun002V1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("write_selected_next_card") == token, "write closeout next drift")
for key in [
    "mapload_basis_hash",
    "string_search_rerun_hash",
    "collection_len_rerun_hash",
    "write_scalar_i64_routes_closeout_rerun_hash",
]:
    need(inputs.get(key), f"missing input hash: {key}")

surfaces = fixture.get("accepted_scalar_known_surfaces") or []
need({row.get("surface_id") for row in surfaces} == {
    "MapLoadScalarI64Routes",
    "StringScalarI64Routes",
    "CollectionScalarI64Routes",
    "WriteScalarI64Routes",
}, "accepted surface set drift")
need({row.get("contract_id") for row in surfaces} == {
    "MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract",
    "StringSearchScalarI64TypedDirectCloseoutContract",
    "CollectionLenScalarI64TypedDirectCloseoutContract",
    "WriteScalarI64RoutesScopedCloseout",
}, "accepted contract set drift")

expect = fixture.get("rust_boundary_expectation") or {}
need(expect.get("accepted_status") == "AcceptedScopedCloseout", "accepted status drift")
need(expect.get("accepted_surface_count") == 4, "accepted count drift")
need(expect.get("candidate_surface_count") == 0, "candidate count drift")
need(expect.get("write_contract_id") == "WriteScalarI64RoutesScopedCloseout", "write contract drift")

need('assert_eq!(accepted.len(), 4);' in rust_boundary, "Rust accepted count test missing")
need('assert!(candidates.is_empty());' in rust_boundary, "Rust candidate empty test missing")
need('WriteScalarI64RoutesScopedCloseout' in rust_boundary, "Rust write closeout id missing")
need('status: ScalarKnownContractStatus::AcceptedScopedCloseout' in rust_boundary, "Rust accepted status missing")
need('accepted_contract_count >= 4' in shadow, "shadow accepted count not refreshed")
need('WriteScalarI64RoutesScopedCloseout' in shadow, "shadow write closeout id missing")
need('candidate_scalar_known_surfaces' not in shadow, "shadow still consumes candidate surface")

summary = fixture.get("summary") or {}
for key in [
    "scalar_known_transport_axis_closeout",
    "write_scalar_i64_routes_closeout",
]:
    need(summary.get(key) == 1, f"summary positive drift: {key}")
need(summary.get("accepted_scalar_known_surface_count") == 4, "summary accepted count drift")
need(summary.get("uncovered_scalar_known_surface_count") == 0, "summary uncovered drift")
for key in [
    "fastpath_connected_closeout",
    "hako_runtime_route_authority",
    "rust_fastpath_rewired",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectFastpathConnectedCloseoutBasis", "decision kind drift")
need(decision.get("reason_token") == "ScalarKnownTransportAxisScopedCloseoutMaterializedButFastpathConnectedCloseoutStillOpen", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "scalar_known_transport_axis_closeout",
    "write_scalar_i64_routes_closeout",
    "rust_boundary_status_refreshed",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
need(claims.get("accepted_scalar_known_surface_count") == 4, "claim accepted count drift")
need(claims.get("uncovered_scalar_known_surface_count") == 0, "claim uncovered drift")
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
need(manifest_row.get("card", "").endswith("3364-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-RERUN-002.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-transport-closeout-rerun-002-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_transport_closeout_rerun_002_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-transport-closeout-rerun-002")
print("scalar_known_transport_axis_closeout=1")
print("accepted_scalar_known_surface_count=4")
print("uncovered_scalar_known_surface_count=0")
print("fastpath_connected_closeout=0")
print("hako_runtime_route_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY

cargo test -q --lib scalar_known
