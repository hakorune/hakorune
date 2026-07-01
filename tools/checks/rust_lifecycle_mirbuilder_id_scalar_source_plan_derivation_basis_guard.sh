#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-source-plan-derivation-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_source_plan_derivation_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2023-MIRBUILDER-ID-SCALAR-SOURCE-PLAN-DERIVATION-BASIS-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-ID-SCALAR-SOURCE-PLAN-DERIVATION-BASIS-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
next_card = "MIRBUILDER-ID-SCALAR-SOURCE-SURFACE-INVENTORY-001"

need(fixture.get("kind") == "MirBuilderIdScalarSourcePlanDerivationBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need((fixture.get("input_state") or {}).get("current_blocker") == design_stop, "blocker drift")

previous = fixture.get("previous_state") or {}
need(previous.get("previous_reason_token") == "NoIdScalarSourcePlanAndRecipeDerivabilityCandidate", "previous reason drift")
for reason in [
    "SourcePlanDerivabilityNotProven",
    "BehaviorRecipeDerivabilityNotProven",
    "DescriptorOnlyIsNotSourcePlanAndRecipe",
]:
    need(reason in (previous.get("blocked_by") or []), f"missing previous blocker {reason}")

basis = fixture.get("basis") or {}
need(basis.get("directability_only_is_source_plan") is False, "directability-only basis drift")
need(basis.get("descriptor_only_is_source_plan") is False, "descriptor-only basis drift")
need(basis.get("source_plan_derivation_allowed") is True, "basis not allowed")
need(basis.get("source_plan_derivation_requires_machine_derived_surface_set") is True, "surface-set requirement drift")
need(basis.get("source_plan_derivation_requires_operation_vocabulary") is True, "operation vocabulary requirement drift")
need(basis.get("source_plan_derivation_requires_behavior_recipe") is True, "behavior recipe requirement drift")

required = set(fixture.get("source_plan_derivable_requires") or [])
for req in [
    "owner_edge_confidence_exact_or_fixture",
    "owner_scope_bounded",
    "required_source_surfaces_complete",
    "operation_vocabulary_complete",
    "behavior_recipe_effect_coverage_complete",
    "nominal_id_domain_isolation_preserved",
    "id_domain_boundary_declared",
    "state_mutation_frame_declared",
    "error_semantics_declared",
    "deterministic_order_declared",
    "verifier_input_contract_declared",
    "no_borrow_policy_gap",
    "no_carrier_type_transport_gap",
    "no_runtime_fallback",
    "no_new_backend_route",
    "no_new_abi",
    "no_new_python_semantic_projector",
]:
    need(req in required, f"missing derivation basis requirement {req}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "PolicyDefined", "decision kind drift")
need(decision.get("reason_token") == "IdScalarSourcePlanDerivationBasisDefined", "decision reason drift")
need(decision.get("selected_next_card") == next_card, "next card drift")

claims = fixture.get("claims") or {}
need(claims.get("source_plan_derivation_basis_defined") == 1, "basis claim drift")
for key in [
    "directability_only_is_source_plan",
    "descriptor_only_is_source_plan",
    "source_plan_implied_by_descriptor",
    "source_plan_implied_by_directability",
    "behavior_recipe_implied_by_descriptor",
    "behavior_recipe_implied_by_directability",
    "verifier_result_implied_by_source_plan",
    "derived_artifact_seed_draft_implied_by_verifier",
    "raw_i64_interchangeability",
    "nominal_id_erasure",
    "id_sentinel_semantics_inferred",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "manual_owner_selection",
    "cluster_size_as_proof",
    "directable_row_count_as_proof",
    "lexical_order_as_seed_selection_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "generated_artifact_as_native_edit_authority",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")
for needle in [
    token,
    "source_plan_derivation_basis_defined = 1",
    "selected_next_card = MIRBUILDER-ID-SCALAR-SOURCE-SURFACE-INVENTORY-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-source-plan-derivation-basis")
print("source_plan_derivation_basis_defined=1")
print("selected_next_card=MIRBUILDER-ID-SCALAR-SOURCE-SURFACE-INVENTORY-001")
print("source_selfhost_claim=0")
print("summary=ok")
PY
