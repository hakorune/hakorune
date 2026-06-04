#!/usr/bin/env python3
"""Report-only fields for the benchmark replacement front.

This module owns descriptor/control-plane text emitted by allocator benchmark
reports. It must not build shims, call Provider ABI operations, or change
allocator behavior.
"""

from __future__ import annotations

from dataclasses import dataclass


PRODUCT_ACTIVATION_BLOCKERS = "benchmark_only,product_gate_closed,no_activation_row"
ROLLBACK_OPTOUT_ENV = "HAKORUNE_REPLACEMENT_FRONT_DISABLE"


@dataclass(frozen=True)
class ReplacementFrontPreflight:
    evidence_ready: int
    quality_ok: int
    provider_dispatch_bypass_ok: int
    type_abi_hot_lookup_zero_ok: int
    cross_thread_policy_ok: int
    remote_abandoned_counters_ok: int
    rollback_optout_ok: int
    missing_fields: str

    @classmethod
    def from_evidence(
        cls,
        *,
        measurement_quality: str,
        has_smoke_pack: bool,
        thread_local_mode: bool,
        cross_thread_smoke: bool,
        provider_dispatch_bypass: bool,
    ) -> "ReplacementFrontPreflight":
        quality_ok = int(measurement_quality == "ok")
        cross_thread_policy_ok = int(
            has_smoke_pack and thread_local_mode and cross_thread_smoke
        )
        remote_abandoned_counters_ok = int(has_smoke_pack)
        provider_dispatch_bypass_ok = int(provider_dispatch_bypass)
        type_abi_hot_lookup_zero_ok = 1
        rollback_optout_ok = 1
        evidence_ready = int(
            quality_ok
            and provider_dispatch_bypass_ok
            and type_abi_hot_lookup_zero_ok
            and cross_thread_policy_ok
            and remote_abandoned_counters_ok
            and rollback_optout_ok
        )
        missing = [
            "product_gate_open",
            "activation_row",
        ]
        if not quality_ok:
            missing.append("quality_ok_measurement")
        if not provider_dispatch_bypass_ok:
            missing.append("provider_dispatch_bypass")
        if not type_abi_hot_lookup_zero_ok:
            missing.append("type_abi_hot_lookup_zero")
        if not cross_thread_policy_ok:
            missing.append("cross_thread_policy")
        if not remote_abandoned_counters_ok:
            missing.append("remote_abandoned_counters")
        if not rollback_optout_ok:
            missing.append("rollback_optout_plan")
        return cls(
            evidence_ready=evidence_ready,
            quality_ok=quality_ok,
            provider_dispatch_bypass_ok=provider_dispatch_bypass_ok,
            type_abi_hot_lookup_zero_ok=type_abi_hot_lookup_zero_ok,
            cross_thread_policy_ok=cross_thread_policy_ok,
            remote_abandoned_counters_ok=remote_abandoned_counters_ok,
            rollback_optout_ok=rollback_optout_ok,
            missing_fields=",".join(missing),
        )


def product_activation_contract_fields() -> list[str]:
    return [
        "replacement_front_product_gate=closed",
        "replacement_front_product_activation_ready=0",
        "replacement_front_product_activation_contract_v0=1",
        "replacement_front_product_activation_requires_quality_ok=1",
        "replacement_front_product_activation_requires_provider_dispatch_bypass=1",
        "replacement_front_product_activation_requires_type_abi_hot_lookup_zero=1",
        "replacement_front_product_activation_requires_cross_thread_policy=1",
        "replacement_front_product_activation_requires_remote_abandoned_counters=1",
        "replacement_front_product_activation_requires_rollback_optout_plan=1",
        "replacement_front_rollback_optout_plan_v0=1",
        f"replacement_front_rollback_optout_env={ROLLBACK_OPTOUT_ENV}",
        "replacement_front_rollback_optout_env_value=1",
        "replacement_front_per_process_disable=1",
        "replacement_front_activation_mode=explicit_only",
        "replacement_front_activation_default=off",
        "replacement_front_activation_report_required=1",
        "replacement_front_rollback_report_path_required=1",
        f"replacement_front_product_activation_blockers={PRODUCT_ACTIVATION_BLOCKERS}",
    ]


