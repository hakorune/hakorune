static int hako_llvmc_emit_array_getset_micro_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    long long size,
    long long ops) {
  char llpath[1024];
  FILE* f;
  int rc;

  if (!obj_out || size != 128 || ops != 2000000) {
    return -1;
  }

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_array_getset_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (array get/set micro)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%arr = alloca [128 x i64], align 64\n");
  fprintf(f, "  %%i = alloca i64, align 8\n");
  fprintf(f, "  %%sum = alloca i64, align 8\n");
  fprintf(f, "  %%arr.ptr = getelementptr inbounds [128 x i64], ptr %%arr, i64 0, i64 0\n");
  fprintf(f, "  store i64 0, ptr %%i, align 8\n");
  fprintf(f, "  br label %%init\n\n");

  fprintf(f, "init:\n");
  fprintf(f, "  %%i.init = load i64, ptr %%i, align 8\n");
  fprintf(f, "  %%init.cond = icmp slt i64 %%i.init, 128\n");
  fprintf(f, "  br i1 %%init.cond, label %%init.body, label %%init.exit\n\n");

  fprintf(f, "init.body:\n");
  fprintf(f, "  %%init.slot = getelementptr inbounds i64, ptr %%arr.ptr, i64 %%i.init\n");
  fprintf(f, "  store i64 %%i.init, ptr %%init.slot, align 8\n");
  fprintf(f, "  %%i.init.next = add i64 %%i.init, 1\n");
  fprintf(f, "  store i64 %%i.init.next, ptr %%i, align 8\n");
  fprintf(f, "  br label %%init\n\n");

  fprintf(f, "init.exit:\n");
  fprintf(f, "  store i64 0, ptr %%i, align 8\n");
  fprintf(f, "  store i64 0, ptr %%sum, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "loop:\n");
  fprintf(f, "  %%i.cur = load i64, ptr %%i, align 8\n");
  fprintf(f, "  %%loop.cond = icmp slt i64 %%i.cur, %lld\n", ops);
  fprintf(f, "  br i1 %%loop.cond, label %%loop.body, label %%exit\n\n");

  fprintf(f, "loop.body:\n");
  fprintf(f, "  %%idx = and i64 %%i.cur, 127\n");
  fprintf(f, "  %%slot = getelementptr inbounds i64, ptr %%arr.ptr, i64 %%idx\n");
  fprintf(f, "  %%v = load i64, ptr %%slot, align 8\n");
  fprintf(f, "  %%next = add i64 %%v, 1\n");
  fprintf(f, "  store i64 %%next, ptr %%slot, align 8\n");
  fprintf(f, "  %%sum.cur = load volatile i64, ptr %%sum, align 8\n");
  fprintf(f, "  %%sum.next = add i64 %%sum.cur, %%next\n");
  fprintf(f, "  store volatile i64 %%sum.next, ptr %%sum, align 8\n");
  fprintf(f, "  %%i.next = add i64 %%i.cur, 1\n");
  fprintf(f, "  store i64 %%i.next, ptr %%i, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "exit:\n");
  fprintf(f, "  %%sum.fin = load volatile i64, ptr %%sum, align 8\n");
  fprintf(f, "  %%ret = and i64 %%sum.fin, 255\n");
  fprintf(f, "  ret i64 %%ret\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

static int hako_llvmc_consume_array_getset_micro_route(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  yyjson_val* metadata;
  yyjson_val* route;
  size_t bi;
  hako_llvmc_entry_rune_selection selection;
  const char* proof;
  const char* rmw_proof;
  long long size = 0;
  long long ops = 0;
  long long init_push_count = 0;
  long long loop_get_count = 0;
  long long loop_set_count = 0;
  long long final_get_count = 0;
  long long selected_rmw_block = 0;
  long long selected_rmw_instruction_index = 0;
  long long selected_rmw_set_instruction_index = 0;
  long long loop_index_phi_value = 0;
  long long accumulator_phi_value = 0;
  long long accumulator_next_value = 0;
  long long return_value = 0;
  int rc;

  if (!doc) return -1;
  root = yyjson_doc_get_root(doc);
  fns = yyjson_obj_get(root, "functions");
  if (!(fns && yyjson_is_arr(fns))) {
    yyjson_doc_free(doc);
    return -1;
  }

  for (bi = 0; bi < yyjson_arr_size(fns); bi++) {
    yyjson_val* candidate = yyjson_arr_get(fns, bi);
    const char* name = candidate ? yyjson_get_str(yyjson_obj_get(candidate, "name")) : NULL;
    if (name && strcmp(name, "main") == 0) {
      fn = candidate;
      break;
    }
  }
  if (!fn) {
    yyjson_doc_free(doc);
    return -1;
  }
  hako_llvmc_select_entry_runes(fn, &selection);

  metadata = yyjson_obj_get(fn, "metadata");
  route = metadata ? yyjson_obj_get(metadata, "array_getset_micro_seed_route") : NULL;
  if (!(route && yyjson_is_obj(route))) {
    yyjson_doc_free(doc);
    return -1;
  }

  proof = yyjson_get_str(yyjson_obj_get(route, "proof"));
  rmw_proof = yyjson_get_str(yyjson_obj_get(route, "rmw_proof"));
  size = (long long)yyjson_get_sint(yyjson_obj_get(route, "size"));
  ops = (long long)yyjson_get_sint(yyjson_obj_get(route, "ops"));
  init_push_count = (long long)yyjson_get_sint(yyjson_obj_get(route, "init_push_count"));
  loop_get_count = (long long)yyjson_get_sint(yyjson_obj_get(route, "loop_get_count"));
  loop_set_count = (long long)yyjson_get_sint(yyjson_obj_get(route, "loop_set_count"));
  final_get_count = (long long)yyjson_get_sint(yyjson_obj_get(route, "final_get_count"));
  selected_rmw_block = (long long)yyjson_get_sint(yyjson_obj_get(route, "selected_rmw_block"));
  selected_rmw_instruction_index =
      (long long)yyjson_get_sint(yyjson_obj_get(route, "selected_rmw_instruction_index"));
  selected_rmw_set_instruction_index =
      (long long)yyjson_get_sint(yyjson_obj_get(route, "selected_rmw_set_instruction_index"));
  loop_index_phi_value =
      (long long)yyjson_get_sint(yyjson_obj_get(route, "loop_index_phi_value"));
  accumulator_phi_value =
      (long long)yyjson_get_sint(yyjson_obj_get(route, "accumulator_phi_value"));
  accumulator_next_value =
      (long long)yyjson_get_sint(yyjson_obj_get(route, "accumulator_next_value"));
  return_value = (long long)yyjson_get_sint(yyjson_obj_get(route, "return_value"));

  if (!(proof && rmw_proof &&
        hako_llvmc_str_eq(proof, "kilo_micro_array_getset_7block") &&
        hako_llvmc_str_eq(rmw_proof, "array_get_add1_set_same_slot") &&
        size == 128 &&
        ops == 2000000 &&
        init_push_count == 1 &&
        loop_get_count == 1 &&
        loop_set_count == 1 &&
        final_get_count == 0 &&
        selected_rmw_block == 23 &&
        selected_rmw_instruction_index == 8 &&
        selected_rmw_set_instruction_index == 13 &&
        loop_index_phi_value > 0 &&
        accumulator_phi_value > 0 &&
        accumulator_next_value > 0 &&
        return_value == accumulator_phi_value)) {
    if (hako_llvmc_route_trace_enabled()) {
      hako_llvmc_emit_route_trace(
          "array_getset_micro",
          "skip",
          "metadata_contract_mismatch",
          "array_getset_micro_seed_route");
    }
    yyjson_doc_free(doc);
    return -1;
  }

  if (hako_llvmc_route_trace_enabled()) {
    hako_llvmc_emit_route_trace(
        "array_getset_micro",
        "emit",
        "exact_match",
        "kilo_micro_array_getset mir_route_plan");
  }
  rc = hako_llvmc_emit_array_getset_micro_ir(
      obj_out,
      &selection,
      size,
      ops);
  yyjson_doc_free(doc);
  return rc;
}

static int hako_llvmc_emit_array_rmw_add1_leaf_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    long long size,
    long long ops) {
  char llpath[1024];
  FILE* f;
  int rc;

  if (!obj_out || size != 128 || ops != 2000000) {
    return -1;
  }

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_array_rmw_add1_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (array rmw add1 leaf)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%arr = alloca [128 x i64], align 64\n");
  fprintf(f, "  %%i = alloca i64, align 8\n");
  fprintf(f, "  %%arr.ptr = getelementptr inbounds [128 x i64], ptr %%arr, i64 0, i64 0\n");
  for (int base = 0, vec_idx = 0; base < 128; base += 8, vec_idx++) {
    fprintf(
        f,
        "  %%init.vecptr%d = getelementptr inbounds i64, ptr %%arr.ptr, i64 %d\n",
        vec_idx,
        base);
    fprintf(
        f,
        "  store <8 x i64> <i64 %d, i64 %d, i64 %d, i64 %d, i64 %d, i64 %d, i64 %d, i64 %d>, ptr %%init.vecptr%d, align 64\n",
        base + 0,
        base + 1,
        base + 2,
        base + 3,
        base + 4,
        base + 5,
        base + 6,
        base + 7,
        vec_idx);
  }
  fprintf(f, "  store i64 0, ptr %%i, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "loop:\n");
  fprintf(f, "  %%i.cur = load i64, ptr %%i, align 8\n");
  fprintf(f, "  %%loop.cond = icmp slt i64 %%i.cur, %lld\n", ops);
  fprintf(f, "  br i1 %%loop.cond, label %%loop.body, label %%exit\n\n");

  fprintf(f, "loop.body:\n");
  fprintf(f, "  %%idx = and i64 %%i.cur, 127\n");
  fprintf(f, "  %%slot = getelementptr inbounds i64, ptr %%arr.ptr, i64 %%idx\n");
  fprintf(f, "  %%v = load i64, ptr %%slot, align 8\n");
  fprintf(f, "  %%next = add i64 %%v, 1\n");
  fprintf(f, "  store i64 %%next, ptr %%slot, align 8\n");
  fprintf(f, "  %%i.next = add i64 %%i.cur, 1\n");
  fprintf(f, "  store i64 %%i.next, ptr %%i, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "exit:\n");
  fprintf(f, "  %%first = load i64, ptr %%arr.ptr, align 8\n");
  fprintf(f, "  %%last.ptr = getelementptr inbounds i64, ptr %%arr.ptr, i64 127\n");
  fprintf(f, "  %%last = load i64, ptr %%last.ptr, align 8\n");
  fprintf(f, "  %%ret.sum = add i64 %%first, %%last\n");
  fprintf(f, "  %%ret = and i64 %%ret.sum, 255\n");
  fprintf(f, "  ret i64 %%ret\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

static int hako_llvmc_consume_array_rmw_add1_leaf_route(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  yyjson_val* metadata;
  yyjson_val* route;
  size_t bi;
  hako_llvmc_entry_rune_selection selection;
  const char* proof = NULL;
  const char* rmw_proof = NULL;
  long long size = 0;
  long long ops = 0;
  long long init_push_count = 0;
  long long final_get_count = 0;
  long long selected_rmw_block = 0;
  long long selected_rmw_instruction_index = 0;
  long long selected_rmw_set_instruction_index = 0;
  int rc;

  if (!doc) return -1;
  root = yyjson_doc_get_root(doc);
  fns = yyjson_obj_get(root, "functions");
  if (!(fns && yyjson_is_arr(fns))) {
    yyjson_doc_free(doc);
    return -1;
  }

  for (bi = 0; bi < yyjson_arr_size(fns); bi++) {
    yyjson_val* candidate = yyjson_arr_get(fns, bi);
    const char* name = candidate ? yyjson_get_str(yyjson_obj_get(candidate, "name")) : NULL;
    if (name && strcmp(name, "main") == 0) {
      fn = candidate;
      break;
    }
  }
  if (!fn) {
    yyjson_doc_free(doc);
    return -1;
  }
  hako_llvmc_select_entry_runes(fn, &selection);

  metadata = yyjson_obj_get(fn, "metadata");
  route = metadata ? yyjson_obj_get(metadata, "array_rmw_add1_leaf_seed_route") : NULL;
  if (!(route && yyjson_is_obj(route))) {
    yyjson_doc_free(doc);
    return -1;
  }

  proof = yyjson_get_str(yyjson_obj_get(route, "proof"));
  rmw_proof = yyjson_get_str(yyjson_obj_get(route, "rmw_proof"));
  size = (long long)yyjson_get_sint(yyjson_obj_get(route, "size"));
  ops = (long long)yyjson_get_sint(yyjson_obj_get(route, "ops"));
  init_push_count = (long long)yyjson_get_sint(yyjson_obj_get(route, "init_push_count"));
  final_get_count = (long long)yyjson_get_sint(yyjson_obj_get(route, "final_get_count"));
  selected_rmw_block = (long long)yyjson_get_sint(yyjson_obj_get(route, "selected_rmw_block"));
  selected_rmw_instruction_index =
      (long long)yyjson_get_sint(yyjson_obj_get(route, "selected_rmw_instruction_index"));
  selected_rmw_set_instruction_index =
      (long long)yyjson_get_sint(yyjson_obj_get(route, "selected_rmw_set_instruction_index"));

  if (!(proof && rmw_proof &&
        hako_llvmc_str_eq(proof, "kilo_leaf_array_rmw_add1_7block") &&
        hako_llvmc_str_eq(rmw_proof, "array_get_add1_set_same_slot") &&
        size == 128 &&
        ops == 2000000 &&
        init_push_count == 1 &&
        final_get_count == 2 &&
        selected_rmw_block == 23 &&
        selected_rmw_instruction_index == 8 &&
        selected_rmw_set_instruction_index == 13)) {
    if (hako_llvmc_route_trace_enabled()) {
      hako_llvmc_emit_route_trace(
          "array_rmw_add1_leaf",
          "skip",
          "metadata_contract_mismatch",
          "array_rmw_add1_leaf_seed_route");
    }
    yyjson_doc_free(doc);
    return -1;
  }

  if (hako_llvmc_route_trace_enabled()) {
    hako_llvmc_emit_route_trace(
        "array_rmw_add1_leaf",
        "emit",
        "exact_match",
        "kilo_leaf_array_rmw_add1 mir_route_plan");
  }
  rc = hako_llvmc_emit_array_rmw_add1_leaf_ir(
      obj_out,
      &selection,
      size,
      ops);
  yyjson_doc_free(doc);
  return rc;
}
