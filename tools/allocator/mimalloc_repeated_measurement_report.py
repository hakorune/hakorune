"""Report rendering for the repeated mimalloc evidence runner."""

from __future__ import annotations

from pathlib import Path


def render_repeated_measurement_report(
    *,
    args,
    workloads: list[str],
    hako_loadset_plan: dict[str, object],
    workload_records: list[dict[str, object]],
) -> str:
    lines = [
        "mimalloc_repeated_measurement_runner=1",
        "output_contract=mimalloc-comparison-repeated-measurement-v0",
        "measurement_profile=phase295x-repeated-v0",
        f"warmup_count={args.warmup_count}",
        f"sample_count={args.sample_count}",
        f"operation_repeat={args.operation_repeat}",
        "timing_repeat_kind=process-invocation-v0",
        f"workload_count={len(workloads)}",
        "workloads=" + ",".join(workloads),
        "summary_statistic=min,median,max",
        "canonical_rss_collector=external-time",
        "internal_rss_evidence=preserved",
        f"hako_runtime_config_profile={args.hako_runtime_config}",
        "hako_runtime_config_default=empty",
        f"hako_selected_loadset={hako_loadset_plan['selected_loadset']}",
        f"hako_plugin_load_policy={hako_loadset_plan['plugin_load_policy']}",
        f"hako_selected_library_count={hako_loadset_plan['library_count']}",
        f"hako_missing_library_count={hako_loadset_plan['missing_library_count']}",
        f"hako_loadset_preflight_ok={hako_loadset_plan['preflight_ok']}",
        f"c_library_path={args.c_library if args.c_library is not None else 'ldconfig-discovery'}",
    ]

    for workload_index, record in enumerate(workload_records):
        workload = str(record["workload"])
        prefix = f"workload_{workload_index}"
        lines.extend(
            [
                f"{prefix}_id={workload}",
                f"{prefix}_operation_family={record['operation_family']}",
                f"{prefix}_operation_repeat={args.operation_repeat}",
                f"{prefix}_timing_repeat_kind=process-invocation-v0",
                f"{prefix}_sample_count={args.sample_count}",
                f"{prefix}_hako_external_rss_min_bytes={record['hako_external_rss_min']}",
                f"{prefix}_hako_external_rss_median_bytes={record['hako_external_rss_median']}",
                f"{prefix}_hako_external_rss_max_bytes={record['hako_external_rss_max']}",
                f"{prefix}_c_external_rss_min_bytes={record['c_external_rss_min']}",
                f"{prefix}_c_external_rss_median_bytes={record['c_external_rss_median']}",
                f"{prefix}_c_external_rss_max_bytes={record['c_external_rss_max']}",
                f"{prefix}_hako_external_elapsed_min_ms={record['hako_external_elapsed_min']}",
                f"{prefix}_hako_external_elapsed_median_ms={record['hako_external_elapsed_median']}",
                f"{prefix}_hako_external_elapsed_max_ms={record['hako_external_elapsed_max']}",
                f"{prefix}_c_external_elapsed_min_ms={record['c_external_elapsed_min']}",
                f"{prefix}_c_external_elapsed_median_ms={record['c_external_elapsed_median']}",
                f"{prefix}_c_external_elapsed_max_ms={record['c_external_elapsed_max']}",
                f"{prefix}_hako_internal_rss_median_bytes={record['hako_internal_rss_median']}",
                f"{prefix}_c_internal_rss_median_bytes={record['c_internal_rss_median']}",
                f"{prefix}_winner_claim=0",
            ]
        )

    lines.extend(
        [
            "provider_activation=0",
            "host_replacement=0",
            "hook_installed=0",
            "global_allocator_installed=0",
            "winner_claim=0",
            "summary=ok",
        ]
    )
    return "\n".join(lines) + "\n"
