"""Provider-backed LD_PRELOAD runtime bootstrap raw C source chunk."""

from __future__ import annotations


BOOTSTRAP_C = r"""
static int hako_ensure_provider(void) {
  if (provider_ready) {
    return 1;
  }
  if (provider_load_attempted || loading_provider) {
    return 0;
  }
  provider_load_attempted = 1;
  loading_provider = 1;
  hako_resolve_real();
  const char* path = getenv("HAKORUNE_PROVIDER_LIBRARY");
  if (!path || !path[0]) {
    provider_bind_failure++;
    loading_provider = 0;
    return 0;
  }
  void* handle = dlopen(path, RTLD_NOW | RTLD_LOCAL);
  if (!handle) {
    provider_bind_failure++;
    loading_provider = 0;
    return 0;
  }
  hako_get_api_fn get_api = (hako_get_api_fn)dlsym(handle, "hakorune_provider_get_api_v1");
  if (!get_api) {
    provider_bind_failure++;
    loading_provider = 0;
    return 0;
  }
  provider_usable_size_mode =
      getenv("HAKORUNE_PROVIDER_LDPRELOAD_USE_USABLE_SIZE") != 0;
  provider_assume_owned_mode =
      provider_usable_size_mode &&
      getenv("HAKORUNE_PROVIDER_LDPRELOAD_ASSUME_PROVIDER_OWNED") != 0;
  struct HakoProviderApiV1* api = get_api();
  if (!api || api->magic != HAKO_PROVIDER_API_MAGIC ||
      api->abi_major != HAKO_PROVIDER_API_MAJOR ||
      api->api_table_size < offsetof(struct HakoProviderApiV1, free_claim) ||
      !api->alloc || !api->free || !api->owns) {
    provider_bind_failure++;
    loading_provider = 0;
    return 0;
  }
  provider_api = api;
  provider_alloc_fn = api->alloc;
  provider_free_fn = api->free;
  provider_owns_fn = api->owns;
  if (api->api_table_size >=
          offsetof(struct HakoProviderApiV1, free_claim) + sizeof(api->free_claim) &&
      api->free_claim) {
    provider_free_claim_fn = api->free_claim;
    provider_free_claim_bound = 1;
  }
  if (api->api_table_size >=
          offsetof(struct HakoProviderApiV1, usable_size_claim) + sizeof(api->usable_size_claim) &&
      api->usable_size_claim) {
    provider_usable_size_claim_fn = api->usable_size_claim;
    provider_usable_size_claim_bound = 1;
  }
  if (api->api_table_size >=
          offsetof(struct HakoProviderApiV1, realloc_claim) + sizeof(api->realloc_claim) &&
      api->realloc_claim) {
    provider_realloc_claim_fn = api->realloc_claim;
    provider_realloc_claim_bound = 1;
  }
  if (api->api_table_size >=
          offsetof(struct HakoProviderApiV1, init_host_allocator) + sizeof(api->init_host_allocator) &&
      api->init_host_allocator) {
    provider_init_host_allocator_fn = api->init_host_allocator;
    host_allocator_init_bound = 1;
  }
  if (provider_init_host_allocator_fn) {
    static struct HakoHostAllocatorV0 host_allocator;
    host_allocator.abi_major = 0u;
    host_allocator.struct_size = sizeof(struct HakoHostAllocatorV0);
    host_allocator.ctx = 0;
    host_allocator.malloc_fn = hako_host_malloc;
    host_allocator.calloc_fn = hako_host_calloc;
    host_allocator.realloc_fn = hako_host_realloc;
    host_allocator.free_fn = hako_host_free;
    host_allocator.usable_size_fn = real_malloc_usable_size_fn ? hako_host_usable_size : 0;
    in_provider_call = 1;
    host_allocator_init_result = provider_init_host_allocator_fn(&host_allocator);
    in_provider_call = 0;
    host_allocator_vtable_init_count++;
  }
  provider_claim_mainline_mode =
      provider_free_claim_fn && provider_realloc_claim_fn && provider_usable_size_claim_fn;
  if (provider_claim_mainline_mode) {
    claim_mainline_mode_enabled = 1;
    provider_usable_size_mode = 1;
  }
  if (provider_usable_size_mode) {
    if (provider_usable_size_claim_fn) {
      usable_size_symbol_bound = 1;
      usable_size_mode_enabled = 1;
      if (provider_assume_owned_mode) {
        assume_owned_mode_enabled = 1;
      }
    } else {
      provider_usable_size_fn =
          (hako_provider_usable_size_fn)dlsym(handle, "hakorune_provider_usable_size_v0");
      if (provider_usable_size_fn) {
        usable_size_symbol_bound = 1;
        usable_size_mode_enabled = 1;
        if (provider_assume_owned_mode) {
          assume_owned_mode_enabled = 1;
        }
      } else {
        provider_usable_size_mode = 0;
        provider_assume_owned_mode = 0;
        provider_claim_mainline_mode = 0;
      }
    }
  }
  provider_ready = 1;
  provider_bind_success++;
  atexit(hako_write_report);
  loading_provider = 0;
  return 1;
}
"""