def product_preflight_fields(preflight: ReplacementFrontPreflight) -> list[str]:
    return [
        "replacement_front_product_preflight_report_v0=1",
        "replacement_front_product_preflight_non_activating=1",
        f"replacement_front_product_preflight_evidence_ready={preflight.evidence_ready}",
        "replacement_front_product_preflight_activation_ready=0",
        f"replacement_front_product_preflight_quality_ok={preflight.quality_ok}",
        "replacement_front_product_preflight_provider_dispatch_bypass_ok="
        f"{preflight.provider_dispatch_bypass_ok}",
        "replacement_front_product_preflight_type_abi_hot_lookup_zero_ok="
        f"{preflight.type_abi_hot_lookup_zero_ok}",
        "replacement_front_product_preflight_cross_thread_policy_ok="
        f"{preflight.cross_thread_policy_ok}",
        "replacement_front_product_preflight_remote_abandoned_counters_ok="
        f"{preflight.remote_abandoned_counters_ok}",
        "replacement_front_product_preflight_rollback_optout_ok="
        f"{preflight.rollback_optout_ok}",
        f"replacement_front_product_preflight_missing={preflight.missing_fields}",
    ]


def product_activation_contract_subject_fields(index: int) -> list[str]:
    prefix = f"subject_{index}_"
    return [
        f"{prefix}replacement_front_product_gate=closed",
        f"{prefix}replacement_front_product_activation_ready=0",
        f"{prefix}replacement_front_product_activation_contract_v0=1",
        f"{prefix}replacement_front_product_activation_requires_quality_ok=1",
        f"{prefix}replacement_front_product_activation_requires_provider_dispatch_bypass=1",
        f"{prefix}replacement_front_product_activation_requires_type_abi_hot_lookup_zero=1",
        f"{prefix}replacement_front_product_activation_requires_cross_thread_policy=1",
        f"{prefix}replacement_front_product_activation_requires_remote_abandoned_counters=1",
        f"{prefix}replacement_front_product_activation_requires_rollback_optout_plan=1",
        f"{prefix}replacement_front_rollback_optout_plan_v0=1",
        f"{prefix}replacement_front_rollback_optout_env={ROLLBACK_OPTOUT_ENV}",
        f"{prefix}replacement_front_rollback_optout_env_value=1",
        f"{prefix}replacement_front_per_process_disable=1",
        f"{prefix}replacement_front_activation_mode=explicit_only",
        f"{prefix}replacement_front_activation_default=off",
        f"{prefix}replacement_front_activation_report_required=1",
        f"{prefix}replacement_front_rollback_report_path_required=1",
        f"{prefix}replacement_front_product_activation_blockers={PRODUCT_ACTIVATION_BLOCKERS}",
    ]


def product_preflight_subject_fields(
    index: int, preflight: ReplacementFrontPreflight
) -> list[str]:
    prefix = f"subject_{index}_"
    return [
        f"{prefix}replacement_front_product_preflight_report_v0=1",
        f"{prefix}replacement_front_product_preflight_non_activating=1",
        f"{prefix}replacement_front_product_preflight_evidence_ready={preflight.evidence_ready}",
        f"{prefix}replacement_front_product_preflight_activation_ready=0",
        f"{prefix}replacement_front_product_preflight_quality_ok={preflight.quality_ok}",
        f"{prefix}replacement_front_product_preflight_provider_dispatch_bypass_ok={preflight.provider_dispatch_bypass_ok}",
        f"{prefix}replacement_front_product_preflight_type_abi_hot_lookup_zero_ok={preflight.type_abi_hot_lookup_zero_ok}",
        f"{prefix}replacement_front_product_preflight_cross_thread_policy_ok={preflight.cross_thread_policy_ok}",
        f"{prefix}replacement_front_product_preflight_remote_abandoned_counters_ok={preflight.remote_abandoned_counters_ok}",
        f"{prefix}replacement_front_product_preflight_rollback_optout_ok={preflight.rollback_optout_ok}",
        f"{prefix}replacement_front_product_preflight_missing={preflight.missing_fields}",
    ]
