#!/usr/bin/env python3
"""Extract target-neutral CoreContext lifecycle facts.

This is an easy-tier consultation slice only. It captures the scalar-counter
source shapes of `crates/hakorune_mir_builder/src/core_context.rs` without
opening any Hako lifecycle plan, route selection, or nightly rustc adapter
path.
"""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from context_fact_extraction import (
    extract_method_body,
    extract_method_signatures,
    immediate_return,
    receiver_fact,
    report_or_emit,
    require,
)


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "crates/hakorune_mir_builder/src/core_context.rs"
REFERENCE = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "core-context-facts-v0.json"
)

SUBJECT = "hakorune_mir_builder::core_context::CoreContext"


def _body_fact(name: str, source: str) -> dict[str, Any]:
    bodies = {
        "new": (
            "Self { value_gen: ValueIdGenerator::new(), block_gen: BasicBlockIdGenerator::new(), next_binding_id: 0, temp_slot_counter: 0, debug_join_counter: 0, }",
            {
                "operation": "ScalarCounterInit",
                "callee_spelling": "ValueIdGenerator::new + BasicBlockIdGenerator::new",
                "selected_fields": [
                    "value_gen",
                    "block_gen",
                    "next_binding_id",
                    "temp_slot_counter",
                    "debug_join_counter",
                ],
                "return_shape": "Self",
            },
        ),
        "next_value": (
            "self.value_gen.next()",
            {
                "operation": "GeneratorNext",
                "callee_spelling": "ValueIdGenerator::next",
                "selected_field": "value_gen",
                "return_shape": "ValueId",
            },
        ),
        "next_block": (
            "self.block_gen.next()",
            {
                "operation": "GeneratorNext",
                "callee_spelling": "BasicBlockIdGenerator::next",
                "selected_field": "block_gen",
                "return_shape": "BasicBlockId",
            },
        ),
        "next_binding": (
            "let id = BindingId::new(self.next_binding_id); self.next_binding_id = self.next_binding_id.saturating_add(1); debug_assert!( self.next_binding_id<u32::MAX, \"BindingId counter overflow: {}\", self.next_binding_id ); id",
            {
                "operation": "BindingIdNewAndIncrement",
                "callee_spelling": "BindingId::new + saturating_add + debug_assert",
                "selected_field": "next_binding_id",
                "return_shape": "BindingId",
            },
        ),
        "next_temp_slot": (
            "let id = self.temp_slot_counter; self.temp_slot_counter = self.temp_slot_counter.saturating_add(1); id",
            {
                "operation": "CounterNext",
                "callee_spelling": "saturating_add",
                "selected_field": "temp_slot_counter",
                "return_shape": "u32",
            },
        ),
        "next_debug_join": (
            "let id = self.debug_join_counter; self.debug_join_counter = self.debug_join_counter.saturating_add(1); id",
            {
                "operation": "CounterNext",
                "callee_spelling": "saturating_add",
                "selected_field": "debug_join_counter",
                "return_shape": "u32",
            },
        ),
        "peek_next_value": (
            "self.value_gen.peek_next()",
            {
                "operation": "GeneratorPeekNext",
                "callee_spelling": "ValueIdGenerator::peek_next",
                "selected_field": "value_gen",
                "return_shape": "ValueId",
            },
        ),
        "peek_next_block": (
            "self.block_gen.peek_next()",
            {
                "operation": "GeneratorPeekNext",
                "callee_spelling": "BasicBlockIdGenerator::peek_next",
                "selected_field": "block_gen",
                "return_shape": "BasicBlockId",
            },
        ),
    }
    expected, fact = bodies[name]
    actual = extract_method_body(source, name)
    require(actual == expected, f"unsupported CoreContext body shape: {name}")
    return {"id": f"CoreContext::{name}", **fact}


def _method_fact(name: str, signature: dict[str, Any]) -> dict[str, Any]:
    fact: dict[str, Any] = {
        "id": f"CoreContext::{name}",
        "receiver": receiver_fact(signature["params"]) if name != "new" else {"borrow_kind": "Owned", "borrow_escape": "LocalOnly", "mutation": False},
        "ownership_effect": "None",
    }
    if name in {"next_value", "next_block", "next_binding", "next_temp_slot", "next_debug_join"}:
        fact["returns"] = immediate_return()
    return fact


def extract_facts(source_path: Path) -> dict[str, Any]:
    source = source_path.read_text()
    require("pub struct CoreContext" in source, "missing CoreContext struct")
    require("impl Drop for CoreContext" not in source, "observable Drop detected")
    signatures = extract_method_signatures(source)
    expected_methods = [
        "new",
        "next_value",
        "next_block",
        "next_binding",
        "next_temp_slot",
        "next_debug_join",
        "peek_next_value",
        "peek_next_block",
    ]
    for name in expected_methods:
        require(name in signatures, f"missing method: {name}")
    for fragment in [
        "ValueIdGenerator::new()",
        "BasicBlockIdGenerator::new()",
        "BindingId::new(self.next_binding_id)",
        "self.next_binding_id = self.next_binding_id.saturating_add(1);",
        "self.temp_slot_counter = self.temp_slot_counter.saturating_add(1);",
        "self.debug_join_counter = self.debug_join_counter.saturating_add(1);",
    ]:
        require(fragment in source, f"missing source fragment: {fragment}")

    return {
        "schema_version": 0,
        "kind": "RustLifecycleFacts",
        "subject": SUBJECT,
        "source": {
            "crate": "hakorune_mir_builder",
            "module": "crate::core_context",
            "source_path": "src/core_context.rs",
        },
        "type_facts": [
            {
                "id": "CoreContext",
                "copy_kind": "NonCopyOwned",
                "escape_fact": "LocalOnly",
                "drop_fact": "TrivialMemory",
            }
        ],
        "method_facts": [_method_fact(name, signatures[name]) for name in expected_methods],
        "body_facts": [_body_fact(name, source) for name in expected_methods],
        "negative_requirements": [
            {"id": "missing_counter_init", "required_fact": "ScalarCounterInit"},
            {"id": "missing_saturating_add", "required_fact": "saturating_add"},
            {"id": "missing_bindingid_constructor", "required_fact": "BindingId::new"},
        ],
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--source", type=Path, default=SOURCE)
    parser.add_argument("--reference", type=Path, default=REFERENCE)
    parser.add_argument("--emit-json", action="store_true")
    parser.add_argument("--check-reference", action="store_true")
    args = parser.parse_args()

    facts = extract_facts(args.source)
    return report_or_emit(
        facts=facts,
        reference=args.reference,
        check_reference=args.check_reference,
        emit_json=args.emit_json,
        report=[
            ("output_contract", "rustc-semir-core-context-facts-extraction-v0"),
            ("core_context_facts_extraction_green", "1"),
            ("output_kind", "RustLifecycleFacts"),
            ("subject", "CoreContext"),
            ("target_neutral_adapter", "1"),
            ("hako_policy_owner", "0"),
            ("backend_behavior_changed", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())
