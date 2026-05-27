#!/usr/bin/env python3
"""Normalize the hako-alloc facade reason duplicate-eval guard surface."""

from __future__ import annotations

import argparse
from pathlib import Path


def parse_report(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for raw_line in path.read_text(encoding="utf-8").splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def require(values: dict[str, str], key: str, expected: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{key} expected {expected!r}, got {actual!r}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--inventory-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    values = parse_report(args.inventory_report)
    require(values, "output_contract", "hako-alloc-facade-reason-duplicate-inventory-v0")
    require(values, "summary", "ok")
    require(values, "method_0", "objectLifecycleSmallAlloc")
    require(values, "method_0_unused_duplicate_reason_call_count", "0")
    require(values, "failing_method_count", "7")
    require(values, "total_unused_duplicate_reason_call_count", "20")

    lines = [
        "output_contract=hako-alloc-facade-reason-duplicate-eval-guard-v0",
        "input_contract=hako-alloc-facade-reason-duplicate-inventory-v0",
        "guard_scope=hako_alloc_object_lifecycle_facade_reason_calls",
        "small_alloc_fixed=1",
        "known_current_failure_count=7",
        "known_current_unused_duplicate_reason_call_count=20",
        f"known_current_failing_methods={values.get('failing_methods', '')}",
        "selected_next=generic_nested_argument_single_eval_fixture",
        "selected_next_kind=mir_correctness_fixture",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]
    text = "\n".join(lines) + "\n"
    if args.out is None:
        print(text, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
