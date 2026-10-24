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
  else if (lv4_is(origin, "kind", "borrowed_map_slot")) {
    if (!lv4_is(function, "role", "ordinary_borrowed_nullable_handle")) return 0;
    fprintf(f, "  %%return_nonnull%u = icmp ne i64 %%v%u, 0\n"
        "  %%return_outcome%u = select i1 %%return_nonnull%u, i32 2, i32 0\n"
        "  store i32 %%return_outcome%u, ptr %%out_outcome, align 4\n",
        block, lv4_u(term, "value"), block, block, block);
  }
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

/* Validate finite membership without converting a mixed input to one class. */
static void lv4_emit_finite_parameter_guard(FILE* f, yyjson_val* param, uint32_t v) {
  unsigned mask = hako_physical_finite_domain(param), step = 0;
  fprintf(f, "  %%arg_domain%u_0 = icmp eq i32 0, 1\n", v);
  for (unsigned tag = 0; tag <= 4; tag++) {
    if (!(mask & (1u << tag))) continue;
    fprintf(f, "  %%arg_domain_tag%u_%u = icmp eq i32 %%k%u, %u\n", v,tag,v,tag);
    if (tag != 1) {
      fprintf(f, "  %%arg_domain_payload%u_%u = icmp %s i64 %%v%u, %u\n",
          v,tag, tag == 2 ? "ule" : tag == 4 ? "ne" : "eq", v, tag == 2 ? 1 : 0);
      fprintf(f, "  %%arg_domain_valid%u_%u = and i1 %%arg_domain_tag%u_%u, %%arg_domain_payload%u_%u\n",
          v,tag,v,tag,v,tag);
    } else fprintf(f, "  %%arg_domain_valid%u_%u = and i1 %%arg_domain_tag%u_%u, true\n", v,tag,v,tag);
    fprintf(f, "  %%arg_domain%u_%u = or i1 %%arg_domain%u_%u, %%arg_domain_valid%u_%u\n",
        v,step+1,v,step,v,tag);
    step++;
  }
  fprintf(f, "  br i1 %%arg_domain%u_%u, label %%arg_next%u, label %%invalid\narg_next%u:\n", v,step,v,v);
}

