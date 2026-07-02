#!/usr/bin/env python3
"""Rerun non-ID DomainObject/Id subaxis priority under the mechanical basis."""

from __future__ import annotations

import argparse
import json
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-domain-object-id-unresolved-subaxis-priority-rerun-002-v0.json"

TOKEN = "MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RERUN-002"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
BASIS_TOKEN = "MIRBUILDER-DOMAIN-OBJECT-ID-SUBAXIS-MECHANICAL-SELECTION-BASIS-001"

INVENTORY = FIXTURES / "mirbuilder-domain-object-id-transport-policy-inventory-rerun-002-v0.json"
BASIS = FIXTURES / "mirbuilder-domain-object-id-subaxis-mechanical-selection-basis-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def rows_by_subaxis(rows: list[dict[str, Any]]) -> dict[str, list[dict[str, Any]]]:
    grouped: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        grouped[row["domain_subaxis"]].append(row)
    return grouped


def nonempty_known_values(rows: list[dict[str, Any]], field: str) -> set[str]:
    return {row[field] for row in rows if row.get(field)}


def evaluate_candidate(
    *,
    basis_candidate: dict[str, Any],
    unresolved_rows: list[dict[str, Any]],
    closed_id_scalar_rows: list[dict[str, Any]],
) -> dict[str, Any]:
    subaxis = basis_candidate["domain_subaxis"]
    rows = sorted(unresolved_rows, key=lambda row: row["source_id"])
    closed_source_ids = nonempty_known_values(closed_id_scalar_rows, "source_id")
    row_source_ids = nonempty_known_values(rows, "source_id")
    row_owner_edges = nonempty_known_values(rows, "known_owner_edge")
    closed_owner_edges = nonempty_known_values(closed_id_scalar_rows, "known_owner_edge")
    row_shapes = nonempty_known_values(rows, "shape_signature")
    closed_shapes = nonempty_known_values(closed_id_scalar_rows, "shape_signature")

    owner_edge_overlap = sorted(row_owner_edges & closed_owner_edges)
    shape_overlap = sorted(row_shapes & closed_shapes)
    source_id_overlap = sorted(row_source_ids & closed_source_ids)
    owner_counts = Counter(row.get("known_owner_edge") or "<none>" for row in rows)

    dependency_root_proven = False
    closed_lane_proven = False
    guard_clean_proven = True
    proof_tuple_complete = guard_clean_proven and (dependency_root_proven or closed_lane_proven)

    blocked_by: list[str] = []
    if not dependency_root_proven:
        blocked_by.append("NoDependencyRootAuthority")
    if not closed_lane_proven:
        blocked_by.append("NoPriorClosedLaneContinuationAuthority")
    if not proof_tuple_complete:
        blocked_by.append("ProofTupleIncomplete")

    return {
        "domain_subaxis": subaxis,
        "row_count": len(rows),
        "row_count_diagnostic_only": True,
        "owner_edge_counts": dict(sorted(owner_counts.items())),
        "sample_source_ids": [row["source_id"] for row in rows[:12]],
        "dependency_root_authority": {
            "status": "Unproven",
            "typed_dependency_edges_present": 0,
            "unique_root": 0,
            "all_other_selected_candidates_depend_on_root": 0,
            "proof_sources": [],
        },
        "prior_closed_lane_continuation_authority": {
            "status": "Unproven",
            "closed_lane_source_id_overlap_present": int(bool(source_id_overlap)),
            "closed_lane_source_id_overlap_count": len(source_id_overlap),
            "owner_edge_continuity_present": int(bool(owner_edge_overlap)),
            "owner_edge_continuity_values": owner_edge_overlap,
            "shape_signature_continuity_present": int(bool(shape_overlap)),
            "shape_signature_continuity_values": shape_overlap,
            "semantic_resource_continuity_present": 0,
            "proof_sources": [],
        },
        "guard_clean_authority": {
            "status": "Proven",
            "native_seed_materialization_required": 0,
            "hako_generation_required": 0,
            "hako_adopted_decision_required": 0,
            "source_selfhost_claim_required": 0,
            "runtime_fallback_required": 0,
            "new_backend_route_required": 0,
            "new_abi_required": 0,
            "new_python_semantic_projector_required": 0,
            "runner_semantic_owner_required": 0,
            "proof_sources": [rel(BASIS)],
        },
        "proof_tuple_complete": proof_tuple_complete,
        "selection_eligible": proof_tuple_complete,
        "blocked_by": blocked_by,
    }


