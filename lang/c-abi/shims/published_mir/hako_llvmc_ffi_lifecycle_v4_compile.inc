/* One parsed document, one target session, one atomic object publication. */
#if !defined(_WIN32)
#include <sys/stat.h>

#if defined(HAKO_LLVMC_LIFECYCLE_TEST_SEAM)
static int lv4_test_replace_once(char** text, size_t* len, const char* old_text, const char* new_text) {
  char* at;
  size_t old_len, new_len, prefix, suffix;
  char* next;
  if (!text || !*text || !len || !old_text || !new_text) return 0;
  at = strstr(*text, old_text);
  if (!at) return 0;
  old_len = strlen(old_text);
  new_len = strlen(new_text);
  prefix = (size_t)(at - *text);
  suffix = *len - prefix - old_len;
  next = malloc(*len - old_len + new_len + 1);
  if (!next) return 0;
  memcpy(next, *text, prefix);
  memcpy(next + prefix, new_text, new_len);
  memcpy(next + prefix + new_len, at + old_len, suffix);
  next[*len - old_len + new_len] = '\0';
  free(*text);
  *text = next;
  *len = *len - old_len + new_len;
  return 1;
}

static int lv4_test_replace_body_with_verify_invalid(char** text, size_t* len) {
  static const char body[] =
      "define i64 @hako_test_verify_invalid() {\n"
      "entry:\n"
      "  %x = phi i64 [ 1, %other ]\n"
      "  ret i64 %x\n"
      "other:\n"
      "  ret i64 2\n"
      "}\n";
  char* first_newline;
  char* second_newline;
  char* next;
  size_t prefix;
  if (!text || !*text || !len) return 0;
  first_newline = strchr(*text, '\n');
  second_newline = first_newline ? strchr(first_newline + 1, '\n') : NULL;
  if (!second_newline) return 0;
  prefix = (size_t)(second_newline - *text) + 1;
  next = malloc(prefix + sizeof(body));
  if (!next) return 0;
  memcpy(next, *text, prefix);
  memcpy(next + prefix, body, sizeof(body));
  free(*text);
  *text = next;
  *len = prefix + sizeof(body) - 1;
  return 1;
}

static int lv4_test_rewrite_llvm(const char* path) {
  const char* mode = getenv("HAKO_LIFECYCLE_TEST_SEAM");
  FILE* file;
  long size;
  size_t len;
  char* text;
  int changed = 1;
  if (!mode || !strcmp(mode, "missing-library") || !strcmp(mode, "missing-symbol") ||
      !strcmp(mode, "emit-failure") ||
      !strcmp(mode, "empty-output")) return 1;
  file = fopen(path, "rb");
  if (!file || fseek(file, 0, SEEK_END) != 0) {
    if (file) fclose(file);
    return 0;
  }
  size = ftell(file);
  if (size < 0 || fseek(file, 0, SEEK_SET) != 0) {
    fclose(file);
    return 0;
  }
  len = (size_t)size;
  text = malloc(len + 1);
  if (!text || fread(text, 1, len, file) != len) {
    free(text);
    fclose(file);
    return 0;
  }
  text[len] = '\0';
  fclose(file);
  if (!strcmp(mode, "invalid-kind")) {
    changed = lv4_test_replace_once(&text, &len, ", i32 2, i64", ", i32 99, i64");
  } else if (!strcmp(mode, "invalid-bool")) {
    changed = lv4_test_replace_once(&text, &len, "add i64 0, 1", "add i64 0, 2");
  } else if (!strcmp(mode, "malformed-ir")) {
    changed = lv4_test_replace_once(&text, &len, "define ", "broken ");
  } else if (!strcmp(mode, "verify-invalid")) {
    changed = lv4_test_replace_body_with_verify_invalid(&text, &len);
  } else if (!strcmp(mode, "target-drift")) {
    changed = lv4_test_replace_once(
        &text, &len, "x86_64-unknown-linux-gnu", "i386-unknown-linux-gnu");
  }
  if (changed) {
    file = fopen(path, "wb");
    if (!file) {
      changed = 0;
    } else {
      int write_ok = fwrite(text, 1, len, file) == len;
      int close_ok = fclose(file) == 0;
      file = NULL;
      if (!write_ok || !close_ok) changed = 0;
    }
  }
  free(text);
  return changed;
}
#else
static int lv4_test_rewrite_llvm(const char* path) {
  (void)path;
  return 1;
}
#endif

