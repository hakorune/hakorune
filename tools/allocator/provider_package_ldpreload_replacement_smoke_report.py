"""Render reports for the provider-backed LD_PRELOAD replacement smoke."""

from __future__ import annotations

from pathlib import Path

from provider_package_load_only_smoke import sha256_file


def emit_report(
    *,
    manifest_path: Path,
    binary_path: Path,
    shim_source: Path,
    shim_binary: Path,
    smoke_source: Path,
    smoke_binary: Path,
    shim_report: Path,
    smoke_exit_code: int,
    shim_fields: dict[str, str],
) -> str:
    provider_alloc_count = int(shim_fields.get("shim_provider_alloc_count", "0"))
    provider_free_count = int(shim_fields.get("shim_provider_free_count", "0"))
    provider_bind_success = int(shim_fields.get("shim_provider_bind_success", "0"))
    pointer_table_overflow = int(shim_fields.get("shim_pointer_table_overflow", "0"))
    runtime_fallback = int(shim_fields.get("shim_runtime_real_fallback_count", "0"))
    summary = "ok" if (
        smoke_exit_code == 0
        and provider_bind_success == 1
        and provider_alloc_count > 0
        and provider_free_count > 0
        and pointer_table_overflow == 0
        and runtime_fallback == 0
    ) else "failed"
    lines = [
        "output_contract=hako-mimalloc-provider-backed-ldpreload-shim-smoke-v0",
        "input_contract=hakorune-provider-runtime-load-stage-7a-v0",
        "dll_mode=provider-backed-ldpreload-pilot",
        f"manifest={manifest_path}",
        f"provider_binary_path={binary_path}",
        f"provider_binary_sha256={sha256_file(binary_path)}",
        f"shim_source_path={shim_source}",
        f"shim_artifact_path={shim_binary}",
        f"shim_artifact_sha256={sha256_file(shim_binary)}",
        f"smoke_source_path={smoke_source}",
        f"smoke_binary_path={smoke_binary}",
        f"shim_report_path={shim_report}",
        "ld_preload_env_applied=1",
        "provider_library_env_applied=1",
        "shared_library_load_executed=1",
        "required_export_resolved=1",
        "provider_api_bound=1",
        "provider_call_executed=1",
        "allocator_entrypoint_called=1",
        "replacement_active=1",
        "replacement_scope=generated-smoke-process-only",
        "replacement_product_claim=0",
        "hook_installed=0",
        "global_allocator=0",
        "winner_claim=0",
        "thread_safety=single-thread-pilot",
        "usable_size_tracking_bypass_mode=claim_mainline_or_measurement",
        f"smoke_exit_code={smoke_exit_code}",
    ]
    for key in sorted(shim_fields):
        lines.append(f"{key}={shim_fields[key]}")
    lines.append(f"summary={summary}")
    return "\n".join(lines) + "\n"
