#!/usr/bin/env python3
"""Rerun non-ID DomainObject/Id subaxis priority using typed dependency roots."""

from __future__ import annotations

import argparse
import json
from collections import defaultdict, deque
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-domain-object-id-unresolved-subaxis-priority-rerun-003-v0.json"

TOKEN = "MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RERUN-003"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
SELECTED_POLICY_BASIS = "MIRBUILDER-DOMAIN-OBJECT-ID-SELECTED-SUBAXIS-POLICY-BASIS-001"

BASIS = FIXTURES / "mirbuilder-domain-object-id-typed-dependency-root-authority-basis-v0.json"
RERUN_002 = FIXTURES / "mirbuilder-domain-object-id-unresolved-subaxis-priority-rerun-002-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def reachable(graph: dict[str, set[str]], start: str, target: str) -> bool:
    seen: set[str] = set()
    queue: deque[str] = deque([start])
    while queue:
        node = queue.popleft()
        if node == target:
            return True
        if node in seen:
            continue
        seen.add(node)
        queue.extend(sorted(graph.get(node, set()) - seen))
    return False


def has_cycle(graph: dict[str, set[str]], nodes: set[str]) -> bool:
    visiting: set[str] = set()
    visited: set[str] = set()

    def visit(node: str) -> bool:
        if node in visiting:
            return True
        if node in visited:
            return False
        visiting.add(node)
        for next_node in graph.get(node, set()):
            if visit(next_node):
                return True
        visiting.remove(node)
        visited.add(node)
        return False

    return any(visit(node) for node in sorted(nodes) if node not in visited)


