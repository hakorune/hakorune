"""Multi-bin replacement-front benchmark template generation."""

from __future__ import annotations

from replacement_front_bins_report_source import REPORT_C

from replacement_front_support import hako_size_class_bin_size


def generate_replacement_front_bins_shim_c(
    required_bins: list[int],
    *,
    locked: bool = False,
    page_shaped: bool = False,
    hotcore_page_model: bool = False,
    size_class_table: bool = False,
    eager_init: bool = False,
    product_pages_nonlinear_lookup: bool = False,
) -> str:
    """Generate a benchmark-only multi-bin replacement front.

    This is intentionally narrower than the fixed-slot front: no remote-free
    bridge and no product allocator claim. The optional locked route is a
    benchmark-only multithread evidence slice, not allocator activation.
    """

    bin_defs: list[str] = []
    init_cases: list[str] = []
    page_index_register_cases: list[str] = []
    size_cases: list[str] = []
    alloc_cases: list[str] = []
    find_cases: list[str] = []
    helper_defs: list[str] = []
    release_cases: list[str] = []
    bin_sizes: list[tuple[int, int]] = []
    for bin_index in required_bins:
        bin_size = hako_size_class_bin_size(bin_index)
        if bin_size <= 0:
            continue
        bin_sizes.append((bin_index, bin_size))
        tag = f"bin_{bin_index}"
        type_tag = tag.title().replace("_", "")
        slot_expr = f"{tag}_slots"
        used_expr = f"{tag}_used"
        requested_expr = f"{tag}_requested_size"
        free_stack_expr = f"{tag}_free_stack"
        free_top_expr = f"{tag}_free_top"
        bin_defs.extend(
            [
                f"#define HAKO_{tag.upper()}_SIZE {bin_size}u",
                f"typedef union HakoReplacement{type_tag}Slot {{",
                "  max_align_t align;",
                f"  unsigned char bytes[HAKO_{tag.upper()}_SIZE];",
                f"}} HakoReplacement{type_tag}Slot;",
            ]
        )
        if page_shaped:
            bin_defs.extend(
                [
                    f"typedef struct HakoReplacement{type_tag}Page {{",
                    f"  HakoReplacement{type_tag}Slot slots[HAKO_REPLACEMENT_BIN_SLOT_COUNT];",
                    "  unsigned char used[HAKO_REPLACEMENT_BIN_SLOT_COUNT];",
                    "  size_t requested_size[HAKO_REPLACEMENT_BIN_SLOT_COUNT];",
                    "  uint32_t free_stack[HAKO_REPLACEMENT_BIN_SLOT_COUNT];",
                    "  uint32_t free_top;",
                    f"}} HakoReplacement{type_tag}Page;",
                    f"static HakoReplacement{type_tag}Page {tag}_page;",
                ]
            )
            slot_expr = f"{tag}_page.slots"
            used_expr = f"{tag}_page.used"
            requested_expr = f"{tag}_page.requested_size"
            free_stack_expr = f"{tag}_page.free_stack"
            free_top_expr = f"{tag}_page.free_top"
        else:
            bin_defs.extend(
                [
                    f"static HakoReplacement{type_tag}Slot {slot_expr}[HAKO_REPLACEMENT_BIN_SLOT_COUNT];",
                    f"static unsigned char {used_expr}[HAKO_REPLACEMENT_BIN_SLOT_COUNT];",
                    f"static size_t {requested_expr}[HAKO_REPLACEMENT_BIN_SLOT_COUNT];",
                    f"static uint32_t {free_stack_expr}[HAKO_REPLACEMENT_BIN_SLOT_COUNT];",
                    f"static uint32_t {free_top_expr} = 0u;",
                ]
            )
        if hotcore_page_model:
            helper_defs.append(
                f"""
static inline void* hako_page_acquire_fresh_small_{tag}(size_t size) {{
  if ({free_top_expr} == 0u) return 0;
  uint32_t index = {free_stack_expr}[--{free_top_expr}];
  {used_expr}[index] = 1u;
  {requested_expr}[index] = size;
  add_counter(&direct_core_call_count, 1);
  return {slot_expr}[index].bytes;
}}

static inline int hako_page_release_local_known_live_{tag}(uint32_t index) {{
  if (index >= HAKO_REPLACEMENT_BIN_SLOT_COUNT || {used_expr}[index] != 1u) return 0;
  {used_expr}[index] = 0u;
  {requested_expr}[index] = 0u;
  if ({free_top_expr} < HAKO_REPLACEMENT_BIN_SLOT_COUNT) {{
    {free_stack_expr}[{free_top_expr}++] = index;
  }}
  add_counter(&direct_core_call_count, 1);
  return 1;
}}
"""
            )
        init_cases.append(
            f"""
  for (uint32_t i = 0; i < HAKO_REPLACEMENT_BIN_SLOT_COUNT; i++) {{
    {free_stack_expr}[i] = HAKO_REPLACEMENT_BIN_SLOT_COUNT - i - 1u;
    {used_expr}[i] = 0u;
    {requested_expr}[i] = 0u;
  }}
  {free_top_expr} = HAKO_REPLACEMENT_BIN_SLOT_COUNT;
"""
        )
        page_index_register_cases.append(
            f"""
  page_index_register_range(
      (uintptr_t){slot_expr}[0].bytes,
      (uintptr_t)({slot_expr} + HAKO_REPLACEMENT_BIN_SLOT_COUNT),
      sizeof({slot_expr}[0]),
      HAKO_{tag.upper()}_SIZE,
      {bin_index},
      {used_expr},
      {requested_expr},
      {free_stack_expr},
      &{free_top_expr});
"""
        )
        size_cases.append(f"  if (size <= HAKO_{tag.upper()}_SIZE) return {bin_index};")
        if hotcore_page_model:
            alloc_cases.append(
                f"""
    case {bin_index}:
      return hako_page_acquire_fresh_small_{tag}(size);
"""
            )
            release_cases.append(
                f"""
    case {bin_index}:
      return hako_page_release_local_known_live_{tag}(index);
"""
            )
        else:
            alloc_cases.append(
                f"""
    case {bin_index}:
      if ({free_top_expr} == 0u) return 0;
      index = {free_stack_expr}[--{free_top_expr}];
      {used_expr}[index] = 1u;
      {requested_expr}[index] = size;
      add_counter(&direct_core_call_count, 1);
      return {slot_expr}[index].bytes;
"""
            )
        find_cases.append(
            f"""
  base = (uintptr_t){slot_expr}[0].bytes;
  end = (uintptr_t)({slot_expr} + HAKO_REPLACEMENT_BIN_SLOT_COUNT);
  if (value >= base && value < end) {{
    delta = value - base;
    stride = sizeof({slot_expr}[0]);
    if ((delta % stride) != 0) return 0;
    index = (uint32_t)(delta / stride);
    if (index >= HAKO_REPLACEMENT_BIN_SLOT_COUNT) return 0;
    *bin_out = {bin_index};
    *index_out = index;
    *slot_size_out = HAKO_{tag.upper()}_SIZE;
    *used_out = {used_expr};
    *requested_out = {requested_expr};
    *free_stack_out = {free_stack_expr};
    *free_top_out = &{free_top_expr};
    return 1;
  }}
"""
        )

    size_to_bin_source = f"""
static int size_to_bin(size_t size) {{
  if (size == 0) return -1;
{chr(10).join(size_cases)}
  return -1;
}}
"""
    if size_class_table and bin_sizes:
        sorted_bin_sizes = sorted(bin_sizes, key=lambda item: item[1])
        max_bin_size = sorted_bin_sizes[-1][1]
        bucket_unit = 8
        bucket_count = (max_bin_size + bucket_unit - 1) // bucket_unit
        table_values = [-1]
        for bucket in range(1, bucket_count + 1):
            request_ceiling = bucket * bucket_unit
            selected_bin = -1
            for bin_index, bin_size in sorted_bin_sizes:
                if request_ceiling <= bin_size:
                    selected_bin = bin_index
                    break
            table_values.append(selected_bin)
        table_rows = []
        for start in range(0, len(table_values), 16):
            row = ", ".join(str(value) for value in table_values[start : start + 16])
            table_rows.append(f"  {row},")
        size_to_bin_source = f"""
#define HAKO_SIZE_TO_BIN_TABLE_UNIT 8u
#define HAKO_SIZE_TO_BIN_TABLE_MAX {max_bin_size}u
static const signed char hako_size_to_bin_table[{len(table_values)}] = {{
{chr(10).join(table_rows)}
}};

static int size_to_bin(size_t size) {{
  if (size == 0 || size > HAKO_SIZE_TO_BIN_TABLE_MAX) return -1;
  size_t bucket = (size + HAKO_SIZE_TO_BIN_TABLE_UNIT - 1u) / HAKO_SIZE_TO_BIN_TABLE_UNIT;
  return (int)hako_size_to_bin_table[bucket];
}}
"""

    release_from_bin_source = ""
    if hotcore_page_model:
        release_from_bin_source = f"""
static int release_from_bin(int bin, uint32_t index) {{
  switch (bin) {{
{chr(10).join(release_cases)}
    default:
      return 0;
  }}
}}
"""
    if hotcore_page_model:
        free_owned_body = """    (void)slot_size;
    (void)used;
    (void)requested;
    (void)free_stack;
    (void)free_top;
    (void)release_from_bin(bin, index);
"""
    else:
        free_owned_body = """    (void)bin;
    (void)slot_size;
    if (used[index] == 1u) {
      used[index] = 0u;
      requested[index] = 0u;
      if (*free_top < HAKO_REPLACEMENT_BIN_SLOT_COUNT) {
        free_stack[(*free_top)++] = index;
      }
      add_counter(&direct_core_call_count, 1);
    }
"""
    alloc_index_decl = "" if hotcore_page_model else "  uint32_t index = 0u;\n"

    page_index_source = ""
    find_owned_source = f"""
static int find_owned(
    void* ptr,
    int* bin_out,
    uint32_t* index_out,
    size_t* slot_size_out,
    unsigned char** used_out,
    size_t** requested_out,
    uint32_t** free_stack_out,
    uint32_t** free_top_out) {{
  if (!ptr) return 0;
  uintptr_t value = (uintptr_t)ptr;
  uintptr_t base = 0u;
  uintptr_t end = 0u;
  uintptr_t delta = 0u;
  uintptr_t stride = 0u;
  uint32_t index = 0u;
{chr(10).join(find_cases)}
  return 0;
}}
"""
    if product_pages_nonlinear_lookup:
        find_owned_source = ""
        page_index_source = f"""
/* Benchmark-only ownership index for the page-bins front. This is not the
 * product PageMap, allocator activation, or a full .hako mimalloc algorithm
 * claim. */
#define HAKO_PAGE_INDEX_TABLE_CAP 65536u
#define HAKO_PAGE_INDEX_SHIFT 12u

typedef struct HakoReplacementPageIndexEntry {{
  uintptr_t page_key;
  uintptr_t base;
  uintptr_t end;
  uintptr_t stride;
  size_t slot_size;
  int bin;
  unsigned char* used;
  size_t* requested;
  uint32_t* free_stack;
  uint32_t* free_top;
  unsigned char occupied;
}} HakoReplacementPageIndexEntry;

static HakoReplacementPageIndexEntry page_index_table[HAKO_PAGE_INDEX_TABLE_CAP];
static unsigned long long page_index_insert_count = 0;
static unsigned long long page_index_probe_count = 0;
static unsigned long long page_index_collision_count = 0;
static unsigned long long page_index_overflow_count = 0;

static unsigned int page_index_slot(uintptr_t page_key) {{
  uintptr_t mixed = page_key * 11400714819323198485ull;
  return (unsigned int)(mixed & (HAKO_PAGE_INDEX_TABLE_CAP - 1u));
}}

static void page_index_insert(
    uintptr_t page_key,
    uintptr_t base,
    uintptr_t end,
    uintptr_t stride,
    size_t slot_size,
    int bin,
    unsigned char* used,
    size_t* requested,
    uint32_t* free_stack,
    uint32_t* free_top) {{
  unsigned int slot = page_index_slot(page_key);
  for (unsigned int probe = 0; probe < HAKO_PAGE_INDEX_TABLE_CAP; probe++) {{
    HakoReplacementPageIndexEntry* entry =
        &page_index_table[(slot + probe) & (HAKO_PAGE_INDEX_TABLE_CAP - 1u)];
    if (!entry->occupied) {{
      entry->page_key = page_key;
      entry->base = base;
      entry->end = end;
      entry->stride = stride;
      entry->slot_size = slot_size;
      entry->bin = bin;
      entry->used = used;
      entry->requested = requested;
      entry->free_stack = free_stack;
      entry->free_top = free_top;
      entry->occupied = 1u;
      page_index_insert_count++;
      return;
    }}
    if (entry->page_key == page_key) {{
      page_index_collision_count++;
    }}
  }}
  page_index_overflow_count++;
}}

static void page_index_register_range(
    uintptr_t base,
    uintptr_t end,
    uintptr_t stride,
    size_t slot_size,
    int bin,
    unsigned char* used,
    size_t* requested,
    uint32_t* free_stack,
    uint32_t* free_top) {{
  uintptr_t first_page = base >> HAKO_PAGE_INDEX_SHIFT;
  uintptr_t last_page = (end - 1u) >> HAKO_PAGE_INDEX_SHIFT;
  for (uintptr_t page = first_page; page <= last_page; page++) {{
    page_index_insert(page, base, end, stride, slot_size, bin, used, requested, free_stack, free_top);
  }}
}}

static int find_owned(
    void* ptr,
    int* bin_out,
    uint32_t* index_out,
    size_t* slot_size_out,
    unsigned char** used_out,
    size_t** requested_out,
    uint32_t** free_stack_out,
    uint32_t** free_top_out) {{
  if (!ptr) return 0;
  uintptr_t value = (uintptr_t)ptr;
  uintptr_t page_key = value >> HAKO_PAGE_INDEX_SHIFT;
  unsigned int slot = page_index_slot(page_key);
  for (unsigned int probe = 0; probe < HAKO_PAGE_INDEX_TABLE_CAP; probe++) {{
    HakoReplacementPageIndexEntry* entry =
        &page_index_table[(slot + probe) & (HAKO_PAGE_INDEX_TABLE_CAP - 1u)];
    if (!entry->occupied) return 0;
    if (entry->page_key != page_key) continue;
    page_index_probe_count++;
    if (value < entry->base || value >= entry->end) continue;
    uintptr_t delta = value - entry->base;
    if ((delta % entry->stride) != 0) continue;
    uintptr_t index = delta / entry->stride;
    if (index >= HAKO_REPLACEMENT_BIN_SLOT_COUNT) continue;
    *bin_out = entry->bin;
    *index_out = (uint32_t)index;
    *slot_size_out = entry->slot_size;
    *used_out = entry->used;
    *requested_out = entry->requested;
    *free_stack_out = entry->free_stack;
    *free_top_out = entry->free_top;
    return 1;
  }}
  return 0;
}}
"""
    else:
        page_index_source = """
static unsigned long long page_index_insert_count = 0;
static unsigned long long page_index_probe_count = 0;
static unsigned long long page_index_collision_count = 0;
static unsigned long long page_index_overflow_count = 0;
"""

    malloc_init_line = (
        "  if (!init_done) return real_malloc_fn ? real_malloc_fn(size) : 0;"
        if eager_init
        else "  init_bins();"
    )
    constructor_init_line = "  init_bins();" if eager_init else ""

    return f"""
#define _GNU_SOURCE
#include <dlfcn.h>
#include <fcntl.h>
#include <stdint.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#ifdef HAKO_REPLACEMENT_FRONT_LOCKED
#include <pthread.h>
#endif

#define HAKO_REPLACEMENT_BIN_SLOT_COUNT 8192u

typedef void* (*hako_malloc_fn)(size_t);
typedef void* (*hako_calloc_fn)(size_t, size_t);
typedef void* (*hako_realloc_fn)(void*, size_t);
typedef void (*hako_free_fn)(void*);

static hako_malloc_fn real_malloc_fn = 0;
static hako_calloc_fn real_calloc_fn = 0;
static hako_realloc_fn real_realloc_fn = 0;
static hako_free_fn real_free_fn = 0;
static int resolving_real = 0;

{chr(10).join(bin_defs)}

static unsigned char init_done = 0u;
static unsigned long long alloc_count = 0;
static unsigned long long calloc_count = 0;
static unsigned long long realloc_count = 0;
static unsigned long long free_count = 0;
static unsigned long long host_passthrough_count = 0;
static unsigned long long direct_core_call_count = 0;
static unsigned long long realloc_copy_bytes = 0;
static unsigned long long realloc_inplace_count = 0;
static unsigned long long calloc_zero_bytes = 0;
static unsigned long long lock_mode_enabled = 0;
static unsigned long long lock_enter_count = 0;

#ifdef HAKO_REPLACEMENT_FRONT_LOCKED
static pthread_mutex_t arena_lock = PTHREAD_MUTEX_INITIALIZER;
#endif

static inline void add_counter(unsigned long long* counter, unsigned long long delta) {{
#ifdef HAKO_REPLACEMENT_FRONT_LOCKED
  __sync_fetch_and_add(counter, delta);
#else
  *counter += delta;
#endif
}}

static inline void lock_arena(void) {{
#ifdef HAKO_REPLACEMENT_FRONT_LOCKED
  pthread_mutex_lock(&arena_lock);
  add_counter(&lock_enter_count, 1);
#endif
}}

static inline void unlock_arena(void) {{
#ifdef HAKO_REPLACEMENT_FRONT_LOCKED
  pthread_mutex_unlock(&arena_lock);
#endif
}}

{page_index_source}

static void resolve_real(void) {{
  if (resolving_real) return;
  resolving_real = 1;
  if (!real_malloc_fn) real_malloc_fn = (hako_malloc_fn)dlsym(RTLD_NEXT, "malloc");
  if (!real_calloc_fn) real_calloc_fn = (hako_calloc_fn)dlsym(RTLD_NEXT, "calloc");
  if (!real_realloc_fn) real_realloc_fn = (hako_realloc_fn)dlsym(RTLD_NEXT, "realloc");
  if (!real_free_fn) real_free_fn = (hako_free_fn)dlsym(RTLD_NEXT, "free");
  resolving_real = 0;
}}

static void init_bins(void) {{
  if (init_done) return;
{chr(10).join(init_cases)}
{chr(10).join(page_index_register_cases) if product_pages_nonlinear_lookup else ""}
  init_done = 1u;
}}

{chr(10).join(helper_defs)}

{size_to_bin_source}

static void* alloc_from_bin(int bin, size_t size) {{
{alloc_index_decl.rstrip()}
  switch (bin) {{
{chr(10).join(alloc_cases)}
    default:
      return 0;
  }}
}}

{find_owned_source}

{release_from_bin_source}

void* malloc(size_t size) {{
  add_counter(&alloc_count, 1);
  lock_arena();
{malloc_init_line}
  int bin = size_to_bin(size);
  if (bin >= 0) {{
    void* ptr = alloc_from_bin(bin, size);
    if (ptr) {{
      unlock_arena();
      return ptr;
    }}
  }}
  add_counter(&host_passthrough_count, 1);
  unlock_arena();
  resolve_real();
  return real_malloc_fn ? real_malloc_fn(size) : 0;
}}

void free(void* ptr) {{
  add_counter(&free_count, 1);
  if (!ptr) return;
  int bin = 0;
  uint32_t index = 0u;
  size_t slot_size = 0u;
  unsigned char* used = 0;
  size_t* requested = 0;
  uint32_t* free_stack = 0;
  uint32_t* free_top = 0;
  lock_arena();
  if (find_owned(ptr, &bin, &index, &slot_size, &used, &requested, &free_stack, &free_top)) {{
{free_owned_body}
    unlock_arena();
    return;
  }}
  add_counter(&host_passthrough_count, 1);
  unlock_arena();
  resolve_real();
  if (real_free_fn) real_free_fn(ptr);
}}

void* calloc(size_t nmemb, size_t size) {{
  add_counter(&calloc_count, 1);
  if (size != 0 && nmemb > ((size_t)-1) / size) {{
    add_counter(&host_passthrough_count, 1);
    resolve_real();
    return real_calloc_fn ? real_calloc_fn(nmemb, size) : 0;
  }}
  size_t total = nmemb * size;
  void* ptr = malloc(total);
  if (ptr) {{
    memset(ptr, 0, total);
    add_counter(&calloc_zero_bytes, total);
  }}
  return ptr;
}}

void* realloc(void* ptr, size_t size) {{
  add_counter(&realloc_count, 1);
  if (!ptr) return malloc(size);
  if (size == 0) {{
    free(ptr);
    return 0;
  }}
  int bin = 0;
  uint32_t index = 0u;
  size_t slot_size = 0u;
  unsigned char* used = 0;
  size_t* requested = 0;
  uint32_t* free_stack = 0;
  uint32_t* free_top = 0;
  lock_arena();
  if (find_owned(ptr, &bin, &index, &slot_size, &used, &requested, &free_stack, &free_top)) {{
    (void)bin;
    (void)free_stack;
    (void)free_top;
    if (used[index] == 1u && size <= slot_size) {{
      requested[index] = size;
      add_counter(&realloc_inplace_count, 1);
      add_counter(&direct_core_call_count, 1);
      unlock_arena();
      return ptr;
    }}
    size_t old_size = requested[index];
    unlock_arena();
    void* next = malloc(size);
    if (!next) return 0;
    size_t copy_size = old_size < size ? old_size : size;
    memcpy(next, ptr, copy_size);
    add_counter(&realloc_copy_bytes, copy_size);
    free(ptr);
    return next;
  }}
  add_counter(&host_passthrough_count, 1);
  unlock_arena();
  resolve_real();
  return real_realloc_fn ? real_realloc_fn(ptr, size) : 0;
}}
""" + REPORT_C
