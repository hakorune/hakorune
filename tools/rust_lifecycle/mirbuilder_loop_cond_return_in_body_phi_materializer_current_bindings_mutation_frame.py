#!/usr/bin/env python3
"""Materialize the LoopCondReturnInBodyPhiMaterializer current_bindings frame."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
INPUT = FIXTURES / "mirbuilder-borrow-surface-returned-mutable-borrow-policy-v0.json"
OUTPUT = (
    FIXTURES
    / "mirbuilder-loop-cond-return-in-body-phi-materializer-current-bindings-mutation-frame-v0.json"
)
PIPELINE = ROOT / "src/mir/builder/control_flow/plan/features/loop_cond_return_in_body_pipeline.rs"
CARRIER_MERGE = ROOT / "src/mir/builder/control_flow/plan/features/carrier_merge.rs"
NORMALIZER_LOWERING = ROOT / "src/mir/builder/control_flow/plan/normalizer/loop_body_lowering.rs"

TOKEN = (
    "MIRBUILDER-LOOP-COND-RETURN-IN-BODY-PHI-MATERIALIZER-"
    "CURRENT-BINDINGS-MUTATION-FRAME-001"
)
NEXT_CARD = "MIRBUILDER-MULTI-AXIS-DIAGNOSTIC-CLUSTER-RESOLUTION-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def source_order_sections() -> list[dict[str, Any]]:
    return [
        {
            "source_path": rel(PIPELINE),
            "markers": [
                "let mut body_plans = lower_return_in_body_block(",
                "phi_materializer.current_bindings_mut()",
                "fn lower_return_in_body_block(",
                "for item in &recipe.items",
                "plans.extend(lower_stmt_ast(",
                "fn lower_stmt_ast(",
                "ASTNode::Assignment",
                "lower_assignment_stmt(",
                "ASTNode::Local",
                "lower_local_init_stmt(",
                "ASTNode::MethodCall",
                "loop_body_lowering::lower_method_call_stmt(",
                "ASTNode::FunctionCall",
                "loop_body_lowering::lower_function_call_stmt(",
                "ASTNode::Print",
                "PlanNormalizer::lower_value_ast(expression, builder, current_bindings)?",
                "ASTNode::If",
                "lower_if_with_join(",
                "ASTNode::Return",
                "parts::entry::lower_return_with_effects(",
                "fn lower_if_with_join(",
                "lower_if_with_branch_lowerers_and_updates(",
                "carrier_updates.insert(join.name.clone(), join.dst)",
            ],
        },
        {
            "source_path": rel(CARRIER_MERGE),
            "markers": [
                "pub(in crate::mir::builder) fn lower_assignment_stmt(",
                "current_bindings.iter()",
                "loop_body_lowering::lower_assignment_stmt(",
                "carrier_updates.insert(name.clone(), value_id)",
                "current_bindings.insert(name.clone(), value_id)",
                "builder.variable_ctx.variable_map.insert(name, value_id)",
                "pub(in crate::mir::builder) fn lower_local_init_stmt(",
                "loop_body_lowering::lower_local_init_values(",
                "current_bindings.insert(name.clone(), value_id)",
                "builder.variable_ctx.variable_map.insert(name, value_id)",
            ],
        },
        {
            "source_path": rel(NORMALIZER_LOWERING),
            "markers": [
                "pub(in crate::mir::builder) fn lower_assignment_stmt(",
                "PlanNormalizer::lower_value_ast(value, builder, phi_bindings)?",
                "Ok((Some((name.clone(), value_id)), effects))",
                "pub(in crate::mir::builder) fn lower_local_init_values(",
                "PlanNormalizer::lower_value_ast(init_node.as_ref(), builder, phi_bindings)?",
                "inits.push((name.to_string(), value_id))",
                "pub(in crate::mir::builder) fn lower_method_call_stmt(",
                "PlanNormalizer::lower_value_ast(arg, builder, phi_bindings)?",
                "pub(in crate::mir::builder) fn lower_function_call_stmt(",
            ],
        },
    ]


def build_contract() -> dict[str, Any]:
    policy = read_json(INPUT)
    replacement = policy["replacement_policy"]

    return {
        "schema_version": 0,
        "kind": "MirBuilderLoopCondReturnInBodyPhiMaterializerCurrentBindingsMutationFrameV1",
        "token": TOKEN,
        "input_state": {
            "borrow_policy_fixture": rel(INPUT),
            "replacement_id": replacement["replacement_id"],
            "selected_policy": policy["decision"]["selected_policy"],
            "strict_raw_borrow_policy": policy["strict_policy"]["raw_returned_mutable_borrow"],
        },
        "provenance": {
            "borrow_policy_fixture_hash": sha256_file(INPUT),
            "pipeline_source_hash": sha256_file(PIPELINE),
            "carrier_merge_source_hash": sha256_file(CARRIER_MERGE),
            "normalizer_loop_body_lowering_source_hash": sha256_file(NORMALIZER_LOWERING),
        },
        "mutation_frame_contract": {
            "frame_kind": "BoundedOwnerMutationFrame",
            "replacement_policy": "BoundedWithMapOperation",
            "owner": "LoopCondReturnInBodyPhiMaterializer",
            "owned_field": "current_bindings",
            "entry_surface": "current_bindings_mut",
            "bounded_callsite": "lower_return_in_body_block",
            "state_inputs": [
                "LoopCondReturnInBodyPhiMaterializer.current_bindings",
                "carrier_phis",
                "carrier_step_phis",
                "carrier_updates",
                "builder.variable_ctx.variable_map",
                "recipe.items",
                "recipe.body",
            ],
            "state_outputs": [
                "LoopCondReturnInBodyPhiMaterializer.current_bindings",
                "carrier_updates",
                "builder.variable_ctx.variable_map",
                "lowered body plans",
            ],
            "read_only_inputs": [
                "carrier_phis",
                "carrier_step_phis",
                "recipe.items",
                "recipe.body",
            ],
            "local_only_state": [
                "plans vector for lowered body recipes",
                "statement dispatch frame",
            ],
            "mutation_order": [
                "EnterBoundedCurrentBindingsFrame",
                "IterateRecipeItemsInSourceOrder",
                "DispatchStatementAst",
                "LowerAssignmentThroughCarrierMerge",
                "LowerLocalInitThroughCarrierMerge",
                "LowerMethodCallEffectsWithCurrentBindings",
                "LowerFunctionCallEffectsWithCurrentBindings",
                "LowerPrintEffectsWithCurrentBindings",
                "LowerIfWithJoinedBranchBindings",
                "RecordCarrierUpdatesFromJoinedCarrierPhis",
                "LowerReturnWithCurrentBindings",
                "ReturnLoweredBodyPlans",
                "ExitFrameWithoutAliasEscape",
            ],
            "allowed_operations": [
                "MapGetCopied",
                "MapSet",
                "MapRemoveIfExisting",
                "MapClearOnlyIfSourceEvidenceExists",
            ],
            "forbidden_operations": [
                "ReturnMutableMapAlias",
                "StoreMutableBorrow",
                "CallerOwnedMutableAlias",
                "RustLifetimeSyntaxTransport",
                "RuntimeFallback",
            ],
        },
        "source_order_sections": source_order_sections(),
        "decision": {
            "kind": "SelectNextDiagnosticClusterResolution",
            "selected_next_card": NEXT_CARD,
            "reason_token": "BoundedCurrentBindingsMutationFrameContractReady",
        },
        "claims": {
            "bounded_mutation_frame_contract_ready": 1,
            "raw_mutable_alias_selected": 0,
            "returned_mutable_borrow_allowed": 0,
            "stored_borrow_allowed": 0,
            "caller_owned_mutable_alias": 0,
            "hako_generation": 0,
            "hako_shadow_projector_selected": 0,
            "hako_adopted_decision": 0,
            "native_seed_materialization": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
            "manual_family_selection": 0,
            "manual_axis_selection": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in contract fixture.")
    args = parser.parse_args()

    output = stable_json(build_contract())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-loop-cond-return-in-body-phi-materializer-current-bindings-mutation-frame unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
