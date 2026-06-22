#!/usr/bin/env python3
"""Extract lightweight facts for TypeContext aggregate snapshot/restore."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "crates/hakorune_mir_builder/src/type_context.rs"

SNAPSHOT_FIELDS = [
    "value_types",
    "value_kinds",
    "value_origin_newbox",
    "string_literals",
    "map_value_types",
    "map_literal_value_types",
]

FIELD_TYPES = {
    "value_types": "BTreeMap<ValueId, MirType>",
    "value_kinds": "HashMap<ValueId, MirValueKind>",
    "value_origin_newbox": "BTreeMap<ValueId, String>",
    "string_literals": "BTreeMap<ValueId, String>",
    "map_value_types": "BTreeMap<ValueId, MirType>",
    "map_literal_value_types": "BTreeMap<(ValueId, String), MirType>",
}


def _require(source: str, needle: str, label: str) -> None:
    if needle not in source:
        raise SystemExit(f"missing TypeContext snapshot shape: {label}")


def _require_count(source: str, needle: str, expected: int, label: str) -> None:
    actual = source.count(needle)
    if actual != expected:
        raise SystemExit(f"unexpected TypeContext snapshot shape count for {label}: {actual}")


def extract_facts(source_path: Path = SOURCE) -> dict[str, Any]:
    source = source_path.read_text()
    _require(source, "pub struct TypeContextSnapshot", "snapshot struct")
    _require(source, "pub fn take_snapshot(&mut self) -> TypeContextSnapshot", "take_snapshot signature")
    _require(source, "pub fn restore_snapshot(&mut self, snapshot: TypeContextSnapshot)", "restore_snapshot signature")

    for field in SNAPSHOT_FIELDS:
        rust_type = FIELD_TYPES[field]
        _require(source, f"pub {field}: {rust_type}", f"TypeContext.{field}")
        _require(source, f"{field}: {rust_type}", f"TypeContextSnapshot.{field}")
        _require(source, f"{field}: std::mem::take(&mut self.{field})", f"take {field}")
        _require(source, f"self.{field} = snapshot.{field};", f"restore {field}")
        _require_count(source, f"std::mem::take(&mut self.{field})", 1, f"take {field}")
        _require_count(source, f"self.{field} = snapshot.{field};", 1, f"restore {field}")

    return {
        "schema_version": 0,
        "kind": "RustLifecycleFacts",
        "subject": "hakorune_mir_builder::type_context::TypeContext.snapshot_restore",
        "source": {"path": str(source_path.relative_to(ROOT))},
        "type_facts": [
            {"id": "TypeContext", "rust_type": "TypeContext", "drop_fact": "TrivialMemory"},
            {"id": "TypeContextSnapshot", "rust_type": "TypeContextSnapshot", "drop_fact": "TrivialMemory"},
        ],
        "field_facts": [
            {
                "id": f"TypeContext.{field}",
                "rust_type": FIELD_TYPES[field],
                "snapshot_transport": "OpaqueOwnedMapStorage",
                "default_replacement": "NewMap",
                "entry_access_claim": "none",
                "ordering_claim": "none",
            }
            for field in SNAPSHOT_FIELDS
        ],
        "body_facts": [
            {
                "id": "TypeContext::take_snapshot",
                "operation": "AggregateTakeWithDefaults",
                "fields": SNAPSHOT_FIELDS,
                "default_replacement": "NewMap",
                "entry_access_required": False,
                "source_key_transport_required": False,
            },
            {
                "id": "TypeContext::restore_snapshot",
                "operation": "AggregateRestoreWithDefaults",
                "fields": SNAPSHOT_FIELDS,
                "snapshot_parameter": "by_value",
                "default_replacement": "NewMap",
                "entry_access_required": False,
                "source_key_transport_required": False,
            },
        ],
        "excluded_methods": [
            {"id": "TypeContext::get_type", "deny_reason": "OutOfSlice"},
            {"id": "TypeContext::set_type", "deny_reason": "OutOfSlice"},
            {"id": "TypeContext::try_get_kind", "deny_reason": "OutOfSlice"},
            {"id": "TypeContext::get_kind", "deny_reason": "OutOfSlice"},
            {"id": "TypeContext::set_kind", "deny_reason": "OutOfSlice"},
            {"id": "TypeContext::get_origin_box", "deny_reason": "OutOfSlice"},
            {"id": "TypeContext::set_origin_box", "deny_reason": "OutOfSlice"},
            {"id": "TypeContext::clear_origin_boxes", "deny_reason": "OutOfSlice"},
        ],
    }


def main() -> None:
    print(json.dumps(extract_facts(), indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
