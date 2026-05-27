#!/usr/bin/env python3
"""Report env method fallback single-evaluation fixture MIR call counts."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


DEFAULT_METHOD = "EnvMethodProbe.run/0"
DEFAULT_NESTED_SYMBOL = "EnvArgSide.tick/0"


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = json.load(fh)
    if not isinstance(data, dict):
        raise SystemExit("MIR JSON root must be an object")
    return data


def find_function(data: dict[str, Any], method: str) -> dict[str, Any]:
    functions = data.get("functions")
    if not isinstance(functions, list):
        raise SystemExit("MIR JSON missing functions[]")
    matches = [fn for fn in functions if isinstance(fn, dict) and fn.get("name") == method]
    if len(matches) != 1:
        raise SystemExit(f"expected one function {method}, found {len(matches)}")
    return matches[0]


def callee_name(inst: dict[str, Any]) -> str:
    mir_call = inst.get("mir_call")
    if not isinstance(mir_call, dict):
        return ""
    callee = mir_call.get("callee")
    if not isinstance(callee, dict):
        return ""
    name = str(callee.get("name", ""))
    if "/" in name:
        return name
    args = mir_call.get("args", [])
    argc = len(args) if isinstance(args, list) else 0
    return f"{name}/{argc}"


def call_count(function: dict[str, Any], symbol: str) -> int:
    count = 0
    for block in function.get("blocks", []):
        if not isinstance(block, dict):
            continue
        for inst in block.get("instructions", []):
            if isinstance(inst, dict) and inst.get("op") == "mir_call" and callee_name(inst) == symbol:
                count += 1
    return count


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--nested-symbol", default=DEFAULT_NESTED_SYMBOL)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    function = find_function(load_json(args.mir_json), args.method)
    actual = call_count(function, args.nested_symbol)
    selected_next = (
        "mir_builder_env_method_single_eval_owner_fix"
        if actual != 1
        else "post_single_eval_fixes_measurement"
    )

    lines = [
        "output_contract=mir-builder-env-method-single-eval-fixture-v0",
        "input_contract=mir-builder-nested-field-single-eval-owner-fix-v0",
        "fixture=env_method_single_eval",
        f"selected_method={function.get('name', args.method)}",
        f"nested_call_symbol={args.nested_symbol}",
        "expected_nested_call_count=1",
        f"actual_nested_call_count={actual}",
        f"selected_next={selected_next}",
        "winner_claim=0",
        "summary=ok",
    ]
    text = "\n".join(lines) + "\n"
    if args.out is None:
        print(text, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
