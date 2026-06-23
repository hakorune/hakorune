#!/usr/bin/env python3
"""Extract facts for RegionObserver variable_map ordered read-fold."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
VARIABLE_CONTEXT = ROOT / "crates/hakorune_mir_builder/src/variable_context.rs"
REGION_OBSERVER = ROOT / "src/mir/region/observer.rs"


def _require(source: str, needle: str, label: str) -> None:
    if needle not in source:
        raise SystemExit(f"missing RegionObserver variable-map shape: {label}")


def _line_of(source: str, needle: str) -> int:
    for idx, line in enumerate(source.splitlines(), start=1):
        if needle in line:
            return idx
    raise SystemExit(f"missing line for: {needle}")


def extract_facts(
    variable_context_path: Path = VARIABLE_CONTEXT,
    observer_path: Path = REGION_OBSERVER,
) -> dict[str, Any]:
    variable_context = variable_context_path.read_text()
    observer = observer_path.read_text()

    for needle, label in [
        ("variable_map: BTreeMap<String, ValueId>", "variable_map field"),
        ("pub fn variable_map(&self) -> &BTreeMap<String, ValueId>", "returned aggregate borrow"),
    ]:
        _require(variable_context, needle, label)

    for needle, label in [
        ("fn classify_slots_from_variable_map(builder: &MirBuilder) -> Vec<SlotMetadata>", "consumer"),
        ("for (name, &vid) in builder.variable_ctx.variable_map().iter()", "ordered read fold"),
        ("let ref_kind = classify_slot(builder, vid, name.as_str());", "fold body classify"),
        ("name: name.clone()", "owned SlotMetadata name"),
    ]:
        _require(observer, needle, label)

    source_line = _line_of(observer, "for (name, &vid) in builder.variable_ctx.variable_map().iter()")

    return {
        "schema_version": 0,
        "kind": "RustLifecycleFacts",
        "subject": "mir::region::observer::classify_slots_from_variable_map",
        "source": {
            "variable_context_path": str(variable_context_path.relative_to(ROOT)),
            "consumer_path": str(observer_path.relative_to(ROOT)),
            "consumer_line": source_line,
        },
        "field_facts": [
            {
                "id": "VariableContext.variable_map",
                "rust_type": "BTreeMap<String, ValueId>",
                "key_transport": "String",
                "value_transport": "ValueIdAsI64",
                "iteration_order": "SourceOrdered",
                "map_identity_escapes": False,
                "drop_fact": "TrivialMemory",
            }
        ],
        "borrow_use_facts": [
            {
                "id": "RegionObserver::classify_slots_from_variable_map",
                "source": f"{observer_path.relative_to(ROOT)}:{source_line}",
                "borrowed_kind": "Aggregate",
                "consumer_kind": "ReadOnlyFold",
                "escapes": False,
                "owner_mutated_during_use": False,
                "identity_observed": False,
                "element_reference_escapes": False,
                "owned_projection_available": True,
                "order": "SourceOrdered",
                "output": "Vec<SlotMetadata>",
            }
        ],
        "excluded_methods": [
            {
                "id": "VariableContext::variable_map",
                "deny_reason": "ReturnedReadBorrow",
                "detail": "StandaloneAggregateReturn",
            }
        ],
    }


def main() -> None:
    print(json.dumps(extract_facts(), indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
