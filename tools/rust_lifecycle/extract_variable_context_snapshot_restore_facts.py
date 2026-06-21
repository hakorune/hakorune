#!/usr/bin/env python3
"""Extract lightweight facts for VariableContext snapshot/restore."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from context_fact_extraction import extract_method_body, extract_method_signatures, report_or_emit, require


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "crates/hakorune_mir_builder/src/variable_context.rs"
REFERENCE = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "variable-context-snapshot-restore-facts-v0.json"
)

SUBJECT = "hakorune_mir_builder::variable_context::VariableContext.snapshot_restore"
MAP_TYPE = "BTreeMap<String, ValueId>"


def build_body_fact(name: str, source: str) -> dict[str, Any]:
    bodies = {
        "snapshot": (
            "self.variable_map.clone()",
            {
                "operation": "CloneOwnedMap",
                "callee_spelling": "BTreeMap::clone",
                "selected_field": "variable_map",
                "return_shape": MAP_TYPE,
            },
        ),
        "restore": (
            "self.variable_map = snapshot;",
            {
                "operation": "ReplaceOwnedMap",
                "callee_spelling": "assignment",
                "selected_field": "variable_map",
                "argument_shape": "owned_snapshot",
                "return_shape": "()",
            },
        ),
    }
    expected, fact = bodies[name]
    actual = extract_method_body(source, name)
    require(actual == expected, f"unsupported snapshot/restore body shape: {name}")
    return {"id": f"VariableContext::{name}", **fact}


def extract_facts(source_path: Path) -> dict[str, Any]:
    source = source_path.read_text()
    signatures = extract_method_signatures(source)
    for name in ["snapshot", "restore"]:
        require(name in signatures, f"missing method: {name}")
    require(signatures["snapshot"]["ret"] == MAP_TYPE, "snapshot must return owned map")
    require("snapshot: BTreeMap<String, ValueId>" in signatures["restore"]["params"], "restore must consume snapshot")

    return {
        "schema_version": 0,
        "kind": "RustLifecycleFacts",
        "subject": SUBJECT,
        "source": {
            "crate": "hakorune_mir_builder",
            "module": "crate::variable_context",
            "source_path": "src/variable_context.rs",
        },
        "base_facts": "variable-context-simple-map-facts-v0.json",
        "method_facts": [
            {
                "id": "VariableContext::snapshot",
                "receiver_borrow": {"kind": "SharedRead", "scope": "CallOnly", "escapes": False},
                "operation": "CloneOwnedMap",
                "returns": {
                    "rust_type": MAP_TYPE,
                    "copy_kind": "NonCopyOwned",
                    "drop_fact": "TrivialMemory",
                    "deterministic_order_required": True,
                },
            },
            {
                "id": "VariableContext::restore",
                "receiver_borrow": {"kind": "UniqueWrite", "scope": "CallOnly", "escapes": False},
                "operation": "ReplaceOwned",
                "argument_moves": [
                    {
                        "name": "snapshot",
                        "rust_type": MAP_TYPE,
                        "move_kind": "ConsumeArgument",
                        "drop_fact": "TrivialMemory",
                        "deterministic_order_required": True,
                    }
                ],
                "old_value_cleanup": {
                    "field": "VariableContext.variable_map",
                    "required_fact": "VariableContext.variable_map.drop_fact=TrivialMemory",
                },
            },
        ],
        "body_facts": [build_body_fact(name, source) for name in ["snapshot", "restore"]],
        "denied_methods": [
            {
                "id": "VariableContext::variable_map_mut",
                "deny_reason": "ReturnedMutableBorrow",
            }
        ],
        "excluded_consumers": [
            "CarrierInfo::from_variable_map",
            "CarrierInfo::with_explicit_carriers",
            "PHI planner integration",
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
            ("output_contract", "rustc-semir-variable-context-snapshot-restore-facts-v0"),
            ("snapshot_restore_facts_extraction_green", "1"),
            ("output_kind", "RustLifecycleFacts"),
            ("subject", "VariableContext.snapshot_restore"),
            ("lightweight_body_facts", "1"),
            ("nightly_rustc_adapter", "0"),
            ("backend_behavior_changed", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())
