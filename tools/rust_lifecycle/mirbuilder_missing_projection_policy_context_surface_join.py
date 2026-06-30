#!/usr/bin/env python3
"""Join remaining ContextRegistryCluster MissingProjectionPolicy rows by context surface."""

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
PREVIOUS_POLICY = FIXTURES / "mirbuilder-context-registry-projection-policy-v0.json"
OUTPUT = FIXTURES / "mirbuilder-missing-projection-policy-context-surface-join-v0.json"
TOKEN = "MIRBUILDER-MISSING-PROJECTION-POLICY-CONTEXT-SURFACE-JOIN-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def stable_id(*parts: str) -> str:
    return re.sub(r"[^a-zA-Z0-9:._-]+", "_", "::".join(parts))


def context_surface(item: dict[str, Any]) -> str:
    path = item.get("source_path") or ""
    if "binding_context" in path:
        return "binding_context"
    if "metadata_context" in path or "builder_metadata" in path:
        return "metadata_context"
    if "type_context" in path or "type_registry" in path:
        return "type_context"
    if "core_context" in path:
        return "core_context"
    if "scope_context" in path:
        return "scope_context"
    if "compilation_context" in path:
        return "compilation_context"
    if path.endswith("/context.rs") or path.endswith("src/context.rs"):
        return "aggregate_context"
    return "unknown_context_surface"


def operation_role(item: dict[str, Any]) -> str:
    symbol = item.get("symbol") or ""
    receiver = item.get("receiver") or ""
    if symbol == "new" or item.get("return_type") == "Self":
        return "constructor"
    if symbol.startswith("peek_") or symbol in {"contains", "is_empty", "len", "lookup", "current_span", "current_source_file", "value_span"}:
        return "read_only_query"
    if symbol.startswith("next_"):
        return "allocator_counter"
    if symbol.startswith("clear") or symbol.startswith("set_") or symbol.startswith("record_") or symbol.startswith("push_") or symbol.startswith("pop_") or symbol in {"insert", "remove"}:
        return "explicit_mutation"
    if receiver == "&mut self":
        return "mutable_context_operation"
    if receiver == "&self":
        return "read_only_operation"
    return "free_or_static_context_operation"


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


def native_authority_hint(surface: str) -> str:
    if surface in {"binding_context", "metadata_context", "type_context", "core_context", "aggregate_context"}:
        return "HasAdjacentNativeContextEvidence"
    if surface == "scope_context":
        return "RustPrimaryOrUnknown"
    if surface == "compilation_context":
        return "RustPrimaryOrUnknown"
    return "Unknown"


def verifier_or_oracle_state(item: dict[str, Any]) -> str:
    return "Present" if item.get("evidence_refs") else "MissingVerifier"


def public_or_private_surface(item: dict[str, Any]) -> str:
    visibility = item.get("visibility") or ""
    return "public" if visibility.startswith("pub") else "private"


def context_items(report: dict[str, Any]) -> list[dict[str, Any]]:
    return sorted([
        item
        for item in report.get("items", [])
        if item.get("classification") == "MissingProjectionPolicy"
        and item.get("likely_owner_cluster") == "ContextRegistryCluster"
    ], key=lambda item: item["source_id"])


