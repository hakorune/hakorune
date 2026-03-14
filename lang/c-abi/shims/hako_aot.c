// hako_aot.c — Small standalone AOT C‑ABI
// Notes: duplicates minimal TLS + memory + AOT compile/link from kernel shim,
// so it can be linked independently as libhako_aot.{so|dylib|dll}.

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

struct hako_ctx;

// ---- Diagnostics (thread‑local short message)
#if defined(_MSC_VER)
__declspec(thread) static const char* hako_tls_last_error = "OK";
#elif defined(__STDC_VERSION__) && (__STDC_VERSION__ >= 201112L)
static _Thread_local const char* hako_tls_last_error = "OK";
#else
static __thread const char* hako_tls_last_error = "OK";
#endif

const char* hako_last_error(struct hako_ctx* ctx) {
  (void)ctx; return hako_tls_last_error ? hako_tls_last_error : "OK";
}
void hako_set_last_error(const char* short_msg) {
  hako_tls_last_error = short_msg ? short_msg : "OK";
}

// Diagnostics helpers
#include "../include/hako_diag.h"

// ---- Memory (libc)
void* hako_mem_alloc(uint64_t size) {
  if (size == 0) size = 1; void* p = malloc((size_t)size);
  if (!p) hako_set_last_error("OOM"); return p;
}
void* hako_mem_realloc(void* ptr, uint64_t new_size) {
  if (new_size == 0) new_size = 1; void* p = realloc(ptr, (size_t)new_size);
  if (!p) hako_set_last_error("OOM"); return p;
}
void hako_mem_free(void* ptr) { if (ptr) free(ptr); }

#include "hako_aot_shared_impl.inc"
