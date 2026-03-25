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
static void* hako_aot_open_ffi_lib_path(const char* lib_path) {
  if (!lib_path || !*lib_path) {
    return NULL;
  }
  return dlopen(lib_path, RTLD_NOW);
}

static void* hako_aot_try_open_default_ffi_lib(char* buf, size_t buflen, const char* relpath) {
  int n = snprintf(buf, buflen, "%s", relpath);
  if (n <= 0 || (size_t)n >= buflen) {
    return NULL;
  }
  return hako_aot_open_ffi_lib_path(buf);
}

static void* hako_aot_open_ffi_lib(void) {
  const char* lib = getenv("HAKO_AOT_FFI_LIB");
  char buf[1024];
  if (lib && *lib) {
    return hako_aot_open_ffi_lib_path(lib);
  }
  void* h = hako_aot_try_open_default_ffi_lib(
      buf,
      sizeof(buf),
      "target/release/libhako_llvmc_ffi.so");
  if (!h) {
    h = hako_aot_try_open_default_ffi_lib(
        buf,
        sizeof(buf),
        "lib/libhako_llvmc_ffi.so");
  }
  return h;
}
#endif

static int hako_aot_use_ffi(void) {
  return hako_aot_env_truthy(getenv("HAKO_AOT_USE_FFI"));
}

static void hako_aot_ensure_default_opt_env(void) {
  if (!getenv("HAKO_LLVM_OPT_LEVEL")) {
    setenv("HAKO_LLVM_OPT_LEVEL", "0", 1);
  }
  if (!getenv("NYASH_LLVM_OPT_LEVEL")) {
    setenv("NYASH_LLVM_OPT_LEVEL", "0", 1);
  }
}

static int hako_aot_build_compile_log_path(char* log_path, size_t log_path_len) {
  int n = snprintf(
      log_path,
      log_path_len,
      "%s/hako_aot_compile_%ld.log",
      hako_aot_tmp_dir_fallback(),
      (long)GETPID());
  return (n <= 0 || (size_t)n >= log_path_len) ? -1 : 0;
}

static int hako_aot_build_link_log_path(char* log_path, size_t log_path_len) {
  int n = snprintf(
      log_path,
      log_path_len,
      "%s/hako_aot_link_%ld.log",
      hako_aot_tmp_dir_fallback(),
      (long)GETPID());
  return (n <= 0 || (size_t)n >= log_path_len) ? -1 : 0;
}

static int hako_aot_build_compile_command(
    char* cmd,
    size_t cmd_len,
    const char* llvmc,
    const char* mir_json_path,
    const char* obj_path,
    const char* log_path) {
  // Generic AOT transport stays on the explicit harness keep lane to avoid
  // re-entering the boundary-owned default route.
  int n = snprintf(
      cmd,
      cmd_len,
      "\"%s\" --driver harness --in \"%s\" --emit obj --out \"%s\" 2> \"%s\"",
      llvmc,
      mir_json_path,
      obj_path,
      log_path);
  return (n <= 0 || (size_t)n >= cmd_len) ? -1 : 0;
}

static int hako_aot_append_flag_string(char* cmd, size_t cmd_len, const char* flags) {
  if (!flags || !*flags) {
    return 0;
  }
  size_t cur = strlen(cmd);
  int n = snprintf(cmd + cur, cmd_len - cur, " %s", flags);
  return (n <= 0 || (size_t)n >= (cmd_len - cur)) ? -1 : 0;
}

static int hako_aot_build_link_command(
    char* cmd,
    size_t cmd_len,
    const char* linker,
    const char* pie_avoid,
    const char* gc_sections_flag,
    const char* exe_path,
    const char* obj_path,
    const char* runtime_lib_path,
    const char* shim_flag,
    const char* os_libs,
    const char* log_path) {
  int n = snprintf(
      cmd,
      cmd_len,
      "\"%s\"%s%s -o \"%s\" \"%s\" -Wl,--whole-archive \"%s\" -Wl,--no-whole-archive%s %s 2> \"%s\"",
      linker,
      pie_avoid,
      gc_sections_flag,
      exe_path,
      obj_path,
      runtime_lib_path,
      shim_flag,
      os_libs,
      log_path);
  return (n <= 0 || (size_t)n >= cmd_len) ? -1 : 0;
}

static int hako_aot_write_command_seed(const char* log_path, const char* cmd) {
  FILE* lf = fopen(log_path, "wb");
  if (!lf) {
    return -1;
  }
  fprintf(lf, "%s\n", cmd);
  fclose(lf);
  return 0;
}

static int hako_aot_set_log_first_line_or(char** err_out, const char* log_path, const char* fallback) {
  char* first = hako_aot_read_first_line(log_path);
  if (first) {
    hako_aot_set_err(err_out, first);
    hako_mem_free(first);
  } else {
    hako_aot_set_err(err_out, fallback);
  }
  return -1;
}

