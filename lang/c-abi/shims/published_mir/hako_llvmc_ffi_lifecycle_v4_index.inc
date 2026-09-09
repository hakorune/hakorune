/* Borrowed physical indexes, owned by one V4 invocation. No source classification. */
typedef struct {
  uint32_t id, object_id;
  yyjson_val* instruction;
  int kind, origin, resolving;
} hako_lv4_value;

typedef struct {
  uint32_t id;
  size_t ordinal;
  yyjson_val* block;
} hako_lv4_block;

typedef struct hako_lv4_function_index {
  yyjson_val* function;
  hako_lv4_value* values;
  size_t value_count;
  hako_lv4_block* blocks;
  size_t block_count;
  uint32_t receiver_object;
  int receiver_object_set;
} hako_lv4_function_index;

typedef struct hako_lv4_layout_index {
  uint32_t id;
  yyjson_val* row;
} hako_lv4_layout_index;
static int lv4_layout_compare(const void* a, const void* b) {
  uint32_t x = ((const hako_lv4_layout_index*)a)->id, y = ((const hako_lv4_layout_index*)b)->id;
  return (x > y) - (x < y);
}
static yyjson_val* lv4_layout(const hako_lv4_input* in, uint32_t id) {
  hako_lv4_layout_index key = {id, NULL};
  if (!in->layout_count) return NULL;
  hako_lv4_layout_index* found = bsearch(&key, in->layouts, in->layout_count,
      sizeof(key), lv4_layout_compare);
  return found ? found->row : NULL;
}
static int lv4_slot(const hako_lv4_input* in, yyjson_val* op) {
  yyjson_val* fields = yyjson_obj_get(lv4_layout(in, lv4_u(op, "object_id")), "fields");
  for (size_t i = 0; i < yyjson_arr_size(fields); i++) {
    yyjson_val* field = yyjson_arr_get(fields, i);
    if (lv4_u(field, "declaration_ordinal") == lv4_u(op, "field_ordinal"))
      return (int)lv4_u(field, "runtime_slot");
  }
  return -1;
}
static uint32_t lv4_type_id(const hako_lv4_input* in, yyjson_val* op) {
  return lv4_u(lv4_layout(in, lv4_u(op, "object_id")), "runtime_type_id");
}

static int lv4_value_compare(const void* a, const void* b) {
  uint32_t x = ((const hako_lv4_value*)a)->id, y = ((const hako_lv4_value*)b)->id;
  return (x > y) - (x < y);
}
static int lv4_block_compare(const void* a, const void* b) {
  uint32_t x = ((const hako_lv4_block*)a)->id, y = ((const hako_lv4_block*)b)->id;
  return (x > y) - (x < y);
}
static hako_lv4_value* lv4_index_value(hako_lv4_function_index* fn, uint32_t id) {
  hako_lv4_value key = {0}; key.id = id;
  return bsearch(&key, fn->values, fn->value_count, sizeof(key), lv4_value_compare);
}
static hako_lv4_block* lv4_index_block(hako_lv4_function_index* fn, uint32_t id) {
  hako_lv4_block key = {0}; key.id = id;
  return bsearch(&key, fn->blocks, fn->block_count, sizeof(key), lv4_block_compare);
}
static int lv4_operation_result(yyjson_val* op) {
  if (lv4_is(op, "kind", "new_box") || lv4_is(op, "kind", "array_new")) return LV4_HANDLE;
  if (lv4_is(op, "kind", "ordinary_call")) return LV4_I64;
  if (lv4_is(op, "kind", "map_new")) return LV4_MAP;
  if (lv4_is(op, "kind", "map_prepare_key")) return LV4_KEY;
  if (lv4_is(op, "kind", "map_install_indexed") || lv4_is(op, "kind", "map_install_value")) return LV4_OUTCOME;
  return LV4_BAD;
}

static int lv4_index_resolve(hako_lv4_function_index* fn, hako_lv4_value* value) {
  yyjson_val* ins;
  if (!value || value->resolving) return 0;
  if (value->kind != LV4_BAD) return 1;
  value->resolving = 1;
  ins = value->instruction;
  if (lv4_is(ins, "op", "const_i64") || lv4_is(ins, "op", "add") ||
      lv4_is(ins, "op", "object_field_get")) value->kind = LV4_I64;
  else if (lv4_is(ins, "op", "const_bool")) value->kind = LV4_BOOL;
  else if (lv4_is(ins, "op", "const_f64_bits")) value->kind = LV4_F64;
  else if (lv4_is(ins, "op", "const_unit")) value->kind = LV4_UNIT;
  else if (lv4_is(ins, "op", "const_string")) value->kind = LV4_TEXT;
  else if (lv4_is(ins, "op", "fault_frame_enter")) value->kind = LV4_FRAME;
  else if (lv4_is(ins, "op", "invoke_normal_result")) {
    hako_lv4_block* block = lv4_index_block(fn, lv4_u(ins, "invoke_block"));
    yyjson_val* op = block ? yyjson_obj_get(lv4_term(block->block), "operation") : NULL;
    if (!block || !op) return 0;
    value->kind = lv4_operation_result(op);
    value->origin = (int)block->ordinal;
    if (lv4_is(op, "kind", "new_box")) value->object_id = lv4_u(op, "object_id");
  } else if (lv4_is(ins, "op", "copy")) {
    hako_lv4_value* source = lv4_index_value(fn, lv4_u(ins, "src"));
    if (!lv4_index_resolve(fn, source) || source->kind == LV4_MAP ||
        source->kind == LV4_KEY || source->kind == LV4_OUTCOME) return 0;
    value->kind = source->kind;
    value->origin = source->origin;
    value->object_id = source->object_id;
  }
  value->resolving = 0;
  return value->kind != LV4_BAD;
}

