#!/usr/bin/env python3
"""Prepare and run the extracted hakmem mimalloc-bench corpus.

This is a local bridge for external/historical benchmark exploration. It keeps
copied binaries and generated benchres.csv under target/ so the repository does
not start tracking benchmark executables or mutable benchmark output.
"""

from __future__ import annotations

import argparse
import os
import shutil
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_SOURCE = Path("/home/tomoaki/git/hakmem_20260525_extracted/hakmem")
DEFAULT_TARGET = ROOT / "target" / "hakmem-bench"
DEFAULT_BENCHES = ["cfrac"]
DEFAULT_ALLOCATORS = ["sys", "mimalloc"]
SUPPORTED_ALLOCATORS = ["sys", "mimalloc", "tcmalloc", "hz3", "hakozuna"]
SUPPORTED_BENCHES = [
    "alloc-test",
    "barnes",
    "cfrac",
    "cscratch",
    "cthrash",
    "espresso",
    "glibc-simple",
    "glibc-thread",
    "larson",
    "larson-sized",
    "malloc-large",
    "mleak",
    "mstress",
    "rptest",
    "sh6bench",
    "sh8bench",
    "xmalloc-test",
]

EXECUTABLES = [
    "alloc-test",
    "barnes",
    "cache-scratch",
    "cache-thrash",
    "cfrac",
    "espresso",
    "glibc-simple",
    "glibc-thread",
    "larson",
    "larson-sized",
    "malloc-large",
    "malloc-large-old",
    "mleak",
    "mstress",
    "rptest",
    "sh6bench",
    "sh8bench",
    "xmalloc-test",
]


def fail(message: str) -> None:
    raise SystemExit(f"[hakmem-bench] {message}")


def ensure_source(source: Path) -> None:
    required = [
        source / "mimalloc-bench" / "bench.sh",
        source / "mimalloc-bench" / "build-bench-env.sh",
        source / "mimalloc-bench" / "out" / "bench",
        source / "mimalloc-bench" / "bench",
    ]
    for path in required:
        if not path.exists():
            fail(f"missing source path: {path}")


def reset_symlink(path: Path, target: Path) -> None:
    if path.is_symlink() or path.exists():
        if path.is_dir() and not path.is_symlink():
            shutil.rmtree(path)
        else:
            path.unlink()
    path.symlink_to(target, target_is_directory=target.is_dir())


def copy_if_needed(src: Path, dst: Path, refresh: bool) -> None:
    if refresh and dst.exists():
        dst.unlink()
    if dst.exists():
        return
    shutil.copy2(src, dst)


def prepare_tree(source: Path, target: Path, refresh: bool, allocators: list[str]) -> Path:
    ensure_source(source)
    bench_src = source / "mimalloc-bench"
    bench_out_src = bench_src / "out" / "bench"

    target.mkdir(parents=True, exist_ok=True)
    (target / "out" / "bench").mkdir(parents=True, exist_ok=True)
    (target / "extern").mkdir(parents=True, exist_ok=True)
    (target / "extern" / "versions.txt").touch()

    copy_if_needed(bench_src / "bench.sh", target / "bench.sh", refresh)
    copy_if_needed(bench_src / "build-bench-env.sh", target / "build-bench-env.sh", refresh)
    os.chmod(target / "bench.sh", 0o755)

    reset_symlink(target / "bench", bench_src / "bench")

    for name in EXECUTABLES:
        src = bench_out_src / name
        if src.exists():
            dst = target / "out" / "bench" / name
            copy_if_needed(src, dst, refresh)
            os.chmod(dst, 0o755)

    external_file = target / "external_allocators.txt"
    external_candidates = {
        "mimalloc": [source / "libmimalloc.so"],
        "tcmalloc": [
            source / "allocators" / "tcmalloc" / "libtcmalloc_minimal.so",
            Path("/lib/x86_64-linux-gnu/libtcmalloc_minimal.so.4"),
        ],
        "hz3": [source / "libhakozuna_hz3_scale.so"],
        "hakozuna": [source / "libhakozuna.so"],
    }
    lines = []
    for name in allocators:
        if name == "sys":
            continue
        candidates = external_candidates.get(name)
        if candidates is None:
            fail(f"unsupported allocator '{name}'. Supported: sys, {', '.join(sorted(external_candidates))}")
        for candidate in candidates:
            if candidate.exists():
                lines.append(f"{name} {candidate}")
                break
        else:
            fail(f"missing library for allocator '{name}'")
    external_file.write_text("\n".join(lines) + ("\n" if lines else ""), encoding="utf-8")

    return external_file


