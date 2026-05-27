#!/usr/bin/env python3
"""Report MIR method shape counts for one selected method."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = json.load(fh)
    if not isinstance(data, dict):
        raise SystemExit("MIR JSON root must be an object")
    return data


def find_function(data: dict[str, Any], selected_method: str) -> dict[str, Any]:
    functions = data.get("functions")
    if not isinstance(functions, list):
        raise SystemExit("MIR JSON missing functions[]")
    exact_matches = [fn for fn in functions if fn.get("name") == selected_method]
    if exact_matches:
        return exact_matches[0]
    suffix_matches = [fn for fn in functions if str(fn.get("name", "")).endswith(selected_method)]
    if len(suffix_matches) == 1:
        return suffix_matches[0]
    if len(suffix_matches) > 1:
        names = ", ".join(str(fn.get("name", "")) for fn in suffix_matches[:5])
        raise SystemExit(f"selected method is ambiguous: {selected_method}: {names}")
    raise SystemExit(f"selected method not found: {selected_method}")


def instructions(function: dict[str, Any]) -> list[dict[str, Any]]:
    out: list[dict[str, Any]] = []
    blocks = function.get("blocks")
    if not isinstance(blocks, list):
        raise SystemExit("selected function missing blocks[]")
    for block in blocks:
        if not isinstance(block, dict):
            continue
        insts = block.get("instructions", [])
        if not isinstance(insts, list):
            continue
        out.extend(inst for inst in insts if isinstance(inst, dict))
    return out


def callee_name(inst: dict[str, Any]) -> str:
    mir_call = inst.get("mir_call")
    if not isinstance(mir_call, dict):
        return ""
    callee = mir_call.get("callee")
    if not isinstance(callee, dict):
        return ""
    return str(callee.get("name", ""))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--method", required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    data = load_json(args.mir_json)
    function = find_function(data, args.method)
    insts = instructions(function)

    call_count = sum(1 for inst in insts if inst.get("op") == "mir_call")
    field_get_count = sum(1 for inst in insts if inst.get("op") == "field_get")
    field_set_count = sum(1 for inst in insts if inst.get("op") == "field_set")
    array_get_call_count = sum(1 for inst in insts if callee_name(inst) == "get")
    array_length_call_count = sum(1 for inst in insts if callee_name(inst) == "length")
    phi_count = sum(1 for inst in insts if inst.get("op") == "phi")
    copy_count = sum(1 for inst in insts if inst.get("op") == "copy")
    branch_count = sum(1 for inst in insts if inst.get("op") == "branch")
    return_count = sum(1 for inst in insts if inst.get("op") == "ret")

    lines = [
        "output_contract=hako-mir-method-shape-v0",
        "input_kind=mir_json",
        f"selected_method={function.get('name', args.method)}",
        f"mir_instruction_count={len(insts)}",
        f"call_count={call_count}",
        f"field_get_count={field_get_count}",
        f"field_set_count={field_set_count}",
        f"array_get_call_count={array_get_call_count}",
        f"array_length_call_count={array_length_call_count}",
        f"phi_count={phi_count}",
        f"copy_count={copy_count}",
        f"branch_count={branch_count}",
        f"return_count={return_count}",
        "summary=ok",
    ]
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
