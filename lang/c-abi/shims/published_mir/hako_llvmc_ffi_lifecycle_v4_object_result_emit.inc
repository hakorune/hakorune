/* Internal result outcome transport. Borrowed0/2 connection remains unaccepted
 * until its complete consumer/source execution is verified.
 * Payload and outcome share one Normal producer; Fault never reads the slots. */
/* The constructor lends its original nullable formal, including exact Copies.
 * This is storage correspondence after flow admission, never a source issuer. */
static int lv4_object_residence_formal(const hako_lv4_input* in,
    yyjson_val* fn, uint32_t value, uint32_t* formal) {
  if (in->native || !lv4_is(fn, "role", "birth_unit")) return 0;
  for (size_t fi = 0; fi < in->index_count; fi++) {
    hako_lv4_function_index* index = &in->index[fi];
    if (index->function != fn) continue;
    hako_lv4_value* root = lv4_value_alias_root(index, lv4_index_value(index, value));
    if (!root) return -1;
    yyjson_val* params = yyjson_obj_get(fn, "params");
    for (size_t i = 0; i < yyjson_arr_size(params); i++) {
      yyjson_val* param = yyjson_arr_get(params, i);
      if (lv4_u(param, "value") == root->id &&
          lv4_is(param, "representation", "nullable_kind_payload_v1")) {
        *formal = root->id;
        return 1;
      }
    }
    return root->origin == -3 ? -1 : 0;
  }
  return -1;
}

/* Borrow only an already-indexed object's original Copy root. No receiver,
 * ordinary borrowed formal, field borrow or Array gets an invented outcome. */
static int lv4_emit_object_value_outcome(FILE* f, const hako_lv4_input* in,
    yyjson_val* function, uint32_t value, const char* output) {
  uint32_t formal;
  int source = lv4_object_residence_formal(in, function, value, &formal);
  if (source < 0) return 0;
  if (source > 0) {
    fprintf(f, "  %%%s = add i32 0, %%o%u\n", output, formal);
    return !ferror(f);
  }
  for (size_t fi = 0; fi < in->index_count; fi++) {
    hako_lv4_function_index* fn = &in->index[fi];
    if (fn->function != function) continue;
    hako_lv4_value* root = lv4_value_alias_root(fn, lv4_index_value(fn, value));
    if (!root || (root->kind != LV4_HANDLE && root->kind != LV4_MIXED_OBJECT)) return 0;
    if (lv4_is(root->instruction, "op", "const_null"))
      fprintf(f, "  %%%s = add i32 0, 0\n", output);
    else if (lv4_is(root->instruction, "op", "invoke_normal_result")) {
      hako_lv4_block* producer = lv4_index_block(fn, lv4_u(root->instruction, "invoke_block"));
      yyjson_val* operation = producer ? yyjson_obj_get(lv4_term(producer->block), "operation") : NULL;
      if (hako_physical_object_result_call(operation))
        fprintf(f, "  %%%s = add i32 0, %%o%u\n", output, root->id);
      else if (lv4_is(operation, "kind", "new_box"))
        fprintf(f, "  %%%s = add i32 0, 1\n", output);
      else return 0;
    } else return 0;
    return !ferror(f);
  }
  return 0;
}

