/* Borrowed physical indexes, owned by one V4 invocation. No source classification. */
enum { LV4_OWNED_FIELD_BORROW = -4 }; /* Non-owning, distinct from nullable formal -3. */

typedef struct {
  uint32_t id, object_id;
  yyjson_val* instruction;
  yyjson_val* definition_block;
  int kind, origin, object_id_set, kind_class;
  size_t kind_parent, alias_root;
  unsigned char alias_state, seed_state;
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
  uint32_t return_object;
  int return_object_state;
  int has_text_carriers;
} hako_lv4_function_index;

static size_t lv4_state_count(const hako_lv4_function_index* fn) {
  return fn->block_count + (fn->has_text_carriers ? fn->value_count : 0);
}
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
  if (in->measurement) in->measurement->v4_layout_lookups++;
  if (!in->layout_count) return NULL;
  hako_lv4_layout_index* found = bsearch(&key, in->layouts, in->layout_count,
      sizeof(key), lv4_layout_compare);
  return found ? found->row : NULL;
}
static int lv4_layout_slot(yyjson_val* layout, uint32_t ordinal) {
  yyjson_val* fields = yyjson_obj_get(layout, "fields");
  for (size_t i = 0; i < yyjson_arr_size(fields); i++) {
    yyjson_val* field = yyjson_arr_get(fields, i);
    if (lv4_u(field, "declaration_ordinal") == ordinal)
      return (int)lv4_u(field, "runtime_slot");
  }
  return -1;
}
static int lv4_slot(const hako_lv4_input* in, yyjson_val* op) {
  return lv4_layout_slot(lv4_layout(in, lv4_u(op, "object_id")), lv4_u(op, "field_ordinal"));
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
/* Copy borrows the current obligation. A Phi is a carrier location, not a
 * transparent alias of a raw producer that executes again on the backedge. */
static int lv4_text_origin(hako_lv4_function_index* fn, hako_lv4_value* value) {
  for (size_t steps = 0; value && steps <= fn->value_count; steps++) {
    if (lv4_is(value->instruction, "op", "copy")) {
      value = lv4_index_value(fn, lv4_u(value->instruction, "src"));
      continue;
    }
    if (value->kind == LV4_TEXT) return -1;
    return value->kind == LV4_TEXT_OPERAND || value->kind == LV4_OWNED_STRING ? value->origin : -2;
  }
  return -2;
}

#include "hako_llvmc_ffi_lifecycle_v4_indexed_alias.inc"
static int lv4_canonical_receiver(hako_lv4_function_index* fn, const hako_lv4_value* value) {
  return value && value->kind == LV4_HANDLE && value->origin == -2 &&
      value->object_id_set && fn->receiver_object_set &&
      value->object_id == fn->receiver_object &&
      yyjson_is_uint(yyjson_obj_get(fn->function, "receiver")) &&
      value->id == lv4_u(fn->function, "receiver") &&
      (lv4_is_ordinary(fn->function) || lv4_is(fn->function, "role", "birth_unit"));
}
static int lv4_operation_result(yyjson_val* op) {
  if (lv4_is(op, "kind", "field_get")) return LV4_I64;
  if (lv4_is(op, "kind", "dynamic_invoke")) return LV4_DYNAMIC_CARRIER;
  if (lv4_is(op, "kind", "dynamic_carrier_integer")) return LV4_I64;
  if (lv4_is(op, "kind", "dynamic_carrier_sub_i64")) return LV4_DYNAMIC_CARRIER;
  if (lv4_is(op, "kind", "array_owned_get") || lv4_is(op, "kind", "map_field_get_indexed") ||
      lv4_is(op, "kind", "object_field_borrow")) return LV4_SLOT_BORROW;
  int string_result = hako_physical_string_result(yyjson_get_str(yyjson_obj_get(op, "kind")));
  if (string_result) return string_result == 1 ? LV4_OWNED_STRING : string_result == 3 ? LV4_BOOL : LV4_I64;
  if (lv4_is(op, "kind", "new_box") || lv4_is(op, "kind", "array_new") || lv4_is(op, "kind", "script_arguments_new")) return LV4_HANDLE;
  if (lv4_is(op, "kind", "ordinary_call"))
    /* A nullable call result occupies a handle lease slot: it carries a
       live object or the Void sentinel and owns the caller one checked
       release. */
    return lv4_is(op, "result", "bool") ? LV4_BOOL
        : lv4_is(op, "result", "text") ? LV4_OWNED_STRING
        : lv4_is(op, "result", "map") ? LV4_MAP
        : lv4_is(op, "result", "borrowed_nullable_handle") ? LV4_BORROWED_OBJECT
        : lv4_is(op, "result", "mixed_handle") ? LV4_MIXED_OBJECT
        : (lv4_is(op, "result", "handle") || lv4_is(op, "result", "nullable_handle"))
            ? LV4_HANDLE : lv4_is(op, "result", "i64") ? LV4_I64 : LV4_BAD;
  if (lv4_is(op, "kind", "map_field_has")) return LV4_BOOL;
  if (lv4_is(op, "kind", "map_new")) return LV4_MAP;
  if (lv4_is(op, "kind", "map_prepare_key")) return LV4_KEY;
  if (lv4_is(op, "kind", "map_field_install_indexed") || lv4_is(op, "kind", "map_install_indexed") || lv4_is(op, "kind", "map_install_value") ||
      lv4_is(op, "kind", "map_install_text") || lv4_is(op, "kind", "map_install_empty_array") ||
      lv4_is(op, "kind", "map_install_borrowed_array"))
    return LV4_OUTCOME;
  if (lv4_is(op, "kind", "map_checked_get") || lv4_is(op, "kind", "map_array_length")) return LV4_I64;
  if (lv4_is(op, "kind", "map_array_index_map")) return LV4_MAP_VIEW;
  if (lv4_is(op, "kind", "map_get_text")) return LV4_TEXT_VIEW;
  return LV4_BAD;
}

/* A source-owned child read borrows the canonical tuple and original receiver.
 * Scalar field reads keep their existing lane; no opcode sibling issues class. */

static int lv4_index_seed(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value);
static int lv4_owned_value_object(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, int allow_nullable);

static int lv4_child_field_object(const hako_lv4_input* in,
    hako_lv4_function_index* fn, yyjson_val* ins, uint32_t* child, int* origin) {
  uint32_t object_id = lv4_u(ins, "object_id"), ordinal = lv4_u(ins, "field_ordinal");
  yyjson_val* rows = yyjson_obj_get(lv4_layout(in, object_id), "owned_object_residences");
  int found = 0;
  for (size_t i = 0; i < yyjson_arr_size(rows); i++) {
    yyjson_val* row = yyjson_arr_get(rows, i);
    if (lv4_u(row, "field_ordinal") != ordinal) continue;
    uint32_t object = lv4_u(row, "child_object_id");
    hako_lv4_value* root = lv4_value_alias_root(fn,
        lv4_index_value(fn, lv4_u(ins, "base")));
    if (found || !lv4_layout(in, object) || !root || !lv4_index_seed(in, fn, root) ||
        root->kind != LV4_HANDLE) return 0;
    if (root->origin >= 0 && !lv4_owned_value_object(in, fn, root, 0)) return 0;
    if (!root->object_id_set || root->object_id != object_id) return 0;
    if (root->origin == -2) {
      /* Birth classifies an owning-edge loan only. Its Normal commit and
         child Birth must still be checked at both read and call by flow. */
      if (!lv4_canonical_receiver(fn, root)) return 0;
      *origin = LV4_OWNED_FIELD_BORROW;
    } else {
      /* Classification is not liveness. The read-only flow lender later
         proves the original New/Birth Normal and every owning edge. */
      hako_lv4_value* owner = root;
      for (size_t depth = 0; owner && owner->origin == LV4_OWNED_FIELD_BORROW; depth++) {
        if (depth >= fn->value_count || !lv4_is(owner->instruction, "op", "object_field_get")) return 0;
        owner = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(owner->instruction, "base")));
      }
      if (!lv4_canonical_receiver(fn, owner)) {
        if (!lv4_owned_value_object(in, fn, owner, 0)) return 0;
      }
      *origin = LV4_OWNED_FIELD_BORROW;
    }
    *child = object; found = 1;
  }
  return found ? 1 : -1;
}

