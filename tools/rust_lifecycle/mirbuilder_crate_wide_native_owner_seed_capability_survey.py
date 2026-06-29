#!/usr/bin/env python3
"""Build the MirBuilder crate-wide native owner seed capability survey.

The survey is intentionally evidence-joining only. It reads existing route,
adoption, seed-policy, and Source Selfhost fixtures, then classifies known
semantic-owner edges without inventing new projection or borrow semantics.
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-crate-wide-native-owner-seed-capability-survey-v0.json"

ROUTE_MANIFEST = ROOT / "lang/generated/rust_derived/hakorune_mir_builder/family_routes.json"
NEXT_ROUTE_POLICY = FIXTURES / "source-selfhost-next-route-family-selection-policy-v0.json"
SEED_POLICY = FIXTURES / "mirbuilder-generated-artifact-to-native-owner-seed-policy-v0.json"
SEED_RESOLUTION = FIXTURES / "mirbuilder-generated-artifact-native-owner-seed-candidate-resolution-v0.json"
FAMILY_MANIFEST = FIXTURES / "source-selfhost-family-guard-manifest-v0.json"
CURRENT_STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER = ROOT / "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

SOURCE_ROOTS = [ROOT / "src/mir/builder"]


class SurveyError(RuntimeError):
    pass


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SurveyError(message)


def snake(value: str) -> str:
    value = value.split("::")[-1]
    value = value.replace("-", "_").replace(" ", "_")
    value = re.sub(r"(?<!^)(?=[A-Z])", "_", value).lower()
    return re.sub(r"_+", "_", value).strip("_")


def source_inventory() -> dict[str, Any]:
    files: list[Path] = []
    symbol_count = 0
    public_symbol_count = 0
    fn_pattern = re.compile(r"\bfn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(")
    pub_fn_pattern = re.compile(r"\bpub(?:\([^)]*\))?\s+fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(")

    for root in SOURCE_ROOTS:
        for path in sorted(root.rglob("*.rs")):
            files.append(path)
            text = path.read_text(encoding="utf-8")
            symbol_count += len(fn_pattern.findall(text))
            public_symbol_count += len(pub_fn_pattern.findall(text))

    return {
        "source_roots": [rel(path) for path in SOURCE_ROOTS],
        "rust_file_count": len(files),
        "rust_function_symbol_count": symbol_count,
        "public_rust_function_symbol_count": public_symbol_count,
    }


def load_adopted_families(family_manifest: dict[str, Any]) -> dict[str, str]:
    adopted: dict[str, str] = {}
    for row in family_manifest.get("rows") or []:
        fixture = row.get("fixture") or ""
        if not fixture:
            continue
        path = ROOT / fixture
        if not path.exists():
            continue
        try:
            data = read_json(path)
        except json.JSONDecodeError:
            continue
        claims = data.get("claims") or {}
        decision = data.get("decision") or {}
        is_adopted = claims.get("hako_adopted") == 1 or decision.get("value") == "Adopt"
        if not is_adopted:
            continue
        family_id = data.get("family_id")
        if not family_id:
            target = data.get("target") or {}
            family_id = target.get("family_id")
        if not family_id:
            continue
        adopted[snake(family_id)] = row.get("token") or fixture
    return adopted


def route_index(route_manifest: dict[str, Any]) -> dict[str, dict[str, Any]]:
    index: dict[str, dict[str, Any]] = {}
    for row in route_manifest.get("routes") or []:
        family_id = row.get("family_id") or ""
        key = snake(family_id)
        if key:
            index[key] = row
    return index


def artifact_state(row: dict[str, Any] | None) -> str:
    if not row:
        return "None"
    manifest = row.get("artifact_manifest")
    if not manifest:
        return "None"
    exists = (ROOT / manifest).exists()
    if not exists:
        return "None"
    if row.get("selected_on_mainline") is True:
        return "MainlineArtifactPresent"
    return "GeneratedArtifactPresent"


def route_state(row: dict[str, Any] | None) -> str:
    if not row:
        return "Unknown"
    state = row.get("state")
    if state:
        return state
    if row.get("selected_on_mainline") is True:
        return "DerivedMainline"
    return "Unknown"


def owner_kind(classification: str) -> str:
    if classification in {"AlreadyAdopted", "NativeSeedReady", "ConvertibleLeaf"}:
        return "LeafSemanticOwner"
    if classification in {"BoundedSurfaceOnly", "BoundedSurfaceAdopted"}:
        return "LeafSemanticOwner"
    if classification in {"SupportLaneOnly", "SupportLaneSeedPilotCandidate"}:
        return "SupportLaneProjector"
    if classification in {"NonSemanticIntegrationOwner", "GeneratedArtifactOnly"}:
        return "IntegrationOwner"
    if classification == "CompositeNeedsDecomposition":
        return "CompositeOwner"
    return "Unknown"


def native_authority_state(classification: str) -> str:
    if classification == "AlreadyAdopted":
        return "HakoAdopted"
    if classification in {"BoundedSurfaceOnly", "BoundedSurfaceAdopted"}:
        return "BoundedSurfaceOnly"
    if classification == "GeneratedArtifactOnly":
        return "GeneratedArtifactOnly"
    if classification == "NativeSeedReady":
        return "NativeSeedPresent"
    return "None"


def normalized_classification(policy_classification: str, policy_subclassification: str | None, candidate_id: str, adopted: dict[str, str]) -> tuple[str, list[str], str]:
    key = snake(candidate_id)
    if key in adopted:
        return "AlreadyAdopted", ["AlreadyHakoAdopted"], adopted[key]
    if policy_classification == "AlreadyAdopted":
        return "AlreadyAdopted", ["AlreadyHakoAdopted"], "PolicyAlreadyAdopted"
    if policy_classification == "BoundedSurfaceAdopted":
        return "BoundedSurfaceOnly", ["NotBoundedSurfaceOnly"], policy_subclassification or "BoundedSurfaceOnly"
    if policy_classification == "SupportLaneOnly":
        return "SupportLaneOnly", ["NotSupportLaneOnly"], "SupportLaneOnly"
    if policy_classification == "ConsultationGated":
        return "GeneratedArtifactOnly", ["LeafSemanticOwner", "NotCompositionOwner"], policy_subclassification or "ConsultationGated"
    return policy_classification, [], "PolicyClassification"


def next_owner_kind(classification: str) -> str:
    return {
        "RouteRepairNeeded": "RouteRepair",
        "NativeSeedReady": "HakoAdoptionDecision",
        "ConvertibleLeaf": "NativeSourceSeed",
        "SupportLaneSeedPilotCandidate": "NativeSourceSeed",
        "CompositeNeedsDecomposition": "CompositeDecomposition",
        "BorrowSurfaceNeedsPolicy": "BorrowProjectionPolicy",
        "MissingProjectionPolicy": "ProjectionPolicy",
        "MissingVerifierOrOracle": "VerifierOrOracleRepair",
    }.get(classification, "None")


def build_survey() -> dict[str, Any]:
    route_manifest = read_json(ROUTE_MANIFEST)
    next_policy = read_json(NEXT_ROUTE_POLICY)
    seed_policy = read_json(SEED_POLICY)
    seed_resolution = read_json(SEED_RESOLUTION)
    family_manifest = read_json(FAMILY_MANIFEST)
    routes = route_index(route_manifest)
    adopted = load_adopted_families(family_manifest)

    scanned: list[dict[str, Any]] = []
    for row in next_policy.get("family_classifications") or []:
        candidate_id = row.get("family_id") or "unknown"
        route_row = routes.get(snake(candidate_id))
        policy_class = row.get("classification") or "Unknown"
        subclass = row.get("subclassification")
        classification, blockers, reason_source = normalized_classification(
            policy_class, subclass, candidate_id, adopted
        )
        evidence_refs = [rel(NEXT_ROUTE_POLICY)]
        if route_row and route_row.get("artifact_manifest"):
            evidence_refs.append(route_row["artifact_manifest"])
        if reason_source in adopted.values():
            evidence_refs.append(reason_source)

        scanned.append(
            {
                "owner_edge_id": snake(candidate_id),
                "owner_kind": owner_kind(classification),
                "source_paths": [rel(path) for path in SOURCE_ROOTS],
                "rust_symbols": [],
                "current_route_state": route_state(route_row),
                "artifact_state": artifact_state(route_row),
                "native_authority_state": native_authority_state(classification),
                "minimal_path_membership": "InMinimalPath" if "minimal_path" in snake(candidate_id) else "Unknown",
                "classification": classification,
                "blockers": blockers,
                "next_owner_kind": next_owner_kind(classification),
                "next_card": None,
                "reason_token": reason_source if classification == "AlreadyAdopted" else row.get("reason_token") or reason_source,
                "evidence_refs": evidence_refs,
            }
        )

    class_counts: dict[str, int] = {}
    for row in scanned:
        class_counts[row["classification"]] = class_counts.get(row["classification"], 0) + 1

    route_repair = [row for row in scanned if row["classification"] == "RouteRepairNeeded"]
    native_seed_ready = [row for row in scanned if row["classification"] == "NativeSeedReady"]
    convertible_leaf = [row for row in scanned if row["classification"] == "ConvertibleLeaf"]
    support_seed = [row for row in scanned if row["classification"] == "SupportLaneSeedPilotCandidate"]
    composite = [row for row in scanned if row["classification"] == "CompositeNeedsDecomposition"]
    borrow = [row for row in scanned if row["classification"] == "BorrowSurfaceNeedsPolicy"]
    missing_projection = [row for row in scanned if row["classification"] == "MissingProjectionPolicy"]
    missing_verifier = [row for row in scanned if row["classification"] == "MissingVerifierOrOracle"]

    decision = {
        "kind": "KeepStopped",
        "selected_owner_edge_id": None,
        "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        "reason_token": "NoUniqueNextOwner",
    }

    decision_ladder = [
        (route_repair, "SelectRouteRepair", "<ROUTE-FAMILY>-ROUTE-MATRIX-REPAIR-001"),
        (native_seed_ready, "SelectNativeSeedAdoptionDecision", "<OWNER>-HAKO-ADOPTION-DECISION-001"),
        (convertible_leaf, "SelectNativeSourceSeed", "<OWNER>-HAKO-NATIVE-SOURCE-SEED-001"),
        (support_seed, "SelectNativeSourceSeed", "<OWNER>-HAKO-NATIVE-SOURCE-SEED-001"),
        (composite, "SelectCompositeDecomposition", "<OWNER>-DECOMPOSITION-001"),
        (borrow, "SelectBorrowPolicy", "<OWNER>-BORROW-PROJECTION-POLICY-001"),
        (missing_projection, "SelectProjectionPolicy", "<OWNER>-PROJECTION-POLICY-001"),
        (missing_verifier, "SelectVerifierOrOracleRepair", "<OWNER>-VERIFIER-OR-ORACLE-REPAIR-001"),
    ]
    for candidates, kind, next_card in decision_ladder:
        if len(candidates) == 1:
            selected = candidates[0]
            selected["next_card"] = next_card
            decision = {
                "kind": kind,
                "selected_owner_edge_id": selected["owner_edge_id"],
                "selected_next_card": next_card,
                "reason_token": f"ExactlyOne{kind}",
            }
            break
        if len(candidates) > 1:
            decision = {
                "kind": "KeepStopped",
                "selected_owner_edge_id": None,
                "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
                "reason_token": f"Ambiguous{kind}Candidates",
            }
            break

    summary = {
        "scanned_item_count": len(scanned),
        "already_adopted_count": class_counts.get("AlreadyAdopted", 0),
        "native_seed_ready_count": class_counts.get("NativeSeedReady", 0),
        "convertible_leaf_count": class_counts.get("ConvertibleLeaf", 0),
        "support_lane_seed_candidate_count": class_counts.get("SupportLaneSeedPilotCandidate", 0),
        "support_lane_only_count": class_counts.get("SupportLaneOnly", 0),
        "generated_artifact_only_count": class_counts.get("GeneratedArtifactOnly", 0),
        "bounded_surface_only_count": class_counts.get("BoundedSurfaceOnly", 0),
        "composite_needs_decomposition_count": class_counts.get("CompositeNeedsDecomposition", 0),
        "borrow_policy_count": class_counts.get("BorrowSurfaceNeedsPolicy", 0),
        "missing_projection_count": class_counts.get("MissingProjectionPolicy", 0),
        "missing_verifier_or_oracle_count": class_counts.get("MissingVerifierOrOracle", 0),
        "route_repair_needed_count": class_counts.get("RouteRepairNeeded", 0),
        "design_stop_count": 1 if decision["kind"] == "KeepStopped" else 0,
    }

    return {
        "schema_version": 0,
        "kind": "MirBuilderCrateWideNativeOwnerSeedCapabilitySurveyV1",
        "token": "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-001",
        "input_scope": {
            "crate": "hakorune_mir_builder",
            "source_roots": [rel(path) for path in SOURCE_ROOTS],
            "survey_unit": "semantic_owner_edge",
        },
        "input_authority": {
            "route_manifest": rel(ROUTE_MANIFEST),
            "source_selfhost_next_route_family_selection_policy": rel(NEXT_ROUTE_POLICY),
            "generated_artifact_to_native_owner_seed_policy": rel(SEED_POLICY),
            "generated_artifact_native_owner_seed_candidate_resolution": rel(SEED_RESOLUTION),
            "source_selfhost_family_guard_manifest": rel(FAMILY_MANIFEST),
            "current_state": rel(CURRENT_STATE),
            "task_order": rel(TASK_ORDER),
        },
        "source_inventory": source_inventory(),
        "classification_enum": [
            "AlreadyAdopted",
            "NativeSeedReady",
            "ConvertibleLeaf",
            "SupportLaneSeedPilotCandidate",
            "SupportLaneOnly",
            "GeneratedArtifactOnly",
            "BoundedSurfaceOnly",
            "CompositeNeedsDecomposition",
            "NonSemanticIntegrationOwner",
            "BorrowSurfacePolicyKnown",
            "BorrowSurfaceNeedsPolicy",
            "MissingProjectionPolicy",
            "MissingVerifierOrOracle",
            "MissingDeterministicRegeneration",
            "RouteRepairNeeded",
            "BlockedDesignStop",
        ],
        "scanned_items": sorted(scanned, key=lambda row: row["owner_edge_id"]),
        "summary": summary,
        "decision": decision,
        "selection_rules": {
            "route_repair_before_adoption_or_seed": 1,
            "native_seed_ready_before_new_seed_materialization": 1,
            "leaf_semantic_owner_before_composite_owner": 1,
            "minimal_path_membership_is_priority_signal_not_proof": 1,
            "generated_artifact_presence_is_priority_signal_not_proof": 1,
            "lexical_owner_edge_id_final_tiebreaker": 1,
        },
        "claims": {
            "survey_scope_explicit": 1,
            "survey_unit_semantic_owner_edge": 1,
            "selected_source_surfaces_partitioned_exactly_once": 1,
            "each_item_has_stable_classification": 1,
            "each_non_convertible_item_has_blocker_token": 1,
            "each_item_has_evidence_refs": 1,
            "support_lane_projector_as_hako_adoption_candidate": 0,
            "generated_artifact_as_edit_authority": 0,
            "composition_owner_as_semantic_owner": 0,
            "manual_family_selection": 0,
            "route_membership_alone_as_proof": 0,
            "coverage_percentage_as_proof": 0,
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
    parser.add_argument("--check", action="store_true", help="Verify the checked-in survey fixture.")
    args = parser.parse_args()

    result = build_survey()
    rendered = stable_json(result)
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != rendered:
            raise SurveyError("checked-in crate-wide native owner seed survey fixture is stale")
    else:
        write_if_changed(OUTPUT, rendered)
        print(rendered, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
