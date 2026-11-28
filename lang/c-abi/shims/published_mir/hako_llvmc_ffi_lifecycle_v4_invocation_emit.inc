/* Source-issued Dynamic calls and opaque carrier operations. The runtime owns
 * provider selection and token validity; this owner preserves descriptor class
 * and the MIR Normal/Fault boundary. */
static void lv4_invocation_globals(FILE* f, const hako_lv4_input* in) {
  fprintf(f, "%%nyrt.dynamic.operand = type { i32, i32, i64 }\n");
  for (size_t fi = 0; fi < yyjson_arr_size(in->functions); fi++) {
    yyjson_val* blocks = hako_physical_blocks(yyjson_arr_get(in->functions, fi));
    for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
      yyjson_val* block = yyjson_arr_get(blocks, bi);
      yyjson_val* op = yyjson_obj_get(lv4_term(block), "operation");
      if (!lv4_is(op, "kind", "dynamic_invoke")) continue;
      yyjson_val* selector = yyjson_obj_get(op, "selector");
      size_t len = yyjson_get_len(selector);
      const unsigned char* bytes = (const unsigned char*)yyjson_get_str(selector);
      fprintf(f, "@dynsel_%zu_%u = private unnamed_addr constant [%zu x i8] c\"",
          fi, lv4_u(block, "id"), len);
      for (size_t i = 0; i < len; i++) fprintf(f, "\\%02X", bytes[i]);
      fprintf(f, "\", align 1\n");
    }
  }
}

static void lv4_invocation_declarations(FILE* f) {
  fprintf(f,
      "declare i32 @\"nyash.dynamic.checked_invoke_v1\"(ptr, i64, ptr, ptr, i64, ptr, i64, ptr)\n"
      "declare i32 @\"nyash.dynamic.checked_array_get_v1\"(ptr, i64, i64, i64, ptr)\n"
      "declare i32 @\"nyash.dynamic.checked_integer_v1\"(ptr, i64, i64, ptr)\n"
      "declare i32 @\"nyash.dynamic.checked_borrowed_integer_v1\"(ptr, i64, i32, i64, ptr)\n"
      "declare i32 @\"nyash.dynamic.checked_materialize_borrowed_v1\"(ptr, i64, i32, i64, ptr)\n"
      "declare i32 @\"nyash.dynamic.checked_sub_i64_v1\"(ptr, i64, i64, i64, ptr)\n"
      "declare i32 @\"nyash.dynamic.checked_publish_text_or_null_v1\"(ptr, i64, i64, i64, ptr)\n"
      "declare i32 @\"nyash.dynamic.checked_is_null_v1\"(ptr, i64, i64, ptr)\n"
      "declare i32 @\"nyash.dynamic.scalar_equal_v1\"(ptr, i32, i32, i64, i32, i32, i64)\n"
      "declare i32 @\"nyash.dynamic.checked_end_v1\"(ptr, i64, i64)\n");
}

static void lv4_invocation_alloca(FILE* f, yyjson_val* op, uint32_t block) {
  if (lv4_is(op, "kind", "dynamic_invoke")) {
    size_t count = yyjson_arr_size(yyjson_obj_get(op, "arguments"));
    fprintf(f, "  %%dynrecv%u = alloca %%nyrt.dynamic.operand, align 8\n", block);
    if (count) fprintf(f,
        "  %%dynargs%u = alloca [%zu x %%nyrt.dynamic.operand], align 8\n", block, count);
    fprintf(f, "  %%dynout%u = alloca i64, align 8\n", block);
  } else if (lv4_is(op, "kind", "dynamic_array_get") ||
      (lv4_is(op, "kind", "ordinary_call") && lv4_is(op, "result", "nullable_text"))) {
    fprintf(f, "  %%dynout%u = alloca i64, align 8\n", block);
  } else if (lv4_is(op, "kind", "dynamic_carrier_integer") ||
      lv4_is(op, "kind", "dynamic_borrowed_integer")) {
    fprintf(f, "  %%dynint%u = alloca i64, align 8\n", block);
  } else if (lv4_is(op, "kind", "dynamic_carrier_sub_i64") ||
      lv4_is(op, "kind", "dynamic_materialize_borrowed")) {
    fprintf(f, "  %%dynout%u = alloca i64, align 8\n", block);
  }
}

static int lv4_invocation_operand_emit(FILE* f, yyjson_val* operand,
    size_t function, uint32_t block, size_t ordinal, const char* destination) {
  int kind;
  const char* tag = "0";
  const char* payload = "0";
  uint32_t value = lv4_u(operand, "value");
  char tag_value[32], payload_value[32];
  if (lv4_is(operand, "kind", "null")) kind = 1;
  else if (lv4_is(operand, "kind", "integer")) {
    kind = 2; snprintf(payload_value, sizeof(payload_value), "%%v%u", value); payload = payload_value;
  } else if (lv4_is(operand, "kind", "bool")) {
    kind = 3; snprintf(payload_value, sizeof(payload_value), "%%v%u", value); payload = payload_value;
  } else if (lv4_is(operand, "kind", "tagged_dynamic")) {
    kind = 1; snprintf(tag_value, sizeof(tag_value), "%%k%u", value); tag = tag_value;
    snprintf(payload_value, sizeof(payload_value), "%%v%u", value); payload = payload_value;
  } else if (lv4_is(operand, "kind", "borrowed_typed_object")) {
    kind = 1; tag = "5";
    snprintf(payload_value, sizeof(payload_value), "%%v%u", value); payload = payload_value;
  } else if (ordinal && lv4_is(operand, "kind", "borrowed_text")) {
    kind = 1; tag = "4";
    lv4_string_operand_emit(f, yyjson_obj_get(operand, "operand"),
        function, block, (unsigned)(ordinal - 1));
    fprintf(f, "  %%dyntext%u_%zu = ptrtoint ptr %%strarg%u_%zu to i64\n",
        block, ordinal, block, ordinal - 1);
    snprintf(payload_value, sizeof(payload_value), "%%dyntext%u_%zu", block, ordinal);
    payload = payload_value;
  } else if (lv4_is(operand, "kind", "dynamic_carrier")) {
    kind = 4; snprintf(payload_value, sizeof(payload_value), "%%v%u", value); payload = payload_value;
  } else return 0;
  fprintf(f,
      "  %%dyn%u_%zu_k = insertvalue %%nyrt.dynamic.operand zeroinitializer, i32 %d, 0\n"
      "  %%dyn%u_%zu_t = insertvalue %%nyrt.dynamic.operand %%dyn%u_%zu_k, i32 %s, 1\n"
      "  %%dyn%u_%zu_v = insertvalue %%nyrt.dynamic.operand %%dyn%u_%zu_t, i64 %s, 2\n"
      "  store %%nyrt.dynamic.operand %%dyn%u_%zu_v, ptr %s, align 8\n",
      block, ordinal, kind, block, ordinal, block, ordinal, tag,
      block, ordinal, block, ordinal, payload, block, ordinal, destination);
  return !ferror(f);
}

static int lv4_equality_operand_emit(FILE* f, hako_lv4_value* value, int dynamic) {
  uint32_t id = value->id;
  if (dynamic && value->kind != LV4_DYNAMIC_CARRIER) {
    fprintf(f, ", i32 1, i32 %%k%u, i64 %%v%u", id, id);
    return 1;
  }
  int kind = value->kind == LV4_I64 ? 2 : value->kind == LV4_BOOL ? 3
      : value->kind == LV4_DYNAMIC_CARRIER ? 4 : 0;
  if (!kind) return 0;
  fprintf(f, ", i32 %d, i32 0, i64 %%v%u", kind, id);
  return 1;
}

static int lv4_equality_emit(FILE* f, const hako_lv4_input* in, size_t fi, yyjson_val* ins) {
  if (in->native) return 0;
  uint32_t dst = lv4_u(ins, "dst");
  fprintf(f, "  %%eq%u = call i32 @\"nyash.dynamic.scalar_equal_v1\"(ptr %s",
      dst, in->dispatch ? "@dispatch_program" : "null");
  hako_lv4_function_index* fn = &in->index[fi];
  hako_lv4_value* lhs = lv4_index_value(fn, lv4_u(ins, "lhs"));
  hako_lv4_value* rhs = lv4_index_value(fn, lv4_u(ins, "rhs"));
  int side = hako_physical_equality_dynamic_side(ins);
  if (side < 0 || !lhs || !rhs || !lv4_equality_operand_emit(f, lhs, side == 0) ||
      !lv4_equality_operand_emit(f, rhs, side == 1)) return 0;
  fprintf(f, ")\n  %%eq_valid%u = icmp ult i32 %%eq%u, 2\n"
      "  br i1 %%eq_valid%u, label %%eq_ok%u, label %%invalid\neq_ok%u:\n",
      dst, dst, dst, dst, dst);
  if (lv4_is(ins, "predicate", "ne")) {
    fprintf(f, "  %%ne%u = xor i32 %%eq%u, 1\n  %%v%u = zext i32 %%ne%u to i64\n",
        dst, dst, dst, dst);
  } else {
    fprintf(f, "  %%v%u = zext i32 %%eq%u to i64\n", dst, dst);
  }
  return !ferror(f);
}

