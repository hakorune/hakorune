#!/usr/bin/env python3
"""Inventory facade exact-slot field traffic after callsite owner selection."""

from __future__ import annotations

import argparse
import json
import re
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any


CALLER_RE = re.compile(r"^\s+(?:\|--|--)([0-9]+(?:\.[0-9]+)?)%--(.+?)\s*$")
TOP_RE = re.compile(r"^\s*([0-9]+(?:\.[0-9]+)?)%\s+\S+\s+\S+\s+\[\.\]\s+(.+?)\s*$")


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


def box_type_name(value_type: Any) -> str:
    if isinstance(value_type, dict):
        return str(value_type.get("box_type", "unknown"))
    return str(value_type)


def field_family(receiver_type: str) -> str:
    if receiver_type == "HakoAllocObjectLifecycleFacade":
        return "facade_receiver_state"
    if receiver_type == "HakoAllocPageModel":
        return "page_model_bridge"
    if receiver_type == "HakoAllocObjectLifecyclePageQueue":
        return "page_queue_bridge"
    if receiver_type == "HakoAllocObjectLifecycleAllocResult":
        return "alloc_result_capsule"
    if receiver_type == "HakoAllocObjectLifecycleReleaseResult":
        return "release_result_capsule"
    if receiver_type.startswith("HakoAllocObjectLifecycle") and receiver_type.endswith("Result"):
        return "temporary_status_result"
    return "unknown"


def parse_facade_callers(perf_report: Path) -> dict[str, float]:
    callers: dict[str, float] = defaultdict(float)
    current_exact_helper = False
    for line in perf_report.read_text(encoding="utf-8", errors="replace").splitlines():
        top = TOP_RE.match(line)
        if top:
            symbol = top.group(2).strip()
            current_exact_helper = (
                "nyash.object.exact_slot_" in symbol
                and "nyash.object.exact_slot_rmw_" not in symbol
            )
            continue
        if not current_exact_helper:
            continue
        caller = CALLER_RE.match(line)
        if not caller:
            continue
        pct = float(caller.group(1))
        symbol = caller.group(2).strip()
        if symbol.startswith("HakoAllocObjectLifecycleFacade."):
            callers[symbol] += pct
    return dict(callers)


