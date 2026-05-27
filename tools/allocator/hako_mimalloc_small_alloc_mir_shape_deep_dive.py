#!/usr/bin/env python3
"""Inspect objectLifecycleSmallAlloc MIR shape after two non-keepers."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any


DEFAULT_METHOD = "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"


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
    matches = [fn for fn in functions if isinstance(fn, dict) and fn.get("name") == selected_method]
    if matches:
        return matches[0]
    suffix_matches = [
        fn for fn in functions if isinstance(fn, dict) and str(fn.get("name", "")).endswith(selected_method)
    ]
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


def field_name(inst: dict[str, Any]) -> str:
    field = inst.get("field")
    return str(field) if field is not None else ""


def dominant_owner(
    call_count: int,
    field_count: int,
    phi_count: int,
    copy_count: int,
    branch_count: int,
) -> tuple[str, str]:
    phi_copy = phi_count + copy_count
    if phi_copy > call_count + field_count + branch_count:
        return ("phi_copy", "mir_lowering_probe")
    if call_count >= field_count and call_count >= branch_count:
        return ("method_call", "keeper_selection")
    if field_count >= branch_count:
        return ("field_access", "keeper_selection")
    return ("branching", "mir_lowering_probe")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    data = load_json(args.mir_json)
    function = find_function(data, args.method)
    insts = instructions(function)
    ops = Counter(str(inst.get("op", "")) for inst in insts)
    callees = Counter(callee_name(inst) for inst in insts if inst.get("op") == "mir_call")
    fields = Counter(field_name(inst) for inst in insts if inst.get("op") in ("field_get", "field_set"))

    call_count = ops["mir_call"]
    field_count = ops["field_get"] + ops["field_set"]
    phi_count = ops["phi"]
    copy_count = ops["copy"]
    branch_count = ops["branch"]
    owner, next_action = dominant_owner(call_count, field_count, phi_count, copy_count, branch_count)

    lines = [
        "output_contract=hako-mimalloc-small-alloc-mir-shape-deep-dive-v0",
        "input_contract=hako-mimalloc-post-rollback-inline-success-source-mir-refresh-v0",
        f"selected_owner={function.get('name', args.method)}",
        f"mir_instruction_count={len(insts)}",
        f"mir_call_count={call_count}",
        f"mir_field_access_count={field_count}",
        f"mir_phi_count={phi_count}",
        f"mir_copy_count={copy_count}",
        f"mir_branch_count={branch_count}",
        f"dominant_shape_owner={owner}",
        f"next_action={next_action}",
    ]
    for idx, (callee, count) in enumerate(callees.most_common(8)):
        safe_callee = callee if callee else "unknown"
        lines.append(f"top_callee_{idx}={safe_callee}")
        lines.append(f"top_callee_{idx}_count={count}")
    for idx, (field, count) in enumerate(fields.most_common(8)):
        safe_field = field if field else "unknown"
        lines.append(f"top_field_{idx}={safe_field}")
        lines.append(f"top_field_{idx}_count={count}")
    lines.extend(
        [
            "next_diagnostic=small_alloc_phi_copy_lowering_probe",
            "winner_claim=0",
            "replacement_active=0",
            "hook_installed=0",
            "global_allocator=0",
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
