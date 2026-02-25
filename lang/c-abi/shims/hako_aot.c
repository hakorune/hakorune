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

// ---- Helpers
static int set_err(char** err_out, const char* msg) {
  if (err_out) {
    if (msg) {
      size_t n = strlen(msg); char* p = (char*)hako_mem_alloc((uint64_t)n + 1);
      if (p) { memcpy(p, msg, n); p[n] = '\0'; *err_out = p; }
    } else { *err_out = NULL; }
  }
  return -1;
}
static int file_exists(const char* p) {
  if (!p) return 0; FILE* f = fopen(p, "rb"); if (!f) return 0; fclose(f); return 1;
}
static const char* tmp_dir_fallback(void) {
  const char* t = getenv("TMPDIR"); if (!t||!*t) t = getenv("TMP"); if (!t||!*t) t = getenv("TEMP"); if (!t||!*t) t = "/tmp"; return t;
}
static char* read_first_line(const char* path) {
  if (!path) return NULL; FILE* f = fopen(path, "rb"); if (!f) return NULL;
  char buf[512]; size_t n=0; int c; while (n<sizeof(buf)-1 && (c=fgetc(f))!=EOF) { if (c=='\n'||c=='\r') break; buf[n++]=(char)c; }
  buf[n]='\0'; fclose(f); if (!n) return NULL; char* out=(char*)hako_mem_alloc((uint64_t)n+1); if (!out) return NULL; memcpy(out,buf,n+1); return out;
}

// ---- AOT: compile JSON → object (ny-llvmc)
// ---- Optional FFI (dlopen)
#if !defined(_WIN32)
#include <dlfcn.h>
static void* open_ffi_lib(void) {
  const char* lib = getenv("HAKO_AOT_FFI_LIB");
  char buf[1024];
  if (!lib || !*lib) {
    // Try dev default
    snprintf(buf, sizeof(buf), "%s", "target/release/libhako_llvmc_ffi.so");
    lib = buf;
  }
  void* h = dlopen(lib, RTLD_NOW);
  if (!h && (!getenv("HAKO_AOT_FFI_LIB") || !*getenv("HAKO_AOT_FFI_LIB"))) {
    // Try dist-style: lib/libhako_llvmc_ffi.so (if running from dist root)
    snprintf(buf, sizeof(buf), "%s", "lib/libhako_llvmc_ffi.so");
    h = dlopen(buf, RTLD_NOW);
  }
  return h;
}
static int try_ffi_compile(const char* json_in, const char* obj_out, char** err_out) {
  void* h = open_ffi_lib();
  if (!h) { HAKO_FAIL_WITH(err_out, "UNSUPPORTED", "FFI library not found"); }
  typedef int (*ffi_compile_fn)(const char*, const char*, char**);
  ffi_compile_fn fn = (ffi_compile_fn)dlsym(h, "hako_llvmc_compile_json");
  if (!fn) { dlclose(h); HAKO_FAIL_WITH(err_out, "UNSUPPORTED", "FFI symbol missing: hako_llvmc_compile_json"); }
  int rc = fn(json_in, obj_out, err_out);
  dlclose(h);
  return rc;
}
static int try_ffi_link(const char* obj_in, const char* exe_out, const char* extra_ldflags, char** err_out) {
  void* h = open_ffi_lib();
  if (!h) { HAKO_FAIL_WITH(err_out, "UNSUPPORTED", "FFI library not found"); }
  typedef int (*ffi_link_fn)(const char*, const char*, const char*, char**);
  ffi_link_fn fn = (ffi_link_fn)dlsym(h, "hako_llvmc_link_obj");
  if (!fn) { dlclose(h); HAKO_FAIL_WITH(err_out, "UNSUPPORTED", "FFI symbol missing: hako_llvmc_link_obj"); }
  int rc = fn(obj_in, exe_out, extra_ldflags, err_out);
  dlclose(h);
  return rc;
}
#else
static int try_ffi_compile(const char* json_in, const char* obj_out, char** err_out) {
  (void)json_in; (void)obj_out; (void)err_out; hako_set_last_error("UNSUPPORTED"); return set_err(err_out, "FFI unsupported on this platform");
}
static int try_ffi_link(const char* obj_in, const char* exe_out, const char* extra_ldflags, char** err_out) {
  (void)obj_in; (void)exe_out; (void)extra_ldflags; (void)err_out; hako_set_last_error("UNSUPPORTED"); return set_err(err_out, "FFI unsupported on this platform");
}
#endif

