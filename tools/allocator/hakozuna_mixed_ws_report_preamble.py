"""Report preamble assembly for the Hakozuna mixed-ws compare report."""

from __future__ import annotations

from typing import Any

from hakozuna_mixed_ws_report_smoke_fields import replacement_front_smoke_pack_fields
from hakozuna_mixed_ws_report_preamble_workload import (
    build_report_preamble_workload_lines,
)
from hakozuna_mixed_ws_report_support import provider_front_class, provider_kind_from_route
from replacement_front_report import (
    product_activation_contract_fields,
    product_preflight_fields,
)


def build_report_preamble_lines(ctx: dict[str, Any]) -> list[str]:
    args = ctx["args"]
    bench = ctx["bench"]
    root = ctx["root"]
    mimalloc_library = ctx["mimalloc_library"]
    replacement_front_smokes = ctx["replacement_front_smokes"]
    provider_manifest_metadata = ctx["provider_manifest_metadata"]
    provider_route_metadata = ctx["provider_route_metadata"]
    provider_declared_route = ctx["provider_declared_route"]
    provider_execution_route = ctx["provider_execution_route"]

    measurement_quality = ctx["measurement_quality"]
    min_observed_sample_seconds = ctx["min_observed_sample_seconds"]
    median_observed_sample_seconds = ctx["median_observed_sample_seconds"]
    replacement_front_algorithm_shape = ctx["replacement_front_algorithm_shape"]
    replacement_front_preflight = ctx["replacement_front_preflight"]

    lines = [
        "output_contract=hakozuna-mixed-ws-ldpreload-compare-v0",
        "type_abi_route_descriptor_present=1",
        "type_abi_descriptor_plane=route_descriptor_control_plane",
        "type_abi_hot_path_lookup_count=0",
        "benchmark_id=bench_mixed_ws_crt",
        f"benchmark_path={bench}",
        f"hakozuna_root={root}",
        f"mimalloc_library={mimalloc_library}",
        f"provider_manifest={args.manifest.resolve() if args.manifest is not None else 'none'}",
        f"sample_count={args.sample_count}",
        f"warmup_count={args.warmup_count}",
        f"min_sample_seconds_required={args.min_sample_seconds:.6f}",
        f"min_observed_sample_seconds={min_observed_sample_seconds:.6f}",
        f"median_observed_sample_seconds={median_observed_sample_seconds:.6f}",
        f"measurement_quality={measurement_quality}",
        f"benchmark_threads={args.threads}",
        f"benchmark_iters_per_thread={args.iters_per_thread}",
        f"benchmark_working_set={args.working_set}",
        f"benchmark_min_size={args.min_size}",
        f"benchmark_max_size={args.max_size}",
        f"subject_count={ctx['subject_count']}",
        "reference_subject=c_mimalloc_ldpreload",
        "provider_activation=0",
        "production_replacement_active=0",
        "hook_installed=0",
        "global_allocator_product_claim=0",
        "winner_claim=0",
        f"provider_usable_size_mode={1 if args.provider_usable_size_mode else 0}",
        f"provider_assume_owned_mode={1 if args.provider_assume_owned_mode else 0}",
        f"replacement_front_native_slot_mode={1 if args.replacement_front_native_slot_mode else 0}",
        f"replacement_front_native_bins_mode={1 if args.replacement_front_native_bins_mode else 0}",
        f"replacement_front_page_bins_mode={1 if args.replacement_front_page_bins_mode else 0}",
        "replacement_front_hotcore_page_model_mode="
        f"{1 if args.replacement_front_hotcore_page_model_mode else 0}",
        "replacement_front_size_class_table_mode="
        f"{1 if args.replacement_front_size_class_table_mode else 0}",
        "replacement_front_eager_init_mode="
        f"{1 if args.replacement_front_eager_init_mode else 0}",
        "replacement_front_product_pages_nonlinear_mode="
        f"{1 if args.replacement_front_product_pages_nonlinear_mode else 0}",
        "replacement_front_is_full_hako_algorithm=0",
        "replacement_front_ordinary_app_route_candidate=replacement_front_product_ldpreload",
        *product_activation_contract_fields(),
        *product_preflight_fields(replacement_front_preflight),
        f"replacement_front_algorithm_shape={replacement_front_algorithm_shape}",
        *build_report_preamble_workload_lines(ctx),
    ]
    lines.extend(replacement_front_smoke_pack_fields(replacement_front_smokes))
    for key in sorted(provider_manifest_metadata):
        lines.append(f"{key}={provider_manifest_metadata[key]}")
    for key in sorted(provider_route_metadata):
        lines.append(f"{key}={provider_route_metadata[key]}")
    if args.manifest is not None:
        provider_measurement_route = provider_route_metadata.get(
            "provider_ldpreload_measurement_route", ""
        )
        lines.extend(
            [
                f"provider_declared_route={provider_declared_route}",
                f"provider_execution_route={provider_execution_route}",
                "provider_ldpreload_benchmark_front_class="
                f"{provider_front_class(provider_measurement_route)}",
                "provider_ldpreload_measurement_interpretation=provider_abi_wrapper_and_shim_bridge",
                "provider_ldpreload_is_product_allocator_claim=0",
                "provider_ldpreload_is_hako_core_speed_claim=0",
                "provider_registration_v1_present=1",
                "provider_registration_descriptor_plane=type_abi_route_descriptor",
                "provider_registration_ops_plane=provider_abi_execution_ops",
                "provider_registration_report_pairing=1",
                "provider_registration_descriptor_ops_pairing=1",
                "provider_registration_hot_path_uses=provider_ops_only",
                "provider_registration_type_abi_hot_path_lookup_count=0",
                "provider_ops_version=1",
                "provider_claim_ops_enabled="
                f"{provider_manifest_metadata.get('provider_manifest_provider_abi_claim_ops_v1', '0')}",
                f"provider_kind={provider_kind_from_route(provider_measurement_route)}",
            ]
        )
    return lines
