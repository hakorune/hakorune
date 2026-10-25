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
      "declare i32 @\"nyash.dynamic.checked_sub_i64_v1\"(ptr, i64, i64, i64, ptr)\n"
      "declare i32 @\"nyash.dynamic.checked_end_v1\"(ptr, i64, i64)\n");
}

static void lv4_invocation_alloca(FILE* f, yyjson_val* op, uint32_t block) {
  if (lv4_is(op, "kind", "dynamic_invoke")) {
    size_t count = yyjson_arr_size(yyjson_obj_get(op, "arguments"));
    fprintf(f, "  %%dynrecv%u = alloca %%nyrt.dynamic.operand, align 8\n", block);
    if (count) fprintf(f,
        "  %%dynargs%u = alloca [%zu x %%nyrt.dynamic.operand], align 8\n", block, count);
    fprintf(f, "  %%dynout%u = alloca i64, align 8\n", block);
  } else if (lv4_is(op, "kind", "dynamic_array_get")) {
    fprintf(f, "  %%dynout%u = alloca i64, align 8\n", block);
  } else if (lv4_is(op, "kind", "dynamic_carrier_integer")) {
    fprintf(f, "  %%dynint%u = alloca i64, align 8\n", block);
  } else if (lv4_is(op, "kind", "dynamic_carrier_sub_i64")) {
    fprintf(f, "  %%dynout%u = alloca i64, align 8\n", block);
  }
}

static int lv4_invocation_operand_emit(FILE* f, yyjson_val* operand,
    uint32_t block, size_t ordinal, const char* destination) {
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

/* -1 is another operation family, 0 is invalid, 1 emits both MIR edges. */
static int lv4_invocation_emit(FILE* f, yyjson_val* op, size_t function,
    uint32_t block, const char* normal, const char* fault) {
  unsigned long long site;
  if (!(lv4_is(op, "kind", "dynamic_invoke") ||
      lv4_is(op, "kind", "dynamic_array_get") ||
      lv4_is(op, "kind", "dynamic_carrier_integer") ||
      lv4_is(op, "kind", "dynamic_carrier_sub_i64") ||
      lv4_is(op, "kind", "dynamic_carrier_end"))) return -1;
  site = yyjson_get_uint(yyjson_obj_get(op, "site"));
  if (lv4_is(op, "kind", "dynamic_invoke")) {
    yyjson_val* arguments = yyjson_obj_get(op, "arguments");
    size_t count = yyjson_arr_size(arguments);
    size_t len = yyjson_get_len(yyjson_obj_get(op, "selector"));
    char receiver[40], argument_array[40];
    snprintf(receiver, sizeof(receiver), "%%dynrecv%u", block);
    if (!lv4_invocation_operand_emit(f, yyjson_obj_get(op, "receiver"), block, 0, receiver)) return 0;
    if (count) snprintf(argument_array, sizeof(argument_array), "%%dynargs%u", block);
    else snprintf(argument_array, sizeof(argument_array), "null");
    for (size_t i = 0; i < count; i++) {
      char destination[64];
      snprintf(destination, sizeof(destination), "%%dynargptr%u_%zu", block, i);
      fprintf(f,
          "  %%dynargptr%u_%zu = getelementptr inbounds [%zu x %%nyrt.dynamic.operand], ptr %%dynargs%u, i64 0, i64 %zu\n",
          block, i, count, block, i);
      if (!lv4_invocation_operand_emit(f, yyjson_arr_get(arguments, i), block, i + 1, destination)) return 0;
    }
    fprintf(f,
        "  %%dynselector%u = getelementptr inbounds [%zu x i8], ptr @dynsel_%zu_%u, i64 0, i64 0\n"
        "  %%dynstatus%u = call i32 @\"nyash.dynamic.checked_invoke_v1\"(ptr %%frame, i64 %llu, ptr %%dynrecv%u, ptr %%dynselector%u, i64 %zu, ptr %s, i64 %zu, ptr %%dynout%u)\n"
        "  switch i32 %%dynstatus%u, label %%invalid [ i32 0, label %%dynnormal%u i32 1, label %%%s ]\ndynnormal%u:\n"
        "  br label %%%s\n",
        block, len, function, block, block, site, block, block, len,
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
