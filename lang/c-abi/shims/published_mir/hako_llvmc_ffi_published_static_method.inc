/* Typed, synchronous published-call ingress for the canonical MIR publish.
 *
 * The rows are borrowed from Rust for one compile call.  This file contains
 * no resolver, registry, or symbol-to-key parser.  A row is consumed by its
 * exact function/block/instruction coordinate; every supplied row must be
 * consumed before a successful object is returned.  The historical symbol
 * name is retained for the bounded API, while `kind` distinguishes the
 * selected call and ArrayElementWrite physical consumers.
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
        row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_FREE_FUNCTION ||
        row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_LITERAL_APPEND ||
        row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_PUSH ||
        row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_SET ||
        row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_INSERT;
    int valid_payload =
        (row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_STATIC_METHOD ||
         row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_FREE_FUNCTION)
            ? hako_llvmc_published_static_method_symbol_safe(row->target_symbol)
            : (row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_BUILTIN_PRINT
                   ? (row->target_symbol == NULL && row->arity == 1)
                   : (row->target_symbol == NULL && row->arity == 0));
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
    hako_llvmc_published_static_method_peek_row_for_site(
    const char* function_name,
    long long block_id,
    size_t instruction_index) {
  if (!hako_llvmc_published_static_method_rows || !function_name || block_id < 0 ||
      (unsigned long long)block_id > UINT32_MAX || instruction_index > UINT32_MAX) {
    return NULL;
  }
  for (size_t i = 0; i < hako_llvmc_published_static_method_row_count; i++) {
    const hako_llvmc_published_static_method_call_v1* row =
        &hako_llvmc_published_static_method_rows[i];
    if (row->block_id != (uint32_t)block_id ||
        row->instruction_index != (uint32_t)instruction_index ||
        strcmp(row->function_name, function_name) != 0) {
      continue;
    }
    return row;
  }
  return NULL;
}

enum hako_llvmc_published_static_method_shape_status_v1 {
  HAKO_LLVMC_PUBLISHED_ROW_ABSENT = 0,
  HAKO_LLVMC_PUBLISHED_ROW_READY = 1,
  HAKO_LLVMC_PUBLISHED_ROW_MALFORMED = -1,
};

/* Peeking never satisfies the residual check.  Only the physical emitter
 * takes a validated row, and a second take is malformed rather than absent
 * (absent would permit a caller to enter generic compatibility handling). */
static int hako_llvmc_published_static_method_take_row_v1(
    const hako_llvmc_published_static_method_call_v1* row) {
  size_t index = (size_t)(row - hako_llvmc_published_static_method_rows);
  if (hako_llvmc_published_static_method_row_used[index])
    return HAKO_LLVMC_PUBLISHED_ROW_MALFORMED;
  hako_llvmc_published_static_method_row_used[index] = 1;
  return HAKO_LLVMC_PUBLISHED_ROW_READY;
}

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