static void lv4_index_close(hako_lv4_input* in) {
  for (size_t fi = 0; fi < in->index_count; fi++) {
    free(in->index[fi].values);
    free(in->index[fi].blocks);
  }
  free(in->index); in->index = NULL; in->index_count = 0;
  free(in->layouts); in->layouts = NULL; in->layout_count = 0;
}

static int lv4_index_open(hako_lv4_input* in) {
  size_t functions = yyjson_arr_size(in->functions);
  if (!functions || functions > 256) return 0;
  in->index = calloc(functions, sizeof(*in->index));
  if (!in->index) return 0;
  in->index_count = functions;
  yyjson_val* layouts = yyjson_obj_get(in->root, "layouts");
  in->layout_count = yyjson_arr_size(layouts);
  if (in->layout_count > 4096) return 0;
  if (in->layout_count) {
    in->layouts = calloc(in->layout_count, sizeof(*in->layouts));
    if (!in->layouts) return 0;
  }
  for (size_t i = 0; i < in->layout_count; i++) {
    yyjson_val* row = yyjson_arr_get(layouts, i);
    yyjson_val* fields = yyjson_obj_get(row, "fields");
    in->layouts[i] = (hako_lv4_layout_index){lv4_u(row, "object_id"), row};
    for (size_t j = 0; j < yyjson_arr_size(fields); j++)
      if (lv4_u(yyjson_arr_get(fields, j), "storage_kind") != HAKO_LLVMC_LIFECYCLE_STORAGE_I64)
        return 0;
  }
  if (in->layout_count) qsort(in->layouts, in->layout_count, sizeof(*in->layouts), lv4_layout_compare);
  for (size_t fi = 0; fi < functions; fi++) {
    hako_lv4_function_index* fn = &in->index[fi];
    yyjson_val* body = yyjson_arr_get(in->functions, fi);
    int birth = lv4_is(body, "role", "birth_unit");
    int ordinary = lv4_is(body, "role", "ordinary_i64");
    yyjson_val* blocks = hako_physical_blocks(body);
    yyjson_val* params = yyjson_obj_get(body, "params");
    size_t capacity = lv4_value_budget(body) + 1;
    fn->function = body; fn->block_count = yyjson_arr_size(blocks);
    if (!fn->block_count || fn->block_count > 256 || capacity > 4097) return 0;
    fn->values = calloc(capacity, sizeof(*fn->values));
    fn->blocks = calloc(fn->block_count, sizeof(*fn->blocks));
    if (!fn->values || !fn->blocks) return 0;
    if (birth) {
      hako_lv4_value* receiver = &fn->values[fn->value_count++];
      receiver->id = lv4_u(body, "receiver"); receiver->kind = LV4_HANDLE;
      receiver->origin = -2; /* Borrowed physical receiver, never an allocation. */
    }
    for (size_t pi = 0; pi < yyjson_arr_size(params); pi++) {
      hako_lv4_value* param = &fn->values[fn->value_count++];
      param->id = lv4_u(yyjson_arr_get(params, pi), "value");
      param->kind = ordinary ? LV4_I64 : LV4_TAGGED; param->origin = -1;
    }
    for (size_t bi = 0; bi < fn->block_count; bi++) {
      yyjson_val* block = yyjson_arr_get(blocks, bi);
      yyjson_val* rows = yyjson_obj_get(block, "instructions");
      fn->blocks[bi] = (hako_lv4_block){lv4_u(block, "id"), bi, block};
      for (size_t ri = 0; ri < yyjson_arr_size(rows); ri++) {
        yyjson_val* ins = yyjson_obj_get(yyjson_arr_get(rows, ri), "instruction");
        yyjson_val* dst = hako_physical_instruction_dst(ins);
        if (!dst) continue;
        if (fn->value_count == capacity) return 0;
        hako_lv4_value* value = &fn->values[fn->value_count++];
        value->id = (uint32_t)yyjson_get_uint(dst); value->instruction = ins; value->origin = -1;
      }
    }
    qsort(fn->values, fn->value_count, sizeof(*fn->values), lv4_value_compare);
    qsort(fn->blocks, fn->block_count, sizeof(*fn->blocks), lv4_block_compare);
    for (size_t i = 1; i < fn->value_count; i++)
      if (fn->values[i-1].id == fn->values[i].id) return 0;
    for (size_t i = 1; i < fn->block_count; i++)
      if (fn->blocks[i-1].id == fn->blocks[i].id) return 0;
    for (size_t i = 0; i < fn->value_count; i++)
      if (!lv4_index_resolve(fn, &fn->values[i])) return 0;
  }
  return 1;
}

static int lv4_input_type(const hako_lv4_input* in, yyjson_val* fn, uint32_t value) {
  if (in->native) return lv4_type(fn, value, lv4_value_budget(fn));
  for (size_t fi = 0; fi < in->index_count; fi++) {
    if (in->index[fi].function != fn) continue;
    hako_lv4_value* row = lv4_index_value(&in->index[fi], value);
    return row ? row->kind : LV4_BAD;
  }
  return LV4_BAD;
}
