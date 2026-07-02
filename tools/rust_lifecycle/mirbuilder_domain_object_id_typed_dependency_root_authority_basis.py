#!/usr/bin/env python3
"""Define typed dependency-root authority for non-ID DomainObject/Id subaxes."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-domain-object-id-typed-dependency-root-authority-basis-v0.json"

TOKEN = "MIRBUILDER-DOMAIN-OBJECT-ID-TYPED-DEPENDENCY-ROOT-AUTHORITY-BASIS-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_RERUN = "MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RERUN-003"

INVENTORY = FIXTURES / "mirbuilder-domain-object-id-transport-policy-inventory-rerun-002-v0.json"
MECHANICAL_BASIS = FIXTURES / "mirbuilder-domain-object-id-subaxis-mechanical-selection-basis-v0.json"
RERUN_002 = FIXTURES / "mirbuilder-domain-object-id-unresolved-subaxis-priority-rerun-002-v0.json"

ACCEPTED_EDGE_EVIDENCE_KINDS = [
    "ReturnTypeFieldReference",
    "ParameterTypeReference",
    "ConstructedDomainObjectReference",
    "VerifierInputContractReference",
    "PolicyDecisionPayloadReference",
    "FixtureDeclaredSemanticResourceDependency",
]

FORBIDDEN_EDGE_EVIDENCE_KINDS = [
    "RowCount",
    "OwnerName",
    "SourcePath",
    "RouteMembershipAlone",
    "CoveragePercentage",
    "ImplementationConvenience",
    "LexicalOrder",
    "HardcodedSubaxisPriority",
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_candidate(row: dict[str, Any]) -> dict[str, Any]:
    return {
        "domain_subaxis": row["domain_subaxis"],
        "guard_clean_authority": "Proven",
        "dependency_participation": "NotEvaluatedByBasis",
        "incoming_dependency_edge_count": 0,
        "outgoing_dependency_edge_count": 0,
        "outgoing_unresolved_prerequisite_count": 0,
        "depends_on_candidate_set": [],
        "candidate_depended_on_by_set": [],
        "dependency_root_authority": {
            "status": "NotEvaluatedByBasis",
            "positive_dependent_count": 0,
            "all_non_isolated_candidates_depend_on_candidate": 0,
            "unique_root": 0,
            "proof_sources": [],
        },
        "proof_tuple_complete": False,
        "selection_eligible": False,
    }


def build_fixture() -> dict[str, Any]:
    mechanical_basis = read_json(MECHANICAL_BASIS)
    rerun_002 = read_json(RERUN_002)
    candidates = [
        build_candidate(candidate)
        for candidate in sorted(
            mechanical_basis.get("candidate_subaxes") or [],
            key=lambda candidate: candidate["domain_subaxis"],
        )
    ]

    return {
        "schema_version": 0,
        "kind": "MirBuilderDomainObjectIdTypedDependencyRootAuthorityBasisV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "subaxis_mechanical_selection_basis": rel(MECHANICAL_BASIS),
            "unresolved_subaxis_priority_rerun_002": rel(RERUN_002),
            "domain_object_id_transport_policy_inventory_rerun_002": rel(INVENTORY),
        },
        "provenance": {
            "subaxis_mechanical_selection_basis_hash": sha256_file(MECHANICAL_BASIS),
            "unresolved_subaxis_priority_rerun_002_hash": sha256_file(RERUN_002),
            "domain_object_id_transport_policy_inventory_rerun_002_hash": sha256_file(INVENTORY),
        },
        "previous_state": {
            "candidate_subaxis_count": rerun_002.get("summary", {}).get("candidate_subaxis_count"),
            "guard_clean_candidate_count": rerun_002.get("summary", {}).get("guard_clean_candidate_count"),
            "proof_tuple_complete_candidate_count": rerun_002.get("summary", {}).get(
                "proof_tuple_complete_candidate_count"
            ),
            "selection_eligible_subaxis_count": rerun_002.get("summary", {}).get(
                "selection_eligible_subaxis_count"
            ),
            "previous_reason_token": rerun_002.get("decision", {}).get("reason_token"),
        },
        "selector_rule": {
            "name": "DomainObjectIdTypedDependencyRootAuthorityV1",
            "extends": "DomainObjectIdSubaxisMechanicalSelectorV1.dependency_root_authority",
            "edge_direction": "dependent_subaxis_requires_prerequisite_subaxis",
            "selection_requires_exactly_one_dependency_root": True,
            "isolated_candidates_are_unranked": True,
            "hardcoded_subaxis_priority": False,
            "row_count_is_diagnostic_only": True,
            "owner_name_as_proof": False,
            "source_path_as_authority": False,
            "route_membership_alone_as_proof": False,
            "coverage_as_proof": False,
            "convenience_as_proof": False,
        },
        "accepted_edge_evidence_kinds": ACCEPTED_EDGE_EVIDENCE_KINDS,
        "forbidden_edge_evidence_kinds": FORBIDDEN_EDGE_EVIDENCE_KINDS,
        "typed_dependency_edges": [],
        "edge_inventory_state": "ShapeDefinedNotEvaluated",
        "candidate_subaxes": candidates,
        "graph_summary": {
            "candidate_subaxis_count": len(candidates),
            "accepted_typed_dependency_edge_count": 0,
            "rejected_edge_count": 0,
            "ambiguous_edge_count": 0,
            "forbidden_edge_source_count": 0,
            "cycle_count": 0,
            "dependency_root_candidate_count": 0,
            "guard_clean_candidate_count": rerun_002.get("summary", {}).get(
                "guard_clean_candidate_count"
            ),
            "proof_tuple_complete_candidate_count": 0,
            "selection_eligible_subaxis_count": 0,
        },
        "decision": {
            "kind": "SelectDomainObjectIdSubaxisPriorityRerun003",
            "reason_token": "DefineDomainObjectIdTypedDependencyRootAuthorityBeforeSubaxisSelection",
            "selected_domain_subaxis": None,
            "selected_next_card": NEXT_RERUN,
        },
        "recovery_if_rerun_fails": {
            "no_root_reason_token": "NoMachineDerivedDomainObjectIdTypedDependencyRootAuthority",
            "multiple_roots_reason_token": "MultipleDomainObjectIdTypedDependencyRootCandidates",
            "cycle_reason_token": "DomainObjectIdSubaxisDependencyCycleUnresolved",
            "selected_next_card": DESIGN_STOP,
        },
        "claims": {
            "typed_dependency_root_authority_basis_defined": 1,
            "source_selfhost_claim": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
            "manual_subaxis_selection": 0,
            "hardcoded_subaxis_priority": 0,
            "row_count_as_proof": 0,
            "domain_object_count_as_proof": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "owner_name_as_proof": 0,
            "source_path_as_authority": 0,
            "route_membership_alone_as_proof": 0,
            "convenience_as_proof": 0,
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
        print("mirbuilder-domain-object-id-typed-dependency-root-authority-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
