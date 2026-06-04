"""Gap-summary section builders for provider and replacement-front subjects."""

from __future__ import annotations

from typing import Mapping

from hakozuna_mixed_ws_gap_summary_support import as_int, ratio, slower_percent


def build_provider_section(
    values: Mapping[str, str],
    provider: Mapping[str, str] | None,
    *,
    glibc_median: float,
    mimalloc_median: float,
) -> list[str]:
    if provider is None:
        return []

    provider_median = float(provider["throughput_median_ops_per_sec"])
    provider_declared_route = provider.get("declared_route", "unknown")
    provider_execution_route = provider.get("execution_route", "unknown")
    provider_front_class = provider.get("benchmark_front_class", "unknown")
    provider_hako_hot_path_claim = provider.get("hako_hot_path_claim", "0")
    provider_registration_present = values.get("provider_registration_v1_present", "0")
    provider_registration_hot_path_uses = values.get("provider_registration_hot_path_uses", "unknown")
    provider_registration_type_lookup_count = values.get(
        "provider_registration_type_abi_hot_path_lookup_count", "unknown"
    )
    provider_ops_version = values.get("provider_ops_version", "unknown")
    provider_kind = values.get("provider_kind", "unknown")
    provider_claim_ops_enabled = values.get("provider_claim_ops_enabled", "0")
    provider_ops = as_int(provider, "shim_provider_operation_count_total")
    init_fallback = as_int(provider, "shim_init_real_fallback_count_total")
    init_fallback_in_provider = as_int(
        provider, "shim_init_fallback_in_provider_call_count_total"
    )
    host_passthrough = as_int(provider, "shim_host_passthrough_count_total")
    runtime_fallback = as_int(provider, "shim_runtime_real_fallback_count_total")
    pointer_overflow = as_int(provider, "shim_pointer_table_overflow_total")
    free_claim = as_int(provider, "shim_provider_free_claim_count_total")
    free_not_owned = as_int(provider, "shim_provider_free_not_owned_count_total")
    free_claim_bound = as_int(provider, "shim_provider_free_claim_bound_total")
    realloc_claim = as_int(provider, "shim_provider_realloc_claim_count_total")
    realloc_not_owned = as_int(provider, "shim_provider_realloc_not_owned_count_total")
    realloc_failed = as_int(provider, "shim_provider_realloc_failed_count_total")
    realloc_claim_bound = as_int(provider, "shim_provider_realloc_claim_bound_total")
    usable_size_claim = as_int(provider, "shim_provider_usable_size_claim_count_total")
    usable_size_not_owned = as_int(provider, "shim_provider_usable_size_not_owned_count_total")
    usable_size_claim_bound = as_int(provider, "shim_provider_usable_size_claim_bound_total")
    host_allocator_init_bound = as_int(provider, "shim_host_allocator_init_bound_total")
    host_allocator_init_result = as_int(provider, "shim_host_allocator_init_result_total")
    host_allocator_vtable_init = as_int(provider, "shim_host_allocator_vtable_init_count_total")
    host_allocator_usable_size_bound = as_int(
        provider, "shim_host_allocator_usable_size_bound_total"
    )
    claim_mainline = as_int(provider, "shim_claim_mainline_mode_enabled_total")
    tracking_insert = as_int(provider, "shim_track_probe_total_total")
    tracking_lookup = as_int(provider, "shim_find_probe_total_total")
    next_owner = provider.get("next_owner_family", "unknown")
    return [
        f"provider_median_ops_per_sec={provider_median:.3f}",
        f"provider_declared_route={provider_declared_route}",
        f"provider_execution_route={provider_execution_route}",
        f"provider_benchmark_front_class={provider_front_class}",
        f"provider_hako_hot_path_claim={provider_hako_hot_path_claim}",
        f"provider_registration_v1_present={provider_registration_present}",
        f"provider_registration_hot_path_uses={provider_registration_hot_path_uses}",
        "provider_registration_type_abi_hot_path_lookup_count="
        f"{provider_registration_type_lookup_count}",
        f"provider_ops_version={provider_ops_version}",
        f"provider_kind={provider_kind}",
        f"provider_claim_ops_enabled={provider_claim_ops_enabled}",
        f"provider_vs_mimalloc_ratio={ratio(provider_median, mimalloc_median)}",
        "provider_slower_than_mimalloc_percent="
        f"{slower_percent(provider_median, mimalloc_median)}",
        f"provider_vs_glibc_ratio={ratio(provider_median, glibc_median)}",
        f"provider_operation_count_total={provider_ops}",
        f"provider_init_real_fallback_count_total={init_fallback}",
        f"provider_init_fallback_in_provider_call_count_total={init_fallback_in_provider}",
        f"provider_host_passthrough_count_total={host_passthrough}",
        f"provider_runtime_real_fallback_count_total={runtime_fallback}",
        f"provider_pointer_table_overflow_total={pointer_overflow}",
        f"provider_free_claim_count_total={free_claim}",
        f"provider_free_not_owned_count_total={free_not_owned}",
        f"provider_free_claim_bound_total={free_claim_bound}",
        f"provider_realloc_claim_count_total={realloc_claim}",
        f"provider_realloc_not_owned_count_total={realloc_not_owned}",
        f"provider_realloc_failed_count_total={realloc_failed}",
        f"provider_realloc_claim_bound_total={realloc_claim_bound}",
        f"provider_usable_size_claim_count_total={usable_size_claim}",
        f"provider_usable_size_not_owned_count_total={usable_size_not_owned}",
        f"provider_usable_size_claim_bound_total={usable_size_claim_bound}",
        f"provider_host_allocator_init_bound_total={host_allocator_init_bound}",
        f"provider_host_allocator_init_result_total={host_allocator_init_result}",
        f"provider_host_allocator_vtable_init_count_total={host_allocator_vtable_init}",
        f"provider_host_allocator_usable_size_bound_total={host_allocator_usable_size_bound}",
        f"provider_claim_mainline_mode_enabled_total={claim_mainline}",
        f"shim_tracking_insert_probe_total={tracking_insert}",
        f"shim_tracking_lookup_probe_total={tracking_lookup}",
        "shim_provider_owned_truth=0",
        "shim_owns_precheck_hot_path=0",
        "provider_init_real_fallback_per_provider_operation="
        f"{ratio(float(init_fallback), float(provider_ops)) if provider_ops > 0 else 'nan'}",
        "provider_host_passthrough_per_provider_operation="
        f"{ratio(float(host_passthrough), float(provider_ops)) if provider_ops > 0 else 'nan'}",
        f"provider_next_owner_family={next_owner}",
    ]


