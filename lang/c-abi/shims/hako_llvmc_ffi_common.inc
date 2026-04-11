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

static int hako_llvmc_string_corridor_read_plan_source_window_fn(
    yyjson_val* fn,
    long long value,
    const char* kind,
    long long* source_root,
    long long* start,
    long long* end) {
  yyjson_val* metadata;
  yyjson_val* candidates;
  yyjson_val* arr;
  size_t i;
  char key[32];

  if (!(fn && kind && value > 0)) return 0;

  metadata = yyjson_obj_get(fn, "metadata");
  candidates = metadata ? yyjson_obj_get(metadata, "string_corridor_candidates") : NULL;
  if (!(candidates && yyjson_is_obj(candidates))) return 0;

  snprintf(key, sizeof(key), "%lld", value);
  arr = yyjson_obj_get(candidates, key);
  if (!(arr && yyjson_is_arr(arr))) return 0;

  for (i = 0; i < yyjson_arr_size(arr); i++) {
    yyjson_val* candidate = yyjson_arr_get(arr, i);
    yyjson_val* candidate_kind = candidate ? yyjson_obj_get(candidate, "kind") : NULL;
    yyjson_val* plan = candidate ? yyjson_obj_get(candidate, "plan") : NULL;
    yyjson_val* source_val = plan ? yyjson_obj_get(plan, "source_root") : NULL;
    yyjson_val* start_val = plan ? yyjson_obj_get(plan, "start") : NULL;
    yyjson_val* end_val = plan ? yyjson_obj_get(plan, "end") : NULL;

    if (!(candidate_kind && yyjson_is_str(candidate_kind) &&
          strcmp(yyjson_get_str(candidate_kind), kind) == 0)) {
      continue;
    }
    if (!(plan && yyjson_is_obj(plan) && source_val && start_val && end_val &&
          yyjson_is_num(source_val) && yyjson_is_num(start_val) &&
          yyjson_is_num(end_val))) {
      return 0;
    }
    if (source_root) *source_root = (long long)yyjson_get_sint(source_val);
    if (start) *start = (long long)yyjson_get_sint(start_val);
    if (end) *end = (long long)yyjson_get_sint(end_val);
    return 1;
  }

  return 0;
}

static int hako_llvmc_string_corridor_has_candidate(
    yyjson_val* fn,
    long long value,
    const char* kind) {
  yyjson_val* metadata;
  yyjson_val* candidates;
  yyjson_val* arr;
  size_t i;
  char key[32];

  if (!(fn && kind && value > 0)) {
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
    const char* candidate_kind =
        candidate ? yyjson_get_str(yyjson_obj_get(candidate, "kind")) : NULL;
    if (candidate_kind && strcmp(candidate_kind, kind) == 0) {
      return 1;
    }
  }

  return 0;
}

static int hako_llvmc_string_corridor_has_relation_fn(
    yyjson_val* fn,
    long long value,
    const char* kind,
    long long base_value,
    long long witness_value,
    const char* window_contract) {
  yyjson_val* metadata;
  yyjson_val* relations;
  yyjson_val* arr;
  size_t i;
  char key[32];

  if (!(fn && kind && value > 0)) return 0;

  metadata = yyjson_obj_get(fn, "metadata");
  relations = metadata ? yyjson_obj_get(metadata, "string_corridor_relations") : NULL;
  if (!(relations && yyjson_is_obj(relations))) return 0;

  snprintf(key, sizeof(key), "%lld", value);
  arr = yyjson_obj_get(relations, key);
  if (!(arr && yyjson_is_arr(arr))) return 0;

  for (i = 0; i < yyjson_arr_size(arr); i++) {
    yyjson_val* relation = yyjson_arr_get(arr, i);
    yyjson_val* kind_val = relation ? yyjson_obj_get(relation, "kind") : NULL;
    yyjson_val* base_val = relation ? yyjson_obj_get(relation, "base_value") : NULL;
    yyjson_val* witness_val = relation ? yyjson_obj_get(relation, "witness_value") : NULL;
    yyjson_val* window_val = relation ? yyjson_obj_get(relation, "window_contract") : NULL;

    if (!(kind_val && yyjson_is_str(kind_val) &&
          strcmp(yyjson_get_str(kind_val), kind) == 0)) {
      continue;
    }
    if (base_value > 0) {
      if (!(base_val && yyjson_is_num(base_val) &&
            (long long)yyjson_get_sint(base_val) == base_value)) {
        continue;
      }
    }
    if (witness_value > 0) {
      if (!(witness_val && yyjson_is_num(witness_val) &&
            (long long)yyjson_get_sint(witness_val) == witness_value)) {
        continue;
      }
    }
    if (window_contract) {
      if (!(window_val && yyjson_is_str(window_val) &&
            strcmp(yyjson_get_str(window_val), window_contract) == 0)) {
        continue;
      }
    }
    return 1;
  }

  return 0;
}

