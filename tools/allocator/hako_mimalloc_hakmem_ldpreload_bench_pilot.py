#!/usr/bin/env python3
"""Run one probe-only hakmem benchmark sample with LD_PRELOAD applied."""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_HAKMEM_ROOT = (
    ROOT / "benchmarks" / "external" / "hakmem" / "random-mixed-system" / "build"
)
SMOKE_TOOL = Path(__file__).resolve().with_name("hako_mimalloc_hakmem_ldpreload_shim_smoke.py")
THROUGHPUT_RE = re.compile(r"Throughput\s*=\s*([0-9]+)\s+ops/s")


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--hakmem-root", type=Path, default=DEFAULT_HAKMEM_ROOT)
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--iterations", type=int, default=1000)
    parser.add_argument("--working-set", type=int, default=128)
    parser.add_argument("--seed", type=int, default=42)
    args = parser.parse_args()

    root = args.hakmem_root.resolve()
    bench = root / "bench_random_mixed_system"
    if not bench.is_file() or not os.access(bench, os.X_OK):
        raise SystemExit(
            "missing executable hakmem bench: "
            f"{bench}\n"
            "hint: run `make -C benchmarks/external/hakmem/random-mixed-system` "
            "or pass --hakmem-root for an external corpus build"
        )
    if args.iterations < 1 or args.working_set < 1:
        raise SystemExit("--iterations and --working-set must be positive")

    out_dir = args.out_dir.resolve()
    out_dir.mkdir(parents=True, exist_ok=True)
    smoke_report = out_dir / "shim-smoke.out"
    subprocess.run(
        [
            sys.executable,
            str(SMOKE_TOOL),
            "--out-dir",
            str(out_dir / "shim"),
            "--out",
            str(smoke_report),
        ],
        check=True,
    )
    smoke = read_kv(smoke_report)
    shim = Path(smoke["shim_artifact_path"])

    bench_stdout = out_dir / "bench.stdout"
    bench_stderr = out_dir / "bench.stderr"
    env = os.environ.copy()
    env["LD_PRELOAD"] = str(shim)
    command = [
        str(bench),
        str(args.iterations),
        str(args.working_set),
        str(args.seed),
    ]
    completed = subprocess.run(
        command,
        cwd=root,
        env=env,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    bench_stdout.write_text(completed.stdout, encoding="utf-8")
    bench_stderr.write_text(completed.stderr, encoding="utf-8")
    if completed.returncode != 0:
        raise SystemExit(
            f"hakmem bench failed with {completed.returncode}: {completed.stderr.strip()}"
        )
    match = THROUGHPUT_RE.search(completed.stdout)
    if match is None:
        raise SystemExit("hakmem bench output missing Throughput line")

    lines = [
        "output_contract=hako-mimalloc-hakmem-ldpreload-bench-pilot-v0",
        "input_contract=hako-mimalloc-hakmem-ldpreload-shim-smoke-v0",
        "hakmem_script_compatible=probe-only",
        "hakmem_fixture_kind=minimal-repo-random-mixed-system",
        f"hakmem_root={root}",
        "benchmark_id=bench_random_mixed_system",
        f"benchmark_path={bench}",
        f"benchmark_iterations={args.iterations}",
        f"benchmark_working_set={args.working_set}",
        f"benchmark_seed={args.seed}",
        "ld_preload_env_applied=1",
        f"ld_preload_artifact={shim}",
        "benchmark_sample_executed=1",
        f"benchmark_exit_code={completed.returncode}",
        f"throughput_ops_per_sec={match.group(1)}",
        "probe_process_ld_preload_applied=1",
        "hakorune_default_replacement_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "winner_claim=0",
        "next_row=HAKO-MIMALLOC-PERF-PARITY-SELFHOST-HANDOFF-GATE-296X-001",
        "summary=ok",
    ]
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
