#!/usr/bin/env python3
"""Shared helpers for FastMemory MIR-to-LLVM producer evidence."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[2]
LLVM_BUILDER = ROOT / "src" / "llvm_py" / "llvm_builder.py"


def int_flag(value: bool) -> int:
    return 1 if value else 0


def load_json(path: Path) -> dict[str, Any]:
    try:
        with path.open("r", encoding="utf-8") as f:
            data = json.load(f)
    except OSError as exc:
        raise SystemExit(f"failed to read MIR JSON: {path}: {exc}") from exc
    except json.JSONDecodeError as exc:
        raise SystemExit(f"failed to parse MIR JSON: {path}: {exc}") from exc
    if not isinstance(data, dict):
        raise SystemExit(f"expected MIR JSON object: {path}")
    return data


def functions(mir: dict[str, Any]) -> list[dict[str, Any]]:
    values = mir.get("functions", [])
    if not isinstance(values, list):
        return []
    return [value for value in values if isinstance(value, dict)]


def fastmem_regions(mir: dict[str, Any]) -> list[dict[str, Any]]:
    regions: list[dict[str, Any]] = []
    for function in functions(mir):
        metadata = function.get("metadata", {})
        if not isinstance(metadata, dict):
            continue
        for region in metadata.get("fastmem_regions", []):
            if isinstance(region, dict):
                regions.append(region)
    return regions


def function_has_fastmem_region(function: dict[str, Any]) -> bool:
    metadata = function.get("metadata", {})
    if not isinstance(metadata, dict):
        return False
    regions = metadata.get("fastmem_regions", [])
    return isinstance(regions, list) and any(isinstance(region, dict) for region in regions)


def fastmem_access_plans(mir: dict[str, Any]) -> list[dict[str, Any]]:
    plans: list[dict[str, Any]] = []
    for function in functions(mir):
        metadata = function.get("metadata", {})
        if not isinstance(metadata, dict):
            continue
        for plan in metadata.get("fastmem_access_plans", []):
            if isinstance(plan, dict):
                plans.append(plan)
    return plans


def fastmem_memops(mir: dict[str, Any]) -> list[dict[str, Any]]:
    memops: list[dict[str, Any]] = []
    for function in functions(mir):
        blocks = function.get("blocks", [])
        if not isinstance(blocks, list):
            continue
        for block in blocks:
            if not isinstance(block, dict):
                continue
            instructions = block.get("instructions", [])
            if not isinstance(instructions, list):
                continue
            for inst in instructions:
                if isinstance(inst, dict) and inst.get("op") == "memop":
                    memops.append(inst)
    return memops


def branch_cfg_count(mir: dict[str, Any]) -> int:
    count = 0
    for function in functions(mir):
        if not function_has_fastmem_region(function):
            continue
        blocks = function.get("blocks", [])
        if not isinstance(blocks, list):
            continue
        for block in blocks:
            if not isinstance(block, dict):
                continue
            instructions = block.get("instructions", [])
            if isinstance(instructions, list):
                count += sum(
                    1
                    for inst in instructions
                    if isinstance(inst, dict) and inst.get("op") == "branch"
                )
            terminator = block.get("terminator")
            if isinstance(terminator, dict) and terminator.get("op") == "branch":
                count += 1
    return count


def fastmem_free_head_non_empty_facts(mir: dict[str, Any]) -> list[dict[str, Any]]:
    facts: list[dict[str, Any]] = []
    for function in functions(mir):
        metadata = function.get("metadata", {})
        if not isinstance(metadata, dict):
            continue
        for fact in metadata.get("fastmem_free_head_non_empty_facts", []):
            if isinstance(fact, dict):
                facts.append(fact)
    return facts


def metadata_facts(mir: dict[str, Any], key: str) -> list[dict[str, Any]]:
    facts: list[dict[str, Any]] = []
    for function in functions(mir):
        metadata = function.get("metadata", {})
        if not isinstance(metadata, dict):
            continue
        for fact in metadata.get(key, []):
            if isinstance(fact, dict):
                facts.append(fact)
    return facts


def is_verified(plan: dict[str, Any]) -> bool:
    return bool(plan.get("verified")) and plan.get("status") == "verified"


def count_plans(plans: list[dict[str, Any]], kind: str, *, verified: bool | None = None) -> int:
    count = 0
    for plan in plans:
        if plan.get("kind") != kind:
            continue
        if verified is not None and is_verified(plan) != verified:
            continue
        count += 1
    return count


def count_memops(memops: list[dict[str, Any]], kind: str) -> int:
    return sum(1 for inst in memops if inst.get("kind") == kind)


def page_local_alloc_route_candidate(
    *,
    local_free_pop_count: int,
    free_head_push_count: int,
    free_head_pop_count: int,
) -> str:
    if local_free_pop_count == 0 and free_head_push_count == 0 and free_head_pop_count == 0:
        return "none"
    if local_free_pop_count == 1 and free_head_push_count == 0 and free_head_pop_count == 0:
        return "local_free_alloc"
    if local_free_pop_count == 0 and free_head_push_count == 0 and free_head_pop_count == 1:
        return "free_head_alloc"
    if local_free_pop_count == 1 and free_head_push_count == 1 and free_head_pop_count == 1:
        return "refill_then_free_head_alloc"
    return "mixed"


def page_local_free_route_candidate(
    *,
    local_free_push_count: int,
    local_free_pop_count: int,
    free_head_push_count: int,
    free_head_pop_count: int,
) -> str:
    if local_free_push_count == 0:
        return "none"
    if (
        local_free_push_count == 1
        and local_free_pop_count == 0
        and free_head_push_count == 0
        and free_head_pop_count == 0
    ):
        return "same_owner_local_free"
    return "mixed"


def run_llvm_builder(mir_json: Path, object_out: Path) -> None:
    proc = subprocess.run(
        [sys.executable, str(LLVM_BUILDER), str(mir_json), "-o", str(object_out)],
        cwd=str(ROOT),
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if proc.returncode != 0:
        if proc.stdout:
            sys.stderr.write(proc.stdout)
        if proc.stderr:
            sys.stderr.write(proc.stderr)
        raise SystemExit(proc.returncode)


def string_value(value: Any, default: str = "") -> str:
    if value is None:
        return default
    return str(value)
