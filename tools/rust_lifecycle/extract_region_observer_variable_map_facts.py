#!/usr/bin/env python3
"""Extract facts for RegionObserver variable_map ordered read-fold."""

from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Any

from mirbuilder_ordering_capability import RUST_STRING_ORD_V1, key_ascending


ROOT = Path(__file__).resolve().parents[2]
VARIABLE_CONTEXT = ROOT / "crates/hakorune_mir_builder/src/variable_context.rs"
REGION = ROOT / "src/mir/region/mod.rs"
REGION_OBSERVER = ROOT / "src/mir/region/observer.rs"


def _require(source: str, needle: str, label: str) -> None:
    if needle not in source:
        raise SystemExit(f"missing RegionObserver variable-map shape: {label}")


def _line_of(source: str, needle: str) -> int:
    for idx, line in enumerate(source.splitlines(), start=1):
        if needle in line:
            return idx
    raise SystemExit(f"missing line for: {needle}")


def _display_path(path: Path) -> str:
    try:
        return str(path.relative_to(ROOT))
    except ValueError:
        return str(path)


def _parse_ref_kind_groups(region: str) -> tuple[list[dict[str, Any]], str]:
    groups: list[dict[str, Any]] = []
    seen: set[str] = set()
    non_ref_seen = False
    arm_re = re.compile(r"(?P<patterns>MirType::[^=]+?)=>\s*RefSlotKind::(?P<result>[A-Za-z0-9_]+)")
    for match in arm_re.finditer(region):
        patterns = match.group("patterns")
        result = match.group("result")
        variants = []
        for raw in patterns.split("|"):
            item = raw.strip().rstrip(",")
            variant_match = re.fullmatch(r"MirType::(?P<name>[A-Za-z0-9_]+)(?:\((?P<payload>_)\))?", item)
            if not variant_match:
                raise SystemExit("Deny(UnsupportedDirectShape): detail=UnsupportedClassifierPattern")
            name = variant_match.group("name")
            if name in seen:
                raise SystemExit("Deny(UnsupportedDirectShape): detail=UnsupportedClassifierPattern duplicate_variant")
            seen.add(name)
            payload = "Ignored" if variant_match.group("payload") else "None"
            variants.append({"tag": name, "payload": payload})
        if result == "NonRef":
            non_ref_seen = True
            continue
        groups.append({"variants": variants, "result": result})
    if not groups or not non_ref_seen:
        raise SystemExit("Deny(UnsupportedDirectShape): detail=AmbiguousClassifierDefault")
    return groups, "NonRef"


def _parse_name_fallback(observer: str) -> dict[str, Any]:
    body_match = re.search(
        r"fn classify_slot_name_only\(name: &str\) -> RefSlotKind \{(?P<body>.*?)\n\}",
        observer,
        re.S,
    )
    if not body_match:
        raise SystemExit("Deny(UnsupportedDirectShape): detail=UnsupportedClassifierPattern")
    body = body_match.group("body")
    values = re.findall(r'"([^"]+)"', body)
    if not values:
        raise SystemExit("Deny(UnsupportedDirectShape): detail=DynamicClassifierPredicate")
    if "RefSlotKind::StrongRoot" not in body or "RefSlotKind::NonRef" not in body:
        raise SystemExit("Deny(UnsupportedDirectShape): detail=AmbiguousClassifierDefault")
    return {
        "selector": "StringSet",
        "input": "name",
        "values": values,
        "matched": "StrongRoot",
        "unmatched": "NonRef",
    }


def _classifier_facts(region: str, observer: str) -> dict[str, Any]:
    groups, default = _parse_ref_kind_groups(region)
    return {
        "id": "RegionObserver.slot_ref_kind",
        "kind": "ClassifierDecisionFactsV1",
        "primary": {
            "selector": "TaggedVariant",
            "source_enum": "MirType",
            "cases": groups,
            "default": default,
        },
        "missing_primary": _parse_name_fallback(observer),
    }


def extract_facts(
    variable_context_path: Path = VARIABLE_CONTEXT,
    region_path: Path = REGION,
    observer_path: Path = REGION_OBSERVER,
) -> dict[str, Any]:
    variable_context = variable_context_path.read_text()
    region = region_path.read_text()
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
            "variable_context_path": _display_path(variable_context_path),
            "region_path": _display_path(region_path),
            "consumer_path": _display_path(observer_path),
            "consumer_line": source_line,
        },
        "classifier_decision_facts": [_classifier_facts(region, observer)],
        "field_facts": [
            {
                "id": "VariableContext.variable_map",
                "rust_type": "BTreeMap<String, ValueId>",
                "key_transport": "String",
                "value_transport": "ValueIdAsI64",
                "iteration_order": key_ascending(RUST_STRING_ORD_V1),
                "map_identity_escapes": False,
                "drop_fact": "TrivialMemory",
            }
        ],
        "borrow_use_facts": [
            {
                "id": "RegionObserver::classify_slots_from_variable_map",
                "source": f"{_display_path(observer_path)}:{source_line}",
                "borrowed_kind": "Aggregate",
                "consumer_kind": "ReadOnlyFold",
                "escapes": False,
                "owner_mutated_during_use": False,
                "identity_observed": False,
                "element_reference_escapes": False,
                "owned_projection_available": True,
                "order": key_ascending(RUST_STRING_ORD_V1),
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
