#!/usr/bin/env python3
"""Select typed-object exact-slot RMW fusion candidates."""

from __future__ import annotations

import argparse
from collections import Counter
from pathlib import Path
from typing import Any

from mir_typed_field_direct_op_net_inventory import (
    functions_by_name,
    load_module,
    resolve_box_type,
    storage_bucket,
    typed_plans,
)


DEFAULT_METHOD = "HakoAllocPageModel.acquire_usize/1"
FUSIBLE_STORAGES = {"usize", "u64"}


def build_copy_sources(fn: dict[str, Any]) -> dict[int, int]:
    sources: dict[int, int] = {}
    for block in fn.get("blocks") or []:
        for inst in block.get("instructions") or []:
            if inst.get("op") == "copy" and "dst" in inst and "src" in inst:
                sources[int(inst["dst"])] = int(inst["src"])
    return sources


def build_value_types(fn: dict[str, Any]) -> dict[int, Any]:
    raw = (fn.get("metadata") or {}).get("value_types") or {}
    out: dict[int, Any] = {}
    for key, value in raw.items():
        try:
            out[int(key)] = value
        except (TypeError, ValueError):
            continue
    return out


def resolve_reg(value: Any, copies: dict[int, int]) -> int | None:
    try:
        current = int(value)
    except (TypeError, ValueError):
        return None
    seen: set[int] = set()
    while current not in seen:
        seen.add(current)
        if current not in copies:
            return current
        current = copies[current]
    return current


def same_receiver_field(
    get_inst: dict[str, Any],
    set_inst: dict[str, Any],
    value_types: dict[int, Any],
    copies: dict[int, int],
) -> tuple[str, str] | None:
    get_box = resolve_box_type(get_inst.get("box"), value_types, copies)
    set_box = resolve_box_type(set_inst.get("box"), value_types, copies)
    if not get_box or get_box != set_box:
        return None
    get_field = str(get_inst.get("field") or "")
    set_field = str(set_inst.get("field") or "")
    if not get_field or get_field != set_field:
        return None
    if resolve_reg(get_inst.get("box"), copies) != resolve_reg(set_inst.get("box"), copies):
        return None
    return get_box, get_field


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    args = parser.parse_args()

    module = load_module(args.mir_json)
    fn = functions_by_name(module).get(args.method)
    if fn is None:
        raise SystemExit(f"method not found: {args.method}")

    plans = typed_plans(module)
    copies = build_copy_sources(fn)
    value_types = build_value_types(fn)
    counts: Counter[str] = Counter()
    candidates: list[dict[str, Any]] = []

    for block in fn.get("blocks") or []:
        block_id = int(block.get("id") or 0)
        instructions = list(block.get("instructions") or [])
        gets_by_dst: dict[int, tuple[int, dict[str, Any]]] = {}
        binops_by_dst: dict[int, tuple[int, dict[str, Any], int, str]] = {}
        for index, inst in enumerate(instructions):
            op = inst.get("op")
            if op == "field_get" and "dst" in inst:
                gets_by_dst[int(inst["dst"])] = (index, inst)
            elif op == "binop" and inst.get("operation") == "+" and "dst" in inst:
                lhs = resolve_reg(inst.get("lhs"), copies)
                rhs = resolve_reg(inst.get("rhs"), copies)
                if lhs is not None and lhs in gets_by_dst:
                    binops_by_dst[int(inst["dst"])] = (index, inst, lhs, "lhs")
                elif rhs is not None and rhs in gets_by_dst:
                    binops_by_dst[int(inst["dst"])] = (index, inst, rhs, "rhs")
            elif op == "field_set":
                value = resolve_reg(inst.get("value"), copies)
                if value is None or value not in binops_by_dst:
                    continue
                binop_index, binop_inst, get_dst, get_side = binops_by_dst[value]
                get_index, get_inst = gets_by_dst[get_dst]
                receiver_field = same_receiver_field(get_inst, inst, value_types, copies)
                if receiver_field is None:
                    counts["rejected_receiver_or_field_mismatch_count"] += 1
                    continue
                box_name, field = receiver_field
                field_plan = plans.get(box_name, {}).get(field)
                if field_plan is None:
                    counts["rejected_no_field_plan_count"] += 1
                    continue
                storage = storage_bucket(str(field_plan.get("storage") or "unknown"))
                if storage not in FUSIBLE_STORAGES:
                    counts["rejected_storage_count"] += 1
                    continue
                delta_reg = resolve_reg(binop_inst.get("rhs" if get_side == "lhs" else "lhs"), copies)
                candidates.append(
                    {
                        "block": block_id,
                        "get_index": get_index,
                        "binop_index": binop_index,
                        "set_index": index,
                        "field": f"{box_name}.{field}",
                        "storage": storage,
                        "delta_reg": delta_reg or 0,
                    }
                )
                counts["candidate_count"] += 1
                counts[f"candidate_storage_{storage}_count"] += 1
                counts[f"candidate_field.{box_name}.{field}.{storage}"] += 1

    erased = counts["candidate_count"] * 2
    added = counts["candidate_count"]
    net = erased - added
    selected_next = (
        "typed_object_field_rmw_fusion_keeper"
        if net > 0
        else "typed_object_field_owner_refresh"
    )

    print("output_contract=typed-object-field-rmw-fusion-selection-v0")
    print("input_contract=mir-typed-field-direct-op-selected-method-feasibility-v0")
    print("workload_id=representative-object-lifecycle-small-block-v0")
    print(f"selected_method={args.method}")
    print("selected_owner=typed_object_exact_slot_rmw_fusion")
    print(f"rmw_candidate_count={counts['candidate_count']}")
    print(f"rmw_candidate_usize_count={counts['candidate_storage_usize_count']}")
    print(f"rmw_candidate_u64_count={counts['candidate_storage_u64_count']}")
    print(f"planned_erased_get_set_helper_calls={erased}")
    print(f"planned_added_fused_helper_calls={added}")
    print(f"planned_net_helper_call_delta={net}")
    print(f"planned_net_helper_call_delta_positive={1 if net > 0 else 0}")
    print("runtime_storage_owner_preserved=1")
    print("helper_free_direct_op_rejected=1")
    print("generic_residence_open=0")
    print("source_rewrite=0")
    for idx, candidate in enumerate(candidates):
        print(f"candidate_{idx}_block={candidate['block']}")
        print(f"candidate_{idx}_get_index={candidate['get_index']}")
        print(f"candidate_{idx}_binop_index={candidate['binop_index']}")
        print(f"candidate_{idx}_set_index={candidate['set_index']}")
        print(f"candidate_{idx}_field={candidate['field']}")
        print(f"candidate_{idx}_storage={candidate['storage']}")
        print(f"candidate_{idx}_delta_reg={candidate['delta_reg']}")
    fields = sorted(
        (key.removeprefix("candidate_field."), value)
        for key, value in counts.items()
        if key.startswith("candidate_field.")
    )
    for idx, (field, count) in enumerate(fields):
        print(f"candidate_field_{idx}={field}")
        print(f"candidate_field_{idx}_count={count}")
    print(f"selected_next={selected_next}")
    print("by_name_special_case=0")
    print("winner_claim=0")
    print("replacement_active=0")
    print("hook_installed=0")
    print("global_allocator=0")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
