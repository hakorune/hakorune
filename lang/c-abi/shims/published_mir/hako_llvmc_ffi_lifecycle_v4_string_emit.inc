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
      yyjson_val* rows = yyjson_obj_get(block, "instructions");
      for (size_t ri = 0; ri < yyjson_arr_size(rows); ri++) {
        yyjson_val* ins = yyjson_obj_get(yyjson_arr_get(rows, ri), "instruction");
        if (!lv4_is(ins, "op", "const_string")) continue;
        yyjson_val* text = yyjson_obj_get(ins, "value");
        size_t len = yyjson_get_len(text);
        const unsigned char* bytes = (const unsigned char*)yyjson_get_str(text);
        fprintf(f, "@ctext_%zu_%u = private unnamed_addr constant [%zu x i8] c\"", fi, lv4_u(ins, "dst"), len);
        for (size_t i = 0; i < len; i++) fprintf(f, "\\%02X", bytes[i]);
        fprintf(f, "\", align 1\n");
      }
      yyjson_val* op = yyjson_obj_get(lv4_term(block), "operation");
      int call = lv4_is(op, "kind", "ordinary_call") || lv4_is(op, "kind", "birth_call");
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
/* Immutable SSA descriptors are emitted only from original constructors. */
static void lv4_string_constant_emit(FILE* f, yyjson_val* ins, size_t fi) {
  uint32_t dst = lv4_u(ins, "dst");
  size_t len = yyjson_get_len(yyjson_obj_get(ins, "value"));
  fprintf(f, "  %%ctext%u_0 = insertvalue %%nyrt.string.operand zeroinitializer, i32 1, 0\n", dst);
  fprintf(f, "  %%ctext%u_1 = insertvalue %%nyrt.string.operand %%ctext%u_0, ptr @ctext_%zu_%u, 2\n", dst,dst,fi,dst);
  fprintf(f, "  %%text%u = insertvalue %%nyrt.string.operand %%ctext%u_1, i64 %zu, 3\n", dst,dst,len);
}
static void lv4_string_result_emit(FILE* f, uint32_t dst, uint32_t producer_id) {
  fprintf(f,
      "  %%v%u = load i64, ptr %%strout%u, align 8\n"
      "  %%tokenptr%u = getelementptr inbounds %%nyrt.string.owned, ptr %%strout%u, i32 0, i32 1\n"
      "  %%t%u = load i64, ptr %%tokenptr%u, align 8\n",
      dst, producer_id, dst, producer_id, dst, dst);
  fprintf(f, "  %%ctext%u_0 = insertvalue %%nyrt.string.operand zeroinitializer, i32 2, 0\n", dst);
  fprintf(f, "  %%ctext%u_1 = insertvalue %%nyrt.string.operand %%ctext%u_0, i64 %%v%u, 4\n", dst,dst,dst);
  fprintf(f, "  %%text%u = insertvalue %%nyrt.string.operand %%ctext%u_1, i64 %%t%u, 5\n", dst,dst,dst);
}

