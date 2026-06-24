#!/usr/bin/env python3
"""Inventory the deferred MetadataContext consultation row."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from context_fact_extraction import require


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "crates/hakorune_mir_builder/src/metadata_context.rs"
REFERENCE = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "metadata-context-deferred-v0.json"
)

EXPECTED_METHODS = [
    "new",
    "current_span",
    "set_current_span",
    "set_source_file",
    "clear_source_file",
    "current_source_file",
    "hint_scope_enter",
    "hint_scope_leave",
    "hint_join_result",
    "push_region",
    "pop_region",
    "current_region_stack",
    "record_value_span",
    "value_span",
    "record_value_caller",
    "value_caller",
    "value_origin_callers",
]


def inventory_metadata_context_deferred(source_path: Path = SOURCE) -> dict[str, Any]:
    source = source_path.read_text()
    require("pub struct MetadataContext" in source, "missing MetadataContext struct")
    require("#[derive(Debug, Clone)]" in source, "missing MetadataContext derive")
    for method in EXPECTED_METHODS:
        require(f"pub fn {method}" in source, f"missing MetadataContext method: {method}")
    for fragment in [
        "MetadataContext<SpanT: Copy, RegionIdT: Copy>",
        "hint_sink: HintSink",
        "source_file: Option<String>",
        "current_region_stack: Vec<RegionIdT>",
        "value_origin_spans: HashMap<ValueId, SpanT>",
        "value_origin_callers: HashMap<ValueId, String>",
        "caller.file()",
    ]:
        require(fragment in source, f"missing MetadataContext fragment: {fragment}")

    return {
        "schema_version": 0,
        "kind": "MirBuilderMetadataContextDeferredInventory",
        "subject": "hakorune_mir_builder::metadata_context::MetadataContext",
        "source": {
            "crate": "hakorune_mir_builder",
            "module": "crate::metadata_context",
            "source_path": "src/metadata_context.rs",
        },
        "present": {
            "source_methods": EXPECTED_METHODS,
            "field_shapes": [
                "MetadataContext<SpanT, RegionIdT>",
                "current_span: SpanT",
                "hint_sink: HintSink",
                "source_file: Option<String>",
                "current_region_stack: Vec<RegionIdT>",
                "value_origin_spans: HashMap<ValueId, SpanT>",
                "value_origin_callers: HashMap<ValueId, String>",
            ],
        },
        "deferred_reason": [
            "generics",
            "Option<String>",
            "Vec region stack",
            "HashMap origin tables",
            "diagnostic caller provenance",
            "source-file cloning",
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

    report = inventory_metadata_context_deferred(args.source)
    if args.check_reference:
        expected = json.loads(REFERENCE.read_text())
        require(report == expected, "MetadataContext deferred inventory differs from reference fixture")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0

    print("output_contract=rust-mirbuilder-metadata-context-deferred-v0")
    print("metadata_context_deferred_recorded=1")
    print("subject=MetadataContext")
    print("route_selection=0")
    print("nightly_rustc_adapter=0")
    print("deferred=1")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
