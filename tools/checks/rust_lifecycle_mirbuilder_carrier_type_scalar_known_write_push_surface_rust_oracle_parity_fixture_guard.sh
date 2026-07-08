#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-push-surface-rust-oracle-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_push_surface_rust_oracle_parity_fixture.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2115-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-PUSH-SURFACE-RUST-ORACLE-PARITY-FIXTURE-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
WRITE_SOURCE="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"
DESCRIPTORS="$ROOT/src/mir/generated/generic_method_route_descriptors.rs"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$WRITE_SOURCE" "$DESCRIPTORS" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))
write_source = Path(sys.argv[5]).read_text(encoding="utf-8")
descriptors = Path(sys.argv[6]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-PUSH-SURFACE-RUST-ORACLE-PARITY-FIXTURE-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-PUSH-SURFACE-HAKO-PARITY-PILOT-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownWritePushSurfaceRustOracleParityFixtureV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("priority_rerun_decision") == "KeepStopped", "previous decision drift")
need(inputs.get("recommended_consultation_topic") == "WriteSubsurfacePriorityProofAxis", "consult topic drift")
need("NoStableResultPublicationContractProof" in (inputs.get("push_candidate_blocked_by") or []), "push blocker drift")

oracle = fixture.get("oracle_fixture") or {}
need(oracle.get("fixture_id") == "WritePushSurfaceRustOracleV0", "fixture id drift")
need(oracle.get("row_count") == 1, "row count drift")
rows = oracle.get("rows") or []
need(len(rows) == 1, "row len drift")
row = rows[0]
expected = {
    "case_id": "array_append_any_push_surface",
    "subsurface_id": "PushSurfacePolicy",
    "route_kind": "ArrayAppendAny",
    "proof_or_policy_source": "PushSurfacePolicy",
    "core_method_op": "ArrayPush",
    "core_method_lowering_tier": "ColdFallback",
    "result_class": "ScalarI64Result",
    "return_shape": "ScalarI64",
    "value_demand": "WriteAny",
    "publication_policy": "NoPublication",
    "effect_class": "mutate",
    "mutation_class": "MutatesReceiverOrContainer",
    "hako_role": "classifier_policy_mirror_only",
}
for key, value in expected.items():
    need(row.get(key) == value, f"oracle row drift: {key}")

for expected_token in [
    "GenericMethodRouteKind::ArrayAppendAny",
    "GenericMethodRouteProof::PushSurfacePolicy",
    "CoreMethodOp::ArrayPush",
    "CoreMethodLoweringTier::ColdFallback",
    "GenericMethodReturnShape::ScalarI64",
    "GenericMethodValueDemand::WriteAny",
    "GenericMethodPublicationPolicy::NoPublication",
]:
    need(expected_token in write_source, f"missing write source token: {expected_token}")

for expected_token in [
    "GenericMethodRouteKind::ArrayAppendAny",
    "return_shape: Some(\"scalar_i64\")",
    "publication_policy: Some(\"no_publication\")",
    "effects: &[\"mutate.shape\"]",
]:
    need(expected_token in descriptors, f"missing descriptor token: {expected_token}")

boundary = fixture.get("mutation_boundary") or {}
need(boundary.get("mutate_effect_boundary_declared") is True, "mutation boundary drift")
need(boundary.get("runtime_mutation_authority") is False, "runtime mutation drift")
need(boundary.get("hako_implementation_mirrors_classifier_policy_decision") is True, "hako role drift")
need(boundary.get("receiver_or_container_mutation_observed_as_metadata") is True, "metadata boundary drift")

rule = fixture.get("selection_rule") or {}
need(rule.get("fixture_only") is True, "fixture-only drift")
need(rule.get("direct_closeout_materialization_allowed") is False, "direct closeout rule drift")
need(rule.get("hako_adoption_allowed") is False, "adoption rule drift")
need(rule.get("next_hako_parity_pilot_selected") is True, "next hako pilot drift")
for key in [
    "route_count_as_proof",
    "apparent_simplicity_as_proof",
    "accepted_read_contract_similarity_as_proof",
    "manual_subsurface_selection",
]:
    need(rule.get(key) is False, f"forbidden rule drift: {key}")

summary = fixture.get("summary") or {}
for key in [
    "write_push_surface_hako_implementation_candidate",
    "push_surface_policy_scope",
    "array_append_any_scope",
    "rust_oracle_fixture_defined",
    "stable_scalar_i64_result_observed",
    "no_publication_observed",
    "mutate_effect_boundary_declared",
    "hako_implementation_candidate",
    "basis_only_or_fixture_only",
    "next_hako_parity_pilot_selected",
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
need(decision.get("kind") == "SelectWritePushSurfaceHakoParityPilot", "decision kind drift")
need(decision.get("reason_token") == "WritePushSurfaceRustOracleFixtureDefined", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "write_push_surface_hako_implementation_candidate",
    "push_surface_policy_scope",
    "array_append_any_scope",
    "rust_oracle_fixture_defined",
    "stable_scalar_i64_result_observed",
    "no_publication_observed",
    "mutate_effect_boundary_declared",
    "hako_implementation_candidate",
    "basis_only_or_fixture_only",
    "next_hako_parity_pilot_selected",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "write_subsurface_selected",
    "write_direct_closeout_materialized",
    "write_result_policy_ready",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
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
    "manual_subsurface_selection",
    "route_count_as_proof",
    "apparent_simplicity_as_proof",
    "accepted_read_contract_similarity_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2115-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-PUSH-SURFACE-RUST-ORACLE-PARITY-FIXTURE-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-write-push-surface-rust-oracle-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_push_surface_rust_oracle_parity_fixture_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-push-surface-rust-oracle")
print("write_push_surface_hako_implementation_candidate=1")
print("push_surface_policy_scope=1")
print("array_append_any_scope=1")
print("rust_oracle_fixture_defined=1")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