static int lv4_preamble_matches(const char* path, const hako_lts_session* session) {
  FILE* f = fopen(path, "rb");
  char expected[2048], actual[2048];
  int n = snprintf(expected, sizeof(expected), "target triple = \"%s\"\ntarget datalayout = \"%s\"\n",
      session->triple, session->data_layout);
  int ok = 0;
  if (!f) return 0;
  if (n > 0 && (size_t)n < sizeof(expected) && fread(actual, 1, (size_t)n, f) == (size_t)n)
    ok = memcmp(actual, expected, (size_t)n) == 0;
  if (fclose(f) != 0) ok = 0;
  return ok;
}

static void lv4_measure_v2_census(yyjson_val* root,
    hako_llvmc_lifecycle_measurement* measurement) {
  yyjson_val* functions = yyjson_obj_get(root, "functions");
  if (!measurement || !measurement->enabled || !yyjson_is_arr(functions)) return;
  measurement->v2_functions = yyjson_arr_size(functions);
  for (size_t fi = 0; fi < measurement->v2_functions; fi++) {
    yyjson_val* function = yyjson_arr_get(functions, fi);
    yyjson_val* blocks = hako_physical_blocks(function);
    measurement->v2_blocks += yyjson_arr_size(blocks);
    for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
      yyjson_val* rows = yyjson_obj_get(yyjson_arr_get(blocks, bi), "instructions");
      measurement->v2_rows += yyjson_arr_size(rows) + 1;
    }
  }
}

static void lv4_emit_measurement(const hako_llvmc_lifecycle_measurement* measurement,
    int result) {
  char extra[1024];
  if (!measurement || !measurement->enabled) return;
  snprintf(extra, sizeof(extra),
      "input_revision=hako.published-lifecycle-physical-program.v2;profile=%u;native=%d;target=%s;toolchain=llvm-c-api;"
      "index_lifetime=invocation;parse_ns=%llu;v2_ns=%llu;v4_ns=%llu;emit_ns=%llu;library_emit_ns=%llu;"
      "v2_functions=%llu;v2_blocks=%llu;v2_rows=%llu;"
      "v4_index_functions=%llu;v4_index_blocks=%llu;v4_index_values=%llu;"
      "v4_index_resolves=%llu;v4_flow_blocks=%llu;v4_flow_rows=%llu;"
      "v4_emit_functions=%llu;v4_emit_blocks=%llu;v4_emit_rows=%llu;"
      "v4_type_lookups=%llu;v4_layout_lookups=%llu",
      measurement->profile, measurement->native,
      measurement->target_triple[0] ? measurement->target_triple : "unknown",
      (unsigned long long)measurement->parse_ns,
      (unsigned long long)measurement->v2_ns,
      (unsigned long long)measurement->v4_ns,
      (unsigned long long)measurement->emit_ns,
      (unsigned long long)measurement->library_emit_ns,
      (unsigned long long)measurement->v2_functions,
      (unsigned long long)measurement->v2_blocks,
      (unsigned long long)measurement->v2_rows,
      (unsigned long long)measurement->v4_index_functions,
      (unsigned long long)measurement->v4_index_blocks,
      (unsigned long long)measurement->v4_index_values,
      (unsigned long long)measurement->v4_index_resolves,
      (unsigned long long)measurement->v4_flow_blocks,
      (unsigned long long)measurement->v4_flow_rows,
      (unsigned long long)measurement->v4_emit_functions,
      (unsigned long long)measurement->v4_emit_blocks,
      (unsigned long long)measurement->v4_emit_rows,
      (unsigned long long)measurement->v4_type_lookups,
      (unsigned long long)measurement->v4_layout_lookups);
  hako_llvmc_emit_route_trace("lifecycle-v4-measure",
      result == 0 ? "ok" : "fail", "invocation-local", extra);
}

#endif

