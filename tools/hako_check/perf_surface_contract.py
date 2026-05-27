#!/usr/bin/env python3
"""Emit the hako_check perf-surface report contract."""

from __future__ import annotations

import argparse
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    lines = [
        "output_contract=hako-check-perf-surface-contract-v0",
        "tool_surface=hako_check_perf_surface",
        "observation_only=1",
        "rewrite_executed=0",
        "target_file=<repo-relative .hako path>",
        "target_box=<static box name>",
        "target_method=<method name>",
        "method_call_count=<int>",
        "loop_method_call_count=<int>",
        "array_access_count=<int>",
        "linear_search_candidate=0|1",
        "result_capsule_churn=0|1",
        "observer_call_count=<int>",
        "hot_path_risk=low|medium|high",
        "suggested_next=<single keeper candidate>",
        "winner_claim=0",
        "replacement_active=0",
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
