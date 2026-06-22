#!/usr/bin/env python3
"""Extract lightweight RustLifecycleFacts for BoxCompilationContext.

This stays in the easy tier: it reads the checked Rust source and emits the
compact `RustLifecycleFacts` shape for the bounded constructor + is_empty
pilot. `size_info` is intentionally excluded from the pilot.
"""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from context_fact_extraction import (
    extract_method_body,
    extract_method_signatures,
    report_or_emit,
    require,
)


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "crates/hakorune_mir_builder/src/context.rs"
REFERENCE = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "box-compilation-context-facts-v0.json"
)

SUBJECT = "hakorune_mir_builder::context::BoxCompilationContext"
FIELDS = [
    "variable_map",
    "value_origin_newbox",
    "value_types",
]


def build_body_fact(name: str, source: str) -> dict[str, Any]:
    bodies = {
        "new": (
            "Self::default()",
            {
                "operation": "DefaultConstruct",
                "callee_spelling": "Self::default",
                "selected_fields": FIELDS,
                "return_shape": "Self",
            },
        ),
        "is_empty": (
            "self.variable_map.is_empty() && self.value_origin_newbox.is_empty() && self.value_types.is_empty()",
            {
                "operation": "CompositeMapIsEmpty",
                "callee_spelling": "BTreeMap::is_empty && BTreeMap::is_empty && BTreeMap::is_empty",
                "selected_fields": FIELDS,
                "return_shape": "bool",
            },
        ),
    }
    expected, fact = bodies[name]
    actual = extract_method_body(source, name)
    require(actual == expected, f"unsupported BoxCompilationContext body shape: {name}")
    return {"id": f"BoxCompilationContext::{name}", **fact}


def extract_facts(source_path: Path) -> dict[str, Any]:
    source = source_path.read_text()
    require("pub struct BoxCompilationContext" in source, "missing BoxCompilationContext struct")
    require("use std::collections::BTreeMap;" in source, "missing BTreeMap import")
    require("impl Drop for BoxCompilationContext" not in source, "observable Drop detected")

    signatures = extract_method_signatures(source)
    for name in ["new", "is_empty", "size_info"]:
        require(name in signatures, f"missing method: {name}")
    require(signatures["new"]["ret"] == "Self", "new must return Self")
    require(signatures["is_empty"]["params"].startswith("&self"), "is_empty must be a shared receiver")
    require(signatures["is_empty"]["ret"] == "bool", "is_empty must return bool")
    require(signatures["size_info"]["ret"] == "(usize, usize, usize)", "size_info signature changed")

    for field_name, rust_type in [
        ("variable_map", "BTreeMap<String, ValueId>"),
        ("value_origin_newbox", "BTreeMap<ValueId, String>"),
        ("value_types", "BTreeMap<ValueId, MirType>"),
    ]:
        require(f"{field_name}:" in source, f"missing field: {field_name}")
        compact = rust_type.replace(" ", "")
        require(compact in source.replace(" ", ""), f"missing value type for {field_name}")

    return {
        "schema_version": 0,
        "kind": "RustLifecycleFacts",
        "subject": SUBJECT,
        "source": {
            "crate": "hakorune_mir_builder",
            "module": "crate::context",
            "source_path": "src/context.rs",
        },
        "type_facts": [
            {
                "id": "BoxCompilationContext",
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
            {
                "id": "MirType",
                "copy_kind": "NonCopyOwned",
                "escape_fact": "LocalOnly",
                "drop_fact": "TrivialMemory",
            },
        ],
        "field_facts": [
            {
                "id": "BoxCompilationContext.variable_map",
                "rust_type": "BTreeMap<String, ValueId>",
                "copy_kind": "NonCopyOwned",
                "escape_fact": "LocalOnly",
                "drop_fact": "TrivialMemory",
                "deterministic_order_required": True,
            },
            {
                "id": "BoxCompilationContext.value_origin_newbox",
                "rust_type": "BTreeMap<ValueId, String>",
                "copy_kind": "NonCopyOwned",
                "escape_fact": "LocalOnly",
                "drop_fact": "TrivialMemory",
                "deterministic_order_required": True,
            },
            {
                "id": "BoxCompilationContext.value_types",
                "rust_type": "BTreeMap<ValueId, MirType>",
                "copy_kind": "NonCopyOwned",
                "escape_fact": "LocalOnly",
                "drop_fact": "TrivialMemory",
                "deterministic_order_required": True,
            },
        ],
        "method_facts": [
            {
                "id": "BoxCompilationContext::new",
                "ownership_effect": "None",
                "returns": {
                    "copy_kind": "NonCopyOwned",
                    "drop_fact": "TrivialMemory",
                },
            },
            {
                "id": "BoxCompilationContext::is_empty",
                "receiver_borrow": {
                    "kind": "SharedRead",
                    "scope": "CallOnly",
                    "escapes": False,
                },
                "returns": {
                    "copy_kind": "ImmediateValue",
                    "drop_fact": "TrivialMemory",
                },
            },
        ],
        "body_facts": [build_body_fact(name, source) for name in ["new", "is_empty"]],
        "excluded_methods": [
            {
                "id": "BoxCompilationContext::size_info",
                "reason": "tuple shape is outside the bounded constructor/is_empty pilot",
            }
        ],
        "negative_requirements": [
            {
                "id": "missing_deterministic_order",
                "required_fact": "deterministic_order_required",
            },
            {
                "id": "missing_trivial_memory_drop",
                "required_fact": "drop_fact=TrivialMemory",
            },
        ],
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
            ("output_contract", "rustc-semir-box-compilation-context-facts-extraction-v0"),
            ("box_compilation_context_facts_extraction_green", "1"),
            ("output_kind", "RustLifecycleFacts"),
            ("subject", "BoxCompilationContext"),
            ("lightweight_body_facts", "1"),
            ("nightly_rustc_adapter", "0"),
            ("backend_behavior_changed", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())
