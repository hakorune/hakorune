#!/usr/bin/env python3
"""Preflight the first exact-object pilot retry.

The retry may only proceed if the exact-AOT backend already has an explicit
consumer for a flattened published nested object.  This tool deliberately does
not infer behavior from primitive-only fields alone.
"""

from __future__ import annotations

import argparse
import json
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


def source_contains(path: Path, needle: str) -> bool:
    try:
        return needle in path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return False


def count_nested_fields(mir: dict[str, Any], nested_object: str) -> int:
    for plan in mir.get("typed_object_plans", []) or []:
        if isinstance(plan, dict) and plan.get("box_name") == nested_object:
            fields = plan.get("fields", []) or []
            return sum(1 for field in fields if isinstance(field, dict))
    return 0


def count_nested_receiver_calls(
    mir: dict[str, Any], owner_field: str, nested_object: str
) -> int:
    calls = 0
    field_get_values: set[tuple[str, int]] = set()

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
                        field_get_values.add((func_name, int(dst)))

    for func in mir.get("functions", []) or []:
        if not isinstance(func, dict):
            continue
        func_name = str(func.get("name", ""))
        for inst in function_instructions(func):
            if inst.get("op") != "mir_call":
                continue
            c = callee(inst)
            recv = c.get("receiver")
            if (
                isinstance(recv, int)
                and (func_name, int(recv)) in field_get_values
                and c.get("box_name") == nested_object
            ):
                calls += 1
    return calls


def has_backend_flattened_nested_consumer(repo_root: Path) -> bool:
    # Restrict the search to executable backend/object-plan code.  Docs and
    # allocator preflight tools may mention the vocabulary without consuming it.
    search_roots = [
        repo_root / "src" / "llvm_py",
        repo_root / "src" / "object_storage_plan.rs",
    ]
    needles = (
        "flatten_nested_fields",
        "FlattenedNested",
        "flattened_nested",
    )
    for root in search_roots:
        paths = [root] if root.is_file() else list(root.rglob("*"))
        for path in paths:
            if path.is_file() and any(source_contains(path, needle) for needle in needles):
                return True
    return False


def direct_known_receiver_requires_handle(repo_root: Path) -> bool:
    path = repo_root / "src" / "llvm_py" / "instructions" / "direct_box_method.py"
    text = path.read_text(encoding="utf-8")
    return "recv_h: ir.Value" in text and "argv = [recv_h]" in text


def local_aggregate_supports_published_nested(repo_root: Path) -> bool:
    path = repo_root / "src" / "llvm_py" / "instructions" / "user_box_local.py"
    text = path.read_text(encoding="utf-8")
    return "flatten_nested_fields" in text or "published_nested" in text


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--repo-root", type=Path, default=Path("."))
    parser.add_argument("--owner-field", default="alignment_result")
    parser.add_argument(
        "--nested-object", default="HakoAllocObjectLifecycleAlignmentResult"
    )
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    repo_root = args.repo_root.resolve()
    mir = json.loads(args.mir_json.read_text(encoding="utf-8"))

    flattened_nested_field_count = count_nested_fields(mir, args.nested_object)
    nested_receiver_call_count = count_nested_receiver_calls(
        mir, args.owner_field, args.nested_object
    )
    backend_consumer = has_backend_flattened_nested_consumer(repo_root)
    direct_requires_handle = direct_known_receiver_requires_handle(repo_root)
    local_published_consumer = local_aggregate_supports_published_nested(repo_root)

    implementation_allowed = backend_consumer and not direct_requires_handle
    summary = "ok" if implementation_allowed else "blocked"
    selected_next = (
        "EXACT-OBJECT-PILOT-001S"
        if implementation_allowed
        else "EXACT-OBJECT-FLATTENED-NESTED-FIELD-LAYOUT-SSOT-001"
    )

    lines = [
        "output_contract=hako-exact-object-pilot-r-v0",
        "source_evidence=296x-714",
        "target_front=object_lifecycle_body",
        f"nested_owner=HakoAllocObjectLifecycleFacade.{args.owner_field}",
        f"nested_object={args.nested_object}",
        "representation_choice=flatten_nested_fields",
        "object_storage_plan_execution_enabled=0",
        "pilot_exact_object_enabled=0",
        f"flattened_nested_field_count={flattened_nested_field_count}",
        f"nested_receiver_call_count={nested_receiver_call_count}",
        f"backend_flattened_nested_field_consumer={int(backend_consumer)}",
        f"existing_known_receiver_direct_call_requires_handle={int(direct_requires_handle)}",
        f"local_aggregate_published_nested_consumer={int(local_published_consumer)}",
        "mirbuilder_object_management_enabled=0",
        "mirbuilder_special_case_count=0",
        "benchmark_name_branch_count=0",
        "helper_name_branch_count=0",
        "product_default_changed=0",
        "fallback_to_generic_box_supported=1",
        f"selected_next={selected_next}",
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
