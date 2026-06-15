#!/usr/bin/env python3
"""Select a nested-object publication representation for one object field."""

from __future__ import annotations

import argparse
import json
from collections import defaultdict
from pathlib import Path
from typing import Any


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


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--owner-field", default="alignment_result")
    parser.add_argument("--nested-object", required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    mir = json.loads(args.mir_json.read_text(encoding="utf-8"))
    owner_field = args.owner_field
    nested_object = args.nested_object

    field_get_values: list[tuple[str, int]] = []
    field_set_values: list[tuple[str, int]] = []
    uses_by_value: dict[tuple[str, int], list[dict[str, Any]]] = defaultdict(list)

    for func in mir.get("functions", []) or []:
        if not isinstance(func, dict):
            continue
        func_name = str(func.get("name", ""))
        for inst in function_instructions(func):
            if inst.get("op") == "field_get" and inst.get("field") == owner_field:
                declared = inst.get("declared_type")
                if isinstance(declared, dict) and declared.get("box_type") == nested_object:
                    dst = inst.get("dst")
                    if isinstance(dst, int):
                        field_get_values.append((func_name, int(dst)))
            if inst.get("op") == "field_set" and inst.get("field") == owner_field:
                declared = inst.get("declared_type")
                if isinstance(declared, dict) and declared.get("box_type") == nested_object:
                    value = inst.get("value")
                    if isinstance(value, int):
                        field_set_values.append((func_name, int(value)))

            op = inst.get("op")
            if op == "mir_call":
                c = callee(inst)
                recv = c.get("receiver")
                if isinstance(recv, int):
                    uses_by_value[(func_name, int(recv))].append(inst)
                for arg in inst.get("mir_call", {}).get("args", []) or inst.get("args", []) or []:
                    if isinstance(arg, int):
                        uses_by_value[(func_name, int(arg))].append(inst)
            elif op == "ret":
                value = inst.get("value")
                if isinstance(value, int):
                    uses_by_value[(func_name, int(value))].append(inst)
            elif op in {"field_set", "boxcall", "call"}:
                for key in ("box", "value"):
                    value = inst.get(key)
                    if isinstance(value, int):
                        uses_by_value[(func_name, int(value))].append(inst)
                for arg in inst.get("args", []) or []:
                    if isinstance(arg, int):
                        uses_by_value[(func_name, int(arg))].append(inst)

    nested_receiver_calls = 0
    escaping_uses = 0
    for key in field_get_values:
        _, value = key
        for inst in uses_by_value.get(key, []):
            if inst.get("op") == "mir_call":
                c = callee(inst)
                if c.get("receiver") == value and c.get("box_name") == nested_object:
                    nested_receiver_calls += 1
                    continue
            escaping_uses += 1

    representation_choice = "flatten_nested_fields"
    summary = "ok"
    if escaping_uses:
        representation_choice = "materialized_view_handle"
        summary = "blocked"

    lines = [
        "output_contract=hako-exact-object-nested-publication-plan-v0",
        "source_evidence=296x-713",
        "target_front=object_lifecycle_body",
        f"nested_owner=HakoAllocObjectLifecycleFacade.{owner_field}",
        f"nested_object={nested_object}",
        f"publication_boundary_count={len(field_get_values) + len(field_set_values)}",
        f"facade_nested_field_set_count={len(field_set_values)}",
        f"facade_nested_field_get_count={len(field_get_values)}",
        f"nested_receiver_call_count={nested_receiver_calls}",
        f"nested_handle_escape_count={escaping_uses}",
        f"representation_choice={representation_choice}",
        "mirbuilder_object_management_enabled=0",
        "benchmark_name_branch_count=0",
        "helper_name_branch_count=0",
        "product_default_changed=0",
        "fallback_to_generic_box_supported=1",
        f"summary={summary}",
    ]
    text = "\n".join(lines) + "\n"
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    else:
        print(text, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
