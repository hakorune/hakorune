"""Render the Hakozuna mixed-ws gap ladder summary report."""

from __future__ import annotations

from pathlib import Path

from hakozuna_mixed_ws_gap_summary_support import (
    append_if_present,
    read_kv,
    ratio,
    subject_rows,
    require,
)
from hakozuna_mixed_ws_gap_summary_sections import (
    build_provider_section,
    build_replacement_front_section,
)


def emit_summary(compare_report: Path) -> str:
    values = read_kv(compare_report)
    require(values, "output_contract", "hakozuna-mixed-ws-ldpreload-compare-v0", "compare")
    require(values, "winner_claim", "0", "compare")
    rows = subject_rows(values)

    glibc = rows.get("system_malloc")
    mimalloc = rows.get("c_mimalloc_ldpreload")
    provider = rows.get("hakorune_provider_ldpreload")
    replacement_front = rows.get("hakorune_replacement_front_ldpreload")
    if glibc is None:
        raise SystemExit("compare report missing system_malloc subject")
    if mimalloc is None:
        raise SystemExit("compare report missing c_mimalloc_ldpreload subject")

    glibc_median = float(glibc["throughput_median_ops_per_sec"])
    mimalloc_median = float(mimalloc["throughput_median_ops_per_sec"])

    lines = [
        "output_contract=hakozuna-mixed-ws-gap-ladder-v0",
        "input_contract=hakozuna-mixed-ws-ldpreload-compare-v0",
        f"compare_report={compare_report}",
        f"type_abi_route_descriptor_present={values.get('type_abi_route_descriptor_present', '0')}",
        f"type_abi_descriptor_plane={values.get('type_abi_descriptor_plane', 'unknown')}",
        f"type_abi_hot_path_lookup_count={values.get('type_abi_hot_path_lookup_count', 'unknown')}",
        f"benchmark_threads={values.get('benchmark_threads', 'unknown')}",
        f"benchmark_iters_per_thread={values.get('benchmark_iters_per_thread', 'unknown')}",
        f"benchmark_working_set={values.get('benchmark_working_set', 'unknown')}",
        f"sample_count={values.get('sample_count', 'unknown')}",
        f"warmup_count={values.get('warmup_count', 'unknown')}",
        "min_sample_seconds_required="
        f"{values.get('min_sample_seconds_required', 'unknown')}",
        "min_observed_sample_seconds="
        f"{values.get('min_observed_sample_seconds', 'unknown')}",
        "median_observed_sample_seconds="
        f"{values.get('median_observed_sample_seconds', 'unknown')}",
        f"measurement_quality={values.get('measurement_quality', 'unknown')}",
        f"glibc_median_ops_per_sec={glibc_median:.3f}",
        f"system_mimalloc_median_ops_per_sec={mimalloc_median:.3f}",
        f"glibc_vs_mimalloc_ratio={ratio(glibc_median, mimalloc_median)}",
        "provider_subject_present=" + ("1" if provider is not None else "0"),
        "replacement_front_subject_present="
        + ("1" if replacement_front is not None else "0"),
        f"provider_usable_size_mode={values.get('provider_usable_size_mode', '0')}",
        f"provider_assume_owned_mode={values.get('provider_assume_owned_mode', '0')}",
        "same_benchmark_binary=1",
        "same_workload=1",
        "same_threads=1",
        "same_iters_per_thread=1",
        "same_working_set=1",
        "same_sample_count=1",
    ]

    for key in (
        "replacement_front_product_smoke_pack_v0",
        "replacement_front_product_smoke_pack_non_activating",
        "replacement_front_malloc_family_smoke_ok",
        "replacement_front_malloc_family_null_free_smoke_ok",
        "replacement_front_malloc_family_host_passthrough_count",
        "replacement_front_cross_thread_free_smoke_ok",
        "replacement_front_abandoned_owner_smoke_ok",
        "replacement_front_cross_thread_realloc_smoke_ok",
        "replacement_front_cross_thread_free_policy",
        "replacement_front_cross_thread_realloc_policy",
        "replacement_front_cross_thread_free_remote_free_push_count",
        "replacement_front_cross_thread_free_remote_free_drain_count",
        "replacement_front_abandoned_owner_abandoned_arena_count",
        "replacement_front_abandoned_owner_abandoned_remote_free_count",
        "replacement_front_cross_thread_realloc_unsupported_count",
        "replacement_front_cross_thread_realloc_host_passthrough_count",
        "provider_registration_v1_present",
        "provider_registration_descriptor_plane",
        "provider_registration_ops_plane",
        "provider_registration_descriptor_ops_pairing",
        "provider_registration_hot_path_uses",
        "provider_registration_type_abi_hot_path_lookup_count",
        "provider_ops_version",
        "provider_kind",
        "provider_claim_ops_enabled",
        "provider_ldpreload_declared_package_origin",
        "provider_ldpreload_declared_route",
        "provider_ldpreload_execution_route",
        "provider_ldpreload_measurement_route",
        "provider_ldpreload_provider_allocator_kind",
        "provider_ldpreload_alloc_free_route",
        "provider_ldpreload_uses_host_malloc",
        "provider_ldpreload_uses_hako_object_lifecycle",
        "provider_ldpreload_object_lifecycle_entrypoint_usage",
        "provider_ldpreload_hako_hot_path_claim",
        "provider_ldpreload_hako_object_lifecycle_hot_path",
        "provider_ldpreload_hako_object_lifecycle_metadata_only",
        "provider_manifest_hako_semantic_provider_codegen",
        "provider_manifest_hako_provider_object_lifecycle_entrypoint_verified",
        "provider_manifest_hako_provider_alloc_free_route",
        "provider_manifest_provider_allocator_kind",
        "provider_manifest_provider_abi_claim_ops_v1",
        "provider_manifest_provider_free_claim_enabled",
        "provider_manifest_provider_realloc_claim_enabled",
        "provider_manifest_provider_usable_size_claim_enabled",
        "provider_manifest_compat_alloc_free_owns_still_supported",
        "provider_manifest_compat_owns_free_mainline",
        "provider_manifest_host_allocator_vtable_init",
        "provider_manifest_hako_provider_alloc_free_uses_host_malloc",
        "provider_manifest_hako_provider_alloc_free_uses_hako_object_lifecycle",
        "provider_manifest_hako_provider_object_lifecycle_entrypoint_usage",
    ):
        append_if_present(lines, values, key)

    lines.extend(
        build_provider_section(
            values,
            provider,
            glibc_median=glibc_median,
            mimalloc_median=mimalloc_median,
        )
    )

    lines.extend(
        build_replacement_front_section(
            replacement_front,
            glibc_median=glibc_median,
            mimalloc_median=mimalloc_median,
        )
    )

    lines.extend(
        [
            "provider_activation=0",
            "production_replacement_active=0",
            "hook_installed=0",
            "global_allocator_product_claim=0",
            "winner_claim=0",
            "summary="
            f"{'ok' if values.get('measurement_quality', 'ok') == 'ok' else 'measurement_too_short'}",
        ]
    )
    return "\n".join(lines) + "\n"
