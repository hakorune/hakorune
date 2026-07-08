#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-rust-typed-direct-closeout-contract-boundary-refactor-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_rust_typed_direct_closeout_contract_boundary_refactor.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2107-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-RUST-TYPED-DIRECT-CLOSEOUT-CONTRACT-BOUNDARY-REFACTOR-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
RUST_BOUNDARY="$ROOT/src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs"
RUST_MOD="$ROOT/src/mir/generic_method_route_plan.rs"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$RUST_BOUNDARY" "$RUST_MOD" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))
rust_boundary = Path(sys.argv[5]).read_text(encoding="utf-8")
rust_mod = Path(sys.argv[6]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-RUST-TYPED-DIRECT-CLOSEOUT-CONTRACT-BOUNDARY-REFACTOR-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-REMAINING-SURFACE-BOUNDARY-INVENTORY-RERUN-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownRustTypedDirectCloseoutContractBoundaryRefactorV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

boundary = fixture.get("rust_boundary") or {}
need(boundary.get("struct_name") == "ScalarKnownTypedDirectCloseoutContract", "struct drift")
need(boundary.get("status_enum") == "ScalarKnownContractStatus", "status enum drift")
need(boundary.get("accepted_status") == "AcceptedScopedCloseout", "accepted status drift")
need(boundary.get("candidate_status") == "CandidateNeedsPolicy", "candidate status drift")

accepted = fixture.get("accepted_contracts") or []
need(accepted == [
    "MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract",
    "StringSearchScalarI64TypedDirectCloseoutContract",
], "accepted contract drift")
remaining = fixture.get("remaining_candidate_surfaces") or []
need(remaining == ["CollectionScalarI64Routes", "WriteScalarI64Routes"], "candidate surface drift")

for expected in [
    "struct ScalarKnownTypedDirectCloseoutContract",
    "enum ScalarKnownContractStatus",
    "AcceptedScopedCloseout",
    "CandidateNeedsPolicy",
    "MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract",
    "StringSearchScalarI64TypedDirectCloseoutContract",
    "CollectionLenScalarI64TypedDirectCloseoutContract",
    "WriteResultScalarI64ClassificationOnly",
    "SCALAR_KNOWN_TYPED_DIRECT_CLOSEOUT_CONTRACTS",
    "accepted_scalar_known_contracts",
    "candidate_scalar_known_surfaces",
]:
    need(expected in rust_boundary, f"missing rust boundary token: {expected}")
need("mod scalar_known_typed_direct_closeout_contract;" in rust_mod, "module not registered")

preservation = fixture.get("behavior_preservation") or {}
for key in [
    "route_selection_changed",
    "route_kind_semantics_changed",
    "return_shape_semantics_changed",
    "value_demand_semantics_changed",
    "publication_policy_semantics_changed",
    "effect_semantics_changed",
    "lowering_path_changed",
]:
    need(preservation.get(key) is False, f"behavior preservation drift: {key}")

summary = fixture.get("summary") or {}
need(summary.get("rust_contract_boundary_refactor") == 1, "summary refactor drift")
need(summary.get("scalar_known_typed_direct_closeout_contract_boundary") == 1, "summary boundary drift")
need(summary.get("accepted_scoped_closeout_contract_count") == 2, "accepted count drift")
need(summary.get("remaining_candidate_surface_count") == 2, "candidate count drift")
need(summary.get("behavior_preserved") == 1, "behavior preserved drift")
need(summary.get("existing_rust_owner_evidence_repackaged") == 1, "repackage drift")
for key in [
    "direct_contract_selection",
    "collection_direct_closeout_ready",
    "write_direct_closeout_ready",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"summary forbidden claim drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectRemainingSurfaceBoundaryInventoryRerunAfterRustBoundaryRefactor", "decision kind drift")
need(decision.get("reason_token") == "ScalarKnownTypedDirectCloseoutContractBoundaryRepackaged", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "rust_contract_boundary_refactor",
    "scalar_known_typed_direct_closeout_contract_boundary",
    "behavior_preserved",
    "existing_rust_owner_evidence_repackaged",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "direct_contract_selection",
    "collection_direct_closeout_ready",
    "write_direct_closeout_ready",
    "scalar_known_transport_axis_closeout",
    "hako_adoption",
    "source_selfhost_claim",
    "new_route_authority",
    "behavior_change",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "new_python_semantic_projector",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2107-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-RUST-TYPED-DIRECT-CLOSEOUT-CONTRACT-BOUNDARY-REFACTOR-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-rust-typed-direct-closeout-contract-boundary-refactor-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_rust_typed_direct_closeout_contract_boundary_refactor_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-rust-typed-direct-closeout-contract-boundary-refactor-v0")
print("token=" + token)
print("rust_contract_boundary_refactor=1")
print("scalar_known_typed_direct_closeout_contract_boundary=1")
print("accepted_scoped_closeout_contract_count=2")
print("accepted_contracts=" + ",".join(accepted))
print("remaining_candidate_surface_count=2")
print("remaining_candidate_surfaces=" + ",".join(remaining))
print("behavior_preserved=1")
print("route_kind_semantics_changed=0")
print("return_shape_semantics_changed=0")
print("publication_policy_semantics_changed=0")
print("effect_semantics_changed=0")
print("direct_contract_selection=0")
print("scalar_known_transport_axis_closeout=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
