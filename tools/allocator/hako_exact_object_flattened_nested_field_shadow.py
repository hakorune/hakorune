#!/usr/bin/env python3
"""Produce a shadow rewrite inventory for flattened nested object fields."""

from __future__ import annotations

import argparse
import json
from collections import defaultdict
from pathlib import Path
from typing import Any


READ_METHOD_FIELDS = {
    "requested": "last_requested",
    "normalized": "last_normalized",
    "reason": "last_reason",
    "supported": "last_supported",
}

WRITE_METHODS = {"recordFailure", "recordSuccess", "reset"}


def function_instructions(func: dict[str, Any]) -> list[dict[str, Any]]:
    out: list[dict[str, Any]] = []
    for block in func.get("blocks", []) or []:
        for inst in block.get("instructions", []) or []:
            if isinstance(inst, dict):
                out.append(inst)
    return out


def callee(inst: dict[str, Any]) -> dict[str, Any]:
    raw = inst.get("mir_call", {}).get("callee", {})
    return raw if isinstance(raw, dict) else {}


def typed_fields(mir: dict[str, Any], nested_object: str) -> list[dict[str, Any]]:
    for plan in mir.get("typed_object_plans", []) or []:
        if isinstance(plan, dict) and plan.get("box_name") == nested_object:
            fields = [field for field in plan.get("fields", []) or [] if isinstance(field, dict)]
            return sorted(fields, key=lambda field: int(field.get("slot", 0)))
    return []


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--owner-field", default="alignment_result")
    parser.add_argument(
        "--nested-object", default="HakoAllocObjectLifecycleAlignmentResult"
    )
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    mir = json.loads(args.mir_json.read_text(encoding="utf-8"))
    fields = typed_fields(mir, args.nested_object)
    flattened_field_names = {str(field.get("name")) for field in fields}

    field_get_values: set[tuple[str, int]] = set()
    field_set_values: set[tuple[str, int]] = set()
    uses_by_value: dict[tuple[str, int], list[dict[str, Any]]] = defaultdict(list)

    for func in mir.get("functions", []) or []:
        if not isinstance(func, dict):
            continue
        func_name = str(func.get("name", ""))
        for inst in function_instructions(func):
            op = inst.get("op")
            if op == "field_get" and inst.get("field") == args.owner_field:
                declared = inst.get("declared_type")
                if isinstance(declared, dict) and declared.get("box_type") == args.nested_object:
                    dst = inst.get("dst")
                    if isinstance(dst, int):
                        field_get_values.add((func_name, int(dst)))
            elif op == "field_set" and inst.get("field") == args.owner_field:
                declared = inst.get("declared_type")
                if isinstance(declared, dict) and declared.get("box_type") == args.nested_object:
                    value = inst.get("value")
                    if isinstance(value, int):
                        field_set_values.add((func_name, int(value)))

            if op == "mir_call":
                c = callee(inst)
                recv = c.get("receiver")
                if isinstance(recv, int):
                    uses_by_value[(func_name, int(recv))].append(inst)

    method_candidates = 0
    read_method_candidates = 0
    write_method_candidates = 0
    fallback_reasons: list[str] = []

    for key in sorted(field_get_values):
        for inst in uses_by_value.get(key, []):
            c = callee(inst)
            if c.get("box_name") != args.nested_object:
                fallback_reasons.append("non_nested_receiver_use")
                continue
            method = str(c.get("name", ""))
            method_candidates += 1
            if method in READ_METHOD_FIELDS:
                if READ_METHOD_FIELDS[method] in flattened_field_names:
                    read_method_candidates += 1
                else:
                    fallback_reasons.append(f"missing_read_field:{method}")
            elif method in WRITE_METHODS:
                write_method_candidates += 1
            else:
                fallback_reasons.append(f"unknown_nested_method:{method}")

    summary = "ok" if not fallback_reasons and fields else "blocked"

    lines = [
        "output_contract=hako-exact-object-flattened-nested-field-shadow-v0",
        "source_evidence=296x-716",
        "target_front=object_lifecycle_body",
        f"nested_owner=HakoAllocObjectLifecycleFacade.{args.owner_field}",
        f"nested_object={args.nested_object}",
        "representation_choice=flatten_nested_fields",
        f"flattened_nested_field_count={len(fields)}",
        f"rewritten_get_candidate_count={len(field_get_values)}",
        f"rewritten_set_candidate_count={len(field_set_values)}",
        f"rewritten_method_candidate_count={method_candidates}",
        f"read_method_candidate_count={read_method_candidates}",
        f"write_method_candidate_count={write_method_candidates}",
        f"fallback_reason_count={len(fallback_reasons)}",
    ]
    for index, reason in enumerate(fallback_reasons):
        lines.append(f"fallback_reason[{index}]={reason}")
    lines.extend(
        [
            "object_storage_plan_execution_enabled=0",
            "backend_lowering_enabled=0",
            "mirbuilder_object_management_enabled=0",
            "mirbuilder_special_case_count=0",
            "benchmark_name_branch_count=0",
            "helper_name_branch_count=0",
            "product_default_changed=0",
            "fallback_to_generic_box_supported=1",
            "selected_next=EXACT-OBJECT-FLATTENED-NESTED-FIELD-BACKEND-SEAM-001",
            f"summary={summary}",
        ]
    )
    text = "\n".join(lines) + "\n"
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    else:
        print(text, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