__attribute__((visibility("default")))
int hako_llvmc_compile_published_lifecycle_physical_v4(
    const char* json_in, const hako_llvmc_lifecycle_target_session_v2* abi,
    const char* obj_out, char** error) {
#if defined(_WIN32)
  return set_err_owned(error, "[freeze:contract][published-lifecycle-v4/platform]");
#else
  yyjson_doc* doc = NULL;
  hako_lv4_input input = {0};
  hako_llvmc_lifecycle_measurement measurement = {0};
  hako_lts_session session = {0};
  char *ll = NULL, *object = NULL;
  FILE* file = NULL;
  int fd = -1, rc = -1, ll_owned = 0, object_owned = 0;
  struct stat st;
  const char* failure = "input";
  uint64_t started;
  measurement.enabled = hako_llvmc_route_trace_enabled();
  if (!json_in || !json_in[0] || !obj_out || !obj_out[0]) goto done;
  started = hako_llvmc_measure_now_ns();
  doc = yyjson_read_file(json_in, 0, NULL, NULL);
  measurement.parse_ns = hako_llvmc_measure_now_ns() - started;
  if (!doc) { failure = "json"; goto done; }
  lv4_measure_v2_census(yyjson_doc_get_root(doc), &measurement);
  started = hako_llvmc_measure_now_ns();
  int v2_result = hako_llvmc_validate_published_lifecycle_physical_v2_doc(
      yyjson_doc_get_root(doc), error);
  measurement.v2_ns = hako_llvmc_measure_now_ns() - started;
  if (v2_result != 0) {
    failure = NULL; goto done;
  }
  input.measurement = measurement.enabled ? &measurement : NULL;
  started = hako_llvmc_measure_now_ns();
  int v4_result = lv4_admit(yyjson_doc_get_root(doc), &input, error);
  measurement.v4_ns = hako_llvmc_measure_now_ns() - started;
  if (v4_result != 0 || hako_lts_open(abi, &session, error) != 0) {
    failure = NULL; goto done;
  }
  measurement.native = input.native;
  measurement.profile = input.profile;
  if (session.triple)
    snprintf(measurement.target_triple, sizeof(measurement.target_triple), "%s", session.triple);
  failure = "target";
  if (strcmp(session.triple, "x86_64-unknown-linux-gnu") != 0) goto done;
  failure = "temporary-file";
  size_t len = strlen(obj_out) + 32;
  ll = malloc(len); object = malloc(len);
  if (!ll || !object) goto done;
  snprintf(ll, len, "%s.ll.XXXXXX", obj_out);
  snprintf(object, len, "%s.obj.XXXXXX", obj_out);
  fd = mkstemp(ll);
  if (fd < 0) goto done;
  ll_owned = 1;
  file = fdopen(fd, "wb");
  if (!file) { close(fd); fd = -1; goto done; }
  fd = -1;
  started = hako_llvmc_measure_now_ns();
  int emitted = lv4_emit(file, &input, abi, &session);
  if (fflush(file) != 0) emitted = 0;
  if (fclose(file) != 0) emitted = 0;
  measurement.emit_ns = hako_llvmc_measure_now_ns() - started;
  file = NULL;
  failure = "module-write";
  if (!emitted || !lv4_test_rewrite_llvm(ll) || !lv4_preamble_matches(ll, &session)) goto done;
  fd = mkstemp(object);
  if (fd < 0) goto done;
  object_owned = 1;
  close(fd); fd = -1;
  failure = "library-emit";
  started = hako_llvmc_measure_now_ns();
  int library_emit_ok = hako_lts_emit_object_from_ir(&session, ll, object, error);
  measurement.library_emit_ns = hako_llvmc_measure_now_ns() - started;
  if (library_emit_ok != 0) { failure = NULL; goto done; }
  if (stat(object, &st) != 0 || st.st_size <= 0) goto done;
  failure = "publish";
  if (rename(object, obj_out) != 0) goto done;
  object_owned = 0;
  rc = 0;
done:
  if (file) fclose(file);
  if (fd >= 0) close(fd);
  if (ll) { if (ll_owned) unlink(ll); free(ll); }
  if (object) { if (object_owned) unlink(object); free(object); }
  hako_lts_close(&session);
  lv4_index_close(&input);
  if (doc) yyjson_doc_free(doc);
  lv4_emit_measurement(&measurement, rc);
  if (rc != 0 && failure) {
    char message[128];
    snprintf(message, sizeof(message), "[freeze:contract][published-lifecycle-v4/%s]", failure);
    return set_err_owned(error, message);
  }
  return rc;
#endif
}
