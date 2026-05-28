#!/usr/bin/env python3
"""Build a selected-method MIR typed-field residence plan."""

from __future__ import annotations

import argparse
import json
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any


DEFAULT_METHOD = "HakoAllocPageModel.acquire_usize/1"
SCALAR_DECLARED_TYPES = {"i64", "usize", "u64", "isize", "i8", "i16", "i32", "u8", "u16", "u32"}


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
        kind = declared.get("kind") or "unknown"
        box = declared.get("box_type") or declared.get("box_name") or ""
        return f"{kind}:{box}" if box else str(kind)
    if declared is None:
        return "dynamic_or_missing"
    return "other"


def is_scalar_declared(declared: Any) -> bool:
    return isinstance(declared, str) and declared in SCALAR_DECLARED_TYPES


def is_handle_declared(declared: Any) -> bool:
    return isinstance(declared, dict) and declared.get("kind") == "handle"


def call_has_effects(inst: dict[str, Any]) -> bool:
    call = inst.get("mir_call") or {}
    return bool(call.get("effects") or [])


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--method", default=DEFAULT_METHOD)
    args = parser.parse_args()

    module = load_module(args.mir_json)
    fn = functions_by_name(module).get(args.method)
    if fn is None:
        raise SystemExit(f"method not found: {args.method}")

    field_reads: defaultdict[str, int] = defaultdict(int)
    field_writes: defaultdict[str, int] = defaultdict(int)
    field_kind: dict[str, str] = {}
    rejected_handle: Counter[str] = Counter()
    barriers: Counter[str] = Counter()

    for block in fn.get("blocks") or []:
        for inst in block.get("instructions") or []:
            op = inst.get("op")
            if op in {"field_get", "field_set"}:
                field = str(inst.get("field") or "unknown")
                declared = inst.get("declared_type")
                kind = declared_kind(declared)
                if is_scalar_declared(declared):
                    field_kind.setdefault(field, kind)
                    if op == "field_get":
                        field_reads[field] += 1
                    else:
                        field_writes[field] += 1
                elif is_handle_declared(declared):
                    rejected_handle[f"{field}.{kind}"] += 1
                elif kind == "dynamic_or_missing":
                    barriers["dynamic_slot"] += 1
            elif op == "mir_call" and call_has_effects(inst):
                barriers["unknown_call"] += 1
            elif op == "phi":
                barriers["phi"] += 1
            elif op == "ret":
                barriers["return"] += 1

    scalar_fields = sorted(field_kind)
    readonly_fields = [field for field in scalar_fields if field_writes[field] == 0]
    writeback_fields = [field for field in scalar_fields if field_writes[field] > 0]

    print("output_contract=mir-typed-field-residence-selected-method-plan-v0")
    print("input_contract=mir-typed-field-residence-inventory-v0")
    print(f"selected_method={args.method}")
    print("residence_kind=method_receiver_cache_writeback")
    print("init_policy=helper_load_on_first_use")
    print("writeback_policy=writeback_on_return")
    print(f"scalar_field_count={len(scalar_fields)}")
    print(f"readonly_field_count={len(readonly_fields)}")
    print(f"writeback_field_count={len(writeback_fields)}")
    print(f"rejected_handle_field_count={sum(rejected_handle.values())}")
    print(f"barrier_unknown_call_count={barriers['unknown_call']}")
    print(f"barrier_phi_count={barriers['phi']}")
    print(f"barrier_return_count={barriers['return']}")
    print(f"barrier_dynamic_slot_count={barriers['dynamic_slot']}")
    print(f"helper_load_on_first_use_count={len(scalar_fields)}")
    print(f"writeback_on_return_count={len(writeback_fields)}")
    for idx, field in enumerate(scalar_fields):
        mode = "writeback" if field in writeback_fields else "readonly"
        print(f"field_{idx}_name={field}")
        print(f"field_{idx}_storage={field_kind[field]}")
        print(f"field_{idx}_read_count={field_reads[field]}")
        print(f"field_{idx}_write_count={field_writes[field]}")
        print(f"field_{idx}_residence_mode={mode}")
    for idx, (field, count) in enumerate(sorted(rejected_handle.items())):
        print(f"rejected_handle_{idx}_field={field}")
        print(f"rejected_handle_{idx}_count={count}")
    print("next_step=mir_typed_field_residence_selected_method_keeper")
    print("transform_open=0")
    print("by_name_special_case=0")
    print("winner_claim=0")
    print("replacement_active=0")
    print("hook_installed=0")
    print("global_allocator=0")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
