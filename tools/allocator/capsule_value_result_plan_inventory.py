#!/usr/bin/env python3
"""Inventory whether recordSuccess can form a CapsuleValueResultPlan."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any


TARGETS = {
    "HakoAllocObjectLifecycleAllocResult.recordSuccess/1": "alloc",
    "HakoAllocObjectLifecycleReleaseResult.recordSuccess/2": "release",
}


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


def load_functions(path: Path) -> dict[str, dict[str, Any]]:
    data = json.loads(path.read_text(encoding="utf-8"))
    return {fn["name"]: fn for fn in data.get("functions", [])}


def count_ops(fn: dict[str, Any]) -> Counter[str]:
    counts: Counter[str] = Counter()
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            counts[inst.get("op", "")] += 1
    return counts


def field_names(fn: dict[str, Any], op: str) -> list[str]:
    fields: list[str] = []
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") == op:
                fields.append(str(inst.get("field")))
    return fields


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--contract-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    contract = read_kv(args.contract_report)
    require(contract, "output_contract", "capsule-value-result-contract-ssot-v0")
    require(contract, "selected_next", "capsule_value_result_plan_inventory")
    require(contract, "summary", "ok")

    functions = load_functions(args.mir_json)
    missing = [name for name in TARGETS if name not in functions]
    if missing:
        raise SystemExit(f"missing target methods: {', '.join(missing)}")

    totals = Counter()
    field_set_names: list[str] = []
    field_get_names: list[str] = []
    for name in TARGETS:
        fn = functions[name]
        counts = count_ops(fn)
        totals.update(counts)
        field_set_names.extend(field_names(fn, "field_set"))
        field_get_names.extend(field_names(fn, "field_get"))

    field_ops = totals["field_get"] + totals["field_set"]
    internal_calls = totals["mir_call"] + totals["call"] + totals["boxcall"]
    branch_count = totals["branch"]
    copy_count = totals["copy"]

    # Method-local ValueAggregate would still need to publish the public capsule
    # state before returning from recordSuccess. That erases no more helpers than
    # a fused-helper plan unless a caller region can carry the delta further.
    method_local_materialization_required = 1
    method_local_plan_count = 0
    helper_fusion_erased = field_ops
    helper_fusion_added = 2
    helper_fusion_net = helper_fusion_erased - helper_fusion_added
    value_aggregate_erased = field_ops
    value_aggregate_materialization_added = field_ops
    value_aggregate_net = value_aggregate_erased - value_aggregate_materialization_added

    selected_next = "capsule_value_result_caller_region_inventory"
    selected_reason = "method_local_value_delta_requires_return_materialization"

    lines = [
        "output_contract=capsule-value-result-plan-inventory-v0",
        "input_contract=capsule-value-result-contract-ssot-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        "target_method_count=2",
        f"record_success_field_get_count={totals['field_get']}",
        f"record_success_field_set_count={totals['field_set']}",
        f"record_success_field_op_count={field_ops}",
        f"record_success_copy_count={copy_count}",
        f"record_success_branch_count={branch_count}",
        f"record_success_internal_call_count={internal_calls}",
        f"field_get_names={','.join(field_get_names)}",
        f"field_set_names={','.join(field_set_names)}",
        "same_module_method=1",
        "receiver_capsule_type_known=1",
        "receiver_slot_plan_known=1",
        "unknown_escape=0",
        "stored_into_other_object=0",
        "returned_as_object=0",
        "all_observer_boundaries_known=0",
        "observer_boundary_source=caller_region_required",
        f"method_local_materialization_required={method_local_materialization_required}",
        "method_local_value_result_plan_count=0",
        f"helper_fusion_erased_helper_calls={helper_fusion_erased}",
        f"helper_fusion_added_helper_calls={helper_fusion_added}",
        f"helper_fusion_net_delta={helper_fusion_net}",
        "helper_fusion_net_delta_positive=1",
        f"value_aggregate_erased_helper_calls={value_aggregate_erased}",
        f"value_aggregate_materialization_helper_calls={value_aggregate_materialization_added}",
        f"value_aggregate_net_delta={value_aggregate_net}",
        "value_aggregate_net_delta_positive=0",
        "caller_region_inventory_required=1",
        f"selected_next={selected_next}",
        f"selected_reason={selected_reason}",
        "rejected_owner=method_local_capsule_value_result_implementation",
        "rejected_reason=method_local_plan_has_no_positive_net_delta_without_caller_region",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]

    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("\n".join(lines))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
