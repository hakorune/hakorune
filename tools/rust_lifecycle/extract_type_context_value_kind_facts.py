#!/usr/bin/env python3
"""Extract lightweight facts for TypeContext.value_kinds direct conversion."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "crates/hakorune_mir_builder/src/type_context.rs"


def _require(source: str, needle: str, label: str) -> None:
    if needle not in source:
        raise SystemExit(f"missing TypeContext value-kind shape: {label}")


def extract_facts(source_path: Path = SOURCE) -> dict[str, Any]:
    source = source_path.read_text()
    for needle, label in [
        ("pub value_kinds: HashMap<ValueId, MirValueKind>", "value_kinds field"),
        ("pub fn new() -> Self", "new"),
        ("pub fn try_get_kind(&self, value_id: ValueId) -> Option<MirValueKind>", "try_get_kind"),
        ("self.value_kinds.get(&value_id).copied()", "try_get_kind copied lookup"),
        ("pub fn get_kind(&self, value_id: ValueId) -> MirValueKind", "get_kind"),
        (".unwrap_or(MirValueKind::Temporary)", "get_kind default"),
        ("pub fn set_kind(&mut self, value_id: ValueId, kind: MirValueKind)", "set_kind"),
        ("self.value_kinds.insert(value_id, kind)", "set_kind insert"),
    ]:
        _require(source, needle, label)

    return {
        "schema_version": 0,
        "kind": "RustLifecycleFacts",
        "subject": "hakorune_mir_builder::type_context::TypeContext.value_kinds",
        "source": {"path": str(source_path.relative_to(ROOT))},
        "type_facts": [
            {
                "id": "TypeContext",
                "rust_type": "TypeContext",
                "drop_fact": "TrivialMemory",
            },
            {
                "id": "MirValueKind",
                "rust_type": "MirValueKind",
                "transport": "DirectEnum",
                "variants": [
                    {"name": "Parameter", "payload": "u32"},
                    {"name": "Local", "payload": "u32"},
                    {"name": "Constant"},
                    {"name": "Temporary"},
                    {"name": "Pinned"},
                    {"name": "LoopCarrier"},
                ],
            },
            {
                "id": "ValueId",
                "rust_type": "ValueId",
                "transport": "i64",
            },
        ],
        "field_facts": [
            {
                "id": "TypeContext.value_kinds",
                "rust_type": "HashMap<ValueId, MirValueKind>",
                "key_transport": "ValueIdAsI64",
                "value_transport": "DirectEnum",
                "iteration_observed": False,
                "drop_fact": "TrivialMemory",
            },
        ],
        "body_facts": [
            {
                "id": "TypeContext::new",
                "operation": "NewMap",
                "selected_field": "value_kinds",
            },
            {
                "id": "TypeContext::try_get_kind",
                "operation": "MapGetOption",
                "selected_field": "value_kinds",
                "return": "Option<MirValueKind>",
                "copy_value": True,
            },
            {
                "id": "TypeContext::get_kind",
                "operation": "MapGetDefault",
                "selected_field": "value_kinds",
                "default": "MirValueKind::Temporary",
            },
            {
                "id": "TypeContext::set_kind",
                "operation": "MapSet",
                "selected_field": "value_kinds",
            },
        ],
        "excluded_methods": [
            {"id": "TypeContext::get_type", "deny_reason": "ReturnedReadBorrow"},
            {"id": "TypeContext::set_type", "deny_reason": "UnsupportedTypeTransport"},
            {"id": "TypeContext::get_origin_box", "deny_reason": "ReturnedReadBorrow"},
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
