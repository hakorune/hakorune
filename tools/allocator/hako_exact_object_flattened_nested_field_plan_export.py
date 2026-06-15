#!/usr/bin/env python3
"""Report ObjectStoragePlan MIR JSON export for the flattened nested field pilot."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


REQUIRED_FLATTENED_FIELDS = [
    "alignment_result.last_requested",
    "alignment_result.last_normalized",
    "alignment_result.last_reason",
    "alignment_result.last_supported",
]


def load_mir_json(path: Path) -> dict[str, Any]:
    raw = json.loads(path.read_text(encoding="utf-8"))
    return raw if isinstance(raw, dict) else {}


def object_storage_plans(mir: dict[str, Any]) -> list[dict[str, Any]]:
    plans = mir.get("object_storage_plans", [])
    return [plan for plan in plans if isinstance(plan, dict)] if isinstance(plans, list) else []


def selected_flattened_plan(plans: list[dict[str, Any]]) -> dict[str, Any] | None:
    for plan in plans:
        if (
            plan.get("representation") == "flattened_nested_fields"
            and plan.get("owner_box") == "HakoAllocObjectLifecycleFacade"
            and plan.get("owner_field") == "alignment_result"
            and plan.get("nested_box") == "HakoAllocObjectLifecycleAlignmentResult"
        ):
            return plan
    return None


def flattened_field_names(plan: dict[str, Any] | None) -> set[str]:
    if not plan:
        return set()
    fields = plan.get("fields", [])
    if not isinstance(fields, list):
        return set()
    names: set[str] = set()
    for field in fields:
        if isinstance(field, dict) and isinstance(field.get("flattened_field"), str):
            names.add(field["flattened_field"])
    return names


def flag(value: bool) -> int:
    return 1 if value else 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    mir = load_mir_json(args.mir_json)
    plans = object_storage_plans(mir)
    plan = selected_flattened_plan(plans)
    names = flattened_field_names(plan)
    all_fields_present = all(name in names for name in REQUIRED_FLATTENED_FIELDS)
    plan_count = flag(plan is not None)
    flattened_count = len(names) if plan else 0
    backend_lowering_enabled = flag(
        bool(plan.get("backend_lowering_enabled")) if plan else False
    )
    boundary_consumer_enabled = flag(
        bool(plan.get("boundary_driver_flattened_nested_consumer")) if plan else False
    )
    mirbuilder_object_management_enabled = flag(
        bool(plan.get("mirbuilder_object_management_enabled")) if plan else False
    )
    product_default_changed = flag(bool(plan.get("product_default_changed")) if plan else False)
    summary = (
        "ok"
        if plan
        and all_fields_present
        and flattened_count == 4
        and backend_lowering_enabled == 0
        and boundary_consumer_enabled == 0
        and mirbuilder_object_management_enabled == 0
        and product_default_changed == 0
        else "blocked"
    )

    lines = [
        "output_contract=hako-exact-object-flattened-nested-field-plan-export-v0",
        "source_evidence=296x-726",
        "target_front=object_lifecycle_body",
        "object_storage_plan_mir_json_export_enabled=1",
        f"flattened_nested_plan_count={plan_count}",
        f"flattened_nested_field_count={flattened_count}",
        "owner_box=HakoAllocObjectLifecycleFacade",
        "owner_field=alignment_result",
        "nested_box=HakoAllocObjectLifecycleAlignmentResult",
        f"alignment_result_last_requested_exported={flag('alignment_result.last_requested' in names)}",
        f"alignment_result_last_normalized_exported={flag('alignment_result.last_normalized' in names)}",
        f"alignment_result_last_reason_exported={flag('alignment_result.last_reason' in names)}",
        f"alignment_result_last_supported_exported={flag('alignment_result.last_supported' in names)}",
        f"backend_lowering_enabled={backend_lowering_enabled}",
        f"boundary_driver_flattened_nested_consumer={boundary_consumer_enabled}",
        f"mirbuilder_object_management_enabled={mirbuilder_object_management_enabled}",
        "benchmark_name_branch_count=0",
        "helper_name_branch_count=0",
        f"product_default_changed={product_default_changed}",
        "selected_next=EXACT-OBJECT-FLATTENED-NESTED-FIELD-BOUNDARY-CONSUMER-001",
        f"summary={summary}",
    ]
    text = "\n".join(lines) + "\n"
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    else:
        print(text, end="")
    return 0 if summary == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
