// hako_hostbridge.h — HostBridge C‑ABI v1 (skeleton header)
// This header is provided for reference and external integration experiments.
// Implementation is tracked in docs and the Rust engine; ABI is subject to review.

#pragma once

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
  HAKO_OK = 0,
  HAKO_NOT_FOUND = 1,
  HAKO_BAD_LOCK = 2,
  HAKO_INCOMPATIBLE = 3,
  HAKO_OOM = 4,
  HAKO_UNSUPPORTED = 5,
  HAKO_VALIDATION = 6,
  HAKO_PANIC = 7,
} hako_status;

typedef struct {
  uint32_t struct_size;    // sizeof(hako_api_info)
  uint16_t abi_major;
  uint16_t abi_minor;
  uint32_t caps;           // capability bits
  const void* opt_alloc;   // reserved (allocator pointer)
} hako_api_info;

typedef struct hako_ctx hako_ctx;           // opaque
typedef uint32_t hako_type_id;
typedef uint64_t hako_method_id;            // stable id (Box.method/Arity → u64)

// lifecycle / diagnostics
hako_status hako_open(const char* lock_or_capsule_path, hako_ctx** out);
void        hako_close(hako_ctx*);
// Diagnostics
// - Thread-local short message buffer for the last error on the current thread.
// - Returns a short, stable string literal such as "OK", "NOT_FOUND", "OOM", "UNSUPPORTED", "VALIDATION".
// - The context parameter may be NULL; the minimal shim ignores it and uses TLS.
const char* hako_last_error(hako_ctx*);
// Set the thread-local last error to a short, stable string. Passing NULL clears the error (becomes "OK").
void        hako_set_last_error(const char* short_msg);

// discovery (experimental; disabled by default to avoid identifier conflicts in C)
#if defined(HAKO_EXPERIMENTAL_DISCOVERY)
hako_status hako_list_types(hako_ctx*, const char*** out_names, size_t* out_count);
hako_status hako_type_id(hako_ctx*, const char* type_name, hako_type_id* out);
hako_status hako_method_id(hako_ctx*, hako_type_id tid, const char* method, uint32_t arity, hako_method_id* out);
void        hako_free_cstrings(const char** names, size_t count);
#endif

// unified call (Extern/Method/ModuleFunction/Constructor)
typedef enum {
  HAKO_V_NULL = 0,
  HAKO_V_I64  = 1,
  HAKO_V_BOOL = 2,
  HAKO_V_F64  = 3,   // optional in v1
  HAKO_V_STR  = 4,
  HAKO_V_BYTES= 5,
  HAKO_V_HANDLE=6,
} hako_value_tag;

typedef struct {
  hako_value_tag tag;
  union {
    int64_t  i64;
    double   f64;
    int32_t  b32;      // 0|1
    struct { const char* ptr; uint64_t len; } str;   // len bytes; not NUL-terminated
    struct { const void* ptr; uint64_t len; } bytes; // len bytes
    void*    handle;
  } as;
} hako_value;

// Slice limits (recommendations for cross-ABI safety)
#ifndef HAKO_STR_MAX
#define HAKO_STR_MAX   ((uint64_t)((1ULL<<31)-1))
#endif
#ifndef HAKO_BYTES_MAX
#define HAKO_BYTES_MAX ((uint64_t)((1ULL<<31)-1))
#endif
hako_status hako_call(hako_ctx*, hako_method_id mid,
                      void* self_or_null,
                      const hako_value* args, size_t argc,
                      hako_value* out_ret);

// ---- Memory API (Phase 20.9: script→C ABI path) ----
// Minimal allocator interface. Implementations may wrap libc, mimalloc, etc.
// Contracts:
// - Ownership: Any pointer returned by these functions must be freed by hako_mem_free(). Do NOT mix CRT frees.
// - Alignment: Returned pointers satisfy alignment of max_align_t for the platform (or stricter allocator defaults).
// - Thread-safety: These functions are required to be thread-safe.
// - Error handling: On OOM, functions return NULL (or leave input unmodified for realloc) and should set a diagnosable error
//   via hako_last_error() when a context is available; callers must check for NULL.
void*      hako_mem_alloc(uint64_t size);
void*      hako_mem_realloc(void* ptr, uint64_t new_size);
void       hako_mem_free(void* ptr);

// ---- GC read-only convenience (Phase 20.9: script→C ABI path) ----
// Ownership: returned char* is heap-allocated by the callee; caller must free via hako_mem_free().
// Platform note: Always pair frees with hako_mem_free() to avoid CRT boundary issues (Windows msvcrt vs. ucrt, etc.).
// When not available, implementations should return NULL and set hako_last_error for diagnostics.
const char* hako_gc_stats(void);
int64_t     hako_gc_roots_snapshot(void);
// Local environment: get value for key (UTF‑8). Returns newly allocated char* or NULL.
const char* hako_env_local_get(const char* key);

// ---- Console / Time / String (minimal developer shim) ----
// Console: prints string with newline (stderr for warn/error). Thread-safe for concurrent calls.
void        hako_console_log(const char* s);
void        hako_console_warn(const char* s);
void        hako_console_error(const char* s);
// Console (numeric): print 64-bit integer with newline (bench/dev convenience)
void        hako_console_log_i64(int64_t x);
// Bench barrier: very light side-effect to prevent over-aggressive optimization
void        hako_barrier_touch_i64(int64_t x);
// Time: current wall-clock ms (dev canary; precision depends on platform)
int64_t     hako_time_now_ms(void);
// String: duplicate C string to heap; caller must free with hako_mem_free.
// The returned buffer is NUL-terminated and aligned as per allocator guarantees.
const char* hako_string_to_i8p(const char* s);

// ---- AOT (minimal C API; Phase 20.10 bring-up) ----
// Compile MIR JSON v0 source to an object file.
// - json_in: path to MIR JSON (v0) file
// - obj_out: path to write the object (.o)
// - err_out: optional; on failure, set to heap-allocated short message (free via hako_mem_free)
// Returns 0 on success; non-zero on failure.
int hako_aot_compile_json(const char* json_in, const char* obj_out, char** err_out);

// Link an object file into a native executable.
// - obj_in: path to object (.o)
// - exe_out: output executable path
// - extra_ldflags: optional linker flags (may be NULL)
// - err_out: optional; on failure, set to heap-allocated short message (free via hako_mem_free)
// Returns 0 on success; non-zero on failure.
int hako_aot_link_obj(const char* obj_in, const char* exe_out, const char* extra_ldflags, char** err_out);

#ifdef __cplusplus
}
#endif
