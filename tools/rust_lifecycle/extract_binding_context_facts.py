#!/usr/bin/env python3
"""Extract target-neutral BindingContext lifecycle adapter facts.

This focused pilot reads the selected Rust source slice and emits the existing
RustLifecycleAdapterFacts-v0 shape. It is intentionally narrow: no Hako plan,
no .hako source, no backend behavior, and no VariableContext facts.
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "crates/hakorune_mir_builder/src/binding_context.rs"
REFERENCE = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "binding-context-adapter-facts-v0.json"
)

SUBJECT = "hakorune_mir_builder::binding_context::BindingContext"


class ExtractionError(AssertionError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ExtractionError(message)


def normalized_rust_type(text: str) -> str:
    return re.sub(r"\s+", " ", text.strip())


def extract_binding_map_type(source: str) -> str:
    match = re.search(
        r"binding_map\s*:\s*(?P<ty>BTreeMap\s*<\s*String\s*,\s*BindingId\s*>)",
        source,
    )
    require(match is not None, "missing BindingContext.binding_map BTreeMap field")
    return normalized_rust_type(match.group("ty")).replace(" <", "<").replace(" >", ">")


def extract_method_signatures(source: str) -> dict[str, dict[str, Any]]:
    pattern = re.compile(
        r"pub\s+fn\s+(?P<name>\w+)\s*"
        r"\((?P<params>[^)]*)\)\s*"
        r"(?:->\s*(?P<ret>[^{]+))?\{",
        re.MULTILINE,
    )
    methods: dict[str, dict[str, Any]] = {}
    for match in pattern.finditer(source):
        name = match.group("name")
        if name == "new":
            continue
        params = normalized_rust_type(match.group("params"))
        ret = normalized_rust_type(match.group("ret") or "")
        methods[name] = {"params": params, "ret": ret}
    return methods


def receiver_fact(params: str) -> dict[str, Any]:
    if params.startswith("&mut self"):
        return {
            "borrow_kind": "UniqueWrite",
            "borrow_escape": "CallOnly",
            "mutation": True,
        }
    if params.startswith("&self"):
        return {
            "borrow_kind": "SharedRead",
            "borrow_escape": "CallOnly",
            "mutation": False,
        }
    raise ExtractionError(f"unsupported receiver params: {params}")


def immediate_return() -> dict[str, str]:
    return {"copy_class": "ImmediateValue", "drop_class": "TrivialMemory"}


def build_method_fact(name: str, signature: dict[str, Any]) -> dict[str, Any]:
    fact: dict[str, Any] = {
        "id": f"BindingContext::{name}",
        "receiver": receiver_fact(signature["params"]),
        "ownership_effect": "None",
    }
    ret = signature["ret"]
    if ret == "Option<BindingId>":
        fact["returns"] = immediate_return()
    if name == "insert":
        fact["ownership_effect"] = "ConsumeArgument"
        require("name: String" in signature["params"], "insert must consume String name")
        require("binding_id: BindingId" in signature["params"], "insert binding_id")
        fact["arguments"] = [
            {"name": "name", "move_kind": "ConsumeArgument"},
            {"name": "binding_id", **immediate_return()},
        ]
    return fact


def extract_facts(source_path: Path) -> dict[str, Any]:
    source = source_path.read_text()
    require("pub struct BindingContext" in source, "missing BindingContext struct")
    require("use std::collections::BTreeMap;" in source, "missing BTreeMap import")
    require("impl Drop for BindingContext" not in source, "observable Drop detected")

    binding_map_type = extract_binding_map_type(source)
    methods = extract_method_signatures(source)
    expected_methods = [
        "is_empty",
        "len",
        "contains",
        "lookup",
        "insert",
        "remove",
        "clear_for_function_entry",
    ]
    for method in expected_methods:
        require(method in methods, f"missing method: {method}")

    return {
        "schema_version": 0,
        "kind": "RustLifecycleAdapterFacts",
        "subject": SUBJECT,
        "source": {
            "crate": "hakorune_mir_builder",
            "module": "crate::binding_context",
            "source_path": "src/binding_context.rs",
        },
        "types": [
            {
                "id": "BindingContext",
                "copy_class": "NonCopyOwned",
                "escape": "LocalOnly",
                "drop_class": "TrivialMemory",
                "identity_observed": False,
                "address_observed": False,
                "layout_observed": False,
                "thread_atomic_observed": False,
            },
            {
                "id": "BindingId",
                "copy_class": "ImmediateValue",
                "drop_class": "TrivialMemory",
            },
        ],
        "fields": [
            {
                "id": "BindingContext.binding_map",
                "rust_type": binding_map_type,
                "key_type": "String",
                "value_type": "BindingId",
                "copy_class": "NonCopyOwned",
                "escape": "LocalOnly",
                "drop_class": "TrivialMemory",
                "deterministic_order_required": True,
                "identity_observed": False,
                "address_observed": False,
                "layout_observed": False,
                "thread_atomic_observed": False,
            }
        ],
        "methods": [build_method_fact(name, methods[name]) for name in expected_methods],
        "negative_requirements": [
            {"id": "borrow_escape_unknown", "required_fact": "borrow_escape"},
            {
                "id": "missing_deterministic_order",
                "required_fact": "deterministic_order_required",
            },
            {
                "id": "missing_trivial_memory_drop",
                "required_fact": "drop_class=TrivialMemory",
            },
        ],
        "target_neutral": {
            "hako_policy_owner": False,
            "hako_plan_kind_spelling_allowed": False,
            "rendering_instruction_allowed": False,
            "rustc_toolchain_invoked": False,
        },
    }


def assert_no_hako_policy_spelling(facts: dict[str, Any]) -> None:
    text = json.dumps(facts, sort_keys=True)
    for forbidden in [
        "OrderedMapBox",
        "BorrowView",
        "TransferOwned",
        "LocalBox",
        "HakoLifecyclePlan",
        ".hako source",
        "backend lowering",
    ]:
        require(forbidden not in text, f"Hako policy spelling leaked: {forbidden}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--source", type=Path, default=SOURCE)
    parser.add_argument("--reference", type=Path, default=REFERENCE)
    parser.add_argument("--emit-json", action="store_true")
    parser.add_argument("--check-reference", action="store_true")
    args = parser.parse_args()

    facts = extract_facts(args.source)
    assert_no_hako_policy_spelling(facts)

    if args.check_reference:
        reference = json.loads(args.reference.read_text())
        require(facts == reference, "extracted facts differ from reference fixture")

    if args.emit_json:
        print(json.dumps(facts, indent=2))
    else:
        print("output_contract=rustc-semir-binding-context-facts-extraction-v0")
        print("binding_context_facts_extraction_green=1")
        print("output_kind=RustLifecycleAdapterFacts")
        print("subject=BindingContext")
        print("target_neutral_adapter=1")
        print("hako_policy_owner=0")
        print("variable_context_facts_generated=0")
        print("backend_behavior_changed=0")
        print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
