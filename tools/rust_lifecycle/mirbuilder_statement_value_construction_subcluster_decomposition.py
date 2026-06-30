#!/usr/bin/env python3
"""Decompose the selected StatementValueConstruction projection-policy cluster."""

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
OUTPUT = FIXTURES / "mirbuilder-statement-value-construction-subcluster-decomposition-v0.json"
TOKEN = "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-SUBCLUSTER-DECOMPOSITION-001"
SELECTED_CLUSTER_ID = (
    "projection_policy::UnsupportedDirectShape::shape.statement_value_construction::"
    "FixtureMapped::StatementValueConstructionCluster::borrow=NoReturnedBorrow::"
    "control=StraightLine::type=Known::call=AllKnown::verifier=Present"
)


SUBCLUSTERS: dict[str, dict[str, Any]] = {
    "DiagnosticStringHelpers": {
        "symbols": {"undefined_variable_message"},
        "next_owner_kind": "ProjectionPolicy",
        "next_card": "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-DIAGNOSTIC-HELPERS-PROJECTION-POLICY-001",
        "reason_token": "DiagnosticMessageHelperIsParentOwned",
        "selection_eligible": True,
    },
    "BlockTerminationPredicate": {
        "symbols": {"is_current_block_terminated"},
        "next_owner_kind": "ProjectionPolicy",
        "next_card": "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BLOCK-TERMINATION-PREDICATE-PROJECTION-POLICY-001",
        "reason_token": "BlockTerminationPredicateNeedsSeparatePolicy",
        "selection_eligible": False,
    },
    "BoxFieldInitialization": {
        "symbols": {"build_new_expression_with_field_initializers", "build_box_field_initializers"},
        "next_owner_kind": "ProjectionPolicy",
        "next_card": "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-PROJECTION-POLICY-001",
        "reason_token": "BoxFieldInitializationMutatesObjectFields",
        "selection_eligible": False,
    },
    "RecordValueConstruction": {
        "symbols": {"is_record_constructor_class", "build_record_literal_value", "build_record_update_value"},
        "next_owner_kind": "ProjectionPolicy",
        "next_card": "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-RECORD-VALUE-PROJECTION-POLICY-001",
        "reason_token": "RecordValueConstructionNeedsOwnedFieldPolicy",
        "selection_eligible": False,
    },
    "FreeVariableCollection": {
        "symbols": {"collect_free_vars"},
        "next_owner_kind": "ProjectionPolicy",
        "next_card": "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-FREE-VARIABLE-COLLECTION-PROJECTION-POLICY-001",
        "reason_token": "FreeVariableCollectionUsesExplicitMutableAccumulator",
        "selection_eligible": False,
    },
    "LexicalScopeStack": {
        "symbols": {"push_lexical_scope", "pop_lexical_scope"},
        "next_owner_kind": "ProjectionPolicy",
        "next_card": "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-LEXICAL-SCOPE-STACK-PROJECTION-POLICY-001",
        "reason_token": "LexicalScopeStackMutatesScopeContext",
        "selection_eligible": False,
    },
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def known_type_transport(item: dict[str, Any]) -> bool:
    return (item.get("return_type") or "") in {"", "bool", "usize", "i64", "String"}


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
        and item.get("shape_signature") == "shape.statement_value_construction"
        and item.get("likely_owner_cluster") == "StatementValueConstructionCluster"
        and item.get("owner_edge_confidence") == "FixtureMapped"
        and item.get("stable_deny_reason") == "UnsupportedDirectShape"
        and item.get("evidence_refs")
        and borrow_axis(item) == "NoReturnedBorrow"
        and known_type_transport(item)
    ]


def subcluster_for_symbol(symbol: str) -> str:
    matches = [
        name for name, definition in SUBCLUSTERS.items()
        if symbol in definition["symbols"]
    ]
    if len(matches) != 1:
        raise SystemExit(f"unclassified or ambiguous StatementValueConstruction symbol: {symbol}")
    return matches[0]


def priority_cluster_state(priority: dict[str, Any]) -> dict[str, Any]:
    for cluster in priority.get("ranked_clusters", []):
        if cluster["cluster_id"] == SELECTED_CLUSTER_ID:
            return {
                "state": "SelectedByPriorityResolver",
                "rank": cluster["rank"],
                "reason_token": "ProjectionPolicyClusterSelected",
            }
    for cluster in priority.get("excluded_existing_decision_clusters", []):
        if cluster["cluster_id"] == SELECTED_CLUSTER_ID:
            return {
                "state": "ExcludedAfterDecompositionLanded",
                "rank": None,
                "reason_token": cluster["reason_token"],
            }
    raise SystemExit("StatementValueConstruction selected cluster missing from priority resolver")


def build_decomposition() -> dict[str, Any]:
    report = read_json(REPORT)
    priority = read_json(PRIORITY)
    surfaces = selected_surfaces(report)
    priority_state = priority_cluster_state(priority)

    source_surfaces = []
    subcluster_counts = Counter()
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

    expected_symbols = set().union(*(definition["symbols"] for definition in SUBCLUSTERS.values()))
    actual_symbols = {item["symbol"] for item in surfaces}
    if expected_symbols != actual_symbols:
        raise SystemExit(
            "StatementValueConstruction surface drift: "
            f"missing={sorted(expected_symbols - actual_symbols)} "
            f"extra={sorted(actual_symbols - expected_symbols)}"
        )

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
            "classification": "StatementValueConstructionPolicySubcluster",
            "next_owner_kind": definition["next_owner_kind"],
            "next_card": definition["next_card"],
            "reason_token": definition["reason_token"],
            "selection_eligible": definition["selection_eligible"],
        })

    return {
        "schema_version": 0,
        "kind": "MirBuilderStatementValueConstructionSubclusterDecompositionV1",
        "token": TOKEN,
        "input_state": {
            "source_report": rel(REPORT),
            "priority_resolution": rel(PRIORITY),
            "source_cluster_id": SELECTED_CLUSTER_ID,
            "source_cluster_priority_state": priority_state["state"],
            "source_cluster_priority_reason_token": priority_state["reason_token"],
            "source_cluster_rank": priority_state["rank"],
            "source_count": len(surfaces),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "selection_axes": {
            "owner_edge_confidence": "FixtureMapped",
            "stable_deny_reason": "UnsupportedDirectShape",
            "shape_signature": "shape.statement_value_construction",
            "borrow_axis": "NoReturnedBorrow",
            "type_transport_axis": "Known",
            "verifier_or_oracle_state": "Present",
        },
        "source_surfaces": source_surfaces,
        "subclusters": subclusters,
        "subcluster_counts": dict(sorted(subcluster_counts.items())),
        "decomposition_policy": {
            "whole_cluster_projection_policy_selected": False,
            "whole_cluster_keep_parent_owner_selected": False,
            "diagnostic_helpers_first": True,
            "record_and_field_construction_require_owned_value_policy": True,
            "lexical_scope_stack_requires_scope_context_policy": True,
            "reason_token": "StatementValueConstructionClusterContainsMixedPolicySurfaces",
        },
        "decision": {
            "kind": "SelectSubclusterProjectionPolicy",
            "selected_subcluster_id": "DiagnosticStringHelpers",
            "selected_next_card": "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-DIAGNOSTIC-HELPERS-PROJECTION-POLICY-001",
            "reason_token": "SelectDiagnosticHelpersBeforeMutationAndRecordConstruction",
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
        print("mirbuilder-statement-value-construction-subcluster-decomposition unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
