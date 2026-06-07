#include <stdint.h>
#include <stddef.h>
#include <stdlib.h>

typedef struct HakoProviderDescriptorV1 {
  uint32_t magic;
  uint16_t abi_major;
  uint16_t abi_minor;
  uint32_t descriptor_size;
  const char* provider_id;
  const char* provider_kind;
  const char* provider_version;
  uint64_t capability_bits;
  uint64_t safety_flags;
  const char* contract_hash;
  const char* function_table_hash;
  uint32_t api_table_size;
  uint32_t reserved;
} HakoProviderDescriptorV1;

typedef struct HakoHostAllocatorV0 {
  uint32_t abi_major;
  uint32_t struct_size;
  void* ctx;
  void* (*malloc_fn)(void* ctx, size_t size);
  void* (*calloc_fn)(void* ctx, size_t count, size_t size);
  void* (*realloc_fn)(void* ctx, void* ptr, size_t size);
  void (*free_fn)(void* ctx, void* ptr);
  size_t (*usable_size_fn)(void* ctx, void* ptr);
} HakoHostAllocatorV0;

typedef struct HakoProviderApiV1 {
  uint32_t magic;
  uint16_t abi_major;
  uint16_t abi_minor;
  uint32_t api_table_size;
  int (*ping)(void);
  void* (*alloc)(size_t size, size_t align);
  void (*free)(void* ptr);
  int (*owns)(void* ptr);
  int (*free_claim)(void* ptr);
  int (*usable_size_claim)(void* ptr, size_t* out_size);
  int (*realloc_claim)(void* ptr, size_t new_size, void** out_ptr);
  int (*init_host_allocator)(const HakoHostAllocatorV0* host);
} HakoProviderApiV1;

static const HakoHostAllocatorV0* HAKO_HOST_ALLOCATOR = 0;

static int hako_ping(void) { return __PING_VALUE__; }
static void* hako_alloc(size_t size, size_t align) {
  (void)align;
  if (!HAKO_HOST_ALLOCATOR || !HAKO_HOST_ALLOCATOR->malloc_fn) {
    return 0;
  }
  return HAKO_HOST_ALLOCATOR->malloc_fn(HAKO_HOST_ALLOCATOR->ctx, size);
}
static void hako_free(void* ptr) {
  if (HAKO_HOST_ALLOCATOR && HAKO_HOST_ALLOCATOR->free_fn) {
    HAKO_HOST_ALLOCATOR->free_fn(HAKO_HOST_ALLOCATOR->ctx, ptr);
  }
}
static int hako_owns(void* ptr) {
  if (ptr == NULL || !HAKO_HOST_ALLOCATOR) {
    return 0;
  }
  return __OWNS_VALUE__;
}
static int hako_free_claim(void* ptr) {
  if (ptr == NULL || !hako_owns(ptr)) {
    return 0;
  }
  hako_free(ptr);
  return 1;
}
static int hako_usable_size_claim(void* ptr, size_t* out_size) {
  if (ptr != NULL && hako_owns(ptr) && HAKO_HOST_ALLOCATOR &&
      HAKO_HOST_ALLOCATOR->usable_size_fn) {
    if (out_size) {
      *out_size = HAKO_HOST_ALLOCATOR->usable_size_fn(HAKO_HOST_ALLOCATOR->ctx, ptr);
    }
    return 1;
  }
  if (out_size) {
    *out_size = 0u;
  }
  return 0;
}
static int hako_realloc_claim(void* ptr, size_t new_size, void** out_ptr) {
  if (ptr == NULL || !hako_owns(ptr)) {
    if (out_ptr) {
      *out_ptr = 0;
    }
    return 0;
  }
  if (new_size == 0) {
    hako_free(ptr);
    if (out_ptr) {
      *out_ptr = 0;
    }
    return 1;
  }
  if (!HAKO_HOST_ALLOCATOR || !HAKO_HOST_ALLOCATOR->realloc_fn) {
    if (out_ptr) {
      *out_ptr = 0;
    }
    return -1;
  }
  void* next = HAKO_HOST_ALLOCATOR->realloc_fn(HAKO_HOST_ALLOCATOR->ctx, ptr, new_size);
  if (!next) {
    if (out_ptr) {
      *out_ptr = 0;
    }
    return -1;
  }
  if (out_ptr) {
    *out_ptr = next;
  }
  return 1;
}
static int hako_init_host_allocator(const HakoHostAllocatorV0* host) {
  if (!host || host->abi_major != 0u ||
      host->struct_size < sizeof(HakoHostAllocatorV0) ||
      !host->malloc_fn || !host->realloc_fn || !host->free_fn) {
    HAKO_HOST_ALLOCATOR = 0;
    return 0;
  }
  HAKO_HOST_ALLOCATOR = host;
  return 1;
}

static const HakoProviderApiV1 API = {
  0x484B5241u, 1, 0, sizeof(HakoProviderApiV1),
  hako_ping, hako_alloc, hako_free, hako_owns, hako_free_claim, hako_usable_size_claim, hako_realloc_claim, hako_init_host_allocator
};

static const HakoProviderDescriptorV1 DESCRIPTOR = {
  0x484B5250u, 1, 0, sizeof(HakoProviderDescriptorV1),
  "__PACKAGE_ID__", "__PROVIDER_KIND__", "__PROVIDER_VERSION__",
  3u, 1u,
  "__CONTRACT_HASH__",
  "__FUNCTION_TABLE_HASH__",
  sizeof(HakoProviderApiV1), 0
};

__attribute__((visibility("default")))
const HakoProviderDescriptorV1* hakorune_provider_descriptor_v1(void) {
  return &DESCRIPTOR;
}

__attribute__((visibility("default")))
const HakoProviderApiV1* hakorune_provider_get_api_v1(void) {
  return &API;
}
