#!/usr/bin/env python3
"""Project finalize_module PHI input materialization from live Rust source.

This is a plan-only capability for the prepared-state
`build_module(AST Literal Integer(0))` frontier. It owns only the delegated
`phi_input_materializer::materialize_all_phi_inputs` call and the SSOT helper
shape that rematerializes edge-local PHI inputs. It does not claim dev birth
verification, module function insertion, semantic refresh, full finalize,
generated Hako, backend routes, or runtime behavior.
"""

from __future__ import annotations

import argparse
from copy import deepcopy
from pathlib import Path
from typing import Any

from context_fact_extraction import report_or_emit, require


ROOT = Path(__file__).resolve().parents[2]
FIXTURE = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-phi-input-materialization-plan-v0.json"
)
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"
PHI_INPUT_MATERIALIZER = ROOT / "src/mir/builder/ssa/phi_input_materializer.rs"
PHI_RETURN_TYPE_INFERENCE_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-phi-return-type-inference-plan-v0.json"
)


def _read(path: Path) -> str:
    return path.read_text()


def _read_json(path: Path) -> dict[str, Any]:
    import json

    return json.loads(path.read_text())


def _function_body(source: str, signature: str) -> str:
    start = source.find(signature)
    require(start >= 0, f"missing function signature: {signature}")
    brace = source.find("{", start)
    require(brace >= 0, f"missing function body brace: {signature}")
    depth = 0
    for index in range(brace, len(source)):
        char = source[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return source[brace + 1 : index]
    raise AssertionError(f"unterminated function body: {signature}")


def _require_order(text: str, markers: list[str], label: str) -> list[dict[str, Any]]:
    cursor = -1
    rows: list[dict[str, Any]] = []
    for marker in markers:
        index = text.find(marker, cursor + 1)
        require(index >= 0, f"{label}: missing or out-of-order marker: {marker}")
        rows.append({"marker": marker, "byte_offset": index})
        cursor = index
    return rows


def extract_plan() -> dict[str, Any]:
    lifecycle = _read(MODULE_LIFECYCLE)
    materializer_source = _read(PHI_INPUT_MATERIALIZER)
    phi_return = _read_json(PHI_RETURN_TYPE_INFERENCE_PLAN)
    finalize = _function_body(
        lifecycle, "pub(super) fn finalize_module(&mut self, result_value: ValueId)"
    )
    materialize_body = _function_body(
        materializer_source,
        "pub(in crate::mir::builder) fn materialize_all_phi_inputs",
    )

    finalize_order = _require_order(
        finalize,
        [
            "phi_type_inference::infer_return_type_from_phi",
            "phi_input_materializer::materialize_all_phi_inputs",
            '"finalize_module"',
            "if crate::config::env::using_is_dev()",
        ],
        "MirBuilder::finalize_module PHI input materialization",
    )
    materializer_order = _require_order(
        materialize_body,
        [
            "prune_unused_phi_instructions(func)",
            "complete_missing_self_carried_phi_inputs(func)",
            "for (block_id, block) in &func.blocks",
            "if let MirInstruction::Phi { inputs, .. } = inst",
            "PhiInputMaterializationAnalysis::new(func)",
            "PhiInputRematContext::new(pred)",
            "rematerialize_for_pred(func, &analysis, incoming, context, \"phi\", remat_ctx)?",
            "*slot = materialized;",
            "Ok(changed)",
        ],
        "phi_input_materializer materialize_all_phi_inputs",
    )
    require(
        phi_return.get("non_claims", {}).get("phi_input_materialization") == 0,
        "PhiReturnTypeInference must not claim PHI input materialization",
    )
    for marker in [
        "struct PhiInputMaterializationAnalysis",
        "def_blocks: HashMap<ValueId, BasicBlockId>",
        "DominatorTree",
        "memo: HashMap<ValueId, ValueId>",
        "visiting: HashSet<ValueId>",
        "[freeze:contract][ssa/phi_input/remat_cycle]",
        "[freeze:contract][ssa/phi_input/non_rematerializable]",
        "block.add_instruction_before_terminator(remat_inst);",
    ]:
        require(marker in materializer_source, f"PHI materializer marker missing: {marker}")

    return {
        "schema_version": 0,
        "kind": "MirBuilderPhiInputMaterializationPlanV1",
        "subject": "MirBuilder::finalize_module phi_input_materializer::materialize_all_phi_inputs",
        "source_authority": {
            "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
            "provider": "src/mir/builder/ssa/phi_input_materializer.rs::materialize_all_phi_inputs",
            "predecessor_plan": "mirbuilder-phi-return-type-inference-plan-v0.json",
        },
        "execution_profile": {
            "input": "ASTNode::Literal(Integer(0))",
            "function_transport": "MirFunctionPreparedMain",
            "context": "finalize_module",
        },
        "observed_source_order": {
            "finalize_module": finalize_order,
            "materializer": materializer_order,
        },
        "materialization_steps": [
            "PruneUnusedPhiInstructions",
            "CompleteMissingSelfCarriedPhiInputs",
            "CollectPhiInputWorklist",
            "BuildDefBlocksAndDominators",
            "RematerializeIncomingPerPredWithMemo",
            "RewritePhiInputSlots",
            "ReturnChangedCount",
        ],
        "available_capabilities": [
            "PhiInputMaterialization",
        ],
        "result_contract": {
            "mutates": [
                "function.blocks",
                "function.next_value_id",
            ],
            "entrypoint": "phi_input_materializer::materialize_all_phi_inputs",
            "minimal_path_expected_result": "Result<usize, String>",
        },
        "non_claims": {
            "dev_birth_verification": 0,
            "module_function_insertion": 0,
            "condition_fn_injection": 0,
            "all_functions_phi_materialization": 0,
            "semantic_refresh": 0,
            "full_finalize_module": 0,
            "generated_hako_artifact": 0,
            "backend_route_changed": 0,
            "abi_changed": 0,
            "runtime_fallback": 0,
            "mainline_selected": 0,
        },
    }


def validate_plan(plan: dict[str, Any]) -> None:
    require(
        plan["kind"] == "MirBuilderPhiInputMaterializationPlanV1",
        "wrong PHI input materialization plan kind",
    )
    require(
        "PhiInputMaterialization" in plan["available_capabilities"],
        "missing PhiInputMaterialization capability",
    )
    profile = plan["execution_profile"]
    require(profile["function_transport"] == "MirFunctionPreparedMain", "function transport drift")
    require(profile["context"] == "finalize_module", "materialization context drift")
    require(
        plan["materialization_steps"]
        == [
            "PruneUnusedPhiInstructions",
            "CompleteMissingSelfCarriedPhiInputs",
            "CollectPhiInputWorklist",
            "BuildDefBlocksAndDominators",
            "RematerializeIncomingPerPredWithMemo",
            "RewritePhiInputSlots",
            "ReturnChangedCount",
        ],
        f"materialization step drift: {plan['materialization_steps']}",
    )
    result = plan["result_contract"]
    require(result["entrypoint"] == "phi_input_materializer::materialize_all_phi_inputs", "entrypoint drift")
    require(result["minimal_path_expected_result"] == "Result<usize, String>", "expectation drift")
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("missing capability", ["available_capabilities"], []),
        ("step order drift", ["materialization_steps"], list(reversed(plan["materialization_steps"]))),
        ("dev birth claim drift", ["non_claims", "dev_birth_verification"], 1),
    ]
    for label, path, value in probes:
        mutated = deepcopy(plan)
        cursor: Any = mutated
        for key in path[:-1]:
            cursor = cursor[key]
        cursor[path[-1]] = value
        try:
            validate_plan(mutated)
        except AssertionError:
            continue
        raise AssertionError(f"drift probe did not fail: {label}")


def build_plan() -> dict[str, Any]:
    plan = extract_plan()
    validate_plan(plan)
    return plan


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--reference", type=Path, default=FIXTURE)
    parser.add_argument("--emit-json", action="store_true")
    parser.add_argument("--check-reference", action="store_true")
    parser.add_argument("--drift-probes", action="store_true")
    args = parser.parse_args()

    plan = build_plan()
    if args.drift_probes:
        run_drift_probes(plan)

    return report_or_emit(
        facts=plan,
        reference=args.reference,
        check_reference=args.check_reference,
        emit_json=args.emit_json,
        report=[
            ("output_contract", "rust-lifecycle-mirbuilder-phi-input-materialization-v0"),
            ("mirbuilder_phi_input_materialization", "green"),
            ("capability", "PhiInputMaterialization"),
            ("entrypoint", plan["result_contract"]["entrypoint"]),
            ("materialization_steps", ",".join(plan["materialization_steps"])),
            ("dev_birth_verification_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())
