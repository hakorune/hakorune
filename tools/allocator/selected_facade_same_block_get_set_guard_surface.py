#!/usr/bin/env python3
"""Freeze selected facade same-block field get/add/set fusion candidates."""

from __future__ import annotations

import argparse
import sys
from collections import Counter
from pathlib import Path
from typing import Any

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

from mir_typed_object_field_rmw_fusion_selection import (  # noqa: E402
    build_copy_sources,
    build_value_types,
    reg_use_count,
    resolve_reg,
    same_receiver_field,
)
from mir_typed_field_direct_op_net_inventory import (  # noqa: E402
    functions_by_name,
    load_module,
    storage_bucket,
    typed_plans,
)


TARGET_PREFIX = "HakoAllocObjectLifecycleFacade."
FUSIBLE_STORAGES = {"i64", "usize", "u64"}


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def require(values: dict[str, str], key: str, expected: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{key} expected {expected!r}, got {actual!r}")


def collect_candidates(fn: dict[str, Any], plans: dict[str, dict[str, Any]]) -> list[dict[str, Any]]:
    copies = build_copy_sources(fn)
    value_types = build_value_types(fn)
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
                if reg_use_count(instructions, get_dst) != 1:
                    continue
                receiver_field = same_receiver_field(get_inst, inst, value_types, copies)
                if receiver_field is None:
                    continue
                box_name, field = receiver_field
                field_plan = plans.get(box_name, {}).get(field)
                if field_plan is None:
                    continue
                storage = storage_bucket(str(field_plan.get("storage") or "unknown"))
                if storage not in FUSIBLE_STORAGES:
                    continue
                delta_reg = resolve_reg(binop_inst.get("rhs" if get_side == "lhs" else "lhs"), copies)
                candidates.append(
                    {
                        "method": fn["name"],
                        "block": block_id,
                        "get_index": get_index,
                        "binop_index": binop_index,
                        "set_index": index,
                        "field": f"{box_name}.{field}",
                        "storage": storage,
                        "delta_reg": delta_reg or 0,
                    }
                )
    return candidates


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--owner-selection-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    owner = read_kv(args.owner_selection_report)
    require(owner, "output_contract", "object-lifecycle-facade-field-owner-selection-v0")
    require(owner, "selected_owner", "selected_facade_same_block_get_set_fusion")
    require(owner, "summary", "ok")

    module = load_module(args.mir_json)
    plans = typed_plans(module)
    by_name = functions_by_name(module)
    candidates: list[dict[str, Any]] = []
    for name, fn in sorted(by_name.items()):
        if name.startswith(TARGET_PREFIX):
            candidates.extend(collect_candidates(fn, plans))

    counts: Counter[str] = Counter()
    for candidate in candidates:
        counts[f"storage_{candidate['storage']}"] += 1
        counts[f"field.{candidate['field']}"] += 1
        counts[f"method.{candidate['method']}"] += 1

    erased = len(candidates) * 2
    added = len(candidates)
    net = erased - added

    lines = [
        "output_contract=selected-facade-same-block-get-set-guard-surface-v0",
        "input_contract=object-lifecycle-facade-field-owner-selection-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        "selected_owner=selected_facade_same_block_get_set_fusion",
        "target_family=object_lifecycle_facade",
        f"candidate_count={len(candidates)}",
        f"candidate_i64_count={counts['storage_i64']}",
        f"candidate_usize_count={counts['storage_usize']}",
        f"candidate_u64_count={counts['storage_u64']}",
        f"planned_erased_get_set_helper_calls={erased}",
        f"planned_added_fused_helper_calls={added}",
        f"planned_net_helper_call_delta={net}",
        f"planned_net_helper_call_delta_positive={1 if net > 0 else 0}",
        "runtime_storage_owner_preserved=1",
        "helper_free_direct_op_rejected=1",
        "generic_residence_open=0",
        "source_rewrite=0",
    ]
    for idx, candidate in enumerate(candidates):
        lines.append(f"candidate_{idx}_method={candidate['method']}")
        lines.append(f"candidate_{idx}_block={candidate['block']}")
        lines.append(f"candidate_{idx}_get_index={candidate['get_index']}")
        lines.append(f"candidate_{idx}_binop_index={candidate['binop_index']}")
        lines.append(f"candidate_{idx}_set_index={candidate['set_index']}")
        lines.append(f"candidate_{idx}_field={candidate['field']}")
        lines.append(f"candidate_{idx}_storage={candidate['storage']}")
        lines.append(f"candidate_{idx}_delta_reg={candidate['delta_reg']}")
    for idx, (key, count) in enumerate(
        sorted((key.removeprefix("field."), value) for key, value in counts.items() if key.startswith("field."))
    ):
        lines.append(f"candidate_field_{idx}={key}")
        lines.append(f"candidate_field_{idx}_count={count}")
    for idx, (key, count) in enumerate(
        sorted((key.removeprefix("method."), value) for key, value in counts.items() if key.startswith("method."))
    ):
        lines.append(f"candidate_method_{idx}={key}")
        lines.append(f"candidate_method_{idx}_count={count}")
    lines.extend(
        [
            "selected_next=selected_facade_same_block_get_set_keeper",
            "by_name_special_case=0",
            "winner_claim=0",
            "replacement_active=0",
            "hook_installed=0",
            "global_allocator=0",
            "summary=ok",
        ]
    )

    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("\n".join(lines))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