static void lv4_string_alloca(FILE* f, yyjson_val* op, uint32_t b) {
  if (lv4_is(op, "kind", "ordinary_call") || lv4_is(op, "kind", "birth_call")) {
    if (lv4_is(op, "result", "text") || lv4_is(op, "result", "nullable_text"))
      fprintf(f, "  %%strout%u = alloca %%nyrt.string.owned, align 8\n", b);
    yyjson_val* args = yyjson_obj_get(yyjson_obj_get(op, "call"), "args");
    for (size_t i = 0; i < yyjson_arr_size(args); i++)
      if (lv4_is(yyjson_arr_get(args, i), "kind", "borrowed_text"))
        fprintf(f, "  %%strarg%u_%zu = alloca %%nyrt.string.operand, align 8\n", b, i);
    return;
  }
  if (hako_physical_map_field_text_access(op) || hako_physical_array_text_append(op)) {
    fprintf(f, "  %%strarg%u_0 = alloca %%nyrt.string.operand, align 8\n", b);
    return;
  }
  if (!lv4_string_operation(op) || lv4_is(op, "kind", "string_end") || lv4_is(op, "kind", "string_end_if_owned")) return;
  if (lv4_is(op, "kind", "string_field_materialize")) {
    fprintf(f, "  %%strfield%u = alloca { i32, i32, i64, i64 }, align 8\n", b);
    fprintf(f, "  %%strarg%u_0 = alloca %%nyrt.string.operand, align 8\n", b);
  }
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
  int carrier = lv4_is(operand, "kind", "dynamic_carrier");
  uint32_t value = lv4_u(operand, "value");
  if (lv4_is(operand, "kind", "text_operand")) {
    fprintf(f, "  store %%nyrt.string.operand %%text%u, ptr %%strarg%u_%u, align 8\n", value,b,role);
    return;
  }
  if (lv4_is(operand, "kind", "normal_bool")) {
    /* Bool displays through the tagged Dynamic spelling: tag 2, payload 0/1. */
    fprintf(f,
        "  %%str%u_%u_0 = insertvalue %%nyrt.string.operand zeroinitializer, i32 4, 0\n"
        "  %%str%u_%u_1 = insertvalue %%nyrt.string.operand %%str%u_%u_0, i32 2, 1\n"
        "  %%str%u_%u_2 = insertvalue %%nyrt.string.operand %%str%u_%u_1, i64 %%v%u, 6\n"
        "  store %%nyrt.string.operand %%str%u_%u_2, ptr %%strarg%u_%u, align 8\n",
        b,role,b,role,b,role,b,role,b,role,value,b,role,b,role);
    return;
  }
  if (carrier) {
    fprintf(f,
        "  %%str%u_%u_0 = insertvalue %%nyrt.string.operand zeroinitializer, i32 5, 0\n"
        "  %%str%u_%u_1 = insertvalue %%nyrt.string.operand %%str%u_%u_0, i64 %%v%u, 5\n"
        "  store %%nyrt.string.operand %%str%u_%u_1, ptr %%strarg%u_%u, align 8\n",
        b,role,b,role,b,role,value,b,role,b,role);
    return;
  }
  if (dynamic) {
    fprintf(f, "  %%str_dynamic_text%u_%u = icmp eq i32 %%k%u, 4\n", b,role,value);
    fprintf(f, "  br i1 %%str_dynamic_text%u_%u, label %%str_dynamic_borrow%u_%u, label %%str_dynamic_scalar%u_%u\n",
        b,role,b,role,b,role);
    fprintf(f, "str_dynamic_scalar%u_%u:\n", b,role);
    fprintf(f, "  %%str%u_%u_0 = insertvalue %%nyrt.string.operand zeroinitializer, i32 4, 0\n", b,role);
    fprintf(f, "  %%str%u_%u_1 = insertvalue %%nyrt.string.operand %%str%u_%u_0, i32 %%k%u, 1\n", b,role,b,role,value);
    fprintf(f, "  %%str%u_%u_2 = insertvalue %%nyrt.string.operand %%str%u_%u_1, i64 %%v%u, 6\n", b,role,b,role,value);
    fprintf(f, "  store %%nyrt.string.operand %%str%u_%u_2, ptr %%strarg%u_%u, align 8\n", b,role,b,role);
    fprintf(f, "  br label %%str_dynamic_ready%u_%u\nstr_dynamic_borrow%u_%u:\n", b,role,b,role);
  }
  if (dynamic || lv4_is(operand, "kind", "borrowed_text")) {
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
    if (dynamic) fprintf(f, "  br label %%str_dynamic_ready%u_%u\nstr_dynamic_ready%u_%u:\n", b,role,b,role);
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
static void lv4_string_emit(FILE* f, const hako_lv4_input* in, yyjson_val* op, size_t fi, uint32_t b,
    const char* normal, const char* fault) {
  unsigned long long site = yyjson_get_uint(yyjson_obj_get(op, "site"));
  if (lv4_is(op, "kind", "string_field_materialize")) {
    /* Dynamic kind4 is a borrowed field pair. Only checked materialize
       creates the independent owned result; neither read nor Fault consumes it. */
    fprintf(f, "  %%strfield_status%u = call i32 @\"nyash.object.checked_dynamic_get_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 %%v%u, i64 %u, i64 %d, i32 4, ptr %%strfield%u)\n",
        b, in->profile, site, lv4_u(op, "base"), lv4_type_id(in, op), lv4_slot(in, op), b);
    fprintf(f, "  switch i32 %%strfield_status%u, label %%invalid [ i32 0, label %%strfield_normal%u i32 1, label %%%s ]\nstrfield_normal%u:\n", b,b,fault,b);
    fprintf(f, "  %%strfield_payload%u = getelementptr { i32, i32, i64, i64 }, ptr %%strfield%u, i32 0, i32 2\n", b,b);
    fprintf(f, "  %%strfield_handle%u = load i64, ptr %%strfield_payload%u, align 8\n", b,b);
    fprintf(f, "  %%strfield_tokenptr%u = getelementptr { i32, i32, i64, i64 }, ptr %%strfield%u, i32 0, i32 3\n", b,b);
    fprintf(f, "  %%strfield_token%u = load i64, ptr %%strfield_tokenptr%u, align 8\n", b,b);
    fprintf(f, "  %%strfield_operand%u_0 = insertvalue %%nyrt.string.operand zeroinitializer, i32 2, 0\n", b);
    fprintf(f, "  %%strfield_operand%u_1 = insertvalue %%nyrt.string.operand %%strfield_operand%u_0, i64 %%strfield_handle%u, 4\n", b,b,b);
    fprintf(f, "  %%strfield_operand%u_2 = insertvalue %%nyrt.string.operand %%strfield_operand%u_1, i64 %%strfield_token%u, 5\n", b,b,b);
    fprintf(f, "  store %%nyrt.string.operand %%strfield_operand%u_2, ptr %%strarg%u_0, align 8\n", b,b);
    fprintf(f, "  %%status%u = call i32 @\"nyash.string.checked_materialize_v1\"(ptr %%frame, i64 %llu, ptr %%strarg%u_0, ptr %%strout%u)\n", b,site,b,b);
  } else if (lv4_is(op, "kind", "string_end_if_owned")) {
    uint32_t value = lv4_u(op, "value");
    fprintf(f, "  %%end_kind%u = extractvalue %%nyrt.string.operand %%text%u, 0\n", b,value);
    fprintf(f, "  switch i32 %%end_kind%u, label %%invalid [ i32 1, label %%end_static%u i32 2, label %%end_owned%u ]\n", b,b,b);
    fprintf(f, "end_static%u:\n  br label %%end_join%u\nend_owned%u:\n", b,b,b);
    fprintf(f, "  %%end_handle%u = extractvalue %%nyrt.string.operand %%text%u, 4\n", b,value);
    fprintf(f, "  %%end_token%u = extractvalue %%nyrt.string.operand %%text%u, 5\n", b,value);
    fprintf(f, "  %%end_status%u = call i32 @\"nyash.string.checked_end_v1\"(ptr %%frame, i64 %llu, i64 %%end_handle%u, i64 %%end_token%u)\n", b,site,b,b);
    fprintf(f, "  br label %%end_join%u\nend_join%u:\n", b,b);
    fprintf(f, "  %%status%u = phi i32 [ 0, %%end_static%u ], [ %%end_status%u, %%end_owned%u ]\n", b,b,b,b);
  } else if (lv4_is(op, "kind", "string_end")) {
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