def parse_benchres(path: Path) -> list[dict[str, str]]:
    rows: list[dict[str, str]] = []
    if not path.exists():
        return rows
    for raw in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = raw.strip()
        if not line or line.startswith("#") or line.startswith("Command "):
            continue
        parts = line.split()
        if len(parts) < 8:
            rows.append({"raw": line, "parse_status": "short"})
            continue
        rows.append(
            {
                "benchmark": parts[0],
                "allocator": parts[1],
                "elapsed": parts[2],
                "rss_kb": parts[3],
                "user_sec": parts[4],
                "sys_sec": parts[5],
                "major_faults": parts[6],
                "minor_faults": parts[7],
                "parse_status": "ok",
            }
        )
    return rows


def emit_summary(rows: list[dict[str, str]], copied_to: Path | None, target: Path) -> None:
    print("output_contract=hakmem-external-bench-bridge-v0")
    print("dataset_role=external-historical-benchmark-corpus")
    print(f"target_root={target}")
    print(f"benchres_copied_to={copied_to or ''}")
    print(f"row_count={len(rows)}")
    print(f"parsed_row_count={sum(1 for row in rows if row.get('parse_status') == 'ok')}")
    print("winner_claim=0")
    print("provider_activation=0")
    print("host_replacement=0")
    print("hook_installed=0")
    print("global_allocator_installed=0")
    for idx, row in enumerate(rows[:12]):
        prefix = f"row_{idx}"
        for key in ["benchmark", "allocator", "elapsed", "rss_kb", "user_sec", "sys_sec"]:
            if key in row:
                print(f"{prefix}_{key}={row[key]}")
    print("summary=ok")


def run_bench(args: argparse.Namespace, external_file: Path) -> Path:
    bench_dir = args.target / "out" / "bench"
    cmd = [
        "../../bench.sh",
        f"-j={args.jobs}",
        f"-r={args.repeats}",
        f"-n={args.test_repeats}",
    ]
    if external_file.read_text(encoding="utf-8").strip():
        cmd.append(f"--external={external_file}")
    cmd.extend(allocator for allocator in args.allocators if allocator == "sys")
    cmd.extend(args.benches)
    env = os.environ.copy()
    if args.verbose:
        print("[hakmem-bench] running:", " ".join(cmd), file=sys.stderr)
    subprocess.run(cmd, cwd=bench_dir, env=env, check=True)
    return bench_dir / "benchres.csv"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, default=Path(os.environ.get("HAKMEM_ROOT", DEFAULT_SOURCE)))
    parser.add_argument("--target", type=Path, default=DEFAULT_TARGET)
    parser.add_argument("--bench", dest="benches", action="append", default=[])
    parser.add_argument("--allocator", dest="allocators", action="append", default=[])
    parser.add_argument("--jobs", type=int, default=1)
    parser.add_argument("--repeats", type=int, default=1)
    parser.add_argument("--test-repeats", type=int, default=1)
    parser.add_argument("--out", type=Path, default=None)
    parser.add_argument("--prepare-only", action="store_true")
    parser.add_argument("--refresh", action="store_true")
    parser.add_argument("--list", action="store_true")
    parser.add_argument("--verbose", action="store_true")
    args = parser.parse_args()

    args.source = args.source.resolve()
    args.target = args.target.resolve()
    if args.list:
        print("output_contract=hakmem-external-bench-bridge-list-v0")
        print(f"default_source={args.source}")
        print(f"default_target={args.target}")
        print("supported_allocators=" + ",".join(SUPPORTED_ALLOCATORS))
        print("supported_benches=" + ",".join(SUPPORTED_BENCHES))
        print("mutable_output=target/hakmem-bench/out/bench/benchres.csv")
        print("snapshot_output=target/hakmem-bench/results/*.benchres.csv")
        print("winner_claim=0")
        print("summary=ok")
        return 0
    args.benches = args.benches or DEFAULT_BENCHES
    args.allocators = args.allocators or DEFAULT_ALLOCATORS

    external_file = prepare_tree(args.source, args.target, args.refresh, args.allocators)
    benchres: Path | None = None
    copied_to: Path | None = None
    if not args.prepare_only:
        benchres = run_bench(args, external_file)
        if args.out is not None:
            args.out.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(benchres, args.out)
            copied_to = args.out.resolve()

    rows = parse_benchres(benchres) if benchres is not None else []
    emit_summary(rows, copied_to, args.target)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