def build_resolution() -> dict[str, Any]:
    report = read_json(REPORT)
    cluster_resolution = read_json(CLUSTER_RESOLUTION)
    priority = read_json(PRIORITY)
    previous_policy = read_json(PREVIOUS_POLICY)
    items = context_items(report)

    grouped: dict[tuple[str, str, str, str, str, str, str, str, str], list[dict[str, Any]]] = defaultdict(list)
    for item in items:
        surface = context_surface(item)
        key = (
            surface,
            operation_role(item),
            item.get("shape_signature") or "unknown_shape",
            borrow_axis(item),
            type_transport_axis(item),
            return_family(item),
            verifier_or_oracle_state(item),
            public_or_private_surface(item),
            native_authority_hint(surface),
        )
        grouped[key].append(item)

    subclusters: list[dict[str, Any]] = []
    for key, cluster_items in grouped.items():
        surface, role, shape, borrow, type_axis, ret_family, verifier, visibility, native_hint = key
        blocked_by: list[str] = []
        if shape == "unknown_shape":
            blocked_by.append("MissingShapeSignature")
        if verifier != "Present":
            blocked_by.append(verifier)
        if type_axis in {"MissingBorrowTransport", "MissingTypeTransport", "ResultCarrierNeedsVerifier"}:
            blocked_by.append(type_axis)
        if borrow in {"BorrowPolicyNeeded", "ReturnedMutableAliasUnknown"}:
            blocked_by.append(borrow)
        if native_hint == "RustPrimaryOrUnknown":
            blocked_by.append("NativeAuthorityEvidenceMissing")
        subclusters.append({
            "subcluster_id": stable_id(
                "context_surface",
                surface,
                role,
                shape,
                borrow,
                type_axis,
                ret_family,
                verifier,
                visibility,
                native_hint,
            ),
            "context_surface": surface,
            "operation_role": role,
            "shape_signature": shape,
            "borrow_axis": borrow,
            "type_transport_axis": type_axis,
            "return_family": ret_family,
            "verifier_or_oracle_state": verifier,
            "public_or_private_surface": visibility,
            "native_authority_hint": native_hint,
            "candidate_count": len(cluster_items),
            "source_ids": [item["source_id"] for item in cluster_items[:20]],
            "source_id_count": len(cluster_items),
            "selection_eligible": False,
            "blocked_by": blocked_by,
            "reason_token": "ContextRegistrySurfaceJoinedForFollowUp",
        })

    subclusters.sort(key=lambda item: (-item["candidate_count"], item["subcluster_id"]))
    eligible_subclusters = [
        item for item in subclusters
        if not item["blocked_by"]
        and item["public_or_private_surface"] == "public"
    ]
    surface_counts = Counter(context_surface(item) for item in items)
    operation_counts = Counter(operation_role(item) for item in items)
    type_axis_counts = Counter(type_transport_axis(item) for item in items)
    native_hint_counts = Counter(native_authority_hint(context_surface(item)) for item in items)

    if len(eligible_subclusters) == 1:
        selected = eligible_subclusters[0]
        decision = {
            "kind": "SelectProjectionPolicySubcluster",
            "selected_subcluster_id": selected["subcluster_id"],
            "selected_next_card": (
                "MIRBUILDER-CONTEXT-SURFACE-"
                f"{selected['context_surface'].upper().replace('_', '-')}-PROJECTION-POLICY-001"
            ),
            "reason_token": "ExactlyOneContextSurfaceProjectionSubcluster",
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "selected_subcluster_id": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "reason_token": (
                "AmbiguousContextSurfaceProjectionSubclusters"
                if eligible_subclusters else
                "NoEligibleContextSurfaceProjectionSubcluster"
            ),
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderMissingProjectionPolicyContextSurfaceJoinV1",
        "token": TOKEN,
        "input_state": {
            "source_report": rel(REPORT),
            "cluster_resolution": rel(CLUSTER_RESOLUTION),
            "priority_resolution": rel(PRIORITY),
            "previous_parent_owned_policy": rel(PREVIOUS_POLICY),
            "previous_policy_decision": previous_policy.get("decision", {}),
            "priority_decision": priority.get("decision", {}),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "input_context_registry_cluster_count": len(items),
        },
        "provenance": {
            "source_report_hash": sha256_file(REPORT),
            "cluster_resolution_hash": sha256_file(CLUSTER_RESOLUTION),
            "priority_resolution_hash": sha256_file(PRIORITY),
            "previous_parent_owned_policy_hash": sha256_file(PREVIOUS_POLICY),
        },
        "subcluster_axes": [
            "context_surface",
            "operation_role",
            "shape_signature",
            "borrow_axis",
            "type_transport_axis",
            "return_family",
            "verifier_or_oracle_state",
            "public_or_private_surface",
            "native_authority_hint",
        ],
        "subclusters": subclusters,
        "summary": {
            "input_context_registry_cluster_count": len(items),
            "subcluster_count": len(subclusters),
            "context_surface_counts": dict(sorted(surface_counts.items())),
            "operation_role_counts": dict(sorted(operation_counts.items())),
            "type_transport_axis_counts": dict(sorted(type_axis_counts.items())),
            "native_authority_hint_counts": dict(sorted(native_hint_counts.items())),
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
            "previous_parent_owned_policy_consumed": 1,
            "input_context_registry_cluster_count": len(items),
            "all_context_registry_items_joined_exactly_once": 1,
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
    parser.add_argument("--check", action="store_true", help="Verify checked-in Context surface join fixture.")
    args = parser.parse_args()

    output = stable_json(build_resolution())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-missing-projection-policy-context-surface-join unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
