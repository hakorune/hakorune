#!/usr/bin/env python3
"""Run repeated explicit provider alloc/free measurement without winner claims."""

from __future__ import annotations

import argparse
import resource
import time
from pathlib import Path

from provider_package_api_bind_smoke import init_host_allocator_if_enabled, load_api, run


def median_int(values: list[int]) -> int:
    ordered = sorted(values)
    return ordered[len(ordered) // 2]


def rss_bytes() -> int:
    return int(resource.getrusage(resource.RUSAGE_SELF).ru_maxrss) * 1024


def run_alloc_free(api, operation_repeat: int, size: int, align: int) -> tuple[int, int, int]:
    allocation_count = 0
    free_count = 0
    requested_bytes = 0
    for _ in range(operation_repeat):
        ptr = int(api.alloc(size, align) or 0)
        if ptr == 0:
            raise SystemExit("[provider-package-metadata-preflight] provider alloc returned null")
        if int(api.owns(ptr)) != 1:
            raise SystemExit("[provider-package-metadata-preflight] provider owns(ptr) did not return 1")
        allocation_count += 1
        requested_bytes += size
        api.free(ptr)
        free_count += 1
    return allocation_count, free_count, requested_bytes


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--sample-count", type=int, default=3)
    parser.add_argument("--warmup-count", type=int, default=1)
    parser.add_argument("--operation-repeat", type=int, default=128)
    parser.add_argument("--size", type=int, default=32)
    parser.add_argument("--align", type=int, default=8)
    args = parser.parse_args()
    if args.sample_count < 1:
        raise SystemExit("--sample-count must be positive")
    if args.warmup_count < 0:
        raise SystemExit("--warmup-count must be non-negative")
    if args.operation_repeat < 1:
        raise SystemExit("--operation-repeat must be positive")

    manifest_path = args.manifest.resolve()
    fields, descriptor, api_info, binary_path = run(manifest_path)
    api = load_api(binary_path)
    host_allocator_init_result = init_host_allocator_if_enabled(api, fields)
    if fields.get("host_allocator_vtable_init") == "1" and host_allocator_init_result != 1:
        raise SystemExit("[provider-package-metadata-preflight] host allocator init failed")

    sample_elapsed_ns: list[int] = []
    sample_rss_bytes: list[int] = []
    total_allocations = 0
    total_frees = 0
    total_requested_bytes = 0

    total_runs = args.warmup_count + args.sample_count
    lines = [
        "output_contract=hakorune-provider-explicit-repeated-measurement-v0",
        "measurement_profile=phase296x-provider-explicit-repeated-v0",
        "dll_mode=provider-explicit-repeated-measurement",
        f"source_path={manifest_path}",
        f"binary_path={binary_path}",
        f"schema_version={fields['schema_version']}",
        f"provider_name={fields['provider_name']}",
        f"abi={fields['abi']}",
        f"target={fields['target']}",
        f"profile={fields['profile']}",
        f"binary={fields['binary']}",
        f"binary_sha256={fields['binary_sha256']}",
        f"contract_hash={fields['contract_hash']}",
        f"descriptor_provider_id={descriptor['provider_id']}",
        f"descriptor_provider_kind={descriptor['provider_kind']}",
        f"api_abi_major={api_info['api_abi_major']}",
        f"api_table_size={api_info['api_table_size']}",
        f"host_allocator_vtable_init={fields.get('host_allocator_vtable_init', '0')}",
        f"host_allocator_init_bound={api_info.get('host_allocator_init_bound', '0')}",
        f"host_allocator_init_result={host_allocator_init_result}",
        f"warmup_count={args.warmup_count}",
        f"sample_count={args.sample_count}",
        f"operation_repeat={args.operation_repeat}",
        f"request_size={args.size}",
        f"request_align={args.align}",
        "timing_repeat_kind=in-process-provider-loop-v0",
        "summary_statistic=min,median,max",
        "manifest_ready=1",
        "descriptor_ready=1",
        "binary_hash_ready=1",
        "shared_library_load_executed=1",
        "required_export_resolved=1",
        "descriptor_read_executed=1",
        "provider_api_bound=1",
        "provider_call_executed=1",
        "allocator_entrypoint_called=1",
        "provider_alloc_executed=1",
        "provider_free_executed=1",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "winner_claim=0",
    ]

    for run_index in range(total_runs):
        kind = "warmup" if run_index < args.warmup_count else "sample"
        start = time.perf_counter_ns()
        allocs, frees, requested = run_alloc_free(api, args.operation_repeat, args.size, args.align)
        elapsed = time.perf_counter_ns() - start
        current_rss = rss_bytes()
        total_allocations += allocs
        total_frees += frees
        total_requested_bytes += requested
        if kind == "sample":
            sample_index = run_index - args.warmup_count
            sample_elapsed_ns.append(elapsed)
            sample_rss_bytes.append(current_rss)
            lines.extend(
                [
                    f"sample_{sample_index}_elapsed_ns={elapsed}",
                    f"sample_{sample_index}_rss_bytes={current_rss}",
                    f"sample_{sample_index}_allocation_count={allocs}",
                    f"sample_{sample_index}_free_count={frees}",
                    f"sample_{sample_index}_winner_claim=0",
                ]
            )

    lines.extend(
        [
            f"allocation_count={total_allocations}",
            f"free_count={total_frees}",
            f"requested_bytes={total_requested_bytes}",
            f"sample_elapsed_min_ns={min(sample_elapsed_ns)}",
            f"sample_elapsed_median_ns={median_int(sample_elapsed_ns)}",
            f"sample_elapsed_max_ns={max(sample_elapsed_ns)}",
            f"sample_rss_min_bytes={min(sample_rss_bytes)}",
            f"sample_rss_median_bytes={median_int(sample_rss_bytes)}",
            f"sample_rss_max_bytes={max(sample_rss_bytes)}",
            "summary=ok",
        ]
    )
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
