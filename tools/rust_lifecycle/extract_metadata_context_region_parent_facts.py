#!/usr/bin/env python3
"""Extract facts for MetadataContext region parent borrow-use elimination."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "crates/hakorune_mir_builder/src/metadata_context.rs"
CONSUMER = ROOT / "src/mir/region/observer.rs"


def _require(source: str, needle: str, label: str) -> None:
    if needle not in source:
        raise SystemExit(f"missing MetadataContext region-parent shape: {label}")


def extract_facts(source_path: Path = SOURCE, consumer_path: Path = CONSUMER) -> dict[str, Any]:
    source = source_path.read_text()
    consumer = consumer_path.read_text()
    for needle, label in [
        ("pub(super) current_region_stack: Vec<RegionIdT>", "current_region_stack field"),
        ("current_region_stack: Vec::new()", "current_region_stack init"),
        ("pub fn push_region(&mut self, region_id: RegionIdT)", "push_region"),
        ("self.current_region_stack.push(region_id);", "push_region push"),
        ("pub fn current_region_stack(&self) -> &[RegionIdT]", "standalone returned slice"),
    ]:
        _require(source, needle, label)
    _require(consumer, "builder.metadata_ctx.current_region_stack().last().copied()", "call-local last copied consumer")

    return {
        "schema_version": 0,
        "kind": "RustLifecycleFacts",
        "subject": "hakorune_mir_builder::metadata_context::MetadataContext.region_parent",
        "source": {
            "path": str(source_path.relative_to(ROOT)),
            "consumer_path": str(consumer_path.relative_to(ROOT)),
        },
        "type_facts": [
            {
                "id": "MetadataContext",
                "rust_type": "MetadataContext<SpanT, RegionIdT>",
                "selected_concrete_instantiation": "MetadataContext<i64, i64>",
                "generic_wide_claim": False,
                "drop_fact": "TrivialMemory",
            },
            {"id": "RegionIdT", "rust_type": "RegionIdT", "transport": "i64"},
        ],
        "field_facts": [
            {
                "id": "MetadataContext.current_region_stack",
                "rust_type": "Vec<RegionIdT>",
                "transport": "ArrayBox",
                "element_transport": "i64",
                "identity_escapes": False,
                "drop_fact": "TrivialMemory",
            }
        ],
        "body_facts": [
            {"id": "MetadataContext::new", "operation": "NewSequence", "selected_field": "current_region_stack"},
            {"id": "MetadataContext::push_region", "operation": "SequencePush", "selected_field": "current_region_stack"},
            {
                "id": "RegionObserver::parent_region",
                "operation": "SequenceLastOption",
                "selected_field": "current_region_stack",
                "source_chain": "current_region_stack().last().copied()",
            },
        ],
        "borrow_use_facts": [
            {
                "id": "RegionObserver::parent_region",
                "borrowed_kind": "Aggregate",
                "consumer_kind": "LastCopy",
                "escapes": False,
                "owner_mutated_during_use": False,
                "identity_observed": False,
                "order_observed": True,
                "element_reference_escapes": False,
                "owned_projection_available": True,
            }
        ],
        "excluded_methods": [
            {"id": "MetadataContext::current_region_stack", "deny_reason": "ReturnedReadBorrow", "detail": "StandaloneAggregateReturn"},
            {"id": "MetadataContext::value_origin_callers", "deny_reason": "ReturnedReadBorrow", "detail": "StandaloneAggregateReturn"},
        ],
    }


def main() -> None:
    print(json.dumps(extract_facts(), indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
