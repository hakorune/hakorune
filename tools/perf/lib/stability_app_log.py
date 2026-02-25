#!/usr/bin/env python3
"""App wallclock stability aggregation helpers for phase21.5."""

from __future__ import annotations

import json
import pathlib
import re
import statistics
from typing import Any


def parse_app_log(path: pathlib.Path) -> tuple[int, dict[str, int]]:
    text = path.read_text()

    # Preferred format: one-line JSON emitted by bench_apps_wallclock.sh
    for raw in text.splitlines():
        line = raw.strip()
        if not line or not line.startswith("{"):
            continue
        try:
            payload = json.loads(line)
        except json.JSONDecodeError:
            continue
        if not isinstance(payload, dict):
            continue
        total = int(payload.get("total_ms", 0) or 0)
        cases_raw = payload.get("cases", {})
        if total > 0 and isinstance(cases_raw, dict):
            cases = {}
            for key, value in cases_raw.items():
                try:
                    iv = int(value)
                except (TypeError, ValueError):
                    continue
                if iv > 0:
                    cases[str(key)] = iv
            if cases:
                return total, cases

    # Backward-compatible fallback: parse text lines.
    total = 0
    cases: dict[str, int] = {}
    app_pat = re.compile(r"name=([^\s]+)\s+backend=\S+\s+ms=(\d+)")
    for line in text.splitlines():
        m = app_pat.search(line)
        if not m:
            continue
        name = m.group(1)
        ms = int(m.group(2))
        total += ms
        cases[name] = ms
    return total, cases


def collect_app_aggregate(out_dir: pathlib.Path) -> dict[str, Any]:
    app_totals: list[int] = []
    app_by_name: dict[str, list[int]] = {}
    for app_log in sorted(out_dir.glob("apps_vm.round*.log")):
        total, per_app = parse_app_log(app_log)
        if total > 0:
            app_totals.append(total)
        for app_name, app_ms in per_app.items():
            app_by_name.setdefault(app_name, []).append(app_ms)
    return {
        "app_totals": app_totals,
        "app_by_name": app_by_name,
    }


def build_apps_baseline_fields(aggregate: dict[str, Any]) -> dict[str, Any]:
    app_totals = aggregate["app_totals"]
    app_by_name = aggregate["app_by_name"]
    apps_total_ms = int(statistics.median(sorted(app_totals))) if app_totals else 0
    apps_per_app_ms = {
        app_name: int(statistics.median(sorted(vals)))
        for app_name, vals in app_by_name.items()
        if vals
    }
    return {
        "apps_vm_total_ms": apps_total_ms,
        "apps_vm_per_app_ms": apps_per_app_ms,
    }