static int lv4_index_seed(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value) {
  yyjson_val* ins = value->instruction;
  if (value->seed_state == 2) return 1;
  if (value->seed_state == 1) return 0;
  value->seed_state = 1;
  if (value->kind != LV4_BAD) { value->seed_state = 2; return 1; }
  if (lv4_is(ins, "op", "const_i64") || lv4_is(ins, "op", "add") ||
      lv4_is(ins, "op", "mul") || lv4_is(ins, "op", "sub") ||
      lv4_is(ins, "op", "div") || lv4_is(ins, "op", "mod") ||
      lv4_is(ins, "op", "array_get") || lv4_is(ins, "op", "array_length"))
    value->kind = LV4_I64;
  else if (lv4_is(ins, "op", "object_field_get")) {
    uint32_t child; int origin = -1;
    int sealed = lv4_child_field_object(in, fn, ins, &child, &origin);
    if (!sealed) return 0;
    if (sealed > 0) {
      value->kind = LV4_HANDLE; value->origin = origin; value->object_id = child;
      value->object_id_set = 1;
    } else value->kind = LV4_I64;
  }
  else if (lv4_is(ins, "op", "const_bool") || lv4_is(ins, "op", "compare") ||
      lv4_is(ins, "op", "borrowed_null_compare"))
    value->kind = LV4_BOOL;
  else if (lv4_is(ins, "op", "const_f64_bits")) value->kind = LV4_F64;
  else if (lv4_is(ins, "op", "const_unit")) value->kind = LV4_UNIT;
  /* The Void null sentinel shares the handle lane statically: origin stays
     negative so it can never be a live lease or reach a release operand. */
  else if (lv4_is(ins, "op", "const_null")) value->kind = LV4_HANDLE;
  else if (lv4_is(ins, "op", "const_string")) value->kind = LV4_TEXT;
  else if (lv4_is(ins, "op", "fault_frame_enter")) value->kind = LV4_FRAME;
  else if (lv4_is(ins, "op", "invoke_normal_result")) {
    hako_lv4_block* block = lv4_index_block(fn, lv4_u(ins, "invoke_block"));
    yyjson_val* op = block ? yyjson_obj_get(lv4_term(block->block), "operation") : NULL;
    if (!block || !op) return 0;
    value->kind = lv4_operation_result(op);
    value->origin = (int)block->ordinal;
    if (lv4_is(op, "kind", "new_box") || lv4_is(op, "kind", "array_owned_get")) {
      value->object_id = lv4_u(op, "object_id"); value->object_id_set = 1;
    } else if (lv4_is(op, "kind", "object_field_borrow")) {
      value->object_id = lv4_u(op, "expected_object_id"); value->object_id_set = 1;
    } else if (lv4_is(op, "kind", "map_field_get_indexed")) {
      value->object_id = lv4_u(op, "payload_object_id"); value->object_id_set = 1;
    }
  }
  int valid = value->kind != LV4_BAD || lv4_is(ins, "op", "copy") || lv4_is(ins, "op", "phi");
  if (valid) value->seed_state = 2;
  return valid;
}

