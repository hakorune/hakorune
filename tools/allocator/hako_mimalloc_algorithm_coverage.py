#!/usr/bin/env python3
"""Report current .hako mimalloc algorithm coverage.

This is a read-only inventory tool. It separates:

- `.hako` hako_alloc policy/model coverage
- benchmark-only replacement-front execution coverage

It does not run benchmarks, choose keepers, or claim allocator readiness.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from hako_mimalloc_algorithm_coverage_render import emit_text
from hako_mimalloc_algorithm_coverage_report import report_dict
from hako_mimalloc_algorithm_coverage_support import (
    CoverageRow,
    REPLACEMENT_FRONT,
    REPLACEMENT_TEMPLATES,
    has_all,
    has_file,
    hako_file,
    read_text,
)


def build_rows() -> list[CoverageRow]:
    page_box = read_text(hako_file("page_box.hako"))
    hot_core = read_text(hako_file("object_lifecycle_hot_core_box.hako"))
    size_class = read_text(hako_file("size_class_box.hako"))
    page_map = read_text(hako_file("page_map_box.hako"))
    realloc_same = read_text(hako_file("page_map_realloc_same_class_box.hako"))
    realloc_grow = read_text(hako_file("page_map_realloc_alloc_copy_release_box.hako"))
    remote_policy = read_text(hako_file("remote_free_policy_box.hako"))
    osvm_source = read_text(hako_file("osvm_page_source_pilot_box.hako"))
    huge_model = read_text(hako_file("huge_page_model_box.hako"))
    replacement = read_text(REPLACEMENT_FRONT) + "\n" + read_text(REPLACEMENT_TEMPLATES)

    fixed_slot_front = has_all(
        replacement,
        [
            "HAKO_REPLACEMENT_SLOT_SIZE",
            "direct_alloc_fast",
            "direct_free_local",
            "free_stack",
        ],
    )
    tls_front = has_all(
        replacement,
        [
            "HAKO_REPLACEMENT_FRONT_THREAD_LOCAL",
            "remote_free_to_owner",
            "arena_registry",
        ],
    )
    inplace_realloc = has_all(
        replacement,
        [
            "realloc_inplace_count",
            "if (size <= HAKO_REPLACEMENT_SLOT_SIZE)",
        ],
    )

    direct_array_source = "DirectArrayI64" in page_box
    page_arrays_are_arraybox = has_all(
        page_box,
        [
            "free: ArrayBox",
            "local_free: ArrayBox",
            "block_used: ArrayBox",
        ],
    )

    return [
        CoverageRow(
            area="size_class_policy",
            hako_model=int(has_all(size_class, ["size_to_bin", "bin_size", "huge_bin"])),
            replacement_front=0,
            status="model_only",
            evidence="size_class_box.hako",
            next_bridge="connect size_class_policy to replacement bins/pages",
        ),
        CoverageRow(
            area="page_local_free_stack",
            hako_model=int(has_all(page_box, ["free_top", "acquireFreshSmall", "block_used"])),
            replacement_front=int(fixed_slot_front),
            status="split_model_and_fixed_front",
            evidence="page_box.hako + generated fixed-slot front",
            next_bridge="replace fixed one-size front with page/bin-backed route or prove selected fixture remains fixed-slot only",
        ),
        CoverageRow(
            area="same_thread_local_free",
            hako_model=int(has_all(page_box, ["local_free_top", "releaseLocalKnownLive"])),
            replacement_front=int(fixed_slot_front),
            status="split_model_and_fixed_front",
            evidence="page_box.hako + direct_free_local",
            next_bridge="connect PageModel release/local_free semantics to replacement free route",
        ),
        CoverageRow(
            area="object_lifecycle_hot_core",
            hako_model=int(has_all(hot_core, ["objectLifecycleSmallAlloc", "objectLifecycleReleaseBlock"])),
            replacement_front=0,
            status="model_only",
            evidence="object_lifecycle_hot_core_box.hako",
            next_bridge="consume HotCore/PageModel plans in replacement-front lowering",
        ),
        CoverageRow(
            area="page_map_lookup",
            hako_model=int(has_all(page_map, ["register", "lookup", "unregister"])),
            replacement_front=0,
            status="model_only",
            evidence="page_map_box.hako",
            next_bridge="connect pointer ownership lookup to product replacement route",
        ),
        CoverageRow(
            area="realloc_same_class",
            hako_model=int(has_file(hako_file("page_map_realloc_same_class_box.hako")) and "realloc" in realloc_same.lower()),
            replacement_front=int(inplace_realloc),
            status="split_model_and_fixed_front",
            evidence="page_map_realloc_same_class_box.hako + fixed-slot inplace realloc",
            next_bridge="connect requested-size/slot-class proof to general page-map realloc",
        ),
        CoverageRow(
            area="realloc_grow_copy_release",
            hako_model=int(has_file(hako_file("page_map_realloc_alloc_copy_release_box.hako")) and "copy" in realloc_grow.lower()),
            replacement_front=int("memcpy(next, ptr, copy_size)" in replacement),
            status="split_model_and_fixed_front",
            evidence="page_map_realloc_alloc_copy_release_box.hako + replacement memcpy fallback",
            next_bridge="connect hako realloc grow route to replacement bins/pages",
        ),
        CoverageRow(
            area="remote_free_policy",
            hako_model=int(has_file(hako_file("remote_free_policy_box.hako")) and "remote" in remote_policy.lower()),
            replacement_front=int(tls_front),
            status="split_model_and_fixed_front",
            evidence="remote_free_policy_box.hako + thread-local replacement remote queue",
            next_bridge="align .hako remote-free policy with replacement arena registry route",
        ),
        CoverageRow(
            area="osvm_page_source",
            hako_model=int(has_file(hako_file("osvm_page_source_pilot_box.hako")) and "osvm" in osvm_source.lower()),
            replacement_front=0,
            status="model_only",
            evidence="osvm_page_source_pilot_box.hako",
            next_bridge="connect page source to product allocator, not benchmark-only fixed slots",
        ),
        CoverageRow(
            area="huge_allocation_route",
            hako_model=int(has_file(hako_file("huge_page_model_box.hako")) and "huge" in huge_model.lower()),
            replacement_front=0,
            status="model_only",
            evidence="huge_page_model_box.hako",
            next_bridge="connect huge threshold/page model to replacement route",
        ),
        CoverageRow(
            area="directarray_source_storage",
            hako_model=int(direct_array_source),
            replacement_front=0,
            status="open" if page_arrays_are_arraybox else "source_migrated",
            evidence="page_box.hako",
            next_bridge="migrate hot page arrays from ArrayBox source to DirectArrayI64-backed storage when owner evidence selects it"
            if page_arrays_are_arraybox
            else "measure migrated DirectArrayI64 source route",
        ),
    ]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true", help="emit JSON instead of text")
    parser.add_argument(
        "--benchmark-report",
        type=Path,
        help=(
            "optional hakozuna mixed-ws compare report to overlay executed "
            "benchmark-only replacement-front route fields"
        ),
    )
    parser.add_argument(
        "--fastpath-report",
        type=Path,
        help=(
            "optional hako_check fastpath-explain JSON/KV report to overlay "
            "DirectArray source-route measurement fields"
        ),
    )
    parser.add_argument(
        "--state-report",
        type=Path,
        help=(
            "optional hako_check state-explain KV report to overlay "
            "record-state residence and access-site measurement fields"
        ),
    )
    parser.add_argument(
        "--perf-attribution-report",
        type=Path,
        help=(
            "optional hako-mimalloc-perf-attribution report to overlay "
            "PageModel hot-array perf-delta readiness fields"
        ),
    )
    parser.add_argument(
        "--accumulator-report",
        type=Path,
        help=(
            "optional requested-bytes accumulator contract report to overlay "
            "workload bounded-overflow evidence"
        ),
    )
    args = parser.parse_args()

    data = report_dict(
        build_rows(),
        benchmark_report=args.benchmark_report,
        fastpath_report=args.fastpath_report,
        state_report=args.state_report,
        perf_attribution_report=args.perf_attribution_report,
        accumulator_report=args.accumulator_report,
    )
    if args.json:
        print(json.dumps(data, indent=2, sort_keys=True))
    else:
        emit_text(data)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
