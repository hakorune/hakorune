#!/usr/bin/env python3
"""Extract lightweight facts for TypeContext.string_literals leaf projection."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
TYPE_CONTEXT_SOURCE = ROOT / "crates/hakorune_mir_builder/src/type_context.rs"
SOURCE = ROOT / "src/mir/builder/types/map_value.rs"


def _require(source: str, needle: str, label: str) -> None:
    if needle not in source:
        raise SystemExit(f"missing TypeContext string-literal shape: {label}")


def extract_facts(source_path: Path = SOURCE) -> dict[str, Any]:
    type_context_source = TYPE_CONTEXT_SOURCE.read_text()
    source = source_path.read_text()
    _require(type_context_source, "pub string_literals: BTreeMap<ValueId, String>", "string_literals field")
    _require(source, "fn string_literal(builder: &MirBuilder, value: ValueId) -> Option<String>", "helper signature")
    _require(source, "builder.type_ctx.string_literals.get(&value).cloned()", "get cloned projection")

    return {
        "schema_version": 0,
        "kind": "RustLifecycleFacts",
        "subject": "hakorune_mir_builder::type_context::TypeContext.string_literals",
        "source": {"path": str(source_path.relative_to(ROOT))},
        "type_facts": [
            {"id": "TypeContext", "rust_type": "TypeContext", "drop_fact": "TrivialMemory"},
            {"id": "ValueId", "rust_type": "ValueId", "transport": "i64"},
            {"id": "String", "rust_type": "String", "transport": "ImmutableStringAtom"},
        ],
        "field_facts": [
            {
                "id": "TypeContext.string_literals",
                "rust_type": "BTreeMap<ValueId, String>",
                "key_transport": "ValueIdAsI64",
                "value_transport": "ImmutableStringAtom",
                "iteration_observed": False,
                "map_identity_escapes": False,
                "drop_fact": "TrivialMemory",
            }
        ],
        "body_facts": [
            {"id": "TypeContext::new", "operation": "NewMap", "selected_field": "string_literals"},
            {
                "id": "map_value::string_literal",
                "operation": "MapGetOption",
                "selected_field": "string_literals",
                "return": "Option<String>",
                "value_projection": "ImmutableStringAtom",
                "returned_aggregate_alias": False,
            },
        ],
        "borrow_use_facts": [
            {
                "id": "TypeContext::string_literals.get_cloned",
                "borrowed_kind": "Aggregate",
                "consumer_kind": "GetClone",
                "escapes": False,
                "owner_mutated_during_use": False,
                "identity_observed": False,
                "order": "Unobserved",
                "owned_projection_available": True,
                "element_reference_escapes": False,
            }
        ],
        "excluded_methods": [
            {"id": "emit_string", "deny_reason": "OutOfSlice"},
            {"id": "TypeContext::take_snapshot", "deny_reason": "OutOfSlice"},
            {"id": "TypeContext::restore_snapshot", "deny_reason": "OutOfSlice"},
            {"id": "TypeContext.map_value_types", "deny_reason": "OutOfSlice"},
            {"id": "TypeContext.map_literal_value_types", "deny_reason": "OutOfSlice"},
        ],
    }


def main() -> None:
    print(json.dumps(extract_facts(), indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