static int hako_llvmc_string_corridor_read_relation_details_fn(
    yyjson_val* fn,
    long long value,
    const char* kind,
    long long* base_value,
    long long* witness_value,
    const char** window_contract) {
  yyjson_val* metadata;
  yyjson_val* relations;
  yyjson_val* arr;
  size_t i;
  char key[32];

  if (!(fn && kind && value > 0)) return 0;

  metadata = yyjson_obj_get(fn, "metadata");
  relations = metadata ? yyjson_obj_get(metadata, "string_corridor_relations") : NULL;
  if (!(relations && yyjson_is_obj(relations))) return 0;

  snprintf(key, sizeof(key), "%lld", value);
  arr = yyjson_obj_get(relations, key);
  if (!(arr && yyjson_is_arr(arr))) return 0;

  for (i = 0; i < yyjson_arr_size(arr); i++) {
    yyjson_val* relation = yyjson_arr_get(arr, i);
    yyjson_val* kind_val = relation ? yyjson_obj_get(relation, "kind") : NULL;
    yyjson_val* base_val = relation ? yyjson_obj_get(relation, "base_value") : NULL;
    yyjson_val* witness_val = relation ? yyjson_obj_get(relation, "witness_value") : NULL;
    yyjson_val* window_val = relation ? yyjson_obj_get(relation, "window_contract") : NULL;

    if (!(kind_val && yyjson_is_str(kind_val) &&
          strcmp(yyjson_get_str(kind_val), kind) == 0)) {
      continue;
    }
    if (base_value) {
      *base_value =
          (base_val && yyjson_is_num(base_val))
              ? (long long)yyjson_get_sint(base_val)
              : 0;
    }
    if (witness_value) {
      *witness_value =
          (witness_val && yyjson_is_num(witness_val))
              ? (long long)yyjson_get_sint(witness_val)
              : 0;
    }
    if (window_contract) {
      *window_contract =
          (window_val && yyjson_is_str(window_val)) ? yyjson_get_str(window_val)
                                                    : NULL;
    }
    return 1;
  }

  return 0;
}

static yyjson_val* hako_llvmc_string_kernel_plan_by_corridor_root_fn(
    yyjson_val* fn,
    long long corridor_root) {
  yyjson_val* metadata;
  yyjson_val* plans;
  char key[32];

  if (!(fn && corridor_root > 0)) return NULL;
  metadata = yyjson_obj_get(fn, "metadata");
  plans = metadata ? yyjson_obj_get(metadata, "string_kernel_plans") : NULL;
  if (!(plans && yyjson_is_obj(plans))) return NULL;

  snprintf(key, sizeof(key), "%lld", corridor_root);
  plans = yyjson_obj_get(plans, key);
  if (!(plans && yyjson_is_obj(plans))) return NULL;
  return plans;
}

