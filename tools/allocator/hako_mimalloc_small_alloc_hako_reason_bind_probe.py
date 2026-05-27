#!/usr/bin/env python3
"""Compare before/after reason-call shape for a temporary .hako reason bind probe."""

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


def require_int(values: dict[str, str], key: str) -> int:
    if key not in values:
        raise SystemExit(f"missing report key: {key}")
    try:
        return int(values[key])
    except ValueError as exc:
        raise SystemExit(f"report key must be integer: {key}") from exc


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--before-report", type=Path, required=True)
    parser.add_argument("--after-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    before = parse_report(args.before_report)
    after = parse_report(args.after_report)

    selected_owner = before.get(
        "selected_owner", "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
    )
    before_reason_call_count = require_int(before, "reason_call_count")
    before_duplicate_reason_call_count = require_int(before, "duplicate_reason_call_count")
    after_reason_call_count = require_int(after, "reason_call_count")
    after_duplicate_reason_call_count = require_int(after, "duplicate_reason_call_count")

    if before_duplicate_reason_call_count and after_duplicate_reason_call_count == 0:
        next_action = "apply_hako_reason_bind_keeper"
        selected_reason = "temporary_hako_reason_bind_removed_duplicate_reason_calls"
    elif after_duplicate_reason_call_count:
        next_action = "reason_singleton_lowering_probe"
        selected_reason = "hako_reason_bind_did_not_remove_duplicate_reason_calls"
    else:
        next_action = "stop_line"
        selected_reason = "no_before_duplicate_reason_calls"

    lines = [
        "output_contract=hako-mimalloc-small-alloc-hako-reason-bind-probe-v0",
        "input_contract=hako-mimalloc-small-alloc-duplicate-reason-call-probe-v0",
        f"selected_owner={selected_owner}",
        f"before_reason_call_count={before_reason_call_count}",
        f"before_duplicate_reason_call_count={before_duplicate_reason_call_count}",
        f"after_reason_call_count={after_reason_call_count}",
        f"after_duplicate_reason_call_count={after_duplicate_reason_call_count}",
        f"reason_call_delta={after_reason_call_count - before_reason_call_count}",
        f"duplicate_reason_call_delta={after_duplicate_reason_call_count - before_duplicate_reason_call_count}",
        f"selected_reason={selected_reason}",
        f"next_action={next_action}",
        "next_diagnostic=small_alloc_hako_reason_bind_keeper",
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
