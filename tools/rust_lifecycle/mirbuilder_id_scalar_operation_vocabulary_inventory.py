#!/usr/bin/env python3
"""Inventory operation vocabulary for ID scalar source surfaces."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-operation-vocabulary-inventory-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-OPERATION-VOCABULARY-INVENTORY-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-002"

SURFACE_INVENTORY = FIXTURES / "mirbuilder-id-scalar-source-surface-inventory-v0.json"
BASIS = FIXTURES / "mirbuilder-id-scalar-source-plan-derivation-basis-v0.json"


ROLE_OPERATIONS = {
    "analysis_predicate": "PredicateRead",
    "builder_phi_definition": "PhiInstructionDefine",
    "builder_phi_patch": "PhiInstructionPatch",
    "contract_validator": "VerifierContractCheck",
    "core_phi_info_builder": "PhiInfoBuild",
    "current_block_phi_definition": "PhiInstructionDefineCurrentBlock",
    "diagnostic_formatter": "DiagnosticStringBuild",
    "edgecfg_compose": "EdgeCfgCompose",
    "facts_or_recognizer": "PredicateRead",
    "function_phi_definition": "PhiInstructionDefineFunction",
    "join_payload": "JoinPayloadBuild",
    "lifecycle_scope_patch": "PhiLifecyclePatch",
    "loop_binding_builder": "LoopBindingBuild",
    "merge_contract_or_logging": "MergeContractOrDiagnostic",
    "merge_rewriter": "MergeRewriteDecision",
    "plan_helper": "PlanListBuild",
    "planner_gate_or_fact_count": "PlannerGateOrFactRead",
    "planner_session_or_rule_dispatch": "PlannerSessionDispatch",
    "recipe_index": "RecipeIndexRead",
    "route_local_closure_constructor": "RouteLocalClosureConstruct",
    "route_rewrite_helper": "RouteRewriteBuild",
    "route_verify_helper": "RouteVerifyPredicate",
    "skeleton_or_wiring": "SkeletonOrWiringBuild",
    "trace_or_debug": "TraceDiagnosticEmission",
    "verify_diagnostic": "VerifierDiagnosticState",
    "verify_observability": "VerifierObservabilityEmit",
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def infer_unclassified_operation(surface: dict[str, Any]) -> str:
    symbol = surface.get("symbol") or ""
    return_type = surface.get("return_type") or ""
    if symbol.startswith("has_") or symbol.startswith("matches_") or symbol.startswith("collect_"):
        return "PredicateRead"
    if symbol == "len":
        return "ReadOnlyLength"
    if symbol == "with_plugin_sigs":
        return "ContextRegistryConstruct"
    if symbol == "apply_fallthrough_continue_exit":
        return "RouteLocalCleanupMutationFrame"
    if symbol == "new":
        return "RouteLocalClosureConstruct"
    if return_type.startswith("bool"):
        return "PredicateRead"
    if return_type.startswith("Result"):
        return "RouteOrPlanResultBuild"
    return "UnknownOperation"


def operation_for(surface: dict[str, Any]) -> tuple[str, str]:
    role = surface.get("role") or "UnclassifiedSurfaceRole"
    if role in ROLE_OPERATIONS:
        return ROLE_OPERATIONS[role], "RoleMapped"
    inferred = infer_unclassified_operation(surface)
    return inferred, "SymbolReturnTypeMapped" if inferred != "UnknownOperation" else "Unknown"


def build_fixture() -> dict[str, Any]:
    source_inventory = read_json(SURFACE_INVENTORY)
    basis = read_json(BASIS)

    candidates = []
    operation_counts: dict[str, int] = {}
    unknown_count = 0
    row_count = 0
    for candidate in source_inventory.get("candidates") or []:
        rows = []
        candidate_unknown = 0
        for surface in candidate.get("surfaces") or []:
            operation, authority = operation_for(surface)
            if operation == "UnknownOperation":
                unknown_count += 1
                candidate_unknown += 1
            operation_counts[operation] = operation_counts.get(operation, 0) + 1
            row_count += 1
            rows.append(
                {
                    "source_id": surface.get("source_id"),
                    "source_path": surface.get("source_path"),
                    "symbol": surface.get("symbol"),
                    "role": surface.get("role"),
                    "return_type": surface.get("return_type", ""),
                    "operation_token": operation,
                    "classification_authority": authority,
                    "nominal_id_domain_required": True,
                    "raw_i64_interchangeability": 0,
                    "evidence_ref": surface.get("evidence_ref"),
                }
            )
        complete = candidate_unknown == 0 and bool(rows)
        candidates.append(
            {
                "owner_edge_id": candidate.get("owner_edge_id"),
                "source_surface_count": len(rows),
                "operation_vocabulary_complete": complete,
                "unknown_operation_count": candidate_unknown,
                "operation_rows": rows,
                "blocked_by": [] if complete else ["UnknownOperationVocabulary"],
                "next_card": NEXT_CARD if complete else None,
            }
        )

    all_complete = bool(candidates) and unknown_count == 0
    decision = {
        "kind": "SelectSourcePlanAndRecipeDerivabilityRerun" if all_complete else "KeepStopped",
        "reason_token": (
            "IdScalarOperationVocabularyInventoried"
            if all_complete
            else "IdScalarOperationVocabularyIncomplete"
        ),
        "selected_next_card": NEXT_CARD if all_complete else DESIGN_STOP,
    }

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarOperationVocabularyInventoryV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "source_surface_inventory": rel(SURFACE_INVENTORY),
            "source_plan_derivation_basis": rel(BASIS),
        },
        "provenance": {
            "source_surface_inventory_hash": sha256_file(SURFACE_INVENTORY),
            "source_plan_derivation_basis_hash": sha256_file(BASIS),
        },
        "previous_state": {
            "previous_token": source_inventory.get("token"),
            "previous_reason_token": (source_inventory.get("decision") or {}).get("reason_token"),
            "previous_selected_next_card": (source_inventory.get("decision") or {}).get("selected_next_card"),
            "basis_token": basis.get("token"),
        },
        "inventory_policy": {
            "classification_authority": "surface_role_then_symbol_return_type_rule_table",
            "manual_operation_selection": False,
            "operation_vocabulary_inventory_only": True,
            "source_plan_materialization": False,
        },
        "operation_vocabulary": [
            {"operation_token": key, "surface_count": operation_counts[key]}
            for key in sorted(operation_counts)
        ],
        "candidates": candidates,
        "candidate_pool": {
            "input_candidate_count": len(candidates),
            "operation_surface_count": row_count,
            "operation_vocabulary_token_count": len(operation_counts),
            "operation_vocabulary_complete_candidate_count": len(
                [row for row in candidates if row["operation_vocabulary_complete"]]
            ),
            "unknown_operation_count": unknown_count,
            "selection_eligible_for_source_plan_count": 0,
        },
        "decision": decision,
        "claims": {
            "operation_vocabulary_inventory_defined": 1,
            "manual_operation_selection": 0,
            "source_plan_materialization": 0,
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
            "raw_i64_interchangeability": 0,
            "nominal_id_erasure": 0,
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
        print("mirbuilder-id-scalar-operation-vocabulary-inventory unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