static int hako_llvmc_published_static_method_peek_i64_global_row_v1(
    const char* function_name,
    long long block_id,
    size_t instruction_index,
    const char* call_type,
    long long dst,
    yyjson_val* args,
    const hako_llvmc_published_static_method_call_v1** out_row) {
  const hako_llvmc_published_static_method_call_v1* row =
      hako_llvmc_published_static_method_peek_row_for_site(
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

static int hako_llvmc_published_static_method_take_i64_global_row_v1(
    const char* function_name,
    long long block_id,
    size_t instruction_index,
    const char* call_type,
    long long dst,
    yyjson_val* args,
    const hako_llvmc_published_static_method_call_v1** out_row) {
  const hako_llvmc_published_static_method_call_v1* row = NULL;
  int status = hako_llvmc_published_static_method_peek_i64_global_row_v1(
      function_name, block_id, instruction_index, call_type, dst, args, &row);
  if (out_row) *out_row = NULL;
  if (status != HAKO_LLVMC_PUBLISHED_ROW_READY) return status;
  status = hako_llvmc_published_static_method_take_row_v1(row);
  if (status == HAKO_LLVMC_PUBLISHED_ROW_READY && out_row) *out_row = row;
  return status;
}

static int hako_llvmc_published_mir_read_u32_field_v1(
    yyjson_val* object,
    const char* key,
    uint32_t* out) {
  yyjson_val* value = object && key ? yyjson_obj_get(object, key) : NULL;
  long long raw = 0;
  if (!(value && yyjson_is_num(value))) return 0;
  raw = (long long)yyjson_get_sint(value);
  if (raw < 0 || (unsigned long long)raw > UINT32_MAX) return 0;
  if (out) *out = (uint32_t)raw;
  return 1;
}

static int hako_llvmc_published_mir_read_optional_u32_field_v1(
    yyjson_val* object,
    const char* key,
    int* present,
    uint32_t* out) {
  yyjson_val* value = object && key ? yyjson_obj_get(object, key) : NULL;
  if (!value || yyjson_is_null(value)) {
    if (present) *present = 0;
    if (out) *out = 0;
    return 1;
  }
  if (!hako_llvmc_published_mir_read_u32_field_v1(object, key, out)) return 0;
  if (present) *present = 1;
  return 1;
}

static uint32_t hako_llvmc_published_array_write_kind_v1(const char* kind) {
  if (!kind) return 0;
  if (!strcmp(kind, "literal_append"))
    return HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_LITERAL_APPEND;
  if (!strcmp(kind, "push")) return HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_PUSH;
  if (!strcmp(kind, "set")) return HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_SET;
  if (!strcmp(kind, "insert")) return HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_INSERT;
  return 0;
}

static int hako_llvmc_published_static_method_take_array_write_row_v1(
    const char* function_name,
    long long block_id,
    size_t instruction_index,
    yyjson_val* instruction,
    const hako_llvmc_published_static_method_call_v1** out_row) {
  const hako_llvmc_published_static_method_call_v1* row =
      hako_llvmc_published_static_method_peek_row_for_site(
          function_name, block_id, instruction_index);
  yyjson_val* op_value = instruction ? yyjson_obj_get(instruction, "op") : NULL;
  yyjson_val* kind_value = instruction ? yyjson_obj_get(instruction, "kind") : NULL;
  const char* op = op_value && yyjson_is_str(op_value) ? yyjson_get_str(op_value) : NULL;
  const char* kind = kind_value && yyjson_is_str(kind_value) ? yyjson_get_str(kind_value) : NULL;
  uint32_t site_id = 0;
  uint32_t receiver = 0;
  uint32_t index = 0;
  uint32_t value = 0;
  uint32_t dst = 0;
  int index_present = 0;
  int dst_present = 0;
  if (out_row) *out_row = NULL;
  if (!row) return HAKO_LLVMC_PUBLISHED_ROW_ABSENT;
  if (!(row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_LITERAL_APPEND ||
        row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_PUSH ||
        row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_SET ||
        row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_INSERT)) {
    return HAKO_LLVMC_PUBLISHED_ROW_MALFORMED;
  }
  if (!(op && !strcmp(op, "array_element_write") &&
        hako_llvmc_published_mir_read_u32_field_v1(instruction, "site_id", &site_id) &&
        hako_llvmc_published_mir_read_u32_field_v1(instruction, "receiver", &receiver) &&
        hako_llvmc_published_mir_read_u32_field_v1(instruction, "value", &value) &&
        hako_llvmc_published_mir_read_optional_u32_field_v1(
            instruction, "index", &index_present, &index) &&
        hako_llvmc_published_mir_read_optional_u32_field_v1(
            instruction, "dst", &dst_present, &dst))) {
    return HAKO_LLVMC_PUBLISHED_ROW_MALFORMED;
  }
  if (site_id != row->site_id || receiver != row->receiver || value != row->value ||
      index_present != ((row->flags & HAKO_LLVMC_PUBLISHED_ROW_FLAG_INDEX_PRESENT) != 0) ||
      dst_present != ((row->flags & HAKO_LLVMC_PUBLISHED_ROW_FLAG_DST_PRESENT) != 0) ||
      index != row->index || dst != row->dst ||
      hako_llvmc_published_array_write_kind_v1(kind) != row->kind) {
    return HAKO_LLVMC_PUBLISHED_ROW_MALFORMED;
  }
  if (hako_llvmc_published_static_method_take_row_v1(row) < 0)
    return HAKO_LLVMC_PUBLISHED_ROW_MALFORMED;
  if (out_row) *out_row = row;
  return HAKO_LLVMC_PUBLISHED_ROW_READY;
}

/* Emit one already-admitted ArrayElementWrite row.  The macro is expanded
 * inside each physical instruction walker so it can use that walker's
 * register/constant environment; the row-to-symbol and operand order remain
 * one shared owner.  ArrayElementWrite is Void even though the runtime helper
 * returns a status i64, so no result register is ever assigned here. */
#define HAKO_LLVMC_EMIT_PUBLISHED_ARRAY_WRITE_ROW(_f, _row, _value_is_i64)  \
  do {                                                                        \
    char _array_write_args[320];                                               \
    const char* _array_write_symbol = NULL;                                    \
    _array_write_args[0] = '\0';                                               \
    if ((_row)->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_LITERAL_APPEND || \
        (_row)->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_PUSH) {           \
      if ((_row)->flags & HAKO_LLVMC_PUBLISHED_ROW_FLAG_INDEX_PRESENT)          \
        return -1;                                                              \
      _array_write_symbol = "nyash.array.slot_append_hh";                     \
      append_i64_arg_ref(_array_write_args, sizeof(_array_write_args),          \
                         (long long)(_row)->receiver, 1);                      \
      append_i64_arg_ref(_array_write_args, sizeof(_array_write_args),          \
                         (long long)(_row)->value, 0);                         \
    } else if ((_row)->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_SET) {     \
      if (!((_row)->flags & HAKO_LLVMC_PUBLISHED_ROW_FLAG_INDEX_PRESENT))       \
        return -1;                                                              \
      _array_write_symbol = (_value_is_i64) ? "nyash.array.slot_store_hii"     \
                                           : "nyash.array.slot_store_hih";     \
      append_i64_arg_ref(_array_write_args, sizeof(_array_write_args),          \
                         (long long)(_row)->receiver, 1);                      \
      append_i64_arg_ref(_array_write_args, sizeof(_array_write_args),          \
                         (long long)(_row)->index, 0);                         \
      append_i64_arg_ref(_array_write_args, sizeof(_array_write_args),          \
                         (long long)(_row)->value, 0);                         \
    } else if ((_row)->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_INSERT) {   \
      if (!((_row)->flags & HAKO_LLVMC_PUBLISHED_ROW_FLAG_INDEX_PRESENT))       \
        return -1;                                                              \
      _array_write_symbol = "nyash.array.slot_insert_hih";                    \
      append_i64_arg_ref(_array_write_args, sizeof(_array_write_args),          \
                         (long long)(_row)->receiver, 1);                      \
      append_i64_arg_ref(_array_write_args, sizeof(_array_write_args),          \
                         (long long)(_row)->index, 0);                         \
      append_i64_arg_ref(_array_write_args, sizeof(_array_write_args),          \
                         (long long)(_row)->value, 0);                         \
    } else {                                                                    \
      return -1;                                                                \
    }                                                                           \
    if (!_array_write_symbol || !_array_write_args[0]) return -1;              \
    fprintf((_f), "  call i64 @\"%s\"(%s)\n",                            \
            _array_write_symbol, _array_write_args);                            \
  } while (0)

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
