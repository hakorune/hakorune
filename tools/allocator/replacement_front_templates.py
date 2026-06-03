"""Support code for Hakozuna replacement-front benchmark probes.

This module owns generated C source text plus the small deterministic workload
and size-class helpers needed to build those benchmark-only fronts.
Runtime/report orchestration stays in hakozuna_mixed_ws_ldpreload_compare.py.
"""

from __future__ import annotations


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

static void write_str(int fd, const char* s) {
  size_t len = 0;
  while (s[len]) {
    len++;
  }
  ssize_t ignored = write(fd, s, len);
  (void)ignored;
}

static void write_u64(int fd, unsigned long long value) {
  char buf[32];
  int pos = 31;
  buf[pos--] = '\n';
  if (value == 0) {
    buf[pos--] = '0';
  } else {
    while (value > 0 && pos >= 0) {
      buf[pos--] = (char)('0' + (value % 10));
      value /= 10;
    }
  }
  ssize_t ignored = write(fd, buf + pos + 1, (size_t)(31 - pos));
  (void)ignored;
}

static void write_kv(int fd, const char* key, unsigned long long value) {
  write_str(fd, key);
  write_str(fd, "=");
  write_u64(fd, value);
}

static void write_report(void) {
#ifdef HAKO_REPLACEMENT_FRONT_TLS_COUNTERS
  flush_thread_counters();
#endif
  const char* path = getenv("HAKORUNE_REPLACEMENT_FRONT_REPORT");
  if (!path || !path[0]) {
    return;
  }
  int fd = open(path, O_CREAT | O_TRUNC | O_WRONLY, 0644);
  if (fd < 0) {
    return;
  }
  write_kv(fd, "replacement_front_alloc_count", alloc_count);
  write_kv(fd, "replacement_front_calloc_count", calloc_count);
  write_kv(fd, "replacement_front_realloc_count", realloc_count);
  write_kv(fd, "replacement_front_free_count", free_count);
  write_kv(fd, "replacement_front_host_passthrough_count", host_passthrough_count);
  write_kv(fd, "replacement_front_direct_core_call_count", direct_core_call_count);
  write_kv(fd, "replacement_front_realloc_copy_bytes", realloc_copy_bytes);
  write_kv(fd, "replacement_front_realloc_inplace_count", realloc_inplace_count);
  write_kv(fd, "replacement_front_calloc_zero_bytes", calloc_zero_bytes);
  write_kv(fd, "replacement_front_lock_mode_enabled", lock_mode_enabled);
  write_kv(fd, "replacement_front_lock_enter_count", lock_enter_count);
  write_kv(fd, "replacement_front_thread_local_mode_enabled", thread_local_mode_enabled);
  write_kv(fd, "replacement_front_remote_free_push_count", remote_free_push_count);
  write_kv(fd, "replacement_front_remote_free_drain_count", remote_free_drain_count);
  write_kv(
      fd,
      "replacement_front_cross_thread_realloc_unsupported_count",
      cross_thread_realloc_unsupported_count);
  write_kv(fd, "replacement_front_arena_registry_overflow_count", arena_registry_overflow_count);
  write_kv(fd, "replacement_front_abandoned_arena_count", abandoned_arena_count);
  write_kv(fd, "replacement_front_abandoned_remote_free_count", abandoned_remote_free_count);
  write_kv(fd, "replacement_front_skip_hot_counters_enabled", skip_hot_counters_enabled);
  write_kv(fd, "replacement_front_tls_counter_mode_enabled", tls_counter_mode_enabled);
  write_kv(
      fd,
      "replacement_front_tls_initial_exec_model_enabled",
      tls_initial_exec_model_enabled);
  close(fd);
}

