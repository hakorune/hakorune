#!/usr/bin/env python3
"""Validate and atomically publish S6C exact-leaf benchmark evidence."""

import argparse
import csv
import hashlib
import json
import math
import os
from pathlib import Path
import re
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
    if " switch " in text:
        fail("exact leaf IR must not dispatch equality through a width switch")
    cached = re.findall(r"%(ptfc_byte_(\d+)) = load i8, ptr %ptfc_byte_ptr_\2, align 1", text)
    if len(cached) != 1:
        fail("exact leaf IR must contain one cached WidthAt lead-byte load")
    width_id = cached[0][1]
    lead_compare = re.findall(
        rf"%ptfc_c0_(\d+) = icmp eq i8 %ptfc_byte_{width_id}, %ptfc_r0_\1",
        text,
    )
    if len(lead_compare) != 1:
        fail("scalar equality must consume the cached WidthAt lead byte once")
    dst = lead_compare[0]
    lead_lines = [
        line for line in text.splitlines()
        if f"label %ptfc_eq_after_b0_{dst}" in line
    ]
    if len(lead_lines) != 1 or f"%ptfc_c0_{dst}" not in lead_lines[0] or "br i1" not in lead_lines[0]:
        fail("byte 1 must remain behind the cached lead-byte equality")
    for byte in range(1, 4):
        if text.count(f"%ptfc_l{byte}_{dst}") != 2:
            fail(f"byte {byte} load/compare census drift")
        label = f"label %ptfc_eq_b{byte}_{dst}"
        lines = [line for line in text.splitlines() if label in line]
        done_value = f"%ptfc_done_{byte}_{dst}"
        if len(lines) != 1 or done_value not in lines[0] or "br i1" not in lines[0]:
            fail(f"byte {byte} is not width-short-circuited")
        done = f"%ptfc_done_{byte}_{dst} = icmp eq i64 %r{width_id}, {byte}"
        if text.count(done) != 1:
            fail(f"width {byte} direct-ladder stop is missing")
        if byte > 1:
            predecessor = f"%ptfc_c{byte - 1}_{dst}"
            after = f"label %ptfc_eq_after_b{byte - 1}_{dst}"
            prior_lines = [line for line in text.splitlines() if after in line]
            if len(prior_lines) != 1 or predecessor not in prior_lines[0] or "br i1" not in prior_lines[0]:
                fail(f"byte {byte} is not equality-short-circuited")
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
