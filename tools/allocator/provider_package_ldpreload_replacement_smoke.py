#!/usr/bin/env python3
"""Smoke a provider-backed LD_PRELOAD malloc-family replacement pilot."""

from __future__ import annotations

import argparse
import os
import subprocess
from pathlib import Path

from provider_package_api_bind_smoke import run
from provider_package_load_only_smoke import sha256_file


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
typedef size_t (*hako_provider_usable_size_fn)(void*);
typedef void* (*hako_malloc_fn)(size_t);
typedef void* (*hako_calloc_fn)(size_t, size_t);
typedef void* (*hako_realloc_fn)(void*, size_t);
typedef void (*hako_free_fn)(void*);

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
static hako_provider_usable_size_fn provider_usable_size_fn = 0;
static int resolving_real = 0;
static int loading_provider = 0;
static int provider_load_attempted = 0;
static int provider_ready = 0;
static int provider_usable_size_mode = 0;
static int provider_assume_owned_mode = 0;
static int in_provider_call = 0;
static struct HakoTrackedPtr tracked[HAKO_POINTER_TABLE_CAP];

static unsigned long long provider_alloc_count = 0;
static unsigned long long provider_calloc_count = 0;
static unsigned long long provider_realloc_count = 0;
static unsigned long long provider_free_count = 0;
static unsigned long long provider_free_claim_count = 0;
static unsigned long long provider_free_not_owned_count = 0;
static unsigned long long provider_free_claim_bound = 0;
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
static unsigned long long assume_owned_mode_enabled = 0;
static unsigned long long assume_owned_free_count = 0;
static unsigned long long assume_owned_realloc_count = 0;

static void hako_count_init_fallback(void) {
  init_real_fallback_count++;
  if (loading_provider) {
    init_fallback_loading_provider_count++;
  }
  if (resolving_real) {
    init_fallback_resolving_real_count++;
  }
  if (in_provider_call) {
    init_fallback_in_provider_call_count++;
  }
}

