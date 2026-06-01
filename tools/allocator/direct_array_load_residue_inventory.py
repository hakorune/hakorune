#!/usr/bin/env python3
"""Inventory residual DirectArray i64 load helper calls in lowered IR."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any


DEFINE_RE = re.compile(r'^define\s+\S+\s+@"([^"]+)"\(')
SLOT_LOAD_RE = re.compile(r"call\s+i64\s+@nyash\.array\.slot_load_hi\(")


def function_metadata(mir_json: Path) -> dict[str, dict[str, Any]]:
    data = json.loads(mir_json.read_text(encoding="utf-8"))
    out: dict[str, dict[str, Any]] = {}
    for function in data.get("functions", []):
        name = function.get("name")
        if not isinstance(name, str):
            continue
        metadata = function.get("metadata") or {}
        out[name] = {
            "direct_array_access_plans": metadata.get("direct_array_access_plans") or [],
            "direct_array_extent_facts": metadata.get("direct_array_extent_facts") or [],
            "range_index_facts": metadata.get("range_index_facts") or [],
            "region_stability_facts": metadata.get("region_stability_facts") or [],
        }
    return out


def slot_load_calls(llvm_ir: Path) -> list[tuple[str, int]]:
    calls: list[tuple[str, int]] = []
    current = "<unknown>"
    for line_no, line in enumerate(llvm_ir.read_text(encoding="utf-8", errors="replace").splitlines(), 1):
        match = DEFINE_RE.match(line)
        if match:
            current = match.group(1)
            continue
        if SLOT_LOAD_RE.search(line):
            calls.append((current, line_no))
    return calls


def classify(meta: dict[str, Any]) -> str:
    load_plans = [plan for plan in meta["direct_array_access_plans"] if plan.get("op") == "load"]
    if load_plans:
        return "plan_exists_but_lowering_fell_back"
    if not meta["range_index_facts"]:
        return "missing_range_index_fact"
    if not meta["direct_array_extent_facts"]:
        return "missing_direct_array_extent_fact"
    if not meta["region_stability_facts"]:
        return "missing_region_stability_fact"
    return "missing_direct_array_load_plan"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--llvm-ir", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    metadata_by_function = function_metadata(args.mir_json)
    calls = slot_load_calls(args.llvm_ir)
    executable_calls = [(name, line_no) for name, line_no in calls if name != "<unknown>"]
    by_function: dict[str, list[int]] = {}
    for name, line_no in executable_calls:
        by_function.setdefault(name, []).append(line_no)

    lines = [
        "output_contract=direct-array-load-residue-inventory-v0",
        "input_contract=direct-exact-lowered-ir-and-mir-metadata-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"slot_load_hi_executable_call_count={len(executable_calls)}",
        f"slot_load_hi_function_count={len(by_function)}",
    ]

    missing_range = 0
    missing_extent = 0
    missing_stability = 0
    plan_exists_fallback = 0
    for idx, (name, lines_for_function) in enumerate(sorted(by_function.items())):
        meta = metadata_by_function.get(name, {
            "direct_array_access_plans": [],
            "direct_array_extent_facts": [],
            "range_index_facts": [],
            "region_stability_facts": [],
        })
        reason = classify(meta)
        missing_range += int(reason == "missing_range_index_fact")
        missing_extent += int(reason == "missing_direct_array_extent_fact")
        missing_stability += int(reason == "missing_region_stability_fact")
        plan_exists_fallback += int(reason == "plan_exists_but_lowering_fell_back")
        lines.extend(
            [
                f"residue_{idx}_function={name}",
                f"residue_{idx}_call_count={len(lines_for_function)}",
                f"residue_{idx}_llvm_lines={','.join(str(item) for item in lines_for_function)}",
                f"residue_{idx}_range_index_fact_count={len(meta['range_index_facts'])}",
                f"residue_{idx}_direct_array_extent_fact_count={len(meta['direct_array_extent_facts'])}",
                f"residue_{idx}_region_stability_fact_count={len(meta['region_stability_facts'])}",
                f"residue_{idx}_direct_array_load_plan_count={len([plan for plan in meta['direct_array_access_plans'] if plan.get('op') == 'load'])}",
                f"residue_{idx}_classification={reason}",
            ]
        )

    if missing_range > 0 and missing_extent > 0:
        next_owner = "direct_array_load_fact_gap_split"
    elif missing_extent > 0 and missing_range == 0:
        next_owner = "direct_array_load_extent_and_stability_facts"
    elif missing_range > 0:
        next_owner = "direct_array_load_range_index_fact_producer"
    elif plan_exists_fallback > 0:
        next_owner = "direct_array_load_lowering_consumer"
    else:
        next_owner = "direct_array_load_plan_producer"

    lines.extend(
        [
            f"missing_range_index_fact_function_count={missing_range}",
            f"missing_direct_array_extent_fact_function_count={missing_extent}",
            f"missing_region_stability_fact_function_count={missing_stability}",
            f"plan_exists_but_lowering_fell_back_function_count={plan_exists_fallback}",
            f"selected_next_owner={next_owner}",
            "new_fastpath_lane_open=0",
            "source_hand_expansion_allowed=0",
            "direct_block_syntax_added=0",
            "mixed_base_inline_widening_allowed=0",
            "summary=ok",
        ]
    )

    text = "\n".join(lines) + "\n"
    if args.out is None:
        print(text, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
