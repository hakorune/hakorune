/* Opaque runtime storage completion. No source cleanup or status repair. */
static void lv4_map_bookkeeping(FILE* f, const char* name, const char* pointer,
    uint32_t value, uint32_t block, unsigned step) {
  fprintf(f,
      "  %%book%u_%u = call i32 @\"nyash.map.%s_v1\"(ptr %%%s%u)\n"
      "  %%book_ok%u_%u = icmp eq i32 %%book%u_%u, 0\n"
      "  br i1 %%book_ok%u_%u, label %%map_step%u_%u, label %%map_invalid\nmap_step%u_%u:\n",
      block, step, name, pointer, value, block, step, block, step,
      block, step, block, step, block, step);
}

static void lv4_map_dispatch(FILE* f, yyjson_val* term, uint32_t block) {
  fprintf(f, "  switch i32 %%status%u, label %%map_invalid [ i32 0, label %%bb%u i32 1, label %%bb%u ]\n",
      block, lv4_u(term, "normal"), lv4_u(term, "fault"));
}

static void lv4_map_emit_operation(FILE* f, const hako_lv4_input* in,
    yyjson_val* term, uint32_t b, size_t fi) {
  yyjson_val* op = yyjson_obj_get(term, "operation");
  unsigned long long site = yyjson_get_uint(yyjson_obj_get(op, "site"));
  int make_map = lv4_is(op, "kind", "map_new");
  int make_key = lv4_is(op, "kind", "map_prepare_key");
  int scalar = lv4_is(op, "kind", "map_install_value");
  int install = scalar || lv4_is(op, "kind", "map_install_indexed");
  int end_map = lv4_is(op, "kind", "map_end");
  if (make_map || make_key || install) {
    const char* storage = make_map ? "map" : make_key ? "key" : "outcome";
    lv4_map_bookkeeping(f, make_map ? "storage_init" : make_key ? "key_init" : "outcome_init",
        storage, b, b, 0);
  }
  fprintf(f, "  %%status%u = call i32 ", b);
  if (make_map) {
    fprintf(f, "@\"nyash.map.checked_new_v1\"(ptr %%frame, i32 %u, i64 %llu, ptr %%map%u)\n",
        in->profile, site, b);
  } else if (make_key) {
    size_t len = yyjson_get_len(yyjson_obj_get(op, "utf8"));
    fprintf(f, "@\"nyash.map.key_prepare_utf8_v1\"(ptr %%frame, i64 %llu, ptr %%key%u, ptr @map_key_%zu_%u, i64 %zu)\n",
        site, b, fi, b, len);
  } else if (scalar) {
    fprintf(f, "@\"nyash.map.checked_install_value_v1\"(ptr %%frame, i32 %u, i64 %llu, ptr %%v%u, ptr %%v%u, i32 %u, i64 %%v%u, ptr %%outcome%u)\n",
        in->profile, site, lv4_u(op, "map"), lv4_u(op, "key"), lv4_u(op, "value_kind"),
        lv4_u(op, "value"), b);
  } else if (install) {
    fprintf(f, "@\"nyash.map.checked_install_indexed_v1\"(ptr %%frame, i32 %u, i64 %llu, ptr %%v%u, ptr %%v%u, i64 %%v%u, i64 %u, ptr %%outcome%u)\n",
        in->profile, site, lv4_u(op, "map"), lv4_u(op, "key"), lv4_u(op, "value"), lv4_type_id(in, op), b);
  } else {
    fprintf(f, "@\"nyash.map.%s_v1\"(ptr %%frame, i64 %llu, ptr %%v%u)\n",
        end_map ? "checked_end" : "outcome_end", site, lv4_u(op, end_map ? "map" : "outcome"));
  }
  /* Never dispose based on guessed state after InvalidContract or unknown status. */
  fprintf(f, "  %%map_status_ok%u = icmp ult i32 %%status%u, 2\n"
      "  br i1 %%map_status_ok%u, label %%map_returned%u, label %%map_invalid\nmap_returned%u:\n",
      b, b, b, b, b);
  if (!make_map && !make_key && !install) {
    lv4_map_bookkeeping(f, end_map ? "storage_dispose" : "outcome_dispose", "v",
        lv4_u(op, end_map ? "map" : "outcome"), b, 1);
    lv4_map_dispatch(f, term, b);
    return;
  }
  if (install) lv4_map_bookkeeping(f, "key_dispose", "v", lv4_u(op, "key"), b, 1);
  fprintf(f, "  %%map_normal%u = icmp eq i32 %%status%u, 0\n"
      "  br i1 %%map_normal%u, label %%bb%u, label %%map_failed%u\nmap_failed%u:\n",
      b, b, b, lv4_u(term, "normal"), b, b);
  lv4_map_bookkeeping(f, make_map ? "storage_dispose" : make_key ? "key_dispose" : "outcome_dispose",
      make_map ? "map" : make_key ? "key" : "outcome", b, b, 2);
  fprintf(f, "  br label %%bb%u\n", lv4_u(term, "fault"));
}

