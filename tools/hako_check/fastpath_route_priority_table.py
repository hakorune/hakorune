#!/usr/bin/env python3
"""Emit the static FastPath route priority table.

This is an observation surface. It does not read MIR, select routes, change
backend behavior, or make performance claims.
"""

from __future__ import annotations

import argparse
import json
from dataclasses import asdict
from typing import Any

from fastpath_route_priority import ROUTE_PRIORITIES


def build_report() -> dict[str, Any]:
    rows = [asdict(row) for row in ROUTE_PRIORITIES]
    priorities = [row["priority"] for row in rows]
    return {
        "output_contract": "hako-fastpath-route-priority-table-v0",
        "route_priority_table_version": "v0",
        "entry_count": str(len(rows)),
        "priority_unique": "1" if len(set(priorities)) == len(priorities) else "0",
        "lowest_priority_wins": "1",
        "route_priority_changes_backend_lowering": "0",
        "route_priority_retires_exact_seed": "0",
        "summary": "ok",
        "entries": rows,
    }


def emit_kv(report: dict[str, Any]) -> None:
    for key, value in report.items():
        if key == "entries":
            continue
        print(f"{key}={value}")
    entries = report.get("entries")
    if isinstance(entries, list):
        for idx, entry in enumerate(entries):
            if not isinstance(entry, dict):
                continue
            for key, value in entry.items():
                print(f"entry_{idx}_{key}={value}")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--format", choices=("kv", "json"), default="kv")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    report = build_report()
    if args.format == "json":
        print(json.dumps(report, indent=2, sort_keys=True))
    else:
        emit_kv(report)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
