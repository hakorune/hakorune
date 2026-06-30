#!/usr/bin/env python3
"""Resolve CallLowering value-return AST scan projection policy."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
PURE_METHOD_POLICY = FIXTURES / "mirbuilder-call-lowering-pure-method-catalog-policy-v0.json"
FEATURE_POLICY = FIXTURES / "mirbuilder-call-lowering-feature-predicates-projection-policy-v0.json"
OUTPUT = FIXTURES / "mirbuilder-call-lowering-value-return-ast-scan-projection-policy-v0.json"
SUBCLUSTER_ID = "ValueReturnAstScan"

AST_VARIANT_RE = re.compile(r"ASTNode::([A-Za-z_][A-Za-z0-9_]*)")


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_source(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def ast_variants(source_text: str) -> list[str]:
    return sorted(set(AST_VARIANT_RE.findall(source_text)))


def recursion_markers(source_text: str) -> list[str]:
    markers = [
        "contains_value_return(then_body)",
        "contains_value_return(body)",
        "contains_value_return(try_body)",
        "contains_value_return(&clause.body)",
        "contains_value_return(statements)",
        "nodes.iter().any(node_has_value_return)",
    ]
    return [marker for marker in markers if marker in source_text]


def function_text(source_text: str, symbol: str) -> str:
    marker = f"pub fn {symbol}"
    start = source_text.find(marker)
    if start < 0:
        raise SystemExit(f"function marker not found: {marker}")
    return source_text[start:]


def build_policy() -> dict[str, Any]:
    feature_policy = read_json(FEATURE_POLICY)
    surfaces = [
        surface for surface in feature_policy["source_surfaces"]
        if surface["feature_subcluster_id"] == SUBCLUSTER_ID
    ]
    if len(surfaces) != 1 or surfaces[0]["symbol"] != "contains_value_return":
        raise SystemExit(f"unexpected ValueReturnAstScan surfaces: {surfaces}")

    surface = surfaces[0]
    source_text = read_source(surface["source_path"])
    surface_text = function_text(source_text, surface["symbol"])
    variants = ast_variants(surface_text)

    return {
        "schema_version": 0,
        "kind": "MirBuilderCallLoweringValueReturnAstScanProjectionPolicyV1",
        "token": "MIRBUILDER-CALL-LOWERING-VALUE-RETURN-AST-SCAN-PROJECTION-POLICY-001",
        "input_state": {
            "feature_predicates_policy": rel(FEATURE_POLICY),
            "previous_policy": rel(PURE_METHOD_POLICY),
            "selected_feature_subcluster_id": SUBCLUSTER_ID,
            "source_count": len(surfaces),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "selection_axes": {
            "owner_edge_confidence": "FixtureMapped",
            "stable_deny_reason": "UnsupportedDirectShape",
            "shape_signature": "shape.ast_scan_predicate",
            "borrow_axis": "NoBorrow",
            "type_transport_axis": "Known",
            "verifier_or_oracle_state": "Present",
        },
        "source_surface": {
            "source_id": surface["source_id"],
            "symbol": surface["symbol"],
            "source_path": surface["source_path"],
            "params": surface["params"],
            "return_type": surface["return_type"],
            "ast_variants": variants,
            "recursion_markers": recursion_markers(surface_text),
        },
        "selected_policy": {
            "policy": "KeepParentAstScan",
            "owner_edge": "mirbuilder::call_lowering_value_return_ast_scan",
            "projection_surface_selected": False,
            "ast_traversal_projection_selected": False,
            "reason_token": "ValueReturnScanIsParentOwnedAstTraversal",
        },
        "decision": {
            "kind": "KeepParentOwner",
            "selected_next_card": "MIRBUILDER-CALL-LOWERING-NAME-CANONICALIZATION-PROJECTION-POLICY-001",
            "reason_token": "AstScanDoesNotOpenStandaloneProjectionOwner",
        },
        "claims": {
            "manual_family_selection": 0,
            "projection_surface_selected": 0,
            "ast_traversal_projection_selected": 0,
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
        print("mirbuilder-call-lowering-value-return-ast-scan-projection-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