static void hako_aot_cleanup_log_path(const char* log_path) {
  if (log_path && *log_path) {
    remove(log_path);
  }
}

static int hako_aot_finish_failed_command(
    char** err_out,
    const char* log_path,
    const char* fallback) {
  hako_set_last_error("FAILED");
  hako_aot_set_log_first_line_or(err_out, log_path, fallback);
  hako_aot_cleanup_log_path(log_path);
  return -1;
}

static int hako_aot_finish_success_command(const char* log_path) {
  hako_set_last_error(NULL);
  hako_aot_cleanup_log_path(log_path);
  return 0;
}

static int hako_aot_run_command_or_project_fail(
    const char* cmd,
    char** err_out,
    const char* log_path,
    const char* fallback) {
  int rc = system(cmd);
  if (rc != 0) {
    return hako_aot_finish_failed_command(err_out, log_path, fallback);
  }
  return 0;
}

static int hako_aot_finish_compile_success(
    const char* obj_path,
    char** err_out,
    const char* log_path) {
  if (!hako_aot_file_exists(obj_path)) {
    hako_set_last_error("FAILED");
    hako_aot_cleanup_log_path(log_path);
    return hako_aot_set_err(err_out, "object not produced");
  }
  return hako_aot_finish_success_command(log_path);
}

static int hako_aot_pick_existing_dir(
    char* out,
    size_t out_len,
    const char* primary_dir,
    const char* secondary_dir,
    const char* lib_name) {
  char primary_path[1024];
  char secondary_path[1024];
  snprintf(primary_path, sizeof(primary_path), "%s/%s", primary_dir, lib_name);
  snprintf(secondary_path, sizeof(secondary_path), "%s/%s", secondary_dir, lib_name);
  if (hako_aot_file_exists(primary_path)) {
    snprintf(out, out_len, "%s", primary_dir);
    return 1;
  }
  if (hako_aot_file_exists(secondary_path)) {
    snprintf(out, out_len, "%s", secondary_dir);
    return 1;
  }
  return 0;
}

static const char* hako_aot_resolve_runtime_archive(char* buf, size_t buflen, const char* dir) {
  char primary_path[1024];
  char legacy_path[1024];
  snprintf(primary_path, sizeof(primary_path), "%s/libhako_kernel.a", dir);
  if (hako_aot_file_exists(primary_path)) {
    snprintf(buf, buflen, "%s", primary_path);
    return buf;
  }
  snprintf(legacy_path, sizeof(legacy_path), "%s/libnyash_kernel.a", dir);
  if (hako_aot_file_exists(legacy_path)) {
    snprintf(buf, buflen, "%s", legacy_path);
    return buf;
  }
  return NULL;
}

static const char* hako_aot_link_os_libs(void) {
#if defined(__linux__)
  return "-ldl -lpthread -lm";
#elif defined(__APPLE__)
  return "";
#elif defined(_WIN32) || defined(__MINGW32__) || defined(__MINGW64__)
  return "-lws2_32 -lbcrypt";
#else
  return "";
#endif
}

static const char* hako_aot_link_pie_avoid_flag(void) {
#if defined(__linux__)
  return " -no-pie";
#else
  return "";
#endif
}

static const char* hako_aot_link_gc_sections_flag(void) {
#if defined(__linux__)
  return " -Wl,--gc-sections";
#else
  return "";
#endif
}

static const char* hako_aot_resolve_shim_flag(char* buf, size_t buflen, const char* dir) {
  buf[0] = '\0';
#if defined(__linux__) || defined(__APPLE__)
  char shim_so[1024];
#if defined(__APPLE__)
  snprintf(shim_so, sizeof(shim_so), "%s/libhako_kernel_shim.dylib", dir);
#else
  snprintf(shim_so, sizeof(shim_so), "%s/libhako_kernel_shim.so", dir);
#endif
  if (hako_aot_file_exists(shim_so)) {
    snprintf(buf, buflen, " -L\"%s\" -Wl,-rpath,\"%s\" -lhako_kernel_shim", dir, dir);
  }
#else
  (void)dir;
#endif
  return buf;
}

static int hako_aot_append_linker_option_keeps(
    char* cmd,
    size_t cmd_len,
    const char* extra_ldflags) {
  if (hako_aot_append_flag_string(cmd, cmd_len, extra_ldflags) != 0) {
    return -1;
  }
  {
    const char* env_ld = getenv("HAKO_AOT_LDFLAGS");
    if (hako_aot_append_flag_string(cmd, cmd_len, env_ld) != 0) {
      return -1;
    }
  }
  return 0;
}

#if !defined(_WIN32)
static int hako_aot_try_ffi_compile(const char* mir_json_path, const char* obj_path, char** err_out) {
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
  int rc = fn(mir_json_path, obj_path, err_out);
  dlclose(h);
  return rc;
}