static size_t lv4_kind_root(hako_lv4_function_index* fn, size_t index) {
  while (fn->values[index].kind_parent != index) {
    size_t parent = fn->values[index].kind_parent;
    fn->values[index].kind_parent = fn->values[parent].kind_parent;
    index = fn->values[index].kind_parent;
  }
  return index;
}

static int lv4_kind_unify(hako_lv4_function_index* fn, size_t index, uint32_t id) {
  hako_lv4_value* other = lv4_index_value(fn, id);
  if (!other) return 0;
  size_t left = lv4_kind_root(fn, index);
  size_t right = lv4_kind_root(fn, (size_t)(other - fn->values));
  if (left == right) return 1;
  int a = fn->values[left].kind, b = fn->values[right].kind;
  int text_a = a == LV4_TEXT || a == LV4_OWNED_STRING || a == LV4_TEXT_OPERAND;
  int text_b = b == LV4_TEXT || b == LV4_OWNED_STRING || b == LV4_TEXT_OPERAND;
  if ((text_a && (text_b || b == LV4_BAD)) || (text_b && a == LV4_BAD))
    a = b = LV4_TEXT_OPERAND;
  if (a != LV4_BAD && b != LV4_BAD && a != b) return 0;
  fn->values[right].kind_parent = left;
  fn->values[left].kind = a != LV4_BAD ? a : b;
  return 1;
}

