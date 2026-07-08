#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-delete-surface-typed-direct-closeout-contract-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_delete_surface_typed_direct_closeout_contract_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2128-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
WRITE_SOURCE="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"
HAKO_SOURCE="$ROOT/lang/src/compiler/lib/write_delete_surface_policy_classifier.hako"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-delete-surface-typed-direct-closeout-contract-basis"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" "$WRITE_SOURCE" "$HAKO_SOURCE"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$WRITE_SOURCE" "$HAKO_SOURCE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))
write_source = Path(sys.argv[5]).read_text(encoding="utf-8")
hako_source = Path(sys.argv[6]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-DIRECT-CLOSEOUT-RERUN-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownWriteDeleteSurfaceTypedDirectCloseoutContractBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("selected_write_subsurface") == "DeleteSurfacePolicy", "selected subsurface drift")
need(inputs.get("selected_next_card_from_rerun") == token, "input next drift")
need(inputs.get("adoption_decision") == "Adopt", "adoption drift")

contract = fixture.get("contract") or {}
expected = {
    "contract_id": "WriteDeleteSurfaceTypedDirectCloseoutContract",
    "source_kind": "TypedDirectCloseoutContract",
    "target_axis": "ScalarKnownTransportAxis",
    "surface_id": "WriteScalarI64Routes",
    "subsurface_id": "DeleteSurfacePolicy",
    "core_method_op": "MapDelete",
    "core_method_lowering_tier": "ColdFallback",
    "result_class": "ScalarI64Result",
    "return_shape": "ScalarI64",
    "value_demand": "WriteAny",
    "publication_policy": "NonePublication",
    "effect_class": "mutate",
    "mutation_class": "MutatesReceiverOrContainer",
    "hako_owner": "write_delete_surface_policy_classifier",
}
for key, value in expected.items():
    need(contract.get(key) == value, f"contract drift: {key}")
need(contract.get("route_kind_set") == ["MapDeleteAny"], "route set drift")
need(contract.get("proof_or_policy_source") == ["DeleteSurfacePolicy"], "policy source drift")
need(contract.get("runtime_mutation_authority") is False, "runtime mutation drift")
need(contract.get("publication_execution") is False, "publication execution drift")
need(contract.get("push_surface_policy_included") is False, "push inclusion drift")
need(contract.get("set_surface_policy_included") is False, "set inclusion drift")

for expected_token in [
    "GenericMethodRouteKind::MapDeleteAny",
    "GenericMethodRouteProof::DeleteSurfacePolicy",
    "CoreMethodOp::MapDelete",
    "CoreMethodLoweringTier::ColdFallback",
    "GenericMethodReturnShape::ScalarI64",
    "GenericMethodValueDemand::WriteAny",
    "None,",
]:
    need(expected_token in write_source, f"missing write source token: {expected_token}")

for expected_token in [
    "WriteDeleteSurfacePolicyClassifierBox",
    "MapDeleteAny",
    "DeleteSurfacePolicy",
    "NonePublication",
    "MutatesReceiverOrContainer",
]:
    need(expected_token in hako_source, f"missing hako source token: {expected_token}")

rule = fixture.get("selection_rule") or {}
need(rule.get("basis_only") is True, "basis-only drift")
need(rule.get("contract_materialization_requires_rerun") is True, "rerun rule drift")
need(rule.get("whole_write_surface_closeout_forbidden_at_basis") is True, "whole closeout rule drift")
need(rule.get("axis_closeout_forbidden_at_basis") is True, "axis closeout rule drift")
for key in [
    "source_path_as_authority",
    "owner_name_as_proof",
    "row_count_as_proof",
    "route_count_as_proof",
    "route_membership_alone_as_proof",
]:
    need(rule.get(key) is False, f"forbidden rule drift: {key}")

summary = fixture.get("summary") or {}
need(summary.get("write_delete_surface_typed_direct_closeout_contract_basis") == 1, "summary basis drift")
need(summary.get("write_delete_route_count") == 1, "summary route count drift")
for key in [
    "direct_contract_materialized",
    "write_delete_direct_closeout_ready",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "runtime_mutation_authority",
    "publication_execution",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectWriteDeleteSurfaceDirectCloseoutRerun", "decision kind drift")
need(decision.get("reason_token") == "WriteDeleteSurfaceTypedDirectCloseoutContractBasisDefined", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in ["write_delete_surface_typed_direct_closeout_contract_basis", "basis_only"]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "direct_contract_materialized",
    "write_delete_direct_closeout_ready",
    "write_scalar_i64_routes_closeout",
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
need(manifest_row.get("card", "").endswith("2128-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-write-delete-surface-typed-direct-closeout-contract-basis-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_delete_surface_typed_direct_closeout_contract_basis_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-delete-surface-typed-direct-closeout-contract-basis")
print("write_delete_surface_typed_direct_closeout_contract_basis=1")
print("write_delete_route_count=1")
print("direct_contract_materialized=0")
print("runtime_mutation_authority=0")
print("publication_execution=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY
