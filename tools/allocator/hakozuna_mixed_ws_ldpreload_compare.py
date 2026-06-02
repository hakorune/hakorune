#!/usr/bin/env python3
"""Compare Hakozuna mixed-ws under system, C mimalloc, and optional provider LD_PRELOAD."""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
from pathlib import Path
from typing import Any

from hako_mimalloc_provider_backed_hakmem_ldpreload_bench_pilot import read_kv


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_HAKOZUNA_ROOT = ROOT / "benchmarks" / "external" / "hakozuna" / "mixed-ws" / "build"
SMOKE_TOOL = Path(__file__).resolve().with_name("provider_package_ldpreload_replacement_smoke.py")
OPS_RE = re.compile(r"ops/s=([0-9]+(?:\.[0-9]+)?)")


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

#define HAKO_REPLACEMENT_SLOT_SIZE 2048u
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
#define HAKO_REPLACEMENT_STORAGE _Thread_local
#else
#define HAKO_REPLACEMENT_STORAGE
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

static void add_counter(unsigned long long* counter, unsigned long long delta) {
#ifdef HAKO_REPLACEMENT_FRONT_SKIP_HOT_COUNTERS
  (void)counter;
  (void)delta;
#else
  __sync_fetch_and_add(counter, delta);
#endif
}

#ifdef HAKO_REPLACEMENT_FRONT_LOCKED
static pthread_mutex_t arena_lock = PTHREAD_MUTEX_INITIALIZER;
#endif

static void lock_arena(void) {
#ifdef HAKO_REPLACEMENT_FRONT_LOCKED
  pthread_mutex_lock(&arena_lock);
  add_counter(&lock_enter_count, 1);
#endif
}

