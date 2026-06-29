#!/usr/bin/env python3
"""Report MirBuilder Rust source surfaces not yet tied to a conversion path.

This diagnostic scans Rust source text conservatively. It does not emit Hako,
does not infer new semantic projection policy, and does not select Source
Selfhost. Its job is to make unconverted or under-classified Rust surfaces
visible with stable reason tokens.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"

SOURCE_ROOTS = [
    ROOT / "src/mir/builder",
    ROOT / "src/mir/region",
    ROOT / "crates/hakorune_mir_builder/src",
]
NATIVE_SEED_SURVEY = FIXTURES / "mirbuilder-crate-wide-native-owner-seed-capability-survey-v0.json"
REFERENCE_PROJECTION = FIXTURES / "variable-context-reference-projection-contract-v0.json"
FAMILY_MANIFEST = FIXTURES / "source-selfhost-family-guard-manifest-v0.json"
CURRENT_STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"

FN_PATTERN = re.compile(
    r"(?P<vis>pub(?:\([^)]*\))?)?\s*fn\s+"
    r"(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*"
    r"\((?P<params>[^)]*)\)\s*"
    r"(?:->\s*(?P<ret>[^{;\n]+))?",
    re.MULTILINE,
)

REASON_TOKEN_TABLE = {
    "DebugOnlySurfaceIgnored": "debug-only source surface; not a selfhost conversion owner",
    "KnownFinalizeModuleContainsMultipleSemanticEdges": "regex-level composite suspicion only; not decomposition proof",
    "MappedToKnownOwnerEdge": "source surface is mapped to an existing owner edge",
    "NoKnownOwnerEdge": "source surface has no known owner-edge evidence yet",
    "PrivateHelperSurfaceNotSelected": "private helper not selected as a source-selfhost conversion surface",
    "PublicRustSurfaceMissingProjectionPolicy": "public Rust surface lacks a selected Hako projection policy",
    "ReturnedMutableBorrowCoveredByExplicitMutationApiOnly": "known returned mutable borrow replacement policy",
    "ReturnedMutableBorrowPolicyMissing": "returned mutable borrow detected without an existing replacement policy",
    "ReturnedReadBorrowCoveredByOwnedReadSnapshotProjection": "known returned read borrow replacement policy",
    "ReturnedReadBorrowPolicyMissing": "returned read borrow detected without an existing replacement policy",
    "TestOnlySurfaceIgnored": "test-only source surface; not a selfhost conversion owner",
}

OWNER_CLUSTER_RULES = [
    ("JoinIRPlanCluster", ("src/mir/builder/control_flow/plan/",)),
    ("JoinIRRouteRegistryCluster", ("src/mir/builder/control_flow/joinir/route_entry/",)),
    (
        "JoinIRRouteVerifyCluster",
        (
            "src/mir/builder/control_flow/facts/",
            "src/mir/builder/control_flow/verify/",
            "src/mir/builder/control_flow/edgecfg/",
            "src/mir/builder/control_flow/recipes/",
            "src/mir/builder/control_flow/generic_loop/",
            "src/mir/builder/control_flow/joinir/",
        ),
    ),
    (
        "ContextRegistryCluster",
        (
            "src/mir/builder/compilation_context",
            "src/mir/builder/type_registry.rs",
            "src/mir/builder/scope_context.rs",
            "src/mir/builder/builder_metadata.rs",
            "crates/hakorune_mir_builder/src/",
        ),
    ),
    ("CallLoweringCluster", ("src/mir/builder/calls/", "src/mir/builder/method_call_handlers.rs")),
    (
        "EmissionSsaPhiCluster",
        (
            "src/mir/builder/emission/",
            "src/mir/builder/ssa/",
            "src/mir/builder/phi.rs",
            "src/mir/builder/constants.rs",
        ),
    ),
    ("StatementValueConstructionCluster", ("src/mir/builder/stmts/", "src/mir/builder/vars/", "src/mir/builder/ops/", "src/mir/builder/fields", "src/mir/builder/record_values.rs", "src/mir/builder/builder_build.rs")),
    ("FastMemCluster", ("src/mir/builder/fastmem/",)),
]


class ReportError(RuntimeError):
    pass


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: Path) -> str:
    return sha256_bytes(path.read_bytes())


def hash_source_roots() -> str:
    digest = hashlib.sha256()
    for root in SOURCE_ROOTS:
        for path in sorted(root.rglob("*.rs")):
            digest.update(rel(path).encode("utf-8"))
            digest.update(b"\0")
            digest.update(sha256_file(path).encode("ascii"))
            digest.update(b"\0")
    return digest.hexdigest()


def normalize(text: str) -> str:
    return re.sub(r"\s+", " ", (text or "").strip())


def source_id(path: Path, symbol: str, line: int) -> str:
    return f"{rel(path)}::{symbol}:L{line}"


def extract_surfaces() -> list[dict[str, Any]]:
    surfaces: list[dict[str, Any]] = []
    for root in SOURCE_ROOTS:
        for path in sorted(root.rglob("*.rs")):
            text = path.read_text(encoding="utf-8")
            for match in FN_PATTERN.finditer(text):
                symbol = match.group("name")
                params = normalize(match.group("params"))
                ret = normalize(match.group("ret") or "")
                visibility = match.group("vis") or "private"
                line = text.count("\n", 0, match.start()) + 1
                receiver = "None"
                if params.startswith("&mut self"):
                    receiver = "&mut self"
                elif params.startswith("&self"):
                    receiver = "&self"
                elif params.startswith("self"):
                    receiver = "self"
                surfaces.append(
                    {
                        "source_id": source_id(path, symbol, line),
                        "source_path": rel(path),
                        "symbol": symbol,
                        "line": line,
                        "visibility": visibility,
                        "receiver": receiver,
                        "return_type": ret,
                        "params": params,
                        "is_public_surface": visibility.startswith("pub"),
                    }
                )
    return surfaces


def known_owner_for(surface: dict[str, Any]) -> tuple[str | None, str, list[str]]:
    path = surface["source_path"]
    symbol = surface["symbol"]
    ret = surface["return_type"]

    if path.endswith("variable_context.rs"):
        if symbol == "variable_map":
            return "variable_context", "ExactSymbol", [rel(REFERENCE_PROJECTION)]
        if symbol == "variable_map_mut":
            return "variable_context", "ExactSymbol", [rel(REFERENCE_PROJECTION)]
        return "variable_context", "FileScoped", [rel(REFERENCE_PROJECTION)]

    if path.endswith("module_lifecycle.rs") and symbol == "finalize_module":
        return "minimal_path_composed_execution_closure", "Heuristic", [rel(NATIVE_SEED_SURVEY)]

    if path.endswith("observer.rs") and symbol == "pop_function_region":
        return "function_region_stack_pop", "ExactSymbol", [rel(NATIVE_SEED_SURVEY)]

    if "&" in ret:
        return None, "None", []

    return None, "None", []


def classify(surface: dict[str, Any]) -> dict[str, Any]:
    owner, confidence, evidence = known_owner_for(surface)
    path = surface["source_path"]
    symbol = surface["symbol"]
    ret = surface["return_type"]

    classification = "UnmappedRustSurface"
    reason = "NoKnownOwnerEdge"
    replacement = None
    blockers: list[str] = []
    next_owner_kind = "OwnerEdgeClassification"
    next_card = "<OWNER>-OWNER-EDGE-CLASSIFICATION-001"

    if "/tests" in path or path.endswith("tests.rs") or "_test" in symbol:
        classification = "TestOnlySurface"
        reason = "TestOnlySurfaceIgnored"
        next_owner_kind = "None"
        next_card = None
    elif "debug" in path or "debug" in symbol:
        classification = "DebugOnlySurface"
        reason = "DebugOnlySurfaceIgnored"
        next_owner_kind = "None"
        next_card = None
    elif symbol == "variable_map":
        classification = "BorrowSurfacePolicyKnown"
        reason = "ReturnedReadBorrowCoveredByOwnedReadSnapshotProjection"
        replacement = "OwnedReadSnapshotProjection"
        next_owner_kind = "None"
        next_card = None
    elif symbol == "variable_map_mut":
        classification = "BorrowSurfacePolicyKnown"
        reason = "ReturnedMutableBorrowCoveredByExplicitMutationApiOnly"
        replacement = "ExplicitMutationApiOnly"
        next_owner_kind = "None"
        next_card = None
    elif ret.startswith("&mut") or ret.startswith("Option<&mut") or "-> &mut" in ret:
        classification = "BorrowSurfaceNeedsPolicy"
        reason = "ReturnedMutableBorrowPolicyMissing"
        blockers = ["BorrowSurfaceNeedsPolicy"]
        next_owner_kind = "BorrowProjectionPolicy"
        next_card = "<OWNER>-BORROW-PROJECTION-POLICY-001"
    elif ret.startswith("&") or ret.startswith("Option<&") or "-> &" in ret:
        classification = "BorrowSurfaceNeedsPolicy"
        reason = "ReturnedReadBorrowPolicyMissing"
        blockers = ["BorrowSurfaceNeedsPolicy"]
        next_owner_kind = "BorrowProjectionPolicy"
        next_card = "<OWNER>-BORROW-PROJECTION-POLICY-001"
    elif symbol == "finalize_module":
        classification = "CompositeSuspected"
        reason = "KnownFinalizeModuleContainsMultipleSemanticEdges"
        blockers = ["CompositeSuspected"]
        next_owner_kind = "None"
        next_card = None
    elif owner in {"variable_context", "function_region_stack_pop"}:
        classification = "MappedToKnownOwner"
        reason = "MappedToKnownOwnerEdge"
        next_owner_kind = "None"
        next_card = None
    elif surface["is_public_surface"]:
        classification = "MissingProjectionPolicy"
        reason = "PublicRustSurfaceMissingProjectionPolicy"
        blockers = ["MissingProjectionPolicy"]
        next_owner_kind = "ProjectionPolicy"
        next_card = "<OWNER>-PROJECTION-POLICY-001"
    else:
        classification = "IgnoredNonSemanticHelper"
        reason = "PrivateHelperSurfaceNotSelected"
        next_owner_kind = "None"
        next_card = None

    return {
        **surface,
        "known_owner_edge": owner,
        "owner_edge_confidence": confidence,
        "classification": classification,
        "reason_token": reason,
        "known_replacement": replacement,
        "blockers": blockers,
        "next_owner_kind": next_owner_kind,
        "next_card": next_card,
        "evidence_refs": evidence,
    }


def likely_owner_cluster(item: dict[str, Any]) -> str:
    if item["classification"] != "MissingProjectionPolicy":
        return "NotMissingProjectionPolicy"
    path = item["source_path"]
    for cluster, prefixes in OWNER_CLUSTER_RULES:
        if any(path.startswith(prefix) for prefix in prefixes):
            return cluster
    return "OtherMissingProjectionPolicyCluster"


def included_item(item: dict[str, Any]) -> bool:
    if item["is_public_surface"]:
        return True
    if item["classification"] in {"BorrowSurfaceNeedsPolicy", "BorrowSurfacePolicyKnown", "CompositeSuspected"}:
        return True
    if item["known_owner_edge"]:
        return True
    return False


def build_report() -> dict[str, Any]:
    seed_survey = read_json(NATIVE_SEED_SURVEY)
    family_manifest = read_json(FAMILY_MANIFEST)
    all_surfaces = extract_surfaces()
    classified = [classify(surface) for surface in all_surfaces]
    for item in classified:
        item["likely_owner_cluster"] = likely_owner_cluster(item)
    items = [item for item in classified if included_item(item)]
    known_owner_edges = {item["known_owner_edge"] for item in items if item["known_owner_edge"]}
    orphan_evidence_rows = [
        {
            "owner_edge_id": item.get("owner_edge_id"),
            "classification": item.get("classification"),
            "reason_token": "EvidenceRowHasNoExactSourceSurfaceJoin",
        }
        for item in seed_survey.get("scanned_items", [])
        if item.get("owner_edge_id") not in known_owner_edges
    ]

    counts: dict[str, int] = {}
    cluster_counts: dict[str, int] = {}
    for item in items:
        counts[item["classification"]] = counts.get(item["classification"], 0) + 1
        if item["classification"] == "MissingProjectionPolicy":
            cluster = item["likely_owner_cluster"]
            cluster_counts[cluster] = cluster_counts.get(cluster, 0) + 1

    candidate_classes = [
        "MissingProjectionPolicy",
        "CompositeNeedsDecomposition",
        "BorrowSurfaceNeedsPolicy",
        "MissingVerifierOrOracle",
        "UnmappedRustSurface",
    ]
    candidates = [item for item in items if item["classification"] in candidate_classes]

    if len(candidates) == 1:
        selected = candidates[0]
        decision = {
            "kind": {
                "MissingProjectionPolicy": "SelectMissingProjectionPolicy",
                "CompositeNeedsDecomposition": "SelectCompositeDecomposition",
                "BorrowSurfaceNeedsPolicy": "SelectBorrowPolicy",
                "MissingVerifierOrOracle": "SelectVerifierOrOracleRepair",
                "UnmappedRustSurface": "SelectOwnerEdgeClassification",
            }[selected["classification"]],
            "selected_source_id": selected["source_id"],
            "selected_next_card": selected["next_card"],
            "reason_token": f"ExactlyOne{selected['classification']}",
        }
    elif len(candidates) > 1:
        decision = {
            "kind": "KeepStopped",
            "selected_source_id": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "reason_token": "AmbiguousUnconvertedSurfaceCandidates",
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "selected_source_id": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "reason_token": "NoUnconvertedSurfaceCandidate",
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderCrateWideUnconvertedSurfaceReportV1",
        "token": "MIRBUILDER-CRATE-WIDE-UNCONVERTED-SURFACE-REPORT-001",
        "input": {
            "source_roots": [rel(path) for path in SOURCE_ROOTS],
            "scan_unit": "rust_function_or_method",
            "join_unit": "semantic_owner_edge",
            "scan_method": "regex_source_text_v0",
            "native_owner_seed_capability_survey": rel(NATIVE_SEED_SURVEY),
            "source_selfhost_family_guard_manifest": rel(FAMILY_MANIFEST),
            "current_state": rel(CURRENT_STATE),
        },
        "provenance": {
            "tool_version": "regex_source_text_v0",
            "source_root_hash": hash_source_roots(),
            "native_owner_seed_capability_survey_hash": sha256_file(NATIVE_SEED_SURVEY),
            "source_selfhost_family_guard_manifest_hash": sha256_file(FAMILY_MANIFEST),
            "variable_context_reference_projection_contract_hash": sha256_file(REFERENCE_PROJECTION),
        },
        "source_inventory": {
            "rust_surface_count": len(all_surfaces),
            "public_rust_surface_count": sum(1 for item in all_surfaces if item["is_public_surface"]),
            "reported_item_count": len(items),
            "seed_survey_decision": seed_survey.get("decision", {}).get("kind"),
            "family_manifest_rows": len(family_manifest.get("rows") or []),
        },
        "reverse_evidence_checks": {
            "known_owner_edge_count": len(known_owner_edges),
            "orphan_source_surface_count": sum(1 for item in items if not item["known_owner_edge"]),
            "orphan_evidence_row_count": len(orphan_evidence_rows),
            "orphan_evidence_rows": sorted(orphan_evidence_rows, key=lambda item: item["owner_edge_id"] or ""),
        },
        "owner_cluster_rules": [
            {"cluster": cluster, "path_prefixes": list(prefixes)}
            for cluster, prefixes in OWNER_CLUSTER_RULES
        ],
        "missing_projection_cluster_summary": [
            {"cluster": cluster, "count": count}
            for cluster, count in sorted(cluster_counts.items(), key=lambda item: (-item[1], item[0]))
        ],
        "reason_token_table": REASON_TOKEN_TABLE,
        "classification_enum": [
            "AlreadyConverted",
            "MappedToKnownOwner",
            "UnmappedRustSurface",
            "BorrowSurfacePolicyKnown",
            "BorrowSurfaceNeedsPolicy",
            "UnsupportedReturnedReadBorrow",
            "UnsupportedReturnedMutableBorrow",
            "CompositeSuspected",
            "CompositeNeedsDecomposition",
            "MissingProjectionPolicy",
            "MissingVerifierOrOracle",
            "MissingRouteOrArtifactEvidence",
            "GeneratedArtifactOnly",
            "SupportLaneOnly",
            "IgnoredNonSemanticHelper",
            "TestOnlySurface",
            "DebugOnlySurface",
        ],
        "items": sorted(items, key=lambda item: item["source_id"]),
        "summary": {
            "scanned_surface_count": len(items),
            "classified_once_count": len(items),
            "already_converted_count": counts.get("AlreadyConverted", 0),
            "mapped_to_known_owner_count": counts.get("MappedToKnownOwner", 0),
            "unmapped_count": counts.get("UnmappedRustSurface", 0),
            "borrow_policy_known_count": counts.get("BorrowSurfacePolicyKnown", 0),
            "borrow_policy_needed_count": counts.get("BorrowSurfaceNeedsPolicy", 0),
            "missing_projection_policy_count": counts.get("MissingProjectionPolicy", 0),
            "missing_verifier_or_oracle_count": counts.get("MissingVerifierOrOracle", 0),
            "composite_suspected_count": counts.get("CompositeSuspected", 0),
            "generated_artifact_only_count": counts.get("GeneratedArtifactOnly", 0),
            "support_lane_only_count": counts.get("SupportLaneOnly", 0),
            "ignored_helper_count": counts.get("IgnoredNonSemanticHelper", 0),
            "test_only_count": counts.get("TestOnlySurface", 0),
            "debug_only_count": counts.get("DebugOnlySurface", 0),
        },
        "decision": decision,
        "claims": {
            "tool_output_matches_checked_in_fixture": 1,
            "scan_unit_rust_function_or_method": 1,
            "join_unit_semantic_owner_edge": 1,
            "scan_method_regex_source_text_v0": 1,
            "rust_ast_parser_required": 0,
            "rustc_adapter_required": 0,
            "semantic_inference_beyond_existing_ssot": 0,
            "every_scanned_public_method_classified_exactly_once": 1,
            "every_unconverted_item_has_reason_token": 1,
            "every_reason_token_is_stable": 1,
            "owner_edge_confidence_recorded": 1,
            "likely_owner_cluster_recorded": 1,
            "missing_projection_items_clustered": 1,
            "heuristic_owner_edge_not_selectable": 1,
            "public_ignored_requires_reason": 1,
            "multiple_candidates_keep_stopped": 1,
            "borrow_policy_known_does_not_select_owner": 1,
            "composite_suspected_is_not_decomposition_proof": 1,
            "generated_artifact_only_is_not_native_edit_authority": 1,
            "support_lane_only_is_not_hako_adoption_candidate": 1,
            "manual_family_selection": 0,
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
    parser.add_argument("--check", action="store_true", help="Verify the checked-in report fixture.")
    args = parser.parse_args()

    result = build_report()
    rendered = stable_json(result)
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != rendered:
            raise ReportError("checked-in unconverted surface report fixture is stale")
    else:
        write_if_changed(OUTPUT, rendered)
        print(rendered, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
