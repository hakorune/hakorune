"""Replacement-front shim raw C source only."""

from __future__ import annotations

from replacement_front_shim_report_source import REPORT_C


REPLACEMENT_FRONT_SHIM_C = r"""
#define _GNU_SOURCE
#include <dlfcn.h>
#include <fcntl.h>
#include <stdint.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#if defined(HAKO_REPLACEMENT_FRONT_LOCKED) || defined(HAKO_REPLACEMENT_FRONT_THREAD_LOCAL)
#include <pthread.h>
#endif

#ifndef HAKO_REPLACEMENT_SLOT_SIZE
#define HAKO_REPLACEMENT_SLOT_SIZE 2048u
#endif
#define HAKO_REPLACEMENT_SLOT_COUNT 8192u
#define HAKO_REPLACEMENT_ARENA_REGISTRY_CAP 128u

typedef void* (*hako_malloc_fn)(size_t);
typedef void* (*hako_calloc_fn)(size_t, size_t);
typedef void* (*hako_realloc_fn)(void*, size_t);
typedef void (*hako_free_fn)(void*);

typedef union HakoReplacementSlot {
  max_align_t align;
  unsigned char bytes[HAKO_REPLACEMENT_SLOT_SIZE];
} HakoReplacementSlot;

static hako_malloc_fn real_malloc_fn = 0;
static hako_calloc_fn real_calloc_fn = 0;
static hako_realloc_fn real_realloc_fn = 0;
static hako_free_fn real_free_fn = 0;
static int resolving_real = 0;

#ifdef HAKO_REPLACEMENT_FRONT_THREAD_LOCAL
#if defined(__GNUC__)
#define HAKO_REPLACEMENT_STORAGE _Thread_local __attribute__((tls_model("initial-exec")))
#else
#define HAKO_REPLACEMENT_STORAGE _Thread_local
#endif
#else
#define HAKO_REPLACEMENT_STORAGE
#endif

#if defined(__GNUC__)
#define HAKO_ALWAYS_INLINE static inline __attribute__((always_inline))
#define HAKO_COLD_NOINLINE static __attribute__((cold, noinline))
#else
#define HAKO_ALWAYS_INLINE static inline
#define HAKO_COLD_NOINLINE static
#endif

static HAKO_REPLACEMENT_STORAGE HakoReplacementSlot slots[HAKO_REPLACEMENT_SLOT_COUNT];
static HAKO_REPLACEMENT_STORAGE unsigned char used[HAKO_REPLACEMENT_SLOT_COUNT];
static HAKO_REPLACEMENT_STORAGE size_t requested_size[HAKO_REPLACEMENT_SLOT_COUNT];
static HAKO_REPLACEMENT_STORAGE uint32_t free_stack[HAKO_REPLACEMENT_SLOT_COUNT];
static HAKO_REPLACEMENT_STORAGE uint32_t free_top = 0u;
static HAKO_REPLACEMENT_STORAGE unsigned char init_done = 0u;

#ifdef HAKO_REPLACEMENT_FRONT_THREAD_LOCAL
static HAKO_REPLACEMENT_STORAGE uint32_t remote_next[HAKO_REPLACEMENT_SLOT_COUNT];
static HAKO_REPLACEMENT_STORAGE int remote_head = -1;
static HAKO_REPLACEMENT_STORAGE unsigned char arena_registered = 0u;

typedef struct HakoReplacementArenaView {
  uintptr_t base;
  uintptr_t end;
  unsigned char* used;
  size_t* requested_size;
  uint32_t* free_stack;
  uint32_t* free_top;
  uint32_t* remote_next;
  int* remote_head;
  unsigned char active;
} HakoReplacementArenaView;

static HakoReplacementArenaView arena_registry[HAKO_REPLACEMENT_ARENA_REGISTRY_CAP];
static unsigned int arena_registry_count = 0u;
static pthread_mutex_t arena_registry_lock = PTHREAD_MUTEX_INITIALIZER;
static pthread_key_t arena_tls_key;
static pthread_once_t arena_tls_key_once = PTHREAD_ONCE_INIT;
#endif

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
static unsigned long long thread_local_mode_enabled = 0;
static unsigned long long remote_free_push_count = 0;
static unsigned long long remote_free_drain_count = 0;
static unsigned long long cross_thread_realloc_unsupported_count = 0;
static unsigned long long arena_registry_overflow_count = 0;
static unsigned long long abandoned_arena_count = 0;
static unsigned long long abandoned_remote_free_count = 0;
static unsigned long long skip_hot_counters_enabled = 0;
static unsigned long long tls_counter_mode_enabled = 0;
static unsigned long long tls_initial_exec_model_enabled = 0;

#ifdef HAKO_REPLACEMENT_FRONT_TLS_COUNTERS
static HAKO_REPLACEMENT_STORAGE unsigned long long local_alloc_count = 0;
static HAKO_REPLACEMENT_STORAGE unsigned long long local_calloc_count = 0;
static HAKO_REPLACEMENT_STORAGE unsigned long long local_realloc_count = 0;
static HAKO_REPLACEMENT_STORAGE unsigned long long local_free_count = 0;
static HAKO_REPLACEMENT_STORAGE unsigned long long local_host_passthrough_count = 0;
static HAKO_REPLACEMENT_STORAGE unsigned long long local_direct_core_call_count = 0;
static HAKO_REPLACEMENT_STORAGE unsigned long long local_realloc_copy_bytes = 0;
static HAKO_REPLACEMENT_STORAGE unsigned long long local_realloc_inplace_count = 0;
static HAKO_REPLACEMENT_STORAGE unsigned long long local_calloc_zero_bytes = 0;
static HAKO_REPLACEMENT_STORAGE unsigned long long local_lock_enter_count = 0;
static HAKO_REPLACEMENT_STORAGE unsigned long long local_remote_free_push_count = 0;
static HAKO_REPLACEMENT_STORAGE unsigned long long local_remote_free_drain_count = 0;
static HAKO_REPLACEMENT_STORAGE unsigned long long local_cross_thread_realloc_unsupported_count = 0;
static HAKO_REPLACEMENT_STORAGE unsigned long long local_arena_registry_overflow_count = 0;
static HAKO_REPLACEMENT_STORAGE unsigned long long local_abandoned_arena_count = 0;
static HAKO_REPLACEMENT_STORAGE unsigned long long local_abandoned_remote_free_count = 0;

static void flush_one_counter(unsigned long long* global, unsigned long long* local) {
  if (*local != 0) {
    __sync_fetch_and_add(global, *local);
    *local = 0;
  }
}

static void flush_thread_counters(void) {
  flush_one_counter(&alloc_count, &local_alloc_count);
  flush_one_counter(&calloc_count, &local_calloc_count);
  flush_one_counter(&realloc_count, &local_realloc_count);
  flush_one_counter(&free_count, &local_free_count);
  flush_one_counter(&host_passthrough_count, &local_host_passthrough_count);
  flush_one_counter(&direct_core_call_count, &local_direct_core_call_count);
  flush_one_counter(&realloc_copy_bytes, &local_realloc_copy_bytes);
  flush_one_counter(&realloc_inplace_count, &local_realloc_inplace_count);
  flush_one_counter(&calloc_zero_bytes, &local_calloc_zero_bytes);
  flush_one_counter(&lock_enter_count, &local_lock_enter_count);
  flush_one_counter(&remote_free_push_count, &local_remote_free_push_count);
  flush_one_counter(&remote_free_drain_count, &local_remote_free_drain_count);
  flush_one_counter(
      &cross_thread_realloc_unsupported_count,
      &local_cross_thread_realloc_unsupported_count);
  flush_one_counter(&arena_registry_overflow_count, &local_arena_registry_overflow_count);
  flush_one_counter(&abandoned_arena_count, &local_abandoned_arena_count);
  flush_one_counter(&abandoned_remote_free_count, &local_abandoned_remote_free_count);
}
#endif

static void add_counter(unsigned long long* counter, unsigned long long delta) {
#ifdef HAKO_REPLACEMENT_FRONT_SKIP_HOT_COUNTERS
  (void)counter;
  (void)delta;
#elif defined(HAKO_REPLACEMENT_FRONT_TLS_COUNTERS)
  if (!arena_registered) {
    __sync_fetch_and_add(counter, delta);
  } else if (counter == &alloc_count) {
    local_alloc_count += delta;
  } else if (counter == &calloc_count) {
    local_calloc_count += delta;
  } else if (counter == &realloc_count) {
    local_realloc_count += delta;
  } else if (counter == &free_count) {
    local_free_count += delta;
  } else if (counter == &host_passthrough_count) {
    local_host_passthrough_count += delta;
  } else if (counter == &direct_core_call_count) {
    local_direct_core_call_count += delta;
  } else if (counter == &realloc_copy_bytes) {
    local_realloc_copy_bytes += delta;
  } else if (counter == &realloc_inplace_count) {
    local_realloc_inplace_count += delta;
  } else if (counter == &calloc_zero_bytes) {
    local_calloc_zero_bytes += delta;
  } else if (counter == &lock_enter_count) {
    local_lock_enter_count += delta;
  } else if (counter == &remote_free_push_count) {
    local_remote_free_push_count += delta;
  } else if (counter == &remote_free_drain_count) {
    local_remote_free_drain_count += delta;
  } else if (counter == &cross_thread_realloc_unsupported_count) {
    local_cross_thread_realloc_unsupported_count += delta;
  } else if (counter == &arena_registry_overflow_count) {
    local_arena_registry_overflow_count += delta;
  } else if (counter == &abandoned_arena_count) {
    local_abandoned_arena_count += delta;
  } else if (counter == &abandoned_remote_free_count) {
    local_abandoned_remote_free_count += delta;
  }
#else
  __sync_fetch_and_add(counter, delta);
#endif
}

#ifdef HAKO_REPLACEMENT_FRONT_LOCKED
static pthread_mutex_t arena_lock = PTHREAD_MUTEX_INITIALIZER;
#endif

HAKO_ALWAYS_INLINE void lock_arena(void) {
#ifdef HAKO_REPLACEMENT_FRONT_LOCKED
  pthread_mutex_lock(&arena_lock);
  add_counter(&lock_enter_count, 1);
#endif
}

HAKO_ALWAYS_INLINE void unlock_arena(void) {
#ifdef HAKO_REPLACEMENT_FRONT_LOCKED
  pthread_mutex_unlock(&arena_lock);
#endif
}

static void resolve_real(void) {
  if (resolving_real) {
    return;
  }
  resolving_real = 1;
  if (!real_malloc_fn) {
    real_malloc_fn = (hako_malloc_fn)dlsym(RTLD_NEXT, "malloc");
  }
  if (!real_calloc_fn) {
    real_calloc_fn = (hako_calloc_fn)dlsym(RTLD_NEXT, "calloc");
  }
  if (!real_realloc_fn) {
    real_realloc_fn = (hako_realloc_fn)dlsym(RTLD_NEXT, "realloc");
  }
  if (!real_free_fn) {
    real_free_fn = (hako_free_fn)dlsym(RTLD_NEXT, "free");
  }
  resolving_real = 0;
}

static void init_slots(void) {
  if (init_done) {
    return;
  }
  for (uint32_t i = 0; i < HAKO_REPLACEMENT_SLOT_COUNT; i++) {
    free_stack[i] = HAKO_REPLACEMENT_SLOT_COUNT - i - 1u;
#ifdef HAKO_REPLACEMENT_FRONT_THREAD_LOCAL
    remote_next[i] = (uint32_t)-1;
#endif
  }
  free_top = HAKO_REPLACEMENT_SLOT_COUNT;
  init_done = 1u;
}

#ifdef HAKO_REPLACEMENT_FRONT_THREAD_LOCAL
static void arena_tls_destructor(void* value) {
  if (!value) {
    return;
  }
  unsigned int arena_index = (unsigned int)((uintptr_t)value - 1u);
  pthread_mutex_lock(&arena_registry_lock);
  if (arena_index < arena_registry_count) {
    arena_registry[arena_index].active = 0u;
    add_counter(&abandoned_arena_count, 1);
#ifdef HAKO_REPLACEMENT_FRONT_TLS_COUNTERS
    flush_thread_counters();
#endif
  }
  pthread_mutex_unlock(&arena_registry_lock);
}

static void make_arena_tls_key(void) {
  pthread_key_create(&arena_tls_key, arena_tls_destructor);
}

static int register_thread_arena(void) {
  if (arena_registered) {
    return 1;
  }
  pthread_once(&arena_tls_key_once, make_arena_tls_key);
  pthread_mutex_lock(&arena_registry_lock);
  if (arena_registry_count < HAKO_REPLACEMENT_ARENA_REGISTRY_CAP) {
    unsigned int arena_index = arena_registry_count++;
    HakoReplacementArenaView* view = &arena_registry[arena_index];
    view->base = (uintptr_t)slots[0].bytes;
    view->end = (uintptr_t)(slots + HAKO_REPLACEMENT_SLOT_COUNT);
    view->used = used;
    view->requested_size = requested_size;
    view->free_stack = free_stack;
    view->free_top = &free_top;
    view->remote_next = remote_next;
    view->remote_head = &remote_head;
    view->active = 1u;
    arena_registered = 1u;
    pthread_setspecific(arena_tls_key, (void*)((uintptr_t)arena_index + 1u));
  } else {
    add_counter(&arena_registry_overflow_count, 1);
  }
  pthread_mutex_unlock(&arena_registry_lock);
  return arena_registered ? 1 : 0;
}

static int arena_view_slot_index(HakoReplacementArenaView* view, void* ptr) {
  uintptr_t value = (uintptr_t)ptr;
  if (value < view->base || value >= view->end) {
    return -1;
  }
  uintptr_t delta = value - view->base;
  uintptr_t stride = sizeof(HakoReplacementSlot);
  if ((delta % stride) != 0) {
    return -1;
  }
  uintptr_t index = delta / stride;
  if (index >= HAKO_REPLACEMENT_SLOT_COUNT) {
    return -1;
  }
  return (int)index;
}

static HakoReplacementArenaView* find_foreign_arena(void* ptr, int* index_out) {
  uintptr_t local_base = (uintptr_t)slots[0].bytes;
  uintptr_t local_end = (uintptr_t)(slots + HAKO_REPLACEMENT_SLOT_COUNT);
  uintptr_t value = (uintptr_t)ptr;
  if (value >= local_base && value < local_end) {
    return 0;
  }
  pthread_mutex_lock(&arena_registry_lock);
  for (unsigned int i = 0; i < arena_registry_count; i++) {
    HakoReplacementArenaView* view = &arena_registry[i];
    int index = arena_view_slot_index(view, ptr);
    if (index >= 0) {
      *index_out = index;
      pthread_mutex_unlock(&arena_registry_lock);
      return view;
    }
  }
  pthread_mutex_unlock(&arena_registry_lock);
  return 0;
}

static void drain_remote_frees(void) {
  for (;;) {
    int head = remote_head;
    if (head < 0) {
      return;
    }
    int next = (int)remote_next[(uint32_t)head];
    if (!__sync_bool_compare_and_swap(&remote_head, head, next)) {
      continue;
    }
    used[(uint32_t)head] = 0u;
    requested_size[(uint32_t)head] = 0u;
    if (free_top < HAKO_REPLACEMENT_SLOT_COUNT) {
      free_stack[free_top++] = (uint32_t)head;
      add_counter(&remote_free_drain_count, 1);
    }
  }
}

static int remote_free_to_owner(void* ptr) {
  uintptr_t local_base = (uintptr_t)slots[0].bytes;
  uintptr_t local_end = (uintptr_t)(slots + HAKO_REPLACEMENT_SLOT_COUNT);
  uintptr_t value = (uintptr_t)ptr;
  if (value >= local_base && value < local_end) {
    return 0;
  }
  pthread_mutex_lock(&arena_registry_lock);
  for (unsigned int i = 0; i < arena_registry_count; i++) {
    HakoReplacementArenaView* view = &arena_registry[i];
    int index = arena_view_slot_index(view, ptr);
    if (index < 0) {
      continue;
    }
    if (!view->active) {
      pthread_mutex_unlock(&arena_registry_lock);
      add_counter(&abandoned_remote_free_count, 1);
      add_counter(&direct_core_call_count, 1);
      return 1;
    }
    uint32_t uindex = (uint32_t)index;
    if (!__sync_bool_compare_and_swap(&view->used[uindex], 1u, 2u)) {
      pthread_mutex_unlock(&arena_registry_lock);
      return 0;
    }
    for (;;) {
      int old_head = *view->remote_head;
      view->remote_next[uindex] = (uint32_t)old_head;
      if (__sync_bool_compare_and_swap(view->remote_head, old_head, index)) {
        pthread_mutex_unlock(&arena_registry_lock);
        add_counter(&remote_free_push_count, 1);
        add_counter(&direct_core_call_count, 1);
        return 1;
      }
    }
  }
  pthread_mutex_unlock(&arena_registry_lock);
  return 0;
}
#endif

static int slot_index(void* ptr) {
  if (!ptr) {
    return -1;
  }
  uintptr_t value = (uintptr_t)ptr;
  uintptr_t base = (uintptr_t)slots[0].bytes;
  uintptr_t end = (uintptr_t)(slots + HAKO_REPLACEMENT_SLOT_COUNT);
  if (value < base || value >= end) {
    return -1;
  }
  uintptr_t delta = value - base;
  uintptr_t stride = sizeof(HakoReplacementSlot);
  if ((delta % stride) != 0) {
    return -1;
  }
  uintptr_t index = delta / stride;
  if (index >= HAKO_REPLACEMENT_SLOT_COUNT) {
    return -1;
  }
  return (int)index;
}

HAKO_ALWAYS_INLINE void* direct_alloc_fast(size_t size) {
  if (free_top == 0u) {
    return 0;
  }
  uint32_t index = free_stack[--free_top];
  used[index] = 1u;
  requested_size[index] = size;
  add_counter(&direct_core_call_count, 1);
  return slots[index].bytes;
}

HAKO_COLD_NOINLINE void* direct_alloc_slow(size_t size) {
  if (size == 0 || size > HAKO_REPLACEMENT_SLOT_SIZE) {
    return 0;
  }
  init_slots();
#ifdef HAKO_REPLACEMENT_FRONT_THREAD_LOCAL
  if (!register_thread_arena()) {
    return 0;
  }
  if (remote_head >= 0) {
    drain_remote_frees();
  }
#endif
  return direct_alloc_fast(size);
}

HAKO_ALWAYS_INLINE void* direct_alloc(size_t size) {
  if (size == 0 || size > HAKO_REPLACEMENT_SLOT_SIZE) {
    return 0;
  }
  if (!init_done) {
    return direct_alloc_slow(size);
  }
#ifdef HAKO_REPLACEMENT_FRONT_THREAD_LOCAL
  if (!arena_registered) {
    return direct_alloc_slow(size);
  }
  if (remote_head >= 0) {
    return direct_alloc_slow(size);
  }
#endif
  return direct_alloc_fast(size);
}

HAKO_ALWAYS_INLINE int direct_free_local(void* ptr) {
  int index = slot_index(ptr);
  if (index < 0 || used[(uint32_t)index] != 1u) {
    return 0;
  }
  used[(uint32_t)index] = 0u;
  requested_size[(uint32_t)index] = 0u;
  if (free_top < HAKO_REPLACEMENT_SLOT_COUNT) {
    free_stack[free_top++] = (uint32_t)index;
  }
  add_counter(&direct_core_call_count, 1);
  return 1;
}

__attribute__((visibility("default")))
void* malloc(size_t size) {
  if (resolving_real) {
    return real_malloc_fn ? real_malloc_fn(size) : 0;
  }
  lock_arena();
  void* ptr = direct_alloc(size);
  unlock_arena();
  if (ptr) {
    add_counter(&alloc_count, 1);
    return ptr;
  }
  add_counter(&host_passthrough_count, 1);
  resolve_real();
  return real_malloc_fn ? real_malloc_fn(size) : 0;
}

__attribute__((visibility("default")))
void* calloc(size_t count, size_t size) {
  if (resolving_real) {
    return real_calloc_fn ? real_calloc_fn(count, size) : 0;
  }
  if (size != 0 && count > ((size_t)-1) / size) {
    return 0;
  }
  size_t bytes = count * size;
  lock_arena();
  void* ptr = direct_alloc(bytes);
  unlock_arena();
  if (!ptr) {
    add_counter(&host_passthrough_count, 1);
    resolve_real();
    return real_calloc_fn ? real_calloc_fn(count, size) : 0;
  }
  memset(ptr, 0, bytes);
  add_counter(&calloc_zero_bytes, bytes);
  add_counter(&calloc_count, 1);
  return ptr;
}

__attribute__((visibility("default")))
void* realloc(void* ptr, size_t size) {
  if (!ptr) {
    return malloc(size);
  }
  if (size == 0) {
    free(ptr);
    return 0;
  }
  if (resolving_real) {
    return real_realloc_fn ? real_realloc_fn(ptr, size) : 0;
  }
  lock_arena();
  int index = slot_index(ptr);
  if (index < 0 || used[(uint32_t)index] != 1u) {
    unlock_arena();
#ifdef HAKO_REPLACEMENT_FRONT_THREAD_LOCAL
    int foreign_index = -1;
    if (find_foreign_arena(ptr, &foreign_index)) {
      add_counter(&cross_thread_realloc_unsupported_count, 1);
      return 0;
    }
#endif
    add_counter(&host_passthrough_count, 1);
    resolve_real();
    return real_realloc_fn ? real_realloc_fn(ptr, size) : 0;
  }
  size_t old_size = requested_size[(uint32_t)index];
  if (size <= HAKO_REPLACEMENT_SLOT_SIZE) {
    requested_size[(uint32_t)index] = size;
    unlock_arena();
    add_counter(&realloc_inplace_count, 1);
    add_counter(&realloc_count, 1);
    return ptr;
  }
  void* next = direct_alloc(size);
  if (!next) {
    unlock_arena();
    return 0;
  }
  size_t copy_size = old_size < size ? old_size : size;
  memcpy(next, ptr, copy_size);
  add_counter(&realloc_copy_bytes, copy_size);
  direct_free_local(ptr);
  unlock_arena();
  add_counter(&realloc_count, 1);
  return next;
}

__attribute__((visibility("default")))
void free(void* ptr) {
  if (!ptr) {
    return;
  }
  if (resolving_real) {
    if (real_free_fn) {
      real_free_fn(ptr);
    }
    return;
  }
  lock_arena();
  int freed = direct_free_local(ptr);
  unlock_arena();
#ifdef HAKO_REPLACEMENT_FRONT_THREAD_LOCAL
  if (!freed) {
    freed = remote_free_to_owner(ptr);
  }
#endif
  if (freed) {
    add_counter(&free_count, 1);
    return;
  }
  add_counter(&host_passthrough_count, 1);
  resolve_real();
  if (real_free_fn) {
    real_free_fn(ptr);
  }
}
""" + REPORT_C
