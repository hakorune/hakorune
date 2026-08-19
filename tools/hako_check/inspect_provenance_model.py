"""Validate issuer-emitted MIR-to-declared-LLVM-boundary block/edge origins."""

from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path
from typing import Any

from inspect_shape_model import extract_llvm_function
from inspect_scope_identity import require_unique_mir_function


PROVENANCE_CONTRACT = "hako-lowering-provenance-v0"
DISPOSITIONS = {"preserved", "split", "merged", "deleted", "introduced"}
MIR_EDGES = {
    "jump": (("target", "target"),),
    "branch": (("then", "then"), ("else", "else")),
    "checked_callout": (("normal", "normal"), ("fault", "fault")),
    "pinned_text_residence_enter": (("normal", "normal"), ("trap", "trap")),
}
LABEL = re.compile(r"^([A-Za-z$._][-A-Za-z$._0-9]*):\s*(?:;.*)?$")


def _sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def parse_raw_events(
    path: Path, *, issuer: str = "selected_pinned_text_lowerer",
) -> list[dict[str, Any]]:
    if not issuer:
        raise SystemExit("provenance issuer is missing")
    rows: list[dict[str, Any]] = []
    seen: set[tuple[Any, ...]] = set()
    for line_no, raw in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        fields = raw.split("\t")
        if len(fields) != 9:
            raise SystemExit(f"provenance raw row {line_no} must have 9 fields")
        entity, block, instruction, arm, target, llvm_from, llvm_to, disposition, reason = fields
        if entity not in {"block", "edge"} or disposition not in DISPOSITIONS:
            raise SystemExit(f"provenance raw row {line_no} vocabulary mismatch")
        try:
            block_id, instruction_id, target_id = int(block), int(instruction), int(target)
        except ValueError as error:
            raise SystemExit(f"provenance raw row {line_no} integer mismatch") from error
        identity = tuple(fields)
        if identity in seen:
            raise SystemExit(f"provenance raw row {line_no} is duplicated")
        seen.add(identity)
        if disposition == "introduced":
            if (block_id, instruction_id, arm, target_id) != (-1, -1, "none", -1):
                raise SystemExit(f"provenance raw row {line_no} introduced source mismatch")
            if not llvm_from:
                raise SystemExit(f"provenance raw row {line_no} introduced endpoint mismatch")
        elif disposition == "deleted":
            if block_id < 0 or llvm_from or llvm_to:
                raise SystemExit(f"provenance raw row {line_no} deleted endpoint mismatch")
        elif block_id < 0 or not llvm_from:
            raise SystemExit(f"provenance raw row {line_no} endpoint mismatch")
        if entity == "block" and llvm_to:
            raise SystemExit(f"provenance raw row {line_no} block endpoint mismatch")
        if entity == "edge" and disposition not in {"deleted"} and not llvm_to:
            raise SystemExit(f"provenance raw row {line_no} edge endpoint mismatch")
        rows.append(
            {
                "entity": entity,
                "mir": {
                    "block": block_id,
                    "instruction": instruction_id,
                    "arm": arm,
                    "target": target_id,
                },
                "llvm": {"from": llvm_from, "to": llvm_to},
                "disposition": disposition,
                "reason_kind": reason,
                "issuer": issuer,
            }
        )
    if not rows:
        raise SystemExit("provenance raw journal is empty")
    return rows


def _mir_census(mir: dict[str, Any], function_name: str) -> tuple[set[int], set[tuple[int, int, str, int]]]:
    require_unique_mir_function(mir, function_name)
    function = next(row for row in mir["functions"] if row.get("name") == function_name)
    blocks: set[int] = set()
    edges: set[tuple[int, int, str, int]] = set()
    for block in function.get("blocks", []):
        bid = block.get("id")
        if not isinstance(bid, int) or isinstance(bid, bool) or bid in blocks:
            raise SystemExit("provenance MIR block identity mismatch")
        blocks.add(bid)
        for ii, instruction in enumerate(block.get("instructions", [])):
            op = str(instruction.get("op", "")).lower()
            for arm, field in MIR_EDGES.get(op, ()):
                target = instruction.get(field)
                if not isinstance(target, int) or isinstance(target, bool):
                    raise SystemExit("provenance MIR edge target mismatch")
                edges.add((bid, ii, arm, target))
    return blocks, edges