static int hako_llvmc_string_kernel_plan_read_concat_triplet_len_fn(
    yyjson_val* fn,
    long long corridor_root,
    long long* source_root,
    long long* middle_value,
    long long* middle_known_length) {
  yyjson_val* plan = hako_llvmc_string_kernel_plan_by_corridor_root_fn(fn, corridor_root);
  yyjson_val* parts = plan ? yyjson_obj_get(plan, "parts") : NULL;
  yyjson_val* legality = plan ? yyjson_obj_get(plan, "legality") : NULL;
  yyjson_val* first_part = parts ? yyjson_arr_get(parts, 0) : NULL;
  yyjson_val* middle_part = parts ? yyjson_arr_get(parts, 1) : NULL;
  yyjson_val* last_part = parts ? yyjson_arr_get(parts, 2) : NULL;
  yyjson_val* source_root_val = plan ? yyjson_obj_get(plan, "source_root") : NULL;
  yyjson_val* family_val = plan ? yyjson_obj_get(plan, "family") : NULL;
  yyjson_val* consumer_val = plan ? yyjson_obj_get(plan, "consumer") : NULL;
  yyjson_val* retained_form_val = plan ? yyjson_obj_get(plan, "retained_form") : NULL;
  yyjson_val* byte_exact_val = legality ? yyjson_obj_get(legality, "byte_exact") : NULL;
  yyjson_val* first_kind = first_part ? yyjson_obj_get(first_part, "kind") : NULL;
  yyjson_val* middle_kind = middle_part ? yyjson_obj_get(middle_part, "kind") : NULL;
  yyjson_val* last_kind = last_part ? yyjson_obj_get(last_part, "kind") : NULL;
  yyjson_val* first_source = first_part ? yyjson_obj_get(first_part, "source") : NULL;
  yyjson_val* last_source = last_part ? yyjson_obj_get(last_part, "source") : NULL;
  yyjson_val* middle_value_val = middle_part ? yyjson_obj_get(middle_part, "value") : NULL;
  yyjson_val* middle_len_val =
      middle_part ? yyjson_obj_get(middle_part, "known_length") : NULL;

  if (!(plan && parts && yyjson_is_arr(parts) && yyjson_arr_size(parts) == 3 &&
        source_root_val && yyjson_is_num(source_root_val) &&
        family_val && yyjson_is_str(family_val) &&
        consumer_val && yyjson_is_str(consumer_val) &&
        retained_form_val && yyjson_is_str(retained_form_val) &&
        byte_exact_val && yyjson_is_bool(byte_exact_val) &&
        yyjson_get_bool(byte_exact_val) &&
        first_kind && yyjson_is_str(first_kind) &&
        middle_kind && yyjson_is_str(middle_kind) &&
        last_kind && yyjson_is_str(last_kind) &&
        !strcmp(yyjson_get_str(family_val), "concat_triplet_window") &&
        !strcmp(yyjson_get_str(consumer_val), "direct_kernel_entry") &&
        !strcmp(yyjson_get_str(retained_form_val), "borrowed_text") &&
        !strcmp(yyjson_get_str(first_kind), "slice") &&
        !strcmp(yyjson_get_str(middle_kind), "const") &&
        !strcmp(yyjson_get_str(last_kind), "slice") &&
        first_source && yyjson_is_num(first_source) &&
        last_source && yyjson_is_num(last_source) &&
        middle_value_val && yyjson_is_num(middle_value_val) &&
        middle_len_val && yyjson_is_num(middle_len_val))) {
    return 0;
  }

  if ((long long)yyjson_get_sint(first_source) !=
      (long long)yyjson_get_sint(source_root_val)) {
    return 0;
  }
  if ((long long)yyjson_get_sint(last_source) !=
      (long long)yyjson_get_sint(source_root_val)) {
    return 0;
  }

  if (source_root) *source_root = (long long)yyjson_get_sint(source_root_val);
  if (middle_value) *middle_value = (long long)yyjson_get_sint(middle_value_val);
  if (middle_known_length) {
    *middle_known_length = (long long)yyjson_get_sint(middle_len_val);
  }
  return 1;
}

static int hako_llvmc_has_thin_entry_selection(
    yyjson_val* fn,
    const char* surface,
    const char* subject,
    const char* manifest_row,
    const char* selected_entry) {
  yyjson_val* metadata;
  yyjson_val* selections;
  size_t i;

  if (!(fn && yyjson_is_obj(fn) && surface && subject && manifest_row &&
        selected_entry)) {
    return 0;
  }

  metadata = yyjson_obj_get(fn, "metadata");
  selections = metadata ? yyjson_obj_get(metadata, "thin_entry_selections") : NULL;
  if (!(selections && yyjson_is_arr(selections))) {
    return 0;
  }

  for (i = 0; i < yyjson_arr_size(selections); i++) {
    yyjson_val* row = yyjson_arr_get(selections, i);
    const char* row_surface = row ? yyjson_get_str(yyjson_obj_get(row, "surface")) : NULL;
    const char* row_subject = row ? yyjson_get_str(yyjson_obj_get(row, "subject")) : NULL;
    const char* row_manifest =
        row ? yyjson_get_str(yyjson_obj_get(row, "manifest_row")) : NULL;
    const char* row_selected =
        row ? yyjson_get_str(yyjson_obj_get(row, "selected_entry")) : NULL;
    if (!(row_surface && row_subject && row_manifest && row_selected)) {
      continue;
    }
    if (strcmp(row_surface, surface) == 0 &&
        strcmp(row_subject, subject) == 0 &&
        strcmp(row_manifest, manifest_row) == 0 &&
        strcmp(row_selected, selected_entry) == 0) {
      return 1;
    }
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
