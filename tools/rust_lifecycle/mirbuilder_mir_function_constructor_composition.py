#!/usr/bin/env python3
"""Project MirFunction constructor composition from live Rust source.

This covers `MirFunction::new` together with its nested entry `BasicBlock::new`
dependency. It does not claim function body lowering, instruction emission,
parameter setup compatibility fallback, or finalize behavior.
"""

from __future__ import annotations

import argparse
import json
from copy import deepcopy
from pathlib import Path
from typing import Any

from context_fact_extraction import report_or_emit, require


ROOT = Path(__file__).resolve().parents[2]
FIXTURE = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mir-function-constructor-composition-plan-v0.json"
)
FUNCTION_IMPL = ROOT / "src/mir/function/function_impl.rs"
BASIC_BLOCK = ROOT / "src/mir/basic_block.rs"
FUNCTION_TYPES = ROOT / "src/mir/function/types.rs"


def _read(path: Path) -> str:
    return path.read_text()


def _method_body(source: str, signature: str) -> str:
    start = source.find(signature)
    require(start >= 0, f"missing method signature: {signature}")
    brace = source.find("{", start)
    require(brace >= 0, f"missing method body brace: {signature}")
    depth = 0
    for index in range(brace, len(source)):
        char = source[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return source[brace + 1 : index]
    raise AssertionError(f"unterminated method body: {signature}")


def extract_plan() -> dict[str, Any]:
    function_impl = _read(FUNCTION_IMPL)
    basic_block = _read(BASIC_BLOCK)
    function_types = _read(FUNCTION_TYPES)
    fn_new = _method_body(
        function_impl, "pub fn new(signature: FunctionSignature, entry_block: BasicBlockId) -> Self"
    )
    bb_new = _method_body(basic_block, "pub fn new(id: BasicBlockId) -> Self")

    for marker in [
        "let mut blocks = HashMap::new();",
        "blocks.insert(entry_block, BasicBlock::new(entry_block));",
        "let param_count = signature.params.len() as u32;",
        "let total_value_ids = param_count;",
        "let initial_counter = total_value_ids.max(1);",
        "let mut pre_params = Vec::new();",
        "for i in 0..total_value_ids",
        "pre_params.push(ValueId::new(i));",
        "signature,",
        "blocks,",
        "entry_block,",
        "locals: Vec::new(),",
        "params: pre_params,",
        "next_value_id: initial_counter,",
        "metadata: FunctionMetadata::default(),",
    ]:
        require(marker in fn_new, f"MirFunction::new marker drift: {marker}")

    for marker in [
        "id,",
        "instructions: Vec::new(),",
        "instruction_spans: Vec::new(),",
        "terminator: None,",
        "terminator_span: None,",
        "predecessors: BTreeSet::new()",
        "successors: BTreeSet::new()",
        "effects: EffectMask::PURE,",
        "reachable: false,",
        "sealed: false,",
        "return_env: None",
        "return_env_layout: None",
    ]:
        require(marker in bb_new, f"BasicBlock::new marker drift: {marker}")

    for marker in [
        "pub signature: FunctionSignature,",
        "pub blocks: HashMap<BasicBlockId, BasicBlock>,",
        "pub entry_block: BasicBlockId,",
        "pub params: Vec<ValueId>,",
        "pub next_value_id: u32,",
    ]:
        require(marker in function_types, f"MirFunction shape marker drift: {marker}")

    return {
        "schema_version": 0,
        "kind": "MirFunctionConstructorCompositionPlanV1",
        "subject": "hakorune_mir::MirFunction::new",
        "source_authority": {
            "function_constructor": "src/mir/function/function_impl.rs::MirFunction::new",
            "basic_block_constructor": "src/mir/basic_block.rs::BasicBlock::new",
            "shape": "src/mir/function/types.rs::MirFunction",
        },
        "constructor_signature": {
            "params": [
                {"name": "signature", "rust_type": "FunctionSignature", "transport": "FunctionSignaturePrepared"},
                {"name": "entry_block", "rust_type": "BasicBlockId", "transport": "BasicBlockIdAsI64"},
            ],
            "return": {"rust_type": "MirFunction", "transport": "MirFunctionConstructorShell"},
        },
        "composition": {
            "entry_block_child": {
                "constructor": "BasicBlock::new",
                "argument": "entry_block",
                "transport": "BasicBlockConstructorShell",
            },
            "blocks": {
                "initializer": "HashMap::new",
                "insertion": "entry_block -> BasicBlock::new(entry_block)",
                "transport": "EntryBlockOnlyFunctionBlockTable",
            },
            "params": {
                "source": "signature.params.len()",
                "range": "[0, param_count)",
                "value_transport": "ValueIdAsI64",
                "prepopulation": "ValueId::new(i)",
            },
            "next_value_id": {
                "seed": "max(param_count, 1)",
                "transport": "ValueIdCounterAsI64",
            },
            "locals": {"initializer": "Vec::new", "transport": "EmptyLocalTypeList"},
            "metadata": {"initializer": "FunctionMetadata::default", "transport": "FunctionMetadataDefaultShell"},
        },
        "basic_block_defaults": {
            "instructions": "Vec::new",
            "instruction_spans": "Vec::new",
            "terminator": None,
            "terminator_span": None,
            "predecessors": "BTreeSet::new",
            "successors": "BTreeSet::new",
            "effects": "EffectMask::PURE",
            "reachable": False,
            "sealed": False,
            "return_env": None,
            "return_env_layout": None,
        },
        "available_capabilities": [
            "MirFunctionConstructorTransport",
            "PreparedStateInstall",
        ],
        "non_claims": {
            "separate_block_only_claim": 0,
            "function_body_lowering": 0,
            "instruction_emission": 0,
            "parameter_setup_compatibility_fallback": 0,
            "reserve_parameter_value_ids_call": 0,
            "function_finalization": 0,
            "generated_hako_artifact": 0,
            "backend_route_changed": 0,
            "abi_changed": 0,
            "runtime_fallback": 0,
        },
    }


def validate_plan(plan: dict[str, Any]) -> None:
    require(plan["kind"] == "MirFunctionConstructorCompositionPlanV1", "wrong function constructor plan kind")
    caps = set(plan["available_capabilities"])
    require("MirFunctionConstructorTransport" in caps, "missing function constructor capability")
    require("PreparedStateInstall" in caps, "missing prepared-state install capability")
    require(
        plan["composition"]["entry_block_child"]["constructor"] == "BasicBlock::new",
        "entry block constructor drift",
    )
    require(plan["composition"]["params"]["range"] == "[0, param_count)", "parameter range drift")
    require(plan["composition"]["next_value_id"]["seed"] == "max(param_count, 1)", "counter seed drift")
    require(plan["basic_block_defaults"]["effects"] == "EffectMask::PURE", "basic block effect drift")
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("counter seed drift", ["composition", "next_value_id", "seed"], "param_count"),
        ("block split claim drift", ["non_claims", "separate_block_only_claim"], 1),
        ("missing prepared install capability", ["available_capabilities"], ["MirFunctionConstructorTransport"]),
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
            ("output_contract", "rust-lifecycle-mir-function-constructor-composition-v0"),
            ("mir_function_constructor_composition", "green"),
            ("capability", "MirFunctionConstructorTransport"),
            ("prepared_state_install", "green"),
            ("separate_block_only_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())
