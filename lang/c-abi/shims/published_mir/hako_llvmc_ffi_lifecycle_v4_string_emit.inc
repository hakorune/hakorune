/* Physical checked String calls shared by indexed and native V4 cohorts. */
static int lv4_string_operation(yyjson_val* op) {
  return hako_physical_string_kind(yyjson_get_str(yyjson_obj_get(op, "kind")));
}
static void lv4_string_globals(FILE* f, const hako_lv4_input* in) {
  fprintf(f,
      "%%nyrt.string.operand = type { i32, i32, ptr, i64, i64, i64, i64 }\n"
      "%%nyrt.string.owned = type { i64, i64 }\n"
      "declare i32 @\"nyash.string.checked_add_v1\"(ptr, i64, ptr, ptr, ptr)\n"
      "declare i32 @\"nyash.string.checked_print_v1\"(ptr, i64, ptr)\n"
      "declare i32 @\"nyash.string.checked_end_v1\"(ptr, i64, i64, i64)\n"
      "declare i32 @\"nyash.string.checked_compare_v1\"(ptr, i64, i32, ptr, ptr, ptr)\n"
      "declare i32 @\"nyash.string.checked_length_v1\"(ptr, i64, ptr, ptr)\n"
      "declare i32 @\"nyash.string.checked_materialize_v1\"(ptr, i64, ptr, ptr)\n"
      "declare i32 @\"nyash.string.checked_substring_operands_v1\"(ptr, i64, ptr, ptr, ptr, ptr)\n"
      "declare i32 @\"nyash.string.checked_index_of_v1\"(ptr, i64, ptr, ptr, ptr)\n");
  for (size_t fi = 0; fi < yyjson_arr_size(in->functions); fi++) {
    yyjson_val* blocks = hako_physical_blocks(yyjson_arr_get(in->functions, fi));
    for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
      yyjson_val* block = yyjson_arr_get(blocks, bi);
      yyjson_val* op = yyjson_obj_get(lv4_term(block), "operation");
      int call = lv4_is(op, "kind", "ordinary_call");
      if (!call && !hako_physical_string_operand_count(op)) continue;
      yyjson_val* args = yyjson_obj_get(yyjson_obj_get(op, "call"), "args");
      size_t count = call ? yyjson_arr_size(args) : hako_physical_string_operand_count(op);
      for (unsigned role = 0; role < count; role++) {
        yyjson_val* arg = call ? yyjson_arr_get(args, role) : NULL;
        yyjson_val* operand = call ? (lv4_is(arg, "kind", "borrowed_text") ? yyjson_obj_get(arg, "operand") : NULL) :
            hako_physical_string_operand_at(op, role);
        if (!lv4_is(operand, "kind", "static_text")) continue;
        yyjson_val* text = yyjson_obj_get(operand, "utf8");
        size_t len = yyjson_get_len(text);
        const unsigned char* bytes = (const unsigned char*)yyjson_get_str(text);
        fprintf(f, "@str_%zu_%u_%u = private unnamed_addr constant [%zu x i8] c\"", fi, lv4_u(block, "id"), role, len);
        for (size_t i = 0; i < len; i++) fprintf(f, "\\%02X", bytes[i]);
        fprintf(f, "\", align 1\n");
      }
    }
  }
}
static void lv4_string_alloca(FILE* f, yyjson_val* op, uint32_t b) {
  if (lv4_is(op, "kind", "ordinary_call")) {
    if (lv4_is(op, "result", "text"))
      fprintf(f, "  %%strout%u = alloca %%nyrt.string.owned, align 8\n", b);
    yyjson_val* args = yyjson_obj_get(yyjson_obj_get(op, "call"), "args");
    for (size_t i = 0; i < yyjson_arr_size(args); i++)
      if (lv4_is(yyjson_arr_get(args, i), "kind", "borrowed_text"))
        fprintf(f, "  %%strarg%u_%zu = alloca %%nyrt.string.operand, align 8\n", b, i);
    return;
  }
  if (hako_physical_map_field_text_read(op) || hako_physical_array_text_append(op)) {
    fprintf(f, "  %%strarg%u_0 = alloca %%nyrt.string.operand, align 8\n", b);
    return;
  }
  if (!lv4_string_operation(op) || lv4_is(op, "kind", "string_end")) return;
  int result = hako_physical_string_result(yyjson_get_str(yyjson_obj_get(op, "kind")));
  for (unsigned role = 0; role < hako_physical_string_operand_count(op); role++)
    fprintf(f, "  %%strarg%u_%u = alloca %%nyrt.string.operand, align 8\n", b, role);
  if (result) fprintf(f, "  %%strout%u = alloca %s, align 8\n", b, result == 1 ? "%nyrt.string.owned" : "i64");
}
static void lv4_string_operand_emit(FILE* f, yyjson_val* operand,
    size_t fi, uint32_t b, unsigned role) {
  int text = lv4_is(operand, "kind", "static_text");
  int owned = lv4_is(operand, "kind", "owned_string");
  int dynamic = lv4_is(operand, "kind", "tagged_dynamic");
  uint32_t value = lv4_u(operand, "value");
  if (lv4_is(operand, "kind", "borrowed_text")) {
    /* The indexed source domain proves a borrowed formal/Copy root. These
       guards supplement that proof before any descriptor memory is read. */
    fprintf(f,
        "  %%strtag%u_%u = icmp eq i32 %%k%u, 4\n"
        "  %%strnonzero%u_%u = icmp ne i64 %%v%u, 0\n"
        "  %%stralign%u_%u = and i64 %%v%u, 7\n"
        "  %%straligned%u_%u = icmp eq i64 %%stralign%u_%u, 0\n"
        "  %%straddress%u_%u = and i1 %%strnonzero%u_%u, %%straligned%u_%u\n"
        "  %%strvalid%u_%u = and i1 %%strtag%u_%u, %%straddress%u_%u\n"
        "  br i1 %%strvalid%u_%u, label %%strborrow%u_%u, label %%invalid\nstrborrow%u_%u:\n"
        "  %%strptr%u_%u = inttoptr i64 %%v%u to ptr\n"
        "  %%strkind%u_%u = load i32, ptr %%strptr%u_%u, align 8\n"
        "  %%strstatic%u_%u = icmp eq i32 %%strkind%u_%u, 1\n"
        "  %%strowned%u_%u = icmp eq i32 %%strkind%u_%u, 2\n"
        "  %%strtext%u_%u = or i1 %%strstatic%u_%u, %%strowned%u_%u\n"
        "  br i1 %%strtext%u_%u, label %%struse%u_%u, label %%invalid\nstruse%u_%u:\n"
        "  %%strview%u_%u = load %%nyrt.string.operand, ptr %%strptr%u_%u, align 8\n"
        "  store %%nyrt.string.operand %%strview%u_%u, ptr %%strarg%u_%u, align 8\n",
        b,role,value, b,role,value, b,role,value, b,role,b,role,
        b,role,b,role,b,role, b,role,b,role,b,role,
        b,role,b,role,b,role, b,role,value, b,role,b,role,
        b,role,b,role, b,role,b,role, b,role,b,role,b,role,
        b,role,b,role,b,role, b,role,b,role, b,role,b,role);
    return;
  }
  if (dynamic) {
    fprintf(f,
        "  %%str%u_%u_0 = insertvalue %%nyrt.string.operand zeroinitializer, i32 4, 0\n"
        "  %%str%u_%u_1 = insertvalue %%nyrt.string.operand %%str%u_%u_0, i32 %%k%u, 1\n"
        "  %%str%u_%u_2 = insertvalue %%nyrt.string.operand %%str%u_%u_1, i64 %%v%u, 6\n"
        "  store %%nyrt.string.operand %%str%u_%u_2, ptr %%strarg%u_%u, align 8\n",
        b,role,b,role,b,role,value,b,role,b,role,value,b,role,b,role,b,role);
    return;
  }
  fprintf(f, "  %%str%u_%u_0 = insertvalue %%nyrt.string.operand zeroinitializer, i32 %d, 0\n",
      b, role, text ? 1 : owned ? 2 : 3);
  if (text) {
    size_t len = yyjson_get_len(yyjson_obj_get(operand, "utf8"));
    fprintf(f,
        "  %%str%u_%u_1 = insertvalue %%nyrt.string.operand %%str%u_%u_0, ptr @str_%zu_%u_%u, 2\n"
        "  %%str%u_%u_2 = insertvalue %%nyrt.string.operand %%str%u_%u_1, i64 %zu, 3\n",
        b,role,b,role,fi,b,role,b,role,b,role,len);
  } else if (owned) {
    fprintf(f,
        "  %%str%u_%u_1 = insertvalue %%nyrt.string.operand %%str%u_%u_0, i64 %%v%u, 4\n"
        "  %%str%u_%u_2 = insertvalue %%nyrt.string.operand %%str%u_%u_1, i64 %%t%u, 5\n",
        b,role,b,role,value,b,role,b,role,value);
  } else {
    fprintf(f,
        "  %%str%u_%u_2 = insertvalue %%nyrt.string.operand %%str%u_%u_0, i64 %%v%u, 6\n",
        b,role,b,role,value);
  }
  fprintf(f, "  store %%nyrt.string.operand %%str%u_%u_2, ptr %%strarg%u_%u, align 8\n", b,role,b,role);
}
static void lv4_string_emit(FILE* f, yyjson_val* op, size_t fi, uint32_t b,
    const char* normal, const char* fault) {
  unsigned long long site = yyjson_get_uint(yyjson_obj_get(op, "site"));
  if (lv4_is(op, "kind", "string_end")) {
    uint32_t value = lv4_u(op, "value");
    fprintf(f, "  %%status%u = call i32 @\"nyash.string.checked_end_v1\"(ptr %%frame, i64 %llu, i64 %%v%u, i64 %%t%u)\n", b,site,value,value);
  } else {
    const char* kind = yyjson_get_str(yyjson_obj_get(op, "kind"));
    size_t count = hako_physical_string_operand_count(op);
    for (unsigned role = 0; role < count; role++)
      lv4_string_operand_emit(f, hako_physical_string_operand_at(op, role), fi,b,role);
    const char* symbol = lv4_is(op, "kind", "string_substring") ?
        "nyash.string.checked_substring_operands_v1" : NULL;
    if (symbol) fprintf(f, "  %%status%u = call i32 @\"%s\"(ptr %%frame, i64 %llu", b,symbol,site);
    else fprintf(f, "  %%status%u = call i32 @\"nyash.string.checked_%s_v1\"(ptr %%frame, i64 %llu", b,kind + 7,site);
    if (lv4_is(op, "kind", "string_compare")) fprintf(f, ", i32 %d", hako_physical_string_comparison(op));
    fprintf(f, ", ptr %%strarg%u_0", b);
    if (count == 2) fprintf(f, ", ptr %%strarg%u_1", b);
    if (count == 3) fprintf(f, ", ptr %%strarg%u_1, ptr %%strarg%u_2", b,b);
    if (hako_physical_string_result(kind)) fprintf(f, ", ptr %%strout%u", b);
    fprintf(f, ")\n");
  }
  fprintf(f, "  switch i32 %%status%u, label %%invalid [ i32 0, label %%%s i32 1, label %%%s ]\n",b,normal,fault);
}
