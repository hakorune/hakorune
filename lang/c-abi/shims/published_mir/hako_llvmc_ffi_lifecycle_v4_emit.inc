/* LLVM text emission from the admitted physical graph. No generic dispatcher. */
static int lv4_emit_function(FILE* f, const hako_lv4_input* in,
    const hako_llvmc_lifecycle_target_session_v1* session, size_t fi) {
  yyjson_val* fn = yyjson_arr_get(in->functions, fi);
  yyjson_val* blocks = hako_physical_blocks(fn);
  yyjson_val* params = yyjson_obj_get(fn, "params");
  int birth = fi != 0;
  if (birth) {
    fprintf(f, "define internal i32 @hako_lifecycle_birth_1(ptr %%frame");
    for (size_t i = 0; i < yyjson_arr_size(params); i++)
      fprintf(f, ", i64 %%v%u", (uint32_t)yyjson_get_uint(yyjson_arr_get(params, i)));
    fprintf(f, ") {\nstart:\n");
  } else {
    fprintf(f, "define i64 @ny_main() {\nstart:\n  %%frame = alloca [%u x i8], align %u\n",
        session->frame_size, session->frame_align);
  }
  for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    yyjson_val* block = yyjson_arr_get(blocks, bi);
    yyjson_val* rows = yyjson_obj_get(block, "instructions");
    yyjson_val* op = yyjson_obj_get(lv4_term(block), "operation");
    if (op && lv4_is(op, "kind", "new_box"))
      fprintf(f, "  %%new%u = alloca i64, align 8\n", lv4_u(block, "id"));
    for (size_t ri = 0; ri < yyjson_arr_size(rows); ri++) {
      yyjson_val* ins = yyjson_obj_get(yyjson_arr_get(rows, ri), "instruction");
      if (lv4_is(ins, "op", "object_field_get"))
        fprintf(f, "  %%read%u = alloca i64, align 8\n", lv4_u(ins, "dst"));
    }
  }
  fprintf(f, "  br label %%bb%u\n", lv4_u(fn, "entry"));
  for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    yyjson_val* block = yyjson_arr_get(blocks, bi);
    yyjson_val* rows = yyjson_obj_get(block, "instructions");
    yyjson_val* term = lv4_term(block);
    uint32_t b = lv4_u(block, "id");
    fprintf(f, "bb%u:\n", b);
    for (size_t ri = 0; ri < yyjson_arr_size(rows); ri++) {
      yyjson_val* ins = yyjson_obj_get(yyjson_arr_get(rows, ri), "instruction");
      uint32_t dst = lv4_u(ins, "dst");
      if (lv4_is(ins, "op", "fault_frame_enter")) {
        if (!birth) fprintf(f,
            "  %%init = call i32 @\"nyash.fault.frame_init_v1\"(ptr %%frame)\n"
            "  %%init_ok = icmp eq i32 %%init, 0\n"
            "  br i1 %%init_ok, label %%step%u_%zu, label %%init_failed\nstep%u_%zu:\n", b, ri, b, ri);
      } else if (lv4_is(ins, "op", "const_i64")) {
        yyjson_val* value = yyjson_obj_get(ins, "value");
        int64_t n = yyjson_is_sint(value) ? yyjson_get_sint(value) : (int64_t)yyjson_get_uint(value);
        fprintf(f, "  %%v%u = add i64 0, %lld\n", dst, (long long)n);
      } else if (lv4_is(ins, "op", "copy")) {
        fprintf(f, "  %%v%u = add i64 0, %%v%u\n", dst, lv4_u(ins, "src"));
      } else if (lv4_is(ins, "op", "add")) {
        fprintf(f, "  %%v%u = add i64 %%v%u, %%v%u\n", dst, lv4_u(ins, "lhs"), lv4_u(ins, "rhs"));
      } else if (lv4_is(ins, "op", "invoke_normal_result")) {
        fprintf(f, "  %%v%u = load i64, ptr %%new%u, align 8\n", dst, lv4_u(ins, "invoke_block"));
      } else if (lv4_is(ins, "op", "object_field_get")) {
        fprintf(f,
            "  %%get%u = call i32 @\"nyash.object.checked_field_get_i64_v1\"(i32 %u, i64 %%v%u, i64 %lld, i64 %d, ptr %%read%u)\n"
            "  %%get_ok%u = icmp eq i32 %%get%u, 0\n"
            "  br i1 %%get_ok%u, label %%step%u_%zu, label %%invalid\nstep%u_%zu:\n"
            "  %%v%u = load i64, ptr %%read%u, align 8\n",
            dst, in->profile, lv4_u(ins, "base"), (long long)in->type_id, lv4_slot(in, ins), dst,
            dst, dst, dst, b, ri, b, ri, dst, dst);
      }
      /* const_unit has no lane; unused text is eliminated, never materialized as zero. */
    }
    if (lv4_is(term, "op", "jump")) {
      fprintf(f, "  br label %%bb%u\n", lv4_u(term, "target"));
    } else if (lv4_is(term, "op", "return_fault")) {
      fprintf(f, "  br label %%fault\n");
    } else if (lv4_is(term, "op", "return")) {
      if (birth) fprintf(f, "  ret i32 0\n");
      else {
        uint32_t value = lv4_u(term, "value");
        fprintf(f,
            "  %%range_ok%u = icmp ule i64 %%v%u, 255\n"
            "  br i1 %%range_ok%u, label %%exit%u, label %%range%u\nexit%u:\n"
            "  %%disposed%u = call i32 @\"nyash.fault.frame_dispose_v1\"(ptr %%frame)\n"
            "  %%dispose_ok%u = icmp eq i32 %%disposed%u, 0\n"
            "  %%result%u = select i1 %%dispose_ok%u, i64 %%v%u, i64 70\n"
            "  ret i64 %%result%u\nrange%u:\n"
            "  %%record%u = call i32 @\"nyash.fault.record_static_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 %%v%u, i64 0)\n"
            "  %%record_ok%u = icmp eq i32 %%record%u, 1\n"
            "  br i1 %%record_ok%u, label %%fault, label %%invalid\n",
            b, value, b, b, b, b, b, b, b, b, b, value, b, b,
            b, NYRT_FAULT_REASON_EXIT_CODE_OUT_OF_RANGE_V1, (unsigned long long)in->result_site, value, b, b, b);
      }
    } else {
      yyjson_val* op = yyjson_obj_get(term, "operation");
      unsigned long long site = yyjson_get_uint(yyjson_obj_get(op, "site"));
      fprintf(f, "  %%status%u = call i32 ", b);
      if (lv4_is(op, "kind", "birth_call")) {
        yyjson_val* call = yyjson_obj_get(op, "call");
        yyjson_val* args = yyjson_obj_get(call, "args");
        fprintf(f, "@hako_lifecycle_birth_1(ptr %%frame, i64 %%v%u", lv4_u(call, "receiver"));
        for (size_t i = 0; i < yyjson_arr_size(args); i++)
          fprintf(f, ", i64 %%v%u", (uint32_t)yyjson_get_uint(yyjson_arr_get(args, i)));
        fprintf(f, ")\n");
      } else if (lv4_is(op, "kind", "new_box")) {
        fprintf(f, "@\"nyash.object.checked_new_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 %lld, ptr @hako_lifecycle_layout, i64 2, ptr %%new%u)\n",
            in->profile, site, (long long)in->type_id, b);
      } else if (lv4_is(op, "kind", "field_set")) {
        fprintf(f, "@\"nyash.object.checked_field_set_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 %%v%u, i64 %lld, i64 %d, i64 %%v%u)\n",
            in->profile, site, lv4_u(op, "base"), (long long)in->type_id, lv4_slot(in, op), lv4_u(op, "value"));
      } else {
        fprintf(f, "@\"nyash.object.%s_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 %%v%u, i64 %lld)\n",
            lv4_is(op, "kind", "home_release") ? "home_release_plain_i64" : "reclaim_unpublished",
            in->profile, site, lv4_u(op, "value"), (long long)in->type_id);
      }
      fprintf(f, "  switch i32 %%status%u, label %%invalid [ i32 0, label %%bb%u i32 1, label %%bb%u ]\n",
          b, lv4_u(term, "normal"), lv4_u(term, "fault"));
    }
  }
  if (birth) fprintf(f, "fault:\n  ret i32 1\ninvalid:\n  ret i32 2\n}\n");
  else fprintf(f,
      "fault:\n  %%reported = call i32 @\"nyash.fault.report_final_v1\"(ptr %%frame)\n"
      "  br label %%invalid\ninvalid:\n"
      "  %%disposed_bad = call i32 @\"nyash.fault.frame_dispose_v1\"(ptr %%frame)\n"
      "  ret i64 70\ninit_failed:\n  ret i64 70\n}\n");
  return !ferror(f);
}

