#!/usr/bin/env python3
"""Adapt hakmem hakozuna_compare logs into phase-295x-style KV evidence."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


HEADER_RE = re.compile(r"^\[BENCH-HEADER\] ts=(\S+) git=(\S+) label=(\S+)")
ENV_RE = re.compile(r"^\[BENCH-ENV\] iters=(\d+) ws=(\d+) runs=(\d+)")
RSS_RE = re.compile(r"^\[RSS\] max_kb=(\d+)")
ALLOC_RE = re.compile(r"^\[ALLOCATOR\] (\S+)")
THROUGHPUT_RE = re.compile(
    r"^Throughput = (\d+) ops/s \[allocator=(\S+)\] \[iter=(\d+) ws=(\d+)\] time=([0-9.]+)s"
)


def fail(message: str) -> None:
    raise SystemExit(f"[hakmem-hakozuna-log-adapter] {message}")


def normalize_allocator(name: str) -> str:
    return {"sys": "system"}.get(name, name)


def median_int(values: list[int]) -> int:
    ordered = sorted(values)
    if not ordered:
        return 0
    mid = len(ordered) // 2
    if len(ordered) % 2:
        return ordered[mid]
    return (ordered[mid - 1] + ordered[mid]) // 2


def parse_log(path: Path) -> dict[str, object]:
    meta: dict[str, str] = {}
    runs: list[dict[str, str]] = []
    last_rss_kb: str | None = None
    last_allocator: str | None = None

    for raw in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = raw.strip()
        if not line:
            continue
        if match := HEADER_RE.match(line):
            meta["timestamp"] = match.group(1)
            meta["git"] = match.group(2)
            meta["label"] = normalize_allocator(match.group(3))
            meta["label_raw"] = match.group(3)
            continue
        if match := ENV_RE.match(line):
            meta["iters"] = match.group(1)
            meta["working_set"] = match.group(2)
            meta["declared_runs"] = match.group(3)
            continue
        if line.startswith("[BENCH-CMD] "):
            meta["command_present"] = "1"
            continue
        if match := RSS_RE.match(line):
            last_rss_kb = match.group(1)
            continue
        if match := ALLOC_RE.match(line):
            last_allocator = normalize_allocator(match.group(1))
            continue
        if match := THROUGHPUT_RE.match(line):
            throughput = match.group(1)
            allocator = normalize_allocator(match.group(2))
            iterations = match.group(3)
            working_set = match.group(4)
            elapsed_ms = str(int(round(float(match.group(5)) * 1000.0)))
            rss_kb = last_rss_kb or "0"
            runs.append(
                {
                    "allocator": allocator,
                    "allocator_rss_line": last_allocator or "",
                    "iterations": iterations,
                    "working_set": working_set,
                    "throughput_ops_per_sec": throughput,
                    "elapsed_ms": elapsed_ms,
                    "peak_rss_bytes": str(int(rss_kb) * 1024),
                    "rss_kb": rss_kb,
                }
            )
            last_rss_kb = None
            last_allocator = None

    return {"meta": meta, "runs": runs}


def emit(parsed: dict[str, object], source: Path) -> str:
    meta = parsed["meta"]  # type: ignore[assignment]
    runs = parsed["runs"]  # type: ignore[assignment]
    if not isinstance(meta, dict) or not isinstance(runs, list):
        fail("internal parser shape error")
    throughputs = [int(row["throughput_ops_per_sec"]) for row in runs]
    elapsed = [int(row["elapsed_ms"]) for row in runs]
    rss = [int(row["peak_rss_bytes"]) for row in runs]
    allocators = sorted({str(row["allocator"]) for row in runs})

    lines = [
        "output_contract=hakmem-external-hakozuna-compare-log-adapter-v0",
        "dataset_role=external-historical-benchmark-corpus",
        f"source_path={source}",
        f"timestamp={meta.get('timestamp', '')}",
        f"git={meta.get('git', '')}",
        f"label={meta.get('label', '')}",
        f"label_raw={meta.get('label_raw', '')}",
        f"iterations={meta.get('iters', '')}",
        f"working_set={meta.get('working_set', '')}",
        f"declared_runs={meta.get('declared_runs', '')}",
        f"run_count={len(runs)}",
        "allocators=" + ",".join(allocators),
        f"throughput_min_ops_per_sec={min(throughputs) if throughputs else 0}",
        f"throughput_median_ops_per_sec={median_int(throughputs)}",
        f"throughput_max_ops_per_sec={max(throughputs) if throughputs else 0}",
        f"elapsed_min_ms={min(elapsed) if elapsed else 0}",
        f"elapsed_median_ms={median_int(elapsed)}",
        f"elapsed_max_ms={max(elapsed) if elapsed else 0}",
        f"peak_rss_min_bytes={min(rss) if rss else 0}",
        f"peak_rss_median_bytes={median_int(rss)}",
        f"peak_rss_max_bytes={max(rss) if rss else 0}",
        "winner_claim=0",
        "provider_activation=0",
        "host_replacement=0",
        "hook_installed=0",
        "global_allocator_installed=0",
    ]
    for idx, row in enumerate(runs[:32]):
        prefix = f"run_{idx}"
        for key in [
            "allocator",
            "iterations",
            "working_set",
            "throughput_ops_per_sec",
            "elapsed_ms",
            "peak_rss_bytes",
        ]:
            lines.append(f"{prefix}_{key}={row[key]}")
    lines.append("summary=ok")
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--in", dest="input", type=Path, required=True)
    parser.add_argument("--out", type=Path, default=None)
    args = parser.parse_args()

    source = args.input.resolve()
    if not source.exists():
        fail(f"missing input: {source}")
    report = emit(parse_log(source), source)
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
