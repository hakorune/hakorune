"""Replacement-front subject report lines for the Hakozuna mixed-ws compare report."""

from __future__ import annotations

from typing import Any

from hakozuna_mixed_ws_report_support import (
    format_per_operation,
    init_fallback_dominates_provider_ops,
)
from hakozuna_mixed_ws_report_replacement_front_subject_core import (
    build_replacement_front_subject_static_lines,
)


def build_replacement_front_subject_lines(
    ctx: dict[str, Any],
    *,
    index: int,
    counters: dict[str, int],
    tls_initial_exec_model_enabled: bool,
) -> list[str]:
    lines = build_replacement_front_subject_static_lines(
        ctx,
        index=index,
        tls_initial_exec_model_enabled=tls_initial_exec_model_enabled,
    )

    if counters:
        for key in sorted(counters):
            lines.append(f"subject_{index}_{key}_total={counters[key]}")
        provider_ops = (
            counters.get("shim_provider_alloc_count", 0)
            + counters.get("shim_provider_calloc_count", 0)
            + counters.get("shim_provider_realloc_count", 0)
            + counters.get("shim_provider_free_count", 0)
        )
        init_fallback_dominates = init_fallback_dominates_provider_ops(counters, provider_ops)
        lines.extend(
            [
                f"subject_{index}_shim_provider_operation_count_total={provider_ops}",
                "subject_"
                f"{index}_shim_init_real_fallback_per_provider_operation="
                f"{format_per_operation(counters.get('shim_init_real_fallback_count', 0), provider_ops)}",
                "subject_"
                f"{index}_shim_host_passthrough_per_provider_operation="
                f"{format_per_operation(counters.get('shim_host_passthrough_count', 0), provider_ops)}",
                "subject_"
                f"{index}_shim_init_real_fallback_dominates_provider_ops="
                f"{1 if init_fallback_dominates else 0}",
            ]
        )
        if init_fallback_dominates:
            lines.extend(
                [
                    "subject_"
                    f"{index}_next_owner_family=provider_alloc_free_internal_real_malloc_boundary",
                    "subject_"
                    f"{index}_gap_classification=provider_bridge_not_hako_core_speed",
                ]
            )
        lines.append(f"subject_{index}_shim_init_real_fallback_is_perf_diagnostic=1")

    return lines