static int lv4_index_kinds(const hako_lv4_input* in, hako_lv4_function_index* fn) {
  for (size_t i = 0; i < fn->value_count; i++) {
    fn->values[i].kind_parent = i;
    if (in->measurement) in->measurement->v4_index_resolves++;
    if (!lv4_index_seed(in, fn, &fn->values[i])) return 0;
  }
  for (size_t i = 0; i < fn->value_count; i++) {
    yyjson_val* ins = fn->values[i].instruction;
    if (lv4_is(ins, "op", "copy")) {
      if (!lv4_kind_unify(fn, i, lv4_u(ins, "src"))) return 0;
    } else if (lv4_is(ins, "op", "phi")) {
      yyjson_val* inputs = yyjson_obj_get(ins, "inputs");
      for (size_t j = 0; j < yyjson_arr_size(inputs); j++)
        if (!lv4_kind_unify(fn, i, lv4_u(yyjson_arr_get(inputs, j), "value"))) return 0;
    }
  }
  for (size_t i = 0; i < fn->value_count; i++)
    fn->values[i].kind_class = fn->values[lv4_kind_root(fn, i)].kind;
  for (size_t i = 0; i < fn->value_count; i++) {
    hako_lv4_value* value = &fn->values[i];
    int resolved = value->kind_class;
    if (resolved == LV4_TEXT_OPERAND) {
      if (lv4_is(value->instruction, "op", "const_string")) resolved = LV4_TEXT;
      else if (lv4_is(value->instruction, "op", "invoke_normal_result")) resolved = LV4_OWNED_STRING;
      else {
        if (fn->block_count + i > INT_MAX) return 0;
        fn->has_text_carriers = 1;
        value->origin = lv4_is(value->instruction, "op", "phi") ? (int)(fn->block_count + i) : -1;
      }
    }
    value->kind = resolved;
    if (value->kind == LV4_BAD) return 0;
  }
  for (size_t i = 0; i < fn->value_count; i++) {
    hako_lv4_value* value = &fn->values[i];
    if (!lv4_is(value->instruction, "op", "phi") ||
        value->kind == LV4_I64 || value->kind == LV4_BOOL || value->kind == LV4_TEXT_OPERAND) continue;
    hako_lv4_value* root = lv4_value_alias_root(fn, value);
    if (value->kind != LV4_HANDLE || !root || root == value || root->kind != LV4_HANDLE) return 0;
  }
  /* Kind equality grants no identity. Only original Copy and unanimous
   * HANDLE Phi paths lend the independently checked producer's provenance. */
  for (size_t i = 0; i < fn->value_count; i++) {
    hako_lv4_value* value = &fn->values[i];
    if (!lv4_is(value->instruction, "op", "copy") &&
        !(value->kind == LV4_HANDLE && lv4_is(value->instruction, "op", "phi"))) continue;
    if (value->kind == LV4_TEXT_OPERAND) continue;
    if (value->kind == LV4_MAP || value->kind == LV4_KEY || value->kind == LV4_OUTCOME ||
        value->kind == LV4_MAP_VIEW || value->kind == LV4_TEXT_VIEW || value->kind == LV4_OWNED_STRING)
      return 0;
    if (value->kind == LV4_I64 || value->kind == LV4_BOOL) continue;
    hako_lv4_value* source = lv4_value_alias_root(fn, value);
    if (!source || source->kind != value->kind) return 0;
    value->origin = source->origin;
    value->object_id = source->object_id;
    value->object_id_set = source->object_id_set;
  }
  return 1;
}

static void lv4_index_close(hako_lv4_input* in) {
  for (size_t fi = 0; fi < in->index_count; fi++) {
    free(in->index[fi].values);
    free(in->index[fi].blocks);
  }
  free(in->index); in->index = NULL; in->index_count = 0;
  free(in->layouts); in->layouts = NULL; in->layout_count = 0;
}