int hako_aot_compile_json(const char* json_in, const char* obj_out, char** err_out) {
  const char* use_ffi = getenv("HAKO_AOT_USE_FFI");
  if (use_ffi && (*use_ffi=='1' || strcasecmp(use_ffi, "true")==0 || strcasecmp(use_ffi, "on")==0)) {
    return try_ffi_compile(json_in, obj_out, err_out);
  }

  // Inject opt_level defaults when falling back to CLI (insurance for Python harness)
  if (!getenv("HAKO_LLVM_OPT_LEVEL")) {
    setenv("HAKO_LLVM_OPT_LEVEL", "0", 1);
  }
  if (!getenv("NYASH_LLVM_OPT_LEVEL")) {
    setenv("NYASH_LLVM_OPT_LEVEL", "0", 1);
  }

  if (!json_in || !*json_in || !obj_out || !*obj_out) { HAKO_FAIL_WITH(err_out, "VALIDATION", "invalid args"); }
  const char* llvmc = getenv("NYASH_NY_LLVM_COMPILER"); if (!llvmc || !*llvmc) { llvmc = "target/release/ny-llvmc"; }
  if (!file_exists(llvmc)) { HAKO_FAIL_WITH(err_out, "NOT_FOUND", "ny-llvmc not found (NYASH_NY_LLVM_COMPILER)"); }
  char logpath[1024]; snprintf(logpath, sizeof(logpath), "%s/hako_aot_compile_%ld.log", tmp_dir_fallback(), (long)GETPID());
  char cmd[4096]; int n = snprintf(cmd, sizeof(cmd), "\"%s\" --in \"%s\" --emit obj --out \"%s\" 2> \"%s\"", llvmc, json_in, obj_out, logpath);
  if (n <= 0 || (size_t)n >= sizeof(cmd)) { HAKO_FAIL_WITH(err_out, "VALIDATION", "command too long"); }
  if (getenv("HAKO_AOT_DEBUG")) { fprintf(stderr, "[hako_aot] link cmd: %s\n", cmd); }
  // Prepend command line to log for easier diagnostics (first line)
  {
    FILE* lf = fopen(logpath, "wb");
    if (lf) { fprintf(lf, "%s\n", cmd); fclose(lf); }
  }
  int rc = system(cmd);
  if (rc != 0) { hako_set_last_error("FAILED"); char* first = read_first_line(logpath); if (first) { set_err(err_out, first); hako_mem_free(first); } else { set_err(err_out, "COMPILE_FAILED"); } remove(logpath); return -1; }
  hako_set_last_error(NULL); if (!file_exists(obj_out)) { HAKO_FAIL_WITH(err_out, "FAILED", "object not produced"); }
  remove(logpath); return 0;
}

// ---- AOT: link object → exe
static const char* resolve_nyrt_dir(char* buf, size_t buflen) {
  const char* hint = getenv("NYASH_EMIT_EXE_NYRT"); if (hint && *hint) { snprintf(buf, buflen, "%s", hint); return buf; }
  const char* a = "target/release"; const char* b = "crates/hako_kernel/target/release"; char pa[1024], pb[1024];
  snprintf(pa, sizeof(pa), "%s/libhako_kernel.a", a); snprintf(pb, sizeof(pb), "%s/libhako_kernel.a", b);
  if (file_exists(pa) || file_exists(pb)) { snprintf(buf, buflen, "%s", file_exists(pa) ? a : b); return buf; }
  snprintf(pa, sizeof(pa), "%s/libnyash_kernel.a", a); snprintf(pb, sizeof(pb), "%s/libnyash_kernel.a", b);
  if (file_exists(pa) || file_exists(pb)) { snprintf(buf, buflen, "%s", file_exists(pa) ? a : b); return buf; }
  return NULL;
}

