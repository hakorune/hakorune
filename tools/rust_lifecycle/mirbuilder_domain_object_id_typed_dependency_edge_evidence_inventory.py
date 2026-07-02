#!/usr/bin/env python3
"""Inventory typed dependency-edge evidence sources for non-ID DomainObject/Id."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-domain-object-id-typed-dependency-edge-evidence-inventory-v0.json"

TOKEN = "MIRBUILDER-DOMAIN-OBJECT-ID-TYPED-DEPENDENCY-EDGE-EVIDENCE-INVENTORY-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-DOMAIN-OBJECT-ID-RETURN-TYPE-REFERENCE-EDGE-DERIVATION-BASIS-001"

BASIS = FIXTURES / "mirbuilder-domain-object-id-typed-dependency-root-authority-basis-v0.json"
RERUN_003 = FIXTURES / "mirbuilder-domain-object-id-unresolved-subaxis-priority-rerun-003-v0.json"
INVENTORY = FIXTURES / "mirbuilder-domain-object-id-transport-policy-inventory-rerun-002-v0.json"

ACCEPTED_EDGE_EVIDENCE_KINDS = [
    "ReturnTypeFieldReference",
    "ParameterTypeReference",
    "ConstructedDomainObjectReference",
    "VerifierInputContractReference",
    "PolicyDecisionPayloadReference",
    "FixtureDeclaredSemanticResourceDependency",
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def unresolved_rows(inventory: dict[str, Any]) -> list[dict[str, Any]]:
    return [
        row
        for row in inventory.get("domain_object_id_source_id_ledger") or []
        if row.get("scope_state") == "UnresolvedNonIdDomainObject"
    ]


def evidence_row(
    *,
    evidence_kind: str,
    direct_source_field_present: bool,
    source_field: str | None,
    candidate_reference_count: int,
    concrete_typed_reference_count: int,
    selected_for_derivation_basis: bool,
    blocked_by: list[str],
    notes: dict[str, Any] | None = None,
) -> dict[str, Any]:
    return {
        "evidence_kind": evidence_kind,
        "direct_source_field_present": direct_source_field_present,
        "source_field": source_field,
        "candidate_reference_count": candidate_reference_count,
        "concrete_typed_reference_count": concrete_typed_reference_count,
        "accepted_edge_ready_count": 0,
        "selected_for_derivation_basis": selected_for_derivation_basis,
        "blocked_by": blocked_by,
        "notes": notes or {},
    }


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    rerun_003 = read_json(RERUN_003)
    inventory = read_json(INVENTORY)
    rows = unresolved_rows(inventory)

    return_type_rows = [row for row in rows if row.get("return_type")]
    policy_payload_pattern_rows = [
        row for row in return_type_rows if str(row.get("return_type", "")).startswith("PolicyDecision<")
    ]
    return_type_counts = Counter(str(row.get("return_type")) for row in return_type_rows)

    evidence_sources = [
        evidence_row(
            evidence_kind="ReturnTypeFieldReference",
            direct_source_field_present=True,
            source_field="return_type",
            candidate_reference_count=len(return_type_rows),
            concrete_typed_reference_count=len(return_type_rows),
            selected_for_derivation_basis=True,
            blocked_by=[
                "ReturnTypeReferenceIsNotDependencyEdgeByItself",
                "ReturnTypeToDomainSubaxisMappingBasisMissing",
            ],
            notes={
                "distinct_return_type_count": len(return_type_counts),
                "top_return_types": [
                    {"return_type": name, "count": count}
                    for name, count in return_type_counts.most_common(10)
                ],
            },
        ),
        evidence_row(
            evidence_kind="ParameterTypeReference",
            direct_source_field_present=False,
            source_field=None,
            candidate_reference_count=0,
            concrete_typed_reference_count=0,
            selected_for_derivation_basis=False,
            blocked_by=["LedgerFieldMissing:parameter_types"],
        ),
        evidence_row(
            evidence_kind="ConstructedDomainObjectReference",
            direct_source_field_present=False,
            source_field=None,
            candidate_reference_count=0,
            concrete_typed_reference_count=0,
            selected_for_derivation_basis=False,
            blocked_by=["LedgerFieldMissing:constructed_domain_object"],
        ),
        evidence_row(
            evidence_kind="VerifierInputContractReference",
            direct_source_field_present=False,
            source_field=None,
            candidate_reference_count=0,
            concrete_typed_reference_count=0,
            selected_for_derivation_basis=False,
            blocked_by=["LedgerFieldMissing:verifier_input_contract"],
        ),
        evidence_row(
            evidence_kind="PolicyDecisionPayloadReference",
            direct_source_field_present=False,
            source_field=None,
            candidate_reference_count=len(policy_payload_pattern_rows),
            concrete_typed_reference_count=0,
            selected_for_derivation_basis=False,
            blocked_by=[
                "PolicyDecisionPayloadOnlyPatternObservedInReturnType",
                "PolicyPayloadFixtureMissing",
            ],
        ),
        evidence_row(
            evidence_kind="FixtureDeclaredSemanticResourceDependency",
            direct_source_field_present=False,
            source_field=None,
            candidate_reference_count=0,
            concrete_typed_reference_count=0,
            selected_for_derivation_basis=False,
            blocked_by=["FixtureDeclaredDependencyRowsMissing"],
        ),
    ]

    selected_sources = [
        row for row in evidence_sources if row["selected_for_derivation_basis"]
    ]

    return {
        "schema_version": 0,
        "kind": "MirBuilderDomainObjectIdTypedDependencyEdgeEvidenceInventoryV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "typed_dependency_root_authority_basis": rel(BASIS),
            "unresolved_subaxis_priority_rerun_003": rel(RERUN_003),
            "domain_object_id_transport_policy_inventory_rerun_002": rel(INVENTORY),
        },
        "provenance": {
            "typed_dependency_root_authority_basis_hash": sha256_file(BASIS),
            "unresolved_subaxis_priority_rerun_003_hash": sha256_file(RERUN_003),
            "domain_object_id_transport_policy_inventory_rerun_002_hash": sha256_file(INVENTORY),
        },
        "previous_state": {
            "rerun_003_decision": rerun_003.get("decision", {}).get("kind"),
            "rerun_003_reason_token": rerun_003.get("decision", {}).get("reason_token"),
            "accepted_typed_dependency_edge_count": rerun_003.get("graph_summary", {}).get(
                "accepted_typed_dependency_edge_count"
            ),
            "dependency_root_candidate_count": rerun_003.get("graph_summary", {}).get(
                "dependency_root_candidate_count"
            ),
        },
        "accepted_edge_evidence_kinds": basis.get("accepted_edge_evidence_kinds")
        or ACCEPTED_EDGE_EVIDENCE_KINDS,
        "evidence_source_inventory": evidence_sources,
        "summary": {
            "unresolved_non_id_domain_row_count": len(rows),
            "evidence_kind_count": len(evidence_sources),
            "direct_source_field_evidence_kind_count": sum(
                1 for row in evidence_sources if row["direct_source_field_present"]
            ),
            "selected_evidence_kind_count": len(selected_sources),
            "return_type_field_reference_candidate_count": len(return_type_rows),
            "policy_decision_payload_pattern_count": len(policy_payload_pattern_rows),
            "accepted_edge_ready_count": 0,
            "selected_evidence_kind": (
                selected_sources[0]["evidence_kind"] if len(selected_sources) == 1 else None
            ),
        },
        "decision": {
            "kind": "SelectReturnTypeReferenceEdgeDerivationBasis",
            "reason_token": "ReturnTypeFieldReferenceIsOnlyConcreteLedgerEvidenceSource",
            "selected_evidence_kind": "ReturnTypeFieldReference",
            "selected_domain_subaxis": None,
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "typed_dependency_edge_evidence_inventory": 1,
            "return_type_field_as_edge_by_itself": 0,
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
        print("mirbuilder-domain-object-id-typed-dependency-edge-evidence-inventory unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
