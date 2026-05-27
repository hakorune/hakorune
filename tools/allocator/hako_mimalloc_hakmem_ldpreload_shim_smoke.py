#!/usr/bin/env python3
"""Build and load-smoke a probe-only hakmem LD_PRELOAD malloc-family shim."""

from __future__ import annotations

import argparse
import ctypes
import hashlib
import subprocess
from pathlib import Path


SHIM_C = r"""
#define _GNU_SOURCE
#include <dlfcn.h>
#include <stddef.h>
#include <stdlib.h>

typedef void* (*hako_malloc_fn)(size_t);
typedef void* (*hako_calloc_fn)(size_t, size_t);
typedef void* (*hako_realloc_fn)(void*, size_t);
typedef void (*hako_free_fn)(void*);

static hako_malloc_fn real_malloc_fn = 0;
static hako_calloc_fn real_calloc_fn = 0;
static hako_realloc_fn real_realloc_fn = 0;
static hako_free_fn real_free_fn = 0;

static void hako_resolve_malloc_family(void) {
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
}

__attribute__((visibility("default")))
void* malloc(size_t size) {
  hako_resolve_malloc_family();
  if (!real_malloc_fn) {
    return 0;
  }
  return real_malloc_fn(size);
}

__attribute__((visibility("default")))
void* calloc(size_t count, size_t size) {
  hako_resolve_malloc_family();
  if (!real_calloc_fn) {
    return 0;
  }
  return real_calloc_fn(count, size);
}

__attribute__((visibility("default")))
void* realloc(void* ptr, size_t size) {
  hako_resolve_malloc_family();
  if (!real_realloc_fn) {
    return 0;
  }
  return real_realloc_fn(ptr, size);
}

__attribute__((visibility("default")))
void free(void* ptr) {
  hako_resolve_malloc_family();
  if (real_free_fn) {
    real_free_fn(ptr);
  }
}
"""


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def require_symbols(binary: Path) -> list[str]:
    output = subprocess.check_output(["nm", "-D", "--defined-only", str(binary)], text=True)
    required = ["malloc", "free", "calloc", "realloc"]
    missing = [name for name in required if f" {name}\n" not in output]
    if missing:
        raise SystemExit("shim missing malloc-family exports: " + ",".join(missing))
    return required


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    out_dir = args.out_dir.resolve()
    out_dir.mkdir(parents=True, exist_ok=True)
    source = out_dir / "hako_hakmem_ldpreload_probe_shim.c"
    binary = out_dir / "libhako_hakmem_ldpreload_probe.so"
    source.write_text(SHIM_C.lstrip(), encoding="utf-8")

    subprocess.run(
        ["cc", "-shared", "-fPIC", "-O2", "-Wall", "-Wextra", str(source), "-ldl", "-o", str(binary)],
        check=True,
    )
    symbols = require_symbols(binary)
    ctypes.CDLL(str(binary))

    lines = [
        "output_contract=hako-mimalloc-hakmem-ldpreload-shim-smoke-v0",
        "input_contract=hako-mimalloc-hakmem-ldpreload-shim-decision-v0",
        "ld_preload_compatible=1",
        "shim_kind=malloc_family_probe_only",
        f"shim_source_path={source}",
        f"shim_artifact_path={binary}",
        f"shim_artifact_sha256={sha256_file(binary)}",
        f"shim_artifact_size_bytes={binary.stat().st_size}",
        "shared_library_load_executed=1",
        "malloc_family_symbols_exported=1",
        "malloc_family_symbols=" + ",".join(symbols),
        "hakmem_script_compatible=probe-only",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "winner_claim=0",
        "next_row=HAKO-MIMALLOC-HAKMEM-LDPRELOAD-BENCH-PILOT-296X-001",
        "summary=ok",
    ]
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
