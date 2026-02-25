#!/usr/bin/env python3
"""Phase 21.5 stability report helper.

Reads recorder ndjson files and optional app bench logs from OUT_DIR, prints
stability summary, and optionally writes guard baseline JSON.
"""

from __future__ import annotations

import json
import pathlib
import statistics
import sys
from datetime import datetime, timezone

from stability_app_log import (
    build_apps_baseline_fields,
    collect_app_aggregate,
)
from stability_entry_mode import (
    build_entry_mode_baseline_fields,
    collect_entry_mode_aggregate,
)


def fmt(value: float | int) -> str:
    if isinstance(value, int):
        return str(value)
    return f"{value:.2f}"


def series_stats(values: list[int]) -> tuple[int, float | int, int, float, float]:
    vals_sorted = sorted(values)
    med = statistics.median(vals_sorted)
    mn = vals_sorted[0]
    mx = vals_sorted[-1]
    stdev = statistics.pstdev(vals_sorted) if len(vals_sorted) > 1 else 0.0
    cv = (stdev / med) if med else 0.0
    return mn, med, mx, stdev, cv


def print_stats_line(prefix: str, values: list[int]) -> None:
    mn, med, mx, stdev, cv = series_stats(values)
    print(
        f"[stability] {prefix} "
        f"min={fmt(mn):>4s} med={fmt(med):>6s} max={fmt(mx):>4s} "
        f"stdev={fmt(stdev):>6s} cv={fmt(cv):>5s}"
    )


def median_int(values: list[int]) -> int:
    return int(statistics.median(sorted(values))) if values else 0


def main() -> int:
    if len(sys.argv) != 7:
        print(
            "usage: stability_report.py <out_dir> <write_baseline:0|1> "
            "<baseline_out> <rounds> <warmup> <repeat>",
            file=sys.stderr,
        )
        return 2

    out_dir = pathlib.Path(sys.argv[1])
    write_baseline = sys.argv[2] == "1"
    baseline_out = pathlib.Path(sys.argv[3])
    rounds = int(sys.argv[4])
    warmup = int(sys.argv[5])
    repeat = int(sys.argv[6])

    rows = []
    for ndjson in sorted(out_dir.glob("*.ndjson")):
        with ndjson.open() as f:
            for raw in f:
                line = raw.strip()
                if not line:
                    continue
                try:
                    rows.append(json.loads(line))
                except json.JSONDecodeError:
                    continue

    if not rows:
        print("[stability] no data")
        return 0

    metrics = ["c_ms", "py_ms", "ny_vm_ms", "ny_aot_ms"]
    by_bench: dict[str, list[dict]] = {}
    for row in rows:
        by_bench.setdefault(str(row.get("bench", "unknown")), []).append(row)

    for bench in sorted(by_bench):
        entries = by_bench[bench]
        for metric in metrics:
            vals = [int(e.get(metric, 0)) for e in entries]
            if not vals:
                continue
            print_stats_line(f"bench={bench:26s} metric={metric:8s}", vals)

    app_aggregate = collect_app_aggregate(out_dir)
    app_totals = app_aggregate["app_totals"]
    app_by_name = app_aggregate["app_by_name"]

    if app_totals:
        print_stats_line(f"apps={'total_vm_ms':>26s}", app_totals)
        for app_name in sorted(app_by_name):
            print_stats_line(f"app={app_name:27s} metric={'vm_ms':8s}", app_by_name[app_name])

    entry_aggregate = collect_entry_mode_aggregate(out_dir)
    entry_source_totals = entry_aggregate["source_totals"]
    entry_prebuilt_totals = entry_aggregate["prebuilt_totals"]
    entry_delta_abs_vals = entry_aggregate["delta_abs_vals"]
    if entry_source_totals and entry_prebuilt_totals:
        print_stats_line(f"entry_mode={'source_total_ms':>18s}", entry_source_totals)
        print_stats_line(f"entry_mode={'prebuilt_total_ms':>18s}", entry_prebuilt_totals)

    if entry_delta_abs_vals:
        print_stats_line(f"entry_mode={'delta_abs_ms':>18s}", entry_delta_abs_vals)

    if write_baseline:
        numeric_entries = by_bench.get("numeric_mixed_medium", [])
        numeric_aot_vals = [
            int(e.get("ny_aot_ms", 0))
            for e in numeric_entries
            if int(e.get("ny_aot_ms", 0)) > 0
        ]
        numeric_aot_ms = median_int(numeric_aot_vals)
        apps_baseline = build_apps_baseline_fields(app_aggregate)
        entry_baseline = build_entry_mode_baseline_fields(entry_aggregate)
        baseline = {
            "phase": "21.5",
            "generated_at": datetime.now(timezone.utc).isoformat(),
            "unit": "ms",
            "rounds": rounds,
            "warmup": warmup,
            "repeat": repeat,
            "numeric_mixed_medium_ny_aot_ms": numeric_aot_ms,
        }
        baseline.update(apps_baseline)
        baseline.update(entry_baseline)
        baseline_out.parent.mkdir(parents=True, exist_ok=True)
        baseline_out.write_text(json.dumps(baseline, indent=2) + "\n")
        print(f"[stability] baseline saved: {baseline_out}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
