#!/usr/bin/env python3
"""Define typed evidence index policy for tied ID scalar derivable owners."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
PHASE = ROOT / "docs/development/current/main/phases/phase-296x"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-typed-evidence-index-policy-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-TYPED-EVIDENCE-INDEX-POLICY-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-ID-SCALAR-OPERATION-VOCABULARY-AUTHORITY-SPLIT-001"

BASIS_CARD = PHASE / "2040-MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-BASIS-001.md"
DERIVABILITY = FIXTURES / "mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution-003-v0.json"
SURFACES = FIXTURES / "mirbuilder-id-scalar-source-surface-inventory-v0.json"
OPERATIONS = FIXTURES / "mirbuilder-id-scalar-operation-vocabulary-inventory-v0.json"
OWNER_SCOPE = FIXTURES / "mirbuilder-id-scalar-owner-scope-boundedness-resolution-002-v0.json"
FILE_BOUNDARY = FIXTURES / "mirbuilder-id-scalar-native-seed-file-boundary-basis-v0.json"
ID_DOMAIN = FIXTURES / "mirbuilder-id-scalar-id-domain-boundary-basis-v0.json"
MUTATION = FIXTURES / "mirbuilder-id-scalar-state-mutation-frame-basis-v0.json"
ERROR_ORDER = FIXTURES / "mirbuilder-id-scalar-error-and-deterministic-order-basis-v0.json"
EFFECT = FIXTURES / "mirbuilder-id-scalar-behavior-recipe-effect-coverage-basis-v0.json"
VERIFIER = FIXTURES / "mirbuilder-id-scalar-verifier-input-contract-basis-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def by_owner(rows: list[dict[str, Any]], key: str = "owner_edge_id") -> dict[str, dict[str, Any]]:
    return {row[key]: row for row in rows}


def owner_prefix(owner_edge_id: str) -> str:
    return owner_edge_id.replace("::", ".")


def build_entry(kind: str, source_fixture: Path, typed_ref_count: int, complete: bool) -> dict[str, Any]:
    return {
        "artifact_kind": kind,
        "source_fixture": rel(source_fixture),
        "typed_ref_count": typed_ref_count,
        "typed_refs_complete": complete,
        "mention_only_owner_edge_text": False,
    }


def build_fixture() -> dict[str, Any]:
    derivability = read_json(DERIVABILITY)
    surfaces = read_json(SURFACES)
    operations = read_json(OPERATIONS)
    owner_scope = read_json(OWNER_SCOPE)
    file_boundary = read_json(FILE_BOUNDARY)
    id_domain = read_json(ID_DOMAIN)
    mutation = read_json(MUTATION)
    error_order = read_json(ERROR_ORDER)
    effect = read_json(EFFECT)
    verifier = read_json(VERIFIER)

    surface_by_owner = by_owner(surfaces.get("candidates") or [])
    operation_by_owner = by_owner(operations.get("candidates") or [])
    scope_by_owner = by_owner(owner_scope.get("candidates") or [])
    boundary_by_owner = by_owner(file_boundary.get("boundary_rows") or [])

    tied = [
        row["owner_edge_id"]
        for row in derivability.get("candidates") or []
        if row.get("selection_eligible")
    ]

    index_rows = []
    complete_count = 0
    for owner in tied:
        surface_row = surface_by_owner.get(owner) or {}
        operation_row = operation_by_owner.get(owner) or {}
        scope_row = scope_by_owner.get(owner) or {}
        boundary_row = boundary_by_owner.get(owner) or {}
        prefix = owner_prefix(owner)

        id_domain_count = sum(
            1
            for row in id_domain.get("domain_boundaries") or []
            if owner in (row.get("owner_edge_counts") or {})
        )
        mutation_count = sum(
            1 for row in mutation.get("mutation_frames") or [] if row.get("owner_edge_id") == owner
        )
        deterministic_order_count = sum(
            1
            for row in error_order.get("deterministic_order") or []
            if str(row.get("subject") or "").startswith(prefix)
        )
        error_semantics_count = id_domain_count + sum(
            1
            for row in error_order.get("error_semantics") or []
            if str(row.get("error_semantics_id") or "").startswith(owner)
        )
        effect_count = sum(
            1 for row in effect.get("effect_rows") or [] if row.get("owner_edge_id") == owner
        )

        entries = [
            build_entry(
                "SourceSurfaceInventory",
                SURFACES,
                int(surface_row.get("required_source_surface_count") or 0),
                bool(surface_row.get("required_source_surfaces_complete")),
            ),
            build_entry(
                "OperationVocabularyInventory",
                OPERATIONS,
                len(operation_row.get("operation_rows") or []),
                bool(operation_row.get("operation_vocabulary_complete")),
            ),
            build_entry(
                "OwnerScopeBoundedness",
                OWNER_SCOPE,
                1 if scope_row.get("owner_scope_bounded") else 0,
                bool(scope_row.get("owner_scope_bounded")),
            ),
            build_entry(
                "NativeSeedFileBoundary",
                FILE_BOUNDARY,
                1 if boundary_row.get("native_seed_file_boundary_derivable") else 0,
                bool(boundary_row.get("native_seed_file_boundary_derivable")),
            ),
            build_entry("IdDomainBoundary", ID_DOMAIN, id_domain_count, id_domain_count > 0),
            build_entry("StateMutationFrame", MUTATION, mutation_count, mutation_count > 0),
            build_entry(
                "ErrorSemantics",
                ERROR_ORDER,
                error_semantics_count,
                error_semantics_count > 0,
            ),
            build_entry(
                "DeterministicOrder",
                ERROR_ORDER,
                deterministic_order_count,
                deterministic_order_count > 0,
            ),
            build_entry(
                "BehaviorRecipeEffectCoverage",
                EFFECT,
                effect_count,
                effect_count > 0,
            ),
            build_entry(
                "VerifierInputContract",
                VERIFIER,
                (verifier.get("candidate_pool") or {}).get("input_fact_set_count") or 0,
                bool((verifier.get("candidate_pool") or {}).get("input_fact_set_count")),
            ),
        ]
        complete = all(entry["typed_refs_complete"] for entry in entries)
        if complete:
            complete_count += 1
        index_rows.append(
            {
                "owner_edge_id": owner,
                "typed_evidence_complete": complete,
                "evidence_entries": entries,
                "selection_eligible": False,
                "blocked_by": [],
            }
        )

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarTypedEvidenceIndexPolicyV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "derivable_owner_discriminator_basis_card": rel(BASIS_CARD),
            "derivability_rerun_003": rel(DERIVABILITY),
        },
        "provenance": {
            "derivable_owner_discriminator_basis_card_hash": sha256_file(BASIS_CARD),
            "derivability_rerun_003_hash": sha256_file(DERIVABILITY),
            "source_surface_inventory_hash": sha256_file(SURFACES),
            "operation_vocabulary_inventory_hash": sha256_file(OPERATIONS),
            "owner_scope_boundedness_rerun_002_hash": sha256_file(OWNER_SCOPE),
            "native_seed_file_boundary_basis_hash": sha256_file(FILE_BOUNDARY),
            "id_domain_boundary_basis_hash": sha256_file(ID_DOMAIN),
            "state_mutation_frame_basis_hash": sha256_file(MUTATION),
            "error_and_deterministic_order_basis_hash": sha256_file(ERROR_ORDER),
            "behavior_recipe_effect_coverage_basis_hash": sha256_file(EFFECT),
            "verifier_input_contract_basis_hash": sha256_file(VERIFIER),
        },
        "policy": {
            "typed_evidence_index_required": True,
            "mention_only_owner_edge_text_is_not_evidence": True,
            "owner_edge_substring_search_allowed": False,
            "fixture_path_substring_search_allowed": False,
            "typed_fixture_refs_only": True,
            "source_plan_materialization": False,
        },
        "typed_evidence_rows": index_rows,
        "candidate_pool": {
            "input_tied_owner_count": len(tied),
            "typed_evidence_complete_owner_count": complete_count,
            "selection_eligible_count": 0,
        },
        "decision": {
            "kind": "PolicyDefined",
            "reason_token": "IdScalarTypedEvidenceIndexPolicyDefined",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "owner_edge_text_mention_as_evidence": 0,
            "fixture_path_substring_as_evidence": 0,
            "manual_owner_selection": 0,
            "source_plan_materialization": 0,
            "behavior_recipe_materialization": 0,
            "verifier_result_materialization": 0,
            "derived_artifact_seed_draft_materialization": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
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
    parser.add_argument("--check", action="store_true", help="Verify checked-in fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-id-scalar-typed-evidence-index-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
