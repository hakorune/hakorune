#!/usr/bin/env python3
"""Extract easy-tier facts for CarrierInfo owned snapshot projection."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from context_fact_extraction import extract_method_signatures, report_or_emit, require


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs"
REFERENCE = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "variable-context-carrier-snapshot-facts-v0.json"
)

SUBJECT = "hakorune_mir_builder::variable_context::CarrierInfo.from_variable_map"


def extract_facts(source_path: Path) -> dict[str, Any]:
    source = source_path.read_text()
    signatures = extract_method_signatures(source)
    require("from_variable_map" in signatures, "missing CarrierInfo::from_variable_map")
    params = signatures["from_variable_map"]["params"]
    require("loop_var_name: String" in params, "loop_var_name must be owned String")
    require("variable_map: &BTreeMap<String, ValueId>" in params, "variable_map must be shared BTreeMap borrow")
    for required in [
        "variable_map.get(&loop_var_name).copied()",
        ".iter()",
        "name.clone()",
        "host_id: id",
        "join_id: None",
        "CarrierRole::LoopState",
        "CarrierInit::FromHost",
        "carriers.sort_by",
    ]:
        require(required in source, f"missing carrier snapshot source shape: {required}")

    return {
        "schema_version": 0,
        "kind": "RustLifecycleFacts",
        "subject": SUBJECT,
        "source": {
            "crate": "hakorune",
            "module": "src::mir::join_ir::lowering::carrier_info",
            "source_path": "src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs",
        },
        "base_facts": [
            "variable-context-simple-map-facts-v0.json",
            "variable-context-snapshot-restore-facts-v0.json",
        ],
        "method_fact": {
            "id": "CarrierInfo::from_variable_map",
            "input_snapshot": {
                "source": "VariableContext::snapshot",
                "ownership": "OwnedReadSnapshotProjection",
                "access": "read",
                "escapes": False,
            },
            "operation": "CarrierSnapshotFromOwnedMap",
            "loop_var_name": {"move_kind": "ConsumeArgument"},
            "map_requirements": {
                "deterministic_order_required": True,
                "value_drop_fact": "TrivialMemory",
            },
            "output": {
                "owns_carrier_names": True,
                "copies_value_ids": True,
                "value_id_copy_kind": "ImmediateValue",
                "join_id_initialized": False,
            },
        },
        "denied_methods": [
            {
                "id": "VariableContext::variable_map",
                "deny_reason": "ReturnedReadBorrow",
            }
        ],
        "denied_followups": [
            "CarrierInfo::with_explicit_carriers",
            "join_id lifecycle",
            "promoted_body_locals lifecycle",
            "trim_helper lifecycle",
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
            ("output_contract", "rustc-semir-variable-context-carrier-snapshot-facts-v0"),
            ("carrier_snapshot_facts_extraction_green", "1"),
            ("owned_read_snapshot_projection", "1"),
            ("returned_read_borrow_deny", "1"),
            ("nightly_rustc_adapter", "0"),
            ("backend_behavior_changed", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())
