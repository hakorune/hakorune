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
      "declare i32 @\"nyash.string.checked_end_v1\"(ptr, i64, i64, i64)\n");
  for (size_t fi = 0; fi < yyjson_arr_size(in->functions); fi++) {
    yyjson_val* blocks = hako_physical_blocks(yyjson_arr_get(in->functions, fi));
    for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
      yyjson_val* block = yyjson_arr_get(blocks, bi);
      yyjson_val* op = yyjson_obj_get(lv4_term(block), "operation");
      if (!lv4_string_operation(op) || lv4_is(op, "kind", "string_end")) continue;
      int add = lv4_is(op, "kind", "string_add");
      for (unsigned role = 0; role < (add ? 2u : 1u); role++) {
        yyjson_val* operand = yyjson_obj_get(op, add ? (role ? "right" : "left") : "value");
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
  if (!lv4_string_operation(op) || lv4_is(op, "kind", "string_end")) return;
  int add = lv4_is(op, "kind", "string_add");
  for (unsigned role = 0; role < (add ? 2u : 1u); role++)
    fprintf(f, "  %%strarg%u_%u = alloca %%nyrt.string.operand, align 8\n", b, role);
  if (add) fprintf(f, "  %%strout%u = alloca %%nyrt.string.owned, align 8\n", b);
}
static void lv4_string_operand_emit(FILE* f, yyjson_val* operand,
    size_t fi, uint32_t b, unsigned role) {
  int text = lv4_is(operand, "kind", "static_text");
  int owned = lv4_is(operand, "kind", "owned_string");
  uint32_t value = lv4_u(operand, "value");
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
    int add = lv4_is(op, "kind", "string_add");
    for (unsigned role = 0; role < (add ? 2u : 1u); role++)
      lv4_string_operand_emit(f, yyjson_obj_get(op, add ? (role ? "right" : "left") : "value"), fi,b,role);
    fprintf(f, "  %%status%u = call i32 @\"nyash.string.checked_%s_v1\"(ptr %%frame, i64 %llu, ptr %%strarg%u_0", b,add?"add":"print",site,b);
    if (add) fprintf(f, ", ptr %%strarg%u_1, ptr %%strout%u",b,b);
    fprintf(f, ")\n");
  }
  fprintf(f, "  switch i32 %%status%u, label %%invalid [ i32 0, label %%%s i32 1, label %%%s ]\n",b,normal,fault);
}