def _llvm_census(text: str, function_name: str) -> tuple[set[str], set[tuple[str, str]]]:
    function = extract_llvm_function(text, function_name)
    blocks: set[str] = set()
    edges: set[tuple[str, str]] = set()
    current = ""
    for raw in function.splitlines()[1:]:
        row = raw.strip()
        match = LABEL.match(row)
        if match:
            current = match.group(1)
            if current in blocks:
                raise SystemExit("provenance LLVM block identity mismatch")
            blocks.add(current)
            continue
        if not row or row == "}" or row.startswith(";"):
            continue
        if not current:
            raise SystemExit("provenance LLVM function requires explicit entry label")
        if re.match(r"^(?:br|switch|indirectbr)\b", row.split("=", 1)[-1].strip()):
            for target in re.findall(r"\blabel\s+%([A-Za-z$._][-A-Za-z$._0-9]*)", row):
                edges.add((current, target))
    return blocks, edges


def build_provenance(
    *, raw_path: Path, mir_path: Path, llvm_path: Path,
    mir_function: str, llvm_function: str,
    issuer: str = "selected_pinned_text_lowerer",
    llvm_boundary: str = "final",
) -> dict[str, Any]:
    if llvm_boundary not in {"lowered_pre_opt", "final"}:
        raise SystemExit("provenance LLVM boundary mismatch")
    mir = json.loads(mir_path.read_text(encoding="utf-8"))
    llvm_text = llvm_path.read_text(encoding="utf-8", errors="replace")
    rows = parse_raw_events(raw_path, issuer=issuer)
    mir_blocks, mir_edges = _mir_census(mir, mir_function)
    llvm_blocks, llvm_edges = _llvm_census(llvm_text, llvm_function)
    row_mir_blocks = {
        row["mir"]["block"] for row in rows
        if row["entity"] == "block" and row["disposition"] != "introduced"
    }
    row_mir_edges = {
        (m["block"], m["instruction"], m["arm"], m["target"])
        for row in rows
        if row["entity"] == "edge"
        and row["disposition"] != "introduced"
        and (m := row["mir"])["arm"] != "none"
    }
    row_llvm_blocks = {
        row["llvm"]["from"] for row in rows
        if row["entity"] == "block" and row["disposition"] != "deleted"
    }
    row_llvm_edges = {
        (row["llvm"]["from"], row["llvm"]["to"])
        for row in rows if row["entity"] == "edge" and row["disposition"] != "deleted"
    }
    logical_mir_edge_rows = [
        (m["block"], m["instruction"], m["arm"], m["target"])
        for row in rows
        if row["entity"] == "edge"
        and row["disposition"] != "introduced"
        and (m := row["mir"])["arm"] != "none"
    ]
    llvm_block_rows = [
        row["llvm"]["from"] for row in rows
        if row["entity"] == "block" and row["disposition"] != "deleted"
    ]
    llvm_edge_rows = [
        (row["llvm"]["from"], row["llvm"]["to"])
        for row in rows if row["entity"] == "edge" and row["disposition"] != "deleted"
    ]
    if (len(logical_mir_edge_rows) != len(set(logical_mir_edge_rows)) or
            len(llvm_block_rows) != len(set(llvm_block_rows)) or
            len(llvm_edge_rows) != len(set(llvm_edge_rows))):
        raise SystemExit("provenance relation ownership is duplicated")
    if row_mir_blocks != mir_blocks or row_mir_edges != mir_edges:
        raise SystemExit("provenance MIR coverage mismatch")
    if row_llvm_blocks != llvm_blocks or row_llvm_edges != llvm_edges:
        raise SystemExit(f"provenance {llvm_boundary} LLVM coverage mismatch")
    for row in rows:
        if (row["disposition"] != "introduced" and
                row["mir"]["block"] not in mir_blocks):
            raise SystemExit("provenance row has a dangling MIR block")
        if row["disposition"] != "deleted" and (
            row["llvm"]["from"] not in llvm_blocks or (
                row["entity"] == "edge" and row["llvm"]["to"] not in llvm_blocks
            )
        ):
            raise SystemExit("provenance row has a dangling LLVM endpoint")
    return {
        "output_contract": PROVENANCE_CONTRACT,
        "candidate_input": {
            "mir_sha256": _sha256(mir_path),
            "llvm_sha256": _sha256(llvm_path),
            "mir_function": mir_function,
            "llvm_function": llvm_function,
            "llvm_boundary": llvm_boundary,
            "issuer": issuer,
        },
        "coverage": {
            "mir_blocks": len(mir_blocks), "mir_edges": len(mir_edges),
            "llvm_blocks": len(llvm_blocks), "llvm_edges": len(llvm_edges),
        },
        "relations": rows,
        "asm": {"quality": "symbol", "correspondence": "unavailable"},
        "observation_only": True,
    }
