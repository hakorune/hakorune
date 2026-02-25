// hako_kernel.c — Minimal C ABI shim (Phase 20.9, libc-backed)
// Notes
// - This file provides a tiny, portable implementation of the memory API and
//   read-only GC externs used by the LLVM canaries.
// - Link this into the harness or test runner to satisfy symbols.

#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <stdio.h>
#include <time.h>
#if defined(_WIN32)
#include <process.h>
#define GETPID _getpid
#else
#include <unistd.h>
#define GETPID getpid
#endif

// Intentionally do not include project header here to avoid typedef/name conflicts
// with experimental HostBridge declarations during canary phases.

// Forward declarations for functions referenced before definition
#include <stdint.h>
int64_t hako_time_now_ms(void);

// ---- Diagnostics (thread-local short message) ----
#if defined(_MSC_VER)
__declspec(thread) static const char* hako_tls_last_error = "OK";
#elif defined(__STDC_VERSION__) && (__STDC_VERSION__ >= 201112L)
static _Thread_local const char* hako_tls_last_error = "OK";
#else
static __thread const char* hako_tls_last_error = "OK";
#endif

// Forward declaration of opaque ctx from header; we intentionally ignore ctx here.
struct hako_ctx;

const char* hako_last_error(struct hako_ctx* ctx) {
  (void)ctx;
  return hako_tls_last_error ? hako_tls_last_error : "OK";
}

void hako_set_last_error(const char* short_msg) {
  hako_tls_last_error = short_msg ? short_msg : "OK";
}

// ---- Memory API (libc-backed)
void* hako_mem_alloc(uint64_t size) {
  if (size == 0) size = 1; // avoid undefined behavior
  void* p = malloc((size_t)size);
  if (!p) {
    hako_set_last_error("OOM");
  }
  return p;
}

void* hako_mem_realloc(void* ptr, uint64_t new_size) {
  if (new_size == 0) new_size = 1;
  void* p = realloc(ptr, (size_t)new_size);
  if (!p) {
    hako_set_last_error("OOM");
  }
  return p;
}