def load_functions(mir_json: Path) -> dict[str, dict[str, Any]]:
    data = json.loads(mir_json.read_text(encoding="utf-8"))
    return {fn["name"]: fn for fn in data.get("functions", [])}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--perf-report", type=Path, required=True)
    parser.add_argument("--owner-selection-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    owner = read_kv(args.owner_selection_report)
    require(owner, "output_contract", "typed-object-exact-slot-callsite-owner-selection-v0")
    require(owner, "selected_owner", "object_lifecycle_facade_exact_slot_field_inventory")
    require(owner, "summary", "ok")

    facade_callers = parse_facade_callers(args.perf_report)
    if not facade_callers:
        raise SystemExit("no facade exact-slot callsites found")

    functions = load_functions(args.mir_json)
    field_ops: list[tuple[str, int, str, str, str]] = []
    family_counts: Counter[str] = Counter()
    op_counts: Counter[str] = Counter()
    method_counts: Counter[str] = Counter()
    same_block_get_set = 0
    repeated_get = 0
    write_only = 0
    read_only = 0

    for method in sorted(facade_callers):
        fn = functions.get(method)
        if fn is None:
            continue
        value_types = fn.get("metadata", {}).get("value_types", {})
        seen_get: Counter[tuple[int, str, str]] = Counter()
        seen_set: Counter[tuple[int, str, str]] = Counter()
        method_gets: Counter[tuple[str, str]] = Counter()
        method_sets: Counter[tuple[str, str]] = Counter()
        for block in fn.get("blocks", []):
            block_id = int(block.get("id", -1))
            for ins in block.get("instructions", []):
                if ins.get("op") not in {"field_get", "field_set"}:
                    continue
                receiver_type = box_type_name(value_types.get(str(ins.get("box"))))
                family = field_family(receiver_type)
                field = str(ins.get("field"))
                op = str(ins.get("op"))
                field_ops.append((method, block_id, op, receiver_type, field))
                family_counts[family] += 1
                op_counts[op] += 1
                method_counts[method] += 1
                key = (block_id, receiver_type, field)
                method_key = (receiver_type, field)
                if op == "field_get":
                    seen_get[key] += 1
                    method_gets[method_key] += 1
                else:
                    seen_set[key] += 1
                    method_sets[method_key] += 1
        same_block_get_set += sum(min(seen_get[key], seen_set[key]) for key in set(seen_get) & set(seen_set))
        repeated_get += sum(count - 1 for count in method_gets.values() if count > 1)
        write_only += sum(1 for key in method_sets if key not in method_gets)
        read_only += sum(1 for key in method_gets if key not in method_sets)

    positive_net_cache_candidates = same_block_get_set + repeated_get
    top_method, top_method_pct = max(facade_callers.items(), key=lambda item: item[1])
    dominant_family, dominant_family_count = family_counts.most_common(1)[0]

    if dominant_family in {"alloc_result_capsule", "release_result_capsule"}:
        selected_next = "result_capsule_shape_owner_selection"
    elif positive_net_cache_candidates > 0:
        selected_next = "facade_field_owner_selection"
    else:
        selected_next = "post_facade_inventory_owner_refresh"

    lines = [
        "output_contract=object-lifecycle-facade-exact-slot-field-inventory-v0",
        "input_contract=typed-object-exact-slot-callsite-owner-selection-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        "target_family=object_lifecycle_facade",
        f"target_family_pct={owner.get('dominant_family_pct', '18.52')}",
        f"facade_method_count={len(facade_callers)}",
        f"facade_exact_slot_get_count={op_counts['field_get']}",
        f"facade_exact_slot_set_count={op_counts['field_set']}",
        f"facade_exact_slot_field_op_count={len(field_ops)}",
        f"top_facade_method={top_method}",
        f"top_facade_method_pct={top_method_pct:.2f}",
        f"dominant_field_family={dominant_family}",
        f"dominant_field_family_count={dominant_family_count}",
        f"field_family.facade_receiver_state_count={family_counts['facade_receiver_state']}",
        f"field_family.page_model_bridge_count={family_counts['page_model_bridge']}",
        f"field_family.page_queue_bridge_count={family_counts['page_queue_bridge']}",
        f"field_family.alloc_result_capsule_count={family_counts['alloc_result_capsule']}",
        f"field_family.release_result_capsule_count={family_counts['release_result_capsule']}",
        f"field_family.temporary_status_result_count={family_counts['temporary_status_result']}",
        f"field_family.unknown_count={family_counts['unknown']}",
        f"pattern.same_block_get_set_count={same_block_get_set}",
        f"pattern.same_receiver_repeated_get_count={repeated_get}",
        f"pattern.write_only_field_count={write_only}",
        f"pattern.read_only_field_count={read_only}",
        f"pattern.positive_net_cache_candidate_count={positive_net_cache_candidates}",
        f"selected_next={selected_next}",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for idx, (method, pct) in enumerate(sorted(facade_callers.items(), key=lambda item: item[1], reverse=True)):
        lines.append(f"facade_method_{idx}_symbol={method}")
        lines.append(f"facade_method_{idx}_pct={pct:.2f}")
        lines.append(f"facade_method_{idx}_field_op_count={method_counts[method]}")
    for idx, (method, block_id, op, receiver_type, field) in enumerate(field_ops[:20]):
        lines.append(f"field_op_{idx}_method={method}")
        lines.append(f"field_op_{idx}_block={block_id}")
        lines.append(f"field_op_{idx}_op={op}")
        lines.append(f"field_op_{idx}_receiver={receiver_type}")
        lines.append(f"field_op_{idx}_field={field}")
    lines.append("summary=ok")

    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("\n".join(lines))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
