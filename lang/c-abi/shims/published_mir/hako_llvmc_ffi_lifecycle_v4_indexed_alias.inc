/* Original physical identity, separate from kind unions and lease state.
 * Structural roots lend no kind or Home. Full kind closure precedes flow. */
static hako_lv4_value* lv4_index_alias_root(hako_lv4_function_index* fn,
    hako_lv4_value* value, size_t remaining) {
  if (!value || !remaining || value->alias_state == 1 || value->alias_state == 3) return NULL;
  if (value->alias_state == 2) return &fn->values[value->alias_root];
  value->alias_state = 1;
  hako_lv4_value* root = value;
  yyjson_val* ins = value->instruction;
  if (lv4_is(ins, "op", "copy")) {
    root = lv4_index_alias_root(fn, lv4_index_value(fn, lv4_u(ins, "src")), remaining - 1);
  } else if (lv4_is(ins, "op", "phi")) {
    yyjson_val* inputs = yyjson_obj_get(ins, "inputs");
    if (!value->definition_block || !yyjson_arr_size(inputs) ||
        !hako_physical_phi_predecessors_exact(fn->function,
            lv4_u(value->definition_block, "id"), inputs)) root = NULL;
    else {
      root = NULL;
      for (size_t i = 0; i < yyjson_arr_size(inputs); i++) {
        hako_lv4_value* next = lv4_index_alias_root(fn,
            lv4_index_value(fn, lv4_u(yyjson_arr_get(inputs, i), "value")), remaining - 1);
        /* A differing or cyclic scalar Phi remains its own scalar producer.
         * Such a Phi is never a transparent HANDLE alias. */
        if (!next || (root && root != next)) { root = value; break; }
        root = next;
      }
    }
  }
  if (!root) { value->alias_state = 3; return NULL; }
  value->alias_root = (size_t)(root - fn->values);
  value->alias_state = 2;
  return root;
}

static hako_lv4_value* lv4_value_alias_root(hako_lv4_function_index* fn,
    hako_lv4_value* value) {
  hako_lv4_value* root = lv4_index_alias_root(fn, value, fn->value_count + 1);
  if (root && root->kind == LV4_HANDLE && lv4_is(root->instruction, "op", "phi")) return NULL;
  return root;
}
