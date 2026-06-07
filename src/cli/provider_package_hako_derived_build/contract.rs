use super::config::{AllocFreeRoute, ABI_VERSION};
use super::support::sha256_bytes;
use serde_json::json;

pub(super) struct HakoDerivedProviderContractInput<'a> {
    pub provider_kind: &'a str,
    pub hako_source_hash: &'a str,
    pub hako_mir_json_hash: &'a str,
    pub semantic_codegen: &'a str,
    pub semantic_ping_value: Option<i64>,
    pub semantic_owns_value: Option<i64>,
    pub semantic_object_lifecycle_verified: bool,
    pub alloc_route: AllocFreeRoute,
}

pub(super) fn hako_derived_function_table_hash(
    input: HakoDerivedProviderContractInput<'_>,
) -> Result<String, String> {
    let contract = json!({
        "abi_version": ABI_VERSION,
        "api_table_schema_version": "hakorune-provider-api-v1",
        "entrypoints": ["ping", "alloc", "free", "owns", "free_claim", "usable_size_claim", "realloc_claim", "init_host_allocator"],
        "provider_kind": input.provider_kind,
        "provider_allocator_kind": input.alloc_route.allocator_kind,
        "provider_abi_claim_ops_v1": true,
        "provider_free_claim_enabled": true,
        "provider_realloc_claim_enabled": input.alloc_route.realloc_claim_enabled,
        "provider_usable_size_claim_enabled": input.alloc_route.usable_size_claim_enabled,
        "compat_alloc_free_owns_still_supported": true,
        "compat_owns_free_mainline": false,
        "host_allocator_vtable_init": input.alloc_route.host_allocator_vtable_init,
        "provider_direct_libc_symbol_dependency": false,
        "ld_preload_reentry_for_host_alloc": false,
        "hako_source_hash": input.hako_source_hash,
        "hako_mir_json_hash": input.hako_mir_json_hash,
        "hako_semantic_provider_codegen": input.semantic_codegen,
        "hako_provider_ping_value": input.semantic_ping_value,
        "hako_provider_owns_value": input.semantic_owns_value,
        "hako_provider_object_lifecycle_entrypoint_verified": input.semantic_object_lifecycle_verified,
        "hako_provider_alloc_free_route": input.alloc_route.route,
        "hako_provider_alloc_free_uses_host_malloc": input.alloc_route.uses_host_malloc,
        "hako_provider_alloc_free_uses_hako_object_lifecycle": input.alloc_route.uses_hako_object_lifecycle,
        "hako_provider_object_lifecycle_entrypoint_usage": input.alloc_route.object_lifecycle_usage,
    });
    Ok(sha256_bytes(
        serde_json::to_string(&contract)
            .map_err(|error| {
                format!(
                    "[provider-package-hako-build/function-table-hash-serialize-failed] {error}"
                )
            })?
            .as_bytes(),
    ))
}
