#!/usr/bin/env python3
"""Materialize the ID scalar derivable-owner discriminator basis fixture."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
PHASE = ROOT / "docs/development/current/main/phases/phase-296x"

BASIS_OUTPUT = FIXTURES / "mirbuilder-id-scalar-derivable-owner-discriminator-basis-v0.json"
FORMALIZATION_OUTPUT = (
    FIXTURES / "mirbuilder-id-scalar-derivable-owner-discriminator-basis-formalization-v0.json"
)

TOKEN = "MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-BASIS-FORMALIZATION-001"
NEXT_CARD = "MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-PROOF-AXIS-REFINEMENT-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

BASIS_CARD = PHASE / "2040-MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-BASIS-001.md"
RESOLUTION_CARD = (
    PHASE / "2044-MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-RESOLUTION-001.md"
)
RESOLUTION_FIXTURE = (
    FIXTURES / "mirbuilder-id-scalar-derivable-owner-discriminator-resolution-v0.json"
)

ALLOWED_PROOF_AXES = [
    "TypedEvidenceIndexCompleteness",
    "VerifierInputContractCompleteness",
    "NativeSeedFileBoundaryDeterminism",
    "StateTargetClosureQuality",
    "OperationEffectClassCompleteness",
    "SourcePlanRecipeComponentReadiness",
    "SemanticOperationAuthorityComplete",
    "SelectorGuardClean",
]

TIE_BREAK_SIGNALS_ONLY = [
    "AlreadyHakoAdoptedAdjacency",
    "MinimalPathProximity",
    "MigrationUnblockValue",
]

FORBIDDEN_SELECTION_AXES = [
    "OwnerName",
    "LexicalOrder",
    "SurfaceCount",
    "RowCount",
    "ClusterSize",
    "CoveragePercentage",
    "RouteMembershipAlone",
    "ManualOwnerPreference",
]

AUTHORITY_RULES = {
    "typed_evidence_index_required": True,
    "mention_only_owner_edge_text_is_not_evidence": True,
    "fixture_declared_role_required_for_semantic_operation_mapping": True,
    "operation_name_fallback_is_diagnostic_only": True,
    "shape_name_is_provenance_not_semantic_policy": True,
    "eligible_zero_or_lexical_sort_selection_forbidden": True,
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def build_basis_fixture() -> dict:
    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarDerivableOwnerDiscriminatorBasisV1",
        "source_token": "MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-BASIS-001",
        "source_card": rel(BASIS_CARD),
        "allowed_proof_axes": ALLOWED_PROOF_AXES,
        "tie_break_signals_only": TIE_BREAK_SIGNALS_ONLY,
        "forbidden_selection_axes": FORBIDDEN_SELECTION_AXES,
        "authority_rules": AUTHORITY_RULES,
        "claims": {
            "manual_owner_selection": 0,
            "owner_name_as_proof": 0,
            "lexical_order_as_proof": 0,
            "surface_count_as_proof": 0,
            "row_count_as_proof": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
        },
    }


def build_formalization_fixture(basis_hash: str | None = None) -> dict:
    resolution = read_json(RESOLUTION_FIXTURE)
    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarDerivableOwnerDiscriminatorBasisFormalizationV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "basis_card": rel(BASIS_CARD),
            "resolution_card": rel(RESOLUTION_CARD),
            "resolution_fixture": rel(RESOLUTION_FIXTURE),
        },
        "provenance": {
            "basis_card_hash": sha256_file(BASIS_CARD),
            "resolution_card_hash": sha256_file(RESOLUTION_CARD),
            "resolution_fixture_hash": sha256_file(RESOLUTION_FIXTURE),
        },
        "formalized_fixture": {
            "path": rel(BASIS_OUTPUT),
            "materialized": True,
            "hash": basis_hash if basis_hash is not None else sha256_file(BASIS_OUTPUT),
        },
        "resolution_reference_repair": {
            "resolution_referenced_basis_fixture_exists": True,
            "resolution_token": resolution.get("token"),
            "resolution_reason_token": (resolution.get("decision") or {}).get("reason_token"),
        },
        "allowed_proof_axes": ALLOWED_PROOF_AXES,
        "tie_break_signals_only": TIE_BREAK_SIGNALS_ONLY,
        "forbidden_selection_axes": FORBIDDEN_SELECTION_AXES,
        "authority_rules": AUTHORITY_RULES,
        "decision": {
            "kind": "BasisFixtureMaterialized",
            "reason_token": "IdScalarDerivableOwnerDiscriminatorBasisFixtureMaterialized",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "manual_owner_selection": 0,
            "owner_name_as_proof": 0,
            "lexical_order_as_proof": 0,
            "surface_count_as_proof": 0,
            "row_count_as_proof": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "source_plan_materialization": 0,
            "behavior_recipe_materialization": 0,
            "verifier_result_materialization": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
        },
    }


def expected_outputs() -> dict[Path, str]:
    basis_text = stable_json(build_basis_fixture())
    import hashlib

    basis_hash = hashlib.sha256(basis_text.encode("utf-8")).hexdigest()
    formalization_text = stable_json(build_formalization_fixture(basis_hash))
    return {
        BASIS_OUTPUT: stable_json(build_basis_fixture()),
        FORMALIZATION_OUTPUT: formalization_text,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in fixtures.")
    args = parser.parse_args()

    if args.check:
        expected = expected_outputs()
        for path, text in expected.items():
            if not path.exists() or path.read_text(encoding="utf-8") != text:
                raise SystemExit(f"{rel(path)} is stale; rerun without --check")
        print("mirbuilder-id-scalar-derivable-owner-discriminator-basis-formalization unchanged")
        return 0

    write_if_changed(BASIS_OUTPUT, stable_json(build_basis_fixture()))
    expected = expected_outputs()
    for path, text in expected.items():
        changed = write_if_changed(path, text)
        print(("updated=" if changed else "unchanged=") + rel(path))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
