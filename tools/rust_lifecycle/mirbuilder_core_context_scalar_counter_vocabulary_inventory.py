#!/usr/bin/env python3
"""Inventory the CoreContext scalar-counter vocabulary.

This is a consultation-only row. It summarizes the exact body-operation
vocabulary visible in `crates/hakorune_mir_builder/src/core_context.rs`
without selecting a route or opening Hako lifecycle planning.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from context_fact_extraction import require
from extract_core_context_facts import SOURCE, extract_facts


ROOT = Path(__file__).resolve().parents[2]
REFERENCE = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "core-context-scalar-counter-vocabulary-v0.json"
)

EXPECTED_METHODS = [
    "new",
    "next_value",
    "next_block",
    "next_binding",
    "next_temp_slot",
    "next_debug_join",
    "peek_next_value",
    "peek_next_block",
]

EXPECTED_OPERATIONS = [
    "ScalarCounterInit",
    "GeneratorNext",
    "GeneratorNext",
    "BindingIdNewAndIncrement",
    "CounterNext",
    "CounterNext",
    "GeneratorPeekNext",
    "GeneratorPeekNext",
]

OPERATION_VOCABULARY = [
    "ScalarCounterInit",
    "GeneratorNext",
    "BindingIdNewAndIncrement",
    "CounterNext",
    "GeneratorPeekNext",
]


def inventory_core_context_scalar_counter_vocabulary(source_path: Path = SOURCE) -> dict[str, Any]:
    facts = extract_facts(source_path)
    require(facts["subject"] == "hakorune_mir_builder::core_context::CoreContext", "unexpected subject")
    source_methods = [method["id"].split("::", 1)[1] for method in facts["method_facts"]]
    body_operations = [row["operation"] for row in facts["body_facts"]]
    require(source_methods == EXPECTED_METHODS, "unexpected CoreContext method ordering")
    require(body_operations == EXPECTED_OPERATIONS, "unexpected CoreContext body operation vocabulary")
    return {
        "schema_version": 0,
        "kind": "MirBuilderCoreContextScalarCounterVocabulary",
        "subject": facts["subject"],
        "source": facts["source"],
        "present": {
            "source_methods": source_methods,
            "body_operations": body_operations,
            "operation_vocabulary": OPERATION_VOCABULARY,
        },
        "next_design_stop": [
            "scalar counter field initialization",
            "increment / saturating_add",
            "ID constructor calls",
            "struct-return construction",
        ],
        "stop_line": [
            "do_not_select_route=1",
            "do_not_open_nightly_rustc_adapter=1",
            "do_not_claim_mirbuilder_wide_conversion=1",
            "do_not_add_runtime_fallback=1",
        ],
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--source", type=Path, default=SOURCE)
    parser.add_argument("--emit-json", action="store_true")
    parser.add_argument("--check-reference", action="store_true")
    args = parser.parse_args()

    report = inventory_core_context_scalar_counter_vocabulary(args.source)
    if args.check_reference:
        expected = json.loads(REFERENCE.read_text())
        require(report == expected, "CoreContext scalar-counter vocabulary inventory differs from reference fixture")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0

    for key, value in [
        ("output_contract", "rust-mirbuilder-core-context-scalar-counter-vocabulary-v0"),
        ("core_context_scalar_counter_vocabulary_recorded", "1"),
        ("subject", "CoreContext"),
        ("source_methods", str(len(report["present"]["source_methods"]))),
        ("body_operations", str(len(report["present"]["body_operations"]))),
        ("operation_vocabulary", ",".join(report["present"]["operation_vocabulary"])),
    ]:
        print(f"{key}={value}")
    for shape in report["next_design_stop"]:
        print(f"next_design_stop={shape}")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
