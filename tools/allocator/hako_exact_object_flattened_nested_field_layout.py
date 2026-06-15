#!/usr/bin/env python3
"""Emit the passive flattened-nested-field layout contract for one object field."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


def nested_fields(mir: dict[str, Any], nested_object: str) -> list[dict[str, Any]]:
    for plan in mir.get("typed_object_plans", []) or []:
        if isinstance(plan, dict) and plan.get("box_name") == nested_object:
            fields = [field for field in plan.get("fields", []) or [] if isinstance(field, dict)]
            return sorted(fields, key=lambda field: int(field.get("slot", 0)))
    return []


def storage_to_scalar_type(storage: str) -> str:
    mapping = {
        "i64": "I64",
        "u64": "U64",
        "usize": "Usize",
        "bool": "Bool",
    }
    return mapping.get(storage, "Unsupported")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--owner-box", default="HakoAllocObjectLifecycleFacade")
    parser.add_argument("--owner-field", default="alignment_result")
    parser.add_argument(
        "--nested-object", default="HakoAllocObjectLifecycleAlignmentResult"
    )
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    mir = json.loads(args.mir_json.read_text(encoding="utf-8"))
    fields = nested_fields(mir, args.nested_object)
    flattened_names = [f"{args.owner_field}.{field.get('name')}" for field in fields]
    unsupported = [
        field for field in fields if storage_to_scalar_type(str(field.get("storage"))) == "Unsupported"
    ]
    summary = "blocked" if unsupported or not fields else "ok"

    lines = [
        "output_contract=hako-exact-object-flattened-nested-field-layout-ssot-v0",
        "source_evidence=296x-715",
        "target_front=object_lifecycle_body",
        f"nested_owner={args.owner_box}.{args.owner_field}",
        f"nested_object={args.nested_object}",
        "representation_choice=flatten_nested_fields",
        f"flattened_nested_field_count={len(fields)}",
    ]
    for index, name in enumerate(flattened_names):
        lines.append(f"flattened_field_name[{index}]={name}")
    for index, field in enumerate(fields):
        lines.append(f"flattened_field_scalar_type[{index}]={storage_to_scalar_type(str(field.get('storage')))}")
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
            "selected_next=EXACT-OBJECT-FLATTENED-NESTED-FIELD-SHADOW-001",
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
