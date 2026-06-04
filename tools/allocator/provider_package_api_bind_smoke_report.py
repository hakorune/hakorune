"""Render provider API bind smoke reports."""

from __future__ import annotations

from pathlib import Path

GET_API_EXPORT_NAME = "hakorune_provider_get_api_v1"


def emit(
    fields: dict[str, str],
    descriptor: dict[str, str],
    api: dict[str, str],
    manifest_path: Path,
    binary_path: Path,
) -> str:
    lines = [
        "output_contract=hakorune-provider-package-api-bind-smoke-v0",
        "dll_mode=provider-api-bind",
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
        f"descriptor_contract_hash={descriptor['contract_hash']}",
        f"provider_api_export={GET_API_EXPORT_NAME}",
        f"api_abi_major={api['api_abi_major']}",
        f"api_abi_minor={api['api_abi_minor']}",
        f"api_table_size={api['api_table_size']}",
        f"provider_allocator_kind={fields['provider_allocator_kind']}",
        f"provider_free_claim_bound={api['provider_free_claim_bound']}",
        f"provider_usable_size_claim_bound={api['provider_usable_size_claim_bound']}",
        f"provider_realloc_claim_bound={api['provider_realloc_claim_bound']}",
        f"host_allocator_init_bound={api['host_allocator_init_bound']}",
        "provider_abi_claim_ops_v1=1",
        "provider_free_claim_enabled=1",
        f"provider_realloc_claim_enabled={fields['provider_realloc_claim_enabled']}",
        f"provider_usable_size_claim_enabled={fields['provider_usable_size_claim_enabled']}",
        "compat_alloc_free_owns_still_supported=1",
        "compat_owns_free_mainline=0",
        f"host_allocator_vtable_init={fields['host_allocator_vtable_init']}",
        "provider_direct_libc_symbol_dependency=0",
        "ld_preload_reentry_for_host_alloc=0",
        "manifest_ready=1",
        "descriptor_ready=1",
        "binary_hash_ready=1",
        "shared_library_load_executed=1",
        "required_export_resolved=1",
        "descriptor_read_executed=1",
        "provider_api_bound=1",
        "provider_call_executed=0",
        "allocator_entrypoint_called=0",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "winner_claim=0",
        "summary=ok",
    ]
    return "\n".join(lines) + "\n"
