#!/usr/bin/env python3
"""Validate and atomically publish S6C exact-leaf benchmark evidence."""

import argparse
import csv
import hashlib
import json
import math
import os
from pathlib import Path
import statistics
import sys


CASES = {
    f"w{width}-{outcome}": ("ascii" if width == 1 else "mixed")
    for width in range(1, 5)
    for outcome in ("equal", "first-mismatch", "last-mismatch", "length-mismatch", "alias")
}


def fail(message: str) -> None:
    raise ValueError(message)


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def nearest_rank(values: list[float], percentile: float) -> float:
    ordered = sorted(values)
    return ordered[math.ceil(percentile * len(ordered)) - 1]


def validate_ir(path: Path) -> dict[str, int]:
    text = path.read_text()
    forbidden = (" call ", " invoke ", " landingpad ", " noalias ", " memcmp")
    if any(token in text for token in forbidden):
        fail("exact leaf IR contains a forbidden operation")
    if text.count("define i1 @hako_s6c_exact_leaf(") != 1:
        fail("exact leaf IR must contain one evidence callable")
    for width in range(1, 5):
        for byte in range(width):
            load = f"%ptfc_l{width}_{byte}_"
            if text.count(load) != 2:
                fail(f"width {width} byte {byte} load/compare census drift")
            if byte:
                label = f"label %ptfc_eq_w{width}_b{byte}_"
                predecessor = f"%ptfc_c{width}_{byte - 1}_"
                lines = [line for line in text.splitlines() if label in line]
                if len(lines) != 1 or predecessor not in lines[0] or "br i1" not in lines[0]:
                    fail(f"width {width} byte {byte} is not equality-short-circuited")
    return {"align1_loads": text.count("load i8, ptr"), "calls": text.count(" call ")}


def read_samples(path: Path) -> tuple[list[dict[str, object]], dict[str, dict[str, float]]]:
    grouped: dict[str, list[float]] = {name: [] for name in CASES}
    raw: list[dict[str, object]] = []
    with path.open(newline="") as stream:
        rows = csv.DictReader(stream)
        required = {"case", "category", "sample", "iterations", "hako_ns", "c_ns", "sink"}
        if set(rows.fieldnames or ()) != required:
            fail("benchmark CSV header drift")
        for row in rows:
            name = row["case"]
            if name not in CASES or row["category"] != CASES[name]:
                fail("benchmark corpus/category drift")
            sample = int(row["sample"])
            iterations = int(row["iterations"])
            hako_ns = int(row["hako_ns"])
            c_ns = int(row["c_ns"])
            sink = int(row["sink"])
            if sample != len(grouped[name]) or iterations <= 0 or sink <= 0:
                fail(f"invalid sample sequence for {name}")
            if hako_ns < 20_000_000 or c_ns < 20_000_000:
                fail(f"uncalibrated sample for {name}")
            ratio = hako_ns / c_ns
            if not math.isfinite(ratio) or ratio <= 0:
                fail(f"invalid ratio for {name}")
            grouped[name].append(ratio)
            raw.append({
                "case": name, "sample": sample, "iterations": iterations,
                "hako_ns": hako_ns, "c_ns": c_ns, "ratio": ratio,
            })
    if any(len(values) != 51 for values in grouped.values()):
        fail("every exact case must have 51 paired samples")
    stats = {
        name: {"p50": statistics.median(values), "p95": nearest_rank(values, 0.95)}
        for name, values in grouped.items()
    }
    ascii_p50 = max(stats[name]["p50"] for name, category in CASES.items() if category == "ascii")
    mixed_p50 = max(stats[name]["p50"] for name, category in CASES.items() if category == "mixed")
    all_p95 = max(item["p95"] for item in stats.values())
    if ascii_p50 > 1.10 or mixed_p50 > 1.15 or all_p95 > 1.30:
        fail(f"threshold red: ascii={ascii_p50:.6f} mixed={mixed_p50:.6f} p95={all_p95:.6f}")
    stats["summary"] = {"ascii_max_p50": ascii_p50, "mixed_max_p50": mixed_p50, "all_max_p95": all_p95}
    return raw, stats


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--csv", required=True, type=Path)
    parser.add_argument("--ir", required=True, type=Path)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--report", required=True, type=Path)
    parser.add_argument("--commit", required=True)
    parser.add_argument("--cpu", required=True)
    parser.add_argument("--toolchain", required=True)
    args = parser.parse_args()
    try:
        ir = validate_ir(args.ir)
        raw, stats = read_samples(args.csv)
        evidence = {
            "schema": "s6c-pinned-corridor-exact-bench-evidence-v1",
            "authority": "promotion-evidence-only",
            "commit": args.commit,
            "environment": {
                "cpu": args.cpu,
                "toolchain": args.toolchain,
                "kernel": os.uname().release,
            },
            "thresholds": {"ascii_max_p50": 1.10, "mixed_max_p50": 1.15, "all_max_p95": 1.30},
            "digests": {"csv_sha256": digest(args.csv), "ir_sha256": digest(args.ir), "binary_sha256": digest(args.binary)},
            "ir": ir,
            "cases": stats,
            "samples": raw,
        }
        args.report.parent.mkdir(parents=True, exist_ok=True)
        temporary = args.report.with_name(args.report.name + ".tmp")
        temporary.write_text(json.dumps(evidence, indent=2, sort_keys=True) + "\n")
        temporary.replace(args.report)
    except (OSError, ValueError, ZeroDivisionError) as error:
        args.report.unlink(missing_ok=True)
        args.report.with_name(args.report.name + ".tmp").unlink(missing_ok=True)
        print(f"exact benchmark rejected: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
