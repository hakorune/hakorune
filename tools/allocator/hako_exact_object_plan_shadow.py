#!/usr/bin/env python3
"""Report exact-object storage plan candidates without changing execution."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]

PRIMITIVE_STORAGES = {"i64", "usize", "u64", "bool"}


def repo_contains(rel: str, needle: str) -> bool:
    path = ROOT / rel
    return path.is_file() and needle in path.read_text(encoding="utf-8", errors="replace")


def normalize_storage(raw: Any) -> str:
    return str(raw or "").strip().lower()


def field_storages(plan: dict[str, Any]) -> list[str]:
    return [normalize_storage(field.get("storage")) for field in plan.get("fields", []) or []]


def is_primitive_only(plan: dict[str, Any]) -> bool:
    storages = field_storages(plan)
    return bool(storages) and all(storage in PRIMITIVE_STORAGES for storage in storages)


def has_handle_storage(plan: dict[str, Any]) -> bool:
    return "handle" in field_storages(plan)


def has_unknown_storage(plan: dict[str, Any]) -> bool:
    return any(storage not in PRIMITIVE_STORAGES and storage != "handle" for storage in field_storages(plan))


def select_pilot_candidate(plans: list[dict[str, Any]]) -> tuple[str, str, int]:
    candidates = [
        plan
        for plan in plans
        if is_primitive_only(plan) and len(plan.get("fields", []) or []) <= 4
    ]
    if not candidates:
        return "none", "low", 0

    # Prefer the smallest primitive-only object. This is a shadow candidate, not
    # an execution commitment; the next row must still prove backend lowering.
    candidate = sorted(
        candidates,
        key=lambda plan: (len(plan.get("fields", []) or []), str(plan.get("box_name", ""))),
    )[0]
    return str(candidate.get("box_name", "unknown")), "medium", 1


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    mir = json.loads(args.mir_json.read_text(encoding="utf-8"))
    typed_plans = [
        plan for plan in mir.get("typed_object_plans", []) if isinstance(plan, dict)
    ]
    if not typed_plans:
        raise SystemExit("no typed_object_plans found in MIR JSON")

    counts: Counter[str] = Counter()
    for plan in typed_plans:
        fields = plan.get("fields", []) or []
        if not fields or has_unknown_storage(plan):
            counts["generic_box"] += 1
        if has_handle_storage(plan):
            counts["host_handle_escaped"] += 1
        if is_primitive_only(plan):
            counts["exact_stack_object"] += 1
            counts["exact_native_struct"] += 1
            counts["scalarized"] += 1
        elif any(storage in PRIMITIVE_STORAGES for storage in field_storages(plan)):
            counts["exact_native_struct"] += 1

    arc_carrier_visible = repo_contains(
        "src/runtime/host_handles.rs", "StableBox(Arc<dyn NyashBox>)"
    )
    vmvalue_arc_visible = repo_contains("src/backend/vm_types.rs", "BoxRef(Arc<dyn NyashBox>)")
    arc_dynbox_plan_count = int(arc_carrier_visible) + int(vmvalue_arc_visible)

    candidate, confidence, allowed = select_pilot_candidate(typed_plans)

    lines = [
        "output_contract=hako-exact-object-plan-shadow-v0",
        "source_evidence=296x-711",
        "target_front=object_lifecycle_body",
        "object_storage_plan_vocabulary_defined=1",
        "object_storage_plan_execution_enabled=0",
        "exact_object_shadow_enabled=1",
        f"generic_box_plan_count={counts['generic_box']}",
        f"host_handle_escaped_plan_count={counts['host_handle_escaped']}",
        f"arc_dynbox_plan_count={arc_dynbox_plan_count}",
        f"exact_stack_object_plan_count={counts['exact_stack_object']}",
        f"exact_native_struct_plan_count={counts['exact_native_struct']}",
        f"scalarized_plan_count={counts['scalarized']}",
        f"selected_pilot_candidate={candidate}",
        f"selected_pilot_confidence={confidence}",
        f"pilot_allowed={allowed}",
        "product_default_changed=0",
        "source_hako_changed=0",
        "compiler_lowering_changed=0",
        "runtime_object_changed=0",
        "summary=ok",
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
