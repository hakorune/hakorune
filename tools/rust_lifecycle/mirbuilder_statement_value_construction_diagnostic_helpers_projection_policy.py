#!/usr/bin/env python3
"""Resolve StatementValueConstruction diagnostic helper projection policy."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
DECOMPOSITION = (
    FIXTURES / "mirbuilder-statement-value-construction-subcluster-decomposition-v0.json"
)
OUTPUT = (
    FIXTURES
    / "mirbuilder-statement-value-construction-diagnostic-helpers-projection-policy-v0.json"
)
SUBCLUSTER_ID = "DiagnosticStringHelpers"
TOKEN = "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-DIAGNOSTIC-HELPERS-PROJECTION-POLICY-001"
NEXT_CARD = (
    "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BLOCK-TERMINATION-PREDICATE-"
    "PROJECTION-POLICY-001"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_source(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def diagnostic_role(symbol: str) -> str:
    if symbol == "undefined_variable_message":
        return "undefined_variable_diagnostic_message"
    return f"statement_value_construction_diagnostic_helper::{symbol}"


def build_policy() -> dict[str, Any]:
    decomposition = read_json(DECOMPOSITION)
    surfaces = [
        surface
        for surface in decomposition["source_surfaces"]
        if surface["subcluster_id"] == SUBCLUSTER_ID
    ]
    role_counts = Counter(diagnostic_role(surface["symbol"]) for surface in surfaces)
    source_text = "\n".join(read_source(surface["source_path"]) for surface in surfaces)

    evidence_markers = [
        "Undefined variable:",
        "Stage-3 keyword",
        "parser_stage3_enabled()",
        "suggest_using_for_symbol(name)",
        "symbol appears in using module(s)",
        "Consider adding 'using <module> [as Alias]'",
    ]
    present_markers = [marker for marker in evidence_markers if marker in source_text]

    return {
        "schema_version": 0,
        "kind": "MirBuilderStatementValueConstructionDiagnosticHelpersProjectionPolicyV1",
        "token": TOKEN,
        "input_state": {
            "subcluster_decomposition": rel(DECOMPOSITION),
            "selected_subcluster_id": SUBCLUSTER_ID,
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
        "source_surfaces": [
            {
                "source_id": surface["source_id"],
                "symbol": surface["symbol"],
                "source_path": surface["source_path"],
                "params": surface["params"],
                "return_type": surface["return_type"],
                "diagnostic_role": diagnostic_role(surface["symbol"]),
            }
            for surface in surfaces
        ],
        "role_counts": dict(sorted(role_counts.items())),
        "diagnostic_evidence": present_markers,
        "selected_policy": {
            "policy": "KeepParentOwner",
            "owner_edge": "mirbuilder::statement_value_construction_diagnostic_helpers",
            "projection_surface_selected": False,
            "registry_descriptor_selected": False,
            "reason_token": "DiagnosticHelperIsParentOwnedMessage",
        },
        "decision": {
            "kind": "KeepParentOwner",
            "selected_next_card": NEXT_CARD,
            "reason_token": "DiagnosticHelpersDoNotOpenStandaloneProjectionOwner",
        },
        "claims": {
            "manual_family_selection": 0,
            "projection_surface_selected": 0,
            "registry_descriptor_selected": 0,
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
        print("mirbuilder-statement-value-construction-diagnostic-helpers-projection-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
