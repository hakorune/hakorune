/* One invocation's original owned Handle result correspondence.
 * Kind unions and release metadata are never object identity authorities. */
static int lv4_owned_return_object(const hako_lv4_input* in,
    hako_lv4_function_index* fn);

static int lv4_owned_value_object(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, int allow_nullable) {
  if (!value) return 0;
  /* Kind closure follows dependency classification. Only a separate original
   * Copy/unanimous-Phi root may lend provisional object correspondence while
   * the alias kind is BAD; this never fixes or overrides its union kind. */
  hako_lv4_value* root = lv4_value_alias_root(fn, value);
  if (!root || root->kind != LV4_HANDLE || root->origin < 0 ||
      (size_t)root->origin >= fn->block_count) return 0;
  if (value->kind == LV4_BAD) {
    if (root == value || !(lv4_is(value->instruction, "op", "copy") ||
        lv4_is(value->instruction, "op", "phi"))) return 0;
  } else if (value->kind != LV4_HANDLE || root->origin != value->origin) return 0;
  if (!lv4_is(root->instruction, "op", "invoke_normal_result") ||
      !root->definition_block) return 0;
  hako_lv4_block* producer = lv4_index_block(fn, lv4_u(root->instruction, "invoke_block"));
  yyjson_val* term = producer ? lv4_term(producer->block) : NULL;
  yyjson_val* op = yyjson_obj_get(term, "operation");
  if (!producer || !lv4_is(term, "op", "invoke") ||
      root->origin != (int)producer->ordinal ||
      lv4_u(term, "normal") != lv4_u(root->definition_block, "id")) return 0;
  uint32_t object;
  if (lv4_is(op, "kind", "new_box")) {
    object = lv4_u(op, "object_id");
    if (!root->object_id_set || root->object_id != object || !lv4_layout(in, object))
      return 0;
  } else if (lv4_is(op, "kind", "ordinary_call") &&
      (lv4_is(op, "result", "handle") || lv4_is(op, "result", "nullable_handle"))) {
    int nullable = lv4_is(op, "result", "nullable_handle");
    uint32_t target = lv4_u(yyjson_obj_get(op, "call"), "target");
    if ((nullable && !allow_nullable) || !target || target >= in->index_count ||
        !lv4_is(in->index[target].function, "role", nullable ? "ordinary_nullable_handle" : "ordinary_handle") ||
        !lv4_owned_return_object(in, &in->index[target])) return 0;
    object = in->index[target].return_object;
    if (root->object_id_set && root->object_id != object) return 0;
    root->object_id = object; root->object_id_set = 1;
  } else return 0;
  if (value->object_id_set && value->object_id != object) return 0;
  value->object_id = object; value->object_id_set = 1;
  return 1;
}

static int lv4_owned_return_object(const hako_lv4_input* in,
    hako_lv4_function_index* fn) {
  int nullable = lv4_is(fn->function, "role", "ordinary_nullable_handle");
  if (!nullable && !lv4_is(fn->function, "role", "ordinary_handle")) return 0;
  if (fn->return_object_state == 2) return 1;
  if (fn->return_object_state == 1) return 0;
  fn->return_object_state = 1;
  int found = 0;
  uint32_t object = 0;
  for (size_t bi = 0; bi < fn->block_count; bi++) {
    yyjson_val* term = lv4_term(fn->blocks[bi].block);
    if (!lv4_is(term, "op", "return")) continue;
    hako_lv4_value* result = lv4_index_value(fn, lv4_u(term, "value"));
    hako_lv4_value* root = lv4_value_alias_root(fn, result);
    /* Null carries no object identity. Only this exact source constant may
       be excluded, and only the nullable result owner admits it. */
    if (nullable && root && root->kind == LV4_HANDLE &&
        lv4_is(root->instruction, "op", "const_null")) continue;
    if (!lv4_owned_value_object(in, fn, result, nullable) ||
        (found && object != result->object_id)) return 0;
    object = result->object_id; found = 1;
  }
  if (!found) return 0;
  fn->return_object = object; fn->return_object_state = 2;
  return 1;
}

/* A live NewBox is not yet a completed object. Require the one matching
 * constructor's successful edge to dominate the requested use. */