int hako_aot_link_obj(const char* obj_in, const char* exe_out, const char* extra_ldflags, char** err_out) {
  const char* use_ffi = getenv("HAKO_AOT_USE_FFI");
  if (use_ffi && (*use_ffi=='1' || strcasecmp(use_ffi, "true")==0 || strcasecmp(use_ffi, "on")==0)) {
    return try_ffi_link(obj_in, exe_out, extra_ldflags, err_out);
  }
  if (!obj_in || !*obj_in || !exe_out || !*exe_out) { HAKO_FAIL_WITH(err_out, "VALIDATION", "invalid args"); }
  if (!file_exists(obj_in)) { HAKO_FAIL_WITH(err_out, "VALIDATION", "object not found"); }
  char dirbuf[1024]; const char* dir = resolve_nyrt_dir(dirbuf, sizeof(dirbuf)); if (!dir) { HAKO_FAIL_WITH(err_out, "NOT_FOUND", "libhako_kernel.a not found (NYASH_EMIT_EXE_NYRT)"); }
  char lib_a[1024]; snprintf(lib_a, sizeof(lib_a), "%s/libhako_kernel.a", dir); char lib_legacy[1024]; snprintf(lib_legacy, sizeof(lib_legacy), "%s/libnyash_kernel.a", dir);
  const char* lib = file_exists(lib_a) ? lib_a : lib_legacy; const char* linker = getenv("CC"); if (!linker || !*linker) linker = "cc";
  const char* os_libs = "";
#if defined(__linux__)
  os_libs = "-ldl -lpthread -lm";
#elif defined(__APPLE__)
  os_libs = "";
#elif defined(_WIN32) || defined(__MINGW32__) || defined(__MINGW64__)
  os_libs = "-lws2_32 -lbcrypt";
#endif
  char logpath[1024]; snprintf(logpath, sizeof(logpath), "%s/hako_aot_link_%ld.log", tmp_dir_fallback(), (long)GETPID());
  // Prefer static core, but also link optional shim shared lib when available (provides hako_console_*)
  char shim_flag[1024] = "";
#if defined(__linux__) || defined(__APPLE__)
  char shim_so[1024];
#if defined(__APPLE__)
  snprintf(shim_so, sizeof(shim_so), "%s/libhako_kernel_shim.dylib", dir);
#else
  snprintf(shim_so, sizeof(shim_so), "%s/libhako_kernel_shim.so", dir);
#endif
  if (file_exists(shim_so)) {
    // Add -L and -Wl,-rpath to locate the shim at runtime, and -l link flag
    snprintf(shim_flag, sizeof(shim_flag), " -L\"%s\" -Wl,-rpath,\"%s\" -lhako_kernel_shim", dir, dir);
  }
#endif
  const char* pie_avoid = "";
#if defined(__linux__)
  pie_avoid = " -no-pie";
#endif
  char cmd[8192]; int n = snprintf(cmd, sizeof(cmd), "\"%s\"%s -o \"%s\" \"%s\" -Wl,--whole-archive \"%s\" -Wl,--no-whole-archive%s %s 2> \"%s\"", linker, pie_avoid, exe_out, obj_in, lib, shim_flag, os_libs, logpath);
  if (n <= 0) { HAKO_FAIL_WITH(err_out, "VALIDATION", "command too long"); }
  if (extra_ldflags && *extra_ldflags) { size_t cur=strlen(cmd); size_t rem=sizeof(cmd)-cur-1; if (rem>0) { strncat(cmd, " ", rem); cur++; rem=sizeof(cmd)-cur-1; } if (rem>0) { strncat(cmd, extra_ldflags, rem); } }
  // ENV override: HAKO_AOT_LDFLAGS appended at the end (dev convenience)
  {
    const char* env_ld = getenv("HAKO_AOT_LDFLAGS");
    if (env_ld && *env_ld) {
      size_t cur=strlen(cmd); size_t rem=sizeof(cmd)-cur-1; if (rem>0) { strncat(cmd, " ", rem); cur++; rem=sizeof(cmd)-cur-1; }
      if (rem>0) { strncat(cmd, env_ld, rem); }
    }
  }
  int rc = system(cmd);
  if (rc != 0) { hako_set_last_error("FAILED"); char* first=read_first_line(logpath); if (first){ set_err(err_out, first); hako_mem_free(first);} else { set_err(err_out, "LINK_FAILED"); } remove(logpath); return -1; }
  hako_set_last_error(NULL); remove(logpath); return 0;
}