#include "hako_llvmc_ffi_lifecycle_v4_indexed_object_result.inc"

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
      if (lv4_u(yyjson_arr_get(fields, j), "storage_kind") != HAKO_LLVMC_LIFECYCLE_STORAGE_I64 &&
          lv4_u(yyjson_arr_get(fields, j), "storage_kind") != HAKO_LLVMC_LIFECYCLE_STORAGE_DYNAMIC) return 0;
  }
  if (in->layout_count) qsort(in->layouts, in->layout_count, sizeof(*in->layouts), lv4_layout_compare);
  for (size_t fi = 0; fi < functions; fi++) {
    if (in->measurement) in->measurement->v4_index_functions++;
    hako_lv4_function_index* fn = &in->index[fi];
    yyjson_val* body = yyjson_arr_get(in->functions, fi);
    int birth = lv4_is(body, "role", "birth_unit");
    int ordinary = lv4_is_ordinary(body);
    int ordinary_receiver = ordinary && yyjson_is_uint(yyjson_obj_get(body, "receiver"));
    yyjson_val* blocks = hako_physical_blocks(body);
    yyjson_val* params = yyjson_obj_get(body, "params");
    size_t capacity = lv4_value_budget(body);
    if (capacity == SIZE_MAX) return 0;
    capacity++;
    fn->function = body; fn->block_count = yyjson_arr_size(blocks);
    if (!fn->block_count || fn->block_count > INT_MAX || capacity > 4097) return 0;
    fn->values = lv4_checked_calloc(capacity, sizeof(*fn->values));
    fn->blocks = lv4_checked_calloc(fn->block_count, sizeof(*fn->blocks));
    if (!fn->values || !fn->blocks) return 0;
    if (in->measurement) in->measurement->v4_index_blocks += fn->block_count;
    if (birth || ordinary_receiver) {
      hako_lv4_value* receiver = &fn->values[fn->value_count++];
      receiver->id = lv4_u(body, "receiver"); receiver->kind = LV4_HANDLE;
      receiver->origin = -2; /* Borrowed physical receiver, never an allocation. */
      if (ordinary_receiver) {
        if (!yyjson_is_uint(yyjson_obj_get(body, "receiver_object")) ||
            !lv4_layout(in, lv4_u(body, "receiver_object"))) return 0;
        receiver->object_id = lv4_u(body, "receiver_object"); receiver->object_id_set = 1;
        fn->receiver_object = receiver->object_id;
        fn->receiver_object_set = 1;
      }
    }
    for (size_t pi = 0; pi < yyjson_arr_size(params); pi++) {
      hako_lv4_value* param = &fn->values[fn->value_count++];
      yyjson_val* row = yyjson_arr_get(params, pi);
      param->id = lv4_u(row, "value");
      if (ordinary && lv4_is(row, "representation", "map")) {
        /* Borrowed checked-map storage: caller-owned lease, never an
           allocation and never ended here. */
        param->kind = LV4_MAP; param->origin = -2;
      } else if (!ordinary && lv4_is(row, "representation", "nullable_kind_payload_v1")) {
        /* Nullable object formal (origin -3): a birth unit's
           object-field-provided formal carries a possibly-Void handle
           lane — the kind==0 prologue seals payload 0, so the value is
           never a lease and admits no consume. */
        param->kind = LV4_HANDLE; param->origin = -3;
      } else if (!ordinary && lv4_is(row, "representation", "i64")) {
        /* A source-qualified signed Dynamic I64 Birth formal keeps the
           existing kind/payload call ABI but exposes only its checked
           Integer payload to this body. */
        param->kind = LV4_I64; param->origin = -1;
      } else {
        /* The pair ABI is unchanged. Its checked Integer entry projection
           dominates every body use; Copy/Phi transport the projected i64. */
        param->kind = ordinary && yyjson_is_true(yyjson_obj_get(row, "bool_view")) ? LV4_BOOL :
            ordinary && (!lv4_is(row, "representation", "borrowed_kind_payload_v1") ||
            yyjson_is_true(yyjson_obj_get(row, "integer_view"))) ? LV4_I64 : LV4_TAGGED; param->origin = -1;
      }
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
        value->id = (uint32_t)yyjson_get_uint(dst); value->instruction = ins;
        value->definition_block = block; value->origin = -1;
      }
    }
    if (in->measurement) in->measurement->v4_index_values += fn->value_count;
    qsort(fn->values, fn->value_count, sizeof(*fn->values), lv4_value_compare);
    qsort(fn->blocks, fn->block_count, sizeof(*fn->blocks), lv4_block_compare);
    for (size_t i = 1; i < fn->value_count; i++)
      if (fn->values[i-1].id == fn->values[i].id) return 0;
    for (size_t i = 1; i < fn->block_count; i++)
      if (fn->blocks[i-1].id == fn->blocks[i].id) return 0;
  }
  return 1;
}

static int lv4_input_type(const hako_lv4_input* in, yyjson_val* fn, uint32_t value) {
  if (in->measurement) in->measurement->v4_type_lookups++;
  if (in->native) return lv4_type(fn, value, lv4_value_budget(fn));
  for (size_t fi = 0; fi < in->index_count; fi++) {
    if (in->index[fi].function != fn) continue;
    hako_lv4_value* row = lv4_index_value(&in->index[fi], value);
    return row ? row->kind : LV4_BAD;
  }
  return LV4_BAD;
}