def build_replacement_front_section(
    replacement_front: Mapping[str, str] | None,
    *,
    glibc_median: float,
    mimalloc_median: float,
) -> list[str]:
    if replacement_front is None:
        return []

    replacement_median = float(replacement_front["throughput_median_ops_per_sec"])
    replacement_front_class = replacement_front.get("benchmark_front_class", "unknown")
    replacement_execution_route = replacement_front.get("execution_route", "unknown")
    replacement_direct_core = replacement_front.get("direct_core_call", "0")
    replacement_provider_dispatch = replacement_front.get("provider_table_dispatch", "unknown")
    replacement_provider_api_required = replacement_front.get(
        "provider_api_hot_path_required", "unknown"
    )
    replacement_tracking_hot_path = replacement_front.get("tracking_hot_path", "unknown")
    replacement_benchmark_only = replacement_front.get("benchmark_only", "unknown")
    replacement_ordinary_app_route = replacement_front.get(
        "replacement_front_ordinary_app_route_candidate", "unknown"
    )
    replacement_product_gate = replacement_front.get("replacement_front_product_gate", "unknown")
    replacement_product_activation_ready = replacement_front.get(
        "replacement_front_product_activation_ready", "unknown"
    )
    replacement_product_activation_contract = replacement_front.get(
        "replacement_front_product_activation_contract_v0", "unknown"
    )
    replacement_product_activation_blockers = replacement_front.get(
        "replacement_front_product_activation_blockers", "unknown"
    )
    return [
        f"replacement_front_median_ops_per_sec={replacement_median:.3f}",
        f"replacement_front_execution_route={replacement_execution_route}",
        "replacement_front_ordinary_app_route_candidate="
        f"{replacement_ordinary_app_route}",
        f"replacement_front_product_gate={replacement_product_gate}",
        "replacement_front_product_activation_ready="
        f"{replacement_product_activation_ready}",
        "replacement_front_product_activation_contract_v0="
        f"{replacement_product_activation_contract}",
        "replacement_front_product_activation_requires_quality_ok="
        f"{replacement_front.get('replacement_front_product_activation_requires_quality_ok', 'unknown')}",
        "replacement_front_product_activation_requires_provider_dispatch_bypass="
        f"{replacement_front.get('replacement_front_product_activation_requires_provider_dispatch_bypass', 'unknown')}",
        "replacement_front_product_activation_requires_type_abi_hot_lookup_zero="
        f"{replacement_front.get('replacement_front_product_activation_requires_type_abi_hot_lookup_zero', 'unknown')}",
        "replacement_front_product_activation_requires_cross_thread_policy="
        f"{replacement_front.get('replacement_front_product_activation_requires_cross_thread_policy', 'unknown')}",
        "replacement_front_product_activation_requires_remote_abandoned_counters="
        f"{replacement_front.get('replacement_front_product_activation_requires_remote_abandoned_counters', 'unknown')}",
        "replacement_front_product_activation_requires_rollback_optout_plan="
        f"{replacement_front.get('replacement_front_product_activation_requires_rollback_optout_plan', 'unknown')}",
        "replacement_front_rollback_optout_plan_v0="
        f"{replacement_front.get('replacement_front_rollback_optout_plan_v0', 'unknown')}",
        "replacement_front_rollback_optout_env="
        f"{replacement_front.get('replacement_front_rollback_optout_env', 'unknown')}",
        "replacement_front_rollback_optout_env_value="
        f"{replacement_front.get('replacement_front_rollback_optout_env_value', 'unknown')}",
        "replacement_front_per_process_disable="
        f"{replacement_front.get('replacement_front_per_process_disable', 'unknown')}",
        "replacement_front_activation_mode="
        f"{replacement_front.get('replacement_front_activation_mode', 'unknown')}",
        "replacement_front_activation_default="
        f"{replacement_front.get('replacement_front_activation_default', 'unknown')}",
        "replacement_front_activation_report_required="
        f"{replacement_front.get('replacement_front_activation_report_required', 'unknown')}",
        "replacement_front_rollback_report_path_required="
        f"{replacement_front.get('replacement_front_rollback_report_path_required', 'unknown')}",
        "replacement_front_product_activation_blockers="
        f"{replacement_product_activation_blockers}",
        "replacement_front_product_preflight_report_v0="
        f"{replacement_front.get('replacement_front_product_preflight_report_v0', 'unknown')}",
        "replacement_front_product_preflight_non_activating="
        f"{replacement_front.get('replacement_front_product_preflight_non_activating', 'unknown')}",
        "replacement_front_product_preflight_evidence_ready="
        f"{replacement_front.get('replacement_front_product_preflight_evidence_ready', 'unknown')}",
        "replacement_front_product_preflight_activation_ready="
        f"{replacement_front.get('replacement_front_product_preflight_activation_ready', 'unknown')}",
        "replacement_front_product_preflight_quality_ok="
        f"{replacement_front.get('replacement_front_product_preflight_quality_ok', 'unknown')}",
        "replacement_front_product_preflight_provider_dispatch_bypass_ok="
        f"{replacement_front.get('replacement_front_product_preflight_provider_dispatch_bypass_ok', 'unknown')}",
        "replacement_front_product_preflight_type_abi_hot_lookup_zero_ok="
        f"{replacement_front.get('replacement_front_product_preflight_type_abi_hot_lookup_zero_ok', 'unknown')}",
        "replacement_front_product_preflight_cross_thread_policy_ok="
        f"{replacement_front.get('replacement_front_product_preflight_cross_thread_policy_ok', 'unknown')}",
        "replacement_front_product_preflight_remote_abandoned_counters_ok="
        f"{replacement_front.get('replacement_front_product_preflight_remote_abandoned_counters_ok', 'unknown')}",
        "replacement_front_product_preflight_rollback_optout_ok="
        f"{replacement_front.get('replacement_front_product_preflight_rollback_optout_ok', 'unknown')}",
        "replacement_front_product_preflight_missing="
        f"{replacement_front.get('replacement_front_product_preflight_missing', 'unknown')}",
        f"replacement_front_benchmark_front_class={replacement_front_class}",
        f"replacement_front_vs_mimalloc_ratio={ratio(replacement_median, mimalloc_median)}",
        "replacement_front_slower_than_mimalloc_percent="
        f"{slower_percent(replacement_median, mimalloc_median)}",
        f"replacement_front_vs_glibc_ratio={ratio(replacement_median, glibc_median)}",
        f"replacement_front_bypasses_type_abi=1",
        f"replacement_front_bypasses_provider_dispatch={1 if replacement_provider_dispatch == '0' else 0}",
        f"replacement_front_provider_table_dispatch={replacement_provider_dispatch}",
        f"replacement_front_provider_api_hot_path_required={replacement_provider_api_required}",
        f"replacement_front_tracking_hot_path={replacement_tracking_hot_path}",
        f"replacement_front_direct_core_call={replacement_direct_core}",
        f"replacement_front_benchmark_only={replacement_benchmark_only}",
        "replacement_front_product_claim=0",
    ]