static void hako_resolve_real(void) {
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

static unsigned int hako_ptr_hash(void* ptr) {
  return (unsigned int)(((uintptr_t)ptr >> 4u) % HAKO_POINTER_TABLE_CAP);
}

static int hako_find_tracked(void* ptr) {
  if (!ptr) {
    return -1;
  }
  unsigned int hash = hako_ptr_hash(ptr);
  for (unsigned int probe = 0; probe < HAKO_POINTER_TABLE_CAP; probe++) {
    unsigned int index = (unsigned int)((hash + probe) % HAKO_POINTER_TABLE_CAP);
    unsigned int probe_count = probe + 1u;
    if (tracked[index].ptr == ptr) {
      find_probe_total += probe_count;
      if (probe_count > find_probe_max) {
        find_probe_max = probe_count;
      }
      tracked_hit_count++;
      return (int)index;
    }
    if (!tracked[index].ptr) {
      find_probe_total += probe_count;
      if (probe_count > find_probe_max) {
        find_probe_max = probe_count;
      }
      tracked_miss_count++;
      return -1;
    }
  }
  find_probe_total += HAKO_POINTER_TABLE_CAP;
  if (HAKO_POINTER_TABLE_CAP > find_probe_max) {
    find_probe_max = HAKO_POINTER_TABLE_CAP;
  }
  tracked_miss_count++;
  return -1;
}

static void hako_track_ptr(void* ptr, size_t size) {
  if (!ptr) {
    return;
  }
  unsigned int hash = hako_ptr_hash(ptr);
  for (unsigned int probe = 0; probe < HAKO_POINTER_TABLE_CAP; probe++) {
    unsigned int index = (unsigned int)((hash + probe) % HAKO_POINTER_TABLE_CAP);
    unsigned int probe_count = probe + 1u;
    if (tracked[index].ptr == ptr) {
      track_probe_total += probe_count;
      if (probe_count > track_probe_max) {
        track_probe_max = probe_count;
      }
      tracked[index].size = size;
      return;
    }
    if (!tracked[index].ptr) {
      track_probe_total += probe_count;
      if (probe_count > track_probe_max) {
        track_probe_max = probe_count;
      }
      tracked[index].ptr = ptr;
      tracked[index].size = size;
      return;
    }
  }
  track_probe_total += HAKO_POINTER_TABLE_CAP;
  if (HAKO_POINTER_TABLE_CAP > track_probe_max) {
    track_probe_max = HAKO_POINTER_TABLE_CAP;
  }
  pointer_table_overflow++;
}

static void hako_untrack_index(int index) {
  if (index < 0) {
    return;
  }
  unsigned int hole = (unsigned int)index;
  for (;;) {
    unsigned int next = (unsigned int)((hole + 1u) % HAKO_POINTER_TABLE_CAP);
    if (!tracked[next].ptr) {
      tracked[hole].ptr = 0;
      tracked[hole].size = 0;
      return;
    }
    unsigned int home = hako_ptr_hash(tracked[next].ptr);
    unsigned int distance = (unsigned int)((next + HAKO_POINTER_TABLE_CAP - home) % HAKO_POINTER_TABLE_CAP);
    if (distance == 0) {
      tracked[hole].ptr = 0;
      tracked[hole].size = 0;
      return;
    }
    tracked[hole] = tracked[next];
    hole = next;
  }
}

static void hako_write_str(int fd, const char* s) {
  size_t len = 0;
  while (s[len]) {
    len++;
  }
  ssize_t ignored = write(fd, s, len);
  (void)ignored;
}

static void hako_write_u64(int fd, unsigned long long value) {
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

static void hako_write_kv(int fd, const char* key, unsigned long long value) {
  hako_write_str(fd, key);
  hako_write_str(fd, "=");
  hako_write_u64(fd, value);
}

static void hako_write_report(void) {
  const char* path = getenv("HAKORUNE_PROVIDER_LDPRELOAD_REPORT");
  if (!path || !path[0]) {
    return;
  }
  int fd = open(path, O_CREAT | O_TRUNC | O_WRONLY, 0644);
  if (fd < 0) {
    return;
  }
  hako_write_kv(fd, "shim_provider_bind_success", provider_bind_success);
  hako_write_kv(fd, "shim_provider_bind_failure", provider_bind_failure);
  hako_write_kv(fd, "shim_provider_alloc_count", provider_alloc_count);
  hako_write_kv(fd, "shim_provider_calloc_count", provider_calloc_count);
  hako_write_kv(fd, "shim_provider_realloc_count", provider_realloc_count);
  hako_write_kv(fd, "shim_provider_free_count", provider_free_count);
  hako_write_kv(fd, "shim_provider_free_claim_count", provider_free_claim_count);
  hako_write_kv(fd, "shim_provider_free_not_owned_count", provider_free_not_owned_count);
  hako_write_kv(fd, "shim_provider_free_claim_bound", provider_free_claim_bound);
  hako_write_kv(fd, "shim_runtime_real_fallback_count", runtime_real_fallback_count);
  hako_write_kv(fd, "shim_init_real_fallback_count", init_real_fallback_count);
  hako_write_kv(fd, "shim_init_fallback_loading_provider_count", init_fallback_loading_provider_count);
  hako_write_kv(fd, "shim_init_fallback_resolving_real_count", init_fallback_resolving_real_count);
  hako_write_kv(fd, "shim_init_fallback_in_provider_call_count", init_fallback_in_provider_call_count);
  hako_write_kv(fd, "shim_malloc_init_fallback_count", malloc_init_fallback_count);
  hako_write_kv(fd, "shim_calloc_init_fallback_count", calloc_init_fallback_count);
  hako_write_kv(fd, "shim_realloc_init_fallback_count", realloc_init_fallback_count);
  hako_write_kv(fd, "shim_free_init_fallback_count", free_init_fallback_count);
  hako_write_kv(fd, "shim_host_passthrough_count", host_passthrough_count);
  hako_write_kv(fd, "shim_pointer_table_overflow", pointer_table_overflow);
  hako_write_kv(fd, "shim_track_probe_total", track_probe_total);
  hako_write_kv(fd, "shim_track_probe_max", track_probe_max);
  hako_write_kv(fd, "shim_find_probe_total", find_probe_total);
  hako_write_kv(fd, "shim_find_probe_max", find_probe_max);
  hako_write_kv(fd, "shim_tombstone_hit_count", tombstone_hit_count);
  hako_write_kv(fd, "shim_tracked_hit_count", tracked_hit_count);
  hako_write_kv(fd, "shim_tracked_miss_count", tracked_miss_count);
  hako_write_kv(fd, "shim_calloc_zero_bytes", calloc_zero_bytes);
  hako_write_kv(fd, "shim_realloc_copy_bytes", realloc_copy_bytes);
  hako_write_kv(fd, "shim_realloc_tracked_count", realloc_tracked_count);
  hako_write_kv(fd, "shim_realloc_host_passthrough_count", realloc_host_passthrough_count);
  hako_write_kv(fd, "shim_realloc_null_count", realloc_null_count);
  hako_write_kv(fd, "shim_realloc_free_count", realloc_free_count);
  hako_write_kv(fd, "shim_usable_size_mode_enabled", usable_size_mode_enabled);
  hako_write_kv(fd, "shim_usable_size_symbol_bound", usable_size_symbol_bound);
  hako_write_kv(fd, "shim_usable_size_lookup_count", usable_size_lookup_count);
  hako_write_kv(fd, "shim_usable_size_lookup_failure_count", usable_size_lookup_failure_count);
  hako_write_kv(fd, "shim_tracking_bypassed_count", tracking_bypassed_count);
  hako_write_kv(fd, "shim_assume_owned_mode_enabled", assume_owned_mode_enabled);
  hako_write_kv(fd, "shim_assume_owned_free_count", assume_owned_free_count);
  hako_write_kv(fd, "shim_assume_owned_realloc_count", assume_owned_realloc_count);
  close(fd);
}

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
  if (api->api_table_size >= sizeof(struct HakoProviderApiV1) && api->free_claim) {
    provider_free_claim_fn = api->free_claim;
    provider_free_claim_bound = 1;
  }
  if (provider_usable_size_mode) {
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
  int index = provider_usable_size_mode ? -1 : hako_find_tracked(ptr);
  if (index < 0) {
    if (!provider_usable_size_mode || !provider_owns_fn || !provider_usable_size_fn) {
      realloc_host_passthrough_count++;
      host_passthrough_count++;
      hako_resolve_real();
      return real_realloc_fn ? real_realloc_fn(ptr, size) : 0;
    }
    in_provider_call = 1;
    int owned = provider_assume_owned_mode ? 1 : provider_owns_fn(ptr);
    size_t old_size = owned == 1 ? provider_usable_size_fn(ptr) : 0u;
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


SMOKE_C = r"""
#include <stdlib.h>
#include <string.h>

int main(void) {
  unsigned char* p = (unsigned char*)malloc(32);
  if (!p) return 2;
  memset(p, 0xA5, 32);
  unsigned char* q = (unsigned char*)calloc(4, 8);
  if (!q) return 3;
  for (int i = 0; i < 32; i++) {
    if (q[i] != 0) return 4;
  }
  unsigned char* r = (unsigned char*)realloc(p, 64);
  if (!r) return 5;
  for (int i = 0; i < 32; i++) {
    if (r[i] != 0xA5) return 6;
  }
  free(q);
  free(r);
  return 0;
}
"""


def parse_report(path: Path) -> dict[str, str]:
    fields: dict[str, str] = {}
    if not path.exists():
        return fields
    for line in path.read_text(encoding="utf-8").splitlines():
        if "=" not in line:
            continue
        key, value = line.split("=", 1)
        fields[key] = value
    return fields


def emit_report(
    *,
    manifest_path: Path,
    binary_path: Path,
    shim_source: Path,
    shim_binary: Path,
    smoke_source: Path,
    smoke_binary: Path,
    shim_report: Path,
    smoke_exit_code: int,
    shim_fields: dict[str, str],
) -> str:
    provider_alloc_count = int(shim_fields.get("shim_provider_alloc_count", "0"))
    provider_free_count = int(shim_fields.get("shim_provider_free_count", "0"))
    provider_bind_success = int(shim_fields.get("shim_provider_bind_success", "0"))
    pointer_table_overflow = int(shim_fields.get("shim_pointer_table_overflow", "0"))
    runtime_fallback = int(shim_fields.get("shim_runtime_real_fallback_count", "0"))
    summary = "ok" if (
        smoke_exit_code == 0
        and provider_bind_success == 1
        and provider_alloc_count > 0
        and provider_free_count > 0
        and pointer_table_overflow == 0
        and runtime_fallback == 0
    ) else "failed"
    lines = [
        "output_contract=hako-mimalloc-provider-backed-ldpreload-shim-smoke-v0",
        "input_contract=hakorune-provider-runtime-load-stage-7a-v0",
        "dll_mode=provider-backed-ldpreload-pilot",
        f"manifest={manifest_path}",
        f"provider_binary_path={binary_path}",
        f"provider_binary_sha256={sha256_file(binary_path)}",
        f"shim_source_path={shim_source}",
        f"shim_artifact_path={shim_binary}",
        f"shim_artifact_sha256={sha256_file(shim_binary)}",
        f"smoke_source_path={smoke_source}",
        f"smoke_binary_path={smoke_binary}",
        f"shim_report_path={shim_report}",
        "ld_preload_env_applied=1",
        "provider_library_env_applied=1",
        "shared_library_load_executed=1",
        "required_export_resolved=1",
        "provider_api_bound=1",
        "provider_call_executed=1",
        "allocator_entrypoint_called=1",
        "replacement_active=1",
        "replacement_scope=generated-smoke-process-only",
        "replacement_product_claim=0",
        "hook_installed=0",
        "global_allocator=0",
        "winner_claim=0",
        "thread_safety=single-thread-pilot",
        "usable_size_tracking_bypass_mode=measurement_only",
        f"smoke_exit_code={smoke_exit_code}",
    ]
    for key in sorted(shim_fields):
        lines.append(f"{key}={shim_fields[key]}")
    lines.append(f"summary={summary}")
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    parser.add_argument(
        "--use-provider-usable-size",
        action="store_true",
        help="measurement-only: bypass shim pointer tracking through provider usable_size symbol",
    )
    parser.add_argument(
        "--assume-provider-owned",
        action="store_true",
        help="measurement-only: with usable-size mode, skip provider owns checks before free/realloc",
    )
    args = parser.parse_args()
    if args.assume_provider_owned and not args.use_provider_usable_size:
        raise SystemExit("--assume-provider-owned requires --use-provider-usable-size")

    manifest_path = args.manifest.resolve()
    _fields, _descriptor, _api, binary_path = run(manifest_path)

    out_dir = args.out_dir.resolve()
    out_dir.mkdir(parents=True, exist_ok=True)
    shim_source = out_dir / "hako_provider_ldpreload_replacement_pilot.c"
    shim_binary = out_dir / "libhako_provider_ldpreload_replacement_pilot.so"
    smoke_source = out_dir / "hako_provider_ldpreload_replacement_smoke.c"
    smoke_binary = out_dir / "hako_provider_ldpreload_replacement_smoke"
    shim_report = out_dir / "shim_counts.out"

    shim_source.write_text(SHIM_C.lstrip(), encoding="utf-8")
    smoke_source.write_text(SMOKE_C.lstrip(), encoding="utf-8")
    subprocess.run(
        [
            "cc",
            "-shared",
            "-fPIC",
            "-O2",
            "-Wall",
            "-Wextra",
            str(shim_source),
            "-ldl",
            "-o",
            str(shim_binary),
        ],
        check=True,
    )
    subprocess.run(
        ["cc", "-O2", "-Wall", "-Wextra", str(smoke_source), "-o", str(smoke_binary)],
        check=True,
    )

    env = os.environ.copy()
    env["LD_PRELOAD"] = str(shim_binary)
    env["HAKORUNE_PROVIDER_LIBRARY"] = str(binary_path)
    env["HAKORUNE_PROVIDER_LDPRELOAD_REPORT"] = str(shim_report)
    if args.use_provider_usable_size:
        env["HAKORUNE_PROVIDER_LDPRELOAD_USE_USABLE_SIZE"] = "1"
    if args.assume_provider_owned:
        env["HAKORUNE_PROVIDER_LDPRELOAD_ASSUME_PROVIDER_OWNED"] = "1"
    proc = subprocess.run([str(smoke_binary)], env=env, check=False)
    report = emit_report(
        manifest_path=manifest_path,
        binary_path=binary_path,
        shim_source=shim_source,
        shim_binary=shim_binary,
        smoke_source=smoke_source,
        smoke_binary=smoke_binary,
        shim_report=shim_report,
        smoke_exit_code=proc.returncode,
        shim_fields=parse_report(shim_report),
    )
    if "summary=ok" not in report:
        if args.out is not None:
            args.out.parent.mkdir(parents=True, exist_ok=True)
            args.out.write_text(report, encoding="utf-8")
        print(report, end="")
        return 1
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
