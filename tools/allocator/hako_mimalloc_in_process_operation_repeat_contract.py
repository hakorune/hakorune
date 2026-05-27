#!/usr/bin/env python3
"""Emit the hako mimalloc in-process operation repeat measurement contract."""

from __future__ import annotations

import argparse
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--workload", default="representative-small-block-v0")
    parser.add_argument("--operation-repeat", type=int, default=8192)
    parser.add_argument("--process-repeat", type=int, default=3)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    if args.operation_repeat < 1:
        raise SystemExit("--operation-repeat must be positive")
    if args.process_repeat < 1:
        raise SystemExit("--process-repeat must be positive")

    lines = [
        "output_contract=hako-mimalloc-in-process-operation-repeat-contract-v0",
        "measurement_profile=hako-mimalloc-in-process-operation-repeat-v0",
        "timing_repeat_kind=in-process-operation-loop-v0",
        "process_repeat_kind=sample-process-count-v0",
        f"workload_id={args.workload}",
        f"operation_repeat={args.operation_repeat}",
        f"process_repeat={args.process_repeat}",
        f"sample_count={args.process_repeat}",
        "build_compile_excluded=1",
        "same_workload=1",
        "same_operation_count=1",
        "process_invocation_repeat=0",
        "winner_claim=0",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
