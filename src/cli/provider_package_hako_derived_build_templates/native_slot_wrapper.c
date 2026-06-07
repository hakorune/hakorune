#include <stdint.h>
#include <stddef.h>

#define HAKO_PROVIDER_SLOT_SIZE 2048u
#define HAKO_PROVIDER_SLOT_COUNT 8192u

typedef union HakoProviderSlot {
  max_align_t align;
  unsigned char bytes[HAKO_PROVIDER_SLOT_SIZE];
} HakoProviderSlot;

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

static HakoProviderSlot HAKO_PROVIDER_SLOTS[HAKO_PROVIDER_SLOT_COUNT];
static unsigned char HAKO_PROVIDER_USED[HAKO_PROVIDER_SLOT_COUNT];
static size_t HAKO_PROVIDER_REQUESTED_SIZE[HAKO_PROVIDER_SLOT_COUNT];
static uint32_t HAKO_PROVIDER_FREE_STACK[HAKO_PROVIDER_SLOT_COUNT];
static uint32_t HAKO_PROVIDER_FREE_TOP = 0u;
static unsigned char HAKO_PROVIDER_INIT = 0u;

static int hako_ping(void) { return __PING_VALUE__; }

static void hako_init_slots(void) {
  if (HAKO_PROVIDER_INIT) {
    return;
  }
  for (uint32_t i = 0; i < HAKO_PROVIDER_SLOT_COUNT; i++) {
    HAKO_PROVIDER_FREE_STACK[i] = HAKO_PROVIDER_SLOT_COUNT - i - 1u;
  }
  HAKO_PROVIDER_FREE_TOP = HAKO_PROVIDER_SLOT_COUNT;
  HAKO_PROVIDER_INIT = 1u;
}

static int hako_slot_index(void* ptr) {
  if (ptr == 0) {
    return -1;
  }
  uintptr_t value = (uintptr_t)ptr;
  uintptr_t base = (uintptr_t)HAKO_PROVIDER_SLOTS[0].bytes;
  uintptr_t end = (uintptr_t)(HAKO_PROVIDER_SLOTS + HAKO_PROVIDER_SLOT_COUNT);
  if (value < base || value >= end) {
    return -1;
  }
  uintptr_t delta = value - base;
  uintptr_t stride = sizeof(HakoProviderSlot);
  if ((delta % stride) != 0) {
    return -1;
  }
  uintptr_t index = delta / stride;
  if (index >= HAKO_PROVIDER_SLOT_COUNT) {
    return -1;
  }
  return (int)index;
}

static void* hako_alloc(size_t size, size_t align) {
  if (size == 0 || size > HAKO_PROVIDER_SLOT_SIZE) {
    return 0;
  }
  if (align == 0) {
    align = sizeof(void*);
  }
  if (align > 16u) {
    return 0;
  }
  hako_init_slots();
  if (HAKO_PROVIDER_FREE_TOP == 0u) {
    return 0;
  }
  uint32_t index = HAKO_PROVIDER_FREE_STACK[--HAKO_PROVIDER_FREE_TOP];
  HAKO_PROVIDER_USED[index] = 1u;
  HAKO_PROVIDER_REQUESTED_SIZE[index] = size;
  return HAKO_PROVIDER_SLOTS[index].bytes;
}

static void hako_free(void* ptr) {
  int index = hako_slot_index(ptr);
  if (index >= 0 && HAKO_PROVIDER_USED[(uint32_t)index]) {
    HAKO_PROVIDER_USED[(uint32_t)index] = 0u;
    HAKO_PROVIDER_REQUESTED_SIZE[(uint32_t)index] = 0u;
    if (HAKO_PROVIDER_FREE_TOP < HAKO_PROVIDER_SLOT_COUNT) {
      HAKO_PROVIDER_FREE_STACK[HAKO_PROVIDER_FREE_TOP++] = (uint32_t)index;
    }
  }
}

static int hako_owns(void* ptr) {
  int index = hako_slot_index(ptr);
  if (index < 0) {
    return 0;
  }
  return HAKO_PROVIDER_USED[(uint32_t)index] ? __OWNS_VALUE__ : 0;
}
static int hako_free_claim(void* ptr) {
  if (!hako_owns(ptr)) {
    return 0;
  }
  hako_free(ptr);
  return 1;
}
static int hako_usable_size_claim(void* ptr, size_t* out_size) {
  int index = hako_slot_index(ptr);
  if (index < 0 || !HAKO_PROVIDER_USED[(uint32_t)index]) {
    if (out_size) {
      *out_size = 0u;
    }
    return 0;
  }
  if (out_size) {
    *out_size = HAKO_PROVIDER_REQUESTED_SIZE[(uint32_t)index];
  }
  return 1;
}
static int hako_realloc_claim(void* ptr, size_t new_size, void** out_ptr) {
  int index = hako_slot_index(ptr);
  if (index < 0 || !HAKO_PROVIDER_USED[(uint32_t)index]) {
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
  if (new_size > HAKO_PROVIDER_SLOT_SIZE) {
    if (out_ptr) {
      *out_ptr = 0;
    }
    return -1;
  }
  HAKO_PROVIDER_REQUESTED_SIZE[(uint32_t)index] = new_size;
  if (out_ptr) {
    *out_ptr = ptr;
  }
  return 1;
}
static int hako_init_host_allocator(const HakoHostAllocatorV0* host) {
  (void)host;
  return 0;
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

__attribute__((visibility("default")))
size_t hakorune_provider_usable_size_v0(void* ptr) {
  int index = hako_slot_index(ptr);
  if (index < 0 || !HAKO_PROVIDER_USED[(uint32_t)index]) {
    return 0u;
  }
  return HAKO_PROVIDER_REQUESTED_SIZE[(uint32_t)index];
}
