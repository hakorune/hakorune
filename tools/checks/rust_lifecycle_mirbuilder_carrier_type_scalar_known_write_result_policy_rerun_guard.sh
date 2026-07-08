#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-result-policy-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_result_policy_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2112-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-RESULT-POLICY-RERUN-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
RUST_BOUNDARY="$ROOT/src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$RUST_BOUNDARY" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))
rust_boundary = Path(sys.argv[5]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-RESULT-POLICY-RERUN-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SUBSURFACE-PRIORITY-BASIS-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownWriteResultPolicyRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("basis_selected_next_card") == token, "basis next drift")
need(inputs.get("basis_write_result_policy_ready") == 0, "basis readiness drift")
need(inputs.get("basis_write_direct_closeout_materialized") == 0, "basis materialized drift")

rule = fixture.get("selector_rule") or {}
need(rule.get("whole_direct_contract_requires_single_normalized_signature") is True, "whole rule drift")
need(rule.get("mixed_return_publication_forbids_whole_direct_contract") is True, "mixed rule drift")
need(rule.get("subsurface_selection_requires_priority_basis") is True, "priority rule drift")
need(rule.get("if_multiple_subsurfaces_require_priority_basis") is True, "multiple rule drift")
for key in [
    "manual_subsurface_selection",
    "route_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
]:
    need(rule.get(key) is False, f"forbidden selector drift: {key}")

policy = fixture.get("evaluated_policy") or {}
need(policy.get("policy_id") == "WriteResultPolicyV1", "policy id drift")
need(policy.get("target_surface_id") == "WriteScalarI64Routes", "surface drift")
need(policy.get("subsurface_count") == 3, "subsurface count drift")
need(policy.get("normalized_signature_count") == 3, "signature count drift")
need(policy.get("whole_direct_contract_allowed") is False, "whole contract drift")
need(set(policy.get("whole_direct_contract_blocked_by") or []) == {
    "MixedReturnPublicationNotStableDirectContract",
    "MultipleWriteSubsurfaceResultPublicationSignatures",
}, "whole blocker drift")

candidates = {row.get("subsurface_id"): row for row in fixture.get("priority_candidates") or []}
need(set(candidates) == {"PushSurfacePolicy", "DeleteSurfacePolicy", "SetSurfacePolicy"}, "candidate drift")
need(candidates["PushSurfacePolicy"].get("routes") == ["ArrayAppendAny"], "push route drift")
need(candidates["DeleteSurfacePolicy"].get("routes") == ["MapDeleteAny"], "delete route drift")
need(candidates["SetSurfacePolicy"].get("routes") == ["MapStoreI64", "MapStoreAny"], "set route drift")
for row in candidates.values():
    need(row.get("future_direct_contract_split_allowed") is True, "split allowed drift")
    need(row.get("selection_eligible_without_priority_basis") is False, "priority eligibility drift")
    need(row.get("blocked_by") == ["NoWriteSubsurfacePriorityBasis"], "candidate blocker drift")

summary = fixture.get("summary") or {}
for key in [
    "write_result_policy_rerun",
    "write_result_policy_basis_consumed",
    "write_surface_whole_direct_contract_rejected",
    "write_subsurface_split_required",
    "write_subsurface_priority_basis_selected",
]:
    need(summary.get(key) == 1, f"missing summary claim: {key}")
need(summary.get("write_subsurface_candidate_count") == 3, "candidate count drift")
need(summary.get("whole_direct_contract_candidate_count") == 0, "whole count drift")
for key in [
    "write_direct_closeout_materialized",
    "write_result_policy_ready",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"forbidden summary drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectWriteSubsurfacePriorityBasis", "decision kind drift")
need(decision.get("reason_token") == "MultipleWriteSubsurfacesRequirePriorityBasis", "reason drift")
need(decision.get("selected_subsurface") is None, "selected subsurface drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "write_result_policy_rerun",
    "write_result_policy_basis_consumed",
    "write_surface_whole_direct_contract_rejected",
    "write_subsurface_split_required",
    "write_subsurface_priority_basis_selected",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "write_direct_closeout_materialized",
    "write_result_policy_ready",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "direct_whole_write_contract_basis",
    "component_specific_direct_contract_materialized",
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
    "manual_subsurface_selection",
    "manual_axis_selection",
    "manual_carrier_selection",
    "route_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

for expected in [
    "ScalarKnownContractId::WriteResultScalarI64",
    "ScalarKnownSurfaceId::WriteScalarI64Routes",
    "ScalarKnownEffectClass::Mutate",
]:
    need(expected in rust_boundary, f"missing rust boundary token: {expected}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2112-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-RESULT-POLICY-RERUN-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-write-result-policy-rerun-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_result_policy_rerun_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-result-policy-rerun")
print("write_result_policy_rerun=1")
print("write_surface_whole_direct_contract_rejected=1")
print("write_subsurface_split_required=1")
print("write_subsurface_priority_basis_selected=1")
print("write_direct_closeout_materialized=0")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