__attribute__((constructor))
static void install_report(void) {
#ifdef HAKO_REPLACEMENT_FRONT_LOCKED
  lock_mode_enabled = 1;
#endif
#ifdef HAKO_REPLACEMENT_FRONT_THREAD_LOCAL
  thread_local_mode_enabled = 1;
#endif
#ifdef HAKO_REPLACEMENT_FRONT_SKIP_HOT_COUNTERS
  skip_hot_counters_enabled = 1;
#endif
#ifdef HAKO_REPLACEMENT_FRONT_TLS_COUNTERS
  tls_counter_mode_enabled = 1;
#endif
#if defined(HAKO_REPLACEMENT_FRONT_THREAD_LOCAL) && defined(__GNUC__)
  tls_initial_exec_model_enabled = 1;
#endif
  atexit(write_report);
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
"""


def generate_replacement_front_bins_shim_c(
    required_bins: list[int],
    *,
    page_shaped: bool = False,
    hotcore_page_model: bool = False,
    size_class_table: bool = False,
    eager_init: bool = False,
    product_pages_nonlinear_lookup: bool = False,
) -> str:
    """Generate a benchmark-only multi-bin replacement front.

    This is intentionally narrower than the fixed-slot front: single-thread,
    no remote-free bridge, and no product allocator claim. It exists to prove
    the next size-class bridge shape without weakening ProviderFront.
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
  direct_core_call_count++;
  return {slot_expr}[index].bytes;
}}

