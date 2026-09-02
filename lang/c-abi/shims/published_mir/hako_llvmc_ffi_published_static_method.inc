/* Typed, synchronous published-call ingress for the canonical MIR publish.
 *
 * The rows are borrowed from Rust for one compile call.  This file contains
 * no resolver, registry, or symbol-to-key parser.  A row is consumed by its
 * exact function/block/instruction coordinate; every supplied row must be
 * consumed before a successful object is returned.  The historical symbol
 * name is retained for the bounded API, while `kind` distinguishes the
 * selected StaticBoxMethod, Builtin Print, and FreeFunction physical
 * consumers.
 */

static const hako_llvmc_published_static_method_call_v1*
    hako_llvmc_published_static_method_rows = NULL;
static size_t hako_llvmc_published_static_method_row_count = 0;
static unsigned char hako_llvmc_published_static_method_row_used[1024];

static int hako_llvmc_published_static_method_symbol_safe(const char* value) {
  const unsigned char* p = (const unsigned char*)value;
  if (!value || !value[0]) return 0;
  while (*p) {
    if (*p < 0x20 || *p == '"' || *p == '\\') return 0;
    p++;
  }
  return 1;
}

static int hako_llvmc_published_static_method_rows_begin(
    const hako_llvmc_published_static_method_call_v1* rows,
    size_t row_count,
    char** err_out) {
  if (!(rows && row_count > 0 && row_count <= 1024)) {
    return set_err_owned(
        err_out,
        "[freeze:contract][published-mir-static-method] empty or oversized typed row set");
  }
  memset(hako_llvmc_published_static_method_row_used, 0,
         sizeof(hako_llvmc_published_static_method_row_used));
  for (size_t i = 0; i < row_count; i++) {
    const hako_llvmc_published_static_method_call_v1* row = &rows[i];
    int valid_kind =
        row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_STATIC_METHOD ||
        row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_BUILTIN_PRINT ||
        row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_FREE_FUNCTION;
    int valid_payload =
        (row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_STATIC_METHOD ||
         row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_FREE_FUNCTION)
            ? hako_llvmc_published_static_method_symbol_safe(row->target_symbol)
            : (row->target_symbol == NULL && row->arity == 1);
    if (!(row->function_name && row->function_name[0] && valid_kind && valid_payload)) {
      return set_err_owned(
          err_out,
          "[freeze:contract][published-mir-static-method] malformed typed row");
    }
    for (size_t prior = 0; prior < i; prior++) {
      const hako_llvmc_published_static_method_call_v1* other = &rows[prior];
      if (row->block_id == other->block_id &&
          row->instruction_index == other->instruction_index &&
          strcmp(row->function_name, other->function_name) == 0) {
        return set_err_owned(
            err_out,
            "[freeze:contract][published-mir-static-method] duplicate typed site");
      }
    }
  }
  hako_llvmc_published_static_method_rows = rows;
  hako_llvmc_published_static_method_row_count = row_count;
  return 0;
}

static void hako_llvmc_published_static_method_rows_end(void) {
  hako_llvmc_published_static_method_rows = NULL;
  hako_llvmc_published_static_method_row_count = 0;
  memset(hako_llvmc_published_static_method_row_used, 0,
         sizeof(hako_llvmc_published_static_method_row_used));
}

static const hako_llvmc_published_static_method_call_v1*
    hako_llvmc_published_static_method_row_for_site(
    const char* function_name,
    long long block_id,
    size_t instruction_index) {
  if (!hako_llvmc_published_static_method_rows || !function_name || block_id < 0) {
    return NULL;
  }
  for (size_t i = 0; i < hako_llvmc_published_static_method_row_count; i++) {
    const hako_llvmc_published_static_method_call_v1* row =
        &hako_llvmc_published_static_method_rows[i];
    if (hako_llvmc_published_static_method_row_used[i]) continue;
    if (row->block_id != (uint32_t)block_id ||
        row->instruction_index != (uint32_t)instruction_index ||
        strcmp(row->function_name, function_name) != 0) {
      continue;
    }
    hako_llvmc_published_static_method_row_used[i] = 1;
    return row;
  }
  return NULL;
}

enum hako_llvmc_published_static_method_shape_status_v1 {
  HAKO_LLVMC_PUBLISHED_ROW_ABSENT = 0,
  HAKO_LLVMC_PUBLISHED_ROW_READY = 1,
  HAKO_LLVMC_PUBLISHED_ROW_MALFORMED = -1,
};

/*
 * The two physical consumers share this admission only.  It owns the one
 * exact-site take and validates the published Global/dst/arity/argument
 * shape; LLVM emission and argument formatting remain consumer-local.
 */
static int hako_llvmc_published_static_method_read_i64_arg_v1(
    yyjson_val* args,
    size_t index,
    long long* out) {
  yyjson_val* value = args && yyjson_is_arr(args) &&
                              yyjson_arr_size(args) > index
                          ? yyjson_arr_get(args, index)
                          : NULL;
  if (!(value && yyjson_is_num(value))) return 0;
  if (out) *out = (long long)yyjson_get_sint(value);
  return 1;
}

static int hako_llvmc_published_static_method_take_i64_global_row_v1(
    const char* function_name,
    long long block_id,
    size_t instruction_index,
    const char* call_type,
    long long dst,
    yyjson_val* args,
    const hako_llvmc_published_static_method_call_v1** out_row) {
  const hako_llvmc_published_static_method_call_v1* row =
      hako_llvmc_published_static_method_row_for_site(
          function_name, block_id, instruction_index);
  if (out_row) *out_row = NULL;
  if (!row) return HAKO_LLVMC_PUBLISHED_ROW_ABSENT;
  if (!(call_type && !strcmp(call_type, "Global"))) {
    return HAKO_LLVMC_PUBLISHED_ROW_MALFORMED;
  }
  if (row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_BUILTIN_PRINT) {
    long long ignored_arg = 0;
    if (!(dst == 0 && args && yyjson_is_arr(args) &&
          yyjson_arr_size(args) == 1 &&
          hako_llvmc_published_static_method_read_i64_arg_v1(
              args, 0, &ignored_arg))) {
      return HAKO_LLVMC_PUBLISHED_ROW_MALFORMED;
    }
  } else if (row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_STATIC_METHOD ||
             row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_FREE_FUNCTION) {
    if (!(args && yyjson_is_arr(args) &&
          yyjson_arr_size(args) == (size_t)row->arity)) {
      return HAKO_LLVMC_PUBLISHED_ROW_MALFORMED;
    }
    for (size_t ai = 0; ai < (size_t)row->arity; ai++) {
      long long ignored_arg = 0;
      if (!hako_llvmc_published_static_method_read_i64_arg_v1(
              args, ai, &ignored_arg)) {
        return HAKO_LLVMC_PUBLISHED_ROW_MALFORMED;
      }
    }
  } else {
    return HAKO_LLVMC_PUBLISHED_ROW_MALFORMED;
  }
  if (out_row) *out_row = row;
  return HAKO_LLVMC_PUBLISHED_ROW_READY;
}

static int hako_llvmc_published_static_method_rows_finish(char** err_out) {
  for (size_t i = 0; i < hako_llvmc_published_static_method_row_count; i++) {
    if (!hako_llvmc_published_static_method_row_used[i]) {
      return set_err_owned(
          err_out,
          "[freeze:contract][published-mir-static-method] typed row was not consumed");
    }
  }
  return 0;
}
