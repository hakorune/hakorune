typedef struct hako_llvmc_entry_rune_selection {
  const char* symbol;
  const char* callconv;
} hako_llvmc_entry_rune_selection;

static void hako_llvmc_select_entry_runes(
    yyjson_val* fn0,
    hako_llvmc_entry_rune_selection* out);
static const char* hako_llvmc_entry_symbol_default(void);
static const char* hako_llvmc_effective_entry_symbol(
    const hako_llvmc_entry_rune_selection* selection);
static const char* hako_llvmc_entry_callconv_prefix(
    const hako_llvmc_entry_rune_selection* selection);
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

static int hako_llvmc_skip_indexof_line_seed_enabled(void) {
  const char* value = getenv("NYASH_LLVM_SKIP_INDEXOF_LINE_SEED");
  if (!value || !value[0]) return 0;
  return strcmp(value, "1") == 0 ||
         strcmp(value, "on") == 0 ||
         strcmp(value, "true") == 0 ||
         strcmp(value, "yes") == 0;
}

static int hako_llvmc_fast_enabled(void) {
  const char* value = getenv("NYASH_LLVM_FAST");
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

static void hako_llvmc_emit_route_trace(
    const char* stage,
    const char* result,
    const char* reason,
    const char* extra) {
  if (!hako_llvmc_route_trace_enabled()) return;
  fprintf(stderr,
          "[llvm-route/trace] stage=%s result=%s reason=%s extra=%s\n",
          (stage && stage[0]) ? stage : "unknown",
          (result && result[0]) ? result : "unknown",
          (reason && reason[0]) ? reason : "unknown",
          (extra && extra[0]) ? extra : "");
}

static int hako_llvmc_string_corridor_plan_matches(
    yyjson_val* fn,
    long long value,
    const char* kind,
    long long source_root,
    long long start,
    long long end) {
  yyjson_val* metadata;
  yyjson_val* candidates;
  yyjson_val* arr;
  size_t i;
  char key[32];

  if (!(fn && kind && value > 0 && source_root > 0 && start > 0 && end > 0)) {
    return 0;
  }

  metadata = yyjson_obj_get(fn, "metadata");
  candidates = metadata ? yyjson_obj_get(metadata, "string_corridor_candidates") : NULL;
  if (!(candidates && yyjson_is_obj(candidates))) return 0;

  snprintf(key, sizeof(key), "%lld", value);
  arr = yyjson_obj_get(candidates, key);
  if (!(arr && yyjson_is_arr(arr))) return 0;

  for (i = 0; i < yyjson_arr_size(arr); i++) {
    yyjson_val* candidate = yyjson_arr_get(arr, i);
    yyjson_val* plan = candidate ? yyjson_obj_get(candidate, "plan") : NULL;
    const char* candidate_kind =
        candidate ? yyjson_get_str(yyjson_obj_get(candidate, "kind")) : NULL;
    if (!(candidate_kind && strcmp(candidate_kind, kind) == 0)) continue;
    if (!(plan && yyjson_is_obj(plan))) continue;
    if ((long long)yyjson_get_sint(yyjson_obj_get(plan, "corridor_root")) != value) continue;
    if ((long long)yyjson_get_sint(yyjson_obj_get(plan, "source_root")) != source_root) continue;
    if ((long long)yyjson_get_sint(yyjson_obj_get(plan, "start")) != start) continue;
    if ((long long)yyjson_get_sint(yyjson_obj_get(plan, "end")) != end) continue;
    return 1;
  }

  return 0;
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

static int hako_llvmc_file_exists(const char* path);

static int hako_llvmc_tool_exists(const char* tool) {
  const char* path;
  char candidate[4096];
  const char* cursor;
  const char* sep;
  size_t dir_len;

  if (!tool || !tool[0]) return 0;
  if (strchr(tool, '/')) {
#if defined(_WIN32)
    return _access(tool, 0) == 0;
#else
    return access(tool, X_OK) == 0;
#endif
  }

  path = getenv("PATH");
  if (!path || !path[0]) return 0;
  cursor = path;
  while (cursor && *cursor) {
    sep = strchr(cursor, ':');
    dir_len = sep ? (size_t)(sep - cursor) : strlen(cursor);
    if (dir_len == 0) {
      snprintf(candidate, sizeof(candidate), "%s", tool);
    } else {
      snprintf(candidate, sizeof(candidate), "%.*s/%s", (int)dir_len, cursor, tool);
    }
    if (candidate[0]) {
#if defined(_WIN32)
      if (_access(candidate, 0) == 0) return 1;
#else
      if (access(candidate, X_OK) == 0) return 1;
#endif
    }
    if (!sep) break;
    cursor = sep + 1;
  }
  return 0;
}

static const char* hako_llvmc_resolve_tool(const char* env_key, const char* primary, const char* fallback) {
  const char* configured = env_key ? getenv(env_key) : NULL;
  if (configured && configured[0] && hako_llvmc_tool_exists(configured)) {
    return configured;
  }
  if (hako_llvmc_tool_exists(primary)) return primary;
  if (fallback && hako_llvmc_tool_exists(fallback)) return fallback;
  return NULL;
}

static const char* hako_llvmc_llc_flags(void) {
  const char* value = getenv("NYASH_NY_LLVM_LLC_FLAGS");
  if (value && value[0]) {
    return value;
  }
  return "-O3 -mcpu=native";
}

static int hako_llvmc_mem2reg_canonicalize_and_llc(
    const char* llpath,
    const char* obj_out) {
  char canon_ll[1024];
  char cmd[2048];
  const char* opt_tool;
  const char* llc_tool;
  const char* llc_flags;
  int n;
  int rc;

  if (!llpath || !llpath[0] || !obj_out || !obj_out[0]) {
    hako_llvmc_emit_route_trace("mem2reg", "fail", "missing-path", "");
    return -1;
  }

  n = snprintf(
      canon_ll,
      sizeof(canon_ll),
      "%s/hako_pure_mem2reg_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  if (n <= 0 || (size_t)n >= sizeof(canon_ll)) {
    hako_llvmc_emit_route_trace("mem2reg", "fail", "tmp-path-too-long", llpath);
    return -1;
  }

  opt_tool = hako_llvmc_resolve_tool("NYASH_NY_LLVM_OPT_TOOL", "opt", "opt-18");
  llc_tool = hako_llvmc_resolve_tool("NYASH_NY_LLVM_LLC_TOOL", "llc", "llc-18");
  llc_flags = hako_llvmc_llc_flags();
  if (!opt_tool) {
    hako_llvmc_emit_route_trace("mem2reg", "fail", "opt-missing", llpath);
    return -1;
  }
  if (!llc_tool) {
    hako_llvmc_emit_route_trace("llc", "fail", "llc-missing", llpath);
    return -1;
  }

  remove(canon_ll);
  n = snprintf(
      cmd,
      sizeof(cmd),
      "%s -passes=mem2reg -S -o \"%s\" \"%s\" 2>/dev/null",
      opt_tool,
      canon_ll,
      llpath);
  if (n <= 0 || (size_t)n >= sizeof(cmd)) {
    remove(canon_ll);
    hako_llvmc_emit_route_trace("mem2reg", "fail", "opt-cmd-too-long", llpath);
    return -1;
  }

  rc = system(cmd);
  if (rc != 0 || !hako_llvmc_file_exists(canon_ll)) {
    remove(canon_ll);
    hako_llvmc_emit_route_trace("mem2reg", "fail", "opt", llpath);
    return -1;
  }

  hako_llvmc_maybe_dump_ir_file(canon_ll);
  n = snprintf(
      cmd,
      sizeof(cmd),
      "%s %s -filetype=obj -o \"%s\" \"%s\" 2>/dev/null",
      llc_tool,
      llc_flags,
      obj_out,
      canon_ll);
  if (n <= 0 || (size_t)n >= sizeof(cmd)) {
    remove(canon_ll);
    hako_llvmc_emit_route_trace("llc", "fail", "cmd-too-long", canon_ll);
    return -1;
  }

  rc = system(cmd);
  remove(canon_ll);
  if (rc != 0) {
    hako_llvmc_emit_route_trace("llc", "fail", "mem2reg", llpath);
    return -1;
  }
  return 0;
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
