#!/usr/bin/env python3
"""Report MIR builder single-evaluation surface sweep counts."""

from __future__ import annotations

import argparse
import json
from dataclasses import dataclass
from pathlib import Path
from typing import Any


@dataclass(frozen=True)
class Surface:
    surface_id: str
    method: str
    symbols: tuple[str, ...]


SURFACES = (
    Surface("field_assignment", "SingleEvalSurfaceProbe.fieldAssign/0", ("SweepFieldBaseSide.make/0", "SweepFieldValueSide.value/0")),
    Surface("index_read", "SingleEvalSurfaceProbe.indexRead/0", ("SweepIndexSide.idx/0",)),
    Surface("index_write", "SingleEvalSurfaceProbe.indexWrite/0", ("SweepIndexSide.idx/0", "SweepIndexValueSide.value/0")),
    Surface("print_fallback", "SingleEvalSurfaceProbe.printValue/0", ("SweepPrintSide.value/0",)),
    Surface("typeop_method", "SingleEvalSurfaceProbe.typeOpMethod/0", ("SweepTypeSide.make/0",)),
    Surface("constructor_arg", "SingleEvalSurfaceProbe.ctorArg/0", ("SweepCtorArgSide.value/0",)),
)


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = json.load(fh)
    if not isinstance(data, dict):
        raise SystemExit("MIR JSON root must be an object")
    return data


def functions_by_name(data: dict[str, Any]) -> dict[str, dict[str, Any]]:
    functions = data.get("functions")
    if not isinstance(functions, list):
        raise SystemExit("MIR JSON missing functions[]")
    result: dict[str, dict[str, Any]] = {}
    for fn in functions:
        if isinstance(fn, dict) and isinstance(fn.get("name"), str):
            result[fn["name"]] = fn
    return result


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


def method_calls(function: dict[str, Any]) -> list[str]:
    calls: list[str] = []
    for block in function.get("blocks", []):
        if not isinstance(block, dict):
            continue
        for inst in block.get("instructions", []):
            if isinstance(inst, dict) and inst.get("op") == "mir_call":
                calls.append(callee_name(inst))
    return calls


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    functions = functions_by_name(load_json(args.mir_json))
    lines = [
        "output_contract=mir-builder-single-eval-surface-sweep-v0",
        "input_contract=hako-mimalloc-post-single-eval-fixes-measurement-v0",
    ]
    failing: list[str] = []
    symbol_index = 0
    for idx, surface in enumerate(SURFACES):
        function = functions.get(surface.method)
        if function is None:
            raise SystemExit(f"missing function {surface.method}")
        calls = method_calls(function)
        surface_failed = False
        lines.extend(
            [
                f"surface_{idx}_id={surface.surface_id}",
                f"surface_{idx}_method={surface.method}",
            ]
        )
        for symbol in surface.symbols:
            count = calls.count(symbol)
            if count != 1:
                surface_failed = True
            lines.append(f"symbol_{symbol_index}_surface={surface.surface_id}")
            lines.append(f"symbol_{symbol_index}_name={symbol}")
            lines.append(f"symbol_{symbol_index}_expected_count=1")
            lines.append(f"symbol_{symbol_index}_actual_count={count}")
            symbol_index += 1
        if surface_failed:
            failing.append(surface.surface_id)
        lines.append(f"surface_{idx}_summary={'fail' if surface_failed else 'ok'}")

    lines.extend(
        [
            f"surface_count={len(SURFACES)}",
            f"symbol_count={symbol_index}",
            f"failing_surface_count={len(failing)}",
            f"failing_surfaces={','.join(failing)}",
            "selected_next=static_scalar_method_fact_selection" if not failing else "selected_next=single_eval_owner_fix",
            "winner_claim=0",
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
