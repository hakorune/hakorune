"""Provider-backed LD_PRELOAD runtime raw C source chunk."""

from __future__ import annotations


RUNTIME_C = r"""
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

static void* hako_provider_alloc(size_t size, size_t align) {
  if (!provider_ready && !hako_ensure_provider()) {
    runtime_real_fallback_count++;
    hako_resolve_real();
    return real_malloc_fn ? real_malloc_fn(size) : 0;
  }
  in_provider_call = 1;
  void* ptr = provider_alloc_fn(size, align);
  in_provider_call = 0;
  if (ptr) {
    if (provider_usable_size_mode) {
      tracking_bypassed_count++;
    } else {
      hako_track_ptr(ptr, size);
    }
    provider_alloc_count++;
  }
  return ptr;
}

static void hako_provider_free(void* ptr) {
  if (!ptr) {
    return;
  }
  if (!provider_ready && !hako_ensure_provider()) {
    runtime_real_fallback_count++;
    hako_resolve_real();
    if (real_free_fn) {
      real_free_fn(ptr);
    }
    return;
  }
  in_provider_call = 1;
  provider_free_fn(ptr);
  in_provider_call = 0;
  provider_free_count++;
}

__attribute__((visibility("default")))
void* malloc(size_t size) {
  if (loading_provider || resolving_real || in_provider_call) {
    hako_count_init_fallback();
    malloc_init_fallback_count++;
    hako_resolve_real();
    return real_malloc_fn ? real_malloc_fn(size) : 0;
  }
  return hako_provider_alloc(size, 16);
}

__attribute__((visibility("default")))
void* calloc(size_t count, size_t size) {
  if (loading_provider || resolving_real || in_provider_call) {
    hako_count_init_fallback();
    calloc_init_fallback_count++;
    hako_resolve_real();
    return real_calloc_fn ? real_calloc_fn(count, size) : 0;
  }
  if (size != 0 && count > ((size_t)-1) / size) {
    return 0;
  }
  size_t bytes = count * size;
  void* ptr = hako_provider_alloc(bytes, 16);
  if (ptr) {
    memset(ptr, 0, bytes);
    calloc_zero_bytes += bytes;
    provider_calloc_count++;
  }
  return ptr;
}

__attribute__((visibility("default")))
void* realloc(void* ptr, size_t size) {
  if (!ptr) {
    realloc_null_count++;
    return malloc(size);
  }
  if (size == 0) {
    realloc_free_count++;
    free(ptr);
    return 0;
  }
  if (loading_provider || resolving_real || in_provider_call) {
    hako_count_init_fallback();
    realloc_init_fallback_count++;
    hako_resolve_real();
    return real_realloc_fn ? real_realloc_fn(ptr, size) : 0;
  }
  if ((provider_ready || hako_ensure_provider()) && provider_realloc_claim_fn) {
    void* next = 0;
    in_provider_call = 1;
    int handled = provider_realloc_claim_fn(ptr, size, &next);
    in_provider_call = 0;
    provider_realloc_claim_count++;
    if (handled == 1) {
      if (!provider_usable_size_mode) {
        int claim_index = hako_find_tracked(ptr);
        if (claim_index >= 0) {
          hako_untrack_index(claim_index);
        }
        if (next) {
          hako_track_ptr(next, size);
        }
      }
      provider_realloc_count++;
      return next;
    }
    if (handled < 0) {
      provider_realloc_failed_count++;
      return 0;
    }
    provider_realloc_not_owned_count++;
  }
  int index = provider_usable_size_mode ? -1 : hako_find_tracked(ptr);
  if (index < 0) {
    if (!provider_usable_size_mode ||
        (!provider_usable_size_claim_fn && (!provider_owns_fn || !provider_usable_size_fn))) {
      realloc_host_passthrough_count++;
      host_passthrough_count++;
      hako_resolve_real();
      return real_realloc_fn ? real_realloc_fn(ptr, size) : 0;
    }
    int owned = 0;
    size_t old_size = 0u;
    in_provider_call = 1;
    if (provider_usable_size_claim_fn) {
      owned = provider_usable_size_claim_fn(ptr, &old_size);
      provider_usable_size_claim_count++;
      if (owned != 1) {
        provider_usable_size_not_owned_count++;
      }
    } else {
      owned = provider_assume_owned_mode ? 1 : provider_owns_fn(ptr);
      old_size = owned == 1 ? provider_usable_size_fn(ptr) : 0u;
    }
    in_provider_call = 0;
    usable_size_lookup_count++;
    if (provider_assume_owned_mode) {
      assume_owned_realloc_count++;
    }
    if (owned != 1 || old_size == 0u) {
      usable_size_lookup_failure_count++;
      realloc_host_passthrough_count++;
      host_passthrough_count++;
      hako_resolve_real();
      return real_realloc_fn ? real_realloc_fn(ptr, size) : 0;
    }
    realloc_tracked_count++;
    void* next = hako_provider_alloc(size, 16);
    if (!next) {
      return 0;
    }
    size_t copy_size = old_size < size ? old_size : size;
    memcpy(next, ptr, copy_size);
    realloc_copy_bytes += copy_size;
    hako_provider_free(ptr);
    provider_realloc_count++;
    return next;
  }
  realloc_tracked_count++;
  size_t old_size = tracked[index].size;
  void* next = hako_provider_alloc(size, 16);
  if (!next) {
    return 0;
  }
  size_t copy_size = old_size < size ? old_size : size;
  memcpy(next, ptr, copy_size);
  realloc_copy_bytes += copy_size;
  hako_untrack_index(index);
  hako_provider_free(ptr);
  provider_realloc_count++;
  return next;
}

__attribute__((visibility("default")))
void free(void* ptr) {
  if (!ptr) {
    return;
  }
  if (loading_provider || resolving_real || in_provider_call) {
    hako_count_init_fallback();
    free_init_fallback_count++;
    hako_resolve_real();
    if (real_free_fn) {
      real_free_fn(ptr);
    }
    return;
  }
  int index = provider_usable_size_mode ? -1 : hako_find_tracked(ptr);
  if (index >= 0) {
    hako_untrack_index(index);
  }
  if ((provider_ready || hako_ensure_provider()) && provider_free_claim_fn) {
    in_provider_call = 1;
    int handled = provider_free_claim_fn(ptr);
    in_provider_call = 0;
    provider_free_claim_count++;
    if (handled == 1) {
      provider_free_count++;
      return;
    }
    provider_free_not_owned_count++;
    if (index >= 0) {
      hako_provider_free(ptr);
      return;
    }
  } else if (index >= 0) {
    hako_provider_free(ptr);
    return;
  }
  if (provider_assume_owned_mode && (provider_ready || hako_ensure_provider())) {
    assume_owned_free_count++;
    hako_provider_free(ptr);
    return;
  }
  if ((provider_ready || hako_ensure_provider()) && provider_owns_fn) {
    in_provider_call = 1;
    int owned = provider_owns_fn(ptr);
    in_provider_call = 0;
    if (owned == 1) {
      hako_provider_free(ptr);
      return;
    }
  }
  host_passthrough_count++;
  hako_resolve_real();
  if (real_free_fn) {
    real_free_fn(ptr);
  }
}
"""
