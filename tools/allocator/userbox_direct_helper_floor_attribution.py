#!/usr/bin/env python3
"""Measure userbox typed-object floor/helper lanes with startup attribution."""

from __future__ import annotations

import argparse
import os
import re
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
LANES = ROOT / "tools/perf/bench_micro_c_vs_aot_lanes.sh"

CASES: tuple[tuple[str, str], ...] = (
    ("counter_step_chain", "kilo_micro_userbox_counter_step_chain"),
    ("point_add", "kilo_micro_userbox_point_add"),
)


def parse_micro_lanes(stdout: str) -> dict[str, str]:
    line = ""
    for candidate in stdout.splitlines():
        if candidate.startswith("[micro-lanes] "):
            line = candidate
    if not line:
        raise ValueError("missing [micro-lanes] output")

    values: dict[str, str] = {}
    for match in re.finditer(r"(\S+)=([^ ]+)", line):
        values[match.group(1)] = match.group(2)
    return values


def run_case(
    bench_key: str,
    *,
    helper: bool,
    warmup: int,
    repeat: int,
    kernel_inner_runs: int,
) -> tuple[int, dict[str, str], str]:
    env = os.environ.copy()
    env["HAKO_TYPED_OBJECT_STORE"] = "single_thread_exact"
    env.pop("HAKO_ARRAY_SLOT_STORE", None)
    if helper:
        env["HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER"] = "1"
    else:
        env.pop("HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER", None)

    proc = subprocess.run(
        [
            "bash",
            str(LANES),
            bench_key,
            str(warmup),
            str(repeat),
            str(kernel_inner_runs),
        ],
        cwd=ROOT,
        env=env,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        check=False,
    )
    if proc.returncode != 0:
        return proc.returncode, {}, proc.stdout
    try:
        return proc.returncode, parse_micro_lanes(proc.stdout), proc.stdout
    except ValueError:
        return 1, {}, proc.stdout


def status(values: dict[str, str], rc: int) -> str:
    if rc != 0:
        return "fail"
    if values.get("aot_status") != "ok":
        return "fail"
    return "ok"


def result_ok(results: dict[tuple[str, str], tuple[int, dict[str, str]]], prefix: str, mode: str) -> bool:
    rc, values = results[(prefix, mode)]
    return status(values, rc) == "ok"


def int_value(values: dict[str, str], key: str) -> int:
    try:
        return int(float(values.get(key, "0")))
    except ValueError:
        return 0


def emit_lane(lines: list[str], prefix: str, mode: str, values: dict[str, str], rc: int) -> None:
    lines.append(f"{prefix}_{mode}_run_status={status(values, rc)}")
    lines.append(f"{prefix}_{mode}_aot_status={values.get('aot_status', 'missing')}")
    for key in (
        "ny_total_cycles",
        "ny_startup_cycles",
        "ny_kernel_cycles",
        "ny_total_ms",
        "ny_startup_ms",
        "ny_kernel_ms",
        "ratio_total_cycles",
        "ratio_kernel_cycles",
        "ratio_kernel_ms",
        "ny_total_ipc",
        "ny_kernel_ipc",
    ):
        lines.append(f"{prefix}_{mode}_{key}={values.get(key, '0')}")
    lines.append(
        f"{prefix}_{mode}_startup_loader_cycles={int_value(values, 'ny_startup_cycles')}"
    )
    lines.append(f"{prefix}_{mode}_startup_loader_ms={values.get('ny_startup_ms', '0')}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--warmup", type=int, default=0)
    parser.add_argument("--repeat", type=int, default=1)
    parser.add_argument("--kernel-inner-runs", type=int, default=3)
    args = parser.parse_args()

    if args.warmup < 0 or args.repeat < 1 or args.kernel_inner_runs < 1:
        raise SystemExit("--warmup must be >= 0 and repeat/kernel-inner-runs must be >= 1")

    raw_outputs: list[str] = []
    results: dict[tuple[str, str], tuple[int, dict[str, str]]] = {}
    invalid_arraybox_count = 0
    for prefix, bench_key in CASES:
        for mode, helper in (("floor", False), ("helper", True)):
            rc, values, output = run_case(
                bench_key,
                helper=helper,
                warmup=args.warmup,
                repeat=args.repeat,
                kernel_inner_runs=args.kernel_inner_runs,
            )
            raw_outputs.append(f"## {prefix} {mode}\n{output}")
            invalid_arraybox_count += output.count("invalid ArrayBox handle")
            results[(prefix, mode)] = (rc, values)

    failure_count = sum(1 for rc, values in results.values() if status(values, rc) != "ok")
    floor_ok = all(result_ok(results, prefix, "floor") for prefix, _ in CASES)
    helper_ok = all(result_ok(results, prefix, "helper") for prefix, _ in CASES)
    measured = {
        prefix: result_ok(results, prefix, "floor") and result_ok(results, prefix, "helper")
        for prefix, _ in CASES
    }
    attribution_count = sum(1 for ok in measured.values() if ok)

    lines = [
        "output_contract=perf-userbox-direct-helper-floor-attribution-v0",
        "measurement_scope=userbox_typed_object_floor_helper_startup_loader_attribution",
        f"warmup={args.warmup}",
        f"repeat={args.repeat}",
        f"kernel_inner_runs={args.kernel_inner_runs}",
        "typed_object_floor_backend=single_thread_exact",
        "typed_object_helper_backend=single_thread_exact",
        "typed_object_helper_gate=HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1",
        "array_slot_backend=unset",
        f"direct_helper_floor_run_status={'ok' if floor_ok else 'fail'}",
        f"direct_helper_helper_run_status={'ok' if helper_ok else 'fail'}",
        f"floor_run_status={'ok' if floor_ok else 'fail'}",
        f"direct_helper_floor_invalid_arraybox_handle_count={invalid_arraybox_count}",
        f"counter_step_chain_helper_vs_floor_measured={1 if measured['counter_step_chain'] else 0}",
        f"point_add_helper_vs_floor_measured={1 if measured['point_add'] else 0}",
        f"startup_loader_attribution_report={1 if attribution_count == len(CASES) else 0}",
        f"startup_loader_attribution_case_count={attribution_count}",
        f"measurement_harness_failure_count={failure_count}",
    ]

    for prefix, _ in CASES:
        lines.append(f"{prefix}_startup_loader_attribution=available" if measured[prefix] else f"{prefix}_startup_loader_attribution=missing")
        for mode in ("floor", "helper"):
            rc, values = results[(prefix, mode)]
            emit_lane(lines, prefix, mode, values, rc)

    lines.extend(
        [
            "touch_hako_source=0",
            "touch_mirbuilder=0",
            "touch_route_planner=0",
            "touch_exact_helper_lowering=0",
            "touch_runtime_object_representation=0",
            f"summary={'ok' if floor_ok and helper_ok and invalid_arraybox_count == 0 else 'fail'}",
        ]
    )

    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    raw_path = args.out.with_suffix(args.out.suffix + ".raw.log")
    raw_path.write_text("\n\n".join(raw_outputs), encoding="utf-8")
    print("\n".join(lines))
    return 0 if lines[-1] == "summary=ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
