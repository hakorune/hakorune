#!/usr/bin/env python3
"""Resolve CallLowering feature predicates projection policy."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
DECOMPOSITION = FIXTURES / "mirbuilder-call-lowering-policy-subcluster-decomposition-v0.json"
STATIC_CATALOG_POLICY = FIXTURES / "mirbuilder-call-lowering-static-receiver-method-catalog-policy-v0.json"
OUTPUT = FIXTURES / "mirbuilder-call-lowering-feature-predicates-projection-policy-v0.json"
SUBCLUSTER_ID = "CallFeaturePredicates"


FEATURE_SUBCLUSTERS: dict[str, dict[str, Any]] = {
    "UnifiedCallModeGate": {
        "symbols": {"is_unified_call_enabled"},
        "next_owner_kind": "ProjectionPolicy",
        "next_card": "MIRBUILDER-CALL-LOWERING-UNIFIED-CALL-MODE-GATE-PROJECTION-POLICY-001",
        "reason_token": "UnifiedCallModeGateReadsConfig",
    },
    "PureMethodCatalog": {
        "symbols": {"is_pure_method"},
        "next_owner_kind": "RegistryDescriptorPolicy",
        "next_card": "MIRBUILDER-CALL-LOWERING-PURE-METHOD-CATALOG-POLICY-001",
        "reason_token": "PureMethodPredicateRequiresCatalogDescriptor",
    },
    "ValueReturnAstScan": {
        "symbols": {"contains_value_return"},
        "next_owner_kind": "ProjectionPolicy",
        "next_card": "MIRBUILDER-CALL-LOWERING-VALUE-RETURN-AST-SCAN-PROJECTION-POLICY-001",
        "reason_token": "ValueReturnScanRequiresAstTraversalPolicy",
    },
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_source(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def feature_subcluster_for(symbol: str) -> str:
    matches = [
        name for name, definition in FEATURE_SUBCLUSTERS.items()
        if symbol in definition["symbols"]
    ]
    if len(matches) != 1:
        raise SystemExit(f"unclassified or ambiguous CallFeaturePredicates symbol: {symbol}")
    return matches[0]


def source_markers_for(symbol: str, source_text: str) -> list[str]:
    markers_by_symbol = {
        "is_unified_call_enabled": [
            "builder_unified_call_mode",
            "default ON during development; explicit opt-out supported",
        ],
        "is_pure_method": [
            "StringBox",
            "IntegerBox",
            "FloatBox",
            "BoolBox",
        ],
        "contains_value_return": [
            "ASTNode::Return { value: Some(_), .. }",
            "ASTNode::If",
            "ASTNode::Loop",
            "ASTNode::TryCatch",
            "ASTNode::Program",
            "ASTNode::ScopeBox",
            "ASTNode::FunctionDeclaration",
        ],
    }
    return [marker for marker in markers_by_symbol[symbol] if marker in source_text]


def build_policy() -> dict[str, Any]:
    decomposition = read_json(DECOMPOSITION)
    surfaces = [
        surface for surface in decomposition["source_surfaces"]
        if surface["subcluster_id"] == SUBCLUSTER_ID
    ]
    if {surface["symbol"] for surface in surfaces} != {
        "contains_value_return",
        "is_pure_method",
        "is_unified_call_enabled",
    }:
        raise SystemExit(f"unexpected feature predicate surfaces: {surfaces}")

    source_surfaces = []
    subcluster_counts = {name: 0 for name in FEATURE_SUBCLUSTERS}
    for surface in surfaces:
        subcluster = feature_subcluster_for(surface["symbol"])
        subcluster_counts[subcluster] += 1
        source_text = read_source(surface["source_path"])
        source_surfaces.append({
            "source_id": surface["source_id"],
            "symbol": surface["symbol"],
            "source_path": surface["source_path"],
            "params": surface["params"],
            "return_type": surface["return_type"],
            "feature_subcluster_id": subcluster,
            "source_markers": source_markers_for(surface["symbol"], source_text),
        })

    subclusters = []
    for name, definition in FEATURE_SUBCLUSTERS.items():
        members = [
            surface for surface in source_surfaces
            if surface["feature_subcluster_id"] == name
        ]
        subclusters.append({
            "feature_subcluster_id": name,
            "source_count": len(members),
            "symbols": [member["symbol"] for member in members],
            "classification": "CallFeaturePredicateSubcluster",
            "next_owner_kind": definition["next_owner_kind"],
            "next_card": definition["next_card"],
            "reason_token": definition["reason_token"],
            "selection_eligible": name == "UnifiedCallModeGate",
        })

    return {
        "schema_version": 0,
        "kind": "MirBuilderCallLoweringFeaturePredicatesProjectionPolicyV1",
        "token": "MIRBUILDER-CALL-LOWERING-FEATURE-PREDICATES-PROJECTION-POLICY-001",
        "input_state": {
            "subcluster_decomposition": rel(DECOMPOSITION),
            "previous_policy": rel(STATIC_CATALOG_POLICY),
            "selected_subcluster_id": SUBCLUSTER_ID,
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
        "feature_subclusters": subclusters,
        "feature_subcluster_counts": {
            name: subcluster_counts[name] for name in sorted(subcluster_counts)
        },
        "decomposition_policy": {
            "whole_feature_predicate_projection_selected": False,
            "mixed_responsibility_reason": (
                "ConfigGateCatalogPredicateAndAstTraversalMustNotShareOneProjectionPolicy"
            ),
            "unified_call_mode_gate_first": True,
        },
        "decision": {
            "kind": "SelectFeaturePredicateSubcluster",
            "selected_feature_subcluster_id": "UnifiedCallModeGate",
            "selected_next_card": (
                "MIRBUILDER-CALL-LOWERING-UNIFIED-CALL-MODE-GATE-"
                "PROJECTION-POLICY-001"
            ),
            "reason_token": "SelectConfigGateBeforeCatalogAndAstTraversal",
        },
        "claims": {
            "manual_family_selection": 0,
            "whole_feature_predicate_projection": 0,
            "projection_surface_selected": 0,
            "registry_descriptor_selected": 0,
            "ast_traversal_projection_selected": 0,
            "ad_hoc_by_name_policy": 0,
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
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in policy fixture.")
    args = parser.parse_args()

    output = stable_json(build_policy())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-call-lowering-feature-predicates-projection-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
