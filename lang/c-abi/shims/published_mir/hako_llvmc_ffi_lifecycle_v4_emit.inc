#include "hako_llvmc_ffi_lifecycle_v4_map_emit.inc"
/* LLVM text emission from the admitted physical graph. No generic dispatcher. */
static int lv4_emit_function(FILE* f, const hako_lv4_input* in,
    const hako_llvmc_lifecycle_target_session_v2* session, size_t fi) {
  yyjson_val* fn = yyjson_arr_get(in->functions, fi);
  yyjson_val* blocks = hako_physical_blocks(fn);
  yyjson_val* params = yyjson_obj_get(fn, "params");
  int birth = lv4_is(fn, "role", "birth_unit");
  int ordinary = lv4_is(fn, "role", "ordinary_i64");
  if (birth) {
    fprintf(f, "define internal i32 @hako_lifecycle_birth_%zu(ptr %%frame", fi);
    fprintf(f, ", i64 %%v%u", lv4_u(fn, "receiver"));
    for (size_t i = 0; i < yyjson_arr_size(params); i++) {
      uint32_t value = lv4_u(yyjson_arr_get(params, i), "value");
      fprintf(f, ", i32 %%k%u, i64 %%v%u", value, value);
    }
    fprintf(f, ") {\nstart:\n");
  } else if (ordinary) {
    fprintf(f, "define internal i32 @hako_lifecycle_ordinary_%zu(ptr %%frame, ptr %%out_i64", fi);
    if (yyjson_is_uint(yyjson_obj_get(fn, "receiver")))
      fprintf(f, ", i64 %%v%u", lv4_u(fn, "receiver"));
    for (size_t i = 0; i < yyjson_arr_size(params); i++) {
      uint32_t value = lv4_u(yyjson_arr_get(params, i), "value");
      fprintf(f, ", i64 %%v%u", value);
    }
    fprintf(f, ") {\nstart:\n");
  } else {
    fprintf(f, "define i64 @ny_main() {\nstart:\n  %%frame = alloca [%u x i8], align %u\n",
        session->frame_size, session->frame_align);
    fprintf(f, "  %%ordinary_out = alloca i64, align 8\n");
  }
  for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    yyjson_val* block = yyjson_arr_get(blocks, bi);
    yyjson_val* rows = yyjson_obj_get(block, "instructions");
    yyjson_val* op = yyjson_obj_get(lv4_term(block), "operation");
    if (op && (lv4_is(op, "kind", "new_box") || lv4_is(op, "kind", "array_new")))
      fprintf(f, "  %%new%u = alloca i64, align 8\n", lv4_u(block, "id"));
    lv4_map_alloca(f, op, lv4_u(block, "id"), session);
    for (size_t ri = 0; ri < yyjson_arr_size(rows); ri++) {
      yyjson_val* ins = yyjson_obj_get(yyjson_arr_get(rows, ri), "instruction");
      if (lv4_is(ins, "op", "object_field_get"))
        fprintf(f, "  %%read%u = alloca i64, align 8\n", lv4_u(ins, "dst"));
    }
  }
  if (birth) for (size_t i = 0; i < yyjson_arr_size(params); i++) {
    uint32_t v = lv4_u(yyjson_arr_get(params, i), "value");
    fprintf(f,
        "  %%arg_int%u = icmp eq i32 %%k%u, 1\n"
        "  %%arg_bool%u = icmp eq i32 %%k%u, 2\n"
        "  %%arg_bit%u = icmp ule i64 %%v%u, 1\n"
        "  %%arg_valid_bool%u = and i1 %%arg_bool%u, %%arg_bit%u\n"
        "  %%arg_ok%u = or i1 %%arg_int%u, %%arg_valid_bool%u\n"
        "  br i1 %%arg_ok%u, label %%arg_next%u, label %%invalid\narg_next%u:\n",
        v,v,v,v,v,v,v,v,v,v,v,v,v,v,v);
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
        if (!birth && !ordinary) fprintf(f,
            "  %%init = call i32 @\"nyash.fault.frame_init_v1\"(ptr %%frame)\n"
            "  %%init_ok = icmp eq i32 %%init, 0\n"
            "  br i1 %%init_ok, label %%step%u_%zu, label %%init_failed\nstep%u_%zu:\n", b, ri, b, ri);
      } else if (lv4_is(ins, "op", "const_i64")) {
        yyjson_val* value = yyjson_obj_get(ins, "value");
        int64_t n = yyjson_is_sint(value) ? yyjson_get_sint(value) : (int64_t)yyjson_get_uint(value);
        fprintf(f, "  %%v%u = add i64 0, %lld\n", dst, (long long)n);
      } else if (lv4_is(ins, "op", "const_bool")) {
        fprintf(f, "  %%v%u = add i64 0, %u\n", dst,
            yyjson_get_bool(yyjson_obj_get(ins, "value")) ? 1u : 0u);
      } else if (lv4_is(ins, "op", "const_f64_bits")) {
        fprintf(f, "  %%v%u = bitcast i64 %llu to double\n", dst,
            (unsigned long long)yyjson_get_uint(yyjson_obj_get(ins, "bits")));
      } else if (lv4_is(ins, "op", "array_residence_release")) {
        fprintf(f, "  call void @nyrt_handle_release_h(i64 %%v%u)\n", lv4_u(ins, "value"));
      } else if (lv4_is(ins, "op", "copy")) {
        uint32_t src = lv4_u(ins, "src");
        int kind = lv4_input_type(in, fn, src);
        if (kind == LV4_F64)
          fprintf(f, "  %%copy_bits%u = bitcast double %%v%u to i64\n"
              "  %%v%u = bitcast i64 %%copy_bits%u to double\n", dst, src, dst, dst);
        else fprintf(f, "  %%v%u = add i64 0, %%v%u\n", dst, src);
        if (kind == LV4_TAGGED)
          fprintf(f, "  %%k%u = add i32 0, %%k%u\n", dst, src);
      } else if (lv4_is(ins, "op", "add")) {
        fprintf(f, "  %%v%u = add i64 %%v%u, %%v%u\n", dst, lv4_u(ins, "lhs"), lv4_u(ins, "rhs"));
      } else if (lv4_is(ins, "op", "invoke_normal_result")) {
        int kind = lv4_input_type(in, fn, dst);
        const char* storage = kind == LV4_MAP ? "map" : kind == LV4_KEY ? "key" : kind == LV4_OUTCOME ? "outcome" : NULL;
        if (storage) fprintf(f, "  %%v%u = getelementptr i8, ptr %%%s%u, i64 0\n", dst, storage, lv4_u(ins, "invoke_block"));
        else if (kind == LV4_I64 && lv4_is(fn, "role", "root_i64"))
          fprintf(f, "  %%v%u = load i64, ptr %%ordinary_out, align 8\n", dst);
        else fprintf(f, "  %%v%u = load i64, ptr %%new%u, align 8\n", dst, lv4_u(ins, "invoke_block"));
      } else if (lv4_is(ins, "op", "object_field_get")) {
        fprintf(f,
            "  %%get%u = call i32 @\"nyash.object.checked_field_get_i64_v1\"(i32 %u, i64 %%v%u, i64 %lld, i64 %d, ptr %%read%u)\n"
            "  %%get_ok%u = icmp eq i32 %%get%u, 0\n"
            "  br i1 %%get_ok%u, label %%step%u_%zu, label %%invalid\nstep%u_%zu:\n"
            "  %%v%u = load i64, ptr %%read%u, align 8\n",
            dst, in->profile, lv4_u(ins, "base"), (long long)lv4_type_id(in, ins), lv4_slot(in, ins), dst,
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
      else if (ordinary) {
        fprintf(f, "  store i64 %%v%u, ptr %%out_i64, align 8\n  ret i32 0\n", lv4_u(term, "value"));
      }
      else if (lv4_is(fn, "role", "root_unit")) {
        fprintf(f,
            "  %%disposed%u = call i32 @\"nyash.fault.frame_dispose_v1\"(ptr %%frame)\n"
            "  %%dispose_ok%u = icmp eq i32 %%disposed%u, 0\n"
            "  %%result%u = select i1 %%dispose_ok%u, i64 0, i64 70\n"
            "  ret i64 %%result%u\n", b, b, b, b, b, b);
      } else {
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
      if (hako_physical_map_kind(yyjson_get_str(yyjson_obj_get(op, "kind")))) {
        lv4_map_emit_operation(f, in, term, b, fi);
        continue;
      }
      if (lv4_is(op, "kind", "field_set") &&
          lv4_input_type(in, fn, lv4_u(op, "value")) == LV4_TAGGED) {
        uint32_t value = lv4_u(op, "value");
        fprintf(f,
            "  %%store_int%u = icmp eq i32 %%k%u, 1\n"
            "  br i1 %%store_int%u, label %%store%u, label %%type_fault%u\ntype_fault%u:\n"
            "  %%type_status%u = call i32 @\"nyash.fault.record_static_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 1, i64 2)\n"
            "  switch i32 %%type_status%u, label %%invalid [ i32 1, label %%bb%u ]\nstore%u:\n",
            b,value,b,b,b,b,b,NYRT_FAULT_REASON_FIELD_TYPE_MISMATCH_V1,site,
            b,lv4_u(term,"fault"),b);
      }
      if (lv4_is(op, "kind", "array_append") && lv4_is(op, "representation", "bool"))
        fprintf(f, "  %%bool%u = trunc i64 %%v%u to i32\n", b, lv4_u(op, "value"));
      if (lv4_is(op, "kind", "ordinary_call")) {
        yyjson_val* call = yyjson_obj_get(op, "call");
        yyjson_val* args = yyjson_obj_get(call, "args");
        fprintf(f, "  %%status%u = call i32 @hako_lifecycle_ordinary_%u(ptr %%frame, ptr %%%s",
            b, lv4_u(call, "target"), ordinary ? "out_i64" : "ordinary_out");
        if (yyjson_is_uint(yyjson_obj_get(call, "receiver")))
          fprintf(f, ", i64 %%v%u", lv4_u(call, "receiver"));
        for (size_t i = 0; i < yyjson_arr_size(args); i++)
          fprintf(f, ", i64 %%v%u", lv4_u(yyjson_arr_get(args, i), "value"));
        fprintf(f, ")\n");
      } else {
      fprintf(f, "  %%status%u = call i32 ", b);
      if (lv4_is(op, "kind", "array_new")) {
        fprintf(f, "@\"nyash.array.checked_new_v1\"(ptr %%frame, i64 %llu, ptr %%new%u)\n", site, b);
      } else if (lv4_is(op, "kind", "array_claim")) {
        fprintf(f, "@\"nyash.array.checked_claim_v1\"(ptr %%frame, i64 %llu, i64 %%v%u, i32 %u)\n",
            site, lv4_u(op, "array"), lv4_u(op, "element_tag"));
      } else if (lv4_is(op, "kind", "array_append")) {
        const char* repr = yyjson_get_str(yyjson_obj_get(op, "representation"));
        const char* lane = lv4_is(op, "representation", "f64") ? "double" :
            lv4_is(op, "representation", "bool") ? "i32" : "i64";
        int boolean = lv4_is(op, "representation", "bool");
        fprintf(f, "@\"nyash.array.checked_append_%s_v1\"(ptr %%frame, i64 %llu, i64 %%v%u, %s %%%s%u)\n",
            repr, site, lv4_u(op, "array"), lane, boolean ? "bool" : "v",
            boolean ? b : lv4_u(op, "value"));
      } else if (lv4_is(op, "kind", "birth_call")) {
        yyjson_val* call = yyjson_obj_get(op, "call");
        yyjson_val* args = yyjson_obj_get(call, "args");
        fprintf(f, "@hako_lifecycle_birth_%u(ptr %%frame, i64 %%v%u", lv4_u(call, "target"), lv4_u(call, "receiver"));
        for (size_t i = 0; i < yyjson_arr_size(args); i++) {
          yyjson_val* arg = yyjson_arr_get(args, i);
          fprintf(f, ", i32 %u, i64 %%v%u", lv4_u(arg, "kind"), lv4_u(arg, "value"));
        }
        fprintf(f, ")\n");
      } else if (lv4_is(op, "kind", "new_box")) {
        fprintf(f, "@\"nyash.object.checked_new_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 %lld, ptr @hako_lifecycle_layout_%u, i64 %u, ptr %%new%u)\n",
            in->profile, site, (long long)lv4_type_id(in, op), lv4_u(op, "object_id"),
            lv4_u(lv4_layout(in, lv4_u(op, "object_id")), "field_count"), b);
      } else if (lv4_is(op, "kind", "field_set")) {
        fprintf(f, "@\"nyash.object.checked_field_set_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 %%v%u, i64 %lld, i64 %d, i64 %%v%u)\n",
            in->profile, site, lv4_u(op, "base"), (long long)lv4_type_id(in, op), lv4_slot(in, op), lv4_u(op, "value"));
      } else {
        fprintf(f, "@\"nyash.object.%s_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 %%v%u, i64 %lld)\n",
            lv4_is(op, "kind", "home_release") ? "home_release_plain_i64" : "reclaim_unpublished",
            in->profile, site, lv4_u(op, "value"), (long long)lv4_type_id(in, op));
      }
      }
      fprintf(f, "  switch i32 %%status%u, label %%invalid [ i32 0, label %%bb%u i32 1, label %%bb%u ]\n",
          b, lv4_u(term, "normal"), lv4_u(term, "fault"));
    }
  }
  if (!in->native) fprintf(f, "map_invalid:\n  call void @llvm.trap()\n  unreachable\n");
  if (birth || ordinary) fprintf(f, "fault:\n  ret i32 1\ninvalid:\n  ret i32 2\n}\n");
  else fprintf(f,
      "fault:\n  %%reported = call i32 @\"nyash.fault.report_final_v1\"(ptr %%frame)\n"
      "  br label %%invalid\ninvalid:\n"
      "  %%disposed_bad = call i32 @\"nyash.fault.frame_dispose_v1\"(ptr %%frame)\n"
      "  ret i64 70\ninit_failed:\n  ret i64 70\n}\n");
  return !ferror(f);
}

