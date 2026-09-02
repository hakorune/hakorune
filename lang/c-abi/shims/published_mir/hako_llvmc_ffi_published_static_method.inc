/* Typed, synchronous published-call ingress for the canonical MIR publish.
 *
 * The rows are borrowed from Rust for one compile call.  This file contains
 * no resolver, registry, or symbol-to-key parser.  A row is consumed by its
 * exact function/block/instruction coordinate; every supplied row must be
 * consumed before a successful object is returned.  The historical symbol
 * name is retained for the bounded API, while `kind` distinguishes the
 * selected StaticBoxMethod and Builtin Print physical consumers.
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
        row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_BUILTIN_PRINT;
    int valid_payload =
        row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_STATIC_METHOD
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
