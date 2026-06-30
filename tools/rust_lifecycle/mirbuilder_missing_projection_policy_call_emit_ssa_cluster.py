#!/usr/bin/env python3
"""Partition remaining Call/Emit/SSA MissingProjectionPolicy rows."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
CLUSTER_RESOLUTION = FIXTURES / "mirbuilder-crate-wide-missing-projection-policy-cluster-resolution-v0.json"
PRIORITY = FIXTURES / "mirbuilder-projection-policy-cluster-priority-resolution-v0.json"
CALL_DECOMP = FIXTURES / "mirbuilder-call-lowering-policy-subcluster-decomposition-v0.json"
STMT_DECOMP = FIXTURES / "mirbuilder-statement-value-construction-subcluster-decomposition-v0.json"
EMIT_POLICY = FIXTURES / "mirbuilder-emission-ssa-phi-projection-policy-v0.json"
OUTPUT = FIXTURES / "mirbuilder-missing-projection-policy-call-emit-ssa-cluster-v0.json"
TOKEN = "MIRBUILDER-MISSING-PROJECTION-POLICY-CALL-EMIT-SSA-CLUSTER-001"

SOURCE_CLUSTERS = {
    "CallLoweringCluster",
    "EmissionSsaPhiCluster",
    "StatementValueConstructionCluster",
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def stable_id(*parts: str) -> str:
    return re.sub(r"[^a-zA-Z0-9:._-]+", "_", "::".join(parts))


def source_module(path: str) -> str:
    if not path:
        return "unknown"
    source = Path(path)
    if len(source.parts) > 1 and source.suffix == ".rs":
        return str(source.parent)
    return path


def borrow_axis(item: dict[str, Any]) -> str:
    ret = item.get("return_type") or ""
    receiver = item.get("receiver") or ""
    if "&mut" in ret:
        return "ReturnedMutableAliasUnknown"
    if "&" in ret:
        return "BorrowPolicyNeeded"
    if receiver == "&mut self":
        return "NoReturnedBorrowMutableReceiver"
    if receiver == "&self":
        return "NoReturnedBorrowSharedReceiver"
    return "NoBorrow"


def type_transport_axis(item: dict[str, Any]) -> str:
    ret = item.get("return_type") or ""
    if ret in {"", "bool", "usize", "u32", "i64", "String"}:
        return "Known"
    if ret == "Self":
        return "ConstructorCarrier"
    if ret.startswith("Option<") and "&" not in ret:
        return "KnownOptionCarrier"
    if ret.startswith("Result<") and "&" not in ret:
        return "ResultCarrierNeedsVerifier"
    if "&" in ret:
        return "MissingBorrowTransport"
    return "MissingTypeTransport"


def return_family(item: dict[str, Any]) -> str:
    ret = item.get("return_type") or ""
    if ret == "":
        return "unit"
    if ret == "bool":
        return "bool"
    if ret in {"usize", "u32", "i64"}:
        return "scalar"
    if ret == "String":
        return "string"
    if ret == "Self":
        return "constructor_self"
    if ret.startswith("Option<"):
        return "option"
    if ret.startswith("Result<"):
        return "result"
    if "&" in ret:
        return "borrow_return"
    return "custom_carrier"


def surface_role(item: dict[str, Any]) -> str:
    cluster = item.get("likely_owner_cluster") or "UnknownCluster"
    path = item.get("source_path") or ""
    symbol = item.get("symbol") or ""
    if cluster == "CallLoweringCluster":
        if "/calls/" in path:
            if any(part in path for part in ["extern", "resolver", "method_resolution", "member_route", "static_receiver"]):
                return "call_resolution_or_registry"
            if any(part in path for part in ["build", "emit", "function_lowering"]):
                return "call_build_or_emit"
            if any(part in path for part in ["guard", "utils", "call_unified", "context_lifecycle"]):
                return "call_feature_or_context_helper"
            return "call_misc_helper"
        if "method_call_handlers" in path:
            return "method_call_handler"
        return "call_other"
    if cluster == "EmissionSsaPhiCluster":
        if "/emission/phi" in path or "/phi.rs" in path or "phi_input" in path:
            return "phi_lifecycle_or_materialization"
        if "/ssa/" in path:
            return "ssa_analysis_or_local"
        if "/emission/constant" in path:
            return "constant_emission"
        if "/emission/" in path:
            return "branch_copy_or_value_emission"
        return "emission_other"
    if cluster == "StatementValueConstructionCluster":
        if "builder_build.rs" in path:
            if symbol in {"undefined_variable_message", "is_current_block_terminated"}:
                return "statement_diagnostic_or_predicate"
            return "expression_or_module_build"
        if "/fields" in path:
            return "field_access_or_initialization"
        if "record_values" in path:
            return "record_value_construction"
        if "/stmts/" in path:
            return "statement_lowering"
        if "/vars/" in path:
            return "lexical_scope_or_variable"
        if "/ops/" in path:
            return "operator_conversion"
        return "statement_other"
    return "unknown_call_emit_ssa_surface"


def prior_state(item: dict[str, Any], call_decomp: dict[str, Any], stmt_decomp: dict[str, Any], emit_policy: dict[str, Any]) -> str:
    source_id = item["source_id"]
    if source_id in {surface["source_id"] for surface in call_decomp.get("source_surfaces", [])}:
        return "CoveredByCallLoweringSubclusterDecomposition"
    if source_id in {surface["source_id"] for surface in stmt_decomp.get("source_surfaces", [])}:
        return "CoveredByStatementValueConstructionSubclusterDecomposition"
    if source_id in {surface["source_id"] for surface in emit_policy.get("source_surfaces", [])}:
        return "CoveredByEmissionSsaPhiProjectionPolicy"
    return "UncoveredByPriorNarrowDecision"


def verifier_or_oracle_state(item: dict[str, Any]) -> str:
    return "Present" if item.get("evidence_refs") else "MissingVerifier"


def public_or_private_surface(item: dict[str, Any]) -> str:
    visibility = item.get("visibility") or ""
    return "public" if visibility.startswith("pub") else "private"


def selected_items(report: dict[str, Any]) -> list[dict[str, Any]]:
    return sorted([
        item
        for item in report.get("items", [])
        if item.get("classification") == "MissingProjectionPolicy"
        and item.get("likely_owner_cluster") in SOURCE_CLUSTERS
    ], key=lambda item: item["source_id"])


def build_resolution() -> dict[str, Any]:
    report = read_json(REPORT)
    cluster_resolution = read_json(CLUSTER_RESOLUTION)
    priority = read_json(PRIORITY)
    call_decomp = read_json(CALL_DECOMP)
    stmt_decomp = read_json(STMT_DECOMP)
    emit_policy = read_json(EMIT_POLICY)
    items = selected_items(report)

    grouped: dict[tuple[str, str, str, str, str, str, str, str, str, str], list[dict[str, Any]]] = defaultdict(list)
    for item in items:
        key = (
            item.get("likely_owner_cluster") or "UnknownCluster",
            surface_role(item),
            source_module(item.get("source_path") or ""),
            item.get("shape_signature") or "unknown_shape",
            borrow_axis(item),
            type_transport_axis(item),
            return_family(item),
            verifier_or_oracle_state(item),
            public_or_private_surface(item),
            prior_state(item, call_decomp, stmt_decomp, emit_policy),
        )
        grouped[key].append(item)

    subclusters: list[dict[str, Any]] = []
    for key, cluster_items in grouped.items():
        (
            source_cluster,
            role,
            module,
            shape,
            borrow,
            type_axis,
            ret_family,
            verifier,
            visibility,
            prior,
        ) = key
        blocked_by: list[str] = []
        if prior.startswith("CoveredBy"):
            blocked_by.append("CoveredByPriorNarrowDecision")
        if shape == "unknown_shape":
            blocked_by.append("MissingShapeSignature")
        if verifier != "Present":
            blocked_by.append(verifier)
        if type_axis in {"MissingBorrowTransport", "MissingTypeTransport", "ResultCarrierNeedsVerifier"}:
            blocked_by.append(type_axis)
        if borrow in {"BorrowPolicyNeeded", "ReturnedMutableAliasUnknown"}:
            blocked_by.append(borrow)
        subclusters.append({
            "subcluster_id": stable_id(
                "call_emit_ssa",
                source_cluster,
                role,
                module,
                shape,
                borrow,
                type_axis,
                ret_family,
                verifier,
                visibility,
                prior,
            ),
            "source_cluster": source_cluster,
            "surface_role": role,
            "source_module": module,
            "shape_signature": shape,
            "borrow_axis": borrow,
            "type_transport_axis": type_axis,
            "return_family": ret_family,
            "verifier_or_oracle_state": verifier,
            "public_or_private_surface": visibility,
            "prior_narrow_decision_state": prior,
            "candidate_count": len(cluster_items),
            "source_ids": [item["source_id"] for item in cluster_items[:20]],
            "source_id_count": len(cluster_items),
            "selection_eligible": False,
            "blocked_by": blocked_by,
            "reason_token": "CallEmitSsaClusterPartitionedForFollowUp",
        })

    subclusters.sort(key=lambda item: (-item["candidate_count"], item["subcluster_id"]))
    eligible_subclusters = [
        item for item in subclusters
        if not item["blocked_by"]
        and item["public_or_private_surface"] == "public"
    ]
    cluster_counts = Counter(item.get("likely_owner_cluster") for item in items)
    role_counts = Counter(surface_role(item) for item in items)
    prior_counts = Counter(prior_state(item, call_decomp, stmt_decomp, emit_policy) for item in items)
    type_axis_counts = Counter(type_transport_axis(item) for item in items)

    if len(eligible_subclusters) == 1:
        selected = eligible_subclusters[0]
        decision = {
            "kind": "SelectProjectionPolicySubcluster",
            "selected_subcluster_id": selected["subcluster_id"],
            "selected_next_card": (
                "MIRBUILDER-CALL-EMIT-SSA-"
                f"{selected['surface_role'].upper().replace('_', '-')}-PROJECTION-POLICY-001"
            ),
            "reason_token": "ExactlyOneCallEmitSsaProjectionSubcluster",
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "selected_subcluster_id": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "reason_token": (
                "AmbiguousCallEmitSsaProjectionSubclusters"
                if eligible_subclusters else
                "NoEligibleCallEmitSsaProjectionSubcluster"
            ),
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderMissingProjectionPolicyCallEmitSsaClusterV1",
        "token": TOKEN,
        "input_state": {
            "source_report": rel(REPORT),
            "cluster_resolution": rel(CLUSTER_RESOLUTION),
            "priority_resolution": rel(PRIORITY),
            "call_lowering_decomposition": rel(CALL_DECOMP),
            "statement_value_construction_decomposition": rel(STMT_DECOMP),
            "emission_ssa_phi_policy": rel(EMIT_POLICY),
            "priority_decision": priority.get("decision", {}),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "input_call_emit_ssa_cluster_count": len(items),
        },
        "provenance": {
            "source_report_hash": sha256_file(REPORT),
            "cluster_resolution_hash": sha256_file(CLUSTER_RESOLUTION),
            "priority_resolution_hash": sha256_file(PRIORITY),
            "call_lowering_decomposition_hash": sha256_file(CALL_DECOMP),
            "statement_value_construction_decomposition_hash": sha256_file(STMT_DECOMP),
            "emission_ssa_phi_policy_hash": sha256_file(EMIT_POLICY),
        },
        "subcluster_axes": [
            "source_cluster",
            "surface_role",
            "source_module",
            "shape_signature",
            "borrow_axis",
            "type_transport_axis",
            "return_family",
            "verifier_or_oracle_state",
            "public_or_private_surface",
            "prior_narrow_decision_state",
        ],
        "subclusters": subclusters,
        "summary": {
            "input_call_emit_ssa_cluster_count": len(items),
            "subcluster_count": len(subclusters),
            "source_cluster_counts": dict(sorted(cluster_counts.items())),
            "surface_role_counts": dict(sorted(role_counts.items())),
            "prior_narrow_decision_state_counts": dict(sorted(prior_counts.items())),
            "type_transport_axis_counts": dict(sorted(type_axis_counts.items())),
            "selection_eligible_subcluster_count": len(eligible_subclusters),
            "selected_subcluster_id": (
                eligible_subclusters[0]["subcluster_id"]
                if len(eligible_subclusters) == 1
                else None
            ),
        },
        "decision": decision,
        "claims": {
            "source_report_consumed": 1,
            "projection_priority_consumed": 1,
            "prior_narrow_decisions_consumed": 1,
            "input_call_emit_ssa_cluster_count": len(items),
            "all_call_emit_ssa_items_partitioned_exactly_once": 1,
            "subcluster_ids_are_stable": 1,
            "subcluster_reason_tokens_are_stable": 1,
            "manual_family_selection": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "generated_artifact_as_edit_authority": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "native_source_seed_materialization": 0,
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
    parser.add_argument("--check", action="store_true", help="Verify checked-in Call/Emit/SSA cluster fixture.")
    args = parser.parse_args()

    output = stable_json(build_resolution())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-missing-projection-policy-call-emit-ssa-cluster unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
