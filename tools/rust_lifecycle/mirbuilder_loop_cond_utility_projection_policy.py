#!/usr/bin/env python3
"""Resolve LoopCondUtility projection policy."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
PRIORITY = FIXTURES / "mirbuilder-projection-policy-cluster-priority-resolution-v0.json"
OUTPUT = FIXTURES / "mirbuilder-loop-cond-utility-projection-policy-v0.json"
SELECTED_CLUSTER_ID = (
    "projection_policy::UnsupportedDirectShape::shape.loop_cond_utility::"
    "FixtureMapped::LoopCondUtilityCluster"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_source(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def no_borrow(item: dict[str, Any]) -> bool:
    ret = item.get("return_type") or ""
    params = item.get("params") or ""
    return "&" not in ret and "&mut" not in params and "&self" not in params


def known_type_transport(item: dict[str, Any]) -> bool:
    ret = item.get("return_type") or ""
    return ret in {"", "bool", "usize", "i64", "String"}


def selected_surfaces(report: dict[str, Any]) -> list[dict[str, Any]]:
    return [
        item for item in report["items"]
        if item.get("classification") == "MissingProjectionPolicy"
        and item.get("shape_signature") == "shape.loop_cond_utility"
        and item.get("loop_cond_feature_subcluster") == "LoopCondUtilityCluster"
        and item.get("owner_edge_confidence") == "FixtureMapped"
        and item.get("stable_deny_reason") == "UnsupportedDirectShape"
        and item.get("evidence_refs")
        and no_borrow(item)
        and known_type_transport(item)
    ]


def build_policy() -> dict[str, Any]:
    report = read_json(REPORT)
    surfaces = selected_surfaces(report)
    source_text = "\n".join(read_source(item["source_path"]) for item in surfaces)

    evidence_markers = [
        "DirectExitRejectReason",
        "DIRECT_EXIT_REJECT_TAG",
        "direct_exit_reason_text",
        "Backward-compatible fallback for legacy message shapes.",
        "BlockContainsDirectExit",
        "ExitMustBeInsideIf",
        "BreakMustBeLast",
        "ReturnMustBeLast",
    ]
    present_markers = [marker for marker in evidence_markers if marker in source_text]

    return {
        "schema_version": 0,
        "kind": "MirBuilderLoopCondUtilityProjectionPolicyV1",
        "token": "MIRBUILDER-LOOP-COND-UTILITY-PROJECTION-POLICY-001",
        "input_state": {
            "source_report": rel(REPORT),
            "priority_resolution": rel(PRIORITY),
            "selected_cluster_id": SELECTED_CLUSTER_ID,
            "source_count": len(surfaces),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "selection_axes": {
            "owner_edge_confidence": "FixtureMapped",
            "stable_deny_reason": "UnsupportedDirectShape",
            "shape_signature": "shape.loop_cond_utility",
            "borrow_axis": "NoBorrow",
            "type_transport_axis": "Known",
            "verifier_or_oracle_state": "Present",
        },
        "source_surfaces": [
            {
                "source_id": item["source_id"],
                "symbol": item["symbol"],
                "source_path": item["source_path"],
                "visibility": item["visibility"],
                "return_type": item["return_type"],
            }
            for item in surfaces
        ],
        "utility_evidence": present_markers,
        "selected_policy": {
            "policy": "KeepParentOwner",
            "owner_edge": "mirbuilder::loop_cond_utility",
            "projection_surface_selected": False,
            "reason_token": "LoopCondUtilityDiagnosticTagHelpersAreParentOwned",
        },
        "decision": {
            "kind": "KeepParentOwner",
            "selected_next_card": "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001",
            "reason_token": "LoopCondUtilityDoesNotOpenStandaloneProjectionOwner",
        },
        "claims": {
            "manual_family_selection": 0,
            "projection_surface_selected": 0,
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
        print("mirbuilder-loop-cond-utility-projection-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
