#!/usr/bin/env python3
"""Extract lightweight facts for TypeContext.value_origin_newbox conversion."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "crates/hakorune_mir_builder/src/type_context.rs"


def _require(source: str, needle: str, label: str) -> None:
    if needle not in source:
        raise SystemExit(f"missing TypeContext origin-map shape: {label}")


def extract_facts(source_path: Path = SOURCE) -> dict[str, Any]:
    source = source_path.read_text()
    for needle, label in [
        ("pub value_origin_newbox: BTreeMap<ValueId, String>", "value_origin_newbox field"),
        ("pub fn new() -> Self", "new"),
        ("pub fn get_origin_box(&self, value_id: ValueId) -> Option<&str>", "get_origin_box"),
        ("self.value_origin_newbox.get(&value_id).map(|s| s.as_str())", "get_origin_box immutable atom projection"),
        ("pub fn set_origin_box(&mut self, value_id: ValueId, class_name: String)", "set_origin_box"),
        ("self.value_origin_newbox.insert(value_id, class_name)", "set_origin_box insert"),
        ("pub fn clear_origin_boxes(&mut self)", "clear_origin_boxes"),
        ("self.value_origin_newbox.clear();", "clear_origin_boxes clear"),
    ]:
        _require(source, needle, label)

    return {
        "schema_version": 0,
        "kind": "RustLifecycleFacts",
        "subject": "hakorune_mir_builder::type_context::TypeContext.value_origin_newbox",
        "source": {"path": str(source_path.relative_to(ROOT))},
        "type_facts": [
            {
                "id": "TypeContext",
                "rust_type": "TypeContext",
                "drop_fact": "TrivialMemory",
            },
            {
                "id": "ValueId",
                "rust_type": "ValueId",
                "transport": "i64",
            },
            {
                "id": "String",
                "rust_type": "String",
                "transport": "ImmutableStringAtom",
            },
        ],
        "field_facts": [
            {
                "id": "TypeContext.value_origin_newbox",
                "rust_type": "BTreeMap<ValueId, String>",
                "key_transport": "ValueIdAsI64",
                "value_transport": "ImmutableStringAtom",
                "iteration_observed": False,
                "map_identity_escapes": False,
                "drop_fact": "TrivialMemory",
            },
        ],
        "body_facts": [
            {
                "id": "TypeContext::new",
                "operation": "NewMap",
                "selected_field": "value_origin_newbox",
            },
            {
                "id": "TypeContext::get_origin_box",
                "operation": "MapGetOption",
                "selected_field": "value_origin_newbox",
                "return": "Option<&str>",
                "value_projection": "ImmutableStringAtom",
                "returned_aggregate_alias": False,
            },
            {
                "id": "TypeContext::set_origin_box",
                "operation": "MapSet",
                "selected_field": "value_origin_newbox",
            },
            {
                "id": "TypeContext::clear_origin_boxes",
                "operation": "MapClear",
                "selected_field": "value_origin_newbox",
            },
        ],
        "excluded_methods": [
            {"id": "TypeContext::get_type", "deny_reason": "ReturnedReadBorrow"},
            {"id": "TypeContext::set_type", "deny_reason": "UnsupportedTypeTransport"},
            {"id": "TypeContext::try_get_kind", "deny_reason": "OutOfSlice"},
            {"id": "TypeContext::get_kind", "deny_reason": "OutOfSlice"},
            {"id": "TypeContext::set_kind", "deny_reason": "OutOfSlice"},
            {"id": "TypeContext::take_snapshot", "deny_reason": "MultiFieldSnapshotDeferred"},
            {"id": "TypeContext::restore_snapshot", "deny_reason": "MultiFieldSnapshotDeferred"},
        ],
    }


def main() -> None:
    print(json.dumps(extract_facts(), indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
