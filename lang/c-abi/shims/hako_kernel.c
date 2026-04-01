// hako_kernel.c — Minimal C ABI shim (Phase 20.9, libc-backed)
// Notes
// - This file provides a tiny, portable implementation of the memory API and
//   read-only GC externs used by the LLVM canaries.
// - Link this into the harness or test runner to satisfy symbols.

#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <stdio.h>
#include <errno.h>
#include <time.h>
#if defined(_WIN32)
#include <windows.h>
#include <process.h>
#define GETPID _getpid
#else
#include <unistd.h>
#include <sys/mman.h>
#define GETPID getpid
#endif

// Intentionally do not include project header here to avoid typedef/name conflicts
// with experimental HostBridge declarations during canary phases.

// Forward declarations for functions referenced before definition
#include <stdint.h>
int64_t hako_time_now_ms(void);

// ---- Hako forward hook registry (C ABI canonical surface) ----
#include "hako_forward_registry_shared_impl.inc"

// ---- Shared diagnostics + memory (libc-backed)
#include "hako_diag_mem_shared_impl.inc"

static uint64_t hako_osvm_page_size_bytes(void) {
#if defined(_WIN32)
  SYSTEM_INFO si;
  GetSystemInfo(&si);
  uint64_t page = (uint64_t)si.dwPageSize;
  return page > 0 ? page : 4096ULL;
#elif defined(_SC_PAGESIZE)
  long page_raw = sysconf(_SC_PAGESIZE);
  return (page_raw > 0) ? (uint64_t)page_raw : 4096ULL;
#else
  return 4096ULL;
#endif
}

static uint64_t hako_osvm_round_up_bytes(uint64_t size, uint64_t page) {
  return ((size + page - 1) / page) * page;
}

// ---- GC read-only externs
// Returns a newly allocated JSON string with basic counters (dummy values).
// Caller must free via hako_mem_free().
const char* hako_gc_stats(void) {
  const char* tmpl = "{\"safepoints\":%d,\"barrier_reads\":%d,\"barrier_writes\":%d}";
  // Minimal deterministic numbers for canary; replace with real counters later.
  int sp = 0, rd = 0, wr = 0;
  // Compute required size
  int n = snprintf(NULL, 0, tmpl, sp, rd, wr);
  if (n <= 0) { hako_set_last_error("VALIDATION"); return NULL; }
  char* buf = (char*)hako_mem_alloc((uint64_t)n + 1);
  if (!buf) { /* hako_mem_alloc sets OOM */ return NULL; }
  (void)snprintf(buf, (size_t)n + 1, tmpl, sp, rd, wr);
  return buf;
}

// Returns a best-effort roots count (dummy 0 for now).
int64_t hako_gc_roots_snapshot(void) {
  return 0;
}

// ---- Console / Time / String helpers (minimal)
// Log string to stdout
void hako_console_log(const char* s) {
  if (!s) {
    fprintf(stdout, "\n");
    fflush(stdout);
    return;
  }
  fprintf(stdout, "%s\n", s);
  fflush(stdout);
}

// Log string as warning to stderr
void hako_console_warn(const char* s) {
  if (!s) {
    fprintf(stderr, "\n");
    fflush(stderr);
    return;
  }
  fprintf(stderr, "%s\n", s);
  fflush(stderr);
}

// Log string as error to stderr
void hako_console_error(const char* s) {
  if (!s) {
    fprintf(stderr, "\n");
    fflush(stderr);
    return;
  }
  fprintf(stderr, "%s\n", s);
  fflush(stderr);
}

// Log 64-bit integer to stdout (bench/dev convenience)
void hako_console_log_i64(int64_t x) {
  fprintf(stdout, "%lld\n", (long long)x);
  fflush(stdout);
}

// Very light barrier (portable no-op with observable call boundary)
void hako_barrier_touch_i64(int64_t x) {
  (void)x;
#if defined(__GNUC__) || defined(__clang__)
  __asm__ __volatile__("" ::: "memory");
#else
  /* Fallback: do nothing */
#endif
}

// No-op bench hook: ensures a cheap, non-optimizable call boundary for micro benches
#if defined(__GNUC__) || defined(__clang__)
__attribute__((visibility("default")))
#endif
void hako_bench_noop_i64(int64_t x) {
  (void)x;
#if defined(__GNUC__) || defined(__clang__)
  __asm__ __volatile__("" ::: "memory");
#endif
}

