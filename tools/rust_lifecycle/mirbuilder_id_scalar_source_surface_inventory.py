#!/usr/bin/env python3
"""Inventory ID scalar source surfaces from projection-policy evidence."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-source-surface-inventory-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-SOURCE-SURFACE-INVENTORY-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-ID-SCALAR-OPERATION-VOCABULARY-INVENTORY-001"

BASIS = FIXTURES / "mirbuilder-id-scalar-source-plan-derivation-basis-v0.json"
DERIVABILITY = FIXTURES / "mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def source_complete(surface: dict[str, Any]) -> bool:
    return bool(surface.get("source_id") and surface.get("source_path") and surface.get("symbol"))


def projection_surface_role(surface: dict[str, Any], fixture: dict[str, Any]) -> str:
    explicit_role = surface.get("role") or surface.get("helper_bucket")
    if explicit_role:
        return explicit_role

    axes = fixture.get("selection_axes") or {}
    selected_policy = fixture.get("selected_policy") or {}
    shape = axes.get("shape_signature")
    policy = selected_policy.get("policy")

    if shape == "shape.context_registry" and policy == "KeepParentOwner":
        return "context_registry_constructor"
    if shape == "shape.loop_true_break_continue":
        return "route_local_cleanup_mutation_frame"
    return "UnclassifiedSurfaceRole"


def normalize_surface(surface: dict[str, Any], owner_edge_id: str, ref: str, fixture: dict[str, Any]) -> dict[str, Any]:
    return {
        "source_id": surface.get("source_id"),
        "source_path": surface.get("source_path"),
        "symbol": surface.get("symbol"),
        "visibility": surface.get("visibility", ""),
        "return_type": surface.get("return_type", ""),
        "role": projection_surface_role(surface, fixture),
        "known_owner_edge": surface.get("known_owner_edge") or owner_edge_id,
        "owner_edge_confidence": surface.get("owner_edge_confidence", "FixtureMapped"),
        "evidence_ref": ref,
    }


def surfaces_for_candidate(candidate: dict[str, Any]) -> tuple[list[dict[str, Any]], list[str]]:
    owner_edge_id = candidate["owner_edge_id"]
    rows: list[dict[str, Any]] = []
    refs = []
    seen = set()
    for ref in candidate.get("projection_policy_refs") or []:
        path = ROOT / ref
        refs.append(ref)
        fixture = read_json(path)
        for surface in fixture.get("source_surfaces") or []:
            row = normalize_surface(surface, owner_edge_id, ref, fixture)
            key = (row["source_id"], row["evidence_ref"])
            if key in seen:
                continue
            seen.add(key)
            rows.append(row)
    rows.sort(key=lambda row: (row["source_path"] or "", row["source_id"] or "", row["evidence_ref"]))
    return rows, refs


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    derivability = read_json(DERIVABILITY)
    basis_decision = basis.get("decision") or {}

    candidates = []
    incomplete = 0
    surface_count = 0
    for candidate in derivability.get("candidates") or []:
        surfaces, refs = surfaces_for_candidate(candidate)
        complete = bool(surfaces) and all(source_complete(row) for row in surfaces)
        if not complete:
            incomplete += 1
        surface_count += len(surfaces)
        candidates.append(
            {
                "owner_edge_id": candidate["owner_edge_id"],
                "owner_edge_confidence": candidate.get("owner_edge_confidence"),
                "projection_policy_refs": refs,
                "required_source_surface_count": len(surfaces),
                "required_source_surfaces_complete": complete,
                "source_surface_confidence": "FixtureJoinedSourceSurfaces" if complete else "IncompleteFixtureJoin",
                "owner_scope": "NotEvaluatedAtThisStage",
                "blocked_by": [] if complete else ["RequiredSourceSurfacesIncomplete"],
                "surfaces": surfaces,
                "next_card": NEXT_CARD if complete else None,
            }
        )

    all_complete = bool(candidates) and incomplete == 0
    decision = {
        "kind": "SelectOperationVocabularyInventory" if all_complete else "KeepStopped",
        "reason_token": (
            "IdScalarRequiredSourceSurfacesInventoried"
            if all_complete
            else "IdScalarRequiredSourceSurfacesIncomplete"
        ),
        "selected_next_card": NEXT_CARD if all_complete else DESIGN_STOP,
    }

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarSourceSurfaceInventoryV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "source_plan_derivation_basis": rel(BASIS),
            "source_plan_derivability_resolution": rel(DERIVABILITY),
        },
        "provenance": {
            "source_plan_derivation_basis_hash": sha256_file(BASIS),
            "source_plan_derivability_resolution_hash": sha256_file(DERIVABILITY),
        },
        "previous_state": {
            "previous_token": basis.get("token"),
            "previous_reason_token": basis_decision.get("reason_token"),
            "previous_selected_next_card": basis_decision.get("selected_next_card"),
            "input_candidate_count": (derivability.get("candidate_pool") or {}).get("input_candidate_count"),
        },
        "inventory_policy": {
            "derivation_authority": "projection_policy_fixture_source_surfaces",
            "manual_surface_selection": False,
            "source_surface_inventory_only": True,
            "source_plan_materialization": False,
            "operation_vocabulary_evaluated": False,
        },
        "candidates": candidates,
        "candidate_pool": {
            "input_candidate_count": len(candidates),
            "required_source_surface_count": surface_count,
            "surface_complete_candidate_count": len([row for row in candidates if row["required_source_surfaces_complete"]]),
            "surface_incomplete_candidate_count": incomplete,
            "selection_eligible_for_source_plan_count": 0,
        },
        "decision": decision,
        "claims": {
            "source_surface_inventory_defined": 1,
            "manual_surface_selection": 0,
            "source_plan_materialization": 0,
            "operation_vocabulary_evaluated": 0,
            "behavior_recipe_materialization": 0,
            "verifier_result_materialization": 0,
            "derived_artifact_seed_draft_materialization": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "generated_artifact_as_native_edit_authority": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-id-scalar-source-surface-inventory unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
