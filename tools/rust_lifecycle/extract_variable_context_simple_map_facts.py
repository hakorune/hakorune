#!/usr/bin/env python3
"""Extract lightweight RustLifecycleFacts for VariableContext simple-map.

This stays in the easy tier: it reads the checked Rust source and emits the
compact `RustLifecycleFacts` shape already used by the simple-map pilot.
"""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from context_fact_extraction import (
    extract_btree_map_type,
    extract_method_signatures,
    report_or_emit,
    require,
)


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "crates/hakorune_mir_builder/src/variable_context.rs"
REFERENCE = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "variable-context-simple-map-facts-v0.json"
)

SUBJECT = "hakorune_mir_builder::variable_context::VariableContext.simple_map"
MAP_TYPE = "BTreeMap<String, ValueId>"
METHODS = [
    "lookup",
    "contains",
    "len",
    "is_empty",
    "insert",
    "remove",
]
EXCLUDED_METHODS = [
    "variable_map",
    "variable_map_mut",
    "snapshot",
    "restore",
]
EXCLUDED_CONSUMERS = [
    "CarrierInfo::from_variable_map",
    "PHI planner integration",
    "JoinIR carrier extraction",
]


def receiver_borrow(params: str) -> dict[str, Any]:
    if params.startswith("&self"):
        return {"kind": "SharedRead", "scope": "CallOnly", "escapes": False}
    if params.startswith("&mut self"):
        return {"kind": "UniqueWrite", "scope": "CallOnly", "escapes": False}
    raise AssertionError(f"unsupported receiver params: {params}")


def build_method_fact(name: str, signature: dict[str, Any]) -> dict[str, Any]:
    fact: dict[str, Any] = {
        "id": f"VariableContext::{name}",
        "receiver_borrow": receiver_borrow(signature["params"]),
    }
    if name == "lookup":
        require(signature["ret"] == "Option<ValueId>", "lookup must return Option<ValueId>")
        fact["returns"] = {"copy_kind": "ImmediateValue", "drop_fact": "TrivialMemory"}
    elif name == "insert":
        require("name: String" in signature["params"], "insert must consume String name")
        require("value_id: ValueId" in signature["params"], "insert value_id")
        fact["argument_moves"] = [
            {"name": "name", "move_kind": "ConsumeArgument"},
            {"name": "value_id", "copy_kind": "ImmediateValue"},
        ]
        fact["overwrite_policy"] = "allowed_when_previous_value_drop_is_TrivialMemory"
    elif name == "remove":
        require(signature["ret"] == "Option<ValueId>", "remove must return Option<ValueId>")
        fact["returns"] = {"copy_kind": "ImmediateValue", "drop_fact": "TrivialMemory"}
    return fact


def extract_facts(source_path: Path) -> dict[str, Any]:
    source = source_path.read_text()
    require("pub struct VariableContext" in source, "missing VariableContext struct")
    require("use std::collections::BTreeMap;" in source, "missing BTreeMap import")
    require("impl Drop for VariableContext" not in source, "observable Drop detected")
    require("CarrierInfo::from_variable_map" in source, "missing carrier consumer note")

    map_type = extract_btree_map_type(source, "variable_map", "ValueId")
    require(map_type == MAP_TYPE, "unexpected variable_map rust type")

    signatures = extract_method_signatures(source)
    for name in METHODS + EXCLUDED_METHODS:
        require(name in signatures, f"missing method: {name}")

    return {
        "schema_version": 0,
        "kind": "RustLifecycleFacts",
        "subject": SUBJECT,
        "source": {
            "crate": "hakorune_mir_builder",
            "module": "crate::variable_context",
            "source_path": "src/variable_context.rs",
        },
        "type_facts": [
            {
                "id": "VariableContext",
                "copy_kind": "NonCopyOwned",
                "escape_fact": "LocalOnly",
                "drop_fact": "TrivialMemory",
            },
            {
                "id": "ValueId",
                "copy_kind": "ImmediateValue",
                "escape_fact": "LocalOnly",
                "drop_fact": "TrivialMemory",
            },
        ],
        "field_facts": [
            {
                "id": "VariableContext.variable_map",
                "rust_type": map_type,
                "copy_kind": "NonCopyOwned",
                "escape_fact": "LocalOnly",
                "drop_fact": "TrivialMemory",
                "deterministic_order_required": True,
            }
        ],
        "method_facts": [build_method_fact(name, signatures[name]) for name in METHODS],
        "excluded_methods": [
            {
                "id": "VariableContext::variable_map",
                "reason": "returned shared map borrow is outside simple-map pilot",
            },
            {
                "id": "VariableContext::variable_map_mut",
                "reason": "returned mutable map borrow is outside simple-map pilot",
            },
            {
                "id": "VariableContext::snapshot",
                "reason": "owned map clone policy is outside simple-map pilot",
            },
            {
                "id": "VariableContext::restore",
                "reason": "ReplaceOwned map transfer is outside simple-map pilot",
            },
        ],
        "excluded_consumers": EXCLUDED_CONSUMERS,
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
            ("output_contract", "rustc-semir-variable-context-simple-map-facts-v0"),
            ("variable_context_simple_map_facts_extraction_green", "1"),
            ("output_kind", "RustLifecycleFacts"),
            ("subject", "VariableContext.simple_map"),
            ("lightweight_signature_facts", "1"),
            ("nightly_rustc_adapter", "0"),
            ("backend_behavior_changed", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())
