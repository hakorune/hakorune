typedef struct hako_llvmc_entry_rune_selection {
  const char* symbol;
  const char* callconv;
} hako_llvmc_entry_rune_selection;

static void hako_llvmc_select_entry_runes(
    yyjson_val* fn0,
    hako_llvmc_entry_rune_selection* out);
static void hako_llvmc_emit_entry_header(
    FILE* f,
    const hako_llvmc_entry_rune_selection* selection);
static void hako_llvmc_emit_entry_alias_if_needed(
    FILE* f,
    const hako_llvmc_entry_rune_selection* selection);

static const char* hako_llvmc_backend_compile_recipe(void) {
  return getenv("HAKO_BACKEND_COMPILE_RECIPE");
}

static const char* hako_llvmc_backend_compat_replay(void) {
  return getenv("HAKO_BACKEND_COMPAT_REPLAY");
}

static int capi_pure_enabled(void) {
  const char* recipe = hako_llvmc_backend_compile_recipe();
  if (recipe && recipe[0]) {
    return strcmp(recipe, "pure-first") == 0;
  }
  const char* v = getenv("HAKO_CAPI_PURE");
  return (v && v[0] == '1');
}

static int compat_harness_replay_enabled(void) {
  const char* replay = hako_llvmc_backend_compat_replay();
  if (replay && replay[0]) {
    return strcmp(replay, "harness") == 0;
  }
  return 0;
}

static int hako_llvmc_route_trace_enabled(void) {
  const char* value = getenv("NYASH_LLVM_ROUTE_TRACE");
  if (!value || !value[0]) return 0;
  return strcmp(value, "1") == 0 ||
         strcmp(value, "on") == 0 ||
         strcmp(value, "true") == 0 ||
         strcmp(value, "yes") == 0;
}

static void hako_llvmc_emit_route_replay(const char* lane, const char* reason) {
  if (!hako_llvmc_route_trace_enabled()) return;
  fprintf(stderr, "[llvm-route/replay] lane=%s reason=%s\n",
          (lane && lane[0]) ? lane : "unknown",
          (reason && reason[0]) ? reason : "unknown");
}

static int set_err_owned(char** err_out, const char* msg) {
  if (!err_out) return -1;
  if (!msg) { *err_out = NULL; return -1; }
  size_t n = strlen(msg);
  char* p = (char*)malloc(n + 1);
  if (!p) { *err_out = NULL; return -1; }
  memcpy(p, msg, n + 1);
  *err_out = p;
  return -1;
}

static void hako_llvmc_set_env_value(const char* key, const char* value) {
#if defined(_WIN32)
  _putenv_s(key, value ? value : "");
#else
  if (value) {
    setenv(key, value, 1);
  } else {
    unsetenv(key);
  }
#endif
}

static const char* hako_llvmc_tmp_dir_fallback(void) {
  const char* tmp = getenv("TMPDIR");
  return (tmp && tmp[0]) ? tmp : "/tmp";
}

static void hako_llvmc_maybe_dump_ir_file(const char* llpath) {
  const char* dump_path = getenv("NYASH_LLVM_DUMP_IR");
  FILE* src;
  FILE* dst;
  char buf[4096];
  size_t n;

  if (!llpath || !llpath[0] || !dump_path || !dump_path[0]) return;

  src = fopen(llpath, "rb");
  if (!src) return;
  dst = fopen(dump_path, "wb");
  if (!dst) {
    fclose(src);
    return;
  }

  while ((n = fread(buf, 1, sizeof(buf), src)) > 0) {
    if (fwrite(buf, 1, n, dst) != n) {
      break;
    }
  }

  fclose(dst);
  fclose(src);
}

static int hako_llvmc_file_exists(const char* path) {
  FILE* f;
  if (!path || !path[0]) return 0;
  f = fopen(path, "rb");
  if (!f) return 0;
  fclose(f);
  return 1;
}

static char* hako_llvmc_read_first_line_owned(const char* path) {
  FILE* f = fopen(path, "rb");
  char buf[512];
  size_t n;
  char* out;
  if (!f) return NULL;
  if (!fgets(buf, sizeof(buf), f)) {
    fclose(f);
    return NULL;
  }
  fclose(f);
  n = strcspn(buf, "\r\n");
  buf[n] = '\0';
  if (!buf[0]) return NULL;
  out = (char*)malloc(n + 1);
  if (!out) return NULL;
  memcpy(out, buf, n + 1);
  return out;
}

static int hako_llvmc_build_harness_log_path(char* log_path, size_t log_path_len) {
  int n = snprintf(
      log_path,
      log_path_len,
      "%s/hako_llvmc_harness_compile_%ld.log",
      hako_llvmc_tmp_dir_fallback(),
      (long)getpid());
  return (n <= 0 || (size_t)n >= log_path_len) ? -1 : 0;
}

static int hako_llvmc_ascii_strlen(const char* s, long long* out_len) {
  size_t n = 0;
  if (!s) return 0;
  while (s[n]) {
    if (((unsigned char)s[n]) & 0x80) {
      return 0;
    }
    n++;
  }
  if (out_len) {
    *out_len = (long long)n;
  }
  return 1;
}

static int hako_llvmc_ascii_index_of(const char* haystack, const char* needle, long long* out_idx) {
  size_t hlen = 0;
  size_t nlen = 0;
  size_t i = 0;
  if (!hako_llvmc_ascii_strlen(haystack, (long long*)&hlen)) return 0;
  if (!hako_llvmc_ascii_strlen(needle, (long long*)&nlen)) return 0;
  if (nlen == 0) {
    if (out_idx) *out_idx = 0;
    return 1;
  }
  if (nlen > hlen) {
    if (out_idx) *out_idx = -1;
    return 1;
  }
  for (i = 0; i + nlen <= hlen; i++) {
    if (memcmp(haystack + i, needle, nlen) == 0) {
      if (out_idx) *out_idx = (long long)i;
      return 1;
    }
  }
  if (out_idx) *out_idx = -1;
  return 1;
}

static int hako_llvmc_str_eq(const char* lhs, const char* rhs) {
  return lhs && rhs && strcmp(lhs, rhs) == 0;
}

static void hako_llvmc_emit_llvm_byte_string(FILE* f, const char* s, long long len) {
  long long i;
  for (i = 0; i < len; i++) {
    fprintf(f, "\\%02X", (unsigned char)s[i]);
  }
}

