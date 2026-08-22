"""Build an honest MIR-origin/lowered-LLVM footprint plus symbol-only ASM totals."""

from __future__ import annotations

import re
from collections import defaultdict
from typing import Any

from inspect_shape_model import asm_shape, extract_llvm_function


FOOTPRINT_CONTRACT = "hako-origin-footprint-v0"
COUNT_KEYS = ("instructions", "phi", "calls", "loads", "stores", "branches", "returns")
LABEL = re.compile(r"^([A-Za-z$._][-A-Za-z$._0-9]*):\s*(?:;.*)?$")


def _llvm_block_shapes(text: str, function_name: str) -> dict[str, dict[str, int]]:
    function = extract_llvm_function(text, function_name)
    shapes: dict[str, dict[str, int]] = {}
    current: str | None = None
    for raw in function.splitlines()[1:]:
        row = raw.strip()
        if not row or row == "}" or row.startswith(";"):
            continue
        match = LABEL.match(row)
        if match:
            current = match.group(1)
            if current in shapes:
                raise SystemExit("origin footprint duplicate LLVM block")
            shapes[current] = {key: 0 for key in COUNT_KEYS}
            continue
        if current is None:
            raise SystemExit("origin footprint requires explicit LLVM entry label")
        counts = shapes[current]
        counts["instructions"] += 1
        rhs = row.split("=", 1)[-1].strip() if "=" in row else row
        op = rhs.split(None, 1)[0] if rhs else ""
        if op == "phi":
            counts["phi"] += 1
        if re.search(r"\b(?:call|invoke|callbr)\b", rhs):
            counts["calls"] += 1
        if op == "load":
            counts["loads"] += 1
        if op == "store":
            counts["stores"] += 1
        if op in {"br", "switch", "indirectbr"}:
            counts["branches"] += 1
        if op == "ret":
            counts["returns"] += 1
    return shapes


def build_origin_footprint(
    *, provenance: dict[str, Any], llvm_text: str, asm_text: str,
    asm_symbol: str,
) -> dict[str, Any]:
    candidate = provenance.get("candidate_input")
    if provenance.get("output_contract") != "hako-lowering-provenance-v0" or not isinstance(
        candidate, dict
    ):
        raise SystemExit("origin footprint provenance contract mismatch")
    boundary = candidate.get("llvm_boundary")
    if boundary not in {"lowered_pre_opt", "final"}:
        raise SystemExit("origin footprint LLVM boundary mismatch")
    llvm_function = str(candidate.get("llvm_function", ""))
    block_shapes = _llvm_block_shapes(llvm_text, llvm_function)
    groups: dict[tuple[int, int, str], dict[str, Any]] = {}
    block_owners: dict[str, tuple[int, int, str]] = {}
    for relation in provenance.get("relations", []):
        mir = relation["mir"]
        llvm = relation["llvm"]
        key = (mir["block"], mir["instruction"], relation["reason_kind"])
        group = groups.setdefault(key, {"blocks": set(), "edges": set()})
        if relation["entity"] == "block":
            block = llvm["from"]
            if block in block_owners and block_owners[block] != key:
                raise SystemExit("origin footprint LLVM block has multiple origins")
            block_owners[block] = key
            group["blocks"].add(block)
        elif relation["entity"] == "edge":
            group["edges"].add((llvm["from"], llvm["to"]))
        else:
            raise SystemExit("origin footprint relation entity mismatch")
    if set(block_owners) != set(block_shapes):
        raise SystemExit("origin footprint LLVM block coverage mismatch")

    origins: list[dict[str, Any]] = []
    for (block, instruction, reason), group in sorted(groups.items()):
        counts = {key: 0 for key in COUNT_KEYS}
        for llvm_block in group["blocks"]:
            for name, value in block_shapes[llvm_block].items():
                counts[name] += value
        origins.append({
            "mir_origin": {"block": block, "instruction": instruction, "reason_kind": reason},
            "lowered_llvm": {
                "blocks": sorted(group["blocks"]),
                "block_count": len(group["blocks"]),
                "edge_count": len(group["edges"]),
                "shape": counts,
            },
            "machine_origin_attribution": "unavailable",
        })
    return {
        "output_contract": FOOTPRINT_CONTRACT,
        "observation_only": True,
        "keeper_selection": False,
        "measurement_authority": False,
        "mir_llvm_correspondence": "issuer_exact",
        "llvm_boundary": boundary,
        "lowered_llvm_to_machine": "unavailable",
        "origins": origins,
        "asm": {
            "symbol": asm_symbol,
            "origin_attribution": "unavailable",
            "shape": asm_shape(asm_text, asm_symbol),
        },
        "summary": "ok",
    }


def render_origin_footprint_markdown(footprint: dict[str, Any]) -> str:
    rows = [
        "## MIR origin → lowered LLVM footprint", "",
        "| MIR origin | reason | LLVM blocks | edges | instructions | calls | branches |",
        "|---|---|---:|---:|---:|---:|---:|",
    ]
    for origin in footprint["origins"]:
        mir = origin["mir_origin"]
        llvm = origin["lowered_llvm"]
        shape = llvm["shape"]
        rows.append(
            f"| bb{mir['block']}:{mir['instruction']} | {mir['reason_kind']} | "
            f"{llvm['block_count']} | {llvm['edge_count']} | "
            f"{shape['instructions']} | {shape['calls']} | {shape['branches']} |"
        )
    asm = footprint["asm"]
    shape = asm["shape"]
    rows.extend([
        "", "## Selected ASM symbol", "",
        f"- symbol: `{asm['symbol']}`",
        f"- instructions: {shape['instructions']}",
        f"- branches: {shape['branches']}",
        f"- calls: {shape['calls']}",
        f"- returns: {shape['returns']}",
        "- MIR/LLVM origin attribution: unavailable",
        f"- {footprint['llvm_boundary']} → machine: unavailable", "",
    ])
    return "\n".join(rows)
