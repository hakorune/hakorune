#!/usr/bin/env python3
"""Extract lightweight facts for MetadataContext.value_caller conversion."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "crates/hakorune_mir_builder/src/metadata_context.rs"
PHI_LIFECYCLE_SOURCE = ROOT / "src/mir/builder/emission/phi_lifecycle.rs"
MODULE_LIFECYCLE_SOURCE = ROOT / "src/mir/builder/module_lifecycle.rs"
CALL_LOWERING_SOURCE = ROOT / "src/mir/builder/calls/lowering.rs"


def _require(source: str, needle: str, label: str) -> None:
    if needle not in source:
        raise SystemExit(f"missing MetadataContext value-caller shape: {label}")


def extract_facts(source_path: Path = SOURCE) -> dict[str, Any]:
    source = source_path.read_text()
    consumer_source = PHI_LIFECYCLE_SOURCE.read_text()
    module_lifecycle_source = MODULE_LIFECYCLE_SOURCE.read_text()
    call_lowering_source = CALL_LOWERING_SOURCE.read_text()
    for needle, label in [
        ("pub(super) value_origin_callers: HashMap<ValueId, String>", "value_origin_callers field"),
        ("pub fn new(current_span: SpanT) -> Self", "new"),
        ("value_origin_callers: HashMap::new()", "value_origin_callers init"),
        ("pub fn value_caller(&self, value_id: ValueId) -> Option<&str>", "value_caller"),
        ("self.value_origin_callers.get(&value_id).map(|s| s.as_str())", "value_caller immutable leaf projection"),
    ]:
        _require(source, needle, label)
    _require(consumer_source, ".value_origin_callers()", "value_origin_callers aggregate borrow consumer")
    _require(consumer_source, ".get(&dst)", "value_origin_callers get dst consumer")
    _require(consumer_source, ".cloned()", "value_origin_callers cloned consumer")
    for fold_source, label in [
        (module_lifecycle_source, "module lifecycle value_origin_callers read fold"),
        (call_lowering_source, "call lowering value_origin_callers read fold"),
    ]:
        _require(fold_source, "let mut origin_callers = ", label)
        _require(fold_source, ".metadata.value_origin_callers.clone()", label)
        _require(fold_source, "for (k, v) in self.metadata_ctx.value_origin_callers().iter()", label)
        _require(fold_source, "origin_callers.insert(*k, v.clone())", label)
        _require(fold_source, ".metadata.value_origin_callers = origin_callers", label)

    fold_semantics = {
        "input": "MapEntries",
        "key_projection": "Copy(ValueIdAsI64)",
        "value_projection": "OwnedImmutableAtom",
        "base": "CloneOwned",
        "collision": "SourceWins",
        "output": "OwnedOrderedMap",
        "output_order": "KeyAscending(ValueIdOrdV1)",
        "source_destination_alias": False,
        "source_mutated_during_use": False,
        "element_reference_escapes": False,
        "destination_identity_observed": False,
    }

    return {
        "schema_version": 0,
        "kind": "RustLifecycleFacts",
        "subject": "hakorune_mir_builder::metadata_context::MetadataContext.value_caller",
        "source": {"path": str(source_path.relative_to(ROOT))},
        "type_facts": [
            {
                "id": "MetadataContext",
                "rust_type": "MetadataContext<SpanT, RegionIdT>",
                "selected_concrete_instantiation": "MetadataContext<i64, i64>",
                "generic_wide_claim": False,
                "drop_fact": "TrivialMemory",
            },
            {"id": "ValueId", "rust_type": "ValueId", "transport": "i64"},
            {"id": "String", "rust_type": "String", "transport": "ImmutableStringAtom"},
        ],
        "field_facts": [
            {
                "id": "MetadataContext.value_origin_callers",
                "rust_type": "HashMap<ValueId, String>",
                "key_transport": "ValueIdAsI64",
                "value_transport": "ImmutableStringAtom",
                "iteration_observed": False,
                "key_domain_roundtrip": "CanonicalI64Text",
                "map_identity_escapes": False,
                "drop_fact": "TrivialMemory",
            },
        ],
        "body_facts": [
            {
                "id": "MetadataContext::new",
                "operation": "NewMap",
                "selected_field": "value_origin_callers",
            },
            {
                "id": "MetadataContext::value_caller",
                "operation": "MapGetOption",
                "selected_field": "value_origin_callers",
                "return": "Option<&str>",
                "value_projection": "ImmutableStringAtom",
                "returned_aggregate_alias": False,
            },
        ],
        "borrow_use_facts": [
            {
                "id": "MetadataContext::value_origin_callers.get_cloned",
                "source": "src/mir/builder/emission/phi_lifecycle.rs",
                "borrowed_kind": "Aggregate",
                "consumer_kind": "GetClone",
                "escapes": False,
                "owner_mutated_during_use": False,
                "identity_observed": False,
                "element_reference_escapes": False,
                "owned_projection_available": True,
                "order": "Unobserved",
            },
            {
                "id": "MetadataContext::value_origin_callers.iter_owned_copy.finalize_module",
                "source": "src/mir/builder/module_lifecycle.rs",
                "borrowed_kind": "Aggregate",
                "consumer_kind": "ReadOnlyFold",
                "escapes": False,
                "owner_mutated_during_use": False,
                "identity_observed": False,
                "element_reference_escapes": False,
                "owned_projection_available": True,
                "order": "Unobserved",
                "fold_semantics": fold_semantics,
            },
            {
                "id": "MetadataContext::value_origin_callers.iter_owned_copy.finalize_function",
                "source": "src/mir/builder/calls/lowering.rs",
                "borrowed_kind": "Aggregate",
                "consumer_kind": "ReadOnlyFold",
                "escapes": False,
                "owner_mutated_during_use": False,
                "identity_observed": False,
                "element_reference_escapes": False,
                "owned_projection_available": True,
                "order": "Unobserved",
                "fold_semantics": fold_semantics,
                "parity_only": True,
            },
        ],
        "excluded_methods": [
            {"id": "MetadataContext::record_value_caller", "deny_reason": "UnsupportedResolvedCallTarget"},
            {"id": "MetadataContext::value_origin_callers", "deny_reason": "ReturnedReadBorrow", "detail": "StandaloneAggregateReturn"},
            {"id": "MetadataContext::current_region_stack", "deny_reason": "ReturnedReadBorrow", "detail": "StandaloneAggregateReturn"},
            {"id": "MetadataContext::current_span", "deny_reason": "OutOfSlice"},
            {"id": "MetadataContext::set_current_span", "deny_reason": "OutOfSlice"},
            {"id": "MetadataContext::current_source_file", "deny_reason": "OutOfSlice"},
        ],
    }


def main() -> None:
    print(json.dumps(extract_facts(), indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
