// hako_aot.h — AOT C‑ABI (minimal), standalone small library
// Purpose: Provide tiny, portable AOT helpers to compile MIR(JSON v0) to an
// object and link it into a native executable. This header accompanies the
// small shared library libhako_aot.{so|dylib|dll}.

#pragma once

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Short diagnostics (thread‑local): "OK","VALIDATION","NOT_FOUND","FAILED","OOM"...
// Context parameter is ignored by the small lib and may be NULL.
struct hako_ctx;
const char* hako_last_error(struct hako_ctx*);
void        hako_set_last_error(const char* short_msg);

// Memory API (libc‑backed). Pair alloc/free within the same CRT.
void*       hako_mem_alloc(uint64_t size);
void*       hako_mem_realloc(void* ptr, uint64_t new_size);
void        hako_mem_free(void* ptr);

// AOT: compile MIR(JSON v0) path → object file path
// Returns 0 on success; non‑zero on failure. On failure, err_out (optional)
// receives a short heap message (free via hako_mem_free). hako_last_error()
// is set to a short token (VALIDATION/NOT_FOUND/FAILED…)
int hako_aot_compile_json(const char* mir_json_path, const char* obj_path, char** err_out);

// AOT: link object path → native executable path
// extra_ldflags may be NULL. Returns 0 on success; non‑zero on failure.
int hako_aot_link_obj(const char* obj_path, const char* exe_path, const char* extra_ldflags, char** err_out);

#ifdef __cplusplus
}
#endif
