#!/usr/bin/env python3
"""Inventory page-model hotpath exact-slot callsites against current MIR shape."""

from __future__ import annotations

import argparse
import json
import re
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any


TOP_RE = re.compile(r"^\s*([0-9]+(?:\.[0-9]+)?)%\s+\S+\s+\S+\s+\[\.\]\s+(.+?)\s*$")
CALLER_RE = re.compile(r"^\s+(?:\|--|--)([0-9]+(?:\.[0-9]+)?)%--(.+?)\s*$")


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


def is_exact_get_set(symbol: str) -> bool:
    return "nyash.object.exact_slot_" in symbol and "nyash.object.exact_slot_rmw_" not in symbol


def parse_page_model_callers(path: Path) -> dict[str, float]:
    callers: dict[str, float] = defaultdict(float)
    current_exact_helper = False
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        top = TOP_RE.match(line)
        if top:
            current_exact_helper = is_exact_get_set(top.group(2).strip())
            continue
        if not current_exact_helper:
            continue
        caller = CALLER_RE.match(line)
        if not caller:
            continue
        symbol = caller.group(2).strip()
        if symbol.startswith("HakoAllocPageModel."):
            callers[symbol] += float(caller.group(1))
    return dict(callers)


def load_functions(path: Path) -> dict[str, dict[str, Any]]:
    data = json.loads(path.read_text(encoding="utf-8"))
    functions = data.get("functions")
    if not isinstance(functions, list):
        raise SystemExit("MIR JSON missing functions[]")
    return {str(fn.get("name", "")): fn for fn in functions if isinstance(fn, dict)}


def instruction_counter(fn: dict[str, Any]) -> Counter[str]:
    out: Counter[str] = Counter()
    for block in fn.get("blocks", []):
        if not isinstance(block, dict):
            continue
        for ins in block.get("instructions", []):
            if isinstance(ins, dict):
                out[str(ins.get("op", ""))] += 1
    return out


def dominant_shape_owner(ops: Counter[str]) -> str:
    field_ops = ops["field_get"] + ops["field_set"]
    control_ops = ops["branch"] + ops["phi"]
    if field_ops >= ops["copy"] and field_ops >= control_ops:
        return "exact_slot_field_traffic"
    if ops["copy"] >= control_ops:
        return "copy_materialization"
    return "control_shape"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--perf-report", type=Path, required=True)
    parser.add_argument("--owner-selection-report", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    owner = read_kv(args.owner_selection_report)
    output_contract = owner.get("output_contract")
    if output_contract not in {
        "weighted-exact-slot-owner-selection-v0",
        "weighted-exact-slot-owner-selection-after-result-capsule-reset-v0",
    }:
        raise SystemExit(f"unsupported owner selection contract: {output_contract!r}")
    selected_owner = owner.get("selected_owner")
    if selected_owner not in {
        "page_model_hotpath_ir_shape_diff_inventory",
        "page_model_hotpath_ir_shape_diff_refresh",
    }:
        raise SystemExit(f"unsupported selected owner: {selected_owner!r}")
    require(owner, "summary", "ok")

    page_model_callers = parse_page_model_callers(args.perf_report)
    if not page_model_callers:
        raise SystemExit("no page-model exact-slot callsites found")
    functions = load_functions(args.mir_json)

    method_rows: list[tuple[str, float, Counter[str]]] = []
    missing_methods: list[str] = []
    totals: Counter[str] = Counter()
    for method, pct in sorted(page_model_callers.items(), key=lambda item: item[1], reverse=True):
        fn = functions.get(method)
        if fn is None:
            missing_methods.append(method)
            continue
        ops = instruction_counter(fn)
        totals.update(ops)
        method_rows.append((method, pct, ops))

    if not method_rows:
        raise SystemExit("no page-model perf callsites matched MIR methods")

    selected_method, selected_pct, selected_ops = method_rows[0]
    field_ops = totals["field_get"] + totals["field_set"]
    selected_field_ops = selected_ops["field_get"] + selected_ops["field_set"]
    selected_owner = dominant_shape_owner(selected_ops)

    lines = [
        "output_contract=page-model-hotpath-ir-shape-diff-inventory-v0",
        f"input_contract={output_contract}",
        "workload_id=representative-object-lifecycle-small-block-v0",
        "target_family=page_model_hotpath",
        f"target_family_pct={owner.get('top_unblocked_family_pct', '0.00')}",
        f"page_model_method_count={len(method_rows)}",
        f"missing_page_model_method_count={len(missing_methods)}",
        f"page_model_exact_slot_perf_pct={sum(page_model_callers.values()):.2f}",
        f"page_model_mir_field_get_count={totals['field_get']}",
        f"page_model_mir_field_set_count={totals['field_set']}",
        f"page_model_mir_field_op_count={field_ops}",
        f"page_model_mir_copy_count={totals['copy']}",
        f"page_model_mir_call_count={totals['mir_call']}",
        f"page_model_mir_phi_count={totals['phi']}",
        f"selected_method={selected_method}",
        f"selected_method_pct={selected_pct:.2f}",
        f"selected_method_field_get_count={selected_ops['field_get']}",
        f"selected_method_field_set_count={selected_ops['field_set']}",
        f"selected_method_field_op_count={selected_field_ops}",
        f"selected_method_copy_count={selected_ops['copy']}",
        f"selected_method_call_count={selected_ops['mir_call']}",
        f"selected_method_phi_count={selected_ops['phi']}",
        f"selected_method_shape_owner={selected_owner}",
        "recent_selected_method_rmw_keeper_already_applied=1",
        "direct_op_previous_rejected=1",
        "page_queue_recent_nonkeeper_retry_closed=1",
        "ir_shape_diff_inventory_only=1",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for idx, (method, pct, ops) in enumerate(method_rows[:10]):
        lines.append(f"method_{idx}_symbol={method}")
        lines.append(f"method_{idx}_pct={pct:.2f}")
        lines.append(f"method_{idx}_field_get_count={ops['field_get']}")
        lines.append(f"method_{idx}_field_set_count={ops['field_set']}")
        lines.append(f"method_{idx}_copy_count={ops['copy']}")
        lines.append(f"method_{idx}_call_count={ops['mir_call']}")
    for idx, method in enumerate(missing_methods[:10]):
        lines.append(f"missing_method_{idx}={method}")
    lines.extend(
        [
            "selected_next=page_model_hotpath_shape_owner_selection",
            "summary=ok",
        ]
    )

    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("\n".join(lines))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