static int lv4_emit(FILE* f, const hako_lv4_input* in,
    const hako_llvmc_lifecycle_target_session_v1* abi, const hako_lts_session* session) {
  fprintf(f, "target triple = \"%s\"\ntarget datalayout = \"%s\"\n", session->triple, session->data_layout);
  fprintf(f,
      "@hako_lifecycle_layout = private constant [2 x i32] [i32 1, i32 1]\n"
      "declare i32 @\"nyash.fault.frame_init_v1\"(ptr)\n"
      "declare i32 @\"nyash.fault.record_static_v1\"(ptr, i32, i64, i64, i64)\n"
      "declare i32 @\"nyash.fault.frame_dispose_v1\"(ptr)\n"
      "declare i32 @\"nyash.fault.report_final_v1\"(ptr)\n"
      "declare i32 @\"nyash.object.checked_new_v1\"(ptr, i32, i64, i64, ptr, i64, ptr)\n"
      "declare i32 @\"nyash.object.checked_field_set_v1\"(ptr, i32, i64, i64, i64, i64, i64)\n"
      "declare i32 @\"nyash.object.checked_field_get_i64_v1\"(i32, i64, i64, i64, ptr)\n"
      "declare i32 @\"nyash.object.home_release_plain_i64_v1\"(ptr, i32, i64, i64, i64)\n"
      "declare i32 @\"nyash.object.reclaim_unpublished_v1\"(ptr, i32, i64, i64, i64)\n");
  return lv4_emit_function(f, in, abi, 1) && lv4_emit_function(f, in, abi, 0);
}
