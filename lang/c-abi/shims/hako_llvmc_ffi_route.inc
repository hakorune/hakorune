static int compile_json_compat_harness_keep(const char* json_in, const char* obj_out, char** err_out) {
  const char* llvmc = getenv("NYASH_NY_LLVM_COMPILER");
  char log_path[1024];
  char cmd[4096];
  char* first_line;
  int n;
  int rc;
  if (!llvmc || !*llvmc) llvmc = "target/release/ny-llvmc";
  if (!hako_llvmc_file_exists(llvmc)) {
    hako_set_last_error("NOT_FOUND");
    return set_err_owned(err_out, "ny-llvmc not found (NYASH_NY_LLVM_COMPILER)");
  }
  if (hako_llvmc_build_harness_log_path(log_path, sizeof(log_path)) != 0) {
    hako_set_last_error("FAILED");
    return set_err_owned(err_out, "hako_llvmc harness log path too long");
  }
  remove(log_path);
  n = snprintf(
      cmd,
      sizeof(cmd),
      "\"%s\" --driver harness --in \"%s\" --emit obj --out \"%s\" 2> \"%s\"",
      llvmc,
      json_in,
      obj_out,
      log_path);
  if (n <= 0 || (size_t)n >= sizeof(cmd)) {
    hako_set_last_error("FAILED");
    return set_err_owned(err_out, "hako_llvmc harness compile command too long");
  }
  remove(obj_out);
  rc = system(cmd);
  hako_llvmc_emit_route_trace(
      "child",
      (rc == 0 && hako_llvmc_file_exists(obj_out)) ? "success" : "failure",
      "ny-llvmc",
      "driver=harness");
  if (rc == 0 && hako_llvmc_file_exists(obj_out)) {
    remove(log_path);
    return 0;
  }
  hako_set_last_error("FAILED");
  first_line = hako_llvmc_read_first_line_owned(log_path);
  remove(log_path);
  if (first_line) {
    if (err_out) {
      *err_out = first_line;
    } else {
      free(first_line);
    }
    return -1;
  }
  if (rc == 0) {
    return set_err_owned(err_out, "ny-llvmc harness compile finished without object");
  }
  return set_err_owned(err_out, "ny-llvmc harness compile failed");
}

static yyjson_val* hako_llvmc_find_entry_function(yyjson_val* fns) {
  size_t i;
  yyjson_val* fallback = NULL;
  if (!(fns && yyjson_is_arr(fns))) {
    return NULL;
  }
  for (i = 0; i < yyjson_arr_size(fns); i++) {
    yyjson_val* candidate = yyjson_arr_get(fns, i);
    const char* name = candidate ? yyjson_get_str(yyjson_obj_get(candidate, "name")) : NULL;
    if (!name) {
      continue;
    }
    if (strcmp(name, "main") == 0) {
      return candidate;
    }
    if (!fallback && strcmp(name, "ny_main") == 0) {
      fallback = candidate;
    }
  }
  return fallback ? fallback : yyjson_arr_get_first(fns);
}

static int hako_llvmc_rune_trace_enabled(void) {
  const char* v = getenv("NYASH_RUNE_TRACE");
  return (v && v[0] == '1');
}

static const char* hako_llvmc_selected_rune_name(yyjson_val* rune) {
  yyjson_val* name = rune ? yyjson_obj_get(rune, "name") : NULL;
  return (name && yyjson_is_str(name)) ? yyjson_get_str(name) : NULL;
}

static const char* hako_llvmc_selected_rune_arg0(yyjson_val* rune) {
  yyjson_val* args = rune ? yyjson_obj_get(rune, "args") : NULL;
  if (!(args && yyjson_is_arr(args) && yyjson_arr_size(args) > 0)) {
    return NULL;
  }
  yyjson_val* arg0 = yyjson_arr_get(args, 0);
  return (arg0 && yyjson_is_str(arg0)) ? yyjson_get_str(arg0) : NULL;
}

