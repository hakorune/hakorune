#!/usr/bin/env python3
"""Extract target-neutral VariableContext lifecycle adapter facts."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from context_fact_extraction import (
    extract_btree_map_type,
    extract_method_signatures,
    immediate_return,
    receiver_fact,
    report_or_emit,
    require,
)


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "crates/hakorune_mir_builder/src/variable_context.rs"
REFERENCE = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "variable-context-adapter-facts-v0.json"
)

SUBJECT = "hakorune_mir_builder::variable_context::VariableContext"
MAP_TYPE = "BTreeMap<String, ValueId>"


def returned_reference_method(
    *,
    name: str,
    signature: dict[str, Any],
    borrow_kind: str,
    rust_type: str,
    mutation_allowed: bool,
    ownership_effect: str,
) -> dict[str, Any]:
    fact = {
        "id": f"VariableContext::{name}",
        "receiver": receiver_fact(signature["params"]),
        "returned_reference": {
            "rust_type": rust_type,
            "owner": "VariableContext",
            "mutation_allowed": mutation_allowed,
        },
        "ownership_effect": ownership_effect,
    }
    fact["receiver"]["borrow_kind"] = borrow_kind
    fact["receiver"]["borrow_escape"] = "Returned"
    return fact


def build_simple_method_fact(name: str, signature: dict[str, Any]) -> dict[str, Any]:
    fact: dict[str, Any] = {
        "id": f"VariableContext::{name}",
        "receiver": receiver_fact(signature["params"]),
        "ownership_effect": "None",
    }
    if signature["ret"] == "Option<ValueId>":
        fact["returns"] = immediate_return()
    if name == "insert":
        require("name: String" in signature["params"], "insert must consume String name")
        require("value_id: ValueId" in signature["params"], "insert value_id")
        fact["ownership_effect"] = "ConsumeArgument"
        fact["arguments"] = [
            {"name": "name", "move_kind": "ConsumeArgument"},
            {"name": "value_id", **immediate_return()},
        ]
    return fact


def snapshot_method(signature: dict[str, Any]) -> dict[str, Any]:
    require(signature["ret"] == MAP_TYPE, "snapshot must return owned map")
    return {
        "id": "VariableContext::snapshot",
        "receiver": receiver_fact(signature["params"]),
        "ownership_effect": "CloneOwnedMap",
        "returns": {
            "rust_type": MAP_TYPE,
            "drop_class": "TrivialMemory",
            "deterministic_order_required": True,
        },
    }


def restore_method(signature: dict[str, Any]) -> dict[str, Any]:
    require(f"snapshot: {MAP_TYPE}" in signature["params"], "restore snapshot arg")
    return {
        "id": "VariableContext::restore",
        "receiver": receiver_fact(signature["params"]),
        "ownership_effect": "ReplaceOwned",
        "arguments": [
            {
                "name": "snapshot",
                "rust_type": MAP_TYPE,
                "move_kind": "ConsumeArgument",
                "deterministic_order_required": True,
            }
        ],
        "replaced_field_drop_class": "TrivialMemory",
    }


def build_methods(methods: dict[str, dict[str, Any]]) -> list[dict[str, Any]]:
    simple = ["lookup", "contains", "insert", "remove"]
    for name in simple + ["variable_map", "variable_map_mut", "snapshot", "restore"]:
        require(name in methods, f"missing method: {name}")

    rows = [build_simple_method_fact(name, methods[name]) for name in simple]
    require(methods["variable_map"]["ret"] == f"&{MAP_TYPE}", "variable_map return")
    rows.append(
        returned_reference_method(
            name="variable_map",
            signature=methods["variable_map"],
            borrow_kind="SharedRead",
            rust_type=f"&{MAP_TYPE}",
            mutation_allowed=False,
            ownership_effect="ReturnSharedReference",
        )
    )
    require(
        methods["variable_map_mut"]["ret"] == f"&mut {MAP_TYPE}",
        "variable_map_mut return",
    )
    rows.append(
        returned_reference_method(
            name="variable_map_mut",
            signature=methods["variable_map_mut"],
            borrow_kind="UniqueWrite",
            rust_type=f"&mut {MAP_TYPE}",
            mutation_allowed=True,
            ownership_effect="ReturnUniqueReference",
        )
    )
    rows.append(snapshot_method(methods["snapshot"]))
    rows.append(restore_method(methods["restore"]))
    return rows


def extract_facts(source_path: Path) -> dict[str, Any]:
    source = source_path.read_text()
    require("pub struct VariableContext" in source, "missing VariableContext struct")
    require("use std::collections::BTreeMap;" in source, "missing BTreeMap import")
    require("impl Drop for VariableContext" not in source, "observable Drop detected")
    require("CarrierInfo::from_variable_map" in source, "missing carrier consumer note")

    map_type = extract_btree_map_type(source, "variable_map", "ValueId")
    require(map_type == MAP_TYPE, "unexpected variable_map rust type")

    return {
        "schema_version": 0,
        "kind": "RustLifecycleAdapterFacts",
        "subject": SUBJECT,
        "source": {
            "crate": "hakorune_mir_builder",
            "module": "crate::variable_context",
            "source_path": "src/variable_context.rs",
        },
        "types": [
            {
                "id": "VariableContext",
                "copy_class": "NonCopyOwned",
                "escape": "LocalOnly",
                "drop_class": "TrivialMemory",
                "identity_observed": False,
                "address_observed": False,
                "layout_observed": False,
                "thread_atomic_observed": False,
            },
            {
                "id": "ValueId",
                "copy_class": "ImmediateValue",
                "drop_class": "TrivialMemory",
            },
        ],
        "fields": [
            {
                "id": "VariableContext.variable_map",
                "rust_type": map_type,
                "key_type": "String",
                "value_type": "ValueId",
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
        "methods": build_methods(extract_method_signatures(source)),
        "consumers": [
            {
                "id": "CarrierInfo::from_variable_map",
                "input_method": "VariableContext::variable_map",
                "required_access": "ReadOnly",
                "mutation": False,
                "requires_deterministic_order": True,
            },
            {
                "id": "CarrierInfo::with_explicit_carriers",
                "input_kind": "ExplicitCarrierValues",
                "missing_carrier_policy": "FailFast",
                "mutation": False,
            },
        ],
        "negative_requirements": [
            {
                "id": "returned_mutable_map_reference",
                "required_fact": "borrow_escape=Returned,borrow_kind=UniqueWrite",
            },
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
            ("output_contract", "rustc-semir-variable-context-facts-extraction-v0"),
            ("variable_context_facts_extraction_green", "1"),
            ("output_kind", "RustLifecycleAdapterFacts"),
            ("subject", "VariableContext"),
            ("target_neutral_adapter", "1"),
            ("hako_policy_owner", "0"),
            ("binding_context_behavior_changed", "0"),
            ("backend_behavior_changed", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())