def evaluate_edges(
    *,
    basis: dict[str, Any],
    candidate_set: set[str],
) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    accepted_kinds = set(basis.get("accepted_edge_evidence_kinds") or [])
    accepted_edges: list[dict[str, Any]] = []
    rejected_edges: list[dict[str, Any]] = []
    for edge in basis.get("typed_dependency_edges") or []:
        from_axis = edge.get("from_domain_subaxis")
        to_axis = edge.get("to_domain_subaxis")
        evidence_kind = edge.get("evidence_kind")
        errors: list[str] = []
        if from_axis not in candidate_set:
            errors.append("from_domain_subaxis_not_candidate")
        if to_axis not in candidate_set:
            errors.append("to_domain_subaxis_not_candidate")
        if from_axis == to_axis:
            errors.append("self_dependency_edge")
        if edge.get("edge_direction") != "dependent_subaxis_requires_prerequisite_subaxis":
            errors.append("bad_edge_direction")
        if evidence_kind not in accepted_kinds:
            errors.append("unsupported_evidence_kind")
        if edge.get("typed_reference_is_concrete") != 1:
            errors.append("typed_reference_not_concrete")
        if not edge.get("proof_source"):
            errors.append("missing_proof_source")

        if errors:
            rejected = dict(edge)
            rejected["accepted"] = False
            rejected["rejected_reason"] = ",".join(errors)
            rejected_edges.append(rejected)
        else:
            accepted = dict(edge)
            accepted["accepted"] = True
            accepted["rejected_reason"] = None
            accepted_edges.append(accepted)
    return accepted_edges, rejected_edges


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    rerun_002 = read_json(RERUN_002)
    candidate_names = sorted(
        candidate["domain_subaxis"] for candidate in basis.get("candidate_subaxes") or []
    )
    candidate_set = set(candidate_names)
    accepted_edges, rejected_edges = evaluate_edges(basis=basis, candidate_set=candidate_set)

    graph: dict[str, set[str]] = defaultdict(set)
    reverse_graph: dict[str, set[str]] = defaultdict(set)
    for edge in accepted_edges:
        from_axis = edge["from_domain_subaxis"]
        to_axis = edge["to_domain_subaxis"]
        graph[from_axis].add(to_axis)
        reverse_graph[to_axis].add(from_axis)

    cycle = has_cycle(graph, candidate_set)
    participating = {
        node for node in candidate_names if graph.get(node) or reverse_graph.get(node)
    }
    root_candidates: list[str] = []
    candidate_rows: list[dict[str, Any]] = []
    for name in candidate_names:
        outgoing = sorted(graph.get(name, set()))
        incoming = sorted(reverse_graph.get(name, set()))
        is_isolated = name not in participating
        all_participants_depend = (
            bool(participating)
            and not is_isolated
            and all(name == node or reachable(graph, node, name) for node in participating)
        )
        is_root = (
            bool(accepted_edges)
            and not cycle
            and not rejected_edges
            and bool(incoming)
            and not outgoing
            and all_participants_depend
        )
        if is_root:
            root_candidates.append(name)
        candidate_rows.append(
            {
                "domain_subaxis": name,
                "guard_clean_authority": "Proven",
                "dependency_participation": (
                    "Isolated"
                    if is_isolated
                    else ("PrerequisiteRootCandidate" if is_root else "DependentOrIntermediate")
                ),
                "incoming_dependency_edge_count": len(incoming),
                "outgoing_dependency_edge_count": len(outgoing),
                "outgoing_unresolved_prerequisite_count": len(outgoing),
                "depends_on_candidate_set": outgoing,
                "candidate_depended_on_by_set": incoming,
                "dependency_root_authority": {
                    "status": "Proven" if is_root and len(root_candidates) == 1 else "Unproven",
                    "positive_dependent_count": len(incoming),
                    "all_non_isolated_candidates_depend_on_candidate": int(all_participants_depend),
                    "unique_root": 0,
                    "proof_sources": [rel(BASIS)] if is_root else [],
                },
                "proof_tuple_complete": False,
                "selection_eligible": False,
            }
        )

    unique_root = len(root_candidates) == 1
    for row in candidate_rows:
        if row["domain_subaxis"] in root_candidates and unique_root:
            row["dependency_root_authority"]["unique_root"] = 1
            row["proof_tuple_complete"] = True
            row["selection_eligible"] = True

    if cycle:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "DomainObjectIdSubaxisDependencyCycleUnresolved",
            "selected_domain_subaxis": None,
            "selected_next_card": DESIGN_STOP,
        }
    elif len(root_candidates) > 1:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "MultipleDomainObjectIdTypedDependencyRootCandidates",
            "selected_domain_subaxis": None,
            "selected_next_card": DESIGN_STOP,
        }
    elif unique_root:
        decision = {
            "kind": "SelectSelectedSubaxisPolicyBasis",
            "reason_token": "ExactlyOneDomainObjectIdTypedDependencyRootCandidate",
            "selected_domain_subaxis": root_candidates[0],
            "selected_next_card": SELECTED_POLICY_BASIS,
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoMachineDerivedDomainObjectIdTypedDependencyRootAuthority",
            "selected_domain_subaxis": None,
            "selected_next_card": DESIGN_STOP,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderDomainObjectIdUnresolvedSubaxisPriorityRerunV3",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "typed_dependency_root_authority_basis": rel(BASIS),
            "unresolved_subaxis_priority_rerun_002": rel(RERUN_002),
        },
        "provenance": {
            "typed_dependency_root_authority_basis_hash": sha256_file(BASIS),
            "unresolved_subaxis_priority_rerun_002_hash": sha256_file(RERUN_002),
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
        },
        "selector_rule": basis.get("selector_rule"),
        "typed_dependency_edges": {
            "accepted": accepted_edges,
            "rejected": rejected_edges,
        },
        "candidate_subaxes": candidate_rows,
        "graph_summary": {
            "candidate_subaxis_count": len(candidate_names),
            "accepted_typed_dependency_edge_count": len(accepted_edges),
            "rejected_edge_count": len(rejected_edges),
            "ambiguous_edge_count": 0,
            "forbidden_edge_source_count": 0,
            "cycle_count": 1 if cycle else 0,
            "dependency_root_candidate_count": len(root_candidates),
            "guard_clean_candidate_count": rerun_002.get("summary", {}).get(
                "guard_clean_candidate_count"
            ),
            "proof_tuple_complete_candidate_count": 1 if unique_root else 0,
            "selection_eligible_subaxis_count": 1 if unique_root else 0,
        },
        "decision": decision,
        "claims": {
            "typed_dependency_root_authority_basis_consumed": 1,
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
        print("mirbuilder-domain-object-id-unresolved-subaxis-priority-rerun-003 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
