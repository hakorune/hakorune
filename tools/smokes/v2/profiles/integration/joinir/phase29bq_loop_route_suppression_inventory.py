#!/usr/bin/env python3
"""Fail-closed loop route suppression inventory for phase29bq fixtures.

The tool runs every non-comment row in phase29bq_fast_gate_cases.tsv unless an
explicit --only case id is provided. It does not sample with head/tail and it
does not mask compiler failures: timeout or an rc outside the row's allowed_rc
set fails the inventory. Failures are reported through the same key-value
contract rather than a Python traceback so follow-up cards can consume the
first blocker directly.
"""

from __future__ import annotations

import argparse
import csv
import os
import re
import subprocess
import sys
from collections import Counter
from dataclasses import dataclass
from pathlib import Path


OBSERVER_RE = re.compile(
    r"\[plan/trace:loop_legacy_observer\].*legacy_suppressed=(\S+)"
)
SELECTED_RE = re.compile(r"\[plan/trace:loop_legacy_selected\] route=([^ \n]+)")


@dataclass(frozen=True)
class Case:
    fixture: str
    allowed_rc: frozenset[int]
    case_id: str


def repo_root() -> Path:
    return Path(__file__).resolve().parents[6]


def default_cases_path(root: Path) -> Path:
    return (
        root
        / "tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv"
    )


def default_bin(root: Path) -> Path:
    env_bin = os.environ.get("NYASH_BIN")
    if env_bin:
        return Path(env_bin)
    debug_bin = root / "target/debug/hakorune"
    if debug_bin.exists():
        return debug_bin
    return root / "target/release/hakorune"


def parse_allowed_rc(raw: str) -> frozenset[int]:
    values: set[int] = set()
    for part in raw.replace("|", ",").split(","):
        part = part.strip()
        if not part:
            continue
        values.add(int(part))
    if not values:
        raise ValueError(f"empty allowed_rc: {raw!r}")
    return frozenset(values)


def load_cases(path: Path, only: str | None) -> list[Case]:
    cases: list[Case] = []
    with path.open(newline="") as handle:
        reader = csv.reader(handle, delimiter="\t")
        for row in reader:
            if not row or row[0].startswith("#"):
                continue
            if len(row) < 5:
                raise ValueError(f"bad row in {path}: {row!r}")
            fixture, _expected, allowed_rc, _planner_tag, case_id = row[:5]
            if only and case_id != only:
                continue
            cases.append(Case(fixture, parse_allowed_rc(allowed_rc), case_id))
    if only and not cases:
        raise ValueError(f"case_id not found: {only}")
    return cases


def run_case(root: Path, bin_path: Path, case: Case, timeout_secs: int) -> str:
    env = os.environ.copy()
    env.update(
        {
            "NYASH_DISABLE_PLUGINS": "1",
            "NYASH_CLI_VERBOSE": "0",
            "NYASH_JOINIR_DEV": "1",
            "NYASH_VM_HAKO_PREFER_STRICT_DEV": "0",
            "NYASH_ALLOW_USING_FILE": "1",
            "NYASH_ENABLE_USING": "1",
            "HAKO_JOINIR_STRICT": "1",
            "HAKO_JOINIR_PLANNER_REQUIRED": "1",
            "HAKO_JOINIR_DEBUG": "1",
            "HAKO_DEBUG": "0",
            "HAKO_SHOW_CALL_LOGS": "0",
            "HAKO_SILENT_TAGS": "0",
        }
    )
    cmd = [str(bin_path), "--backend", "vm", case.fixture]
    try:
        proc = subprocess.run(
            cmd,
            cwd=root,
            env=env,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            timeout=timeout_secs,
            check=False,
        )
    except subprocess.TimeoutExpired as exc:
        raise RuntimeError(f"{case.case_id}: timeout after {timeout_secs}s") from exc
    if proc.returncode not in case.allowed_rc:
        tail = "\n".join(proc.stdout.splitlines()[-40:])
        raise RuntimeError(
            f"{case.case_id}: rc={proc.returncode} allowed={sorted(case.allowed_rc)}\n{tail}"
        )
    return proc.stdout


def inventory(cases: list[Case], root: Path, bin_path: Path, timeout_secs: int) -> int:
    selected_counts: Counter[str] = Counter()
    suppressed_counts: Counter[str] = Counter()
    observer_case_count = 0
    selected_case_count = 0
    suppressed_case_count = 0

    for case in cases:
        output = run_case(root, bin_path, case, timeout_secs)
        observer_values = OBSERVER_RE.findall(output)
        selected_values = SELECTED_RE.findall(output)
        if observer_values:
            observer_case_count += 1
        if selected_values:
            selected_case_count += 1
            selected_counts.update(selected_values)
        non_none_suppressed = [value for value in observer_values if value != "none"]
        if non_none_suppressed:
            suppressed_case_count += 1
            for value in non_none_suppressed:
                for route in value.split(","):
                    if route:
                        suppressed_counts[route] += 1
            print(
                f"[inventory/suppressed] case={case.case_id} values={','.join(non_none_suppressed)}"
            )

    print("output_contract=coreplan-loop-suppression-full-inventory-v0")
    print(f"case_count={len(cases)}")
    print(f"observer_case_count={observer_case_count}")
    print(f"actual_selected_case_count={selected_case_count}")
    print(f"suppressed_non_none_case_count={suppressed_case_count}")
    print(
        "actual_selected_route_counts="
        + ",".join(f"{route}:{count}" for route, count in sorted(selected_counts.items()))
        if selected_counts
        else "actual_selected_route_counts=none"
    )
    print(
        "suppressed_route_counts="
        + ",".join(f"{route}:{count}" for route, count in sorted(suppressed_counts.items()))
        if suppressed_counts
        else "suppressed_route_counts=none"
    )
    print("failure_masking=0")
    print("sampling_limit=none")
    print("summary=ok")
    return 0


def print_failure(error: Exception) -> int:
    message = str(error).replace("\n", "\\n")
    print("output_contract=coreplan-loop-suppression-full-inventory-v0")
    print("failure_masking=0")
    print("sampling_limit=none")
    print(f"failed_reason={message}")
    print("summary=failed")
    return 1


def main(argv: list[str]) -> int:
    root = repo_root()
    parser = argparse.ArgumentParser()
    parser.add_argument("--cases", type=Path, default=default_cases_path(root))
    parser.add_argument("--bin", type=Path, default=default_bin(root))
    parser.add_argument("--timeout", type=int, default=int(os.environ.get("RUN_TIMEOUT_SECS", "10")))
    parser.add_argument("--only", help="Run a single phase29bq case_id")
    args = parser.parse_args(argv)

    if not args.bin.exists():
        raise SystemExit(f"missing hakorune binary: {args.bin}")
    cases = load_cases(args.cases, args.only)
    try:
        return inventory(cases, root, args.bin, args.timeout)
    except RuntimeError as exc:
        return print_failure(exc)


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