static void hako_llvmc_select_entry_runes(
    yyjson_val* fn0,
    hako_llvmc_entry_rune_selection* out) {
  yyjson_val* attrs;
  yyjson_val* runes;
  size_t i;
  if (!out) {
    return;
  }
  out->symbol = NULL;
  out->callconv = NULL;
  if (!(fn0 && yyjson_is_obj(fn0))) {
    return;
  }
  attrs = yyjson_obj_get(fn0, "attrs");
  if (!(attrs && yyjson_is_obj(attrs))) {
    return;
  }
  runes = yyjson_obj_get(attrs, "runes");
  if (!(runes && yyjson_is_arr(runes))) {
    return;
  }

  for (i = 0; i < yyjson_arr_size(runes); i++) {
    yyjson_val* rune = yyjson_arr_get(runes, i);
    const char* name = hako_llvmc_selected_rune_name(rune);
    if (!name) {
      continue;
    }
    if (!out->symbol && strcmp(name, "Symbol") == 0) {
      out->symbol = hako_llvmc_selected_rune_arg0(rune);
    } else if (!out->callconv && strcmp(name, "CallConv") == 0) {
      out->callconv = hako_llvmc_selected_rune_arg0(rune);
    }
  }
}

static const char* hako_llvmc_entry_symbol_default(void) {
  return "ny_main";
}

static const char* hako_llvmc_effective_entry_symbol(
    const hako_llvmc_entry_rune_selection* selection) {
  if (selection && selection->symbol && selection->symbol[0]) {
    return selection->symbol;
  }
  return hako_llvmc_entry_symbol_default();
}

static const char* hako_llvmc_entry_callconv_prefix(
    const hako_llvmc_entry_rune_selection* selection) {
  const char* callconv = selection ? selection->callconv : NULL;
  if (!callconv || !callconv[0]) {
    return "";
  }
  if (strcmp(callconv, "c") == 0 || strcmp(callconv, "ccc") == 0) {
    return "";
  }
  return "";
}

static void hako_llvmc_emit_entry_header(
    FILE* f,
    const hako_llvmc_entry_rune_selection* selection) {
  const char* symbol = hako_llvmc_effective_entry_symbol(selection);
  const char* callconv_prefix = hako_llvmc_entry_callconv_prefix(selection);
  fprintf(f, "define %si64 @\"%s\"() {\n", callconv_prefix, symbol);
}

/*
 * Selected Dynamic entry signatures are a physical projection of the MIR
 * function parameter list.  Keep the historical no-parameter header above
 * for the old seed lane; the selected lane must never silently reuse it.
 */
static int hako_llvmc_selected_dynamic_admission_present(yyjson_val* fn) {
  yyjson_val* metadata = fn ? yyjson_obj_get(fn, "metadata") : NULL;
  yyjson_val* admission = metadata
      ? yyjson_obj_get(metadata, "dynamic_v2_aot_call_admission_v2")
      : NULL;
  return admission && yyjson_is_obj(admission);
}

static int hako_llvmc_selected_dynamic_parameter_signature_valid(yyjson_val* fn) {
  yyjson_val* params = fn ? yyjson_obj_get(fn, "params") : NULL;
  size_t count;
  long long values[4];
  if (!(params && yyjson_is_arr(params))) return 0;
  count = yyjson_arr_size(params);
  if (count != 4) return 0;
  for (size_t i = 0; i < count; i++) {
    yyjson_val* value = yyjson_arr_get(params, i);
    if (!(value && yyjson_is_int(value))) return 0;
    values[i] = (long long)yyjson_get_sint(value);
    if (values[i] < 0) return 0;
    for (size_t prior = 0; prior < i; prior++) {
      if (values[prior] == values[i]) return 0;
    }
  }
  return 1;
}

