#!/usr/bin/env python3
"""Partition OtherMissingProjectionPolicyCluster rows into source-derived subclusters."""

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
OWNER_FIELD = FIXTURES / "mirbuilder-crate-wide-surface-report-owner-cluster-field-v0.json"
OUTPUT = FIXTURES / "mirbuilder-missing-projection-policy-other-owner-cluster-v0.json"
TOKEN = "MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-CLUSTER-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def stable_id(*parts: str) -> str:
    return re.sub(r"[^a-zA-Z0-9:._-]+", "_", "::".join(parts))


def source_module(path: str) -> str:
    source = Path(path)
    if source.suffix == ".rs":
        return str(source.with_suffix(""))
    return path or "unknown"


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
    if ret.startswith("Vec<") and "&" not in ret:
        return "KnownVecCarrier"
    if "impl Iterator" in ret:
        return "ReturnedIteratorNeedsPolicy"
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
    if ret.startswith("Vec<"):
        return "vec"
    if "impl Iterator" in ret:
        return "iterator"
    if "&" in ret:
        return "borrow_return"
    return "custom_carrier"


def surface_role(item: dict[str, Any]) -> str:
    path = item.get("source_path") or ""
    symbol = item.get("symbol") or ""
    if "builder_emit" in path or "emit_guard" in path:
        return "instruction_emission_or_guard_surface"
    if "builder_init" in path:
        return "builder_lifecycle_surface"
    if "builder_value_kind" in path:
        return "value_kind_registry_surface"
    if "call_resolution" in path or "builder_method_index" in path:
        return "call_resolution_registry_surface"
    if "collection_literals" in path:
        return "collection_literal_surface"
    if "control_flow/generic_loop_canon" in path:
        return "loop_canon_fact_surface"
    if "control_flow/cleanup" in path:
        return "control_flow_cleanup_policy_surface"
    if "control_flow" in path:
        return "control_flow_dispatch_surface"
    if "decl" in path:
        return "declaration_lowering_surface"
    if "exprs" in path or "expressions" in path or "indexing" in path or "short_circuit" in path:
        return "expression_lowering_surface"
    if "fastmem" in path:
        return "fastmem_surface"
    if "field_facts" in path or "/fields" in path:
        return "field_fact_surface"
    if "function_slot_registry" in path:
        return "function_slot_registry_surface"
    if "joinir_id_remapper" in path:
        return "joinir_id_remapper_surface"
    if "local_ssa" in path:
        return "local_ssa_surface"
    if "phi" in path:
        return "phi_surface"
    if "properties" in path:
        return "property_surface"
    if "rewrite" in path:
        return "rewrite_known_surface"
    if "types" in path or "observe/types" in path:
        return "type_annotation_surface"
    if "weak_ref" in path:
        return "weak_ref_surface"
    if symbol.startswith(("build_", "try_build_")):
        return "builder_expression_helper_surface"
    if symbol.startswith(("is_", "has_", "should_")):
        return "predicate_helper_surface"
    return "unmapped_other_surface"


def public_or_private_surface(item: dict[str, Any]) -> str:
    visibility = item.get("visibility") or ""
    return "public" if visibility.startswith("pub") else "private"


def verifier_or_oracle_state(item: dict[str, Any]) -> str:
    return "Present" if item.get("evidence_refs") else "MissingVerifier"


def selected_items(report: dict[str, Any]) -> list[dict[str, Any]]:
    return sorted([
        item
        for item in report.get("items", [])
        if item.get("classification") == "MissingProjectionPolicy"
        and item.get("likely_owner_cluster") == "OtherMissingProjectionPolicyCluster"
    ], key=lambda item: item["source_id"])


