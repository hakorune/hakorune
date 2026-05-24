#!/usr/bin/env python3
"""Compare root vs generated-empty runtime config for `.hako` exact-EXE evidence."""

from __future__ import annotations

import argparse
import json
import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
RUNNER = ROOT / "tools/allocator/hako_exe_memory_runner.sh"

WORKLOAD_APPS = {
    "representative-empty-v0": ROOT / "apps/hako-alloc-mimalloc-comparison-empty-exe-proof/main.hako",
    "representative-small-block-v0": ROOT / "apps/hako-alloc-mimalloc-comparison-representative-small-block-proof/main.hako",
    "representative-realloc-aligned-v0": ROOT / "apps/hako-alloc-mimalloc-comparison-realloc-aligned-exe-proof/main.hako",
    "representative-mixed-small-v0": ROOT / "apps/hako-alloc-mimalloc-comparison-mixed-small-exe-proof/main.hako",
    "representative-huge-ish-v0": ROOT / "apps/hako-alloc-mimalloc-comparison-huge-ish-exe-proof/main.hako",
}

DEFAULT_WORKLOADS = [
    "representative-empty-v0",
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
    try:
        return int(values.get(key, "0"))
    except ValueError as exc:
        raise SystemExit(f"{key} must be integer: {values.get(key)!r}") from exc


def run_runner(workload: str, profile: str, out_path: Path) -> dict[str, str]:
    app = WORKLOAD_APPS[workload]
    subprocess.run(
        [
            "bash",
            str(RUNNER),
            "--app",
            str(app),
            "--workload",
            workload,
            "--runtime-config",
            profile,
            "--out",
            str(out_path),
        ],
        cwd=ROOT,
        stdout=subprocess.DEVNULL,
        check=True,
    )
    values = read_kv(out_path)
    if values.get("output_contract") != "hako-exe-memory-evidence-v0":
        raise SystemExit(f"{workload}:{profile}: bad output contract")
    if values.get("summary") != "ok":
        raise SystemExit(f"{workload}:{profile}: summary must be ok")
    if values.get("runtime_config_profile") != profile:
        raise SystemExit(f"{workload}:{profile}: runtime_config_profile mismatch")
    if as_int(values, "external_peak_rss_bytes") <= 0:
        raise SystemExit(f"{workload}:{profile}: external RSS must be positive")
    return values


def validate_same_workload(root: dict[str, str], empty: dict[str, str], workload: str) -> None:
    for values, profile in ((root, "root"), (empty, "empty")):
        if values.get("workload") != workload:
            raise SystemExit(f"{workload}:{profile}: workload mismatch")
        for key in ("provider_activation", "host_replacement", "hook_installed", "global_allocator_installed"):
            if values.get(key) != "0":
                raise SystemExit(f"{workload}:{profile}: {key} must remain 0")

    same_keys = [
        "operation_family",
        "operation_sequence_id",
        "free_order_id",
        "allocation_count",
        "free_count",
        "requested_bytes",
        "realloc_count",
        "aligned_alloc_count",
        "large_request_count",
        "output_summary_ok",
    ]
    for key in same_keys:
        if root.get(key, "") != empty.get(key, ""):
            raise SystemExit(f"{workload}: {key} changed under empty runtime config")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--out", required=True, type=Path)
    parser.add_argument("--workload", action="append", choices=sorted(WORKLOAD_APPS))
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    workloads = args.workload or DEFAULT_WORKLOADS
    for workload in workloads:
        if not WORKLOAD_APPS[workload].exists():
            raise SystemExit(f"missing workload app: {WORKLOAD_APPS[workload]}")

    rows = []
    with tempfile.TemporaryDirectory(prefix="hakorune_min_config_evidence.") as tmp_text:
        tmp = Path(tmp_text)
        for workload in workloads:
            root = run_runner(workload, "root", tmp / f"{workload}.root.out")
            empty = run_runner(workload, "empty", tmp / f"{workload}.empty.out")
            validate_same_workload(root, empty, workload)
            root_rss = as_int(root, "external_peak_rss_bytes")
            empty_rss = as_int(empty, "external_peak_rss_bytes")
            rows.append(
                {
                    "workload": workload,
                    "operation_family": root.get("operation_family", ""),
                    "root_external_peak_rss_bytes": root_rss,
                    "empty_external_peak_rss_bytes": empty_rss,
                    "rss_reduction_bytes": root_rss - empty_rss,
                    "winner_claim": 0,
                }
            )

    args.out.write_text(
        json.dumps(
            {
                "output_contract": "hako-exact-exe-minimal-config-evidence-v0",
                "workload_count": len(rows),
                "winner_claim": 0,
                "rows": rows,
            },
            sort_keys=True,
        )
        + "\n",
        encoding="utf-8",
    )
    print(args.out.read_text(encoding="utf-8"), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
