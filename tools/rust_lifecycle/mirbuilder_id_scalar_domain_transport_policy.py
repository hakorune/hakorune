#!/usr/bin/env python3
"""Define nominal scalar transport for MirBuilder ID domain types."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed
from mirbuilder_crate_wide_missing_projection_policy_cluster_resolution import type_transport_axis
from mirbuilder_carrier_type_transport_policy_inventory_rerun_002 import labels_for_return_type, lane_for
from mirbuilder_carrier_type_transport_unclassified_evidence_resolution import unclassified_axis
from mirbuilder_domain_object_id_transport_policy_inventory import domain_subaxis


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-domain-transport-policy-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-DOMAIN-TRANSPORT-POLICY-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT = "MIRBUILDER-ID-SCALAR-DOMAIN-TRANSPORT-DIRECTABILITY-RERUN-001"
INVENTORY = FIXTURES / "mirbuilder-domain-object-id-transport-policy-inventory-v0.json"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
NEWTYPE_ID_PRECEDENT = ROOT / "docs/development/current/main/phases/phase-296x/296x-1679-NEWTYPE-ID-GENERATOR-SCALARIZATION-001.md"

NOMINAL_TRANSPORTS = {
    "BasicBlockId": "BasicBlockIdAsI64",
    "BindingId": "BindingIdAsI64",
    "BodyId": "BodyIdAsI64",
    "SlotId": "SlotIdAsI64",
    "TypedValueId": "TypedValueIdAsI64",
    "ValueId": "ValueIdAsI64",
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def canonical_id_type(return_type: str) -> str | None:
    short = return_type.rsplit("::", 1)[-1]
    return short if short in NOMINAL_TRANSPORTS else None


def id_scalar_rows(report: dict[str, Any]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for item in report.get("items") or []:
        return_type = item.get("return_type") or "<unit>"
        if (
            item.get("classification") == "MissingProjectionPolicy"
            and type_transport_axis(item) == "Missing"
            and lane_for(labels_for_return_type(return_type)) == "CarrierTypeTransportEvidenceInventoryRequired"
            and unclassified_axis(return_type) == "DomainObjectOrIdTransportAxis"
            and domain_subaxis(return_type) == "IdScalarDomainTransportAxis"
        ):
            id_type = canonical_id_type(return_type)
            rows.append(
                {
                    "source_id": item["source_id"],
                    "return_type": return_type,
                    "canonical_id_type": id_type,
                    "nominal_transport": NOMINAL_TRANSPORTS.get(id_type or ""),
                    "known_owner_edge": item.get("known_owner_edge"),
                    "owner_edge_confidence": item.get("owner_edge_confidence"),
                    "shape_signature": item.get("shape_signature"),
                    "policy_state": "IdScalarDomainTransportSelected" if id_type else "UnsupportedIdScalarDomainType",
                }
            )
    return rows


def build_fixture() -> dict[str, Any]:
    inventory = read_json(INVENTORY)
    report = read_json(REPORT)
    rows = id_scalar_rows(report)

    type_counts = Counter(row["canonical_id_type"] or "<unsupported>" for row in rows)
    transport_counts = Counter(row["nominal_transport"] or "<unsupported>" for row in rows)
    owner_confidence_counts = Counter(row["owner_edge_confidence"] or "None" for row in rows)
    unsupported_count = sum(1 for row in rows if not row["canonical_id_type"])
    policy_ready = bool(rows) and unsupported_count == 0

    decision = {
        "kind": "SelectIdScalarDomainTransportDirectabilityRerun",
        "reason_token": "NominalIdScalarTransportPolicyDefined",
        "selected_next_card": NEXT,
    } if policy_ready else {
        "kind": "KeepStopped",
        "reason_token": "IdScalarDomainTransportPolicyEvidenceIncomplete",
        "selected_next_card": DESIGN_STOP,
    }

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarDomainTransportPolicyV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "domain_object_id_transport_policy_inventory": rel(INVENTORY),
            "unconverted_surface_report": rel(REPORT),
            "newtype_id_scalarization_precedent": rel(NEWTYPE_ID_PRECEDENT),
        },
        "provenance": {
            "domain_object_id_transport_policy_inventory_hash": sha256_file(INVENTORY),
            "unconverted_surface_report_hash": sha256_file(REPORT),
            "newtype_id_scalarization_precedent_hash": sha256_file(NEWTYPE_ID_PRECEDENT),
        },
        "input_decision": inventory.get("decision"),
        "selected_policy": {
            "policy_id": "NominalIdScalarDomainTransportV1",
            "physical_lane": "i64",
            "semantic_transport_is_nominal": True,
            "raw_i64_interchangeability": False,
            "object_layout_transport": False,
            "generator_object_transport": False,
            "invalid_sentinel_semantics": False,
            "reserved_id_policy": False,
            "hako_generation": False,
        },
        "nominal_transports": dict(sorted(NOMINAL_TRANSPORTS.items())),
        "policy_rows": rows,
        "summary": {
            "id_scalar_input_count": len(rows),
            "canonical_id_type_counts": dict(sorted(type_counts.items())),
            "nominal_transport_counts": dict(sorted(transport_counts.items())),
            "owner_edge_confidence_counts": dict(sorted(owner_confidence_counts.items())),
            "unsupported_id_scalar_type_count": unsupported_count,
            "id_scalar_domain_transport_policy_ready": 1 if policy_ready else 0,
        },
        "decision": decision,
        "claims": {
            "domain_object_id_transport_policy_inventory_consumed": 1,
            "id_scalar_domain_transport_policy_defined": 1 if policy_ready else 0,
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
            "domain_object_count_as_proof": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "raw_i64_interchangeability": 0,
            "object_layout_transport": 0,
            "generator_object_transport": 0,
            "invalid_sentinel_semantics": 0,
            "reserved_id_policy": 0,
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
        print("mirbuilder-id-scalar-domain-transport-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
