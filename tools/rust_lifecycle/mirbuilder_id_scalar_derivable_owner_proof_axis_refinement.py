#!/usr/bin/env python3
"""Define refined proof axes for tied ID scalar derivable owners."""

from __future__ import annotations

import argparse
from pathlib import Path

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-derivable-owner-proof-axis-refinement-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-PROOF-AXIS-REFINEMENT-001"
NEXT_CARD = "MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-RESOLUTION-002"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

BASIS = FIXTURES / "mirbuilder-id-scalar-derivable-owner-discriminator-basis-v0.json"
FORMALIZATION = (
    FIXTURES / "mirbuilder-id-scalar-derivable-owner-discriminator-basis-formalization-v0.json"
)
RESOLUTION = FIXTURES / "mirbuilder-id-scalar-derivable-owner-discriminator-resolution-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def build_fixture() -> dict:
    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarDerivableOwnerProofAxisRefinementV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "basis_fixture": rel(BASIS),
            "basis_formalization": rel(FORMALIZATION),
            "discriminator_resolution_001": rel(RESOLUTION),
        },
        "provenance": {
            "basis_fixture_hash": sha256_file(BASIS),
            "basis_formalization_hash": sha256_file(FORMALIZATION),
            "discriminator_resolution_001_hash": sha256_file(RESOLUTION),
        },
        "candidate_axes": [
            {
                "axis_name": "PriorProjectionPolicyDisposition",
                "decision": "ReplaceWithRefinedProofAxis",
                "refined_axis": "StandaloneProjectionSubjectEstablished",
                "proof_allowed": True,
                "conditions": [
                    "projection_policy_fixture_present",
                    "decision_kind_is_SelectProjectionPolicyDescriptor",
                    "descriptor_selected = 1",
                    "owner_edge_matches_candidate",
                    "not_parent_owned",
                    "not_diagnostic_only",
                    "standalone_subject_boundary_declared",
                ],
                "forbidden_interpretation": "Prefer owner because descriptor existed historically",
            },
            {
                "axis_name": "ContractLifecycleDescriptorPresence",
                "decision": "ReplaceWithRefinedProofAxis",
                "refined_axis": "LifecycleContractDescriptorCompleteness",
                "proof_allowed": True,
                "conditions": [
                    "contract_descriptor_exists",
                    "return_contract_declared",
                    "mutation_entrypoints_declared",
                    "diagnostic_error_semantics_declared",
                    "verifier_observable_effects_declared",
                    "typed_evidence_refs_present",
                ],
                "forbidden_interpretation": "Prefer richer lifecycle owner",
            },
            {
                "axis_name": "LifecycleMutationShape",
                "decision": "ReplaceWithRefinedProofAxis",
                "refined_axis": "MutationFrameSemanticCompleteness",
                "proof_allowed": True,
                "conditions": [
                    "read_set_declared_when_read_exists",
                    "write_set_declared_when_write_exists",
                    "append_targets_declared_when_append_exists",
                    "patch_targets_declared_when_patch_exists",
                    "owner_return_state_declared",
                    "mutation_order_declared",
                    "rollback_or_no_rollback_declared",
                    "cleanup_or_no_cleanup_declared",
                ],
                "forbidden_interpretation": "Prefer complexity or more mutation kinds",
            },
            {
                "axis_name": "VerifierEffectClassPresence",
                "decision": "ReplaceWithRefinedProofAxis",
                "refined_axis": "VerifierEffectClassCoverageCompleteness",
                "proof_allowed": True,
                "conditions": [
                    "all_operation_tokens_map_to_effect_classes",
                    "all_effect_classes_map_to_verifier_input_facts",
                    "no_verifier_visible_effect_unaccounted",
                    "predicate_reads_declared",
                    "diagnostic_error_effects_declared",
                    "state_mutation_effects_declared",
                ],
                "forbidden_interpretation": "Prefer owner with more effect classes",
            },
        ],
        "decision": {
            "kind": "ProofAxesRefined",
            "reason_token": "IdScalarDerivableOwnerProofAxesRefinedWithoutCountOrNameProof",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "owner_name_as_proof": 0,
            "historical_descriptor_presence_as_preference": 0,
            "lifecycle_richness_as_proof": 0,
            "mutation_complexity_as_proof": 0,
            "effect_class_count_as_proof": 0,
            "surface_count_as_proof": 0,
            "row_count_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "source_plan_materialization": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-id-scalar-derivable-owner-proof-axis-refinement unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
