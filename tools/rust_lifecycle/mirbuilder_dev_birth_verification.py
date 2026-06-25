#!/usr/bin/env python3
"""Project finalize_module dev NewBox birth verification from live Rust source.

This is a plan-only capability for the prepared-state
`build_module(AST Literal Integer(0))` frontier. It owns only the guarded
developer warning pass that checks nearby `birth` calls after `NewBox`
instructions. It does not claim module insertion, condition_fn injection,
region cleanup, metadata publication, semantic refresh, full finalize,
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
    / "mirbuilder-dev-birth-verification-plan-v0.json"
)
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"
PHI_INPUT_MATERIALIZATION_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-phi-input-materialization-plan-v0.json"
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
    phi_input = _read_json(PHI_INPUT_MATERIALIZATION_PLAN)
    finalize = _function_body(
        lifecycle, "pub(super) fn finalize_module(&mut self, result_value: ValueId)"
    )

    finalize_order = _require_order(
        finalize,
        [
            "phi_input_materializer::materialize_all_phi_inputs",
            "if crate::config::env::using_is_dev()",
            "&& config::env::stageb_dev_verify_enabled()",
            "&& crate::config::env::cli_verbose_enabled()",
            "module.add_function(function);",
        ],
        "MirBuilder::finalize_module dev birth verification",
    )
    verifier_order = _require_order(
        finalize,
        [
            "let mut warn_count = 0usize;",
            "for (_bid, bb) in function.blocks.iter()",
            "if let MirInstruction::NewBox",
            'if box_type == "StageBDriverBox"',
            'if box_type != "StringBox"',
            'let expect_tail = format!("{}.birth/{}", box_type, args.len());',
            "while j < insns.len() && j <= idx + 3",
            "method == \"birth\" && recv == dst",
            "last_const_name = Some(s.clone());",
            "prev == &expect_tail",
            "[warn] dev verify: NewBox",
            "[warn] dev verify: NewBox→birth invariant warnings:",
        ],
        "dev birth verification warning pass",
    )
    require(
        phi_input.get("non_claims", {}).get("dev_birth_verification") == 0,
        "PhiInputMaterialization must not claim dev birth verification",
    )
    for marker in [
        "MirInstruction::NewBox",
        "StageBDriverBox",
        "StringBox",
        "Call(Method birth)",
        "Global(expect_tail) compatibility path",
        "ring0.log.warn",
    ]:
        require(marker in finalize, f"dev birth verification marker missing: {marker}")

    return {
        "schema_version": 0,
        "kind": "MirBuilderDevBirthVerificationPlanV1",
        "subject": "MirBuilder::finalize_module dev NewBox birth verification",
        "source_authority": {
            "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
            "predecessor_plan": "mirbuilder-phi-input-materialization-plan-v0.json",
        },
        "execution_profile": {
            "input": "ASTNode::Literal(Integer(0))",
            "function_transport": "MirFunctionPreparedMain",
            "context": "finalize_module",
        },
        "observed_source_order": {
            "finalize_module": finalize_order,
            "warning_pass": verifier_order,
        },
        "guard_conditions": [
            "using_is_dev",
            "stageb_dev_verify_enabled",
            "cli_verbose_enabled",
        ],
        "verification_steps": [
            "IterateFunctionBlocks",
            "ScanNewBoxInstructions",
            "SkipStageBDriverBox",
            "SkipStringBox",
            "ExpectBirthTailByBoxTypeAndArity",
            "LookAheadThreeInstructions",
            "AcceptMethodBirthOnSameReceiver",
            "AcceptConstStringGlobalCompatibilityPath",
            "WarnOnMissingBirth",
            "WarnSummaryWhenAnyMissing",
        ],
        "available_capabilities": [
            "DevBirthVerification",
        ],
        "result_contract": {
            "mutates": [],
            "side_effect": "dev_warning_only",
            "entrypoint": "MirBuilder::finalize_module dev birth verification block",
            "minimal_path_expected_result": "NoErrorReturn",
        },
        "non_claims": {
            "module_function_insertion": 0,
            "condition_fn_injection": 0,
            "all_functions_phi_materialization": 0,
            "region_stack_pop": 0,
            "slot_registry_release": 0,
            "metadata_publication": 0,
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
        plan["kind"] == "MirBuilderDevBirthVerificationPlanV1",
        "wrong dev birth verification plan kind",
    )
    require(
        "DevBirthVerification" in plan["available_capabilities"],
        "missing DevBirthVerification capability",
    )
    profile = plan["execution_profile"]
    require(profile["function_transport"] == "MirFunctionPreparedMain", "function transport drift")
    require(profile["context"] == "finalize_module", "verification context drift")
    require(
        plan["guard_conditions"]
        == ["using_is_dev", "stageb_dev_verify_enabled", "cli_verbose_enabled"],
        f"guard condition drift: {plan['guard_conditions']}",
    )
    require(
        plan["verification_steps"]
        == [
            "IterateFunctionBlocks",
            "ScanNewBoxInstructions",
            "SkipStageBDriverBox",
            "SkipStringBox",
            "ExpectBirthTailByBoxTypeAndArity",
            "LookAheadThreeInstructions",
            "AcceptMethodBirthOnSameReceiver",
            "AcceptConstStringGlobalCompatibilityPath",
            "WarnOnMissingBirth",
            "WarnSummaryWhenAnyMissing",
        ],
        f"verification step drift: {plan['verification_steps']}",
    )
    result = plan["result_contract"]
    require(result["mutates"] == [], "dev birth verification must not mutate MIR state")
    require(result["side_effect"] == "dev_warning_only", "side effect drift")
    require(
        result["entrypoint"] == "MirBuilder::finalize_module dev birth verification block",
        "entrypoint drift",
    )
    require(result["minimal_path_expected_result"] == "NoErrorReturn", "expectation drift")
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("missing capability", ["available_capabilities"], []),
        ("guard condition drift", ["guard_conditions"], ["using_is_dev"]),
        ("module insertion claim drift", ["non_claims", "module_function_insertion"], 1),
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
            ("output_contract", "rust-lifecycle-mirbuilder-dev-birth-verification-v0"),
            ("mirbuilder_dev_birth_verification", "green"),
            ("capability", "DevBirthVerification"),
            ("entrypoint", plan["result_contract"]["entrypoint"]),
            ("guard_conditions", ",".join(plan["guard_conditions"])),
            ("module_function_insertion_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())
