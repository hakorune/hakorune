#!/usr/bin/env python3
"""Estimate net helper-call erasure for a selected MIR typed-field residence plan."""

from __future__ import annotations

import argparse
import json
from collections import defaultdict
from pathlib import Path
from typing import Any


DEFAULT_METHOD = "HakoAllocPageModel.acquire_usize/1"
SCALAR_DECLARED_TYPES = {"i64", "usize", "u64", "isize", "i8", "i16", "i32", "u8", "u16", "u32"}
HANDLE_DECLARED_KIND = "handle"


def load_module(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def functions_by_name(module: dict[str, Any]) -> dict[str, dict[str, Any]]:
    funcs = module.get("functions") or []
    if isinstance(funcs, dict):
        return funcs
    return {fn.get("name", ""): fn for fn in funcs if fn.get("name")}


def declared_kind(declared: Any) -> str:
    if isinstance(declared, str):
        return declared
    if isinstance(declared, dict):
        return str(declared.get("kind") or "unknown")
    return "unknown"


def is_scalar_declared(declared: Any) -> bool:
    return isinstance(declared, str) and declared in SCALAR_DECLARED_TYPES


def is_handle_declared(declared: Any) -> bool:
    return isinstance(declared, dict) and declared.get("kind") == HANDLE_DECLARED_KIND


def op_is_barrier(inst: dict[str, Any]) -> bool:
    op = inst.get("op")
    if op in {"ret", "jump", "branch", "phi", "call", "boxcall", "externcall"}:
        return True
    if op == "mir_call":
        return True
    return False


def box_key(raw: Any) -> str:
    return str(raw) if isinstance(raw, int) else "dynamic"


def field_key(inst: dict[str, Any]) -> tuple[str, str]:
    return (box_key(inst.get("box")), str(inst.get("field") or "unknown"))


def analyze_block(block: dict[str, Any]) -> dict[str, int]:
    seen_resident: set[tuple[str, str]] = set()
    dirty_fields: set[tuple[str, str]] = set()
    duplicate_get_erasure = 0
    scalar_get = 0
    scalar_set = 0
    handle_reject = 0
    coalesced_set_erasure = 0

    for inst in block.get("instructions") or []:
        op = inst.get("op")
        if op_is_barrier(inst):
            seen_resident.clear()
            dirty_fields.clear()
            continue
        if op == "field_get":
            declared = inst.get("declared_type")
            if is_handle_declared(declared):
                handle_reject += 1
                continue
            if not is_scalar_declared(declared):
                continue
            scalar_get += 1
            key = field_key(inst)
            if key in seen_resident:
                duplicate_get_erasure += 1
            seen_resident.add(key)
        elif op == "field_set":
            declared = inst.get("declared_type")
            if is_handle_declared(declared):
                handle_reject += 1
                continue
            if not is_scalar_declared(declared):
                continue
            scalar_set += 1
            key = field_key(inst)
            if key in dirty_fields:
                coalesced_set_erasure += 1
            dirty_fields.add(key)
            seen_resident.add(key)

    return {
        "scalar_get": scalar_get,
        "scalar_set": scalar_set,
        "handle_reject": handle_reject,
        "duplicate_get_erasure": duplicate_get_erasure,
        "coalesced_set_erasure": coalesced_set_erasure,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    module = load_module(args.mir_json)
    fn = functions_by_name(module).get(args.method)
    if fn is None:
        raise SystemExit(f"method not found: {args.method}")

    totals: defaultdict[str, int] = defaultdict(int)
    block_count = 0
    for block in fn.get("blocks") or []:
        block_count += 1
        report = analyze_block(block)
        for key, value in report.items():
            totals[key] += value

    scalar_get = totals["scalar_get"]
    scalar_set = totals["scalar_set"]
    duplicate_get_erasure = totals["duplicate_get_erasure"]
    coalesced_set_erasure = totals["coalesced_set_erasure"]
    writeback_required = scalar_set - coalesced_set_erasure
    set_replacement_count = scalar_set
    net_helper_call_delta = duplicate_get_erasure + coalesced_set_erasure
    feasible = net_helper_call_delta > 0

    lines = [
        "output_contract=mir-typed-field-residence-erasure-feasibility-v0",
        "input_contract=mir-typed-field-residence-selected-method-plan-v0",
        f"selected_method={args.method}",
        f"block_count={block_count}",
        f"scalar_field_get_count={scalar_get}",
        f"scalar_field_set_count={scalar_set}",
        f"set_replacement_count={set_replacement_count}",
        f"writeback_required_count={writeback_required}",
        f"duplicate_get_erasure_count={duplicate_get_erasure}",
        f"coalesced_set_erasure_count={coalesced_set_erasure}",
        f"net_helper_call_delta={net_helper_call_delta}",
        f"block_local_residence_feasible={1 if feasible else 0}",
        f"rejected_handle_field_count={totals['handle_reject']}",
        "barrier_policy=block_local_only",
        "implementation_recommendation="
        + ("implement_block_local_residence" if feasible else "do_not_implement_block_local_residence"),
        "next_diagnostic="
        + ("selected_method_keeper" if feasible else "cfg_residence_or_runtime_owner_selection"),
        "transform_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