static int hako_llvmc_selected_dynamic_entry_signature_gate(yyjson_val* root) {
  yyjson_val* fns = root ? yyjson_obj_get(root, "functions") : NULL;
  yyjson_val* fn = hako_llvmc_find_entry_function(fns);
  if (!fn || !hako_llvmc_selected_dynamic_admission_present(fn)) return 1;
  return hako_llvmc_selected_dynamic_parameter_signature_valid(fn);
}

static int hako_llvmc_emit_selected_dynamic_entry_header(
    FILE* f,
    const hako_llvmc_entry_rune_selection* selection,
    yyjson_val* fn) {
  yyjson_val* params = fn ? yyjson_obj_get(fn, "params") : NULL;
  const char* symbol = hako_llvmc_effective_entry_symbol(selection);
  const char* callconv_prefix = hako_llvmc_entry_callconv_prefix(selection);
  if (!f || !hako_llvmc_selected_dynamic_parameter_signature_valid(fn)) return 0;
  fprintf(f, "define %si64 @\"%s\"(", callconv_prefix, symbol);
  for (size_t i = 0; i < 4; i++) {
    yyjson_val* value = yyjson_arr_get(params, i);
    long long reg = (long long)yyjson_get_sint(value);
    fprintf(f, "%si64 %%r%lld", i ? ", " : "", reg);
  }
  fprintf(f, ") {\n");
  return 1;
}

static void hako_llvmc_emit_entry_alias_if_needed(
    FILE* f,
    const hako_llvmc_entry_rune_selection* selection) {
  const char* symbol = hako_llvmc_effective_entry_symbol(selection);
  if (strcmp(symbol, hako_llvmc_entry_symbol_default()) == 0) {
    return;
  }
  fprintf(f, "\ndefine i64 @ny_main() {\n");
  fprintf(f, "entry:\n");
  fprintf(f, "  %%entry_alias = call i64 @\"%s\"()\n", symbol);
  fprintf(f, "  ret i64 %%entry_alias\n");
  fprintf(f, "}\n");
}

static void hako_llvmc_log_selected_entry_runes(yyjson_val* fn0) {
  hako_llvmc_entry_rune_selection selection;
  if (!hako_llvmc_rune_trace_enabled()) {
    return;
  }
  hako_llvmc_select_entry_runes(fn0, &selection);
  if (selection.symbol || selection.callconv) {
    fprintf(stderr, "[rune/entry] selected attrs");
    if (selection.symbol) {
      fprintf(stderr, " Symbol(%s)", selection.symbol);
    }
    if (selection.callconv) {
      fprintf(stderr, " CallConv(%s)", selection.callconv);
    }
    fprintf(stderr, "\n");
  }
}

static int hako_llvmc_forward_link_to_aot_without_ffi(
    const char* obj_in,
    const char* exe_out,
    const char* extra_ldflags,
    char** err_out) {
  const char* prev_raw = getenv("HAKO_AOT_USE_FFI");
  char prev_buf[64];
  int had_prev = prev_raw && prev_raw[0];
  if (had_prev) {
    snprintf(prev_buf, sizeof(prev_buf), "%s", prev_raw);
  }
  hako_llvmc_set_env_value("HAKO_AOT_USE_FFI", "0");
  int rc = hako_aot_link_obj(obj_in, exe_out, extra_ldflags, err_out);
  hako_llvmc_set_env_value("HAKO_AOT_USE_FFI", had_prev ? prev_buf : NULL);
  return rc;
}

static int hako_llvmc_forward_link_to_aot_v2_without_ffi(
    const char* obj_in,
    const char* exe_out,
    const char* runtime_archive_path,
    const char* extra_ldflags,
    char** err_out) {
  const char* prev_raw = getenv("HAKO_AOT_USE_FFI");
  char prev_buf[64];
  int had_prev = prev_raw && prev_raw[0];
  if (had_prev) {
    snprintf(prev_buf, sizeof(prev_buf), "%s", prev_raw);
  }
  hako_llvmc_set_env_value("HAKO_AOT_USE_FFI", "0");
  int rc = hako_aot_link_obj_v2(
      obj_in, exe_out, runtime_archive_path, extra_ldflags, err_out);
  hako_llvmc_set_env_value("HAKO_AOT_USE_FFI", had_prev ? prev_buf : NULL);
  return rc;
}