void hako_mem_free(void* ptr) {
  if (ptr) free(ptr);
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

// ---- AOT C API (bring-up) ----
// Minimal external invocations to ny-llvmc and system linker.
// - compile: invokes ny-llvmc to produce an object from MIR JSON v0.
// - link:    invokes cc/clang/gcc to link the object with libhako_kernel.a into an executable.
// Diagnostics:
// - On failure, sets hako_last_error to a short token ("VALIDATION"/"NOT_FOUND"/"FAILED").
// - err_out (optional) receives a short heap-allocated message; caller must free via hako_mem_free.
static int set_err(char** err_out, const char* msg) {
  if (err_out) {
    if (msg) {
      size_t n = strlen(msg);
      char* p = (char*)hako_mem_alloc((uint64_t)n + 1);
      if (p) { memcpy(p, msg, n); p[n] = '\0'; *err_out = p; }
    } else {
      *err_out = NULL;
    }
  }
  return -1;
}

static const char* tmp_dir_fallback(void) {
  const char* t = getenv("TMPDIR");
  if (!t || !*t) t = getenv("TMP");
  if (!t || !*t) t = getenv("TEMP");
  if (!t || !*t) t = "/tmp";
  return t;
}

// Read first line into heap and return pointer; NULL on failure
static char* read_first_line(const char* path) {
  if (!path) return NULL;
  FILE* f = fopen(path, "rb");
  if (!f) return NULL;
  char buf[512];
  size_t n = 0;
  int c;
  while (n < sizeof(buf) - 1 && (c = fgetc(f)) != EOF) {
    if (c == '\n' || c == '\r') break;
    buf[n++] = (char)c;
  }
  buf[n] = '\0';
  fclose(f);
  if (n == 0) return NULL;
  char* out = (char*)hako_mem_alloc((uint64_t)n + 1);
  if (!out) return NULL;
  memcpy(out, buf, n + 1);
  return out;
}

static int file_exists(const char* p) {
  if (!p) return 0;
  FILE* f = fopen(p, "rb");
  if (!f) return 0;
  fclose(f);
  return 1;
}

int hako_aot_compile_json(const char* json_in, const char* obj_out, char** err_out) {
  if (!json_in || !*json_in || !obj_out || !*obj_out) {
    hako_set_last_error("VALIDATION");
    return set_err(err_out, "invalid args");
  }
  const char* llvmc = getenv("NYASH_NY_LLVM_COMPILER");
  if (!llvmc || !*llvmc) { llvmc = "target/release/ny-llvmc"; }
  if (!file_exists(llvmc)) {
    hako_set_last_error("NOT_FOUND");
    return set_err(err_out, "ny-llvmc not found (NYASH_NY_LLVM_COMPILER)");
  }
  char logpath[1024];
  snprintf(logpath, sizeof(logpath), "%s/hako_aot_compile_%ld.log", tmp_dir_fallback(), (long)GETPID());
  char cmd[4096];
  int n = snprintf(cmd, sizeof(cmd), "\"%s\" --in \"%s\" --emit obj --out \"%s\" 2> \"%s\"", llvmc, json_in, obj_out, logpath);
  if (n <= 0 || (size_t)n >= sizeof(cmd)) {
    hako_set_last_error("VALIDATION");
    return set_err(err_out, "command too long");
  }
  int rc = system(cmd);
  if (rc != 0) {
    hako_set_last_error("FAILED");
    char* first = read_first_line(logpath);
    if (first) {
      set_err(err_out, first);
      hako_mem_free(first);
    } else {
      set_err(err_out, "COMPILE_FAILED");
    }
    remove(logpath);
    return -1;
  }
  hako_set_last_error(NULL);
  if (!file_exists(obj_out)) {
    hako_set_last_error("FAILED");
    return set_err(err_out, "object not produced");
  }
  remove(logpath);
  return 0;
}

// Resolve a candidate directory containing libhako_kernel.a or legacy libnyash_kernel.a
static const char* resolve_nyrt_dir(char* buf, size_t buflen) {
  const char* hint = getenv("NYASH_EMIT_EXE_NYRT");
  if (hint && *hint) {
    // trust caller; just copy
    snprintf(buf, buflen, "%s", hint);
    return buf;
  }
  // try target/release then crates/hako_kernel/target/release
  const char* a = "target/release";
  const char* b = "crates/hako_kernel/target/release";
  char path_a[1024];
  char path_b[1024];
  snprintf(path_a, sizeof(path_a), "%s/%s", a, "libhako_kernel.a");
  snprintf(path_b, sizeof(path_b), "%s/%s", b, "libhako_kernel.a");
  if (file_exists(path_a) || file_exists(path_b)) {
    // prefer a if exists
    snprintf(buf, buflen, "%s", file_exists(path_a) ? a : b);
    return buf;
  }
  // legacy name
  snprintf(path_a, sizeof(path_a), "%s/%s", a, "libnyash_kernel.a");
  snprintf(path_b, sizeof(path_b), "%s/%s", b, "libnyash_kernel.a");
  if (file_exists(path_a) || file_exists(path_b)) {
    snprintf(buf, buflen, "%s", file_exists(path_a) ? a : b);
    return buf;
  }
  return NULL;
}

int hako_aot_link_obj(const char* obj_in, const char* exe_out, const char* extra_ldflags, char** err_out) {
  if (!obj_in || !*obj_in || !exe_out || !*exe_out) {
    hako_set_last_error("VALIDATION");
    return set_err(err_out, "invalid args");
  }
  if (!file_exists(obj_in)) {
    hako_set_last_error("VALIDATION");
    return set_err(err_out, "object not found");
  }
  char nyrt_dir[1024];
  const char* dir = resolve_nyrt_dir(nyrt_dir, sizeof(nyrt_dir));
  if (!dir) {
    hako_set_last_error("NOT_FOUND");
    return set_err(err_out, "libhako_kernel.a not found (NYASH_EMIT_EXE_NYRT)");
  }
  char lib_a[1024];
  snprintf(lib_a, sizeof(lib_a), "%s/libhako_kernel.a", dir);
  char lib_legacy[1024];
  snprintf(lib_legacy, sizeof(lib_legacy), "%s/libnyash_kernel.a", dir);
  const char* lib = file_exists(lib_a) ? lib_a : lib_legacy;

  // Choose a linker (prefer cc)
  const char* linker = getenv("CC");
  if (!linker || !*linker) linker = "cc";

  char logpath[1024];
  snprintf(logpath, sizeof(logpath), "%s/hako_aot_link_%ld.log", tmp_dir_fallback(), (long)GETPID());
  char cmd[8192];
  // OS-specific default libraries
  const char* os_libs = "";
#if defined(__linux__)
  os_libs = "-ldl -lpthread -lm";
#elif defined(__APPLE__)
  os_libs = ""; // clang on macOS usually links required system libs by default
#elif defined(_WIN32) || defined(__MINGW32__) || defined(__MINGW64__)
  os_libs = "-lws2_32 -lbcrypt"; // minimal set for networking/crypto primitives
#else
  os_libs = "";
#endif
  // Base link command
  int n = snprintf(cmd, sizeof(cmd),
                   "\"%s\" -o \"%s\" \"%s\" -Wl,--whole-archive \"%s\" -Wl,--no-whole-archive %s 2> \"%s\"",
                   linker, exe_out, obj_in, lib, os_libs, logpath);
  if (n <= 0) {
    hako_set_last_error("VALIDATION");
    return set_err(err_out, "command too long");
  }
  // Append extra flags if provided
  if (extra_ldflags && *extra_ldflags) {
    size_t cur = (size_t)n;
    size_t rem = sizeof(cmd) - cur - 1;
    if (rem > 0) {
      strncat(cmd, " ", rem);
      cur += 1;
      rem = sizeof(cmd) - cur - 1;
    }
    if (rem > 0) {
      strncat(cmd, extra_ldflags, rem);
    }
  }
  int rc = system(cmd);
  if (rc != 0) {
    hako_set_last_error("FAILED");
    char* first = read_first_line(logpath);
    if (first) {
      set_err(err_out, first);
      hako_mem_free(first);
    } else {
      set_err(err_out, "LINK_FAILED");
    }
    remove(logpath);
    return -1;
  }
  hako_set_last_error(NULL);
  remove(logpath);
  return 0;
}
