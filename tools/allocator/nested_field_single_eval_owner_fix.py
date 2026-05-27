#!/usr/bin/env python3
"""Close out the MIR builder nested field single-eval owner fix."""

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
    parser.add_argument("--fixture-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    values = parse_report(args.fixture_report)
    require(values, "output_contract", "mir-builder-nested-field-single-eval-fixture-v0")
    require(values, "fixture", "nested_field_single_eval")
    require(values, "expected_nested_call_count", "1")
    require(values, "actual_nested_call_count", "1")
    require(values, "summary", "ok")

    lines = [
        "output_contract=mir-builder-nested-field-single-eval-owner-fix-v0",
        "input_contract=mir-builder-nested-field-single-eval-fixture-v0",
        "fixture=nested_field_single_eval",
        f"actual_nested_call_count={values['actual_nested_call_count']}",
        "owner_fix=field_access_inference_uses_published_origin_facts",
        "semantic_summary=ok",
        "generic_cse_added=0",
        "static_scalar_lowering_added=0",
        "selected_next=post_single_eval_fixes_measurement",
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