static inline int hako_page_release_local_known_live_{tag}(uint32_t index) {{
  if (index >= HAKO_REPLACEMENT_BIN_SLOT_COUNT || {used_expr}[index] != 1u) return 0;
  {used_expr}[index] = 0u;
  {requested_expr}[index] = 0u;
  if ({free_top_expr} < HAKO_REPLACEMENT_BIN_SLOT_COUNT) {{
    {free_stack_expr}[{free_top_expr}++] = index;
  }}
  direct_core_call_count++;
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
      direct_core_call_count++;
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
      direct_core_call_count++;
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

static void write_str(int fd, const char* s) {{
  size_t len = 0;
  while (s[len]) len++;
  ssize_t ignored = write(fd, s, len);
  (void)ignored;
}}

static void write_u64(int fd, unsigned long long value) {{
  char buf[32];
  int pos = 31;
  buf[pos--] = '\\n';
  if (value == 0) {{
    buf[pos--] = '0';
  }} else {{
    while (value > 0 && pos >= 0) {{
      buf[pos--] = (char)('0' + (value % 10));
      value /= 10;
    }}
  }}
  ssize_t ignored = write(fd, buf + pos + 1, (size_t)(31 - pos));
  (void)ignored;
}}

static void write_kv(int fd, const char* key, unsigned long long value) {{
  write_str(fd, key);
  write_str(fd, "=");
  write_u64(fd, value);
}}

static void write_report(void) {{
  const char* path = getenv("HAKORUNE_REPLACEMENT_FRONT_REPORT");
  if (!path || !path[0]) return;
  int fd = open(path, O_CREAT | O_TRUNC | O_WRONLY, 0644);
  if (fd < 0) return;
  write_kv(fd, "replacement_front_alloc_count", alloc_count);
  write_kv(fd, "replacement_front_calloc_count", calloc_count);
  write_kv(fd, "replacement_front_realloc_count", realloc_count);
  write_kv(fd, "replacement_front_free_count", free_count);
  write_kv(fd, "replacement_front_host_passthrough_count", host_passthrough_count);
  write_kv(fd, "replacement_front_direct_core_call_count", direct_core_call_count);
  write_kv(fd, "replacement_front_realloc_copy_bytes", realloc_copy_bytes);
  write_kv(fd, "replacement_front_realloc_inplace_count", realloc_inplace_count);
  write_kv(fd, "replacement_front_calloc_zero_bytes", calloc_zero_bytes);
  write_kv(fd, "replacement_front_page_index_insert_count", page_index_insert_count);
  write_kv(fd, "replacement_front_page_index_probe_count", page_index_probe_count);
  write_kv(fd, "replacement_front_page_index_collision_count", page_index_collision_count);
  write_kv(fd, "replacement_front_page_index_overflow_count", page_index_overflow_count);
  close(fd);
}}

__attribute__((constructor)) static void replacement_front_init(void) {{
{constructor_init_line}
  resolve_real();
  atexit(write_report);
}}

void* malloc(size_t size) {{
  alloc_count++;
{malloc_init_line}
  int bin = size_to_bin(size);
  if (bin >= 0) {{
    void* ptr = alloc_from_bin(bin, size);
    if (ptr) return ptr;
  }}
  host_passthrough_count++;
  resolve_real();
  return real_malloc_fn ? real_malloc_fn(size) : 0;
}}

void free(void* ptr) {{
  free_count++;
  if (!ptr) return;
  int bin = 0;
  uint32_t index = 0u;
  size_t slot_size = 0u;
  unsigned char* used = 0;
  size_t* requested = 0;
  uint32_t* free_stack = 0;
  uint32_t* free_top = 0;
  if (find_owned(ptr, &bin, &index, &slot_size, &used, &requested, &free_stack, &free_top)) {{
{free_owned_body}
    return;
  }}
  host_passthrough_count++;
  resolve_real();
  if (real_free_fn) real_free_fn(ptr);
}}

void* calloc(size_t nmemb, size_t size) {{
  calloc_count++;
  if (size != 0 && nmemb > ((size_t)-1) / size) {{
    host_passthrough_count++;
    resolve_real();
    return real_calloc_fn ? real_calloc_fn(nmemb, size) : 0;
  }}
  size_t total = nmemb * size;
  void* ptr = malloc(total);
  if (ptr) {{
    memset(ptr, 0, total);
    calloc_zero_bytes += total;
  }}
  return ptr;
}}

void* realloc(void* ptr, size_t size) {{
  realloc_count++;
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
  if (find_owned(ptr, &bin, &index, &slot_size, &used, &requested, &free_stack, &free_top)) {{
    (void)bin;
    (void)free_stack;
    (void)free_top;
    if (used[index] == 1u && size <= slot_size) {{
      requested[index] = size;
      realloc_inplace_count++;
      direct_core_call_count++;
      return ptr;
    }}
    size_t old_size = requested[index];
    void* next = malloc(size);
    if (!next) return 0;
    size_t copy_size = old_size < size ? old_size : size;
    memcpy(next, ptr, copy_size);
    realloc_copy_bytes += copy_size;
    free(ptr);
    return next;
  }}
  host_passthrough_count++;
  resolve_real();
  return real_realloc_fn ? real_realloc_fn(ptr, size) : 0;
}}
"""


REPLACEMENT_FRONT_CROSS_THREAD_FREE_SMOKE_C = r"""
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

static void* shared_ptr = 0;

static void* free_from_worker(void* arg) {
  (void)arg;
  free(shared_ptr);
  shared_ptr = 0;
  return 0;
}

int main(void) {
  shared_ptr = malloc(64);
  if (!shared_ptr) {
    fputs("malloc failed\n", stderr);
    return 1;
  }
  pthread_t thread;
  if (pthread_create(&thread, 0, free_from_worker, 0) != 0) {
    fputs("pthread_create failed\n", stderr);
    return 2;
  }
  if (pthread_join(thread, 0) != 0) {
    fputs("pthread_join failed\n", stderr);
    return 3;
  }
  void* drained = malloc(64);
  if (!drained) {
    fputs("drain malloc failed\n", stderr);
    return 4;
  }
  free(drained);
  return 0;
}
"""


REPLACEMENT_FRONT_ABANDONED_OWNER_SMOKE_C = r"""
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

static void* shared_ptr = 0;

static void* allocate_from_worker(void* arg) {
  (void)arg;
  shared_ptr = malloc(64);
  if (!shared_ptr) {
    return (void*)1;
  }
  return 0;
}

int main(void) {
  pthread_t thread;
  void* thread_result = 0;
  if (pthread_create(&thread, 0, allocate_from_worker, 0) != 0) {
    fputs("pthread_create failed\n", stderr);
    return 1;
  }
  if (pthread_join(thread, &thread_result) != 0) {
    fputs("pthread_join failed\n", stderr);
    return 2;
  }
  if (thread_result != 0 || !shared_ptr) {
    fputs("worker malloc failed\n", stderr);
    return 3;
  }
  free(shared_ptr);
  shared_ptr = 0;
  return 0;
}
"""


REPLACEMENT_FRONT_CROSS_THREAD_REALLOC_SMOKE_C = r"""
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

static void* shared_ptr = 0;
static void* realloc_result = 0;

static void* realloc_from_worker(void* arg) {
  (void)arg;
  realloc_result = realloc(shared_ptr, 128);
  return 0;
}

int main(void) {
  shared_ptr = malloc(64);
  if (!shared_ptr) {
    fputs("malloc failed\n", stderr);
    return 1;
  }
  pthread_t thread;
  if (pthread_create(&thread, 0, realloc_from_worker, 0) != 0) {
    fputs("pthread_create failed\n", stderr);
    return 2;
  }
  if (pthread_join(thread, 0) != 0) {
    fputs("pthread_join failed\n", stderr);
    return 3;
  }
  if (realloc_result != 0) {
    fputs("cross-thread realloc unexpectedly succeeded\n", stderr);
    return 4;
  }
  free(shared_ptr);
  shared_ptr = 0;
  return 0;
}
"""


def median_float(values: list[float]) -> float:
    ordered = sorted(values)
    return ordered[len(ordered) // 2]


def positive_int(value: int, label: str) -> None:
    if value < 1:
        raise SystemExit(f"{label} must be positive")


def counter_value(counters: dict[str, object], key: str) -> int:
    value = counters.get(key, "0")
    if isinstance(value, int):
        return value
    if isinstance(value, str) and value.isdigit():
        return int(value)
    return 0


WORKLOAD_HISTOGRAM_MAX_TOTAL_ITERS = 1_000_000


def lcg_next(value: int) -> int:
    return ((value * 1664525) + 1013904223) & 0xFFFFFFFF


def mixed_ws_pick_size(value: int, min_size: int, max_size: int) -> int:
    span = (max_size - min_size + 1) if max_size > min_size else 1
    return min_size + (value % span)


def size_bucket(size: int) -> str:
    if size <= 64:
        return "le_64"
    if size <= 128:
        return "le_128"
    if size <= 256:
        return "le_256"
    if size <= 512:
        return "le_512"
    if size <= 1024:
        return "le_1024"
    return "gt_1024"


def hako_size_class_bin_size(bin_index: int) -> int:
    """Mirror SizeClassBox.bin_size for report-only workload classification."""
    word_size = 8
    max_regular_bin = 72
    if bin_index <= 0:
        return -1
    if bin_index <= 8:
        return bin_index * word_size
    if bin_index > max_regular_bin:
        return -1

    x = bin_index + 3
    bit_group = x // 4
    top = x - (bit_group * 4)
    scale = 1 << max(0, bit_group - 2)
    words = (5 + top) * scale
    return words * word_size


def hako_size_to_bin(size: int) -> int:
    """Mirror SizeClassBox.size_to_bin for report-only workload classification."""
    max_regular_bin = 72
    huge_bin = 73
    n = size if size > 0 else 1
    for bin_index in range(1, max_regular_bin + 1):
        if n <= hako_size_class_bin_size(bin_index):
            return bin_index
    return huge_bin


def hako_good_size(size: int) -> int:
    """Mirror SizeClassBox.good_size for benchmark-only size-class bridging."""
    bin_index = hako_size_to_bin(size)
    if bin_index == 73:
        return -1
    return hako_size_class_bin_size(bin_index)


def mixed_ws_workload_histogram(
    *,
    threads: int,
    iters_per_thread: int,
    working_set: int,
    min_size: int,
    max_size: int,
    replacement_slot_size: int,
) -> dict[str, int | str]:
    sampled_iters_per_thread = iters_per_thread
    if threads * iters_per_thread > WORKLOAD_HISTOGRAM_MAX_TOTAL_ITERS:
        sampled_iters_per_thread = max(1, WORKLOAD_HISTOGRAM_MAX_TOTAL_ITERS // threads)
    exact = sampled_iters_per_thread == iters_per_thread

    buckets = {
        "le_64": 0,
        "le_128": 0,
        "le_256": 0,
        "le_512": 0,
        "le_1024": 0,
        "gt_1024": 0,
    }
    alloc_requests = 0
    free_path_count = 0
    cleanup_free_count = 0
    realloc_requests = 0
    realloc_gt_slot = 0
    realloc_gt_max_size = 0
    memset_le_64_count = 0
    memset_gt_64_count = 0
    size_class_counts: dict[int, int] = {}

    ws = working_set if working_set > 0 else 1

    def record_size_class(request_size: int) -> None:
        bin_index = hako_size_to_bin(request_size)
        size_class_counts[bin_index] = size_class_counts.get(bin_index, 0) + 1

    for thread_index in range(threads):
        seed = 1234 + thread_index
        slots = [False] * ws
        for iteration in range(sampled_iters_per_thread):
            seed = lcg_next(seed)
            idx = seed % ws
            if slots[idx]:
                free_path_count += 1
                slots[idx] = False
                continue

            size = mixed_ws_pick_size(seed, min_size, max_size)
            alloc_requests += 1
            buckets[size_bucket(size)] += 1
            record_size_class(size)
            if (iteration & 0x3F) == 0:
                new_size = size + 16
                realloc_requests += 1
                buckets[size_bucket(new_size)] += 1
                record_size_class(new_size)
                if new_size > replacement_slot_size:
                    realloc_gt_slot += 1
                if new_size > max_size:
                    realloc_gt_max_size += 1
                size = new_size
            if size < 64:
                memset_le_64_count += 1
            else:
                memset_gt_64_count += 1
            slots[idx] = True
        cleanup_free_count += sum(1 for occupied in slots if occupied)

    regular_bins = [bin_index for bin_index in size_class_counts if bin_index != 73]
    regular_bins_sorted = sorted(regular_bins)
    max_bin = max(size_class_counts) if size_class_counts else 0
    max_regular_seen = max(regular_bins) if regular_bins else 0

    return {
        "source": "deterministic_prefix_exact" if exact else "deterministic_prefix_sampled",
        "sampled_iters_per_thread": sampled_iters_per_thread,
        "sampled_total_iterations": sampled_iters_per_thread * threads,
        "full_total_iterations": iters_per_thread * threads,
        "sample_exact": 1 if exact else 0,
        "alloc_request_count": alloc_requests,
        "free_path_count": free_path_count,
        "cleanup_free_count": cleanup_free_count,
        "realloc_request_count": realloc_requests,
        "realloc_request_gt_replacement_slot_size": realloc_gt_slot,
        "realloc_request_gt_max_size": realloc_gt_max_size,
        "memset_le_64_count": memset_le_64_count,
        "memset_gt_64_count": memset_gt_64_count,
        "size_class_policy_source": "hako_size_class_box_report_mirror",
        "size_class_distinct_count": len(size_class_counts),
        "size_class_regular_distinct_count": len(regular_bins_sorted),
        "size_class_regular_bins": ",".join(str(bin_index) for bin_index in regular_bins_sorted)
        or "none",
        "size_class_max_bin": max_bin,
        "size_class_max_good_size": hako_size_class_bin_size(max_regular_seen),
        "size_class_huge_count": size_class_counts.get(73, 0),
        "size_class_regular_request_count": sum(
            count for bin_index, count in size_class_counts.items() if bin_index != 73
        ),
        **{f"request_{bucket}": count for bucket, count in buckets.items()},
    }
