#!/usr/bin/env python3
"""Close out the MIR builder nested argument single-eval owner fix."""

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
    parser.add_argument("--generic-fixture-report", type=Path, required=True)
    parser.add_argument("--facade-inventory-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    generic = parse_report(args.generic_fixture_report)
    facade = parse_report(args.facade_inventory_report)

    require(generic, "output_contract", "generic-nested-argument-single-eval-fixture-v0")
    require(generic, "fixture", "generic_nested_argument_single_eval")
    require(generic, "expected_nested_call_count", "1")
    require(generic, "actual_nested_call_count", "1")
    require(generic, "summary", "ok")

    require(facade, "output_contract", "hako-alloc-facade-reason-duplicate-inventory-v0")
    require(facade, "failing_method_count", "0")
    require(facade, "total_unused_duplicate_reason_call_count", "0")
    require(facade, "summary", "ok")

    lines = [
        "output_contract=mir-builder-nested-argument-single-eval-owner-fix-v0",
        "input_contract=generic-nested-argument-single-eval-fixture-v0",
        "fixture=generic_nested_argument_single_eval",
        f"actual_nested_call_count={generic['actual_nested_call_count']}",
        f"facade_reason_duplicate_failure_count={facade['failing_method_count']}",
        "facade_unused_duplicate_reason_call_count=0",
        "owner_fix=me_call_argument_lowering_deferred_until_route_selected",
        "generic_cse_added=0",
        "static_scalar_lowering_added=0",
        "semantic_summary=ok",
        "selected_next=post_nested_argument_single_eval_fix_measurement",
        "selected_next_kind=measurement_refresh",
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
