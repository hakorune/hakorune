#!/usr/bin/env python3
"""Run and summarize Hakozuna mixed-ws allocator gap evidence.

This is a narrow orchestration layer over hakozuna_mixed_ws_ldpreload_compare.py.
The underlying compare report remains the detailed evidence; this tool emits a
small front-door summary for deciding why provider-backed replacement is cold
against C mimalloc.
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

from hakozuna_mixed_ws_gap_summary import emit_summary
from python_template_c_bridge import add_baseline_flag


ROOT = Path(__file__).resolve().parents[2]
COMPARE_TOOL = ROOT / "tools" / "allocator" / "hakozuna_mixed_ws_ldpreload_compare.py"


def run_compare(args: argparse.Namespace, compare_report: Path) -> None:
    cmd = [
        sys.executable,
        str(COMPARE_TOOL),
        "--out",
        str(compare_report),
        "--out-dir",
        str(args.out_dir / "compare-artifacts"),
        "--sample-count",
        str(args.sample_count),
        "--warmup-count",
        str(args.warmup_count),
        "--min-sample-seconds",
        str(args.min_sample_seconds),
        "--threads",
        str(args.threads),
        "--iters-per-thread",
        str(args.iters_per_thread),
        "--working-set",
        str(args.working_set),
        "--min-size",
        str(args.min_size),
        "--max-size",
        str(args.max_size),
    ]
    if args.allow_ldconfig_discovery:
        cmd.append("--allow-ldconfig-discovery")
    if args.hakozuna_root is not None:
        cmd.extend(["--hakozuna-root", str(args.hakozuna_root)])
    if args.mimalloc_library is not None:
        cmd.extend(["--mimalloc-library", str(args.mimalloc_library)])
    if args.manifest is not None:
        cmd.extend(["--manifest", str(args.manifest)])
    if args.provider_usable_size_mode:
        cmd.append("--provider-usable-size-mode")
    if args.provider_assume_owned_mode:
        cmd.append("--provider-assume-owned-mode")
    if args.allow_python_template_c_bridge_baseline:
        cmd.append("--allow-python-template-c-bridge-baseline")
    if args.replacement_front_native_slot_mode:
        cmd.append("--replacement-front-native-slot-mode")
    if args.replacement_front_lock_mode:
        cmd.append("--replacement-front-lock-mode")
    if args.replacement_front_thread_local_mode:
        cmd.append("--replacement-front-thread-local-mode")
    if args.replacement_front_cross_thread_smoke:
        cmd.append("--replacement-front-cross-thread-smoke")
    if args.replacement_front_skip_hot_counters:
        cmd.append("--replacement-front-skip-hot-counters")
    if args.replacement_front_tls_counter_mode:
        cmd.append("--replacement-front-tls-counter-mode")
    if args.replacement_front_slot_size is not None:
        cmd.extend(["--replacement-front-slot-size", str(args.replacement_front_slot_size)])
    if args.replacement_front_match_workload_realloc_size:
        cmd.append("--replacement-front-match-workload-realloc-size")
    if args.replacement_front_match_hako_size_class:
        cmd.append("--replacement-front-match-hako-size-class")
    subprocess.run(cmd, cwd=ROOT, check=True)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--out-dir", type=Path)
    parser.add_argument("--hakozuna-root", type=Path)
    parser.add_argument("--mimalloc-library", type=Path)
    parser.add_argument("--allow-ldconfig-discovery", action="store_true")
    parser.add_argument("--manifest", type=Path)
    parser.add_argument(
        "--provider-usable-size-mode",
        action="store_true",
        help="measurement-only: bypass provider shim tracking through private usable_size symbol",
    )
    parser.add_argument(
        "--provider-assume-owned-mode",
        action="store_true",
        help="measurement-only: with usable-size mode, skip provider owns checks before free/realloc",
    )
    parser.add_argument("--sample-count", type=int, default=5)
    parser.add_argument("--warmup-count", type=int, default=1)
    parser.add_argument(
        "--min-sample-seconds",
        type=float,
        default=0.0,
        help=(
            "require every sampled bench run to last at least this many seconds; "
            "0 preserves legacy smoke-sized probes"
        ),
    )
    parser.add_argument("--threads", type=int, default=4)
    parser.add_argument("--iters-per-thread", type=int, default=1000)
    parser.add_argument("--working-set", type=int, default=128)
    parser.add_argument("--min-size", type=int, default=16)
    parser.add_argument("--max-size", type=int, default=1024)
    parser.add_argument("--replacement-front-native-slot-mode", action="store_true")
    add_baseline_flag(parser)
    parser.add_argument("--replacement-front-lock-mode", action="store_true")
    parser.add_argument("--replacement-front-thread-local-mode", action="store_true")
    parser.add_argument("--replacement-front-cross-thread-smoke", action="store_true")
    parser.add_argument("--replacement-front-skip-hot-counters", action="store_true")
    parser.add_argument("--replacement-front-tls-counter-mode", action="store_true")
    parser.add_argument("--replacement-front-slot-size", type=int)
    parser.add_argument("--replacement-front-match-workload-realloc-size", action="store_true")
    parser.add_argument("--replacement-front-match-hako-size-class", action="store_true")
    args = parser.parse_args()
    if args.min_sample_seconds < 0.0:
        raise SystemExit("--min-sample-seconds must be non-negative")

    if args.out_dir is None:
        args.out_dir = Path(f"{args.out}.artifacts.d")
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out_dir.mkdir(parents=True, exist_ok=True)

    compare_report = args.out_dir / "compare.out"
    run_compare(args, compare_report)
    emit_summary(compare_report, args.out)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
