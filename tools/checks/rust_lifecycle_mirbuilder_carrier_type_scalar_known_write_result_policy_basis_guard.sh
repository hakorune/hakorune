#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-result-policy-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_result_policy_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2111-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-RESULT-POLICY-BASIS-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
RUST_BOUNDARY="$ROOT/src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs"
WRITE_SOURCE="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"
DESCRIPTORS="$ROOT/src/mir/generated/generic_method_route_descriptors.rs"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$RUST_BOUNDARY" "$WRITE_SOURCE" "$DESCRIPTORS" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))
rust_boundary = Path(sys.argv[5]).read_text(encoding="utf-8")
write_source = Path(sys.argv[6]).read_text(encoding="utf-8")
descriptors = Path(sys.argv[7]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-RESULT-POLICY-BASIS-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-RESULT-POLICY-RERUN-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownWriteResultPolicyBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("accepted_scoped_closeout_count_before_basis") == 3, "accepted count drift")
need(inputs.get("remaining_candidate_surface_id") == "WriteScalarI64Routes", "remaining surface drift")
need(inputs.get("write_blocker") == "WriteResultPolicyRequiredBeforeDirectCloseout", "write blocker drift")

policy = fixture.get("policy") or {}
need(policy.get("policy_id") == "WriteResultPolicyV1", "policy id drift")
need(policy.get("target_surface_id") == "WriteScalarI64Routes", "target surface drift")
need(policy.get("basis_only") is True, "basis-only drift")
need(set(policy.get("route_kind_set") or []) == {
    "ArrayAppendAny",
    "MapDeleteAny",
    "MapStoreI64",
    "MapStoreAny",
}, "write route drift")

sub = {row.get("subsurface_id"): row for row in policy.get("sub_surfaces") or []}
need(set(sub) == {"PushSurfacePolicy", "DeleteSurfacePolicy", "SetSurfacePolicy"}, "subsurface drift")
need(sub["PushSurfacePolicy"].get("routes") == ["ArrayAppendAny"], "push route drift")
need(sub["PushSurfacePolicy"].get("normalized_result_class") == "ScalarI64Result", "push result drift")
need(sub["PushSurfacePolicy"].get("publication_class") == "NoPublication", "push publication drift")
need(sub["DeleteSurfacePolicy"].get("routes") == ["MapDeleteAny"], "delete route drift")
need(sub["DeleteSurfacePolicy"].get("publication_class") == "NonePublication", "delete publication drift")
need(sub["SetSurfacePolicy"].get("routes") == ["MapStoreI64", "MapStoreAny"], "set route drift")
need(sub["SetSurfacePolicy"].get("normalized_result_class") == "NoneResult", "set result drift")
set_subcases = {row.get("route_kind"): row for row in sub["SetSurfacePolicy"].get("subcases") or []}
need(set_subcases["MapStoreI64"].get("typed_scalar_write") == 1, "map store i64 type drift")
need(set_subcases["MapStoreAny"].get("typed_scalar_write") == 0, "map store any type drift")

mixed = policy.get("mixed_return_publication_decomposition") or {}
need(mixed.get("observed_return_shape") == "ScalarI64OrNoneMixed", "mixed return drift")
need(mixed.get("observed_publication_policy") == "MixedNoPublicationAndNone", "mixed publication drift")
need(mixed.get("mixed_state_is_not_direct_closeout_contract") is True, "mixed accepted drift")
effect = policy.get("effect_boundary") or {}
need(effect.get("effect_class") == "mutate", "effect class drift")
need(effect.get("direct_closeout_requires_rerun") is True, "rerun effect drift")

for expected in [
    "ScalarKnownSurfaceId::WriteScalarI64Routes",
    "ScalarKnownContractId::WriteResultScalarI64",
    "ScalarKnownEffectClass::Mutate",
    "GenericMethodRouteKind::ArrayAppendAny",
    "GenericMethodRouteKind::MapDeleteAny",
    "GenericMethodRouteKind::MapStoreI64",
    "GenericMethodRouteKind::MapStoreAny",
    "GenericMethodRouteProof::PushSurfacePolicy",
    "GenericMethodRouteProof::DeleteSurfacePolicy",
    "GenericMethodRouteProof::SetSurfacePolicy",
    "GenericMethodValueDemand::WriteAny",
]:
    need(expected in rust_boundary or expected in write_source, f"missing rust write token: {expected}")

for expected in [
    "GenericMethodRouteKind::ArrayAppendAny",
    "GenericMethodRouteKind::MapDeleteAny",
    "GenericMethodRouteKind::MapStoreI64",
    "GenericMethodRouteKind::MapStoreAny",
    "value_demand: \"write_any\"",
    "effects: &[\"mutate.",
]:
    need(expected in descriptors, f"missing descriptor token: {expected}")

rule = fixture.get("selection_rule") or {}
need(rule.get("basis_only") is True, "rule basis drift")
need(rule.get("direct_closeout_materialization_allowed") is False, "direct closeout rule drift")
need(rule.get("rerun_required_before_direct_closeout") is True, "rerun rule drift")
need(rule.get("subsurface_classification_allowed_in_basis") is True, "subsurface rule drift")
need(rule.get("write_surface_direct_closeout_forbidden_at_basis") is True, "write closeout rule drift")
need(rule.get("axis_closeout_forbidden_at_basis") is True, "axis rule drift")
for key in [
    "source_path_as_authority",
    "owner_name_as_proof",
    "row_count_as_proof",
    "route_membership_alone_as_proof",
]:
    need(rule.get(key) is False, f"forbidden rule drift: {key}")

summary = fixture.get("summary") or {}
for key in [
    "write_result_policy_basis",
    "write_surface_policy_boundary_defined",
    "mutate_effect_boundary_declared",
    "write_subsurface_classification_defined",
    "push_surface_policy_defined",
    "delete_surface_policy_defined",
    "set_surface_policy_defined",
    "mixed_return_publication_policy_declared",
    "basis_only",
    "rerun_required_before_direct_closeout",
]:
    need(summary.get(key) == 1, f"missing summary claim: {key}")
for key in [
    "write_direct_closeout_materialized",
    "write_result_policy_ready",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"forbidden summary drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectWriteResultPolicyRerun", "decision kind drift")
need(decision.get("reason_token") == "WriteResultPolicyBasisDefined", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "write_result_policy_basis",
    "write_surface_policy_boundary_defined",
    "mutate_effect_boundary_declared",
    "write_subsurface_classification_defined",
    "push_surface_policy_defined",
    "delete_surface_policy_defined",
    "set_surface_policy_defined",
    "mixed_return_publication_policy_declared",
    "basis_only",
    "rerun_required_before_direct_closeout",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "write_direct_closeout_materialized",
    "write_result_policy_ready",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "component_specific_card_selection",
    "concrete_carrier_type_axis_selection",
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
    "manual_axis_selection",
    "manual_carrier_selection",
    "row_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2111-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-RESULT-POLICY-BASIS-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-write-result-policy-basis-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_result_policy_basis_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-result-policy-basis")
print("write_result_policy_basis=1")
print("write_subsurface_classification_defined=1")
print("write_direct_closeout_materialized=0")
print("write_result_policy_ready=0")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