static int lv4_emit(FILE* f, const hako_lv4_input* in,
    const hako_llvmc_lifecycle_target_session_v2* abi, const hako_lts_session* session) {
  fprintf(f, "target triple = \"%s\"\ntarget datalayout = \"%s\"\n", session->triple, session->data_layout);
  fprintf(f,
      "declare i32 @\"nyash.fault.frame_init_v1\"(ptr)\n"
      "declare i32 @\"nyash.fault.record_static_v1\"(ptr, i32, i64, i64, i64)\n"
      "declare i32 @\"nyash.fault.frame_dispose_v1\"(ptr)\n"
      "declare i32 @\"nyash.fault.report_final_v1\"(ptr)\n");
  if (in->native) {
    fprintf(f,
        "declare i32 @\"nyash.array.checked_new_v1\"(ptr, i64, ptr)\n"
        "declare i32 @\"nyash.array.checked_claim_v1\"(ptr, i64, i64, i32)\n"
        "declare i32 @\"nyash.array.checked_append_i64_v1\"(ptr, i64, i64, i64)\n"
        "declare i32 @\"nyash.array.checked_append_bool_v1\"(ptr, i64, i64, i32)\n"
        "declare i32 @\"nyash.array.checked_append_f64_v1\"(ptr, i64, i64, double)\n"
        "declare void @nyrt_handle_release_h(i64)\n");
    return lv4_emit_function(f, in, abi, 0);
  }
  fprintf(f,
      "declare i32 @\"nyash.object.checked_new_v1\"(ptr, i32, i64, i64, ptr, i64, ptr)\n"
      "declare i32 @\"nyash.object.checked_field_set_v1\"(ptr, i32, i64, i64, i64, i64, i64)\n"
      "declare i32 @\"nyash.object.checked_field_get_i64_v1\"(i32, i64, i64, i64, ptr)\n"
      "declare i32 @\"nyash.object.home_release_plain_i64_v1\"(ptr, i32, i64, i64, i64)\n"
      "declare i32 @\"nyash.object.reclaim_unpublished_v1\"(ptr, i32, i64, i64, i64)\n");
  lv4_map_declarations(f, in);
  for (size_t li = 0; li < in->layout_count; li++) {
    yyjson_val* layout = in->layouts[li].row;
    uint32_t count = lv4_u(layout, "field_count");
    fprintf(f, "@hako_lifecycle_layout_%u = private constant [%u x i32] [", in->layouts[li].id, count);
    for (uint32_t i = 0; i < count; i++) fprintf(f, "%si32 %u", i ? ", " : "", HAKO_LLVMC_LIFECYCLE_STORAGE_I64);
    fprintf(f, "]\n");
  }
  for (size_t fi = 1; fi < in->index_count; fi++)
    if (!lv4_emit_function(f, in, abi, fi)) return 0;
  return lv4_emit_function(f, in, abi, 0);
}
