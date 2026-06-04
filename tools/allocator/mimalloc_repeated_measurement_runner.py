#!/usr/bin/env python3
"""Run repeated `.hako` and C mimalloc evidence samples without winner claims."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import tempfile
from pathlib import Path

from mimalloc_repeated_measurement_report import render_repeated_measurement_report


ROOT = Path(__file__).resolve().parents[2]
HAKO_RUNNER = ROOT / "tools/allocator/hako_exe_memory_runner.sh"
C_RUNNER = ROOT / "tools/allocator/c_mimalloc_explicit_runner.sh"
LOADSET_PLAN = ROOT / "tools/allocator/hako_plugin_loadset_plan.py"

WORKLOAD_APPS = {
    "representative-empty-v0": ROOT
    / "apps/hako-alloc-mimalloc-comparison-empty-exe-proof/main.hako",
    "representative-small-block-v0": ROOT
    / "apps/hako-alloc-mimalloc-comparison-representative-small-block-proof/main.hako",
    "representative-realloc-aligned-v0": ROOT
    / "apps/hako-alloc-mimalloc-comparison-realloc-aligned-exe-proof/main.hako",
    "representative-mixed-small-v0": ROOT
    / "apps/hako-alloc-mimalloc-comparison-mixed-small-exe-proof/main.hako",
    "representative-huge-ish-v0": ROOT
    / "apps/hako-alloc-mimalloc-comparison-huge-ish-exe-proof/main.hako",
    "representative-reuse-cycle-small-v0": ROOT
    / "apps/hako-alloc-mimalloc-comparison-reuse-cycle-small-exe-proof/main.hako",
}

DEFAULT_WORKLOADS = [
    "representative-small-block-v0",
    "representative-realloc-aligned-v0",
    "representative-mixed-small-v0",
    "representative-huge-ish-v0",
]


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def as_int(values: dict[str, str], key: str) -> int:
    text = values.get(key, "0")
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{key} must be an integer, got {text!r}") from exc


def require(values: dict[str, str], key: str, expected: str, label: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{label}: {key} expected {expected!r}, got {actual!r}")


def median_int(values: list[int]) -> int:
    ordered = sorted(values)
    return ordered[len(ordered) // 2]


def key_part(text: str) -> str:
    return re.sub(r"[^A-Za-z0-9]+", "_", text).strip("_")


def hako_loadset_for_runtime_config(runtime_config: str) -> str:
    if runtime_config == "empty":
        return "empty"
    if runtime_config == "root":
        return "root"
    raise SystemExit(f"unsupported hako runtime config: {runtime_config}")


def load_hako_loadset_plan(runtime_config: str) -> dict[str, object]:
    loadset = hako_loadset_for_runtime_config(runtime_config)
    completed = subprocess.run(
        [
            sys.executable,
            str(LOADSET_PLAN),
            "--config",
            "hako.toml",
            "--loadset",
            loadset,
        ],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        check=True,
    )
    plan = json.loads(completed.stdout)
    if plan.get("output_contract") != "hako-plugin-loadset-plan-v0":
        raise SystemExit("bad hako loadset plan output_contract")
    if plan.get("selected_loadset") != loadset:
        raise SystemExit("hako loadset plan selected_loadset mismatch")
    if plan.get("plugin_load_policy") != "eager_selected":
        raise SystemExit("hako loadset plan plugin_load_policy mismatch")
    return plan


def run_one(
    workload: str,
    side: str,
    out_path: Path,
    allow_ldconfig: bool,
    c_library: Path | None,
    hako_runtime_config: str,
    operation_repeat: int,
) -> dict[str, str]:
    if side == "hako":
        app = WORKLOAD_APPS[workload]
        cmd = [
            "bash",
            str(HAKO_RUNNER),
            "--app",
            str(app),
            "--workload",
            workload,
            "--runtime-config",
            hako_runtime_config,
            "--operation-repeat",
            str(operation_repeat),
            "--out",
            str(out_path),
        ]
    elif side == "c":
        cmd = [
            "bash",
            str(C_RUNNER),
            "--out",
            str(out_path),
            "--workload",
            workload,
            "--operation-repeat",
            str(operation_repeat),
        ]
        if c_library is not None:
            cmd.extend(["--library", str(c_library)])
        elif allow_ldconfig:
            cmd.append("--allow-ldconfig-discovery")
        else:
            raise SystemExit("--c-library PATH or --allow-ldconfig-discovery is required for C samples")
    else:
        raise AssertionError(side)

    with (out_path.parent / f"{out_path.name}.stdout").open("w", encoding="utf-8") as stdout:
        subprocess.run(cmd, cwd=ROOT, stdout=stdout, check=True)
    return read_kv(out_path)


def validate_sample(
    workload: str,
    hako: dict[str, str],
    c: dict[str, str],
    label: str,
    expected_c_library: Path | None,
) -> None:
    require(hako, "output_contract", "hako-exe-memory-evidence-v0", f"{label}:hako")
    require(c, "output_contract", "allocator-comparison-c-mimalloc-explicit-runner-v0", f"{label}:c")
    require(hako, "summary", "ok", f"{label}:hako")
    require(c, "summary", "ok", f"{label}:c")
    require(hako, "workload", workload, f"{label}:hako")
    require(c, "workload", workload, f"{label}:c")
    require(hako, "provider_activation", "0", f"{label}:hako")
    require(hako, "host_replacement", "0", f"{label}:hako")
    require(hako, "hook_installed", "0", f"{label}:hako")
    require(hako, "global_allocator_installed", "0", f"{label}:hako")
    require(c, "process_replacement_executed", "0", f"{label}:c")
    require(c, "hook_installed", "0", f"{label}:c")
    require(c, "backend_matcher_added", "0", f"{label}:c")
    require(c, "global_allocator_installed", "0", f"{label}:c")
    require(c, "hidden_discovery_used", "0", f"{label}:c")
    require(c, "provider_package_generated", "0", f"{label}:c")
    if expected_c_library is not None:
        require(c, "library_path", str(expected_c_library), f"{label}:c")
    if hako.get("operation_family", "") != c.get("operation_family", ""):
        raise SystemExit(f"{label}: operation_family mismatch")
    if hako.get("operation_sequence_id", "") != c.get("operation_sequence_id", ""):
        raise SystemExit(f"{label}: operation_sequence_id mismatch")
    if hako.get("free_order_id", "") != c.get("free_order_id", ""):
        raise SystemExit(f"{label}: free_order_id mismatch")
    if hako.get("timing_repeat_kind", "") != c.get("timing_repeat_kind", ""):
        raise SystemExit(f"{label}: timing_repeat_kind mismatch")
    if hako.get("operation_repeat", "") != c.get("operation_repeat", ""):
        raise SystemExit(f"{label}: operation_repeat mismatch")
    for key in (
        "allocation_count",
        "free_count",
        "requested_bytes",
        "realloc_count",
        "aligned_alloc_count",
        "large_request_count",
        "reuse_cycle_count",
    ):
        if as_int(hako, key) != as_int(c, key):
            raise SystemExit(f"{label}: {key} mismatch")
    for key in ("external_peak_rss_bytes", "peak_rss_bytes"):
        if as_int(hako, key) <= 0:
            raise SystemExit(f"{label}: hako {key} must be positive")
        if as_int(c, key) <= 0:
            raise SystemExit(f"{label}: c {key} must be positive")
    if as_int(hako, "external_elapsed_ms") <= 0:
        raise SystemExit(f"{label}: hako external_elapsed_ms must be positive")
    if as_int(c, "external_elapsed_ms") <= 0:
        raise SystemExit(f"{label}: c external_elapsed_ms must be positive")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", required=True, type=Path)
    parser.add_argument("--sample-count", type=int, default=5)
    parser.add_argument("--warmup-count", type=int, default=1)
    parser.add_argument("--workload", action="append", choices=sorted(WORKLOAD_APPS))
    parser.add_argument("--c-library", type=Path)
    parser.add_argument("--allow-ldconfig-discovery", action="store_true")
    parser.add_argument(
        "--hako-runtime-config",
        choices=("root", "empty"),
        default="empty",
        help="runtime config profile for .hako exact-EXE samples; default: empty",
    )
    parser.add_argument("--operation-repeat", type=int, default=1)
    args = parser.parse_args()

    if args.sample_count < 1:
        raise SystemExit("--sample-count must be positive")
    if args.warmup_count < 0:
        raise SystemExit("--warmup-count must be non-negative")
    if args.operation_repeat < 1:
        raise SystemExit("--operation-repeat must be positive")
    if args.c_library is not None:
        if not args.c_library.exists():
            raise SystemExit(f"--c-library path does not exist: {args.c_library}")
        args.allow_ldconfig_discovery = False

    workloads = args.workload or DEFAULT_WORKLOADS
    for workload in workloads:
        app = WORKLOAD_APPS[workload]
        if not app.exists():
            raise SystemExit(f"missing .hako workload app: {app}")

    hako_loadset_plan = load_hako_loadset_plan(args.hako_runtime_config)
    workload_records: list[dict[str, object]] = []
    with tempfile.TemporaryDirectory(prefix="hakorune_repeated_measurement.") as tmp:
        tmp_dir = Path(tmp)
        for workload_index, workload in enumerate(workloads):
            sample_hako_rss: list[int] = []
            sample_c_rss: list[int] = []
            sample_hako_elapsed: list[int] = []
            sample_c_elapsed: list[int] = []
            internal_hako_rss: list[int] = []
            internal_c_rss: list[int] = []
            operation_family = ""

            total_runs = args.warmup_count + args.sample_count
            for run_index in range(total_runs):
                kind = "warmup" if run_index < args.warmup_count else "sample"
                sample_index = run_index - args.warmup_count
                hako_out = tmp_dir / f"{key_part(workload)}.{run_index}.hako.out"
                c_out = tmp_dir / f"{key_part(workload)}.{run_index}.c.out"
                hako = run_one(
                    workload,
                    "hako",
                    hako_out,
                    args.allow_ldconfig_discovery,
                    args.c_library,
                    args.hako_runtime_config,
                    args.operation_repeat,
                )
                c = run_one(
                    workload,
                    "c",
                    c_out,
                    args.allow_ldconfig_discovery,
                    args.c_library,
                    args.hako_runtime_config,
                    args.operation_repeat,
                )
                validate_sample(workload, hako, c, f"{workload}:{kind}:{run_index}", args.c_library)
                require(
                    hako,
                    "runtime_config_profile",
                    args.hako_runtime_config,
                    f"{workload}:{kind}:{run_index}:hako",
                )
                operation_family = hako.get("operation_family", "")
                if kind == "sample":
                    sample_hako_rss.append(as_int(hako, "external_peak_rss_bytes"))
                    sample_c_rss.append(as_int(c, "external_peak_rss_bytes"))
                    sample_hako_elapsed.append(as_int(hako, "external_elapsed_ms"))
                    sample_c_elapsed.append(as_int(c, "external_elapsed_ms"))
                    internal_hako_rss.append(as_int(hako, "peak_rss_bytes"))
                    internal_c_rss.append(as_int(c, "peak_rss_bytes"))

            workload_records.append(
                {
                    "workload": workload,
                    "operation_family": operation_family,
                    "hako_external_rss_min": min(sample_hako_rss),
                    "hako_external_rss_median": median_int(sample_hako_rss),
                    "hako_external_rss_max": max(sample_hako_rss),
                    "c_external_rss_min": min(sample_c_rss),
                    "c_external_rss_median": median_int(sample_c_rss),
                    "c_external_rss_max": max(sample_c_rss),
                    "hako_external_elapsed_min": min(sample_hako_elapsed),
                    "hako_external_elapsed_median": median_int(sample_hako_elapsed),
                    "hako_external_elapsed_max": max(sample_hako_elapsed),
                    "c_external_elapsed_min": min(sample_c_elapsed),
                    "c_external_elapsed_median": median_int(sample_c_elapsed),
                    "c_external_elapsed_max": max(sample_c_elapsed),
                    "hako_internal_rss_median": median_int(internal_hako_rss),
                    "c_internal_rss_median": median_int(internal_c_rss),
                }
            )

    report = render_repeated_measurement_report(
        args=args,
        workloads=workloads,
        hako_loadset_plan=hako_loadset_plan,
        workload_records=workload_records,
    )
    args.out.write_text(report, encoding="utf-8")
    print(args.out.read_text(encoding="utf-8"), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
