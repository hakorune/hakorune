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

static void lv4_map_dispatch(FILE* f, uint32_t block,
    const char* normal, const char* fault) {
  fprintf(f, "  switch i32 %%status%u, label %%map_invalid [ i32 0, label %%%s i32 1, label %%%s ]\n",
      block, normal, fault);
}

static void lv4_map_emit_operation(FILE* f, const hako_lv4_input* in,
    yyjson_val* term, uint32_t b, size_t fi, const char* normal, const char* fault) {
  yyjson_val* op = yyjson_obj_get(term, "operation");
  unsigned long long site = yyjson_get_uint(yyjson_obj_get(op, "site"));
  int make_map = lv4_is(op, "kind", "map_new");
  int make_key = lv4_is(op, "kind", "map_prepare_key");
  int scalar = lv4_is(op, "kind", "map_install_value");
  int text = lv4_is(op, "kind", "map_install_text");
  int empty = lv4_is(op, "kind", "map_install_empty_array");
  int install = scalar || text || empty || lv4_is(op, "kind", "map_install_indexed");
  int get = lv4_is(op, "kind", "map_checked_get");
  int array_index = lv4_is(op, "kind", "map_array_index_map");
  int get_text = lv4_is(op, "kind", "map_get_text");
  int end_map = lv4_is(op, "kind", "map_end");
  if (make_map || make_key || install) {
    const char* storage = make_map ? "map" : make_key ? "key" : "outcome";
    lv4_map_bookkeeping(f, make_map ? "storage_init" : make_key ? "key_init" : "outcome_init",
        storage, b, b, 0);
  }
  fprintf(f, "  %%status%u = call i32 ", b);
  if (get) {
    size_t len = yyjson_get_len(yyjson_obj_get(op, "utf8"));
    /* Inline sealed key; kernel writes the i64 only on Normal. */
    fprintf(f, "@\"nyash.map.checked_get_i64_v1\"(ptr %%frame, i64 %llu, ptr %%v%u, ptr @map_get_%zu_%u, i64 %zu, ptr %%get_out%u)\n",
        site, lv4_u(op, "map"), fi, b, len, b);
  } else if (array_index) {
    size_t len = yyjson_get_len(yyjson_obj_get(op, "utf8"));
    long long index = yyjson_is_sint(yyjson_obj_get(op, "index"))
        ? (long long)yyjson_get_sint(yyjson_obj_get(op, "index"))
        : (long long)yyjson_get_uint(yyjson_obj_get(op, "index"));
    fprintf(f, "@\"nyash.map.checked_array_index_map_v1\"(ptr %%frame, i64 %llu, ptr %%v%u, ptr @map_array_key_%zu_%u, i64 %zu, i64 %lld, ptr %%view%u)\n",
        site, lv4_u(op, "map"), fi, b, len, index, b);
  } else if (get_text) {
    size_t len = yyjson_get_len(yyjson_obj_get(op, "utf8"));
    fprintf(f, "@\"nyash.map.checked_get_text_v1\"(ptr %%frame, i64 %llu, ptr %%v%u, ptr @map_text_get_%zu_%u, i64 %zu, ptr %%text%u)\n",
        site, lv4_u(op, "map"), fi, b, len, b);
  } else if (make_map) {
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
  } else if (text) {
    size_t len = yyjson_get_len(yyjson_obj_get(op, "utf8"));
    fprintf(f, "@\"nyash.map.checked_install_text_v1\"(ptr %%frame, i32 %u, i64 %llu, ptr %%v%u, ptr %%v%u, ptr @map_text_%zu_%u, i64 %zu, ptr %%outcome%u)\n",
        in->profile, site, lv4_u(op, "map"), lv4_u(op, "key"), fi, b, len, b);
  } else if (empty) {
    fprintf(f, "@\"nyash.map.checked_install_empty_array_v1\"(ptr %%frame, i32 %u, i64 %llu, ptr %%v%u, ptr %%v%u, ptr %%outcome%u)\n",
        in->profile, site, lv4_u(op, "map"), lv4_u(op, "key"), b);
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
  if (get || array_index || get_text) {
    /* No storage was initialized or consumed: dispatch on status only. */
    lv4_map_dispatch(f, b, normal, fault);
    return;
  }
  if (!make_map && !make_key && !install) {
    lv4_map_bookkeeping(f, end_map ? "storage_dispose" : "outcome_dispose", "v",
        lv4_u(op, end_map ? "map" : "outcome"), b, 1);
    lv4_map_dispatch(f, b, normal, fault);
    return;
  }
  if (install) lv4_map_bookkeeping(f, "key_dispose", "v", lv4_u(op, "key"), b, 1);
  fprintf(f, "  %%map_normal%u = icmp eq i32 %%status%u, 0\n"
      "  br i1 %%map_normal%u, label %%%s, label %%map_failed%u\nmap_failed%u:\n",
      b, b, b, normal, b, b);
  lv4_map_bookkeeping(f, make_map ? "storage_dispose" : make_key ? "key_dispose" : "outcome_dispose",
      make_map ? "map" : make_key ? "key" : "outcome", b, b, 2);
  fprintf(f, "  br label %%%s\n", fault);
}

static void lv4_map_alloca(FILE* f, yyjson_val* op, uint32_t b,
    const hako_llvmc_lifecycle_target_session_v2* abi) {
  const char* name = NULL;
  uint32_t size = 0, align = 0;
  if (lv4_is(op, "kind", "map_new") ||
      (lv4_is(op, "kind", "ordinary_call") && lv4_is(op, "result", "map"))) {
    name = "map"; size = abi->map_size; align = abi->map_align;
  }
  else if (lv4_is(op, "kind", "map_prepare_key")) { name = "key"; size = abi->key_size; align = abi->key_align; }
  else if (lv4_is(op, "kind", "map_array_index_map")) {
    name = "view"; size = (uint32_t)sizeof(NyrtMapViewStorageV1); align = (uint32_t)_Alignof(NyrtMapViewStorageV1);
  }
  else if (lv4_is(op, "kind", "map_get_text")) {
    name = "text"; size = (uint32_t)sizeof(NyrtTextViewStorageV1); align = (uint32_t)_Alignof(NyrtTextViewStorageV1);
  }
  else if (lv4_is(op, "kind", "map_install_indexed") || lv4_is(op, "kind", "map_install_value") ||
      lv4_is(op, "kind", "map_install_text") || lv4_is(op, "kind", "map_install_empty_array")) { name = "outcome"; size = abi->outcome_size; align = abi->outcome_align; }
  if (name) fprintf(f, "  %%%s%u = alloca [%u x i8], align %u\n", name, b, size, align);
  if (lv4_is(op, "kind", "map_checked_get")) {
    /* The kernel writes the slot only on Normal; a Fault read leaves the
     * sentinel so a stray consumer observes -1, never uninitialized stack. */
    fprintf(f, "  %%get_out%u = alloca i64, align 8\n", b);
    fprintf(f, "  store i64 -1, ptr %%get_out%u, align 8\n", b);
  }
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
      "declare i32 @\"nyash.map.checked_install_text_v1\"(ptr, i32, i64, ptr, ptr, ptr, i64, ptr)\n"
      "declare i32 @\"nyash.map.checked_install_empty_array_v1\"(ptr, i32, i64, ptr, ptr, ptr)\n"
      "declare i32 @\"nyash.map.outcome_end_v1\"(ptr, i64, ptr)\n"
      "declare i32 @\"nyash.map.checked_end_v1\"(ptr, i64, ptr)\n"
      "declare i32 @\"nyash.map.checked_get_i64_v1\"(ptr, i64, ptr, ptr, i64, ptr)\n"
      "declare i32 @\"nyash.map.checked_array_index_map_v1\"(ptr, i64, ptr, ptr, i64, i64, ptr)\n"
      "declare i32 @\"nyash.map.checked_get_text_v1\"(ptr, i64, ptr, ptr, i64, ptr)\n"
      "declare i32 @\"nyash.map.storage_dispose_v1\"(ptr)\n"
      "declare i32 @\"nyash.map.storage_move_v1\"(ptr, ptr)\n"
      "declare i32 @\"nyash.map.key_dispose_v1\"(ptr)\n"
      "declare i32 @\"nyash.map.outcome_dispose_v1\"(ptr)\n");
  for (size_t fi = 0; fi < in->index_count; fi++) {
    hako_lv4_function_index* fn = &in->index[fi];
    for (size_t bi = 0; bi < fn->block_count; bi++) {
      yyjson_val* op = yyjson_obj_get(lv4_term(fn->blocks[bi].block), "operation");
      int key = lv4_is(op, "kind", "map_prepare_key");
      int text_install = lv4_is(op, "kind", "map_install_text");
      int get = lv4_is(op, "kind", "map_checked_get");
      int array_index = lv4_is(op, "kind", "map_array_index_map");
      int text_get = lv4_is(op, "kind", "map_get_text");
      if (!key && !text_install && !get && !array_index && !text_get) continue;
      yyjson_val* text = yyjson_obj_get(op, "utf8");
      const unsigned char* bytes = (const unsigned char*)yyjson_get_str(text);
      size_t len = yyjson_get_len(text);
      fprintf(f, "@%s_%zu_%u = private constant [%zu x i8] c\"",
          key ? "map_key" : get ? "map_get" : array_index ? "map_array_key" :
          text_get ? "map_text_get" : "map_text", fi, fn->blocks[bi].id, len);
      for (size_t i = 0; i < len; i++) fprintf(f, "\\%02X", bytes[i]);
      fprintf(f, "\", align 1\n");
    }
  }
}
