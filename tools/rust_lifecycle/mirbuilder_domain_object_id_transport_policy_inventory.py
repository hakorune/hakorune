#!/usr/bin/env python3
"""Inventory domain-object/id transport policy axes."""

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


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-domain-object-id-transport-policy-inventory-v0.json"

TOKEN = "MIRBUILDER-DOMAIN-OBJECT-ID-TRANSPORT-POLICY-INVENTORY-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_ID_SCALAR = "MIRBUILDER-ID-SCALAR-DOMAIN-TRANSPORT-POLICY-001"
UNCLASSIFIED_RESOLUTION = FIXTURES / "mirbuilder-carrier-type-transport-unclassified-evidence-resolution-v0.json"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def domain_subaxis(return_type: str) -> str:
    if "ValueId" in return_type or "BasicBlockId" in return_type or return_type in {
        "BindingId",
        "BodyId",
        "SlotId",
        "TypedValueId",
    }:
        return "IdScalarDomainTransportAxis"
    if "ASTNode" in return_type:
        return "AstNodeDomainTransportAxis"
    if return_type.startswith("Mir") or return_type in {
        "CallFlags",
        "CalleeBoxKind",
        "crate::mir::definitions::call_unified::CalleeBoxKind",
        "FlowboxBoxKind",
        "FunctionSignature",
        "MirCall",
        "OperandTypeClass",
        "RefSlotKind",
    }:
        return "MirDomainTransportAxis"
    if "Recipe" in return_type or "Plan" in return_type or return_type in {
        "BlockContractKind",
        "BranchStub",
        "CanonicalLoopFacts",
        "CarrierSets",
        "CondProfile",
        "ControlFlowCounts",
        "CoreExitPlan",
        "CorePhiInfo",
        "EdgeArgs",
        "EdgeStub",
        "EffectMask",
        "GenericLoopV1CarrierState",
        "HandoffTarget",
        "HeaderPredGroups",
        "LoopFeatures",
        "LoopRouteShadowReport",
        "NestedLoopBodyProfile",
        "Route",
        "ScanConditionObservation",
        "StepPlacementDecision",
        "TailCallKind",
        "crate::mir::loop_route_detection::LoopRouteKind",
        "legacy_observer::LoopRouteShadowReport",
    }:
        return "PlanRecipeDomainTransportAxis"
    if return_type in {"LoweringContext", "SpanT", "TypeContextSnapshot"}:
        return "ContextOrSpanDomainTransportAxis"
    return "OtherDomainObjectTransportAxis"


def build_fixture() -> dict[str, Any]:
    resolution = read_json(UNCLASSIFIED_RESOLUTION)
    report = read_json(REPORT)
    items = [
        item for item in report.get("items") or []
        if item.get("classification") == "MissingProjectionPolicy"
        and type_transport_axis(item) == "Missing"
        and lane_for(labels_for_return_type(item.get("return_type") or "<unit>")) == "CarrierTypeTransportEvidenceInventoryRequired"
        and unclassified_axis(item.get("return_type") or "<unit>") == "DomainObjectOrIdTransportAxis"
    ]

    subaxis_counts: Counter[str] = Counter()
    return_type_counts: Counter[str] = Counter()
    rows_sample: list[dict[str, Any]] = []
    for item in items:
        return_type = item.get("return_type") or "<unit>"
        subaxis = domain_subaxis(return_type)
        subaxis_counts[subaxis] += 1
        return_type_counts[return_type] += 1
        if len(rows_sample) < 60:
            rows_sample.append(
                {
                    "source_id": item["source_id"],
                    "return_type": return_type,
                    "domain_subaxis": subaxis,
                    "known_owner_edge": item.get("known_owner_edge"),
                    "shape_signature": item.get("shape_signature"),
                }
            )

    decision = {
        "kind": "SelectIdScalarDomainTransportPolicy",
        "reason_token": "IdScalarDomainTransportClosestToExistingScalarTransport",
        "selected_subaxis": "IdScalarDomainTransportAxis",
        "selected_next_card": NEXT_ID_SCALAR,
    }
    if not subaxis_counts.get("IdScalarDomainTransportAxis"):
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoIdScalarDomainTransportAxisAvailable",
            "selected_subaxis": None,
            "selected_next_card": DESIGN_STOP,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderDomainObjectIdTransportPolicyInventoryV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "unclassified_evidence_resolution": rel(UNCLASSIFIED_RESOLUTION),
            "unconverted_surface_report": rel(REPORT),
        },
        "provenance": {
            "unclassified_evidence_resolution_hash": sha256_file(UNCLASSIFIED_RESOLUTION),
            "unconverted_surface_report_hash": sha256_file(REPORT),
        },
        "input_decision": resolution.get("decision"),
        "domain_rows_sample": rows_sample,
        "summary": {
            "domain_object_id_input_count": len(items),
            "domain_subaxis_counts": dict(sorted(subaxis_counts.items())),
            "top_return_type_counts": dict(return_type_counts.most_common(30)),
        },
        "selection_rule": {
            "id_scalar_domain_transport_reuses_scalar_transport_precedent": True,
            "domain_object_count_as_proof": False,
            "manual_subaxis_selection": False,
            "object_layout_policy_not_selected": True,
        },
        "decision": decision,
        "claims": {
            "unclassified_evidence_resolution_consumed": 1,
            "domain_object_id_transport_inventory_ready": 1,
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
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
        print("mirbuilder-domain-object-id-transport-policy-inventory unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
