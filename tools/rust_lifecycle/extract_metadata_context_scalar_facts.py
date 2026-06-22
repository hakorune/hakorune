#!/usr/bin/env python3
"""Extract lightweight facts for MetadataContext scalar/source-file conversion."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "crates/hakorune_mir_builder/src/metadata_context.rs"


def _require(source: str, needle: str, label: str) -> None:
    if needle not in source:
        raise SystemExit(f"missing MetadataContext scalar/source-file shape: {label}")


def extract_facts(source_path: Path = SOURCE) -> dict[str, Any]:
    source = source_path.read_text()
    for needle, label in [
        ("pub struct MetadataContext<SpanT: Copy, RegionIdT: Copy>", "generic MetadataContext"),
        ("pub(super) current_span: SpanT", "current_span field"),
        ("pub(super) source_file: Option<String>", "source_file field"),
        ("pub fn new(current_span: SpanT) -> Self", "new"),
        ("current_span,", "new current_span assignment"),
        ("source_file: None", "new source_file none"),
        ("pub fn current_span(&self) -> SpanT", "current_span getter"),
        ("self.current_span", "current_span read"),
        ("pub fn set_current_span(&mut self, span: SpanT)", "set_current_span"),
        ("self.current_span = span;", "set_current_span write"),
        ("pub fn set_source_file<S: Into<String>>(&mut self, source: S)", "set_source_file"),
        ("self.source_file = Some(source.into());", "set_source_file some"),
        ("pub fn clear_source_file(&mut self)", "clear_source_file"),
        ("self.source_file = None;", "clear_source_file none"),
        ("pub fn current_source_file(&self) -> Option<String>", "current_source_file"),
        ("self.source_file.clone()", "current_source_file clone"),
    ]:
        _require(source, needle, label)

    return {
        "schema_version": 0,
        "kind": "RustLifecycleFacts",
        "subject": "hakorune_mir_builder::metadata_context::MetadataContext.scalar_source_file",
        "source": {"path": str(source_path.relative_to(ROOT))},
        "type_facts": [
            {
                "id": "MetadataContext",
                "rust_type": "MetadataContext<SpanT, RegionIdT>",
                "selected_concrete_instantiation": "MetadataContext<i64, i64>",
                "generic_wide_claim": False,
                "drop_fact": "TrivialMemory",
            },
            {
                "id": "SpanT",
                "rust_type": "SpanT",
                "selected_transport": "i64",
            },
            {
                "id": "RegionIdT",
                "rust_type": "RegionIdT",
                "selected_transport": "i64",
            },
            {
                "id": "String",
                "rust_type": "String",
                "transport": "ImmutableStringAtom",
            },
        ],
        "field_facts": [
            {
                "id": "MetadataContext.current_span",
                "rust_type": "SpanT",
                "transport": "i64",
                "drop_fact": "TrivialMemory",
            },
            {
                "id": "MetadataContext.source_file",
                "rust_type": "Option<String>",
                "transport": "OptionStringBox",
                "returned_aggregate_alias": False,
                "drop_fact": "TrivialMemory",
            },
        ],
        "body_facts": [
            {
                "id": "MetadataContext::new",
                "operation": "ConstructScalarOptionContext",
            },
            {
                "id": "MetadataContext::current_span",
                "operation": "FieldGet",
                "selected_field": "current_span",
                "return": "SpanT",
            },
            {
                "id": "MetadataContext::set_current_span",
                "operation": "FieldSet",
                "selected_field": "current_span",
            },
            {
                "id": "MetadataContext::set_source_file",
                "operation": "SetSome",
                "selected_field": "source_file",
            },
            {
                "id": "MetadataContext::clear_source_file",
                "operation": "ClearOption",
                "selected_field": "source_file",
            },
            {
                "id": "MetadataContext::current_source_file",
                "operation": "CloneImmutableString",
                "selected_field": "source_file",
                "return": "Option<String>",
                "returned_aggregate_alias": False,
            },
        ],
        "excluded_methods": [
            {"id": "MetadataContext::hint_scope_enter", "deny_reason": "OutOfSlice"},
            {"id": "MetadataContext::hint_scope_leave", "deny_reason": "OutOfSlice"},
            {"id": "MetadataContext::hint_join_result", "deny_reason": "OutOfSlice"},
            {"id": "MetadataContext::push_region", "deny_reason": "OutOfSlice"},
            {"id": "MetadataContext::pop_region", "deny_reason": "OutOfSlice"},
            {"id": "MetadataContext::current_region_stack", "deny_reason": "ReturnedReadBorrow"},
            {"id": "MetadataContext::record_value_span", "deny_reason": "OutOfSlice"},
            {"id": "MetadataContext::value_span", "deny_reason": "OutOfSlice"},
            {"id": "MetadataContext::record_value_caller", "deny_reason": "UnsupportedTypeTransport"},
            {"id": "MetadataContext::value_caller", "deny_reason": "ReturnedReadBorrow"},
            {"id": "MetadataContext::value_origin_callers", "deny_reason": "ReturnedReadBorrow"},
        ],
    }


def main() -> None:
    print(json.dumps(extract_facts(), indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
