"""Report helper utilities for the Hakozuna mixed-ws compare script.

This module owns manifest decoding, route classification, and report-only
math helpers so the compare runner can stay focused on execution orchestration.
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


def format_ratio(value: float, base: float) -> str:
    if base <= 0:
        return "0.000000"
    return f"{value / base:.6f}"


def load_manifest_metadata(path: Path | None) -> dict[str, str]:
    if path is None:
        return {}
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid provider manifest JSON: {path}: {exc}") from exc
    if not isinstance(data, dict):
        raise SystemExit(f"provider manifest root must be an object: {path}")

    build = data.get("build")
    activation = data.get("activation")
    if not isinstance(build, dict):
        build = {}
    if not isinstance(activation, dict):
        activation = {}

    def manifest_string(source: dict[str, Any], key: str, default: str = "unknown") -> str:
        value = source.get(key)
        if isinstance(value, bool):
            return "1" if value else "0"
        if value is None:
            return default
        return str(value)

    return {
        "provider_manifest_provider_name": manifest_string(data, "provider_name"),
        "provider_manifest_provider_kind": manifest_string(data, "provider_kind"),
        "provider_manifest_profile": manifest_string(data, "profile"),
        "provider_manifest_build_mode": manifest_string(build, "mode"),
        "provider_manifest_hako_semantic_provider_codegen": manifest_string(
            build, "hako_semantic_provider_codegen", "none"
        ),
        "provider_manifest_hako_provider_object_lifecycle_entrypoint_verified": manifest_string(
            build, "hako_provider_object_lifecycle_entrypoint_verified", "0"
        ),
        "provider_manifest_hako_provider_alloc_free_route": manifest_string(
            build, "hako_provider_alloc_free_route", "unknown"
        ),
        "provider_manifest_provider_allocator_kind": manifest_string(
            build, "provider_allocator_kind", "unknown"
        ),
        "provider_manifest_provider_abi_claim_ops_v1": manifest_string(
            build, "provider_abi_claim_ops_v1", "0"
        ),
        "provider_manifest_provider_free_claim_enabled": manifest_string(
            build, "provider_free_claim_enabled", "0"
        ),
        "provider_manifest_provider_realloc_claim_enabled": manifest_string(
            build, "provider_realloc_claim_enabled", "0"
        ),
        "provider_manifest_provider_usable_size_claim_enabled": manifest_string(
            build, "provider_usable_size_claim_enabled", "0"
        ),
        "provider_manifest_compat_alloc_free_owns_still_supported": manifest_string(
            build, "compat_alloc_free_owns_still_supported", "1"
        ),
        "provider_manifest_compat_owns_free_mainline": manifest_string(
            build, "compat_owns_free_mainline", "1"
        ),
        "provider_manifest_host_allocator_vtable_init": manifest_string(
            build, "host_allocator_vtable_init", "0"
        ),
        "provider_manifest_hako_provider_alloc_free_uses_host_malloc": manifest_string(
            build, "hako_provider_alloc_free_uses_host_malloc", "unknown"
        ),
        "provider_manifest_hako_provider_alloc_free_uses_hako_object_lifecycle": manifest_string(
            build, "hako_provider_alloc_free_uses_hako_object_lifecycle", "unknown"
        ),
        "provider_manifest_hako_provider_object_lifecycle_entrypoint_usage": manifest_string(
            build, "hako_provider_object_lifecycle_entrypoint_usage", "unknown"
        ),
        "provider_manifest_allocator_replacement_allowed": manifest_string(
            activation, "allocator_replacement_allowed", "0"
        ),
        "provider_manifest_hook_allowed": manifest_string(activation, "hook_allowed", "0"),
        "provider_manifest_global_allocator_allowed": manifest_string(
            activation, "global_allocator_allowed", "0"
        ),
    }


def provider_ldpreload_route_metadata(metadata: dict[str, str]) -> dict[str, str]:
    if not metadata:
        return {}

    allocator_kind = metadata.get("provider_manifest_provider_allocator_kind", "unknown")
    alloc_free_route = metadata.get(
        "provider_manifest_hako_provider_alloc_free_route", "unknown"
    )
    uses_host_malloc = metadata.get(
        "provider_manifest_hako_provider_alloc_free_uses_host_malloc", "unknown"
    )
    uses_hako_object_lifecycle = metadata.get(
        "provider_manifest_hako_provider_alloc_free_uses_hako_object_lifecycle",
        "unknown",
    )
    entrypoint_usage = metadata.get(
        "provider_manifest_hako_provider_object_lifecycle_entrypoint_usage",
        "unknown",
    )
    semantic_codegen = metadata.get(
        "provider_manifest_hako_semantic_provider_codegen", "none"
    )
    entrypoint_verified = metadata.get(
        "provider_manifest_hako_provider_object_lifecycle_entrypoint_verified", "0"
    )

    route = "provider_ldpreload_unknown"
    hako_hot_path = "0"
    metadata_only = "0"
    if (
        allocator_kind == "host_backed_adapter"
        or alloc_free_route == "host_malloc_free_wrapper"
        or uses_host_malloc == "1"
    ):
        route = "provider_host_adapter_ldpreload"
    elif uses_hako_object_lifecycle == "1":
        route = "provider_hako_object_lifecycle_ldpreload"
        hako_hot_path = "1"
    elif allocator_kind == "pure_allocator":
        route = "provider_pure_allocator_ldpreload"

    if entrypoint_usage == "metadata_verification_only":
        metadata_only = "1"
        hako_hot_path = "0"

    package_origin = "unknown"
    if semantic_codegen not in ("", "0", "none", "unknown"):
        package_origin = "hako_derived"

    declared_route = route
    if entrypoint_verified == "1" or uses_hako_object_lifecycle == "1":
        declared_route = "provider_hako_object_lifecycle_ldpreload"

    if route == "provider_host_adapter_ldpreload" and hako_hot_path == "1":
        raise SystemExit(
            "host_backed_adapter provider route must not claim .hako hot path"
        )

    return {
        "provider_ldpreload_declared_package_origin": package_origin,
        "provider_ldpreload_declared_route": declared_route,
        "provider_ldpreload_execution_route": route,
        "provider_ldpreload_measurement_route": route,
        "provider_ldpreload_provider_allocator_kind": allocator_kind,
        "provider_ldpreload_alloc_free_route": alloc_free_route,
        "provider_ldpreload_uses_host_malloc": uses_host_malloc,
        "provider_ldpreload_uses_hako_object_lifecycle": uses_hako_object_lifecycle,
        "provider_ldpreload_object_lifecycle_entrypoint_usage": entrypoint_usage,
        "provider_ldpreload_hako_hot_path_claim": hako_hot_path,
        "provider_ldpreload_hako_object_lifecycle_hot_path": hako_hot_path,
        "provider_ldpreload_hako_object_lifecycle_metadata_only": metadata_only,
    }


def provider_front_class(route: str) -> str:
    if route == "provider_host_adapter_ldpreload":
        return "provider_host_adapter"
    if route == "provider_hako_object_lifecycle_ldpreload":
        return "provider_pure_object_lifecycle_bridge"
    if route == "provider_pure_allocator_ldpreload":
        return "provider_pure_allocator"
    return "provider_unknown"


def provider_kind_from_route(route: str) -> str:
    if route == "provider_host_adapter_ldpreload":
        return "host_backed_adapter"
    if route == "provider_hako_object_lifecycle_ldpreload":
        return "object_lifecycle_bridge"
    if route == "provider_pure_allocator_ldpreload":
        return "pure_allocator"
    return "unknown"


def format_per_operation(numerator: int, denominator: int) -> str:
    if denominator <= 0:
        return "0.000000"
    return f"{numerator / denominator:.6f}"


def init_fallback_dominates_provider_ops(counters: dict[str, int], provider_ops: int) -> bool:
    if provider_ops <= 0:
        return False
    init_fallback = counters.get("shim_init_real_fallback_count", 0)
    return init_fallback * 2 >= provider_ops