static void lv4_emit_object_result_normal(FILE* f, yyjson_val* operation,
    uint32_t invoke, uint32_t value) {
  fprintf(f, "  %%o%u = load i32, ptr %%call_outcome%u, align 4\n", value, invoke);
  fprintf(f, "  %%object_nonnull%u = icmp ne i64 %%v%u, 0\n", value, value);
  if (lv4_is(operation, "result", "mixed_handle")) {
    fprintf(f,
        "  %%object_zero%u = icmp eq i32 %%o%u, 0\n"
        "  %%object_null%u = xor i1 %%object_nonnull%u, true\n"
        "  %%object_pair%u = icmp eq i1 %%object_zero%u, %%object_null%u\n"
        "  %%object_domain%u = icmp ule i32 %%o%u, 2\n"
        "  %%object_ok%u = and i1 %%object_pair%u, %%object_domain%u\n",
        value,value, value,value, value,value,value, value,value, value,value,value);
  } else {
  fprintf(f, "  %%object_expected%u = select i1 %%object_nonnull%u, i32 %u, i32 0\n",
      value, value, lv4_is(operation, "result", "borrowed_nullable_handle") ? 2u : 1u);
  fprintf(f, "  %%object_pair%u = icmp eq i32 %%o%u, %%object_expected%u\n", value, value, value);
  fprintf(f, "  %%object_ok%u = and i1 %%object_pair%u, ", value, value);
  if (lv4_is(operation, "result", "handle")) fprintf(f, "%%object_nonnull%u\n", value);
  else fprintf(f, "true\n");
  }
  fprintf(f, "  br i1 %%object_ok%u, label %%object_next%u, label %%invalid\nobject_next%u:\n",
      value, value, value);
}

static int lv4_emit_object_return_outcome(FILE* f, const hako_lv4_input* in,
    yyjson_val* function, yyjson_val* term, uint32_t block) {
  yyjson_val* origin = yyjson_obj_get(term, "result_origin");
  if (lv4_is(origin, "kind", "null")) fprintf(f, "  store i32 0, ptr %%out_outcome, align 4\n");
  else if (lv4_is(origin, "kind", "fresh")) fprintf(f, "  store i32 1, ptr %%out_outcome, align 4\n");
  else if (lv4_is(origin, "kind", "borrowed_formal")) {
    if (!lv4_is(function, "role", "ordinary_borrowed_nullable_handle")) return 0;
    uint32_t value = lv4_u(term, "value");
    fprintf(f,
        "  %%return_nonnull%u = icmp ne i64 %%v%u, 0\n"
        "  %%return_tag%u = select i1 %%return_nonnull%u, i32 3, i32 0\n"
        "  %%return_pair%u = icmp eq i32 %%k%u, %%return_tag%u\n"
        "  br i1 %%return_pair%u, label %%return_borrowed%u, label %%invalid\nreturn_borrowed%u:\n"
        "  %%return_outcome%u = select i1 %%return_nonnull%u, i32 2, i32 0\n"
        "  store i32 %%return_outcome%u, ptr %%out_outcome, align 4\n",
        block,value, block,block, block,value,block, block,block,block, block,block, block);
  }
  else if (lv4_is(origin, "kind", "owned_call") || lv4_is(origin, "kind", "borrowed_call")) {
    hako_lv4_function_index* fn = NULL;
    for (size_t i = 0; i < in->index_count; i++)
      if (in->index[i].function == function) fn = &in->index[i];
    yyjson_val* producer = fn ? lv4_result_coordinate(fn, yyjson_obj_get(origin, "producer"), NULL) : NULL;
    if (!producer || !lv4_is(producer, "op", "invoke_normal_result")) return 0;
    fprintf(f, "  store i32 %%o%u, ptr %%out_outcome, align 4\n", lv4_u(producer, "dst"));
  } else return 0;
  return !ferror(f);
}

/* A nullable forwarded formal has the object/null domain even without a
 * concrete class constraint. Validate it before the first callee effect. */
static void lv4_emit_borrowed_result_formal_guard(FILE* f, uint32_t v) {
  fprintf(f,
      "  %%arg_zero%u = icmp eq i64 %%v%u, 0\n"
      "  %%arg_expected%u = select i1 %%arg_zero%u, i32 0, i32 3\n"
      "  %%arg_ok%u = icmp eq i32 %%k%u, %%arg_expected%u\n"
      "  br i1 %%arg_ok%u, label %%arg_next%u, label %%invalid\narg_next%u:\n",
      v,v, v,v, v,v,v, v,v,v);
}
