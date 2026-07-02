#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BASIS="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-derivable-owner-discriminator-basis-v0.json"
FORMALIZATION="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-derivable-owner-discriminator-basis-formalization-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_derivable_owner_discriminator_basis_formalization.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2045-MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-BASIS-FORMALIZATION-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

python3 "$TOOL" --check

python3 - "$BASIS" "$FORMALIZATION" "$CARD" "$STATE" "$TASK_ORDER" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

basis = json.load(open(sys.argv[1], encoding="utf-8"))
formalization = json.load(open(sys.argv[2], encoding="utf-8"))
card = Path(sys.argv[3]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[4]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[5]).read_text(encoding="utf-8")

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

token = "MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-BASIS-FORMALIZATION-001"
next_card = "MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-PROOF-AXIS-REFINEMENT-001"

need(basis.get("kind") == "MirBuilderIdScalarDerivableOwnerDiscriminatorBasisV1", "bad basis kind")
need(formalization.get("kind") == "MirBuilderIdScalarDerivableOwnerDiscriminatorBasisFormalizationV1", "bad formalization kind")
need(formalization.get("token") == token, "bad token")
need(token in card, "card missing token")

for axis in [
    "TypedEvidenceIndexCompleteness",
    "VerifierInputContractCompleteness",
    "NativeSeedFileBoundaryDeterminism",
    "StateTargetClosureQuality",
    "OperationEffectClassCompleteness",
    "SourcePlanRecipeComponentReadiness",
    "SemanticOperationAuthorityComplete",
    "SelectorGuardClean",
]:
    need(axis in basis.get("allowed_proof_axes", []), f"missing allowed proof axis {axis}")
    need(axis in formalization.get("allowed_proof_axes", []), f"formalization missing axis {axis}")

for axis in [
    "OwnerName",
    "LexicalOrder",
    "SurfaceCount",
    "RowCount",
    "ClusterSize",
    "CoveragePercentage",
    "RouteMembershipAlone",
    "ManualOwnerPreference",
]:
    need(axis in basis.get("forbidden_selection_axes", []), f"missing forbidden axis {axis}")

rules = formalization.get("authority_rules") or {}
for key in [
    "typed_evidence_index_required",
    "mention_only_owner_edge_text_is_not_evidence",
    "fixture_declared_role_required_for_semantic_operation_mapping",
    "operation_name_fallback_is_diagnostic_only",
    "shape_name_is_provenance_not_semantic_policy",
    "eligible_zero_or_lexical_sort_selection_forbidden",
]:
    need(rules.get(key) is True, f"authority rule drift: {key}")

fixture = formalization.get("formalized_fixture") or {}
need(fixture.get("materialized") is True, "basis fixture not materialized")
need(fixture.get("path", "").endswith("mirbuilder-id-scalar-derivable-owner-discriminator-basis-v0.json"), "bad basis path")

decision = formalization.get("decision") or {}
need(decision.get("kind") == "BasisFixtureMaterialized", "bad decision kind")
need(decision.get("reason_token") == "IdScalarDerivableOwnerDiscriminatorBasisFixtureMaterialized", "bad reason")
need(decision.get("selected_next_card") == next_card, "bad next card")

claims = formalization.get("claims") or {}
for key in [
    "manual_owner_selection",
    "owner_name_as_proof",
    "lexical_order_as_proof",
    "surface_count_as_proof",
    "row_count_as_proof",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "source_plan_materialization",
    "behavior_recipe_materialization",
    "verifier_result_materialization",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
for needle in [
    token,
    "reason_token = IdScalarDerivableOwnerDiscriminatorBasisFixtureMaterialized",
    "selected_next_card = MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-PROOF-AXIS-REFINEMENT-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-derivable-owner-discriminator-basis-formalization")
print("basis_fixture_materialized=1")
print("reason_token=IdScalarDerivableOwnerDiscriminatorBasisFixtureMaterialized")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
