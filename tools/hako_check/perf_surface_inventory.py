#!/usr/bin/env python3
"""Inventory selected .hako methods for hako_check perf-surface evidence."""

from __future__ import annotations

import argparse
import re
from dataclasses import dataclass
from pathlib import Path


DEFAULT_TARGET = Path("lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako")
DEFAULT_BOX = "HakoAllocObjectLifecycleFacade"
DEFAULT_METHODS = ("objectLifecycleSmallAlloc", "objectLifecycleReleaseBlock")
METHOD_CALL_RE = re.compile(r"\b(?:me|[a-zA-Z_][a-zA-Z0-9_]*)\.[a-zA-Z_][a-zA-Z0-9_]*\s*\(")
ARRAY_ACCESS_RE = re.compile(r"\.(?:get|length)\s*\(")
ARRAY_GET_RE = re.compile(r"\.get\s*\(")
ARRAY_LENGTH_RE = re.compile(r"\.length\s*\(")
FIELD_ACCESS_RE = re.compile(r"\b(?:me|[a-zA-Z_][a-zA-Z0-9_]*)\.[a-zA-Z_][a-zA-Z0-9_]*\b(?!\s*\()")
FIELD_SET_RE = re.compile(r"\b(?:me|[a-zA-Z_][a-zA-Z0-9_]*)\.[a-zA-Z_][a-zA-Z0-9_]*\s*=")
ALLOCATION_LIKE_RE = re.compile(r"\bnew\s+[A-Za-z_][A-Za-z0-9_]*\s*\(")
OBSERVER_CALL_RE = re.compile(r"\bme\.objectLifecycle[A-Z][A-Za-z0-9_]*\s*\(\s*\)")


@dataclass(frozen=True)
class MethodInventory:
    name: str
    method_call_count: int
    loop_method_call_count: int
    array_access_count: int
    linear_search_candidate: int
    result_capsule_churn: int
    observer_call_count: int
    field_get_count: int
    field_set_count: int
    loop_field_get_count: int
    loop_field_set_count: int
    loop_array_get_count: int
    loop_array_length_count: int
    allocation_like_in_loop_count: int
    hot_path_risk: str
    hot_path_reason: str
    suggested_next: str
    suggested_next_kind: str
    confidence: str


def find_method_body(source: str, method_name: str) -> str:
    match = re.search(rf"^\s*{re.escape(method_name)}\s*\([^)]*\)\s*\{{", source, re.M)
    if match is None:
        raise SystemExit(f"method not found: {method_name}")
    brace_start = source.find("{", match.start())
    depth = 0
    for idx in range(brace_start, len(source)):
        ch = source[idx]
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return source[brace_start + 1 : idx]
    raise SystemExit(f"method body not closed: {method_name}")


def loop_bodies(body: str) -> list[str]:
    loops: list[str] = []
    pos = 0
    while True:
        match = re.search(r"\bloop\s*\([^)]*\)\s*\{", body[pos:])
        if match is None:
            return loops
        brace_start = pos + body[pos:].find("{", match.start())
        depth = 0
        for idx in range(brace_start, len(body)):
            ch = body[idx]
            if ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    loops.append(body[brace_start + 1 : idx])
                    pos = idx + 1
                    break
        else:
            return loops


