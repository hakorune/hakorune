#!/usr/bin/env python3
"""Validate and atomically publish fixed-control S6C meso evidence."""

import argparse
import csv
import hashlib
import json
import math
import os
from pathlib import Path
import statistics
import sys


FAMILIES = ("ascii", "width2", "width3", "width4", "mixed")
SIZES = (32, 256, 4096, 1048576)
POSITIONS = ("first", "middle", "last", "miss")
CASES = {(family, size, position) for family in FAMILIES for size in SIZES for position in POSITIONS}


def fail(message: str) -> None:
    raise ValueError(message)


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def nearest_rank(values: list[float], percentile: float) -> float:
    ordered = sorted(values)
    return ordered[math.ceil(percentile * len(ordered)) - 1]


def read_samples(path: Path) -> tuple[list[dict[str, object]], dict[str, object]]:
    grouped: dict[tuple[str, int, str], list[float]] = {case: [] for case in CASES}
    shapes: dict[tuple[str, int, str], tuple[int, ...]] = {}
    raw: list[dict[str, object]] = []
    required = {
        "family", "size", "position", "sample", "iterations", "hako_ns", "c_ns",
        "sink", "scalars", "width1", "width2", "width3", "width4",
    }
    with path.open(newline="") as stream:
        rows = csv.DictReader(stream)
        if set(rows.fieldnames or ()) != required:
            fail("meso CSV header drift")
        for row in rows:
            case = (row["family"], int(row["size"]), row["position"])
            if case not in CASES:
                fail(f"unexpected meso case: {case}")
            sample = int(row["sample"])
            iterations = int(row["iterations"])
            hako_ns, c_ns, sink = int(row["hako_ns"]), int(row["c_ns"]), int(row["sink"])
            shape = tuple(int(row[name]) for name in ("scalars", "width1", "width2", "width3", "width4"))
            if sample != len(grouped[case]) or iterations <= 0 or sink == 0:
                fail(f"invalid sample sequence for {case}")
            if hako_ns < 20_000_000 or c_ns < 20_000_000:
                fail(f"uncalibrated arm for {case}")
            if sum((index + 1) * shape[index + 1] for index in range(4)) != case[1] or sum(shape[1:]) != shape[0]:
                fail(f"UTF-8 histogram/size drift for {case}")
            if case in shapes and shapes[case] != shape:
                fail(f"case shape changed between samples: {case}")
            shapes[case] = shape
            ratio = hako_ns / c_ns
            if not math.isfinite(ratio) or ratio <= 0:
                fail(f"invalid ratio for {case}")
            grouped[case].append(ratio)
            raw.append({"family": case[0], "size": case[1], "position": case[2],
                        "sample": sample, "iterations": iterations, "hako_ns": hako_ns,
                        "c_ns": c_ns, "ratio": ratio})
    if any(len(samples) != 51 for samples in grouped.values()):
        fail("every meso case must have 51 paired samples")
    case_stats: dict[str, object] = {}
    gated_p50: list[tuple[str, float]] = []
    all_p95: list[float] = []
    for case in sorted(grouped):
        samples = grouped[case]
        p50, p95 = statistics.median(samples), nearest_rank(samples, 0.95)
        key = f"{case[0]}/{case[1]}/{case[2]}"
        case_stats[key] = {"p50": p50, "p95": p95, "shape": shapes[case]}
        all_p95.append(p95)
        if case[1] >= 4096:
            gated_p50.append((key, p50))
    worst_case, maximum = max(gated_p50, key=lambda item: item[1])
    if maximum > 1.15:
        fail(f"4KiB+ meso p50 threshold red: {worst_case}={maximum:.6f}")
    return raw, {"cases": case_stats, "summary": {"gated_4k_plus_max_p50": maximum,
                                                    "informational_all_max_p95": max(all_p95)}}


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--csv", required=True, type=Path)
    parser.add_argument("--outline-manifest", required=True, type=Path)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--report", required=True, type=Path)
    parser.add_argument("--commit", required=True)
    parser.add_argument("--cpu", required=True)
    parser.add_argument("--toolchain", required=True)
    args = parser.parse_args()
    temporary = args.report.with_name(args.report.name + ".tmp")
    try:
        outline = json.loads(args.outline_manifest.read_text())
        if outline.get("schema") != "s6c-pinned-corridor-meso-outline-evidence-v1":
            fail("outline evidence schema drift")
        raw, stats = read_samples(args.csv)
        report = {
            "schema": "s6c-pinned-corridor-meso-bench-evidence-v1",
            "authority": "promotion-evidence-only",
            "commit": args.commit,
            "environment": {"cpu": args.cpu, "toolchain": args.toolchain, "kernel": os.uname().release},
            "thresholds": {"gated_sizes_min_bytes": 4096, "max_case_p50": 1.15},
            "outline_graph_sha256": outline["retained_graph_sha256"],
            "digests": {"csv_sha256": digest(args.csv), "binary_sha256": digest(args.binary),
                        "outline_manifest_sha256": digest(args.outline_manifest)},
            **stats,
            "samples": raw,
        }
        args.report.parent.mkdir(parents=True, exist_ok=True)
        temporary.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
        temporary.replace(args.report)
    except (OSError, ValueError, KeyError, ZeroDivisionError, json.JSONDecodeError) as error:
        args.report.unlink(missing_ok=True)
        temporary.unlink(missing_ok=True)
        print(f"[s6c-pinned-corridor-meso-bench] ERROR: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