/* Every borrowed pair uses the same source-issued entry domain before effects. */
static void lv4_emit_borrowed_parameter_guard(FILE* f, const hako_lv4_input* in,
    yyjson_val* fn, yyjson_val* param, size_t i) {
    if (!lv4_is(param, "representation", "borrowed_kind_payload_v1")) return;
    uint32_t v = lv4_u(param, "value");
    yyjson_val* object_view = yyjson_obj_get(param, "object_view");
    if (yyjson_obj_get(param, "finite_domain")) {
      lv4_emit_finite_parameter_guard(f, param, v);
      return;
    }
    if (yyjson_is_true(yyjson_obj_get(param, "integer_view"))) {
      /* This is an ABI projection before the first body effect, not a
         coercion or a guarded mixed-domain numeric operation. */
      fprintf(f,
          "  %%arg_int%u = icmp eq i32 %%k%u, 1\n"
          "  br i1 %%arg_int%u, label %%arg_next%u, label %%invalid\narg_next%u:\n",
          v,v,v,v,v);
      return;
    }
    if (yyjson_is_true(yyjson_obj_get(param, "bool_view"))) {
      /* Exact Bool projection precedes every body effect, without I64 coercion. */
      fprintf(f,
          "  %%arg_bool%u = icmp eq i32 %%k%u, 2\n"
          "  %%arg_bit%u = icmp ule i64 %%v%u, 1\n"
          "  %%arg_ok%u = and i1 %%arg_bool%u, %%arg_bit%u\n"
          "  br i1 %%arg_ok%u, label %%arg_next%u, label %%invalid\narg_next%u:\n",
          v,v,v,v,v,v,v,v,v,v);
      return;
    }
    if (yyjson_is_uint(object_view)) {
      uint32_t runtime_type = lv4_u(lv4_layout(in, (uint32_t)yyjson_get_uint(object_view)), "runtime_type_id");
      /* Caller provenance remains independently verified. A class domain
         validates runtime class before any callee effect; no ownership change. */
      fprintf(f,
          "  %%arg_null%u = icmp eq i32 %%k%u, 0\n"
          "  %%arg_zero%u = icmp eq i64 %%v%u, 0\n"
          "  %%arg_nulok%u = and i1 %%arg_null%u, %%arg_zero%u\n"
          "  br i1 %%arg_nulok%u, label %%arg_next%u, label %%arg_object_check%u\n"
          "arg_object_check%u:\n"
          "  %%arg_object%u = icmp eq i32 %%k%u, 3\n"
          "  %%arg_nonzero%u = icmp ne i64 %%v%u, 0\n"
          "  %%arg_live%u = and i1 %%arg_object%u, %%arg_nonzero%u\n"
          "  br i1 %%arg_live%u, label %%arg_type_check%u, label %%invalid\n"
          "arg_type_check%u:\n"
          "  %%arg_type%u = call i64 @\"nyash.object.type_id_h\"(i64 %%v%u)\n"
          "  %%arg_class%u = icmp eq i64 %%arg_type%u, %u\n"
          "  br i1 %%arg_class%u, label %%arg_next%u, label %%invalid\narg_next%u:\n",
          v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,runtime_type,v,v,v);
      return;
    }
    if ((lv4_is(fn, "role", "ordinary_borrowed_nullable_handle") ||
        lv4_is(fn, "role", "ordinary_mixed_handle")) &&
        hako_physical_u32(yyjson_obj_get(fn, "result_formal_ordinal"), NULL) &&
        i == lv4_u(fn, "result_formal_ordinal")) {
      lv4_emit_borrowed_result_formal_guard(f, v);
      return;
    }
    if (yyjson_is_true(yyjson_obj_get(param, "text_view"))) {
      fprintf(f,
          "  %%arg_text%u = icmp eq i32 %%k%u, 4\n"
          "  %%arg_textptr%u = icmp ne i64 %%v%u, 0\n"
          "  %%arg_textok%u = and i1 %%arg_text%u, %%arg_textptr%u\n"
          "  br i1 %%arg_textok%u, label %%arg_next%u, label %%invalid\narg_next%u:\n",
          v,v, v,v, v,v,v, v,v,v);
      return;
    }
    /* Nonzero supplements the closed incoming/live proof; it is not that
       proof. The null pair (kind 0, payload 0) is the third admitted
       shape — every other kind/payload mix is ABI drift, never a
       silently nulled carrier. */
    fprintf(f,
        "  %%arg_int%u = icmp eq i32 %%k%u, 1\n"
        "  %%arg_bool%u = icmp eq i32 %%k%u, 2\n"
        "  %%arg_bit%u = icmp ule i64 %%v%u, 1\n"
        "  %%arg_valid_bool%u = and i1 %%arg_bool%u, %%arg_bit%u\n"
        "  %%arg_scalar%u = or i1 %%arg_int%u, %%arg_valid_bool%u\n"
        "  %%arg_object%u = icmp eq i32 %%k%u, 3\n"
        "  %%arg_nonzero%u = icmp ne i64 %%v%u, 0\n"
        "  %%arg_live%u = and i1 %%arg_object%u, %%arg_nonzero%u\n"
        "  %%arg_null%u = icmp eq i32 %%k%u, 0\n"
        "  %%arg_void%u = icmp eq i64 %%v%u, 0\n"
        "  %%arg_nulok%u = and i1 %%arg_null%u, %%arg_void%u\n"
        "  %%arg_pre%u = or i1 %%arg_scalar%u, %%arg_live%u\n"
        "  %%arg_ok%u = or i1 %%arg_pre%u, %%arg_nulok%u\n"
        "  br i1 %%arg_ok%u, label %%arg_next%u, label %%invalid\narg_next%u:\n",
        v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v);
}