def classify_method(method_name: str, body: str) -> MethodInventory:
    loops = loop_bodies(body)
    method_call_count = len(METHOD_CALL_RE.findall(body))
    loop_method_call_count = sum(len(METHOD_CALL_RE.findall(loop)) for loop in loops)
    array_access_count = len(ARRAY_ACCESS_RE.findall(body))
    field_set_count = len(FIELD_SET_RE.findall(body))
    field_get_count = max(0, len(FIELD_ACCESS_RE.findall(body)) - field_set_count)
    loop_field_set_count = sum(len(FIELD_SET_RE.findall(loop)) for loop in loops)
    loop_field_get_count = sum(
        max(0, len(FIELD_ACCESS_RE.findall(loop)) - len(FIELD_SET_RE.findall(loop)))
        for loop in loops
    )
    loop_array_get_count = sum(len(ARRAY_GET_RE.findall(loop)) for loop in loops)
    loop_array_length_count = sum(len(ARRAY_LENGTH_RE.findall(loop)) for loop in loops)
    allocation_like_in_loop_count = sum(len(ALLOCATION_LIKE_RE.findall(loop)) for loop in loops)
    result_capsule_churn = int(
        "reset" in body
        and ("alloc_result" in body or "release_result" in body or "realloc_result" in body)
    )
    observer_call_count = len(OBSERVER_CALL_RE.findall(body))
    linear_search_candidate = int("objectLifecycleKnownPageIndexById" in body)

    if method_name == "objectLifecycleReleaseBlock" and linear_search_candidate:
        suggested_next = "release_known_page_fast_path"
        hot_path_risk = "high"
        hot_path_reason = "linear_search_candidate"
        suggested_next_kind = "box_count"
        confidence = "high"
    elif method_name == "objectLifecycleSmallAlloc" and "selectPage" in body:
        suggested_next = "select_page_single_page_fast_path"
        hot_path_risk = "medium"
        hot_path_reason = "source_selectPage_hot_path"
        suggested_next_kind = "box_count"
        confidence = "medium"
    elif loop_array_get_count > 0 or loop_array_length_count > 0:
        suggested_next = "array_loop_access_reduction"
        hot_path_risk = "medium"
        hot_path_reason = "loop_array_access"
        suggested_next_kind = "box_count"
        confidence = "medium"
    elif allocation_like_in_loop_count > 0:
        suggested_next = "allocation_like_in_loop_reduction"
        hot_path_risk = "high"
        hot_path_reason = "allocation_like_in_loop"
        suggested_next_kind = "box_shape"
        confidence = "high"
    elif result_capsule_churn:
        suggested_next = "result_capsule_hot_loop_update_reduction"
        hot_path_risk = "medium"
        hot_path_reason = "result_capsule_churn"
        suggested_next_kind = "box_shape"
        confidence = "medium"
    else:
        suggested_next = "none"
        hot_path_risk = "low"
        hot_path_reason = "none"
        suggested_next_kind = "none"
        confidence = "low"

    return MethodInventory(
        name=method_name,
        method_call_count=method_call_count,
        loop_method_call_count=loop_method_call_count,
        array_access_count=array_access_count,
        linear_search_candidate=linear_search_candidate,
        result_capsule_churn=result_capsule_churn,
        observer_call_count=observer_call_count,
        field_get_count=field_get_count,
        field_set_count=field_set_count,
        loop_field_get_count=loop_field_get_count,
        loop_field_set_count=loop_field_set_count,
        loop_array_get_count=loop_array_get_count,
        loop_array_length_count=loop_array_length_count,
        allocation_like_in_loop_count=allocation_like_in_loop_count,
        hot_path_risk=hot_path_risk,
        hot_path_reason=hot_path_reason,
        suggested_next=suggested_next,
        suggested_next_kind=suggested_next_kind,
        confidence=confidence,
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--target", type=Path, default=DEFAULT_TARGET)
    parser.add_argument("--target-box", default=DEFAULT_BOX)
    parser.add_argument("--methods", default=",".join(DEFAULT_METHODS))
    parser.add_argument("--contract-version", choices=("v0", "v1"), default="v0")
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    target = args.target
    if not target.is_file():
        raise SystemExit(f"missing target file: {target}")
    source = target.read_text(encoding="utf-8", errors="replace")
    methods = [item.strip() for item in args.methods.split(",") if item.strip()]
    if not methods:
        raise SystemExit("--methods must select at least one method")
    inventories = [classify_method(name, find_method_body(source, name)) for name in methods]

    release = next((item for item in inventories if item.name == "objectLifecycleReleaseBlock"), inventories[0])
    selected = release if release.linear_search_candidate else inventories[0]

    output_contract = "hako-check-perf-surface-v1"
    if args.contract_version == "v0":
        output_contract = "hako-check-perf-surface-inventory-v0"

    lines = [
        f"output_contract={output_contract}",
        "input_contract=hako-check-perf-surface-contract-v0",
        f"target_file={target.as_posix()}",
        f"target_box={args.target_box}",
    ]
    for idx, inv in enumerate(inventories):
        prefix = f"target_method_{idx}"
        lines.extend(
            [
                f"{prefix}={inv.name}",
                f"{prefix}_method_call_count={inv.method_call_count}",
                f"{prefix}_loop_method_call_count={inv.loop_method_call_count}",
                f"{prefix}_array_access_count={inv.array_access_count}",
                f"{prefix}_linear_search_candidate={inv.linear_search_candidate}",
                f"{prefix}_result_capsule_churn={inv.result_capsule_churn}",
                f"{prefix}_observer_call_count={inv.observer_call_count}",
                f"{prefix}_field_get_count={inv.field_get_count}",
                f"{prefix}_field_set_count={inv.field_set_count}",
                f"{prefix}_loop_field_get_count={inv.loop_field_get_count}",
                f"{prefix}_loop_field_set_count={inv.loop_field_set_count}",
                f"{prefix}_loop_array_get_count={inv.loop_array_get_count}",
                f"{prefix}_loop_array_length_count={inv.loop_array_length_count}",
                f"{prefix}_allocation_like_in_loop_count={inv.allocation_like_in_loop_count}",
                f"{prefix}_hot_path_risk={inv.hot_path_risk}",
                f"{prefix}_hot_path_reason={inv.hot_path_reason}",
                f"{prefix}_suggested_next={inv.suggested_next}",
                f"{prefix}_suggested_next_kind={inv.suggested_next_kind}",
                f"{prefix}_confidence={inv.confidence}",
            ]
        )
    lines.extend(
        [
            f"target_method={selected.name}",
            f"linear_search_candidate={selected.linear_search_candidate}",
            f"loop_field_get_count={selected.loop_field_get_count}",
            f"loop_field_set_count={selected.loop_field_set_count}",
            f"loop_array_get_count={selected.loop_array_get_count}",
            f"loop_array_length_count={selected.loop_array_length_count}",
            f"allocation_like_in_loop_count={selected.allocation_like_in_loop_count}",
            f"suggested_next={selected.suggested_next}",
            f"suggested_next_kind={selected.suggested_next_kind}",
            f"confidence={selected.confidence}",
            "winner_claim=0",
            "replacement_active=0",
            "summary=ok",
        ]
    )
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
