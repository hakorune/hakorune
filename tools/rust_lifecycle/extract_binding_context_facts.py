#!/usr/bin/env python3
"""Extract target-neutral BindingContext lifecycle adapter facts.

This focused pilot reads the selected Rust source slice and emits the existing
RustLifecycleAdapterFacts-v0 shape. It is intentionally narrow: no Hako plan,
no .hako source, no backend behavior, and no VariableContext facts.
"""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from context_fact_extraction import (
    extract_btree_map_type,
    extract_method_body,
    extract_method_signatures,
    immediate_return,
    receiver_fact,
    report_or_emit,
    require,
)


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "crates/hakorune_mir_builder/src/binding_context.rs"
REFERENCE = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "binding-context-adapter-facts-v0.json"
)

SUBJECT = "hakorune_mir_builder::binding_context::BindingContext"


def build_body_fact(name: str, source: str) -> dict[str, Any]:
    bodies = {
        "new": (
            "Self { binding_map: BTreeMap::new(), }",
            {
                "operation": "NewOrderedMap",
                "callee_spelling": "BTreeMap::new",
                "selected_field": "binding_map",
                "return_shape": "Self",
            },
        ),
        "is_empty": (
            "self.binding_map.is_empty()",
            {
                "operation": "MapIsEmpty",
                "callee_spelling": "BTreeMap::is_empty",
                "selected_field": "binding_map",
                "return_shape": "bool",
            },
        ),
        "len": (
            "self.binding_map.len()",
            {
                "operation": "MapLength",
                "callee_spelling": "BTreeMap::len",
                "selected_field": "binding_map",
                "return_shape": "usize",
            },
        ),
        "contains": (
            "self.binding_map.contains_key(name)",
            {
                "operation": "MapHas",
                "callee_spelling": "BTreeMap::contains_key",
                "selected_field": "binding_map",
                "argument_shape": "borrowed_name",
                "return_shape": "bool",
            },
        ),
        "lookup": (
            "self.binding_map.get(name).copied()",
            {
                "operation": "MapGet",
                "callee_spelling": "BTreeMap::get + Option::copied",
                "selected_field": "binding_map",
                "argument_shape": "borrowed_name",
                "return_shape": "Option<BindingId>",
            },
        ),
        "insert": (
            "self.binding_map.insert(name, binding_id);",
            {
                "operation": "MapSet",
                "callee_spelling": "BTreeMap::insert",
                "selected_field": "binding_map",
                "argument_shape": "owned_name_and_binding_id",
                "return_shape": "()",
            },
        ),
        "remove": (
            "self.binding_map.remove(name)",
            {
                "operation": "MapRemove",
                "callee_spelling": "BTreeMap::remove",
                "selected_field": "binding_map",
                "argument_shape": "borrowed_name",
                "return_shape": "Option<BindingId>",
            },
        ),
        "clear_for_function_entry": (
            "self.binding_map.clear();",
            {
                "operation": "MapClear",
                "callee_spelling": "BTreeMap::clear",
                "selected_field": "binding_map",
                "return_shape": "()",
            },
        ),
    }
    expected, fact = bodies[name]
    actual = extract_method_body(source, name)
    require(actual == expected, f"unsupported binding body shape: {name}")
    return {"id": f"BindingContext::{name}", **fact}


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

    binding_map_type = extract_btree_map_type(source, "binding_map", "BindingId")
    methods = {
        name: sig
        for name, sig in extract_method_signatures(source).items()
        if name != "new"
    }
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
        "body_facts": [build_body_fact(name, source) for name in ["new", *expected_methods]],
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

def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--source", type=Path, default=SOURCE)
    parser.add_argument("--reference", type=Path, default=REFERENCE)
    parser.add_argument("--emit-json", action="store_true")
    parser.add_argument("--check-reference", action="store_true")
    args = parser.parse_args()

    return report_or_emit(
        facts=extract_facts(args.source),
        reference=args.reference,
        check_reference=args.check_reference,
        emit_json=args.emit_json,
        report=[
            ("output_contract", "rustc-semir-binding-context-facts-extraction-v0"),
            ("binding_context_facts_extraction_green", "1"),
            ("output_kind", "RustLifecycleAdapterFacts"),
            ("subject", "BindingContext"),
            ("target_neutral_adapter", "1"),
            ("hako_policy_owner", "0"),
            ("variable_context_facts_generated", "0"),
            ("backend_behavior_changed", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())
