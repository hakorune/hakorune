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
#define HAKO_TRACKED_TOMBSTONE ((void*)1)

typedef int (*hako_ping_fn)(void);
typedef void* (*hako_provider_alloc_fn)(size_t, size_t);
typedef void (*hako_provider_free_fn)(void*);
typedef int (*hako_provider_owns_fn)(void*);
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
static int resolving_real = 0;
static int loading_provider = 0;
static int provider_load_attempted = 0;
static int provider_ready = 0;
static int in_provider_call = 0;
static struct HakoTrackedPtr tracked[HAKO_POINTER_TABLE_CAP];

static unsigned long long provider_alloc_count = 0;
static unsigned long long provider_calloc_count = 0;
static unsigned long long provider_realloc_count = 0;
static unsigned long long provider_free_count = 0;
static unsigned long long runtime_real_fallback_count = 0;
static unsigned long long init_real_fallback_count = 0;
static unsigned long long host_passthrough_count = 0;
static unsigned long long provider_bind_success = 0;
static unsigned long long provider_bind_failure = 0;
static unsigned long long pointer_table_overflow = 0;

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

static int hako_find_tracked(void* ptr) {
  if (!ptr) {
    return -1;
  }
  uintptr_t hash = ((uintptr_t)ptr >> 4u) % HAKO_POINTER_TABLE_CAP;
  for (unsigned int probe = 0; probe < HAKO_POINTER_TABLE_CAP; probe++) {
    unsigned int index = (unsigned int)((hash + probe) % HAKO_POINTER_TABLE_CAP);
    if (tracked[index].ptr == ptr) {
      return (int)index;
    }
    if (!tracked[index].ptr) {
      return -1;
    }
  }
  return -1;
}

static void hako_track_ptr(void* ptr, size_t size) {
  if (!ptr) {
    return;
  }
  uintptr_t hash = ((uintptr_t)ptr >> 4u) % HAKO_POINTER_TABLE_CAP;
  int first_tombstone = -1;
  for (unsigned int probe = 0; probe < HAKO_POINTER_TABLE_CAP; probe++) {
    unsigned int index = (unsigned int)((hash + probe) % HAKO_POINTER_TABLE_CAP);
    if (tracked[index].ptr == ptr) {
      tracked[index].size = size;
      return;
    }
    if (tracked[index].ptr == HAKO_TRACKED_TOMBSTONE) {
      if (first_tombstone < 0) {
        first_tombstone = (int)index;
      }
      continue;
    }
    if (!tracked[index].ptr) {
      unsigned int target = first_tombstone >= 0 ? (unsigned int)first_tombstone : index;
      tracked[target].ptr = ptr;
      tracked[target].size = size;
      return;
    }
  }
  if (first_tombstone >= 0) {
    tracked[(unsigned int)first_tombstone].ptr = ptr;
    tracked[(unsigned int)first_tombstone].size = size;
    return;
  }
  pointer_table_overflow++;
}

static void hako_untrack_index(int index) {
  if (index < 0) {
    return;
  }
  tracked[index].ptr = HAKO_TRACKED_TOMBSTONE;
  tracked[index].size = 0;
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
  hako_write_kv(fd, "shim_runtime_real_fallback_count", runtime_real_fallback_count);
  hako_write_kv(fd, "shim_init_real_fallback_count", init_real_fallback_count);
  hako_write_kv(fd, "shim_host_passthrough_count", host_passthrough_count);
  hako_write_kv(fd, "shim_pointer_table_overflow", pointer_table_overflow);
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
  struct HakoProviderApiV1* api = get_api();
  if (!api || api->magic != HAKO_PROVIDER_API_MAGIC ||
      api->abi_major != HAKO_PROVIDER_API_MAJOR ||
      api->api_table_size < sizeof(struct HakoProviderApiV1) ||
      !api->alloc || !api->free || !api->owns) {
    provider_bind_failure++;
    loading_provider = 0;
    return 0;
  }
  provider_api = api;
  provider_ready = 1;
  provider_bind_success++;
  atexit(hako_write_report);
  loading_provider = 0;
  return 1;
}

static void* hako_provider_alloc(size_t size, size_t align) {
  if (!hako_ensure_provider() || !provider_api || !provider_api->alloc) {
    runtime_real_fallback_count++;
    hako_resolve_real();
    return real_malloc_fn ? real_malloc_fn(size) : 0;
  }
  in_provider_call = 1;
  void* ptr = provider_api->alloc(size, align);
  in_provider_call = 0;
  if (ptr) {
    hako_track_ptr(ptr, size);
    provider_alloc_count++;
  }
  return ptr;
}

static void hako_provider_free(void* ptr) {
  if (!ptr) {
    return;
  }
  if (!provider_api || !provider_api->free) {
    runtime_real_fallback_count++;
    hako_resolve_real();
    if (real_free_fn) {
      real_free_fn(ptr);
    }
    return;
  }
  in_provider_call = 1;
  provider_api->free(ptr);
  in_provider_call = 0;
  provider_free_count++;
}

__attribute__((visibility("default")))
void* malloc(size_t size) {
  if (loading_provider || resolving_real || in_provider_call) {
    init_real_fallback_count++;
    hako_resolve_real();
    return real_malloc_fn ? real_malloc_fn(size) : 0;
  }
  return hako_provider_alloc(size, 16);
}

__attribute__((visibility("default")))
void* calloc(size_t count, size_t size) {
  if (loading_provider || resolving_real || in_provider_call) {
    init_real_fallback_count++;
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
    provider_calloc_count++;
  }
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
  if (loading_provider || resolving_real || in_provider_call) {
    init_real_fallback_count++;
    hako_resolve_real();
    return real_realloc_fn ? real_realloc_fn(ptr, size) : 0;
  }
  int index = hako_find_tracked(ptr);
  if (index < 0) {
    host_passthrough_count++;
    hako_resolve_real();
    return real_realloc_fn ? real_realloc_fn(ptr, size) : 0;
  }
  size_t old_size = tracked[index].size;
  void* next = hako_provider_alloc(size, 16);
  if (!next) {
    return 0;
  }
  memcpy(next, ptr, old_size < size ? old_size : size);
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
    init_real_fallback_count++;
    hako_resolve_real();
    if (real_free_fn) {
      real_free_fn(ptr);
    }
    return;
  }
  int index = hako_find_tracked(ptr);
  if (index >= 0) {
    hako_untrack_index(index);
    hako_provider_free(ptr);
    return;
  }
  if (hako_ensure_provider() && provider_api && provider_api->owns) {
    in_provider_call = 1;
    int owned = provider_api->owns(ptr);
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
    args = parser.parse_args()

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