static void lv4_map_alloca(FILE* f, yyjson_val* op, uint32_t b,
    const hako_llvmc_lifecycle_target_session_v2* abi) {
  const char* name = NULL;
  uint32_t size = 0, align = 0;
  if (lv4_is(op, "kind", "map_new")) { name = "map"; size = abi->map_size; align = abi->map_align; }
  else if (lv4_is(op, "kind", "map_prepare_key")) { name = "key"; size = abi->key_size; align = abi->key_align; }
  else if (lv4_is(op, "kind", "map_install_indexed") || lv4_is(op, "kind", "map_install_value")) { name = "outcome"; size = abi->outcome_size; align = abi->outcome_align; }
  if (name) fprintf(f, "  %%%s%u = alloca [%u x i8], align %u\n", name, b, size, align);
}

static void lv4_map_declarations(FILE* f, const hako_lv4_input* in) {
  fprintf(f,
      "declare void @llvm.trap()\n"
      "declare i32 @\"nyash.map.storage_init_v1\"(ptr)\n"
      "declare i32 @\"nyash.map.key_init_v1\"(ptr)\n"
      "declare i32 @\"nyash.map.outcome_init_v1\"(ptr)\n"
      "declare i32 @\"nyash.map.checked_new_v1\"(ptr, i32, i64, ptr)\n"
      "declare i32 @\"nyash.map.key_prepare_utf8_v1\"(ptr, i64, ptr, ptr, i64)\n"
      "declare i32 @\"nyash.map.checked_install_indexed_v1\"(ptr, i32, i64, ptr, ptr, i64, i64, ptr)\n"
      "declare i32 @\"nyash.map.checked_install_value_v1\"(ptr, i32, i64, ptr, ptr, i32, i64, ptr)\n"
      "declare i32 @\"nyash.map.outcome_end_v1\"(ptr, i64, ptr)\n"
      "declare i32 @\"nyash.map.checked_end_v1\"(ptr, i64, ptr)\n"
      "declare i32 @\"nyash.map.storage_dispose_v1\"(ptr)\n"
      "declare i32 @\"nyash.map.key_dispose_v1\"(ptr)\n"
      "declare i32 @\"nyash.map.outcome_dispose_v1\"(ptr)\n");
  for (size_t fi = 0; fi < in->index_count; fi++) {
    hako_lv4_function_index* fn = &in->index[fi];
    for (size_t bi = 0; bi < fn->block_count; bi++) {
      yyjson_val* op = yyjson_obj_get(lv4_term(fn->blocks[bi].block), "operation");
      if (!lv4_is(op, "kind", "map_prepare_key")) continue;
      yyjson_val* text = yyjson_obj_get(op, "utf8");
      const unsigned char* bytes = (const unsigned char*)yyjson_get_str(text);
      size_t len = yyjson_get_len(text);
      fprintf(f, "@map_key_%zu_%u = private constant [%zu x i8] c\"", fi, fn->blocks[bi].id, len);
      for (size_t i = 0; i < len; i++) fprintf(f, "\\%02X", bytes[i]);
      fprintf(f, "\", align 1\n");
    }
  }
}
