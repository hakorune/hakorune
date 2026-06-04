#!/usr/bin/env python3
"""Compare Hakozuna mixed-ws under system, C mimalloc, and optional provider LD_PRELOAD."""

from __future__ import annotations

import argparse
import os
from pathlib import Path

from hakozuna_mixed_ws_build_support import (
    build_replacement_front_bins_shim,
    build_replacement_front_shim,
    find_mimalloc_library,
)
from hakozuna_mixed_ws_compare_plan import build_compare_plan
from hakozuna_mixed_ws_report_render import render_hakozuna_mixed_ws_report
from hakozuna_mixed_ws_subject_runner import run_hakozuna_mixed_ws_subjects

ROOT = Path(__file__).resolve().parents[2]
DEFAULT_HAKOZUNA_ROOT = ROOT / "benchmarks" / "external" / "hakozuna" / "mixed-ws" / "build"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--hakozuna-root", type=Path, default=DEFAULT_HAKOZUNA_ROOT)
    parser.add_argument("--mimalloc-library", type=Path)
    parser.add_argument("--allow-ldconfig-discovery", action="store_true")
    parser.add_argument("--manifest", type=Path)
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--sample-count", type=int, default=3)
    parser.add_argument("--warmup-count", type=int, default=1)
    parser.add_argument(
        "--min-sample-seconds",
        type=float,
        default=0.0,
        help=(
            "mark measurement_quality=too_short if any sampled bench run is "
            "shorter than this many seconds; default 0 keeps legacy smoke-sized "
            "probes accepted"
        ),
    )
    parser.add_argument("--threads", type=int, default=1)
    parser.add_argument("--iters-per-thread", type=int, default=1000)
    parser.add_argument("--working-set", type=int, default=128)
    parser.add_argument("--min-size", type=int, default=16)
    parser.add_argument("--max-size", type=int, default=1024)
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
    parser.add_argument(
        "--replacement-front-native-slot-mode",
        action="store_true",
        help="benchmark-only: add a thin native-slot malloc/free replacement front subject",
    )
    parser.add_argument(
        "--replacement-front-native-bins-mode",
        action="store_true",
        help=(
            "benchmark-only: add a thin native multi-bin malloc/free replacement "
            "front subject using workload .hako size-class bins"
        ),
    )
    parser.add_argument(
        "--replacement-front-page-bins-mode",
        action="store_true",
        help=(
            "benchmark-only: add a page-shaped multi-bin malloc/free replacement "
            "front subject using workload .hako size-class bins"
        ),
    )
    parser.add_argument(
        "--replacement-front-hotcore-page-model-mode",
        action="store_true",
        help=(
            "benchmark-only: with page-bins mode, route alloc/free through "
            "HotCore/PageModel-shaped acquire/release helpers"
        ),
    )
    parser.add_argument(
        "--replacement-front-product-pages-nonlinear-mode",
        action="store_true",
        help=(
            "benchmark-only: requires page-bins mode; use a page-key indexed "
            "ownership lookup instead of the linear generated find_owned scan. "
            "HotCore/eager-init/size-table are the recommended measurement stack."
        ),
    )
    parser.add_argument(
        "--replacement-front-size-class-table-mode",
        action="store_true",
        help=(
            "benchmark-only: with bins mode, lower SizeClassBox size lookup "
            "through an 8-byte bucket table instead of an ordered range scan"
        ),
    )
    parser.add_argument(
        "--replacement-front-eager-init-mode",
        action="store_true",
        help=(
            "benchmark-only: with bins mode, initialize replacement bins in the "
            "constructor and keep hot malloc on the already-initialized path"
        ),
    )
    parser.add_argument(
        "--replacement-front-lock-mode",
        action="store_true",
        help="benchmark-only: build the replacement front with a global arena mutex",
    )
    parser.add_argument(
        "--replacement-front-thread-local-mode",
        action="store_true",
        help="benchmark-only: build the replacement front with same-thread TLS arenas",
    )
    parser.add_argument(
        "--replacement-front-cross-thread-smoke",
        action="store_true",
        help="run focused cross-thread free and abandoned-owner replacement front smokes",
    )
    parser.add_argument(
        "--replacement-front-skip-hot-counters",
        action="store_true",
        help="measurement-only: skip malloc/free hot-path replacement front counters",
    )
    parser.add_argument(
        "--replacement-front-tls-counter-mode",
        action="store_true",
        help="benchmark-only: aggregate replacement front counters through thread-local buffers",
    )
    parser.add_argument(
        "--replacement-front-slot-size",
        type=int,
        help="benchmark-only: override replacement front fixed slot size in bytes",
    )
    parser.add_argument(
        "--replacement-front-match-workload-realloc-size",
        action="store_true",
        help=(
            "benchmark-only: set replacement front slot size to max-size + 16, "
            "matching the mixed-ws realloc grow request"
        ),
    )
    parser.add_argument(
        "--replacement-front-match-hako-size-class",
        action="store_true",
        help=(
            "benchmark-only: set replacement front slot size to "
            "SizeClassBox.good_size(max-size + 16), matching the mixed-ws "
            "allocation/realloc request ceiling"
        ),
    )
    args = parser.parse_args()
    compare_plan = build_compare_plan(args)

    root = args.hakozuna_root.resolve()
    bench = root / "bench_mixed_ws_crt"
    if not bench.is_file() or not os.access(bench, os.X_OK):
        raise SystemExit(
            "missing executable hakozuna mixed-ws bench: "
            f"{bench}\n"
            "hint: run `make -C benchmarks/external/hakozuna/mixed-ws` "
            "or pass --hakozuna-root for an external hakozuna build"
        )

    out_dir = args.out_dir.resolve()
    out_dir.mkdir(parents=True, exist_ok=True)
    mimalloc_library = find_mimalloc_library(args.mimalloc_library, args.allow_ldconfig_discovery)

    replacement_front_shim: Path | None = None
    if args.replacement_front_native_slot_mode:
        replacement_front_shim = build_replacement_front_shim(
            out_dir,
            locked=args.replacement_front_lock_mode,
            thread_local=args.replacement_front_thread_local_mode,
            skip_hot_counters=args.replacement_front_skip_hot_counters,
            tls_counters=args.replacement_front_tls_counter_mode,
            slot_size=compare_plan.replacement_slot_size,
        )
    if compare_plan.replacement_front_bins_mode:
        if not compare_plan.required_regular_bins:
            raise SystemExit(
                "--replacement-front-native-bins-mode/--replacement-front-page-bins-mode "
                "found no regular bins"
            )
        if int(compare_plan.workload_histogram["size_class_huge_count"]) > 0:
            raise SystemExit(
                "--replacement-front-native-bins-mode/--replacement-front-page-bins-mode "
                "v0 does not support huge bins"
            )
        replacement_front_shim = build_replacement_front_bins_shim(
            out_dir,
            required_bins=compare_plan.required_regular_bins,
            page_shaped=args.replacement_front_page_bins_mode,
            hotcore_page_model=args.replacement_front_hotcore_page_model_mode,
            size_class_table=args.replacement_front_size_class_table_mode,
            eager_init=args.replacement_front_eager_init_mode,
            product_pages_nonlinear_lookup=args.replacement_front_product_pages_nonlinear_mode,
        )
    subject_specs, reports, replacement_front_smokes = run_hakozuna_mixed_ws_subjects(
        args=args,
        bench=bench,
        root=root,
        out_dir=out_dir,
        replacement_front_shim=replacement_front_shim,
        mimalloc_library=mimalloc_library,
    )
    report = render_hakozuna_mixed_ws_report(
        args=args,
        bench=bench,
        root=root,
        mimalloc_library=mimalloc_library,
        workload_histogram=compare_plan.workload_histogram,
        replacement_front_smokes=replacement_front_smokes,
        subject_specs=subject_specs,
        reports=reports,
        replacement_front_bins_mode=compare_plan.replacement_front_bins_mode,
        replacement_slot_size=compare_plan.replacement_slot_size,
        replacement_front_size_class_request_ceiling=(
            compare_plan.replacement_front_size_class_request_ceiling
        ),
        replacement_front_size_class_selected_bin=(
            compare_plan.replacement_front_size_class_selected_bin
        ),
        replacement_front_size_class_selected_good_size=(
            compare_plan.replacement_front_size_class_selected_good_size
        ),
    )
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
