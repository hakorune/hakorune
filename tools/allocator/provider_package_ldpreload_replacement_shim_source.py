"""Raw C sources for provider-backed LD_PRELOAD replacement smoke probes."""

from __future__ import annotations

from provider_package_ldpreload_replacement_tracking_source import TRACKING_C
from provider_package_ldpreload_replacement_runtime_source import RUNTIME_C

SHIM_C = r"""
#define _GNU_SOURCE
#include <dlfcn.h>
#include <fcntl.h>
#include <stdint.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define HAKO_PROVIDER_API_MAGIC 0x484B5241u
#define HAKO_PROVIDER_API_MAJOR 1u
#define HAKO_POINTER_TABLE_CAP 65536u

typedef int (*hako_ping_fn)(void);
typedef void* (*hako_provider_alloc_fn)(size_t, size_t);
typedef void (*hako_provider_free_fn)(void*);
typedef int (*hako_provider_owns_fn)(void*);
typedef int (*hako_provider_free_claim_fn)(void*);
typedef int (*hako_provider_usable_size_claim_fn)(void*, size_t*);
typedef int (*hako_provider_realloc_claim_fn)(void*, size_t, void**);
struct HakoHostAllocatorV0;
typedef int (*hako_provider_init_host_allocator_fn)(const struct HakoHostAllocatorV0*);
typedef size_t (*hako_provider_usable_size_fn)(void*);
typedef void* (*hako_malloc_fn)(size_t);
typedef void* (*hako_calloc_fn)(size_t, size_t);
typedef void* (*hako_realloc_fn)(void*, size_t);
typedef void (*hako_free_fn)(void*);
typedef size_t (*hako_malloc_usable_size_fn)(void*);
typedef void* (*hako_host_malloc_fn)(void*, size_t);
typedef void* (*hako_host_calloc_fn)(void*, size_t, size_t);
typedef void* (*hako_host_realloc_fn)(void*, void*, size_t);
typedef void (*hako_host_free_fn)(void*, void*);
typedef size_t (*hako_host_usable_size_fn)(void*, void*);

struct HakoHostAllocatorV0 {
  uint32_t abi_major;
  uint32_t struct_size;
  void* ctx;
  hako_host_malloc_fn malloc_fn;
  hako_host_calloc_fn calloc_fn;
  hako_host_realloc_fn realloc_fn;
  hako_host_free_fn free_fn;
  hako_host_usable_size_fn usable_size_fn;
};

struct HakoProviderApiV1 {
  uint32_t magic;
  uint16_t abi_major;
  uint16_t abi_minor;
  uint32_t api_table_size;
  hako_ping_fn ping;
  hako_provider_alloc_fn alloc;
  hako_provider_free_fn free;
  hako_provider_owns_fn owns;
  hako_provider_free_claim_fn free_claim;
  hako_provider_usable_size_claim_fn usable_size_claim;
  hako_provider_realloc_claim_fn realloc_claim;
  hako_provider_init_host_allocator_fn init_host_allocator;
};

typedef struct HakoProviderApiV1* (*hako_get_api_fn)(void);

struct HakoTrackedPtr {
  void* ptr;
  size_t size;
};

static hako_malloc_fn real_malloc_fn = 0;
static hako_calloc_fn real_calloc_fn = 0;
static hako_realloc_fn real_realloc_fn = 0;
static hako_free_fn real_free_fn = 0;
static struct HakoProviderApiV1* provider_api = 0;
static hako_provider_alloc_fn provider_alloc_fn = 0;
static hako_provider_free_fn provider_free_fn = 0;
static hako_provider_owns_fn provider_owns_fn = 0;
static hako_provider_free_claim_fn provider_free_claim_fn = 0;
static hako_provider_usable_size_claim_fn provider_usable_size_claim_fn = 0;
static hako_provider_realloc_claim_fn provider_realloc_claim_fn = 0;
static hako_provider_init_host_allocator_fn provider_init_host_allocator_fn = 0;
static hako_provider_usable_size_fn provider_usable_size_fn = 0;
static hako_malloc_usable_size_fn real_malloc_usable_size_fn = 0;
static int resolving_real = 0;
static int loading_provider = 0;
static int provider_load_attempted = 0;
static int provider_ready = 0;
static int provider_usable_size_mode = 0;
static int provider_assume_owned_mode = 0;
static int provider_claim_mainline_mode = 0;
static __thread int in_provider_call = 0;
static struct HakoTrackedPtr tracked[HAKO_POINTER_TABLE_CAP];

static void* hako_host_malloc(void* ctx, size_t size) {
  (void)ctx;
  return real_malloc_fn ? real_malloc_fn(size) : 0;
}

static void* hako_host_calloc(void* ctx, size_t count, size_t size) {
  (void)ctx;
  return real_calloc_fn ? real_calloc_fn(count, size) : 0;
}

static void* hako_host_realloc(void* ctx, void* ptr, size_t size) {
  (void)ctx;
  return real_realloc_fn ? real_realloc_fn(ptr, size) : 0;
}

static void hako_host_free(void* ctx, void* ptr) {
  (void)ctx;
  if (real_free_fn) {
    real_free_fn(ptr);
  }
}

static size_t hako_host_usable_size(void* ctx, void* ptr) {
  (void)ctx;
  return real_malloc_usable_size_fn ? real_malloc_usable_size_fn(ptr) : 0u;
}

static unsigned long long provider_alloc_count = 0;
static unsigned long long provider_calloc_count = 0;
static unsigned long long provider_realloc_count = 0;
static unsigned long long provider_free_count = 0;
static unsigned long long provider_free_claim_count = 0;
static unsigned long long provider_free_not_owned_count = 0;
static unsigned long long provider_free_claim_bound = 0;
static unsigned long long provider_usable_size_claim_count = 0;
static unsigned long long provider_usable_size_not_owned_count = 0;
static unsigned long long provider_usable_size_claim_bound = 0;
static unsigned long long provider_realloc_claim_count = 0;
static unsigned long long provider_realloc_not_owned_count = 0;
static unsigned long long provider_realloc_failed_count = 0;
static unsigned long long provider_realloc_claim_bound = 0;
static unsigned long long host_allocator_init_bound = 0;
static unsigned long long host_allocator_init_result = 0;
static unsigned long long host_allocator_vtable_init_count = 0;
static unsigned long long host_allocator_usable_size_bound = 0;
static unsigned long long runtime_real_fallback_count = 0;
static unsigned long long init_real_fallback_count = 0;
static unsigned long long host_passthrough_count = 0;
static unsigned long long provider_bind_success = 0;
static unsigned long long provider_bind_failure = 0;
static unsigned long long pointer_table_overflow = 0;
static unsigned long long init_fallback_loading_provider_count = 0;
static unsigned long long init_fallback_resolving_real_count = 0;
static unsigned long long init_fallback_in_provider_call_count = 0;
static unsigned long long malloc_init_fallback_count = 0;
static unsigned long long calloc_init_fallback_count = 0;
static unsigned long long realloc_init_fallback_count = 0;
static unsigned long long free_init_fallback_count = 0;
static unsigned long long track_probe_total = 0;
static unsigned long long track_probe_max = 0;
static unsigned long long find_probe_total = 0;
static unsigned long long find_probe_max = 0;
static unsigned long long tombstone_hit_count = 0;
static unsigned long long tracked_hit_count = 0;
static unsigned long long tracked_miss_count = 0;
static unsigned long long calloc_zero_bytes = 0;
static unsigned long long realloc_copy_bytes = 0;
static unsigned long long realloc_tracked_count = 0;
static unsigned long long realloc_host_passthrough_count = 0;
static unsigned long long realloc_null_count = 0;
static unsigned long long realloc_free_count = 0;
static unsigned long long usable_size_mode_enabled = 0;
static unsigned long long usable_size_symbol_bound = 0;
static unsigned long long usable_size_lookup_count = 0;
static unsigned long long usable_size_lookup_failure_count = 0;
static unsigned long long tracking_bypassed_count = 0;
static unsigned long long claim_mainline_mode_enabled = 0;
static unsigned long long assume_owned_mode_enabled = 0;
static unsigned long long assume_owned_free_count = 0;
static unsigned long long assume_owned_realloc_count = 0;
""" + TRACKING_C + RUNTIME_C
