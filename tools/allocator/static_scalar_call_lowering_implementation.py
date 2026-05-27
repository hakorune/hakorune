#!/usr/bin/env python3
"""Report static-scalar call-lowering evidence for facade reason calls."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


REASON_PREFIX = "HakoAllocObjectLifecycleFacadeReason."
METHODS = [
    "objectLifecycleSmallAlloc",
    "objectLifecycleRecordAlignmentRequest",
    "objectLifecycleSmallAllocAligned",
    "objectLifecycleReleaseDirectCachedPage",
    "objectLifecycleReleaseBlock",
    "objectLifecycleReallocGrowFromPage",
    "objectLifecycleReallocShrink",
    "objectLifecycleReallocGrow",
]


def load_json(path: Path) -> dict[str, Any]:
    data = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        raise SystemExit("MIR JSON root must be an object")
    return data


def method_body(source: str, method: str) -> str:
    marker = f"{method}("
    start = source.find(marker)
    if start < 0:
        raise SystemExit(f"source method not found: {method}")
    brace = source.find("{", start)
    if brace < 0:
        raise SystemExit(f"source method body not found: {method}")
    depth = 0
    for idx in range(brace, len(source)):
        char = source[idx]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return source[brace:idx]
    raise SystemExit(f"unterminated method body: {method}")


def source_reason_count(source: str, method: str) -> int:
    return method_body(source, method).count(REASON_PREFIX)


def find_function(data: dict[str, Any], source_method: str) -> dict[str, Any]:
    symbol = f"HakoAllocObjectLifecycleFacade.{source_method}"
    functions = data.get("functions")
    if not isinstance(functions, list):
        raise SystemExit("MIR JSON missing functions[]")
    matches = [
        fn
        for fn in functions
        if isinstance(fn, dict) and str(fn.get("name", "")).startswith(symbol + "/")
    ]
    if len(matches) != 1:
        raise SystemExit(f"expected one MIR function for {source_method}, found {len(matches)}")
    return matches[0]


def callee_name(inst: dict[str, Any]) -> str:
    mir_call = inst.get("mir_call")
    if not isinstance(mir_call, dict):
        return ""
    callee = mir_call.get("callee")
    if not isinstance(callee, dict):
        return ""
    return str(callee.get("name", ""))


def mir_reason_call_count(function: dict[str, Any]) -> int:
    count = 0
    for block in function.get("blocks", []):
        if not isinstance(block, dict):
            continue
        for inst in block.get("instructions", []):
            if (
                isinstance(inst, dict)
                and inst.get("op") == "mir_call"
                and callee_name(inst).startswith(REASON_PREFIX)
            ):
                count += 1
    return count


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    data = load_json(args.mir_json)
    source = args.source.read_text(encoding="utf-8")
    total_source = 0
    total_mir = 0
    lines = [
        "output_contract=static-scalar-call-lowering-implementation-v0",
        "input_contract=static-scalar-call-lowering-selection-v0",
    ]
    for idx, method in enumerate(METHODS):
        source_count = source_reason_count(source, method)
        mir_count = mir_reason_call_count(find_function(data, method))
        total_source += source_count
        total_mir += mir_count
        lines.append(f"method_{idx}={method}")
        lines.append(f"method_{idx}_source_reason_call_count={source_count}")
        lines.append(f"method_{idx}_mir_reason_call_count={mir_count}")

    lowered = max(total_source - total_mir, 0)
    lines.extend(
        [
            f"source_reason_call_count={total_source}",
            f"remaining_reason_call_count={total_mir}",
            f"lowered_static_scalar_const_count={lowered}",
            "missing_fact_keep_call_count=0",
            "generic_cse=0",
            "whole_box_pure=0",
            "selected_next=post_static_scalar_call_lowering_measurement",
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
