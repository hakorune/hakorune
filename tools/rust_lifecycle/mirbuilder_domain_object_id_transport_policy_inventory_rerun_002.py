#!/usr/bin/env python3
"""Rerun DomainObject/Id transport inventory after carrier evidence rerun 003."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed
from mirbuilder_crate_wide_missing_projection_policy_cluster_resolution import type_transport_axis
from mirbuilder_carrier_type_transport_policy_inventory_rerun_003 import labels_for_return_type, lane_for
from mirbuilder_carrier_type_transport_unclassified_evidence_resolution import unclassified_axis
from mirbuilder_domain_object_id_transport_policy_inventory import domain_subaxis


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-domain-object-id-transport-policy-inventory-rerun-002-v0.json"

TOKEN = "MIRBUILDER-DOMAIN-OBJECT-ID-TRANSPORT-POLICY-INVENTORY-RERUN-002"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_NEW_ID_SCALAR = "MIRBUILDER-ID-SCALAR-NEWLY-UNCOVERED-DOMAIN-TRANSPORT-RESOLUTION-001"
NEXT_UNRESOLVED = "MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RESOLUTION-001"

UNCLASSIFIED_RESOLUTION = FIXTURES / "mirbuilder-carrier-type-transport-unclassified-evidence-resolution-002-v0.json"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
ID_SCALAR_DIRECTABILITY = FIXTURES / "mirbuilder-id-scalar-domain-transport-directability-rerun-v0.json"
EMISSION_ADOPTION = FIXTURES / "mirbuilder-emission-ssa-phi-hako-adoption-decision-v0.json"
CONTEXT_PARENT_OWNED = FIXTURES / "mirbuilder-id-scalar-parent-owned-subject-boundary-resolution-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def domain_object_id_rows(report: dict[str, Any]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for item in report.get("items") or []:
        return_type = item.get("return_type") or "<unit>"
        if (
            item.get("classification") == "MissingProjectionPolicy"
            and type_transport_axis(item) == "Missing"
            and lane_for(labels_for_return_type(return_type)) == "CarrierTypeTransportEvidenceInventoryRequired"
            and unclassified_axis(return_type) == "DomainObjectOrIdTransportAxis"
        ):
            subaxis = domain_subaxis(return_type)
            rows.append(
                {
                    "source_id": item["source_id"],
                    "return_type": return_type,
                    "domain_subaxis": subaxis,
                    "known_owner_edge": item.get("known_owner_edge"),
                    "owner_edge_confidence": item.get("owner_edge_confidence"),
                    "shape_signature": item.get("shape_signature"),
                }
            )
    return sorted(rows, key=lambda row: row["source_id"])


def directability_by_source_id(directability: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {
        row["source_id"]: row
        for row in directability.get("rerun_rows") or []
        if row.get("source_id")
    }


def build_fixture() -> dict[str, Any]:
    resolution = read_json(UNCLASSIFIED_RESOLUTION)
    report = read_json(REPORT)
    directability = read_json(ID_SCALAR_DIRECTABILITY)

    domain_rows = domain_object_id_rows(report)
    directability_rows = directability_by_source_id(directability)
    previous_id_scalar_ids = set(directability_rows)
    current_id_scalar_ids = {
        row["source_id"]
        for row in domain_rows
        if row["domain_subaxis"] == "IdScalarDomainTransportAxis"
    }

    overlap_ids = current_id_scalar_ids & previous_id_scalar_ids
    new_id_scalar_ids = current_id_scalar_ids - previous_id_scalar_ids
    previous_missing_ids = previous_id_scalar_ids - current_id_scalar_ids

    ledger: list[dict[str, Any]] = []
    for row in domain_rows:
        source_id = row["source_id"]
        is_id_scalar = row["domain_subaxis"] == "IdScalarDomainTransportAxis"
        prior = directability_rows.get(source_id) if is_id_scalar else None
        if is_id_scalar and prior:
            scope_state = "ClosedIdScalarLane"
        elif is_id_scalar:
            scope_state = "NewlyUncoveredIdScalar"
        else:
            scope_state = "UnresolvedNonIdDomainObject"

        ledger.append(
            {
                **row,
                "scope_state": scope_state,
                "previous_id_scalar_directability_state": prior.get("directability_state") if prior else None,
                "previous_id_scalar_owner_edge": prior.get("known_owner_edge") if prior else None,
            }
        )

    subaxis_counts = Counter(row["domain_subaxis"] for row in domain_rows)
    unresolved_counts = Counter(
        row["domain_subaxis"]
        for row in ledger
        if row["scope_state"] == "UnresolvedNonIdDomainObject"
    )
    scope_counts = Counter(row["scope_state"] for row in ledger)

    if new_id_scalar_ids:
        decision = {
            "kind": "SelectNewlyUncoveredIdScalarResolution",
            "reason_token": "NewIdScalarDomainRowsAfterClosedLaneConsumption",
            "selected_next_card": NEXT_NEW_ID_SCALAR,
        }
    elif scope_counts.get("UnresolvedNonIdDomainObject", 0) > 0:
        decision = {
            "kind": "SelectUnresolvedSubaxisPriorityResolution",
            "reason_token": "ClosedIdScalarLaneConsumedAndNonIdDomainRowsRemain",
            "selected_next_card": NEXT_UNRESOLVED,
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoUnresolvedDomainObjectIdTransportRows",
            "selected_next_card": DESIGN_STOP,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderDomainObjectIdTransportPolicyInventoryRerunV2",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "carrier_type_transport_unclassified_evidence_resolution_002": rel(UNCLASSIFIED_RESOLUTION),
            "unconverted_surface_report": rel(REPORT),
            "previous_id_scalar_directability_rerun": rel(ID_SCALAR_DIRECTABILITY),
            "emission_ssa_phi_adoption": rel(EMISSION_ADOPTION),
            "context_registry_parent_owned_boundary": rel(CONTEXT_PARENT_OWNED),
        },
        "local_authority": {
            "local_selection_authority": "LocalMechanicalSelectorAuthorityV1",
            "worker_inventory": "consumed",
            "worker_inventory_scope": "read_only_current_fixtures_cards_ledgers",
        },
        "provenance": {
            "carrier_type_transport_unclassified_evidence_resolution_002_hash": sha256_file(UNCLASSIFIED_RESOLUTION),
            "unconverted_surface_report_hash": sha256_file(REPORT),
            "previous_id_scalar_directability_rerun_hash": sha256_file(ID_SCALAR_DIRECTABILITY),
            "emission_ssa_phi_adoption_hash": sha256_file(EMISSION_ADOPTION),
            "context_registry_parent_owned_boundary_hash": sha256_file(CONTEXT_PARENT_OWNED),
        },
        "input_decision": resolution.get("decision"),
        "domain_object_id_source_id_ledger": ledger,
        "closed_id_scalar_lane": {
            "previous_id_scalar_directability_row_count": len(previous_id_scalar_ids),
            "current_id_scalar_row_count": len(current_id_scalar_ids),
            "id_scalar_source_id_overlap_with_previous_directability_rerun": len(overlap_ids),
            "new_id_scalar_source_ids": sorted(new_id_scalar_ids),
            "previous_id_scalar_source_ids_missing_from_current": sorted(previous_missing_ids),
            "closed_id_scalar_lane_consumed": len(new_id_scalar_ids) == 0
            and len(previous_missing_ids) == 0
            and len(overlap_ids) == len(current_id_scalar_ids) == len(previous_id_scalar_ids),
        },
        "summary": {
            "domain_object_id_input_count": len(domain_rows),
            "domain_subaxis_counts": dict(sorted(subaxis_counts.items())),
            "scope_state_counts": dict(sorted(scope_counts.items())),
            "closed_id_scalar_row_count": scope_counts.get("ClosedIdScalarLane", 0),
            "new_id_scalar_source_id_count": len(new_id_scalar_ids),
            "unresolved_non_id_domain_row_count": scope_counts.get("UnresolvedNonIdDomainObject", 0),
            "unresolved_non_id_domain_subaxis_counts": dict(sorted(unresolved_counts.items())),
        },
        "selection_rule": {
            "full_source_id_ledger_required": True,
            "closed_id_scalar_lane_must_be_partitioned_before_subaxis_priority": True,
            "id_scalar_reselection_forbidden_when_closed_lane_matches_previous_source_ids": True,
            "manual_subaxis_selection": False,
            "return_type_count_as_proof": False,
            "domain_object_count_as_proof": False,
        },
        "decision": decision,
        "claims": {
            "carrier_type_transport_unclassified_evidence_resolution_002_consumed": 1,
            "domain_object_id_transport_inventory_rerun_ready": 1,
            "full_source_id_ledger_present": 1,
            "closed_id_scalar_lane_consumed": 1
            if not new_id_scalar_ids and not previous_missing_ids and current_id_scalar_ids
            else 0,
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
            "manual_subaxis_selection": 0,
            "return_type_count_as_proof": 0,
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
        print("mirbuilder-domain-object-id-transport-policy-inventory-rerun-002 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
