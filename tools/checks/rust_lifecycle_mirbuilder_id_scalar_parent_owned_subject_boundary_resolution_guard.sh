#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-parent-owned-subject-boundary-resolution-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_parent_owned_subject_boundary_resolution.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2055-MIRBUILDER-ID-SCALAR-PARENT-OWNED-SUBJECT-BOUNDARY-RESOLUTION-001.md"
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


token = "MIRBUILDER-ID-SCALAR-PARENT-OWNED-SUBJECT-BOUNDARY-RESOLUTION-001"
next_card = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-007"
reason = "ContextRegistryRemainsParentOwnedNotSeedEligible"

need(fixture.get("kind") == "MirBuilderIdScalarParentOwnedSubjectBoundaryResolutionV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

candidate = fixture.get("input_candidate") or {}
need(candidate.get("owner_edge_id") == "mirbuilder::context_registry", "bad owner")
need(candidate.get("projection_disposition") == "KeepParentOwner", "projection disposition drift")
need(candidate.get("projection_surface_selected") is False, "projection surface drift")
need(candidate.get("current_reason_token") == "ContextRegistryPluginSignatureConstructorIsParentOwned", "reason drift")
need(candidate.get("remaining_owner_count_as_proof") is False, "remaining owner count proof drift")

tests = fixture.get("subject_boundary_tests") or {}
for key in [
    "keep_parent_owner_as_standalone_proof",
    "source_symbol_as_proof",
    "source_path_as_authority",
    "shape_name_as_semantic_policy",
    "route_membership_alone_as_proof",
]:
    need(tests.get(key) is False, f"forbidden test drift: {key}")
for key in [
    "owned_semantic_resource_declared",
    "source_surface_set_declared",
    "state_target_set_declared",
    "operation_effect_class_set_declared",
    "native_seed_file_boundary_candidate_declared",
    "module_export_candidate_declared",
    "generator_overwrite_guard_candidate_declared",
]:
    need(tests.get(key) is True, f"expected typed evidence missing: {key}")
for key in [
    "standalone_subject_id_declared",
    "parent_owner_id_declared",
    "parent_semantics_not_copied",
    "external_parent_dependencies_declared",
]:
    need(tests.get(key) is False, f"standalone boundary proof should be missing: {key}")

classification = fixture.get("classification") or {}
need(classification.get("kind") == "RemainParentOwned", "classification drift")
need(classification.get("standalone_projection_subject_established") is False, "standalone proof drift")
need(classification.get("lifecycle_contract_descriptor_allowed_next") is False, "lifecycle next drift")
need(classification.get("source_plan_materialization_allowed") is False, "source plan allowed drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectWiderRouteSelectionBasis", "decision kind drift")
need(decision.get("reason_token") == reason, "decision reason drift")
need(decision.get("selected_next_card") == next_card, "next card drift")

claims = fixture.get("claims") or {}
for key in [
    "latest_candidate_rerun_consumed",
    "context_registry_projection_policy_consumed",
    "id_scalar_derivable_owner_discriminator_resolution_consumed",
]:
    need(claims.get(key) == 1, f"required claim missing: {key}")
for key in [
    "remaining_owner_count_as_proof",
    "owner_name_as_proof",
    "source_symbol_as_proof",
    "source_path_as_authority",
    "with_plugin_sigs_symbol_name_as_proof",
    "keep_parent_owner_as_standalone_proof",
    "projection_descriptor_coverage_as_standalone_proof",
    "lifecycle_contract_descriptor_completeness",
    "source_plan_materialization",
    "behavior_recipe_materialization",
    "verifier_result_materialization",
    "derived_artifact_seed_draft_materialization",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
for needle in [
    token,
    "reason_token = " + reason,
    "selected_next_card = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-007",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-parent-owned-subject-boundary-resolution")
print("classification=RemainParentOwned")
print("reason_token=" + reason)
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