/* -1 is another operation family, 0 is invalid, 1 emits both MIR edges. */
static int lv4_invocation_emit(FILE* f, const hako_lv4_input* in, yyjson_val* op, size_t function,
    uint32_t block, const char* normal, const char* fault) {
  unsigned long long site;
  if (!(lv4_is(op, "kind", "dynamic_invoke") ||
      lv4_is(op, "kind", "dynamic_array_get") ||
      lv4_is(op, "kind", "dynamic_carrier_integer") ||
      lv4_is(op, "kind", "dynamic_borrowed_integer") ||
      lv4_is(op, "kind", "dynamic_materialize_borrowed") ||
      lv4_is(op, "kind", "dynamic_carrier_sub_i64") ||
      lv4_is(op, "kind", "dynamic_carrier_end"))) return -1;
  site = yyjson_get_uint(yyjson_obj_get(op, "site"));
  if (lv4_is(op, "kind", "dynamic_invoke")) {
    yyjson_val* arguments = yyjson_obj_get(op, "arguments");
    size_t count = yyjson_arr_size(arguments);
    size_t len = yyjson_get_len(yyjson_obj_get(op, "selector"));
    char receiver[40], argument_array[40];
    snprintf(receiver, sizeof(receiver), "%%dynrecv%u", block);
    if (!lv4_invocation_operand_emit(f, yyjson_obj_get(op, "receiver"), function, block, 0, receiver)) return 0;
    if (count) snprintf(argument_array, sizeof(argument_array), "%%dynargs%u", block);
    else snprintf(argument_array, sizeof(argument_array), "null");
    for (size_t i = 0; i < count; i++) {
      char destination[64];
      snprintf(destination, sizeof(destination), "%%dynargptr%u_%zu", block, i);
      fprintf(f,
          "  %%dynargptr%u_%zu = getelementptr inbounds [%zu x %%nyrt.dynamic.operand], ptr %%dynargs%u, i64 0, i64 %zu\n",
          block, i, count, block, i);
      if (!lv4_invocation_operand_emit(f, yyjson_arr_get(arguments, i), function, block, i + 1, destination)) return 0;
    }
    fprintf(f,
        "  %%dynselector%u = getelementptr inbounds [%zu x i8], ptr @dynsel_%zu_%u, i64 0, i64 0\n"
        "  %%dynstatus%u = call i32 @\"%s\"(ptr %%frame, i64 %llu, %sptr %%dynrecv%u, ptr %%dynselector%u, i64 %zu, ptr %s, i64 %zu, ptr %%dynout%u)\n"
        "  switch i32 %%dynstatus%u, label %%invalid [ i32 0, label %%dynnormal%u i32 1, label %%%s ]\ndynnormal%u:\n"
        "  br label %%%s\n",
        block, len, function, block, block,
        in->dispatch ? "nyash.dynamic.checked_invoke_program_v1" : "nyash.dynamic.checked_invoke_v1",
        site, in->dispatch ? "ptr @dispatch_program, " : "", block, block, len,
        argument_array, count, block, block, block, fault, block, normal);
    return !ferror(f);
  }
  if (lv4_is(op, "kind", "dynamic_array_get")) {
    fprintf(f,
        "  %%dynstatus%u = call i32 @\"nyash.dynamic.checked_array_get_v1\"(ptr %%frame, i64 %llu, i64 %%v%u, i64 %%v%u, ptr %%dynout%u)\n"
        "  switch i32 %%dynstatus%u, label %%invalid [ i32 0, label %%dynnormal%u i32 1, label %%%s ]\ndynnormal%u:\n"
        "  br label %%%s\n",
        block, site, lv4_u(op, "array"), lv4_u(op, "index"), block,
        block, block, fault, block, normal);
    return !ferror(f);
  }
  if (lv4_is(op, "kind", "dynamic_carrier_integer")) {
    fprintf(f,
        "  %%dynstatus%u = call i32 @\"nyash.dynamic.checked_integer_v1\"(ptr %%frame, i64 %llu, i64 %%v%u, ptr %%dynint%u)\n"
        "  switch i32 %%dynstatus%u, label %%invalid [ i32 0, label %%dynnormal%u i32 1, label %%%s ]\ndynnormal%u:\n"
        "  br label %%%s\n",
        block, site, lv4_u(op, "carrier"), block, block, block, fault, block, normal);
    return !ferror(f);
  }
  if (lv4_is(op, "kind", "dynamic_borrowed_integer") ||
      lv4_is(op, "kind", "dynamic_materialize_borrowed")) {
    uint32_t tagged = lv4_u(op, "tagged");
    int materializes = lv4_is(op, "kind", "dynamic_materialize_borrowed");
    const char* checked = materializes
        ? "nyash.dynamic.checked_materialize_borrowed_v1" : "nyash.dynamic.checked_borrowed_integer_v1";
    fprintf(f,
        "  %%dynstatus%u = call i32 @\"%s\"(ptr %%frame, i64 %llu, i32 %%k%u, i64 %%v%u, ptr %%%s%u)\n"
        "  switch i32 %%dynstatus%u, label %%invalid [ i32 0, label %%dynnormal%u i32 1, label %%%s ]\ndynnormal%u:\n"
        "  br label %%%s\n",
        block, checked, site, tagged, tagged, materializes ? "dynout" : "dynint", block,
        block, block, fault, block, normal);
    return !ferror(f);
  }
  if (lv4_is(op, "kind", "dynamic_carrier_sub_i64")) {
    fprintf(f,
        "  %%dynstatus%u = call i32 @\"nyash.dynamic.checked_sub_i64_v1\"(ptr %%frame, i64 %llu, i64 %%v%u, i64 %%v%u, ptr %%dynout%u)\n"
        "  switch i32 %%dynstatus%u, label %%invalid [ i32 0, label %%dynnormal%u i32 1, label %%%s ]\ndynnormal%u:\n"
        "  br label %%%s\n",
        block, site, lv4_u(op, "carrier"), lv4_u(op, "rhs"), block,
        block, block, fault, block, normal);
    return !ferror(f);
  }
  fprintf(f,
      "  %%dynstatus%u = call i32 @\"nyash.dynamic.checked_end_v1\"(ptr %%frame, i64 %llu, i64 %%v%u)\n"
      "  switch i32 %%dynstatus%u, label %%invalid [ i32 0, label %%%s i32 1, label %%%s ]\n",
      block, site, lv4_u(op, "carrier"), block, normal, fault);
  return !ferror(f);
}