// Force value usage (volatile-like sink)
#if defined(__GNUC__) || defined(__clang__)
__attribute__((visibility("default")))
#endif
void hako_bench_use_value_i64(int64_t x) {
#if defined(__GNUC__) || defined(__clang__)
  __asm__ __volatile__("" :: "r"(x) : "memory");
#else
  (void)x;
#endif
}

int64_t hako_osvm_reserve_bytes_i64(int64_t len_bytes) {
  if (len_bytes <= 0) {
    hako_set_last_error("VALIDATION");
    return 0;
  }
#if defined(_WIN32)
  uint64_t page = hako_osvm_page_size_bytes();
  uint64_t size = (uint64_t)len_bytes;
  uint64_t rounded = hako_osvm_round_up_bytes(size, page);
  void* p = VirtualAlloc(NULL, (SIZE_T)rounded, MEM_RESERVE, PAGE_NOACCESS);
  if (!p) {
    hako_set_last_error("OOM");
    return 0;
  }
  hako_set_last_error(NULL);
  return (int64_t)(intptr_t)p;
#elif defined(MAP_PRIVATE) && defined(MAP_ANONYMOUS)
  uint64_t page = hako_osvm_page_size_bytes();
  uint64_t size = (uint64_t)len_bytes;
  uint64_t rounded = hako_osvm_round_up_bytes(size, page);
  void* p = mmap(NULL, (size_t)rounded, PROT_NONE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
  if (p == MAP_FAILED) {
    hako_set_last_error("OOM");
    return 0;
  }
  hako_set_last_error(NULL);
  return (int64_t)(intptr_t)p;
#else
  hako_set_last_error("UNSUPPORTED");
  return 0;
#endif
}

int64_t hako_osvm_commit_bytes_i64(int64_t base, int64_t len_bytes) {
  if (base <= 0 || len_bytes <= 0) {
    hako_set_last_error("VALIDATION");
    return HAKO_VALIDATION;
  }
#if defined(_WIN32)
  uint64_t page = hako_osvm_page_size_bytes();
  uint64_t size = (uint64_t)len_bytes;
  uint64_t rounded = hako_osvm_round_up_bytes(size, page);
  void* p = VirtualAlloc((LPVOID)(intptr_t)base, (SIZE_T)rounded, MEM_COMMIT, PAGE_READWRITE);
  if (!p) {
    hako_set_last_error("OOM");
    return HAKO_OOM;
  }
  hako_set_last_error(NULL);
  return HAKO_OK;
#elif defined(MAP_PRIVATE) && defined(MAP_ANONYMOUS)
  uint64_t page = hako_osvm_page_size_bytes();
  uint64_t size = (uint64_t)len_bytes;
  uint64_t rounded = hako_osvm_round_up_bytes(size, page);
  void* ptr = (void*)(intptr_t)base;
  if (mprotect(ptr, (size_t)rounded, PROT_READ | PROT_WRITE) != 0) {
    hako_set_last_error(errno == ENOMEM ? "OOM" : "VALIDATION");
    return errno == ENOMEM ? HAKO_OOM : HAKO_VALIDATION;
  }
  hako_set_last_error(NULL);
  return HAKO_OK;
#else
  hako_set_last_error("UNSUPPORTED");
  return HAKO_UNSUPPORTED;
#endif
}

int64_t hako_osvm_decommit_bytes_i64(int64_t base, int64_t len_bytes) {
  if (base <= 0 || len_bytes <= 0) {
    hako_set_last_error("VALIDATION");
    return HAKO_VALIDATION;
  }
#if defined(_WIN32)
  uint64_t page = hako_osvm_page_size_bytes();
  uint64_t size = (uint64_t)len_bytes;
  uint64_t rounded = hako_osvm_round_up_bytes(size, page);
  if (!VirtualFree((LPVOID)(intptr_t)base, (SIZE_T)rounded, MEM_DECOMMIT)) {
    hako_set_last_error("VALIDATION");
    return HAKO_VALIDATION;
  }
  hako_set_last_error(NULL);
  return HAKO_OK;
#elif defined(MAP_PRIVATE) && defined(MAP_ANONYMOUS)
  uint64_t page = hako_osvm_page_size_bytes();
  uint64_t size = (uint64_t)len_bytes;
  uint64_t rounded = hako_osvm_round_up_bytes(size, page);
  void* ptr = (void*)(intptr_t)base;
  if (mprotect(ptr, (size_t)rounded, PROT_NONE) != 0) {
    hako_set_last_error(errno == ENOMEM ? "OOM" : "VALIDATION");
    return errno == ENOMEM ? HAKO_OOM : HAKO_VALIDATION;
  }
#if defined(MADV_DONTNEED)
  (void)madvise(ptr, (size_t)rounded, MADV_DONTNEED);
#endif
  hako_set_last_error(NULL);
  return HAKO_OK;
#else
  hako_set_last_error("UNSUPPORTED");
  return HAKO_UNSUPPORTED;
#endif
}

// Simple random-ish i64 (LCG)
#if defined(__GNUC__) || defined(__clang__)
__attribute__((visibility("default")))
#endif
int64_t hako_bench_random_i64(void) {
  static uint64_t s = 0;
  if (s == 0) {
    uint64_t seed = (uint64_t)hako_time_now_ms();
    seed ^= (uint64_t)GETPID();
    seed ^= (uint64_t)(uintptr_t)&s;
    s = seed | 1ULL;
  }
  // LCG: s = s * A + C
  s = s * 6364136223846793005ULL + 1442695040888963407ULL;
  return (int64_t)(s >> 1);
}

// Read environment variable value; duplicate as heap C string
const char* hako_env_local_get(const char* key) {
  if (!key || !*key) { hako_set_last_error("VALIDATION"); return NULL; }
  const char* v = getenv(key);
  if (!v) { hako_set_last_error("NOT_FOUND"); return NULL; }
  size_t n = strlen(v);
  char* out = (char*)hako_mem_alloc((uint64_t)n + 1);
  if (!out) { /* hako_mem_alloc sets OOM */ return NULL; }
  memcpy(out, v, n);
  out[n] = '\0';
  hako_set_last_error(NULL);
  return out;
}

// Export aliases for llvm extern names expected by the harness
#if defined(__GNUC__) || defined(__clang__)
__attribute__((visibility("default"))) int64_t env_console_log_alias(const char* s) __asm__("env.console.log");
int64_t env_console_log_alias(const char* s) { hako_console_log(s); return 0; }

__attribute__((visibility("default"))) int64_t nyash_console_log_alias(const char* s) __asm__("nyash.console.log");
int64_t nyash_console_log_alias(const char* s) { hako_console_log(s); return 0; }

__attribute__((visibility("default"))) int64_t env_console_warn_alias(const char* s) __asm__("env.console.warn");
int64_t env_console_warn_alias(const char* s) { hako_console_warn(s); return 0; }

__attribute__((visibility("default"))) int64_t env_console_error_alias(const char* s) __asm__("env.console.error");
int64_t env_console_error_alias(const char* s) { hako_console_error(s); return 0; }

// Alias for env.local.get symbol (returns char*)
__attribute__((visibility("default"))) const char* env_local_get_alias(const char* key) __asm__("env.local.get");
const char* env_local_get_alias(const char* key) { return hako_env_local_get(key); }
#endif

// Monotonic-ish wall clock (ms). Not strictly monotonic; dev canary only.
#if defined(__GNUC__) || defined(__clang__)
__attribute__((visibility("default")))
#endif
int64_t hako_time_now_ms(void) {
  struct timespec ts;
#ifdef CLOCK_REALTIME
  clock_gettime(CLOCK_REALTIME, &ts);
#else
  ts.tv_sec = time(NULL);
  ts.tv_nsec = 0;
#endif
  return (int64_t)ts.tv_sec * 1000 + (int64_t)(ts.tv_nsec / 1000000);
}

// ---- Minimal host aliases for Box constructors (bench stability) ----
#if defined(__GNUC__) || defined(__clang__)
// Provide a cheap alias for ArrayBox birth to avoid plugin/host dependency in AOT canaries.
// Returns 0 (invalid handle), which is acceptable for fixed-N birth throughput benches.
__attribute__((visibility("default"))) int64_t nyash_array_new_h(void) __asm__("nyash_array_new_h");
int64_t nyash_array_new_h(void) { return 0; }
__attribute__((visibility("default"))) int64_t hako_array_new_h(void) __asm__("hako_array_new_h");
int64_t hako_array_new_h(void) { return 0; }
#endif

// Duplicate a C string into heap memory (ownership to caller)
const char* hako_string_to_i8p(const char* s) {
  if (!s) return NULL;
  size_t n = strlen(s);
  char* out = (char*)hako_mem_alloc((uint64_t)n + 1);
  if (!out) return NULL;
  memcpy(out, s, n);
  out[n] = '\0';
  return out;
}

// ---- AOT C API (canonical source truth shared with hako_aot.c) ----
#include "hako_aot_shared_impl.inc"
