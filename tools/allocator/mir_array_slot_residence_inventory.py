#!/usr/bin/env python3
"""Inventory ArrayBox slot residence candidates from MIR JSON."""

from __future__ import annotations

import argparse
import json
import subprocess
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
APP = ROOT / "apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"


@dataclass
class FunctionInventory:
    name: str
    array_get_count: int = 0
    array_set_count: int = 0
    phi_count: int = 0
    unknown_call_count: int = 0
    helper_barrier_call_count: int = 0

    @property
    def erased_get_set_helper_calls(self) -> int:
        return self.array_get_count + self.array_set_count

    @property
    def added_guard_helper_calls(self) -> int:
        return 1 if self.erased_get_set_helper_calls else 0

    @property
    def added_writeback_helper_calls(self) -> int:
        return 0

    @property
    def net_helper_call_delta(self) -> int:
        return (
            self.erased_get_set_helper_calls
            - self.added_guard_helper_calls
            - self.added_writeback_helper_calls
        )


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = json.load(fh)
    if not isinstance(data, dict):
        raise SystemExit("MIR JSON root must be object")
    return data


def emit_mir_json(path: Path) -> None:
    subprocess.run(
        [
            str(ROOT / "target/release/hakorune"),
            "--backend",
            "mir",
            "--emit-mir-json",
            str(path),
            str(APP),
        ],
        cwd=ROOT,
        stdout=subprocess.DEVNULL,
        check=True,
    )


def instructions(function: dict[str, Any]) -> list[dict[str, Any]]:
    out: list[dict[str, Any]] = []
    blocks = function.get("blocks")
    if not isinstance(blocks, list):
        return out
    for block in blocks:
        if not isinstance(block, dict):
            continue
        insts = block.get("instructions", [])
        if isinstance(insts, list):
            out.extend(inst for inst in insts if isinstance(inst, dict))
    return out


def callee(inst: dict[str, Any]) -> tuple[str, str, str]:
    mir_call = inst.get("mir_call")
    if not isinstance(mir_call, dict):
        return "", "", ""
    callee_obj = mir_call.get("callee")
    if not isinstance(callee_obj, dict):
        return "", "", ""
    return (
        str(callee_obj.get("box_name", "")),
        str(callee_obj.get("name", "")),
        str(callee_obj.get("certainty", "")),
    )


def inventory_function(function: dict[str, Any]) -> FunctionInventory:
    inv = FunctionInventory(name=str(function.get("name", "")))
    for inst in instructions(function):
        op = inst.get("op")
        if op == "phi":
            inv.phi_count += 1
            continue
        if op != "mir_call":
            continue
        box_name, name, certainty = callee(inst)
        if box_name == "ArrayBox" and name == "get":
            inv.array_get_count += 1
            continue
        if box_name == "ArrayBox" and name == "set":
            inv.array_set_count += 1
            continue
        effects = inst.get("mir_call", {}).get("effects", [])
        if certainty != "Known" or effects:
            inv.unknown_call_count += 1
        else:
            inv.helper_barrier_call_count += 1
    return inv


def select_candidate(inventories: list[FunctionInventory], method: str | None) -> FunctionInventory:
    candidates = [inv for inv in inventories if inv.erased_get_set_helper_calls > 0]
    if not candidates:
        raise SystemExit("no ArrayBox get/set residence candidates found")
    if method is not None:
        matches = [inv for inv in candidates if inv.name == method or inv.name.endswith(method)]
        if len(matches) != 1:
            names = ", ".join(inv.name for inv in matches[:5])
            raise SystemExit(f"selected method must match one candidate: {method}: {names}")
        return matches[0]
    return max(
        candidates,
        key=lambda inv: (
            inv.net_helper_call_delta,
            inv.erased_get_set_helper_calls,
            -inv.unknown_call_count,
            inv.name,
        ),
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path)
    parser.add_argument("--method")
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    subprocess.run(["cargo", "build", "--release", "--bin", "hakorune"], cwd=ROOT, check=True)

    with tempfile.TemporaryDirectory(prefix="hakorune_array_slot_residence.") as tmp:
        mir_json = args.mir_json or Path(tmp) / "app.mir.json"
        if args.mir_json is None:
            emit_mir_json(mir_json)
        data = load_json(mir_json)

    functions = data.get("functions")
    if not isinstance(functions, list):
        raise SystemExit("MIR JSON missing functions[]")
    inventories = [
        inventory_function(fn)
        for fn in functions
        if isinstance(fn, dict) and isinstance(fn.get("name"), str)
    ]
    selected = select_candidate(inventories, args.method)
    eligible_functions = [inv for inv in inventories if inv.erased_get_set_helper_calls > 0]

    lines = [
        "output_contract=mir-array-slot-residence-inventory-v0",
        "input_kind=mir_json",
        "workload_id=representative-object-lifecycle-small-block-v0",
        f"candidate_function_count={len(eligible_functions)}",
        f"selected_method={selected.name}",
        f"selected_reason={'explicit_hot_context' if args.method else 'static_max_net_helper_call_delta'}",
        f"eligible_array_get_count={selected.array_get_count}",
        f"eligible_array_set_count={selected.array_set_count}",
        f"erased_get_set_helper_calls={selected.erased_get_set_helper_calls}",
        f"added_guard_helper_calls={selected.added_guard_helper_calls}",
        f"added_writeback_helper_calls={selected.added_writeback_helper_calls}",
        f"net_helper_call_delta={selected.net_helper_call_delta}",
        f"barrier_unknown_call_count={selected.unknown_call_count}",
        "barrier_escape_count=0",
        f"barrier_phi_count={selected.phi_count}",
        "barrier_storage_kind_count=1",
        "transform_open=0",
        "array_helper_abi_fallback=1",
        "positive_net_helper_call_delta_required=1",
        f"positive_net_helper_call_delta={1 if selected.net_helper_call_delta > 0 else 0}",
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