def build_fixture() -> dict[str, Any]:
    report = read_json(REPORT)
    owner_field = read_json(OWNER_FIELD)
    items = selected_items(report)

    grouped: dict[tuple[str, str, str, str, str, str, str, str], list[dict[str, Any]]] = defaultdict(list)
    for item in items:
        key = (
            surface_role(item),
            source_module(item.get("source_path") or ""),
            item.get("shape_signature") or "unknown_shape",
            borrow_axis(item),
            type_transport_axis(item),
            return_family(item),
            verifier_or_oracle_state(item),
            public_or_private_surface(item),
        )
        grouped[key].append(item)

    subclusters: list[dict[str, Any]] = []
    for key, cluster_items in grouped.items():
        role, module, shape, borrow, type_axis, ret_family, verifier, visibility = key
        blocked_by = ["OwnerEdgeConfidenceMissing"]
        if shape == "unknown_shape":
            blocked_by.append("MissingShapeSignature")
        if verifier != "Present":
            blocked_by.append(verifier)
        if type_axis in {
            "MissingBorrowTransport",
            "MissingTypeTransport",
            "ResultCarrierNeedsVerifier",
            "ReturnedIteratorNeedsPolicy",
        }:
            blocked_by.append(type_axis)
        if borrow in {"BorrowPolicyNeeded", "ReturnedMutableAliasUnknown"}:
            blocked_by.append(borrow)

        subclusters.append({
            "subcluster_id": stable_id(
                "other_owner_cluster",
                role,
                module,
                shape,
                borrow,
                type_axis,
                ret_family,
                verifier,
                visibility,
            ),
            "surface_role": role,
            "source_module": module,
            "shape_signature": shape,
            "borrow_axis": borrow,
            "type_transport_axis": type_axis,
            "return_family": ret_family,
            "verifier_or_oracle_state": verifier,
            "public_or_private_surface": visibility,
            "owner_edge_confidence": "None",
            "known_owner_edge": "",
            "candidate_count": len(cluster_items),
            "source_ids": [item["source_id"] for item in cluster_items[:20]],
            "source_id_count": len(cluster_items),
            "selection_eligible": False,
            "blocked_by": blocked_by,
            "reason_token": "OtherOwnerClusterPartitionedForOwnerEdgeRepair",
        })

    subclusters.sort(key=lambda item: (-item["candidate_count"], item["subcluster_id"]))
    role_counts = Counter(surface_role(item) for item in items)
    type_axis_counts = Counter(type_transport_axis(item) for item in items)
    borrow_counts = Counter(borrow_axis(item) for item in items)
    confidence_counts = Counter(item.get("owner_edge_confidence") or "None" for item in items)

    decision = {
        "kind": "SelectOwnerEdgeConfidenceRepair",
        "selected_subcluster_id": None,
        "selected_next_card": "MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-EDGE-CONFIDENCE-REPAIR-001",
        "reason_token": "OtherOwnerClusterRequiresOwnerEdgeConfidenceRepair",
    }

    return {
        "schema_version": 0,
        "kind": "MirBuilderMissingProjectionPolicyOtherOwnerClusterV1",
        "token": TOKEN,
        "input_state": {
            "source_report": rel(REPORT),
            "owner_cluster_field_audit": rel(OWNER_FIELD),
            "owner_cluster_field_decision": owner_field.get("decision", {}),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "input_other_owner_cluster_count": len(items),
        },
        "provenance": {
            "source_report_hash": sha256_file(REPORT),
            "owner_cluster_field_hash": sha256_file(OWNER_FIELD),
        },
        "subcluster_axes": [
            "surface_role",
            "source_module",
            "shape_signature",
            "borrow_axis",
            "type_transport_axis",
            "return_family",
            "verifier_or_oracle_state",
            "public_or_private_surface",
        ],
        "subclusters": subclusters,
        "summary": {
            "input_other_owner_cluster_count": len(items),
            "subcluster_count": len(subclusters),
            "surface_role_counts": dict(sorted(role_counts.items())),
            "type_transport_axis_counts": dict(sorted(type_axis_counts.items())),
            "borrow_axis_counts": dict(sorted(borrow_counts.items())),
            "owner_edge_confidence_counts": dict(sorted(confidence_counts.items())),
            "selection_eligible_subcluster_count": 0,
            "selected_subcluster_id": None,
        },
        "decision": decision,
        "claims": {
            "source_report_consumed": 1,
            "owner_cluster_field_audit_consumed": 1,
            "input_other_owner_cluster_count": len(items),
            "all_other_owner_cluster_items_partitioned_exactly_once": 1,
            "subcluster_ids_are_stable": 1,
            "subcluster_reason_tokens_are_stable": 1,
            "owner_edge_confidence_repair_selected": 1,
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
    parser.add_argument("--check", action="store_true", help="Verify checked-in Other owner cluster fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-missing-projection-policy-other-owner-cluster unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
