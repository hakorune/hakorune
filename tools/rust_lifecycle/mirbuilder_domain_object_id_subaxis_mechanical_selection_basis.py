#!/usr/bin/env python3
"""Define the mechanical selector basis for non-ID DomainObject/Id subaxes."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-domain-object-id-subaxis-mechanical-selection-basis-v0.json"

TOKEN = "MIRBUILDER-DOMAIN-OBJECT-ID-SUBAXIS-MECHANICAL-SELECTION-BASIS-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_RERUN = "MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RERUN-002"

INVENTORY = FIXTURES / "mirbuilder-domain-object-id-transport-policy-inventory-rerun-002-v0.json"
PRIORITY = FIXTURES / "mirbuilder-domain-object-id-unresolved-subaxis-priority-resolution-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_candidate(candidate: dict[str, Any]) -> dict[str, Any]:
    return {
        "domain_subaxis": candidate["domain_subaxis"],
        "row_count": candidate["row_count"],
        "row_count_diagnostic_only": True,
        "dependency_root_authority": {
            "status": "NotEvaluatedAtBasis",
            "typed_dependency_edges_required": True,
            "unique_root_required": True,
            "all_other_selected_candidates_depend_on_root_required": True,
            "proof_sources": [],
        },
        "prior_closed_lane_continuation_authority": {
            "status": "NotEvaluatedAtBasis",
            "closed_lane_source_id_overlap_required": True,
            "semantic_resource_continuity_required": True,
            "owner_edge_or_shape_signature_continuity_required": True,
            "proof_sources": [],
        },
        "guard_clean_authority": {
            "status": "NotEvaluatedAtBasis",
            "native_seed_materialization_required": 0,
            "hako_generation_required": 0,
            "hako_adopted_decision_required": 0,
            "source_selfhost_claim_required": 0,
            "runtime_fallback_required": 0,
            "new_backend_route_required": 0,
            "new_abi_required": 0,
            "new_python_semantic_projector_required": 0,
            "runner_semantic_owner_required": 0,
        },
        "proof_tuple_complete": False,
        "selection_eligible": False,
        "blocked_by": ["MechanicalSelectorBasisDefinedButNotEvaluated"],
    }


def build_fixture() -> dict[str, Any]:
    priority = read_json(PRIORITY)
    candidates = sorted(
        (build_candidate(candidate) for candidate in priority.get("candidate_subaxes") or []),
        key=lambda candidate: candidate["domain_subaxis"],
    )

    candidate_names = [candidate["domain_subaxis"] for candidate in candidates]
    return {
        "schema_version": 0,
        "kind": "MirBuilderDomainObjectIdSubaxisMechanicalSelectionBasisV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "domain_object_id_transport_policy_inventory_rerun_002": rel(INVENTORY),
            "unresolved_subaxis_priority_resolution": rel(PRIORITY),
        },
        "local_authority": {
            "local_selection_authority": "LocalMechanicalSelectorAuthorityV1",
            "worker_inventory": "consumed",
            "worker_inventory_scope": "read_only_current_fixtures_cards_ledgers",
        },
        "provenance": {
            "domain_object_id_transport_policy_inventory_rerun_002_hash": sha256_file(INVENTORY),
            "unresolved_subaxis_priority_resolution_hash": sha256_file(PRIORITY),
        },
        "previous_state": {
            "unresolved_non_id_domain_row_count": priority.get("summary", {}).get(
                "unresolved_non_id_domain_row_count"
            ),
            "candidate_subaxis_count": priority.get("summary", {}).get("candidate_subaxis_count"),
            "selection_eligible_subaxis_count": priority.get("summary", {}).get(
                "selection_eligible_subaxis_count"
            ),
            "previous_reason_token": priority.get("decision", {}).get("reason_token"),
            "previous_decision": priority.get("decision", {}).get("kind"),
        },
        "selector_rule": {
            "name": "DomainObjectIdSubaxisMechanicalSelectorV1",
            "selection_requires_exactly_one_guard_clean_candidate": True,
            "dependency_root_authority_allowed": True,
            "prior_closed_lane_continuation_authority_allowed": True,
            "proof_tuple_complete_requires": (
                "guard_clean_authority AND "
                "(dependency_root_authority OR prior_closed_lane_continuation_authority)"
            ),
            "hardcoded_subaxis_priority": False,
            "row_count_is_diagnostic_only": True,
            "owner_name_as_proof": False,
            "source_path_as_authority": False,
            "route_membership_alone_as_proof": False,
            "manual_subaxis_selection": False,
        },
        "candidate_subaxes": candidates,
        "summary": {
            "candidate_subaxis_count": len(candidates),
            "candidate_set": candidate_names,
            "guard_clean_candidate_count": 0,
            "proof_tuple_complete_candidate_count": 0,
            "selection_eligible_subaxis_count": 0,
        },
        "decision": {
            "kind": "SelectDomainObjectIdSubaxisPriorityRerun",
            "reason_token": "DomainObjectIdSubaxisMechanicalSelectorBasisDefined",
            "selected_domain_subaxis": None,
            "selected_next_card": NEXT_RERUN,
        },
        "recovery_if_rerun_fails": {
            "no_candidate_reason_token": "NoExactlyOneDomainObjectIdSubaxisMechanicalCandidate",
            "multiple_candidate_reason_token": "MultipleDomainObjectIdSubaxisMechanicalCandidates",
            "dependency_root_missing_reason_token": "NoDomainObjectIdSubaxisDependencyRootAuthority",
            "guard_clean_missing_reason_token": "NoDomainObjectIdSubaxisGuardCleanAuthority",
            "selected_next_card": DESIGN_STOP,
        },
        "claims": {
            "domain_object_id_subaxis_mechanical_selection_basis_defined": 1,
            "local_mechanical_selector_authority_consumed": 1,
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
            "generated_artifact_as_native_edit_authority": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
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
        print("mirbuilder-domain-object-id-subaxis-mechanical-selection-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
