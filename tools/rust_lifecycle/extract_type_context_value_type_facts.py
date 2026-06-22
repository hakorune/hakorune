#!/usr/bin/env python3
"""Extract lightweight facts for TypeContext.value_types conversion."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "crates/hakorune_mir_builder/src/type_context.rs"
TYPES_SOURCE = ROOT / "crates/hakorune_mir_core/src/types.rs"


MIR_TYPE_VARIANTS: list[dict[str, Any]] = [
    {"name": "Integer"},
    {"name": "Float"},
    {"name": "Bool"},
    {"name": "String"},
    {"name": "Box", "payload": "String"},
    {"name": "Array", "payload": "MirType"},
    {"name": "Future", "payload": "MirType"},
    {"name": "WeakRef"},
    {"name": "Void"},
    {"name": "Unknown"},
]


def _require(source: str, needle: str, label: str) -> None:
    if needle not in source:
        raise SystemExit(f"missing TypeContext value-type shape: {label}")


def _require_mir_type(types_source: str) -> None:
    for needle, label in [
        ("pub enum MirType", "MirType enum"),
        ("Integer", "Integer variant"),
        ("Float", "Float variant"),
        ("Bool", "Bool variant"),
        ("String,", "String variant"),
        ("Box(String)", "Box(String) variant"),
        ("Array(Box<MirType>)", "Array recursive variant"),
        ("Future(Box<MirType>)", "Future recursive variant"),
        ("WeakRef", "WeakRef variant"),
        ("Void", "Void variant"),
        ("Unknown", "Unknown variant"),
    ]:
        _require(types_source, needle, label)


def extract_facts(source_path: Path = SOURCE, types_path: Path = TYPES_SOURCE) -> dict[str, Any]:
    source = source_path.read_text()
    types_source = types_path.read_text()
    _require_mir_type(types_source)
    for needle, label in [
        ("pub value_types: BTreeMap<ValueId, MirType>", "value_types field"),
        ("pub fn new() -> Self", "new"),
        ("pub fn get_type(&self, value_id: ValueId) -> Option<&MirType>", "get_type"),
        ("self.value_types.get(&value_id)", "get_type lookup"),
        ("pub fn set_type(&mut self, value_id: ValueId, ty: MirType)", "set_type"),
        ("self.value_types.insert(value_id, ty)", "set_type insert"),
    ]:
        _require(source, needle, label)

    return {
        "schema_version": 0,
        "kind": "RustLifecycleFacts",
        "subject": "hakorune_mir_builder::type_context::TypeContext.value_types",
        "source": {
            "path": str(source_path.relative_to(ROOT)),
            "type_source_path": str(types_path.relative_to(ROOT)),
        },
        "type_facts": [
            {"id": "TypeContext", "rust_type": "TypeContext", "drop_fact": "TrivialMemory"},
            {"id": "ValueId", "rust_type": "ValueId", "transport": "i64"},
            {
                "id": "MirType",
                "rust_type": "MirType",
                "transport": "OwnedRecursiveEnum",
                "recursive": True,
                "variants": MIR_TYPE_VARIANTS,
            },
        ],
        "field_facts": [
            {
                "id": "TypeContext.value_types",
                "rust_type": "BTreeMap<ValueId, MirType>",
                "key_transport": "ValueIdAsI64",
                "value_transport": "OwnedRecursiveEnum",
                "iteration_observed": False,
                "map_identity_escapes": False,
                "drop_fact": "TrivialMemory",
            },
        ],
        "body_facts": [
            {"id": "TypeContext::new", "operation": "NewMap", "selected_field": "value_types"},
            {
                "id": "TypeContext::get_type",
                "operation": "MapGetOption",
                "selected_field": "value_types",
                "return": "Option<&MirType>",
                "return_transport": "Option<MirType>",
                "returned_borrow_projected_to_owned": True,
                "returned_aggregate_alias": False,
            },
            {"id": "TypeContext::set_type", "operation": "MapSet", "selected_field": "value_types"},
        ],
        "excluded_methods": [
            {"id": "TypeContext::try_get_kind", "deny_reason": "OutOfSlice"},
            {"id": "TypeContext::get_kind", "deny_reason": "OutOfSlice"},
            {"id": "TypeContext::set_kind", "deny_reason": "OutOfSlice"},
            {"id": "TypeContext::get_origin_box", "deny_reason": "OutOfSlice"},
            {"id": "TypeContext::set_origin_box", "deny_reason": "OutOfSlice"},
            {"id": "TypeContext::clear_origin_boxes", "deny_reason": "OutOfSlice"},
            {"id": "TypeContext::take_snapshot", "deny_reason": "MultiFieldSnapshotDeferred"},
            {"id": "TypeContext::restore_snapshot", "deny_reason": "MultiFieldSnapshotDeferred"},
        ],
    }


def main() -> None:
    print(json.dumps(extract_facts(), indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
