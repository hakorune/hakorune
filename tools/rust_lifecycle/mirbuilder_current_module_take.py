#!/usr/bin/env python3
"""Project finalize_module current_module take from live Rust source.

This is a plan-only capability for the prepared-state
`build_module(AST Literal Integer(0))` frontier. It owns only the edge that
transports the prepared `current_module` shell into finalize via
`current_module.take().unwrap()`. It does not claim typed-value verification,
current function take, full finalize, generated Hako, backend routes, or
runtime behavior.
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
    / "mirbuilder-current-module-take-plan-v0.json"
)
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"
MODULE_SHELL_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mir-module-minimal-shell-transport-plan-v0.json"
)
RETURN_TYPE_PUBLICATION_PLAN = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mirbuilder-return-type-publication-plan-v0.json"
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
    module_shell = _read_json(MODULE_SHELL_PLAN)
    return_type_publication = _read_json(RETURN_TYPE_PUBLICATION_PLAN)
    prepare = _function_body(lifecycle, "pub(super) fn prepare_module(&mut self) -> Result<(), String>")
    finalize = _function_body(
        lifecycle, "pub(super) fn finalize_module(&mut self, result_value: ValueId)"
    )

    prepare_order = _require_order(
        prepare,
        [
            'let mut module = MirModule::new("main".to_string());',
            "self.current_module = Some(module);",
        ],
        "MirBuilder::prepare_module current_module install",
    )
    finalize_order = _require_order(
        finalize,
        [
            "let mut module = self.current_module.take().unwrap();",
            "verify_typed_values_are_defined",
            "let mut function = self.scope_ctx.current_function.take().unwrap();",
        ],
        "MirBuilder::finalize_module current_module take",
    )

    require(
        module_shell.get("directability", {}).get("capability") == "MirModuleMinimalShellTransport",
        "module shell plan must provide MirModuleMinimalShellTransport",
    )
    require(
        return_type_publication.get("non_claims", {}).get("module_take") == 0,
        "ReturnTypePublication must not claim module take",
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderCurrentModuleTakePlanV1",
        "subject": "MirBuilder::finalize_module current_module.take().unwrap()",
        "source_authority": {
            "prepare": "src/mir/builder/module_lifecycle.rs::MirBuilder::prepare_module",
            "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
            "module_shell_plan": "mir-module-minimal-shell-transport-plan-v0.json",
        },
        "execution_profile": {
            "input": "ASTNode::Literal(Integer(0))",
            "current_module": "Present",
            "module_transport": "MirModuleMinimalShell",
        },
        "observed_source_order": {
            "prepare_module": prepare_order,
            "finalize_module": finalize_order,
        },
        "take_sequence": [
            {
                "step": "install_module_shell",
                "source": "prepare_module",
                "operation": "self.current_module = Some(module)",
            },
            {
                "step": "take_module_shell",
                "source": "finalize_module",
                "operation": "self.current_module.take().unwrap()",
            },
        ],
        "available_capabilities": [
            "CurrentModuleTake",
        ],
        "result_contract": {
            "taken_value": "MirModuleMinimalShell",
            "source_state": "self.current_module",
            "post_take_state": "None",
        },
        "non_claims": {
            "verify_typed_values": 0,
            "current_function_take": 0,
            "full_finalize_module": 0,
            "module_metadata_publication": 0,
            "generated_hako_artifact": 0,
            "backend_route_changed": 0,
            "abi_changed": 0,
            "runtime_fallback": 0,
            "mainline_selected": 0,
        },
    }


def validate_plan(plan: dict[str, Any]) -> None:
    require(plan["kind"] == "MirBuilderCurrentModuleTakePlanV1", "wrong current module take plan kind")
    require("CurrentModuleTake" in plan["available_capabilities"], "missing CurrentModuleTake capability")
    profile = plan["execution_profile"]
    require(profile["module_transport"] == "MirModuleMinimalShell", "module transport drift")
    steps = [row["step"] for row in plan["take_sequence"]]
    require(steps == ["install_module_shell", "take_module_shell"], f"take sequence drift: {steps}")
    contract = plan["result_contract"]
    require(contract["taken_value"] == "MirModuleMinimalShell", "taken module transport drift")
    require(contract["post_take_state"] == "None", "post-take state drift")
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("missing capability", ["available_capabilities"], []),
        ("transport drift", ["execution_profile", "module_transport"], "RawModule"),
        ("verify claim drift", ["non_claims", "verify_typed_values"], 1),
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
            ("output_contract", "rust-lifecycle-mirbuilder-current-module-take-v0"),
            ("mirbuilder_current_module_take", "green"),
            ("capability", "CurrentModuleTake"),
            ("module_transport", plan["execution_profile"]["module_transport"]),
            ("verify_typed_values_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())
