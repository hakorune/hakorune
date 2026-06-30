#!/usr/bin/env python3
"""Resolve the next MirBuilder owner from the unconverted-surface report.

This resolver is deterministic and evidence-joining only. It consumes the
crate-wide unconverted surface report plus the task contract, applies the
contract's exclusion and priority rules, and emits either exactly one next owner
or a stable blocked reason. It does not emit Hako, infer projection policy, or
select Source Selfhost by hand.
"""

from __future__ import annotations

import argparse
import hashlib
import json
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
CONTRACT = FIXTURES / "mirbuilder-unconverted-surface-next-owner-resolver-task-contract-v0.json"
OUTPUT = FIXTURES / "mirbuilder-unconverted-surface-next-owner-resolution-v0.json"
CURRENT_STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"


DECISION_BY_CLASSIFICATION = {
    "MissingRouteOrArtifactEvidence": "SelectOwnerEdgeClassification",
    "MissingProjectionPolicy": "SelectProjectionPolicy",
    "BorrowSurfaceNeedsPolicy": "SelectBorrowPolicy",
    "CompositeNeedsDecomposition": "SelectCompositeDecomposition",
    "CompositeSuspected": "SelectCompositeEvidenceInventory",
    "MissingVerifierOrOracle": "SelectVerifierOrOracleRepair",
    "NativeSeedReady": "SelectHakoAdoptionDecision",
    "ConvertibleLeaf": "SelectNativeSourceSeed",
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def cluster_key(item: dict[str, Any]) -> str:
    for key in [
        "loop_cond_co_statement_lowering_subcluster",
        "loop_cond_co_helper_subcluster",
        "loop_cond_co_group_if_subcluster",
        "loop_cond_co_continue_if_subcluster",
        "loop_cond_co_subcluster",
        "loop_cond_bc_pipeline_subcluster",
        "loop_cond_bc_item_lowering_subcluster",
        "loop_cond_bc_cleanup_subcluster",
        "loop_cond_bc_else_pattern_subcluster",
        "loop_cond_bc_subcluster",
        "loop_cond_feature_subcluster",
        "plan_feature_subcluster",
        "joinir_plan_subcluster",
        "likely_owner_cluster",
    ]:
        value = item.get(key)
        if value:
            return value
    return "Unclustered"


def candidate_next_card(item: dict[str, Any], decision: str) -> str | None:
    if decision == "SelectCompositeEvidenceInventory":
        owner = item.get("known_owner_edge") or cluster_key(item)
        return f"{owner}-COMPOSITE-EVIDENCE-INVENTORY-001"
    return item.get("next_card")


def summarize_candidates(items: list[dict[str, Any]]) -> list[dict[str, Any]]:
    clusters = Counter(cluster_key(item) for item in items)
    return [
        {"cluster": cluster, "count": count}
        for cluster, count in sorted(clusters.items(), key=lambda pair: (-pair[1], pair[0]))
    ]


def build_resolution() -> dict[str, Any]:
    report = read_json(REPORT)
    contract = read_json(CONTRACT)

    exclusions = set(contract["exclusion_rules"])
    priorities = list(contract["priority_rules"])
    candidates_by_priority: dict[str, list[dict[str, Any]]] = {}

    for item in report["items"]:
        classification = item["classification"]
        if classification in exclusions:
            continue
        if item.get("next_owner_kind") == "None" and classification != "CompositeSuspected":
            continue
        if classification in priorities:
            candidates_by_priority.setdefault(classification, []).append(item)

    selected_priority = None
    selected_candidates: list[dict[str, Any]] = []
    for priority in priorities:
        bucket = candidates_by_priority.get(priority, [])
        if bucket:
            selected_priority = priority
            selected_candidates = sorted(bucket, key=lambda item: item["source_id"])
            break

    if not selected_candidates:
        decision = {
            "kind": "KeepStopped",
            "selected_source_id": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "reason_token": contract["tie_breaker"]["zero_candidates_reason_token"],
        }
    elif len(selected_candidates) == 1:
        selected = selected_candidates[0]
        decision_kind = DECISION_BY_CLASSIFICATION[selected_priority or selected["classification"]]
        decision = {
            "kind": decision_kind,
            "selected_source_id": selected["source_id"],
            "selected_next_card": candidate_next_card(selected, decision_kind),
            "reason_token": f"ExactlyOne{selected_priority}",
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "selected_source_id": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "reason_token": contract["tie_breaker"]["same_priority_reason_token"],
        }

    candidate_counts = {
        priority: len(candidates_by_priority.get(priority, []))
        for priority in priorities
    }

    return {
        "schema_version": 0,
        "kind": "MirBuilderUnconvertedSurfaceNextOwnerResolutionV1",
        "token": "MIRBUILDER-UNCONVERTED-SURFACE-NEXT-OWNER-RESOLVER-001",
        "input_authority": {
            "unconverted_surface_report": rel(REPORT),
            "resolver_task_contract": rel(CONTRACT),
            "current_state": rel(CURRENT_STATE),
        },
        "provenance": {
            "unconverted_surface_report_hash": sha256_file(REPORT),
            "resolver_task_contract_hash": sha256_file(CONTRACT),
        },
        "resolver_rules": {
            "exclusion_rules": contract["exclusion_rules"],
            "priority_rules": contract["priority_rules"],
            "same_priority_multiple_candidates": contract["tie_breaker"]["same_priority_multiple_candidates"],
            "same_priority_reason_token": contract["tie_breaker"]["same_priority_reason_token"],
            "zero_candidates_reason_token": contract["tie_breaker"]["zero_candidates_reason_token"],
            "manual_selection_allowed": 0,
        },
        "candidate_pool": {
            "candidate_counts_by_priority": candidate_counts,
            "selected_priority": selected_priority,
            "selected_priority_candidate_count": len(selected_candidates),
            "selected_priority_cluster_summary": summarize_candidates(selected_candidates),
        },
        "decision": decision,
        "recovery": {
            "if_ambiguous": "add a narrower machine-derived classification card or rerun after evidence reduces the highest-priority bucket to exactly one",
            "if_zero_candidates": "keep the wider route-selection design stop active",
            "do_not": [
                "manual_family_selection",
                "coverage_percentage_as_proof",
                "route_membership_alone_as_proof",
                "generated_artifact_as_edit_authority",
                "runtime_fallback"
            ]
        },
        "claims": {
            "report_consumed": 1,
            "resolver_implemented": 1,
            "exactly_one_next_owner_selected_if_unambiguous": 1,
            "multiple_candidates_keep_stopped": 1,
            "zero_candidates_keep_stopped": 1,
            "manual_family_selection": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "native_seed_materialization": 0
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify the checked-in resolution fixture.")
    args = parser.parse_args()

    output = stable_json(build_resolution())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-unconverted-surface-next-owner-resolution unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
