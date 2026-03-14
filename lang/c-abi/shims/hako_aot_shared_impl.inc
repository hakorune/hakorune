// hako_aot_shared_impl.inc
// Shared source-truth for AOT compile/link helpers.

static int hako_aot_set_err(char** err_out, const char* msg) {
  if (err_out) {
    if (msg) {
      size_t n = strlen(msg);
      char* p = (char*)hako_mem_alloc((uint64_t)n + 1);
      if (p) {
        memcpy(p, msg, n);
        p[n] = '\0';
        *err_out = p;
      }
    } else {
      *err_out = NULL;
    }
  }
  return -1;
}

static int hako_aot_file_exists(const char* p) {
  if (!p) return 0;
  FILE* f = fopen(p, "rb");
  if (!f) return 0;
  fclose(f);
  return 1;
}

static const char* hako_aot_tmp_dir_fallback(void) {
  const char* t = getenv("TMPDIR");
  if (!t || !*t) t = getenv("TMP");
  if (!t || !*t) t = getenv("TEMP");
  if (!t || !*t) t = "/tmp";
  return t;
}

static char* hako_aot_read_first_line(const char* path) {
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

static int hako_aot_env_truthy(const char* raw) {
  if (!raw || !*raw) return 0;
  if (strcmp(raw, "1") == 0) return 1;
  if (strcmp(raw, "true") == 0 || strcmp(raw, "TRUE") == 0) return 1;
  if (strcmp(raw, "on") == 0 || strcmp(raw, "ON") == 0) return 1;
  return 0;
}

#if !defined(_WIN32)
#include <dlfcn.h>
static void* hako_aot_open_ffi_lib(void) {
  const char* lib = getenv("HAKO_AOT_FFI_LIB");
  char buf[1024];
  if (!lib || !*lib) {
    snprintf(buf, sizeof(buf), "%s", "target/release/libhako_llvmc_ffi.so");
    lib = buf;
  }
  void* h = dlopen(lib, RTLD_NOW);
  if (!h && (!getenv("HAKO_AOT_FFI_LIB") || !*getenv("HAKO_AOT_FFI_LIB"))) {
    snprintf(buf, sizeof(buf), "%s", "lib/libhako_llvmc_ffi.so");
    h = dlopen(buf, RTLD_NOW);
  }
  return h;
}

static int hako_aot_try_ffi_compile(const char* json_in, const char* obj_out, char** err_out) {
  void* h = hako_aot_open_ffi_lib();
  if (!h) {
    hako_set_last_error("UNSUPPORTED");
    return hako_aot_set_err(err_out, "FFI library not found");
  }
  typedef int (*ffi_compile_fn)(const char*, const char*, char**);
  ffi_compile_fn fn = (ffi_compile_fn)dlsym(h, "hako_llvmc_compile_json");
  if (!fn) {
    dlclose(h);
    hako_set_last_error("UNSUPPORTED");
    return hako_aot_set_err(err_out, "FFI symbol missing: hako_llvmc_compile_json");
  }
  int rc = fn(json_in, obj_out, err_out);
  dlclose(h);
  return rc;
}

static int hako_aot_try_ffi_link(const char* obj_in, const char* exe_out, const char* extra_ldflags, char** err_out) {
  void* h = hako_aot_open_ffi_lib();
  if (!h) {
    hako_set_last_error("UNSUPPORTED");
    return hako_aot_set_err(err_out, "FFI library not found");
  }
  typedef int (*ffi_link_fn)(const char*, const char*, const char*, char**);
  ffi_link_fn fn = (ffi_link_fn)dlsym(h, "hako_llvmc_link_obj");
  if (!fn) {
    dlclose(h);
    hako_set_last_error("UNSUPPORTED");
    return hako_aot_set_err(err_out, "FFI symbol missing: hako_llvmc_link_obj");
  }
  int rc = fn(obj_in, exe_out, extra_ldflags, err_out);
  dlclose(h);
  return rc;
}
#else
static int hako_aot_try_ffi_compile(const char* json_in, const char* obj_out, char** err_out) {
  (void)json_in;
  (void)obj_out;
  hako_set_last_error("UNSUPPORTED");
  return hako_aot_set_err(err_out, "FFI unsupported on this platform");
}

static int hako_aot_try_ffi_link(const char* obj_in, const char* exe_out, const char* extra_ldflags, char** err_out) {
  (void)obj_in;
  (void)exe_out;
  (void)extra_ldflags;
  hako_set_last_error("UNSUPPORTED");
  return hako_aot_set_err(err_out, "FFI unsupported on this platform");
}
#endif

static const char* hako_aot_resolve_nyrt_dir(char* buf, size_t buflen) {
  const char* hint = getenv("NYASH_EMIT_EXE_NYRT");
  if (hint && *hint) {
    snprintf(buf, buflen, "%s", hint);
    return buf;
  }
  const char* a = "target/release";
  const char* b = "crates/hako_kernel/target/release";
  char path_a[1024];
  char path_b[1024];
  snprintf(path_a, sizeof(path_a), "%s/%s", a, "libhako_kernel.a");
  snprintf(path_b, sizeof(path_b), "%s/%s", b, "libhako_kernel.a");
  if (hako_aot_file_exists(path_a) || hako_aot_file_exists(path_b)) {
    snprintf(buf, buflen, "%s", hako_aot_file_exists(path_a) ? a : b);
    return buf;
  }
  snprintf(path_a, sizeof(path_a), "%s/%s", a, "libnyash_kernel.a");
  snprintf(path_b, sizeof(path_b), "%s/%s", b, "libnyash_kernel.a");
  if (hako_aot_file_exists(path_a) || hako_aot_file_exists(path_b)) {
    snprintf(buf, buflen, "%s", hako_aot_file_exists(path_a) ? a : b);
    return buf;
  }
  return NULL;
}

int hako_aot_compile_json(const char* json_in, const char* obj_out, char** err_out) {
  if (hako_aot_env_truthy(getenv("HAKO_AOT_USE_FFI"))) {
    return hako_aot_try_ffi_compile(json_in, obj_out, err_out);
  }
  if (!getenv("HAKO_LLVM_OPT_LEVEL")) {
    setenv("HAKO_LLVM_OPT_LEVEL", "0", 1);
  }
  if (!getenv("NYASH_LLVM_OPT_LEVEL")) {
    setenv("NYASH_LLVM_OPT_LEVEL", "0", 1);
  }
  if (!json_in || !*json_in || !obj_out || !*obj_out) {
    hako_set_last_error("VALIDATION");
    return hako_aot_set_err(err_out, "invalid args");
  }
  const char* llvmc = getenv("NYASH_NY_LLVM_COMPILER");
  if (!llvmc || !*llvmc) llvmc = "target/release/ny-llvmc";
  if (!hako_aot_file_exists(llvmc)) {
    hako_set_last_error("NOT_FOUND");
    return hako_aot_set_err(err_out, "ny-llvmc not found (NYASH_NY_LLVM_COMPILER)");
  }
  char logpath[1024];
  snprintf(logpath, sizeof(logpath), "%s/hako_aot_compile_%ld.log", hako_aot_tmp_dir_fallback(), (long)GETPID());
  char cmd[4096];
  int n = snprintf(cmd, sizeof(cmd), "\"%s\" --in \"%s\" --emit obj --out \"%s\" 2> \"%s\"", llvmc, json_in, obj_out, logpath);
  if (n <= 0 || (size_t)n >= sizeof(cmd)) {
    hako_set_last_error("VALIDATION");
    return hako_aot_set_err(err_out, "command too long");
  }
  {
    FILE* lf = fopen(logpath, "wb");
    if (lf) {
      fprintf(lf, "%s\n", cmd);
      fclose(lf);
    }
  }
  int rc = system(cmd);
  if (rc != 0) {
    hako_set_last_error("FAILED");
    char* first = hako_aot_read_first_line(logpath);
    if (first) {
      hako_aot_set_err(err_out, first);
      hako_mem_free(first);
    } else {
      hako_aot_set_err(err_out, "COMPILE_FAILED");
    }
    remove(logpath);
    return -1;
  }
  hako_set_last_error(NULL);
  if (!hako_aot_file_exists(obj_out)) {
    hako_set_last_error("FAILED");
    return hako_aot_set_err(err_out, "object not produced");
  }
  remove(logpath);
  return 0;
}

int hako_aot_link_obj(const char* obj_in, const char* exe_out, const char* extra_ldflags, char** err_out) {
  if (hako_aot_env_truthy(getenv("HAKO_AOT_USE_FFI"))) {
    return hako_aot_try_ffi_link(obj_in, exe_out, extra_ldflags, err_out);
  }
  if (!obj_in || !*obj_in || !exe_out || !*exe_out) {
    hako_set_last_error("VALIDATION");
    return hako_aot_set_err(err_out, "invalid args");
  }
  if (!hako_aot_file_exists(obj_in)) {
    hako_set_last_error("VALIDATION");
    return hako_aot_set_err(err_out, "object not found");
  }
  char nyrt_dir[1024];
  const char* dir = hako_aot_resolve_nyrt_dir(nyrt_dir, sizeof(nyrt_dir));
  if (!dir) {
    hako_set_last_error("NOT_FOUND");
    return hako_aot_set_err(err_out, "libhako_kernel.a not found (NYASH_EMIT_EXE_NYRT)");
  }
  char lib_a[1024];
  char lib_legacy[1024];
  snprintf(lib_a, sizeof(lib_a), "%s/libhako_kernel.a", dir);
  snprintf(lib_legacy, sizeof(lib_legacy), "%s/libnyash_kernel.a", dir);
  const char* lib = hako_aot_file_exists(lib_a) ? lib_a : lib_legacy;
  const char* linker = getenv("CC");
  if (!linker || !*linker) linker = "cc";
  const char* os_libs = "";
#if defined(__linux__)
  os_libs = "-ldl -lpthread -lm";
#elif defined(__APPLE__)
  os_libs = "";
#elif defined(_WIN32) || defined(__MINGW32__) || defined(__MINGW64__)
  os_libs = "-lws2_32 -lbcrypt";
#endif
  char logpath[1024];
  snprintf(logpath, sizeof(logpath), "%s/hako_aot_link_%ld.log", hako_aot_tmp_dir_fallback(), (long)GETPID());
  char shim_flag[1024] = "";
#if defined(__linux__) || defined(__APPLE__)
  char shim_so[1024];
#if defined(__APPLE__)
  snprintf(shim_so, sizeof(shim_so), "%s/libhako_kernel_shim.dylib", dir);
#else
  snprintf(shim_so, sizeof(shim_so), "%s/libhako_kernel_shim.so", dir);
#endif
  if (hako_aot_file_exists(shim_so)) {
    snprintf(shim_flag, sizeof(shim_flag), " -L\"%s\" -Wl,-rpath,\"%s\" -lhako_kernel_shim", dir, dir);
  }
#endif
  const char* pie_avoid = "";
#if defined(__linux__)
  pie_avoid = " -no-pie";
#endif
  char cmd[8192];
  int n = snprintf(cmd, sizeof(cmd), "\"%s\"%s -o \"%s\" \"%s\" -Wl,--whole-archive \"%s\" -Wl,--no-whole-archive%s %s 2> \"%s\"", linker, pie_avoid, exe_out, obj_in, lib, shim_flag, os_libs, logpath);
  if (n <= 0 || (size_t)n >= sizeof(cmd)) {
    hako_set_last_error("VALIDATION");
    return hako_aot_set_err(err_out, "command too long");
  }
  if (extra_ldflags && *extra_ldflags) {
    size_t cur = strlen(cmd);
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
  {
    const char* env_ld = getenv("HAKO_AOT_LDFLAGS");
    if (env_ld && *env_ld) {
      size_t cur = strlen(cmd);
      size_t rem = sizeof(cmd) - cur - 1;
      if (rem > 0) {
        strncat(cmd, " ", rem);
        cur += 1;
        rem = sizeof(cmd) - cur - 1;
      }
      if (rem > 0) {
        strncat(cmd, env_ld, rem);
      }
    }
  }
  int rc = system(cmd);
  if (rc != 0) {
    hako_set_last_error("FAILED");
    char* first = hako_aot_read_first_line(logpath);
    if (first) {
      hako_aot_set_err(err_out, first);
      hako_mem_free(first);
    } else {
      hako_aot_set_err(err_out, "LINK_FAILED");
    }
    remove(logpath);
    return -1;
  }
  hako_set_last_error(NULL);
  remove(logpath);
  return 0;
}
