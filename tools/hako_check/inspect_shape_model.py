"""Pure normalized shape counters for sealed inspect artifacts."""

from __future__ import annotations

import re
from typing import Any

from inspect_scope_identity import (
    objdump_symbols,
    require_unique_asm_symbol,
    require_unique_llvm_function,
    require_unique_mir_function,
)


SHAPE_CONTRACT = "hako-lowering-shape-report-v0"
COUNT_KEYS = (
    "blocks",
    "edges",
    "phi",
    "calls",
    "loads",
    "stores",
    "branches",
    "returns",
    "instructions",
)

MIR_SUCCESSOR_FIELDS = {
    "branch": ("then", "else"),
    "jump": ("target",),
    "checked_callout": ("normal", "fault"),
    "pinned_text_residence_enter": ("normal", "trap"),
}


def _zero_counts() -> dict[str, int | None]:
    return {key: 0 for key in COUNT_KEYS}


def _mir_successor_count(instruction: dict[str, Any], op: str) -> int:
    fields = MIR_SUCCESSOR_FIELDS.get(op)
    if fields is None:
        return 0
    successors: set[int] = set()
    for field in fields:
        value = instruction.get(field)
        if not isinstance(value, int) or isinstance(value, bool) or value < 0:
            raise SystemExit(f"shape MIR {op} successor is invalid: {field}")
        successors.add(value)
    return len(successors)


def mir_shape(mir: dict[str, Any], function_name: str) -> dict[str, int | None]:
    require_unique_mir_function(mir, function_name)
    function = next(
        row
        for row in mir["functions"]
        if isinstance(row, dict) and row.get("name") == function_name
    )
    blocks = function.get("blocks")
    if not isinstance(blocks, list):
        raise SystemExit(f"shape MIR blocks must be an array: {function_name}")
    counts = _zero_counts()
    counts["blocks"] = len(blocks)
    for block in blocks:
        if not isinstance(block, dict):
            raise SystemExit(f"shape MIR block must be an object: {function_name}")
        instructions = block.get("instructions")
        if not isinstance(instructions, list):
            raise SystemExit(f"shape MIR instructions must be an array: {function_name}")
        for instruction in instructions:
            if not isinstance(instruction, dict):
                raise SystemExit(f"shape MIR instruction must be an object: {function_name}")
            op = str(instruction.get("op", "")).lower()
            counts["instructions"] += 1
            if op == "phi":
                counts["phi"] += 1
            if op in {"call", "mir_call"}:
                counts["calls"] += 1
            if op == "load":
                counts["loads"] += 1
            if op == "store":
                counts["stores"] += 1
            if op in MIR_SUCCESSOR_FIELDS:
                counts["branches"] += 1
                counts["edges"] += _mir_successor_count(instruction, op)
            if op in {"ret", "return"}:
                counts["returns"] += 1
    return counts


def extract_llvm_function(text: str, function_name: str) -> str:
    require_unique_llvm_function(text, function_name)
    escaped = re.escape(function_name)
    header = re.compile(
        rf'^[ \t]*define\b[^@]*@(?:"{escaped}"|{escaped})\s*\(', re.MULTILINE
    ).search(text)
    if header is None:
        raise SystemExit(f"shape LLVM function header missing: {function_name}")
    rows = text[header.start() :].splitlines()
    body: list[str] = []
    for row in rows:
        body.append(row)
        if row.strip() == "}":
            return "\n".join(body) + "\n"
    raise SystemExit(f"shape LLVM function is unterminated: {function_name}")


def llvm_shape(text: str, function_name: str) -> dict[str, int | None]:
    function = extract_llvm_function(text, function_name)
    counts = _zero_counts()
    label_pattern = re.compile(r"^[A-Za-z$._][-A-Za-z$._0-9]*:\s*(?:;.*)?$")
    instruction_rows: list[str] = []
    saw_label = False
    for raw in function.splitlines()[1:]:
        row = raw.strip()
        if not row or row == "}" or row.startswith(";"):
            continue
        if label_pattern.match(row):
            counts["blocks"] += 1
            saw_label = True
            continue
        if not saw_label and not instruction_rows:
            counts["blocks"] = 1
        instruction_rows.append(row)
    counts["instructions"] = len(instruction_rows)
    for row in instruction_rows:
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
            counts["edges"] += len(re.findall(r"\blabel\s+%", row))
        if op == "ret":
            counts["returns"] += 1
    return counts