static int lv4_original_birth_completed(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* acquired, uint32_t object, uint32_t use) {
  if (!acquired || !acquired->object_id_set || acquired->object_id != object ||
      !lv4_is(acquired->instruction, "op", "invoke_normal_result")) return 0;
  hako_lv4_block* allocation = lv4_index_block(fn, lv4_u(acquired->instruction, "invoke_block"));
  yyjson_val* creation = allocation ? yyjson_obj_get(lv4_term(allocation->block), "operation") : NULL;
  if (!lv4_is(creation, "kind", "new_box") || lv4_u(creation, "object_id") != object) return 0;
  size_t births = 0;
  for (size_t bi = 0; bi < fn->block_count; bi++) {
    yyjson_val* birth = lv4_term(fn->blocks[bi].block);
    yyjson_val* op = yyjson_obj_get(birth, "operation");
    yyjson_val* call = yyjson_obj_get(op, "call");
    if (!lv4_is(op, "kind", "birth_call") ||
        lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(call, "receiver"))) != acquired) continue;
    uint32_t target = lv4_u(call, "target");
    if (++births != 1 || !target || target >= in->index_count ||
        !lv4_is(in->index[target].function, "role", "birth_unit") ||
        !in->index[target].receiver_object_set || in->index[target].receiver_object != object ||
        lv4_pred_count(fn, lv4_u(birth, "normal")) != 1 ||
        !hako_physical_block_dominates(fn->function, lv4_u(birth, "normal"), use)) return 0;
  }
  return births == 1;
}

#include "hako_llvmc_ffi_lifecycle_v4_indexed_object_return_origin.inc"
#include "hako_llvmc_ffi_lifecycle_v4_indexed_borrowed_return_origin.inc"
#include "hako_llvmc_ffi_lifecycle_v4_indexed_field_dependencies.inc"
#include "hako_llvmc_ffi_lifecycle_v4_indexed_mixed_return_origin.inc"

static int lv4_index_owned_results(const hako_lv4_input* in) {
  for (size_t fi = 0; fi < in->index_count; fi++) {
    hako_lv4_function_index* fn = &in->index[fi];
    int borrowed = lv4_is(fn->function, "role", "ordinary_borrowed_nullable_handle");
    if (lv4_is(fn->function, "role", "ordinary_mixed_handle")) {
      if (!lv4_mixed_return_origins(in, fn)) return 0;
      continue;
    }
    if (!borrowed && !hako_physical_object_result_role(fn->function)) continue;
    size_t returns = 0;
    for (size_t bi = 0; bi < fn->block_count; bi++) {
      yyjson_val* term = lv4_term(fn->blocks[bi].block);
      if (lv4_is(term, "op", "return")) returns++;
      if (lv4_is(term, "op", "return") &&
          !(borrowed ? lv4_borrowed_return_origin(in, fn, term) :
              lv4_owned_return_origin(in, fn, term) && lv4_return_field_dependencies(in, fn, term))) return 0;
    }
    hako_physical_slot_support support;
    if (borrowed && hako_physical_result_slot(fn->function, &support)) {
      if (!returns) return 0;
      fn->return_object = support.payload_object;
      fn->return_object_state = 2; /* All original Returns proved nonnull slot payloads. */
    }
  }

  for (size_t fi = 0; fi < in->index_count; fi++)
    if (!lv4_mixed_birth_dependencies(in, &in->index[fi])) return 0;

  /* Every callee index is complete before following any original Call edge. */
  for (size_t fi = 0; fi < in->index_count; fi++) {
    hako_lv4_function_index* fn = &in->index[fi];
    if (lv4_is(fn->function, "role", "ordinary_handle") &&
        !lv4_owned_return_object(in, fn)) return 0;
  }
  for (size_t fi = 0; fi < in->index_count; fi++) {
    hako_lv4_function_index* fn = &in->index[fi];
    for (size_t vi = 0; vi < fn->value_count; vi++) {
      hako_lv4_value* value = &fn->values[vi];
      if (value->kind != LV4_HANDLE || value->origin < 0) continue;
      hako_lv4_value* root = lv4_value_alias_root(fn, value);
      if (!root || !lv4_is(root->instruction, "op", "invoke_normal_result")) continue;
      hako_lv4_block* producer = lv4_index_block(fn, lv4_u(root->instruction, "invoke_block"));
      yyjson_val* op = producer ? yyjson_obj_get(lv4_term(producer->block), "operation") : NULL;
      if (lv4_is(op, "kind", "ordinary_call") && lv4_is(op, "result", "handle") &&
          !lv4_owned_value_object(in, fn, value, 0)) return 0;
    }
  }
  return 1;
}
