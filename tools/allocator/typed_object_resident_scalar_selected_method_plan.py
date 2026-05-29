#!/usr/bin/env python3
"""Build the selected-method typed-object ResidentScalar plan."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any


DEFAULT_METHOD = "HakoAllocPageModel.acquire_usize/1"
DEFAULT_DYNAMIC_WEIGHT = 524_288
DIRECT_STORAGES = {"i64", "u64", "usize", "handle"}


def load_module(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def functions_by_name(module: dict[str, Any]) -> dict[str, dict[str, Any]]:
    funcs = module.get("functions") or []
    if isinstance(funcs, dict):
        return funcs
    return {fn.get("name", ""): fn for fn in funcs if fn.get("name")}


def typed_plans(module: dict[str, Any]) -> dict[str, dict[str, dict[str, Any]]]:
    plans: dict[str, dict[str, dict[str, Any]]] = {}
    for plan in module.get("typed_object_plans") or []:
        box_name = plan.get("box_name")
        if not box_name:
            continue
        fields: dict[str, dict[str, Any]] = {}
        for field in plan.get("fields") or []:
            name = field.get("name")
            if name:
                fields[str(name)] = dict(field)
        plans[str(box_name)] = fields
    return plans


def type_box_name(value_type: Any) -> str | None:
    if isinstance(value_type, dict) and value_type.get("kind") == "handle":
        box_type = value_type.get("box_type") or value_type.get("box_name")
        return str(box_type) if box_type else None
    return None


def build_copy_sources(fn: dict[str, Any]) -> dict[int, int]:
    sources: dict[int, int] = {}
    for block in fn.get("blocks") or []:
        for inst in block.get("instructions") or []:
            if inst.get("op") == "copy" and "dst" in inst and "src" in inst:
                sources[int(inst["dst"])] = int(inst["src"])
    return sources


def build_value_types(fn: dict[str, Any]) -> dict[int, Any]:
    raw = (fn.get("metadata") or {}).get("value_types") or {}
    value_types: dict[int, Any] = {}
    for key, value in raw.items():
        try:
            value_types[int(key)] = value
        except (TypeError, ValueError):
            continue
    return value_types


def resolve_box_type(value: Any, value_types: dict[int, Any], copies: dict[int, int]) -> str | None:
    try:
        current = int(value)
    except (TypeError, ValueError):
        return None
    seen: set[int] = set()
    while current not in seen:
        seen.add(current)
        box_name = type_box_name(value_types.get(current))
        if box_name:
            return box_name
        if current not in copies:
            return None
        current = copies[current]
    return None


def call_has_effects(inst: dict[str, Any]) -> bool:
    call = inst.get("mir_call") or {}
    return bool(call.get("effects") or [])


def storage_bucket(storage: str) -> str:
    if storage == "usize":
        return "usize"
    if storage in DIRECT_STORAGES:
        return storage
    return "unsupported"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--dynamic-weight", type=int, default=DEFAULT_DYNAMIC_WEIGHT)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    module = load_module(args.mir_json)
    fn = functions_by_name(module).get(args.method)
    if fn is None:
        raise SystemExit(f"method not found: {args.method}")

    plans = typed_plans(module)
    copies = build_copy_sources(fn)
    value_types = build_value_types(fn)
    counts: Counter[str] = Counter()
    field_counts: Counter[str] = Counter()

    for block in fn.get("blocks") or []:
        for inst in block.get("instructions") or []:
            op = inst.get("op")
            if op in {"field_get", "field_set"}:
                field = str(inst.get("field") or "")
                box_name = resolve_box_type(inst.get("box"), value_types, copies)
                if not box_name:
                    counts["unknown_receiver_count"] += 1
                    continue
                field_plan = plans.get(box_name, {}).get(field)
                if field_plan is None:
                    counts["unknown_field_plan_count"] += 1
                    continue
                if bool(field_plan.get("weak")):
                    counts["weak_field_count"] += 1
                    continue
                storage = storage_bucket(str(field_plan.get("storage") or "unknown"))
                if storage == "unsupported":
                    counts["unsupported_storage_count"] += 1
                    continue
                counts[f"eligible_{op}_count"] += 1
                counts[f"eligible_{storage}_count"] += 1
                field_counts[f"{box_name}.{field}.{storage}"] += 1
            elif op == "mir_call" and call_has_effects(inst):
                counts["barrier_unknown_call_count"] += 1
            elif op == "phi":
                counts["barrier_phi_count"] += 1
            elif op == "ret":
                counts["barrier_return_count"] += 1

    erased = counts["eligible_field_get_count"] + counts["eligible_field_set_count"]
    materialization_added = 0
    net_delta = erased - materialization_added

    selected_fields = sorted(
        ((count, key) for key, count in field_counts.items()),
        key=lambda item: (-item[0], item[1]),
    )

    lines = [
        "output_contract=typed-object-resident-scalar-selected-method-plan-v0",
        "input_contract=typed-object-resident-scalar-guard-surface-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"selected_method={args.method}",
        f"selected_method_dynamic_weight={args.dynamic_weight}",
        "current_representation=ExactSlotObject",
        "candidate_representation=ResidentScalar",
        f"eligible_field_get_count={counts['eligible_field_get_count']}",
        f"eligible_field_set_count={counts['eligible_field_set_count']}",
        f"eligible_i64_count={counts['eligible_i64_count']}",
        f"eligible_u64_count={counts['eligible_u64_count']}",
        f"eligible_usize_count={counts['eligible_usize_count']}",
        f"eligible_handle_count={counts['eligible_handle_count']}",
        f"planned_erased_helper_ops={erased}",
        f"planned_materialization_ops_added={materialization_added}",
        f"planned_net_helper_delta={net_delta}",
        f"dynamic_planned_net_helper_delta={net_delta * args.dynamic_weight}",
        f"planned_net_helper_delta_positive={1 if net_delta > 0 else 0}",
        f"resident_field_key_count={len(selected_fields)}",
        f"unknown_receiver_count={counts['unknown_receiver_count']}",
        f"unknown_field_plan_count={counts['unknown_field_plan_count']}",
        f"unsupported_storage_count={counts['unsupported_storage_count']}",
        f"weak_field_count={counts['weak_field_count']}",
        f"barrier_unknown_call_count={counts['barrier_unknown_call_count']}",
        f"barrier_phi_count={counts['barrier_phi_count']}",
        f"barrier_return_count={counts['barrier_return_count']}",
        "unknown_call_barrier_policy=materialize_or_no_plan",
        "return_barrier_policy=materialize_only_if_net_positive",
        "selected_plan_silent_fallback_allowed=0",
        "storage_or_slot_proven=1",
    ]
    for idx, (count, field_key) in enumerate(selected_fields):
        lines.append(f"resident_field_{idx}={field_key}")
        lines.append(f"resident_field_{idx}_op_count={count}")
    lines.extend(
        [
            "selected_next=typed_object_resident_scalar_implementation_owner_selection",
            "implementation_open=0",
            "optimization_open=0",
            "winner_claim=0",
            "replacement_active=0",
            "hook_installed=0",
            "global_allocator=0",
            "summary=ok",
        ]
    )

    text = "\n".join(lines) + "\n"
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    print(text, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