static void unlock_arena(void) {
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

static void* direct_alloc(size_t size) {
  if (size == 0 || size > HAKO_REPLACEMENT_SLOT_SIZE) {
    return 0;
  }
  init_slots();
#ifdef HAKO_REPLACEMENT_FRONT_THREAD_LOCAL
  if (!register_thread_arena()) {
    return 0;
  }
  drain_remote_frees();
#endif
  if (free_top == 0u) {
    return 0;
  }
  uint32_t index = free_stack[--free_top];
  used[index] = 1u;
  requested_size[index] = size;
  add_counter(&direct_core_call_count, 1);
  return slots[index].bytes;
}

static int direct_free(void* ptr) {
  int index = slot_index(ptr);
  if (index < 0 || used[(uint32_t)index] != 1u) {
#ifdef HAKO_REPLACEMENT_FRONT_THREAD_LOCAL
    return remote_free_to_owner(ptr);
#else
    return 0;
#endif
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
  void* next = direct_alloc(size);
  if (!next) {
    unlock_arena();
    return 0;
  }
  size_t copy_size = old_size < size ? old_size : size;
  memcpy(next, ptr, copy_size);
  add_counter(&realloc_copy_bytes, copy_size);
  direct_free(ptr);
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
  int freed = direct_free(ptr);
  unlock_arena();
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


def counter_value(counters: dict[str, str], key: str) -> int:
    value = counters.get(key, "0")
    return int(value) if value.isdigit() else 0


def build_replacement_front_shim(
    out_dir: Path,
    *,
    locked: bool,
    thread_local: bool,
    skip_hot_counters: bool,
) -> Path:
    front_dir = out_dir / (
        "replacement-front-native-slot-locked" if locked else "replacement-front-native-slot"
    )
    if thread_local:
        front_dir = out_dir / "replacement-front-native-slot-thread-local"
    if skip_hot_counters:
        front_dir = out_dir / f"{front_dir.name}-skip-hot-counters"
    front_dir.mkdir(parents=True, exist_ok=True)
    source = front_dir / "hako_alloc_replacement_front_native_slot.c"
    binary = front_dir / "libhako_alloc_replacement_front_native_slot.so"
    source.write_text(REPLACEMENT_FRONT_SHIM_C.lstrip(), encoding="utf-8")
    cmd = [
        "cc",
        "-shared",
        "-fPIC",
        "-O3",
        "-Wall",
        "-Wextra",
    ]
    if locked:
        cmd.append("-DHAKO_REPLACEMENT_FRONT_LOCKED=1")
    if thread_local:
        cmd.append("-DHAKO_REPLACEMENT_FRONT_THREAD_LOCAL=1")
    if skip_hot_counters:
        cmd.append("-DHAKO_REPLACEMENT_FRONT_SKIP_HOT_COUNTERS=1")
    cmd.extend([str(source), "-ldl"])
    if locked or thread_local:
        cmd.append("-pthread")
    cmd.extend(["-o", str(binary)])
    subprocess.run(cmd, check=True)
    return binary


def build_c_smoke(out_dir: Path, *, name: str, source_text: str) -> Path:
    smoke_dir = out_dir / "replacement-front-cross-thread-smoke"
    smoke_dir.mkdir(parents=True, exist_ok=True)
    source = smoke_dir / f"{name}.c"
    binary = smoke_dir / f"{name}.bin"
    source.write_text(source_text.lstrip(), encoding="utf-8")
    subprocess.run(
        [
            "cc",
            "-O2",
            "-Wall",
            "-Wextra",
            str(source),
            "-pthread",
            "-o",
            str(binary),
        ],
        check=True,
    )
    return binary


def run_replacement_front_focused_smoke(
    *,
    out_dir: Path,
    replacement_front_shim: Path,
    name: str,
    source_text: str,
) -> dict[str, str]:
    binary = build_c_smoke(out_dir, name=name, source_text=source_text)
    smoke_dir = out_dir / "replacement-front-cross-thread-smoke" / name
    smoke_dir.mkdir(parents=True, exist_ok=True)
    stdout_path = smoke_dir / "stdout.txt"
    stderr_path = smoke_dir / "stderr.txt"
    report_path = smoke_dir / "replacement-front-report.out"
    env = os.environ.copy()
    env["LD_PRELOAD"] = str(replacement_front_shim)
    env["HAKORUNE_REPLACEMENT_FRONT_REPORT"] = str(report_path)
    completed = subprocess.run(
        [str(binary)],
        cwd=out_dir,
        env=env,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    stdout_path.write_text(completed.stdout, encoding="utf-8")
    stderr_path.write_text(completed.stderr, encoding="utf-8")
    if completed.returncode != 0:
        raise SystemExit(
            f"replacement front focused smoke {name} failed with "
            f"{completed.returncode}: {completed.stderr.strip()}"
        )
    if not report_path.exists():
        raise SystemExit(f"replacement front focused smoke {name} did not write a report")
    return read_kv(report_path)


def run_replacement_front_cross_thread_smokes(
    *,
    out_dir: Path,
    replacement_front_shim: Path,
) -> dict[str, dict[str, str]]:
    cross_thread_free = run_replacement_front_focused_smoke(
        out_dir=out_dir,
        replacement_front_shim=replacement_front_shim,
        name="cross_thread_free",
        source_text=REPLACEMENT_FRONT_CROSS_THREAD_FREE_SMOKE_C,
    )
    if counter_value(cross_thread_free, "replacement_front_remote_free_push_count") < 1:
        raise SystemExit("cross_thread_free smoke did not push a remote free")
    if counter_value(cross_thread_free, "replacement_front_remote_free_drain_count") < 1:
        raise SystemExit("cross_thread_free smoke did not drain a remote free")
    if counter_value(cross_thread_free, "replacement_front_arena_registry_overflow_count") != 0:
        raise SystemExit("cross_thread_free smoke overflowed the arena registry")

    abandoned_owner = run_replacement_front_focused_smoke(
        out_dir=out_dir,
        replacement_front_shim=replacement_front_shim,
        name="abandoned_owner",
        source_text=REPLACEMENT_FRONT_ABANDONED_OWNER_SMOKE_C,
    )
    if counter_value(abandoned_owner, "replacement_front_abandoned_arena_count") < 1:
        raise SystemExit("abandoned_owner smoke did not mark an abandoned arena")
    if counter_value(abandoned_owner, "replacement_front_abandoned_remote_free_count") < 1:
        raise SystemExit("abandoned_owner smoke did not count abandoned remote free")
    if counter_value(abandoned_owner, "replacement_front_host_passthrough_count") != 0:
        raise SystemExit("abandoned_owner smoke passed a recognized pointer to host free")

    cross_thread_realloc = run_replacement_front_focused_smoke(
        out_dir=out_dir,
        replacement_front_shim=replacement_front_shim,
        name="cross_thread_realloc",
        source_text=REPLACEMENT_FRONT_CROSS_THREAD_REALLOC_SMOKE_C,
    )
    if (
        counter_value(
            cross_thread_realloc,
            "replacement_front_cross_thread_realloc_unsupported_count",
        )
        < 1
    ):
        raise SystemExit("cross_thread_realloc smoke did not count unsupported realloc")
    if counter_value(cross_thread_realloc, "replacement_front_host_passthrough_count") != 0:
        raise SystemExit("cross_thread_realloc smoke used host realloc/free fallback")

    return {
        "cross_thread_free": cross_thread_free,
        "abandoned_owner": abandoned_owner,
        "cross_thread_realloc": cross_thread_realloc,
    }


def find_mimalloc_library(c_library: Path | None, allow_ldconfig_discovery: bool) -> Path:
    if c_library is not None:
        path = c_library.resolve()
    else:
        if not allow_ldconfig_discovery:
            raise SystemExit("--mimalloc-library PATH or --allow-ldconfig-discovery is required")
        completed = subprocess.run(
            [
                "bash",
                "-lc",
                r"ldconfig -p 2>/dev/null | awk '/libmimalloc\.so\.2[[:space:]]/ { print $NF; exit }'",
            ],
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        path = Path(completed.stdout.strip()).resolve() if completed.stdout.strip() else Path("")
    if not path.is_file():
        raise SystemExit(f"libmimalloc.so.2 not found: {path}")
    return path


def run_one(
    *,
    bench: Path,
    root: Path,
    out_dir: Path,
    subject: str,
    run_index: int,
    kind: str,
    threads: int,
    iters_per_thread: int,
    working_set: int,
    min_size: int,
    max_size: int,
    ld_preload: Path | None,
    provider_binary: Path | None,
    provider_usable_size_mode: bool,
    provider_assume_owned_mode: bool,
    replacement_front_mode: bool,
) -> tuple[float, dict[str, str], int]:
    run_dir = out_dir / subject / f"{kind}_{run_index}"
    run_dir.mkdir(parents=True, exist_ok=True)
    stdout_path = run_dir / "bench.stdout"
    stderr_path = run_dir / "bench.stderr"
    counts_path = run_dir / "shim-counts.out"
    env = os.environ.copy()
    if ld_preload is not None:
        env["LD_PRELOAD"] = str(ld_preload)
    if provider_binary is not None:
        env["HAKORUNE_PROVIDER_LIBRARY"] = str(provider_binary)
        env["HAKORUNE_PROVIDER_LDPRELOAD_REPORT"] = str(counts_path)
        if provider_usable_size_mode:
            env["HAKORUNE_PROVIDER_LDPRELOAD_USE_USABLE_SIZE"] = "1"
        if provider_assume_owned_mode:
            env["HAKORUNE_PROVIDER_LDPRELOAD_ASSUME_PROVIDER_OWNED"] = "1"
    if replacement_front_mode:
        env["HAKORUNE_REPLACEMENT_FRONT_REPORT"] = str(counts_path)
    completed = subprocess.run(
        [
            str(bench),
            str(threads),
            str(iters_per_thread),
            str(working_set),
            str(min_size),
            str(max_size),
        ],
        cwd=root,
        env=env,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    stdout_path.write_text(completed.stdout, encoding="utf-8")
    stderr_path.write_text(completed.stderr, encoding="utf-8")
    if completed.returncode != 0:
        raise SystemExit(
            f"{subject} {kind} run {run_index} failed with "
            f"{completed.returncode}: {completed.stderr.strip()}"
        )
    match = OPS_RE.search(completed.stdout)
    if match is None:
        raise SystemExit(f"{subject} {kind} run {run_index} output missing ops/s line")
    counts = read_kv(counts_path) if counts_path.exists() else {}
    return float(match.group(1)), counts, completed.returncode


def run_subject(
    *,
    bench: Path,
    root: Path,
    out_dir: Path,
    subject: str,
    warmup_count: int,
    sample_count: int,
    threads: int,
    iters_per_thread: int,
    working_set: int,
    min_size: int,
    max_size: int,
    ld_preload: Path | None,
    provider_binary: Path | None,
    provider_usable_size_mode: bool,
    provider_assume_owned_mode: bool,
    replacement_front_mode: bool,
) -> tuple[list[float], dict[str, int]]:
    sample_throughputs: list[float] = []
    counter_totals: dict[str, int] = {}
    total_runs = warmup_count + sample_count
    for run_index in range(total_runs):
        kind = "warmup" if run_index < warmup_count else "sample"
        throughput, counts, _exit_code = run_one(
            bench=bench,
            root=root,
            out_dir=out_dir,
            subject=subject,
            run_index=run_index,
            kind=kind,
            threads=threads,
            iters_per_thread=iters_per_thread,
            working_set=working_set,
            min_size=min_size,
            max_size=max_size,
            ld_preload=ld_preload,
            provider_binary=provider_binary,
            provider_usable_size_mode=provider_usable_size_mode,
            provider_assume_owned_mode=provider_assume_owned_mode,
            replacement_front_mode=replacement_front_mode,
        )
        for key, value in counts.items():
            if value.isdigit():
                counter_totals[key] = counter_totals.get(key, 0) + int(value)
        if kind == "sample":
            sample_throughputs.append(throughput)
    return sample_throughputs, counter_totals


def format_ratio(value: float, base: float) -> str:
    if base <= 0:
        return "0.000000"
    return f"{value / base:.6f}"


def load_manifest_metadata(path: Path | None) -> dict[str, str]:
    if path is None:
        return {}
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid provider manifest JSON: {path}: {exc}") from exc
    if not isinstance(data, dict):
        raise SystemExit(f"provider manifest root must be an object: {path}")

    build = data.get("build")
    activation = data.get("activation")
    if not isinstance(build, dict):
        build = {}
    if not isinstance(activation, dict):
        activation = {}

    def manifest_string(source: dict[str, Any], key: str, default: str = "unknown") -> str:
        value = source.get(key)
        if isinstance(value, bool):
            return "1" if value else "0"
        if value is None:
            return default
        return str(value)

    return {
        "provider_manifest_provider_name": manifest_string(data, "provider_name"),
        "provider_manifest_provider_kind": manifest_string(data, "provider_kind"),
        "provider_manifest_profile": manifest_string(data, "profile"),
        "provider_manifest_build_mode": manifest_string(build, "mode"),
        "provider_manifest_hako_semantic_provider_codegen": manifest_string(
            build, "hako_semantic_provider_codegen", "none"
        ),
        "provider_manifest_hako_provider_object_lifecycle_entrypoint_verified": manifest_string(
            build, "hako_provider_object_lifecycle_entrypoint_verified", "0"
        ),
        "provider_manifest_hako_provider_alloc_free_route": manifest_string(
            build, "hako_provider_alloc_free_route", "unknown"
        ),
        "provider_manifest_hako_provider_alloc_free_uses_host_malloc": manifest_string(
            build, "hako_provider_alloc_free_uses_host_malloc", "unknown"
        ),
        "provider_manifest_hako_provider_alloc_free_uses_hako_object_lifecycle": manifest_string(
            build, "hako_provider_alloc_free_uses_hako_object_lifecycle", "unknown"
        ),
        "provider_manifest_hako_provider_object_lifecycle_entrypoint_usage": manifest_string(
            build, "hako_provider_object_lifecycle_entrypoint_usage", "unknown"
        ),
        "provider_manifest_allocator_replacement_allowed": manifest_string(
            activation, "allocator_replacement_allowed", "0"
        ),
        "provider_manifest_hook_allowed": manifest_string(activation, "hook_allowed", "0"),
        "provider_manifest_global_allocator_allowed": manifest_string(
            activation, "global_allocator_allowed", "0"
        ),
    }


def format_per_operation(numerator: int, denominator: int) -> str:
    if denominator <= 0:
        return "0.000000"
    return f"{numerator / denominator:.6f}"


def init_fallback_dominates_provider_ops(counters: dict[str, int], provider_ops: int) -> bool:
    if provider_ops <= 0:
        return False
    init_fallback = counters.get("shim_init_real_fallback_count", 0)
    return init_fallback * 2 >= provider_ops


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--hakozuna-root", type=Path, default=DEFAULT_HAKOZUNA_ROOT)
    parser.add_argument("--mimalloc-library", type=Path)
    parser.add_argument("--allow-ldconfig-discovery", action="store_true")
    parser.add_argument("--manifest", type=Path)
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--sample-count", type=int, default=3)
    parser.add_argument("--warmup-count", type=int, default=1)
    parser.add_argument("--threads", type=int, default=1)
    parser.add_argument("--iters-per-thread", type=int, default=1000)
    parser.add_argument("--working-set", type=int, default=128)
    parser.add_argument("--min-size", type=int, default=16)
    parser.add_argument("--max-size", type=int, default=1024)
    parser.add_argument(
        "--provider-usable-size-mode",
        action="store_true",
        help="measurement-only: bypass provider shim tracking through private usable_size symbol",
    )
    parser.add_argument(
        "--provider-assume-owned-mode",
        action="store_true",
        help="measurement-only: with usable-size mode, skip provider owns checks before free/realloc",
    )
    parser.add_argument(
        "--replacement-front-native-slot-mode",
        action="store_true",
        help="benchmark-only: add a thin native-slot malloc/free replacement front subject",
    )
    parser.add_argument(
        "--replacement-front-lock-mode",
        action="store_true",
        help="benchmark-only: build the replacement front with a global arena mutex",
    )
    parser.add_argument(
        "--replacement-front-thread-local-mode",
        action="store_true",
        help="benchmark-only: build the replacement front with same-thread TLS arenas",
    )
    parser.add_argument(
        "--replacement-front-cross-thread-smoke",
        action="store_true",
        help="run focused cross-thread free and abandoned-owner replacement front smokes",
    )
    parser.add_argument(
        "--replacement-front-skip-hot-counters",
        action="store_true",
        help="measurement-only: skip malloc/free hot-path replacement front counters",
    )
    args = parser.parse_args()

    positive_int(args.sample_count, "--sample-count")
    if args.warmup_count < 0:
        raise SystemExit("--warmup-count must be non-negative")
    positive_int(args.threads, "--threads")
    positive_int(args.iters_per_thread, "--iters-per-thread")
    positive_int(args.working_set, "--working-set")
    positive_int(args.min_size, "--min-size")
    positive_int(args.max_size, "--max-size")
    if args.max_size < args.min_size:
        raise SystemExit("--max-size must be >= --min-size")
    if args.provider_assume_owned_mode and not args.provider_usable_size_mode:
        raise SystemExit("--provider-assume-owned-mode requires --provider-usable-size-mode")
    if args.replacement_front_lock_mode and not args.replacement_front_native_slot_mode:
        raise SystemExit(
            "--replacement-front-lock-mode requires --replacement-front-native-slot-mode"
        )
    if args.replacement_front_thread_local_mode and not args.replacement_front_native_slot_mode:
        raise SystemExit(
            "--replacement-front-thread-local-mode requires --replacement-front-native-slot-mode"
        )
    if args.replacement_front_lock_mode and args.replacement_front_thread_local_mode:
        raise SystemExit(
            "--replacement-front-lock-mode and --replacement-front-thread-local-mode are exclusive"
        )
    if args.replacement_front_cross_thread_smoke and not args.replacement_front_thread_local_mode:
        raise SystemExit(
            "--replacement-front-cross-thread-smoke requires "
            "--replacement-front-thread-local-mode"
        )
    if args.replacement_front_skip_hot_counters and not args.replacement_front_native_slot_mode:
        raise SystemExit(
            "--replacement-front-skip-hot-counters requires "
            "--replacement-front-native-slot-mode"
        )
    if args.replacement_front_cross_thread_smoke and args.replacement_front_skip_hot_counters:
        raise SystemExit(
            "--replacement-front-cross-thread-smoke cannot be combined with "
            "--replacement-front-skip-hot-counters because the smoke validates counters"
        )

    root = args.hakozuna_root.resolve()
    bench = root / "bench_mixed_ws_crt"
    if not bench.is_file() or not os.access(bench, os.X_OK):
        raise SystemExit(
            "missing executable hakozuna mixed-ws bench: "
            f"{bench}\n"
            "hint: run `make -C benchmarks/external/hakozuna/mixed-ws` "
            "or pass --hakozuna-root for an external hakozuna build"
        )

    out_dir = args.out_dir.resolve()
    out_dir.mkdir(parents=True, exist_ok=True)
    mimalloc_library = find_mimalloc_library(args.mimalloc_library, args.allow_ldconfig_discovery)
    provider_manifest_metadata = load_manifest_metadata(
        args.manifest.resolve() if args.manifest else None
    )

    provider_shim: Path | None = None
    provider_binary: Path | None = None
    replacement_front_shim: Path | None = None
    if args.manifest is not None:
        smoke_report = out_dir / "provider-ldpreload-smoke.out"
        smoke_cmd = [
            sys.executable,
            str(SMOKE_TOOL),
            "--manifest",
            str(args.manifest.resolve()),
            "--out-dir",
            str(out_dir / "provider-ldpreload-smoke"),
            "--out",
            str(smoke_report),
        ]
        if args.provider_usable_size_mode:
            smoke_cmd.append("--use-provider-usable-size")
        if args.provider_assume_owned_mode:
            smoke_cmd.append("--assume-provider-owned")
        subprocess.run(smoke_cmd, check=True)
        smoke = read_kv(smoke_report)
        provider_shim = Path(smoke["shim_artifact_path"])
        provider_binary = Path(smoke["provider_binary_path"])
    if args.replacement_front_native_slot_mode:
        replacement_front_shim = build_replacement_front_shim(
            out_dir,
            locked=args.replacement_front_lock_mode,
            thread_local=args.replacement_front_thread_local_mode,
            skip_hot_counters=args.replacement_front_skip_hot_counters,
        )
    replacement_front_smokes: dict[str, dict[str, str]] = {}
    if args.replacement_front_cross_thread_smoke:
        if replacement_front_shim is None:
            raise SystemExit("--replacement-front-cross-thread-smoke requires a replacement front")
        replacement_front_smokes = run_replacement_front_cross_thread_smokes(
            out_dir=out_dir,
            replacement_front_shim=replacement_front_shim,
        )

    subject_specs: list[tuple[str, Path | None, Path | None, bool]] = [
        ("system_malloc", None, None, False),
        ("c_mimalloc_ldpreload", mimalloc_library, None, False),
    ]
    if provider_shim is not None and provider_binary is not None:
        subject_specs.append(("hakorune_provider_ldpreload", provider_shim, provider_binary, False))
    if replacement_front_shim is not None:
        subject_specs.append(
            ("hakorune_replacement_front_ldpreload", replacement_front_shim, None, True)
        )

    reports: dict[str, tuple[list[float], dict[str, int]]] = {}
    for subject, ld_preload, provider, replacement_front_mode in subject_specs:
        reports[subject] = run_subject(
            bench=bench,
            root=root,
            out_dir=out_dir,
            subject=subject,
            warmup_count=args.warmup_count,
            sample_count=args.sample_count,
            threads=args.threads,
            iters_per_thread=args.iters_per_thread,
            working_set=args.working_set,
            min_size=args.min_size,
            max_size=args.max_size,
            ld_preload=ld_preload,
            provider_binary=provider,
            provider_usable_size_mode=(
                args.provider_usable_size_mode and subject == "hakorune_provider_ldpreload"
            ),
            provider_assume_owned_mode=(
                args.provider_assume_owned_mode and subject == "hakorune_provider_ldpreload"
            ),
            replacement_front_mode=replacement_front_mode,
        )

    c_mimalloc_median = median_float(reports["c_mimalloc_ldpreload"][0])
    lines = [
        "output_contract=hakozuna-mixed-ws-ldpreload-compare-v0",
        "benchmark_id=bench_mixed_ws_crt",
        f"benchmark_path={bench}",
        f"hakozuna_root={root}",
        f"mimalloc_library={mimalloc_library}",
        f"provider_manifest={args.manifest.resolve() if args.manifest is not None else 'none'}",
        f"sample_count={args.sample_count}",
        f"warmup_count={args.warmup_count}",
        f"benchmark_threads={args.threads}",
        f"benchmark_iters_per_thread={args.iters_per_thread}",
        f"benchmark_working_set={args.working_set}",
        f"benchmark_min_size={args.min_size}",
        f"benchmark_max_size={args.max_size}",
        f"subject_count={len(subject_specs)}",
        "reference_subject=c_mimalloc_ldpreload",
        "provider_activation=0",
        "production_replacement_active=0",
        "hook_installed=0",
        "global_allocator_product_claim=0",
        "winner_claim=0",
        f"provider_usable_size_mode={1 if args.provider_usable_size_mode else 0}",
        f"provider_assume_owned_mode={1 if args.provider_assume_owned_mode else 0}",
        f"replacement_front_native_slot_mode={1 if args.replacement_front_native_slot_mode else 0}",
        f"replacement_front_lock_mode={1 if args.replacement_front_lock_mode else 0}",
        f"replacement_front_thread_local_mode={1 if args.replacement_front_thread_local_mode else 0}",
        f"replacement_front_cross_thread_smoke={1 if args.replacement_front_cross_thread_smoke else 0}",
        f"replacement_front_skip_hot_counters={1 if args.replacement_front_skip_hot_counters else 0}",
    ]
    if replacement_front_smokes:
        cross_thread_free = replacement_front_smokes["cross_thread_free"]
        abandoned_owner = replacement_front_smokes["abandoned_owner"]
        cross_thread_realloc = replacement_front_smokes["cross_thread_realloc"]
        lines.extend(
            [
                "replacement_front_cross_thread_free_smoke_ok=1",
                "replacement_front_abandoned_owner_smoke_ok=1",
                "replacement_front_cross_thread_realloc_smoke_ok=1",
                "replacement_front_cross_thread_free_policy=remote_queue",
                "replacement_front_abandoned_owner_policy=mark_abandoned_no_host_free",
                "replacement_front_cross_thread_realloc_policy=unsupported_counted",
                "replacement_front_cross_thread_free_remote_free_push_count="
                f"{counter_value(cross_thread_free, 'replacement_front_remote_free_push_count')}",
                "replacement_front_cross_thread_free_remote_free_drain_count="
                f"{counter_value(cross_thread_free, 'replacement_front_remote_free_drain_count')}",
                "replacement_front_cross_thread_free_arena_registry_overflow_count="
                f"{counter_value(cross_thread_free, 'replacement_front_arena_registry_overflow_count')}",
                "replacement_front_abandoned_owner_abandoned_arena_count="
                f"{counter_value(abandoned_owner, 'replacement_front_abandoned_arena_count')}",
                "replacement_front_abandoned_owner_abandoned_remote_free_count="
                f"{counter_value(abandoned_owner, 'replacement_front_abandoned_remote_free_count')}",
                "replacement_front_abandoned_owner_host_passthrough_count="
                f"{counter_value(abandoned_owner, 'replacement_front_host_passthrough_count')}",
                "replacement_front_cross_thread_realloc_unsupported_count="
                f"{counter_value(cross_thread_realloc, 'replacement_front_cross_thread_realloc_unsupported_count')}",
                "replacement_front_cross_thread_realloc_host_passthrough_count="
                f"{counter_value(cross_thread_realloc, 'replacement_front_host_passthrough_count')}",
            ]
        )
    for key in sorted(provider_manifest_metadata):
        lines.append(f"{key}={provider_manifest_metadata[key]}")
    if args.manifest is not None:
        lines.extend(
            [
                "provider_ldpreload_measurement_interpretation=provider_abi_wrapper_and_shim_bridge",
                "provider_ldpreload_is_product_allocator_claim=0",
                "provider_ldpreload_is_hako_core_speed_claim=0",
            ]
        )
    for index, (subject, _ld_preload, _provider, replacement_front_mode) in enumerate(subject_specs):
        samples, counters = reports[subject]
        median = median_float(samples)
        lines.extend(
            [
                f"subject_{index}_id={subject}",
                f"subject_{index}_throughput_min_ops_per_sec={min(samples):.3f}",
                f"subject_{index}_throughput_median_ops_per_sec={median:.3f}",
                f"subject_{index}_throughput_max_ops_per_sec={max(samples):.3f}",
                f"subject_{index}_throughput_vs_c_mimalloc={format_ratio(median, c_mimalloc_median)}",
                f"subject_{index}_winner_claim=0",
            ]
        )
        if replacement_front_mode:
            single_thread_smoke = args.threads == 1
            thread_local_smoke = args.threads > 1 and args.replacement_front_thread_local_mode
            multithread_smoke = args.threads > 1 and (
                args.replacement_front_lock_mode or args.replacement_front_thread_local_mode
            )
            lines.extend(
                [
                    f"subject_{index}_provider_table_dispatch=0",
                    f"subject_{index}_function_pointer_hot_call=0",
                    f"subject_{index}_owns_check_hot_path=0",
                    f"subject_{index}_tracking_hot_path=0",
                    f"subject_{index}_direct_core_call=1",
                    f"subject_{index}_single_thread_replacement_front_smoke={1 if single_thread_smoke else 0}",
                    f"subject_{index}_multithread_replacement_front_smoke={1 if multithread_smoke else 0}",
                    f"subject_{index}_thread_local_replacement_front_smoke={1 if thread_local_smoke else 0}",
                    f"subject_{index}_thread_safety_claim={'measured' if (multithread_smoke or thread_local_smoke) else 'none'}",
                    f"subject_{index}_thread_local_arena={1 if args.replacement_front_thread_local_mode else 0}",
                    "subject_"
                    f"{index}_cross_thread_free_policy="
                    f"{'remote_queue' if args.replacement_front_thread_local_mode else 'global_lock_or_not_applicable'}",
                    f"subject_{index}_provider_api_hot_path_required=0",
                    f"subject_{index}_activation=0",
                    f"subject_{index}_benchmark_only=1",
                ]
            )
        if counters:
            for key in sorted(counters):
                lines.append(f"subject_{index}_{key}_total={counters[key]}")
            provider_ops = (
                counters.get("shim_provider_alloc_count", 0)
                + counters.get("shim_provider_calloc_count", 0)
                + counters.get("shim_provider_realloc_count", 0)
                + counters.get("shim_provider_free_count", 0)
            )
            init_fallback_dominates = init_fallback_dominates_provider_ops(counters, provider_ops)
            lines.extend(
                [
                    f"subject_{index}_shim_provider_operation_count_total={provider_ops}",
                    "subject_"
                    f"{index}_shim_init_real_fallback_per_provider_operation="
                    f"{format_per_operation(counters.get('shim_init_real_fallback_count', 0), provider_ops)}",
                    "subject_"
                    f"{index}_shim_host_passthrough_per_provider_operation="
                    f"{format_per_operation(counters.get('shim_host_passthrough_count', 0), provider_ops)}",
                    "subject_"
                    f"{index}_shim_init_real_fallback_dominates_provider_ops="
                    f"{1 if init_fallback_dominates else 0}",
                ]
            )
            if init_fallback_dominates:
                lines.extend(
                    [
                        "subject_"
                        f"{index}_next_owner_family=provider_alloc_free_internal_real_malloc_boundary",
                        "subject_"
                        f"{index}_gap_classification=provider_bridge_not_hako_core_speed",
                    ]
                )
            lines.append(f"subject_{index}_shim_init_real_fallback_is_perf_diagnostic=1")
    lines.append("summary=ok")
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
