#!/usr/bin/env python3
"""Inventory the bounded TypeContext map slice."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from context_fact_extraction import require


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "crates/hakorune_mir_builder/src/type_context.rs"
REFERENCE = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "type-context-bounded-map-slice-v0.json"
)

EXPECTED_METHODS = [
    "new",
    "get_type",
    "set_type",
    "get_kind",
    "set_kind",
    "get_origin_box",
    "set_origin_box",
    "clear_origin_boxes",
    "take_snapshot",
    "restore_snapshot",
]


def _require_fragment(source: str, fragment: str, label: str) -> None:
    require(fragment in source, f"missing TypeContext {label}: {fragment}")


def inventory_type_context_bounded_map_slice(source_path: Path = SOURCE) -> dict[str, Any]:
    source = source_path.read_text()
    require("pub struct TypeContext" in source, "missing TypeContext struct")
    require("pub struct TypeContextSnapshot" in source, "missing TypeContextSnapshot struct")
    for method in EXPECTED_METHODS:
        require(f"pub fn {method}" in source, f"missing TypeContext method: {method}")
    for fragment, label in [
        ("value_types: BTreeMap<ValueId, MirType>", "value_types field"),
        ("value_kinds: HashMap<ValueId, MirValueKind>", "value_kinds field"),
        ("value_origin_newbox: BTreeMap<ValueId, String>", "value_origin_newbox field"),
        ("string_literals: BTreeMap<ValueId, String>", "string_literals field"),
        ("map_value_types: BTreeMap<ValueId, MirType>", "map_value_types field"),
        ("map_literal_value_types: BTreeMap<(ValueId, String), MirType>", "map_literal_value_types field"),
        ("std::mem::take(&mut self.value_types)", "snapshot take_type"),
        ("self.map_literal_value_types = snapshot.map_literal_value_types;", "restore map_literal_value_types"),
    ]:
        _require_fragment(source, fragment, label)
    return {
        "schema_version": 0,
        "kind": "MirBuilderTypeContextBoundedMapSlice",
        "subject": "hakorune_mir_builder::type_context::TypeContext",
        "source": {
            "crate": "hakorune_mir_builder",
            "module": "crate::type_context",
            "source_path": "src/type_context.rs",
        },
        "present": {
            "source_methods": EXPECTED_METHODS,
            "field_shapes": [
                "value_types: BTreeMap<ValueId, MirType>",
                "value_kinds: HashMap<ValueId, MirValueKind>",
                "value_origin_newbox: BTreeMap<ValueId, String>",
                "string_literals: BTreeMap<ValueId, String>",
                "map_value_types: BTreeMap<ValueId, MirType>",
                "map_literal_value_types: BTreeMap<(ValueId, String), MirType>",
            ],
            "snapshot_shapes": [
                "TypeContextSnapshot stores all mirrored fields",
                "take_snapshot uses std::mem::take",
                "restore_snapshot reassigns owned fields",
            ],
        },
        "next_design_stop": [
            "non-String keys",
            "HashMap rather than BTreeMap",
            "Option/default behavior",
            "closure-shaped source paths",
            "snapshot struct behavior",
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

    report = inventory_type_context_bounded_map_slice(args.source)
    if args.check_reference:
        expected = json.loads(REFERENCE.read_text())
        require(report == expected, "TypeContext bounded map slice inventory differs from reference fixture")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0

    print("output_contract=rust-mirbuilder-type-context-bounded-map-slice-v0")
    print("type_context_bounded_map_slice_recorded=1")
    print("subject=TypeContext")
    print("route_selection=0")
    print("nightly_rustc_adapter=0")
    print("snapshot_struct=1")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