static int forward_link_obj_to_aot(const char* obj_in, const char* exe_out, const char* extra_ldflags, char** err_out) {
  return hako_llvmc_forward_link_to_aot_without_ffi(obj_in, exe_out, extra_ldflags, err_out);
}

static int forward_link_obj_to_aot_v2(
    const char* obj_in,
    const char* exe_out,
    const char* runtime_archive_path,
    const char* extra_ldflags,
    char** err_out) {
  return hako_llvmc_forward_link_to_aot_v2_without_ffi(
      obj_in, exe_out, runtime_archive_path, extra_ldflags, err_out);
}

static int compile_json_compat_pure(const char* json_in, const char* obj_out, char** err_out);

static int compile_json_via_explicit_compat_harness_replay(
    const char* json_in,
    const char* obj_out,
    char** err_out) {
  hako_llvmc_emit_route_replay("harness", "unsupported_pure_shape");
  return compile_json_compat_harness_keep(json_in, obj_out, err_out);
}

static int compile_json_via_pure_first_lane(
    const char* json_in,
    const char* obj_out,
    char** err_out) {
  return compile_json_compat_pure(json_in, obj_out, err_out);
}

static int hako_llvmc_require_pure_first_recipe(char** err_out) {
  const char* recipe = hako_llvmc_backend_compile_recipe();
  if (recipe && strcmp(recipe, "pure-first") == 0) {
    return 0;
  }
  hako_set_last_error("UNSUPPORTED");
  return set_err_owned(
      err_out,
      "[freeze:route][generic-capi-recipe-required] set HAKO_BACKEND_COMPILE_RECIPE=pure-first");
}

static int hako_llvmc_reject_ambient_harness_replay(char** err_out) {
  if (!compat_harness_replay_enabled()) {
    return 0;
  }
  hako_set_last_error("UNSUPPORTED");
  return set_err_owned(
      err_out,
      "[freeze:route][generic-capi-compat-admission-required] use hako_llvmc_compile_json_compat_harness");
}

// Exported symbols expected by hako_aot.c when loading libhako_llvmc_ffi.so
// Signature must match: int (*)(const char*, const char*, char**)
__attribute__((visibility("default")))
int hako_llvmc_compile_json(const char* json_in, const char* obj_out, char** err_out) {
  if (hako_llvmc_legacy_capi_pure_alias_enabled()) {
    hako_llvmc_warn_legacy_capi_pure_alias_once();
    return set_err_owned(
        err_out,
        "[freeze:contract][env/hako_capi_pure_retired] HAKO_CAPI_PURE is retired; use HAKO_BACKEND_COMPILE_RECIPE=pure-first");
  }
  if (hako_llvmc_require_pure_first_recipe(err_out) != 0) {
    return -1;
  }
  if (hako_llvmc_reject_ambient_harness_replay(err_out) != 0) {
    return -1;
  }
  return compile_json_via_pure_first_lane(json_in, obj_out, err_out);
}

// Recipe-aware daily callers use this explicit pure-first transport entry so
// generic route selection stays outside the C shim.
__attribute__((visibility("default")))
int hako_llvmc_compile_json_pure_first(const char* json_in, const char* obj_out, char** err_out) {
  if (hako_llvmc_reject_ambient_harness_replay(err_out) != 0) {
    return -1;
  }
  return compile_json_via_pure_first_lane(json_in, obj_out, err_out);
}

// Explicit compatibility ABI.  It does not read replay environment state;
// reaching this symbol is itself the named admission.
__attribute__((visibility("default")))
int hako_llvmc_compile_json_compat_harness(
    const char* json_in,
    const char* obj_out,
    char** err_out) {
  return compile_json_compat_harness_keep(json_in, obj_out, err_out);
}