static int hako_aot_try_ffi_link(const char* obj_path, const char* exe_path, const char* extra_ldflags, char** err_out) {
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
  int rc = fn(obj_path, exe_path, extra_ldflags, err_out);
  dlclose(h);
  return rc;
}
#else
static int hako_aot_try_ffi_compile(const char* mir_json_path, const char* obj_path, char** err_out) {
  (void)mir_json_path;
  (void)obj_path;
  hako_set_last_error("UNSUPPORTED");
  return hako_aot_set_err(err_out, "FFI unsupported on this platform");
}

static int hako_aot_try_ffi_link(const char* obj_path, const char* exe_path, const char* extra_ldflags, char** err_out) {
  (void)obj_path;
  (void)exe_path;
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
  if (hako_aot_pick_existing_dir(buf, buflen, a, b, "libhako_kernel.a")) {
    return buf;
  }
  if (hako_aot_pick_existing_dir(buf, buflen, a, b, "libnyash_kernel.a")) {
    return buf;
  }
  return NULL;
}

int hako_aot_compile_json(const char* mir_json_path, const char* obj_path, char** err_out) {
  if (hako_aot_use_ffi()) {
    return hako_aot_try_ffi_compile(mir_json_path, obj_path, err_out);
  }
  hako_aot_ensure_default_opt_env();
  if (!mir_json_path || !*mir_json_path || !obj_path || !*obj_path) {
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
  if (hako_aot_build_compile_log_path(logpath, sizeof(logpath)) != 0) {
    hako_set_last_error("VALIDATION");
    return hako_aot_set_err(err_out, "command too long");
  }
  char cmd[4096];
  if (hako_aot_build_compile_command(cmd, sizeof(cmd), llvmc, mir_json_path, obj_path, logpath) != 0) {
    hako_set_last_error("VALIDATION");
    return hako_aot_set_err(err_out, "command too long");
  }
  (void)hako_aot_write_command_seed(logpath, cmd);
  if (hako_aot_run_command_or_project_fail(
          cmd,
          err_out,
          logpath,
          "COMPILE_FAILED") != 0) {
    return -1;
  }
  return hako_aot_finish_compile_success(obj_path, err_out, logpath);
}

int hako_aot_link_obj(const char* obj_path, const char* exe_path, const char* extra_ldflags, char** err_out) {
  if (hako_aot_use_ffi()) {
    return hako_aot_try_ffi_link(obj_path, exe_path, extra_ldflags, err_out);
  }
  if (!obj_path || !*obj_path || !exe_path || !*exe_path) {
    hako_set_last_error("VALIDATION");
    return hako_aot_set_err(err_out, "invalid args");
  }
  if (!hako_aot_file_exists(obj_path)) {
    hako_set_last_error("VALIDATION");
    return hako_aot_set_err(err_out, "object not found");
  }
  char nyrt_dir[1024];
  const char* dir = hako_aot_resolve_nyrt_dir(nyrt_dir, sizeof(nyrt_dir));
  if (!dir) {
    hako_set_last_error("NOT_FOUND");
    return hako_aot_set_err(err_out, "libhako_kernel.a not found (NYASH_EMIT_EXE_NYRT)");
  }
  char runtime_archive[1024];
  const char* lib = hako_aot_resolve_runtime_archive(runtime_archive, sizeof(runtime_archive), dir);
  if (!lib) {
    hako_set_last_error("NOT_FOUND");
    return hako_aot_set_err(err_out, "libhako_kernel.a not found (NYASH_EMIT_EXE_NYRT)");
  }
  const char* linker = getenv("CC");
  if (!linker || !*linker) linker = "cc";
  char logpath[1024];
  if (hako_aot_build_link_log_path(logpath, sizeof(logpath)) != 0) {
    hako_set_last_error("VALIDATION");
    return hako_aot_set_err(err_out, "command too long");
  }
  char shim_flag[1024] = "";
  char cmd[8192];
  if (hako_aot_build_link_command(
          cmd,
          sizeof(cmd),
          linker,
          hako_aot_link_pie_avoid_flag(),
          hako_aot_link_gc_sections_flag(),
          exe_path,
          obj_path,
          lib,
          hako_aot_resolve_shim_flag(shim_flag, sizeof(shim_flag), dir),
          hako_aot_link_os_libs(),
          logpath) != 0) {
    hako_set_last_error("VALIDATION");
    return hako_aot_set_err(err_out, "command too long");
  }
  if (hako_aot_append_linker_option_keeps(cmd, sizeof(cmd), extra_ldflags) != 0) {
    hako_set_last_error("VALIDATION");
    return hako_aot_set_err(err_out, "command too long");
  }
  if (hako_aot_run_command_or_project_fail(
          cmd,
          err_out,
          logpath,
          "LINK_FAILED") != 0) {
    return -1;
  }
  return hako_aot_finish_success_command(logpath);
}