/* Ordinary Call actual preparation; preserves the exact source operand order. */
static void lv4_emit_call_operand_preparation(FILE* f, const hako_lv4_input* in,
    size_t fi, yyjson_val* call, yyjson_val* args, uint32_t b, size_t ri,
    unsigned long long site, const char* fault_label) {
  /* Checked Call never dispatches a null receiver. It records one source
     TypeError and uses the operation's existing caller-owned Fault cleanup. */
  if (yyjson_is_uint(yyjson_obj_get(call, "receiver"))) {
    fprintf(f,
        "  %%receiver_live%u = icmp ne i64 %%v%u, 0\n"
        "  br i1 %%receiver_live%u, label %%receiver_call%u, label %%receiver_null%u\nreceiver_null%u:\n"
        "  %%receiver_status%u = call i32 @\"nyash.fault.record_static_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 0, i64 0)\n"
        "  switch i32 %%receiver_status%u, label %%invalid [ i32 1, label %%%s ]\nreceiver_call%u:\n",
        b, lv4_u(call, "receiver"), b, b, b, b, b,
        NYRT_FAULT_REASON_STRING_TYPE_ERROR_V1, site, b, fault_label, b);
  }
        for (size_t i = 0; i < yyjson_arr_size(args); i++) {
          yyjson_val* arg = yyjson_arr_get(args, i);
          if (!lv4_is(arg, "kind", "borrowed_text")) continue;
          lv4_string_operand_emit(f, yyjson_obj_get(arg, "operand"), fi, b, (unsigned)i);
          fprintf(f, "  %%textbits%u_%zu = ptrtoint ptr %%strarg%u_%zu to i64\n", b,i,b,i);
        }
        lv4_emit_nullable_actual_tags(f, in, args, b);
        yyjson_val* callee = yyjson_arr_get(in->functions, lv4_u(call, "target"));
        yyjson_val* formals = yyjson_obj_get(callee, "params");
        for (size_t i = 0; i < yyjson_arr_size(args); i++) {
          yyjson_val* arg = yyjson_arr_get(args, i);
          if (!lv4_is(arg, "kind", "tagged") ||
              !yyjson_is_true(yyjson_obj_get(yyjson_arr_get(formals, i), "integer_view"))) continue;
          /* A take actual is modeled as consumed on both call edges, but
             a failing guard never enters the callee. Keep that case on
             its existing path until it has a pre-entry cleanup edge. */
          int record_type_error = !in->index[lv4_u(call, "target")].has_owned_parameters;
          lv4_emit_tagged_integer_check(f, lv4_u(arg, "value"), b, ri, i,
              fault_label, record_type_error);
        }

}
