#!/usr/bin/env python3
"""Decompose the selected CallLowering projection-policy cluster."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
PRIORITY = FIXTURES / "mirbuilder-projection-policy-cluster-priority-resolution-v0.json"
OUTPUT = FIXTURES / "mirbuilder-call-lowering-policy-subcluster-decomposition-v0.json"
SELECTED_CLUSTER_ID = (
    "projection_policy::UnsupportedDirectShape::shape.call_lowering::"
    "FixtureMapped::CallLoweringCluster"
)


SUBCLUSTERS: dict[str, dict[str, Any]] = {
    "DiagnosticStringHelpers": {
        "symbols": {
            "generate_self_recursion_warning",
            "is_commonly_shadowed_method",
            "suggest_resolution",
        },
        "next_owner_kind": "ProjectionPolicy",
        "next_card": "MIRBUILDER-CALL-LOWERING-DIAGNOSTIC-HELPERS-PROJECTION-POLICY-001",
        "reason_token": "DiagnosticStringHelpersAreParentOwned",
    },
    "BuiltinGlobalFunctionRegistry": {
        "symbols": {"is_builtin_function", "is_math_function"},
        "next_owner_kind": "RegistryDescriptorPolicy",
        "next_card": "MIRBUILDER-CALL-LOWERING-BUILTIN-GLOBAL-FUNCTION-REGISTRY-POLICY-001",
        "reason_token": "BuiltinFunctionNamesRequireRegistryDescriptor",
    },
    "ExternInterfaceRegistry": {
        "symbols": {"is_env_interface", "is_extern_function"},
        "next_owner_kind": "RegistryDescriptorPolicy",
        "next_card": "MIRBUILDER-CALL-LOWERING-EXTERN-INTERFACE-REGISTRY-POLICY-001",
        "reason_token": "ExternInterfaceNamesRequireRegistryDescriptor",
    },
    "StaticReceiverMethodCatalog": {
        "symbols": {"has_method"},
        "next_owner_kind": "RegistryDescriptorPolicy",
        "next_card": "MIRBUILDER-CALL-LOWERING-STATIC-RECEIVER-METHOD-CATALOG-POLICY-001",
        "reason_token": "StaticReceiverMethodsRequireCatalogDescriptor",
    },
    "CallFeaturePredicates": {
        "symbols": {"contains_value_return", "is_pure_method", "is_unified_call_enabled"},
        "next_owner_kind": "ProjectionPolicy",
        "next_card": "MIRBUILDER-CALL-LOWERING-FEATURE-PREDICATES-PROJECTION-POLICY-001",
        "reason_token": "CallFeaturePredicatesNeedSeparatePolicy",
    },
    "CallNameCanonicalizationHelpers": {
        "symbols": {"generate_method_function_name"},
        "next_owner_kind": "ProjectionPolicy",
        "next_card": "MIRBUILDER-CALL-LOWERING-NAME-CANONICALIZATION-PROJECTION-POLICY-001",
        "reason_token": "CallNameCanonicalizationNeedsSeparatePolicy",
    },
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def known_type_transport(item: dict[str, Any]) -> bool:
    ret = item.get("return_type") or ""
    return ret in {"", "bool", "usize", "i64", "String"}


def borrow_axis(item: dict[str, Any]) -> str:
    ret = item.get("return_type") or ""
    params = item.get("params") or ""
    if "&mut" in ret:
        return "ReturnedMutableAliasUnknown"
    if "&" in ret:
        return "BorrowPolicyNeeded"
    if "&mut" in params or "&self" in params:
        return "NoReturnedBorrow"
    return "NoBorrow"


def selected_surfaces(report: dict[str, Any]) -> list[dict[str, Any]]:
    return [
        item for item in report["items"]
        if item.get("classification") == "MissingProjectionPolicy"
        and item.get("shape_signature") == "shape.call_lowering"
        and item.get("likely_owner_cluster") == "CallLoweringCluster"
        and item.get("owner_edge_confidence") == "FixtureMapped"
        and item.get("stable_deny_reason") == "UnsupportedDirectShape"
        and item.get("evidence_refs")
        and borrow_axis(item) == "NoBorrow"
        and known_type_transport(item)
    ]


def subcluster_for_symbol(symbol: str) -> str:
    matches = [
        name for name, definition in SUBCLUSTERS.items()
        if symbol in definition["symbols"]
    ]
    if len(matches) != 1:
        raise SystemExit(f"unclassified or ambiguous CallLowering symbol: {symbol}")
    return matches[0]


def build_decomposition() -> dict[str, Any]:
    report = read_json(REPORT)
    priority = read_json(PRIORITY)
    surfaces = selected_surfaces(report)
    selected_ids = {item["source_id"] for item in surfaces}

    selected_rank = next(
        cluster for cluster in priority["ranked_clusters"]
        if cluster["cluster_id"] == SELECTED_CLUSTER_ID
        and cluster["borrow_axis"] == "NoBorrow"
    )

    subcluster_counts = Counter()
    source_surfaces = []
    for item in surfaces:
        subcluster = subcluster_for_symbol(item["symbol"])
        subcluster_counts[subcluster] += 1
        source_surfaces.append({
            "source_id": item["source_id"],
            "symbol": item["symbol"],
            "source_path": item["source_path"],
            "params": item.get("params") or "",
            "return_type": item.get("return_type") or "",
            "subcluster_id": subcluster,
        })

    subclusters = []
    for name, definition in SUBCLUSTERS.items():
        members = [
            surface for surface in source_surfaces
            if surface["subcluster_id"] == name
        ]
        subclusters.append({
            "subcluster_id": name,
            "source_count": len(members),
            "symbols": [member["symbol"] for member in members],
            "classification": "CallLoweringPolicySubcluster",
            "next_owner_kind": definition["next_owner_kind"],
            "next_card": definition["next_card"],
            "reason_token": definition["reason_token"],
            "selection_eligible": name == "DiagnosticStringHelpers",
        })

    missing = set().union(*(d["symbols"] for d in SUBCLUSTERS.values())) - {
        item["symbol"] for item in surfaces
    }
    if missing:
        raise SystemExit(f"expected CallLowering surfaces missing: {sorted(missing)}")

    return {
        "schema_version": 0,
        "kind": "MirBuilderCallLoweringPolicySubclusterDecompositionV1",
        "token": "MIRBUILDER-CALL-LOWERING-POLICY-SUBCLUSTER-DECOMPOSITION-001",
        "input_state": {
            "source_report": rel(REPORT),
            "priority_resolution": rel(PRIORITY),
            "source_cluster_id": SELECTED_CLUSTER_ID,
            "source_cluster_rank": selected_rank["rank"],
            "source_count": len(surfaces),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "selection_axes": {
            "owner_edge_confidence": "FixtureMapped",
            "stable_deny_reason": "UnsupportedDirectShape",
            "shape_signature": "shape.call_lowering",
            "borrow_axis": "NoBorrow",
            "type_transport_axis": "Known",
            "verifier_or_oracle_state": "Present",
        },
        "source_surfaces": source_surfaces,
        "subclusters": subclusters,
        "subcluster_counts": dict(sorted(subcluster_counts.items())),
        "decomposition_policy": {
            "whole_cluster_projection_policy_selected": False,
            "whole_cluster_keep_parent_owner_selected": False,
            "registry_tables_require_descriptor_fixture": True,
            "diagnostic_helpers_first": True,
            "reason_token": "CallLoweringClusterContainsMixedPolicySurfaces",
        },
        "decision": {
            "kind": "SelectSubclusterProjectionPolicy",
            "selected_subcluster_id": "DiagnosticStringHelpers",
            "selected_next_card": (
                "MIRBUILDER-CALL-LOWERING-DIAGNOSTIC-HELPERS-"
                "PROJECTION-POLICY-001"
            ),
            "reason_token": "SelectDiagnosticHelpersBeforeRegistryDescriptors",
        },
        "claims": {
            "manual_family_selection": 0,
            "whole_cluster_projection_policy": 0,
            "whole_cluster_keep_parent_owner": 0,
            "runtime_or_projection_policy_by_name": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "native_seed_materialization": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
        },
        "provenance": {
            "selected_source_ids": sorted(selected_ids),
            "tool_role": "FactsAdapterGuardOrchestrator",
            "semantic_projection_inference": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in decomposition fixture.")
    args = parser.parse_args()

    output = stable_json(build_decomposition())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-call-lowering-policy-subcluster-decomposition unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
