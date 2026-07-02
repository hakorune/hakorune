#!/usr/bin/env python3
"""Resolve priority for unresolved non-ID DomainObject/Id transport subaxes."""

from __future__ import annotations

import argparse
import json
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-domain-object-id-unresolved-subaxis-priority-resolution-v0.json"

TOKEN = "MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RESOLUTION-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
INPUT = FIXTURES / "mirbuilder-domain-object-id-transport-policy-inventory-rerun-002-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    inventory = read_json(INPUT)
    rows = [
        row
        for row in inventory.get("domain_object_id_source_id_ledger") or []
        if row.get("scope_state") == "UnresolvedNonIdDomainObject"
    ]

    by_subaxis: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        by_subaxis[row["domain_subaxis"]].append(row)

    candidates: list[dict[str, Any]] = []
    for subaxis in sorted(by_subaxis):
        subaxis_rows = sorted(by_subaxis[subaxis], key=lambda row: row["source_id"])
        owner_counts = Counter(row.get("known_owner_edge") or "<none>" for row in subaxis_rows)
        candidates.append(
            {
                "domain_subaxis": subaxis,
                "row_count": len(subaxis_rows),
                "owner_edge_counts": dict(sorted(owner_counts.items())),
                "sample_source_ids": [row["source_id"] for row in subaxis_rows[:12]],
                "machine_priority_authority": "Unproven",
                "selection_eligible": False,
                "blocked_by": [
                    "NoDependencyRootAuthority",
                    "NoPriorClosedLaneConsumptionAuthority",
                    "NoExactlyOneGuardCleanCandidate",
                    "RowCountIsDiagnosticOnly",
                ],
            }
        )

    decision = {
        "kind": "KeepStopped",
        "reason_token": "NoMachineDerivedDomainObjectIdUnresolvedSubaxisPriority",
        "selected_domain_subaxis": None,
        "selected_next_card": DESIGN_STOP,
    }

    return {
        "schema_version": 0,
        "kind": "MirBuilderDomainObjectIdUnresolvedSubaxisPriorityResolutionV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "domain_object_id_transport_policy_inventory_rerun_002": rel(INPUT),
        },
        "local_authority": {
            "local_selection_authority": "LocalMechanicalSelectorAuthorityV1",
            "worker_inventory": "consumed",
            "worker_inventory_scope": "read_only_current_fixtures_cards_ledgers",
        },
        "provenance": {
            "domain_object_id_transport_policy_inventory_rerun_002_hash": sha256_file(INPUT),
        },
        "input_decision": inventory.get("decision"),
        "candidate_subaxes": candidates,
        "summary": {
            "unresolved_non_id_domain_row_count": len(rows),
            "candidate_subaxis_count": len(candidates),
            "selection_eligible_subaxis_count": 0,
            "domain_subaxis_counts": {
                candidate["domain_subaxis"]: candidate["row_count"] for candidate in candidates
            },
        },
        "selection_rule": {
            "subaxis_priority_requires_machine_authority": True,
            "row_count_is_diagnostic_only": True,
            "owner_name_as_proof": False,
            "route_membership_alone_as_proof": False,
            "manual_subaxis_selection": False,
            "design_consultation_required_if_no_machine_authority": True,
        },
        "decision": decision,
        "recovery": {
            "kind": "DesignConsultationRequired",
            "reason": "NoMachineDerivedDomainObjectIdUnresolvedSubaxisPriority",
            "question": "Which non-ID DomainObject/Id subaxis may define the next policy basis without using row count, owner name, route membership, or convenience as proof?",
        },
        "claims": {
            "domain_object_id_transport_policy_inventory_rerun_002_consumed": 1,
            "unresolved_subaxis_priority_resolution_ready": 1,
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
            "manual_subaxis_selection": 0,
            "row_count_as_proof": 0,
            "owner_name_as_proof": 0,
            "domain_object_count_as_proof": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
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
        print("mirbuilder-domain-object-id-unresolved-subaxis-priority-resolution unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