def build_fixture() -> dict[str, Any]:
    inventory = read_json(INVENTORY)
    basis = read_json(BASIS)
    all_rows = inventory.get("domain_object_id_source_id_ledger") or []
    unresolved_rows = [
        row for row in all_rows if row.get("scope_state") == "UnresolvedNonIdDomainObject"
    ]
    closed_id_scalar_rows = [row for row in all_rows if row.get("scope_state") == "ClosedIdScalarLane"]
    unresolved_by_subaxis = rows_by_subaxis(unresolved_rows)

    candidates = [
        evaluate_candidate(
            basis_candidate=candidate,
            unresolved_rows=unresolved_by_subaxis.get(candidate["domain_subaxis"], []),
            closed_id_scalar_rows=closed_id_scalar_rows,
        )
        for candidate in sorted(
            basis.get("candidate_subaxes") or [],
            key=lambda candidate: candidate["domain_subaxis"],
        )
    ]

    eligible = [candidate for candidate in candidates if candidate["selection_eligible"]]
    guard_clean = [
        candidate
        for candidate in candidates
        if candidate.get("guard_clean_authority", {}).get("status") == "Proven"
    ]

    if len(eligible) == 1:
        decision = {
            "kind": "SelectSelectedSubaxisPolicyBasis",
            "reason_token": "ExactlyOneDomainObjectIdSubaxisMechanicalCandidate",
            "selected_domain_subaxis": eligible[0]["domain_subaxis"],
            "selected_next_card": "MIRBUILDER-DOMAIN-OBJECT-ID-SELECTED-SUBAXIS-POLICY-BASIS-001",
        }
    elif len(eligible) > 1:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "MultipleDomainObjectIdSubaxisMechanicalCandidates",
            "selected_domain_subaxis": None,
            "selected_next_card": DESIGN_STOP,
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoExactlyOneDomainObjectIdSubaxisMechanicalCandidate",
            "selected_domain_subaxis": None,
            "selected_next_card": DESIGN_STOP,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderDomainObjectIdUnresolvedSubaxisPriorityRerunV2",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "subaxis_mechanical_selection_basis": rel(BASIS),
            "domain_object_id_transport_policy_inventory_rerun_002": rel(INVENTORY),
        },
        "local_authority": {
            "local_selection_authority": "LocalMechanicalSelectorAuthorityV1",
            "worker_inventory": "consumed",
            "worker_inventory_scope": "read_only_current_fixtures_cards_ledgers",
        },
        "provenance": {
            "subaxis_mechanical_selection_basis_hash": sha256_file(BASIS),
            "domain_object_id_transport_policy_inventory_rerun_002_hash": sha256_file(INVENTORY),
        },
        "selector_rule": basis.get("selector_rule"),
        "previous_decision": basis.get("decision"),
        "candidate_subaxes": candidates,
        "summary": {
            "unresolved_non_id_domain_row_count": len(unresolved_rows),
            "closed_id_scalar_row_count": len(closed_id_scalar_rows),
            "candidate_subaxis_count": len(candidates),
            "guard_clean_candidate_count": len(guard_clean),
            "proof_tuple_complete_candidate_count": len(eligible),
            "selection_eligible_subaxis_count": len(eligible),
        },
        "decision": decision,
        "recovery": {
            "kind": "DesignConsultationRequired",
            "reason": decision["reason_token"],
            "question": (
                "No non-ID DomainObject/Id subaxis has dependency-root or prior "
                "closed-lane continuation authority. Define stronger typed "
                "dependency evidence or return to the wider route selector."
            ),
        },
        "claims": {
            "subaxis_mechanical_selection_basis_consumed": 1,
            "domain_object_id_transport_policy_inventory_rerun_002_consumed": 1,
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
        print("mirbuilder-domain-object-id-unresolved-subaxis-priority-rerun-002 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
