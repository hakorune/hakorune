#!/usr/bin/env python3
"""Generic classifier fact lowering for MirBuilder converter slices."""

from __future__ import annotations

from typing import Any

from verified_hako_family_ir import op


def _facts_by_id(facts: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {row["id"]: row for row in facts.get("classifier_decision_facts", [])}


def _map_result(label: str, result_map: dict[str, str]) -> str:
    mapped = result_map.get(label)
    if not isinstance(mapped, str):
        raise ValueError("Deny(UnsupportedTypeTransport): detail=UnmappedClassifierResult")
    return mapped


def _compile_variant_groups(primary: dict[str, Any], result_map: dict[str, str]) -> list[dict[str, Any]]:
    if primary.get("selector") != "TaggedVariant":
        raise ValueError("Deny(UnsupportedDirectShape): detail=UnsupportedClassifierPattern")
    groups = []
    seen: set[str] = set()
    for case in primary.get("cases", []):
        variants = []
        for variant in case.get("variants", []):
            tag = variant.get("tag")
            payload = variant.get("payload")
            if not isinstance(tag, str) or tag in seen:
                raise ValueError("Deny(UnsupportedDirectShape): detail=UnsupportedClassifierPattern")
            seen.add(tag)
            if payload == "Ignored":
                variants.append({"name": tag, "payload_var": f"_{tag.lower()}_payload"})
            elif payload == "None":
                variants.append(tag)
            else:
                raise ValueError("Deny(UnsupportedDirectShape): detail=UnsupportedClassifierPattern")
        groups.append({"variants": variants, "returns": _map_result(case.get("result"), result_map)})
    return groups


def compile_classifier_operation(facts: dict[str, Any], plan: dict[str, Any]) -> dict[str, Any]:
    """Compile classifier facts into the existing ClassifyEnumVariants operation."""

    classifier_id = plan.get("classifier_fact_id")
    classifier = _facts_by_id(facts).get(classifier_id)
    if classifier is None or classifier.get("kind") != "ClassifierDecisionFactsV1":
        raise ValueError("Deny(UnsupportedDirectShape): detail=UnsupportedClassifierPattern")

    output = plan.get("classifier_output", {})
    result_map = output.get("result_variants", {})
    if not isinstance(result_map, dict):
        raise ValueError("Deny(UnsupportedTypeTransport): detail=UnmappedClassifierResult")

    primary = classifier.get("primary", {})
    missing = classifier.get("missing_primary", {})
    if missing.get("selector") != "StringSet":
        raise ValueError("Deny(UnsupportedDirectShape): detail=DynamicClassifierPredicate")

    values = missing.get("values")
    if not isinstance(values, list) or not all(isinstance(value, str) for value in values):
        raise ValueError("Deny(UnsupportedDirectShape): detail=DynamicClassifierPredicate")

    return op(
        "ClassifyEnumVariants",
        type_source=plan.get("type_source", "type_opt"),
        source_enum=primary.get("source_enum"),
        variant_groups=_compile_variant_groups(primary, result_map),
        default_return=_map_result(primary.get("default"), result_map),
        missing_value_fallback={
            "input": missing.get("input", "name"),
            "string_set": values,
            "matched": _map_result(missing.get("matched"), result_map),
            "unmatched": _map_result(missing.get("unmatched"), result_map),
        },
    ).to_json()
