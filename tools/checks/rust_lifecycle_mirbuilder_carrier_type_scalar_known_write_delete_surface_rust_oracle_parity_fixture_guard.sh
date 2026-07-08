#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-delete-surface-rust-oracle-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_delete_surface_rust_oracle_parity_fixture.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2123-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-RUST-ORACLE-PARITY-FIXTURE-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
WRITE_SOURCE="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"
DESCRIPTORS="$ROOT/src/mir/generated/generic_method_route_descriptors.rs"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-delete-surface-rust-oracle"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" "$WRITE_SOURCE" "$DESCRIPTORS"

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


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-RUST-ORACLE-PARITY-FIXTURE-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-HAKO-PARITY-PILOT-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownWriteDeleteSurfaceRustOracleParityFixtureV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("remaining_rerun_decision") == "KeepStopped", "previous decision drift")
need(inputs.get("recommended_consultation_topic") == "WriteRemainingSubsurfaceHakoPilotSelection", "consult topic drift")
need("NoConsultationApprovedNextWritePilotProofAxis" in (inputs.get("delete_candidate_blocked_by") or []), "delete blocker drift")
need(inputs.get("accepted_scoped_closeout_count") == 4, "accepted closeout drift")

axis = fixture.get("proof_axis") or {}
need(axis.get("name") == "PriorHakoAdoptedWriteSubsurfaceMinimalSemanticDelta", "axis drift")
need(axis.get("prior_subsurface") == "PushSurfacePolicy", "prior subsurface drift")
need(axis.get("selected_subsurface") == "DeleteSurfacePolicy", "selected subsurface drift")
need(axis.get("stable_scalar_i64_result_preserved") is True, "result preservation drift")
need(axis.get("mutate_effect_metadata_boundary_reused") is True, "mutation boundary drift")
need(axis.get("new_policy_dimension") == "NonePublication", "new policy dimension drift")
need(axis.get("typed_non_typed_write_split") is False, "split drift")
for key in [
    "route_count_as_proof",
    "apparent_simplicity_as_proof",
    "accepted_read_contract_similarity_as_proof",
    "manual_subsurface_selection",
]:
    need(axis.get(key) is False, f"forbidden axis drift: {key}")

oracle = fixture.get("oracle_fixture") or {}
need(oracle.get("fixture_id") == "WriteDeleteSurfaceRustOracleV0", "fixture id drift")
need(oracle.get("row_count") == 1, "row count drift")
rows = oracle.get("rows") or []
need(len(rows) == 1, "row len drift")
row = rows[0]
expected = {
    "case_id": "map_delete_any_delete_surface",
    "subsurface_id": "DeleteSurfacePolicy",
    "route_kind": "MapDeleteAny",
    "proof_or_policy_source": "DeleteSurfacePolicy",
    "core_method_op": "MapDelete",
    "core_method_lowering_tier": "ColdFallback",
    "result_class": "ScalarI64Result",
    "return_shape": "ScalarI64",
    "value_demand": "WriteAny",
    "publication_policy": "NonePublication",
    "effect_class": "mutate",
    "mutation_class": "MutatesReceiverOrContainer",
    "hako_role": "classifier_policy_mirror_only",
}
for key, value in expected.items():
    need(row.get(key) == value, f"oracle row drift: {key}")

for expected_token in [
    "GenericMethodRouteKind::MapDeleteAny",
    "GenericMethodRouteProof::DeleteSurfacePolicy",
    "CoreMethodOp::MapDelete",
    "CoreMethodLoweringTier::ColdFallback",
    "GenericMethodReturnShape::ScalarI64",
    "GenericMethodValueDemand::WriteAny",
]:
    need(expected_token in write_source, f"missing write source token: {expected_token}")

for expected_token in [
    "GenericMethodRouteKind::MapDeleteAny",
    "return_shape: Some(\"scalar_i64\")",
    "publication_policy: None",
    "effects: &[\"mutate.shape\"]",
]:
    need(expected_token in descriptors, f"missing descriptor token: {expected_token}")

boundary = fixture.get("metadata_boundary") or {}
need(boundary.get("none_publication_metadata_declared") is True, "none publication drift")
need(boundary.get("publication_execution") is False, "publication execution drift")
need(boundary.get("mutate_effect_boundary_reused") is True, "mutation metadata drift")
need(boundary.get("runtime_mutation_authority") is False, "runtime mutation drift")

rule = fixture.get("selection_rule") or {}
need(rule.get("fixture_only") is True, "fixture-only drift")
need(rule.get("direct_closeout_materialization_allowed") is False, "direct closeout rule drift")
need(rule.get("hako_adoption_allowed") is False, "adoption rule drift")
need(rule.get("next_hako_parity_pilot_selected") is True, "next hako pilot drift")

summary = fixture.get("summary") or {}
for key in [
    "write_delete_surface_hako_implementation_candidate",
    "delete_surface_policy_scope",
    "map_delete_any_scope",
    "prior_hako_adopted_write_subsurface_minimal_semantic_delta",
    "stable_scalar_i64_result_preserved",
    "none_publication_metadata_declared",
    "mutate_effect_metadata_boundary_reused",
    "rust_oracle_fixture_defined",
    "next_hako_parity_pilot_selected",
]:
    need(summary.get(key) == 1, f"missing summary claim: {key}")
for key in [
    "typed_non_typed_write_split",
    "write_direct_closeout_materialized",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "runtime_mutation_authority",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"forbidden summary drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectWriteDeleteSurfaceHakoParityPilot", "decision kind drift")
need(decision.get("reason_token") == "PriorHakoAdoptedWriteSubsurfaceMinimalSemanticDelta", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "write_delete_surface_hako_implementation_candidate",
    "delete_surface_policy_scope",
    "map_delete_any_scope",
    "prior_hako_adopted_write_subsurface_minimal_semantic_delta",
    "stable_scalar_i64_result_preserved",
    "none_publication_metadata_declared",
    "mutate_effect_metadata_boundary_reused",
    "rust_oracle_fixture_defined",
    "next_hako_parity_pilot_selected",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "typed_non_typed_write_split",
    "write_subsurface_selected_for_closeout",
    "write_direct_closeout_materialized",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "hako_adoption",
    "source_selfhost_claim",
    "new_route_authority",
    "behavior_change",
    "runtime_mutation_authority",
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
need(manifest_row.get("card", "").endswith("2123-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-RUST-ORACLE-PARITY-FIXTURE-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-write-delete-surface-rust-oracle-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_delete_surface_rust_oracle_parity_fixture_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-delete-surface-rust-oracle")
print("write_delete_surface_hako_implementation_candidate=1")
print("delete_surface_policy_scope=1")
print("map_delete_any_scope=1")
print("prior_hako_adopted_write_subsurface_minimal_semantic_delta=1")
print("none_publication_metadata_declared=1")
print("typed_non_typed_write_split=0")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
