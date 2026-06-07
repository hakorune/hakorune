pub(super) const SCHEMA_VERSION: &str = "hakorune-provider-package-v1";
pub(super) const ABI_VERSION: &str = "hakorune-provider-abi-v1";
pub(super) const DESCRIPTOR_EXPORT: &str = "hakorune_provider_descriptor_v1";
pub(super) const OUTPUT_CONTRACT: &str = "hakorune-provider-package-hako-derived-build-v0";
pub(super) const BUILD_MODE: &str = "hako-derived-selected-fixture";
pub(super) const PACKAGE_MODE: &str = "hako-derived-provider-package";
pub(super) const OBJECT_LIFECYCLE_MODE: &str = "object-lifecycle-small-alloc-release-v0";
pub(super) const OBJECT_LIFECYCLE_NATIVE_SLOT_MODE: &str = "object-lifecycle-native-slot-bridge-v0";

const HOST_ALLOC_FREE_ROUTE: &str = "host_malloc_free_wrapper";
const HOST_OBJECT_LIFECYCLE_USAGE: &str = "metadata_verification_only";
const HOST_BACKED_ADAPTER_KIND: &str = "host_backed_adapter";
const NATIVE_SLOT_ALLOC_FREE_ROUTE: &str = "native_static_slot_bridge_from_object_lifecycle_shape";
const NATIVE_SLOT_OBJECT_LIFECYCLE_USAGE: &str = "native_shape_codegen";
const PURE_PROVIDER_KIND: &str = "pure_allocator";

#[derive(Clone, Copy)]
pub(super) struct AllocFreeRoute {
    pub route: &'static str,
    pub allocator_kind: &'static str,
    pub realloc_claim_enabled: bool,
    pub usable_size_claim_enabled: bool,
    pub host_allocator_vtable_init: bool,
    pub uses_host_malloc: bool,
    pub uses_hako_object_lifecycle: bool,
    pub object_lifecycle_usage: &'static str,
}

pub(super) fn alloc_free_route(semantic_codegen: &str) -> AllocFreeRoute {
    if semantic_codegen == OBJECT_LIFECYCLE_NATIVE_SLOT_MODE {
        AllocFreeRoute {
            route: NATIVE_SLOT_ALLOC_FREE_ROUTE,
            allocator_kind: PURE_PROVIDER_KIND,
            realloc_claim_enabled: true,
            usable_size_claim_enabled: true,
            host_allocator_vtable_init: false,
            uses_host_malloc: false,
            uses_hako_object_lifecycle: true,
            object_lifecycle_usage: NATIVE_SLOT_OBJECT_LIFECYCLE_USAGE,
        }
    } else {
        AllocFreeRoute {
            route: HOST_ALLOC_FREE_ROUTE,
            allocator_kind: HOST_BACKED_ADAPTER_KIND,
            realloc_claim_enabled: true,
            usable_size_claim_enabled: true,
            host_allocator_vtable_init: true,
            uses_host_malloc: true,
            uses_hako_object_lifecycle: false,
            object_lifecycle_usage: HOST_OBJECT_LIFECYCLE_USAGE,
        }
    }
}

pub(super) fn semantic_codegen_supported(semantic_codegen: &str) -> bool {
    matches!(
        semantic_codegen,
        "none" | "ping-literal-v0" | "alloc-free-owns-literal-v0"
    ) || semantic_codegen == OBJECT_LIFECYCLE_MODE
        || semantic_codegen == OBJECT_LIFECYCLE_NATIVE_SLOT_MODE
}

pub(super) fn semantic_codegen_uses_ping(semantic_codegen: &str) -> bool {
    semantic_codegen == "ping-literal-v0"
        || semantic_codegen == "alloc-free-owns-literal-v0"
        || semantic_codegen == OBJECT_LIFECYCLE_MODE
        || semantic_codegen == OBJECT_LIFECYCLE_NATIVE_SLOT_MODE
}

pub(super) fn semantic_codegen_uses_owns(semantic_codegen: &str) -> bool {
    semantic_codegen == "alloc-free-owns-literal-v0"
        || semantic_codegen == OBJECT_LIFECYCLE_MODE
        || semantic_codegen == OBJECT_LIFECYCLE_NATIVE_SLOT_MODE
}

pub(super) fn semantic_codegen_uses_object_lifecycle(semantic_codegen: &str) -> bool {
    semantic_codegen == OBJECT_LIFECYCLE_MODE
        || semantic_codegen == OBJECT_LIFECYCLE_NATIVE_SLOT_MODE
}