def extract_asm_symbol(text: str, symbol_name: str) -> list[str]:
    start = require_unique_asm_symbol(text, symbol_name)
    symbols = objdump_symbols(text)
    next_lines = [line_no for _, line_no in symbols if line_no > start]
    end = min(next_lines) if next_lines else len(text.splitlines()) + 1
    return text.splitlines()[start:end - 1]


def asm_shape(text: str, symbol_name: str) -> dict[str, int | None]:
    rows = extract_asm_symbol(text, symbol_name)
    counts = _zero_counts()
    counts["blocks"] = None
    counts["edges"] = None
    counts["phi"] = None
    counts["loads"] = None
    counts["stores"] = None
    instruction_pattern = re.compile(
        r"^\s*[0-9a-fA-F]+:\s+(?:(?:[0-9a-fA-F]{2})\s+)+(.+?)\s*$"
    )
    for row in rows:
        match = instruction_pattern.match(row)
        if match is None:
            continue
        mnemonic = match.group(1).split(None, 1)[0].lower()
        counts["instructions"] += 1
        if mnemonic.startswith("call"):
            counts["calls"] += 1
        if mnemonic == "jmp" or (mnemonic.startswith("j") and mnemonic != "jmp"):
            counts["branches"] += 1
        if mnemonic.startswith("ret"):
            counts["returns"] += 1
    return counts


def build_shape_report(
    *,
    identity: dict[str, Any],
    mir: dict[str, Any],
    llvm_text: str,
    asm_text: str,
    provenance: dict[str, Any] | None = None,
    external_c: dict[str, Any] | None = None,
) -> dict[str, Any]:
    if identity.get("shape_ready") is not True:
        raise SystemExit("shape report requires a shape-ready V1 identity")
    mapping_quality = identity.get("mappings")
    required_mappings = {
        "source_to_mir": "exact",
        "mir_to_llvm": "issuer_exact" if provenance is not None else "block",
        "llvm_to_asm": "symbol",
    }
    if mapping_quality != required_mappings:
        raise SystemExit("shape report mapping floor not satisfied")
    if provenance is not None:
        if provenance.get("output_contract") != "hako-lowering-provenance-v0":
            raise SystemExit("shape provenance contract mismatch")
        if provenance.get("asm") != {
            "quality": "symbol", "correspondence": "unavailable"
        }:
            raise SystemExit("shape provenance overclaims ASM correspondence")
    selectors = identity.get("selectors")
    if not isinstance(selectors, dict):
        raise SystemExit("shape report selector table missing")
    report: dict[str, Any] = {
        "output_contract": SHAPE_CONTRACT,
        "candidate_seal": identity.get("candidate_seal"),
        "observation_only": True,
        "cross_layer_correspondence": (
            "mir_llvm_issuer_exact" if provenance is not None else "unclaimed"
        ),
        "keeper_selection": False,
        "measurement_authority": False,
        "mapping_quality": required_mappings,
        "layers": {
            "mir": mir_shape(mir, str(selectors.get("mir_function", ""))),
            "llvm": llvm_shape(llvm_text, str(selectors.get("llvm_function", ""))),
            "asm": asm_shape(asm_text, str(selectors.get("asm_symbol", ""))),
        },
        "summary": "ok",
    }
    if provenance is not None:
        report["provenance"] = provenance
    if external_c is not None:
        c_shape = external_c.get("shape")
        if not isinstance(c_shape, dict):
            raise SystemExit("shape external C shape missing")
        report["layers"]["c_asm"] = c_shape
        report["external_reference"] = {
            key: value for key, value in external_c.items() if key != "shape"
        }
    return report
