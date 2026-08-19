#!/usr/bin/env python3
"""Run one sealed retain-all S6C wall-clock session and publish its receipt."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys

from s6c_paired_wallclock_plan import (
    CANONICAL_CASES, aggregate_development, seal_plan, validate_session,
)


FIELDS = (
    "case", "slot", "block", "block_slot", "order", "attempt", "oracle_equal",
    "family", "size", "position", "sample", "iterations", "hako_ns", "c_ns",
    "sink", "scalars", "width1", "width2", "width3", "width4",
)


class HarnessError(RuntimeError):
    pass


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def orders_text(plan: dict[str, object], case: str) -> str:
    return "".join("A" if order == "AB" else "B" for order in plan["schedules"][case])


def parse_case_output(text: str, expected_case: str) -> list[dict[str, object]]:
    reader = csv.DictReader(text.splitlines())
    if tuple(reader.fieldnames or ()) != FIELDS:
        raise HarnessError("robust case CSV header drift")
    rows, expected_shape, expected_iterations = [], None, None
    for row in reader:
        if row["case"] != expected_case or row["family"] + "/" + row["size"] + "/" + \
                row["position"] != expected_case:
            raise HarnessError("case identity drift")
        slot, size = int(row["slot"]), int(row["size"])
        shape = tuple(int(row[name]) for name in ("scalars", "width1", "width2", "width3", "width4"))
        iterations, sink = int(row["iterations"]), int(row["sink"])
        if int(row["sample"]) != slot or iterations <= 0 or sink == 0 or \
                sum((width + 1) * shape[width + 1] for width in range(4)) != size or \
                sum(shape[1:]) != shape[0]:
            raise HarnessError("sample/iteration/sink/UTF-8 shape drift")
        if expected_shape is not None and (shape != expected_shape or iterations != expected_iterations):
            raise HarnessError("case shape or calibrated iteration drift")
        expected_shape, expected_iterations = shape, iterations
        rows.append({"case": row["case"], "slot": slot,
                     "block": int(row["block"]), "block_slot": int(row["block_slot"]),
                     "order": row["order"], "attempt": int(row["attempt"]),
                     "oracle_equal": row["oracle_equal"] == "true",
                     "hako_ns": int(row["hako_ns"]), "c_ns": int(row["c_ns"])})
    if len(rows) != 51:
        raise HarnessError("robust case did not emit exactly 51 pairs")
    return rows


def run(binary: Path, plan: dict[str, object], cpu: int) -> tuple[list[dict[str, object]], str]:
    rows, raw_parts = [], []
    for case in plan["cases"]:
        family, size, position = case.split("/")
        command = ["taskset", "-c", str(cpu), str(binary), "--robust-case",
                   family, size, position, orders_text(plan, case)]
        process = subprocess.run(command, text=True, stdout=subprocess.PIPE,
                                 stderr=subprocess.PIPE, timeout=900)
        if process.returncode:
            raise HarnessError(f"robust case process rejected: {case}")
        rows.extend(parse_case_output(process.stdout, case))
        raw_parts.append(process.stdout)
    return rows, "".join(raw_parts)


def atomic_write(path: Path, payload: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(path.name + ".tmp")
    try:
        temporary.write_text(payload)
        os.replace(temporary, path)
    except Exception:
        temporary.unlink(missing_ok=True)
        raise


def self_test() -> None:
    plan = seal_plan(commit="a" * 40, binary_sha256="b" * 64,
                     cases=["mixed/4096/first"], environment_class="wsl_development")
    header = ",".join(FIELDS) + "\n"
    body = []
    for slot, order in enumerate(plan["schedules"]["mixed/4096/first"]):
        body.append(f"mixed/4096/first,{slot},{slot // 17},{slot % 17},{order},1,true,"
                    f"mixed,4096,first,{slot},1,40000000,40000000,1,1642,415,409,409,409\n")
    rows = parse_case_output(header + "".join(body), "mixed/4096/first")
    assert validate_session(plan, rows)["outcome"] == "development_green"
    try:
        parse_case_output(header + "".join(body[:-1]), "mixed/4096/first")
    except HarnessError:
        pass
    else:
        raise AssertionError("short robust output accepted")
    assert len(orders_text(plan, "mixed/4096/first")) == 51


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path)
    parser.add_argument("--commit")
    parser.add_argument("--cpu", type=int)
    parser.add_argument("--environment-class", choices=("wsl_development",))
    parser.add_argument("--session-index", type=int, choices=(0, 1))
    parser.add_argument("--report", type=Path)
    parser.add_argument("--aggregate-reports", nargs=2, type=Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        self_test()
        print("[s6c-paired-wallclock-harness] self-test ok")
        return 0
    if args.aggregate_reports:
        if args.report is None:
            parser.error("aggregation requires --report")
        try:
            receipts = [json.loads(path.read_text()) for path in args.aggregate_reports]
            atomic_write(args.report, json.dumps(
                aggregate_development(receipts), indent=2, sort_keys=True) + "\n")
            return 0
        except (OSError, ValueError, KeyError, json.JSONDecodeError) as error:
            args.report.unlink(missing_ok=True)
            args.report.with_name(args.report.name + ".tmp").unlink(missing_ok=True)
            print(f"[s6c-paired-wallclock-harness] NoSafeSlice: {error}", file=sys.stderr)
            return 1
    if any(value is None for value in (
            args.binary, args.commit, args.cpu, args.environment_class,
            args.session_index, args.report)):
        parser.error("session requires binary, commit, cpu, environment class, index and report")
    if not args.binary.is_file() or len(args.commit) != 40:
        print("[s6c-paired-wallclock-harness] NoSafeSlice: identity missing", file=sys.stderr)
        return 1
    try:
        root = Path(__file__).resolve().parents[2]
        head = subprocess.check_output(
            ["git", "-C", str(root), "rev-parse", "HEAD"], text=True).strip()
        if head != args.commit or subprocess.run(
                ["git", "-C", str(root), "diff", "--quiet"]).returncode or \
                subprocess.run(["git", "-C", str(root), "diff", "--cached", "--quiet"]).returncode:
            raise HarnessError("source commit/worktree identity drift")
        is_wsl = "microsoft" in os.uname().release.lower()
        if (args.environment_class == "wsl_development") != is_wsl:
            raise HarnessError("environment authority class drift")
        plan = seal_plan(commit=args.commit, binary_sha256=sha256(args.binary),
                         cases=CANONICAL_CASES, environment_class=args.environment_class,
                         session_index=args.session_index)
        rows, raw_csv = run(args.binary, plan, args.cpu)
        receipt = {"schema": "s6c-meso-paired-wallclock-receipt-v1", "plan": plan,
                   "session": validate_session(plan, rows), "raw_csv_sha256":
                   hashlib.sha256(raw_csv.encode()).hexdigest()}
        atomic_write(args.report, json.dumps(receipt, indent=2, sort_keys=True) + "\n")
        return 0
    except (OSError, ValueError, HarnessError, subprocess.TimeoutExpired) as error:
        args.report.unlink(missing_ok=True)
        args.report.with_name(args.report.name + ".tmp").unlink(missing_ok=True)
        print(f"[s6c-paired-wallclock-harness] NoSafeSlice: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
