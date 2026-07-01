#!/usr/bin/env python3
"""Normalize ID scalar operation tokens into behavior recipe effect classes."""

from __future__ import annotations

import argparse
import json
from collections import defaultdict
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-behavior-recipe-effect-coverage-basis-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-BEHAVIOR-RECIPE-EFFECT-COVERAGE-BASIS-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-ID-SCALAR-VERIFIER-INPUT-CONTRACT-BASIS-001"

ERROR_ORDER = FIXTURES / "mirbuilder-id-scalar-error-and-deterministic-order-basis-v0.json"
MUTATION_FRAME = FIXTURES / "mirbuilder-id-scalar-state-mutation-frame-basis-v0.json"
FILE_BOUNDARY = FIXTURES / "mirbuilder-id-scalar-native-seed-file-boundary-basis-v0.json"
OPERATIONS = FIXTURES / "mirbuilder-id-scalar-operation-vocabulary-inventory-v0.json"

EFFECT_CLASS = {
    "ContextRegistryConstruct": "OwnerStateWrite",
    "DiagnosticStringBuild": "DiagnosticBuild",
    "PhiInstructionDefine": "PhiInstructionAppend",
    "PhiInstructionDefineCurrentBlock": "PhiInstructionAppend",
    "PhiInstructionDefineFunction": "PhiInstructionAppend",
    "PhiInstructionPatch": "PhiInstructionPatch",
    "PhiLifecyclePatch": "PhiInstructionPatch",
    "PredicateRead": "PredicateRead",
    "VerifierContractCheck": "VerifierContractCheck",
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    error_order = read_json(ERROR_ORDER)
    mutation_frame = read_json(MUTATION_FRAME)
    file_boundary = read_json(FILE_BOUNDARY)
    operations = read_json(OPERATIONS)
    bounded_owners = {
        row["owner_edge_id"]
        for row in file_boundary.get("boundary_rows") or []
        if row.get("native_seed_file_boundary_derivable")
    }

    effect_rows = []
    class_to_tokens: dict[str, set[str]] = defaultdict(set)
    covered_tokens: set[str] = set()
    for candidate in operations.get("candidates") or []:
        if candidate["owner_edge_id"] not in bounded_owners:
            continue
        for row in candidate.get("operation_rows") or []:
            token = row["operation_token"]
            effect = EFFECT_CLASS[token]
            covered_tokens.add(token)
            class_to_tokens[effect].add(token)
            effect_rows.append(
                {
                    "owner_edge_id": candidate["owner_edge_id"],
                    "operation_token": token,
                    "effect_class": effect,
                    "source_id": row["source_id"],
                    "verifier_visible": True,
                    "requires_mutation_frame": effect in {
                        "OwnerStateWrite",
                        "PhiInstructionAppend",
                        "PhiInstructionPatch",
                    },
                    "requires_error_semantics": effect in {"DiagnosticBuild", "VerifierContractCheck"},
                    "requires_deterministic_order": effect in {
                        "OwnerStateWrite",
                        "PhiInstructionAppend",
                        "PhiInstructionPatch",
                    },
                }
            )

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarBehaviorRecipeEffectCoverageBasisV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "error_and_deterministic_order_basis": rel(ERROR_ORDER),
            "state_mutation_frame_basis": rel(MUTATION_FRAME),
            "native_seed_file_boundary_basis": rel(FILE_BOUNDARY),
            "operation_vocabulary_inventory": rel(OPERATIONS),
        },
        "provenance": {
            "error_and_deterministic_order_basis_hash": sha256_file(ERROR_ORDER),
            "state_mutation_frame_basis_hash": sha256_file(MUTATION_FRAME),
            "native_seed_file_boundary_basis_hash": sha256_file(FILE_BOUNDARY),
            "operation_vocabulary_inventory_hash": sha256_file(OPERATIONS),
        },
        "coverage_policy": {
            "operation_tokens_normalized_to_effect_classes": True,
            "all_bounded_owner_operation_tokens_covered": True,
            "behavior_recipe_materialization": False,
            "source_plan_materialization": False,
        },
        "effect_class_summary": [
            {"effect_class": effect, "operation_tokens": sorted(tokens)}
            for effect, tokens in sorted(class_to_tokens.items())
        ],
        "effect_rows": effect_rows,
        "candidate_pool": {
            "bounded_owner_count": len(bounded_owners),
            "operation_token_count": len(covered_tokens),
            "effect_class_count": len(class_to_tokens),
            "effect_row_count": len(effect_rows),
            "mutation_frame_count": (mutation_frame.get("candidate_pool") or {}).get(
                "mutation_frame_count"
            ),
            "error_semantics_count": (error_order.get("candidate_pool") or {}).get(
                "error_semantics_count"
            ),
        },
        "decision": {
            "kind": "BehaviorRecipeEffectCoverageBasisDefined",
            "reason_token": "IdScalarBehaviorRecipeEffectCoverageDefined",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
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
        print("mirbuilder-id-scalar-behavior-recipe-effect-coverage-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
