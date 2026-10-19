/* Shared pair transport for ordinary borrows and Normal-only Birth moves. */
static void lv4_emit_nullable_actual_tags(FILE* f, yyjson_val* args, uint32_t b) {
        for (size_t i = 0; i < yyjson_arr_size(args); i++) {
          yyjson_val* arg = yyjson_arr_get(args, i);
          uint32_t value = lv4_u(arg, "value");
          if (lv4_is(arg, "kind", "nullable_typed_object"))
            fprintf(f,
                "  %%nol%u_%u_%zu = icmp ne i64 %%v%u, 0\n"
                "  %%nok%u_%u_%zu = select i1 %%nol%u_%u_%zu, i32 3, i32 0\n",
                value, b, i, value, value, b, i, value, b, i);
        }

}
static int lv4_emit_pair_actual(FILE* f, yyjson_val* arg, uint32_t b, size_t i) {
  uint32_t value = lv4_u(arg, "value");
  yyjson_val* tag = yyjson_obj_get(arg, "kind");
  if (yyjson_is_uint(tag))
    fprintf(f, ", i32 %u, i64 %%v%u", (uint32_t)yyjson_get_uint(tag), value);
  else if (lv4_is(arg, "kind", "borrowed_text"))
    fprintf(f, ", i32 4, i64 %%textbits%u_%zu", b, i);
  else if (lv4_is(arg, "kind", "tagged"))
    fprintf(f, ", i32 %%k%u, i64 %%v%u", value, value);
  else if (lv4_is(arg, "kind", "nullable_typed_object"))
    fprintf(f, ", i32 %%nok%u_%u_%zu, i64 %%v%u", value, b, i, value);
  else return 0;
  return 1;
}

/* Shared bare/Invoke usize check. The caller owns the exact Fault edge. */
static void lv4_emit_usize_field_range(FILE* f, uint32_t b, size_t ri,
    uint32_t fstored, unsigned long long site, const char* fault_label) {
          fprintf(f,
              "  %%rc%u_%zu = icmp slt i64 %%v%u, 0\n"
              "  br i1 %%rc%u_%zu, label %%rcf%u_%zu, label %%fst%u_%zu\nrcf%u_%zu:\n"
              "  %%rcs%u_%zu = call i32 @\"nyash.fault.record_static_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 %%v%u, i64 0)\n"
              "  %%rco%u_%zu = icmp eq i32 %%rcs%u_%zu, 1\n"
              "  br i1 %%rco%u_%zu, label %%%s, label %%invalid\nfst%u_%zu:\n",
              b, ri, fstored, b, ri, b, ri, b, ri, b, ri, b, ri,
              NYRT_FAULT_REASON_FIELD_RANGE_V1, site, fstored, b, ri, b, ri, b, ri, fault_label, b, ri);
}

/* A physical edge owns one LLVM predecessor even when its implementation
 * expands into several status/cleanup blocks. Native admission has no PHIs. */
static int lv4_edge_label(char label[64], const hako_lv4_input* in, size_t fi,
    uint32_t source, size_t ordinal, uint32_t target) {
  if (!in->native) {
    hako_lv4_block* block = lv4_index_block(&in->index[fi], target);
    yyjson_val* first = block ? yyjson_arr_get(yyjson_obj_get(block->block, "instructions"), 0) : NULL;
    if (lv4_is(yyjson_obj_get(first, "instruction"), "op", "phi")) {
      snprintf(label, 64, "edge_%u_%zu", source, ordinal);
      return 1;
    }
  }
  snprintf(label, 64, "bb%u", target);
  return 0;
}

#include "hako_llvmc_ffi_lifecycle_v4_map_emit.inc"
#include "hako_llvmc_ffi_lifecycle_v4_string_emit.inc"
#include "hako_llvmc_ffi_lifecycle_v4_script_arguments_emit.inc"

#include "hako_llvmc_ffi_lifecycle_v4_object_result_emit.inc"
#include "hako_llvmc_ffi_lifecycle_v4_object_residence_emit.inc"
#include "hako_llvmc_ffi_lifecycle_v4_teardown_emit.inc"
#include "hako_llvmc_ffi_lifecycle_v4_owned_array_emit.inc"


#include "hako_llvmc_ffi_lifecycle_v4_return_emit.inc"

/* LLVM text emission from the admitted physical graph. No generic dispatcher. */
static int lv4_emit_function(FILE* f, const hako_lv4_input* in,
    const hako_llvmc_lifecycle_target_session_v2* session, size_t fi) {
  if (in->measurement) in->measurement->v4_emit_functions++;
  yyjson_val* fn = yyjson_arr_get(in->functions, fi);
  yyjson_val* blocks = hako_physical_blocks(fn);
  yyjson_val* params = yyjson_obj_get(fn, "params");
  int birth = lv4_is(fn, "role", "birth_unit");
  int ordinary_unit = lv4_is(fn, "role", "ordinary_unit");
  int ordinary_map = lv4_is(fn, "role", "ordinary_map");
  int ordinary_text = lv4_is(fn, "role", "ordinary_text");
  int ordinary = lv4_is_ordinary(fn);
  if (birth) {
    fprintf(f, "define internal i32 @hako_lifecycle_birth_%zu(ptr %%frame", fi);
    fprintf(f, ", i64 %%v%u", lv4_u(fn, "receiver"));
    for (size_t i = 0; i < yyjson_arr_size(params); i++) {
      uint32_t value = lv4_u(yyjson_arr_get(params, i), "value");
      fprintf(f, ", i32 %%k%u, i64 %%v%u", value, value);
      if (lv4_is(yyjson_arr_get(params, i), "representation", "nullable_kind_payload_v1"))
        fprintf(f, ", i32 %%o%u", value);
    }
    fprintf(f, ") {\nstart:\n");
  } else if (ordinary) {
    fprintf(f, "define internal i32 @hako_lifecycle_ordinary_%zu(ptr %%frame", fi);
    if (!ordinary_unit) fprintf(f, ", ptr %%%s", ordinary_map ? "out_map" : ordinary_text ? "out_text" : "out_i64");
    if (hako_physical_object_result_role(fn)) fprintf(f, ", ptr %%out_outcome");
    if (yyjson_is_uint(yyjson_obj_get(fn, "receiver")))
      fprintf(f, ", i64 %%v%u", lv4_u(fn, "receiver"));
    for (size_t i = 0; i < yyjson_arr_size(params); i++) {
      yyjson_val* param = yyjson_arr_get(params, i);
      if (lv4_is(param, "representation", "borrowed_kind_payload_v1")) {
        uint32_t v = lv4_u(param, "value");
        fprintf(f, ", i32 %%k%u, i64 %%v%u", v, v);
        continue;
      }
      /* Borrowed checked-map storage arrives as a pointer, not an i64. */
      fprintf(f, lv4_is(param, "representation", "map") ? ", ptr %%v%u" : ", i64 %%v%u",
          lv4_u(param, "value"));
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
    if (in->measurement) {
      in->measurement->v4_emit_blocks++;
      in->measurement->v4_emit_rows += yyjson_arr_size(rows);
    }
    yyjson_val* op = yyjson_obj_get(lv4_term(block), "operation");
    if (op && (lv4_is(op, "kind", "new_box") || lv4_is(op, "kind", "array_new") || lv4_is(op, "kind", "script_arguments_new")))
      fprintf(f, "  %%new%u = alloca i64, align 8\n", lv4_u(block, "id"));
    if (op && lv4_is(op, "kind", "field_get"))
      fprintf(f, "  %%field_read%u = alloca i64, align 8\n", lv4_u(block, "id"));
    if (op && lv4_is(op, "kind", "field_residence_release"))
      fprintf(f, "  %%fr_read%u = alloca i64, align 8\n", lv4_u(block, "id"));
    if (op && lv4_is(op, "kind", "object_field_release"))
      fprintf(f, "  %%ofr_read%u = alloca { i64, i32, i32 }, align 8\n", lv4_u(block, "id"));
    /* A nested i64 call writes through a caller-local slot; the callee's
       own %%out_i64 remains reserved for this function's result. A birth
       caller's sealed provider-argument calls are ExactI64 only — the
       flow arm already closed every other result kind. */
    if ((ordinary || birth) && lv4_is(op, "kind", "ordinary_call") &&
        !lv4_is(op, "result", "map") && !lv4_is(op, "result", "unit") && !lv4_is(op, "result", "text"))
      fprintf(f, "  %%call_out%u = alloca i64, align 8\n", lv4_u(block, "id"));
    if (hako_physical_object_result_call(op))
      fprintf(f, "  %%call_outcome%u = alloca i32, align 4\n", lv4_u(block, "id"));
    if (op && lv4_is(op, "kind", "array_owned_get"))
      fprintf(f, "  %%owned_get%u = alloca i64, align 8\n", lv4_u(block, "id"));
    lv4_map_alloca(f, op, lv4_u(block, "id"), session);
    lv4_string_alloca(f, op, lv4_u(block, "id"));
    for (size_t ri = 0; ri < yyjson_arr_size(rows); ri++) {
      yyjson_val* ins = yyjson_obj_get(yyjson_arr_get(rows, ri), "instruction");
      if (lv4_is(ins, "op", "object_field_get"))
        fprintf(f, "  %%read%u = alloca { i64, i32, i32 }, align 8\n", lv4_u(ins, "dst"));
    }
  }
  if (birth) for (size_t i = 0; i < yyjson_arr_size(params); i++) {
    yyjson_val* param = yyjson_arr_get(params, i);
    uint32_t v = lv4_u(param, "value");
    if (lv4_is(param, "representation", "nullable_kind_payload_v1")) {
      lv4_emit_object_residence_parameter_guard(f, v);
      continue;
    }
    fprintf(f,
        "  %%arg_int%u = icmp eq i32 %%k%u, 1\n"
        "  %%arg_bool%u = icmp eq i32 %%k%u, 2\n"
        "  %%arg_bit%u = icmp ule i64 %%v%u, 1\n"
        "  %%arg_valid_bool%u = and i1 %%arg_bool%u, %%arg_bit%u\n"
        "  %%arg_ok%u = or i1 %%arg_int%u, %%arg_valid_bool%u\n"
        "  br i1 %%arg_ok%u, label %%arg_next%u, label %%invalid\narg_next%u:\n",
        v,v,v,v,v,v,v,v,v,v,v,v,v,v,v);
  }
  if (ordinary) for (size_t i = 0; i < yyjson_arr_size(params); i++) {
    yyjson_val* param = yyjson_arr_get(params, i);
    if (!lv4_is(param, "representation", "borrowed_kind_payload_v1")) continue;
    uint32_t v = lv4_u(param, "value");
    yyjson_val* object_view = yyjson_obj_get(param, "object_view");
    if (yyjson_is_true(yyjson_obj_get(param, "integer_view"))) {
      /* This is an ABI projection before the first body effect, not a
         coercion or a guarded mixed-domain numeric operation. */
      fprintf(f,
          "  %%arg_int%u = icmp eq i32 %%k%u, 1\n"
          "  br i1 %%arg_int%u, label %%arg_next%u, label %%invalid\narg_next%u:\n",
          v,v,v,v,v);
      continue;
    }
    if (yyjson_is_uint(object_view)) {
      uint32_t runtime_type = lv4_u(lv4_layout(in, (uint32_t)yyjson_get_uint(object_view)), "runtime_type_id");
      /* Caller provenance remains independently verified. A class domain
         validates runtime class before any callee effect; no ownership change. */
      fprintf(f,
          "  %%arg_null%u = icmp eq i32 %%k%u, 0\n"
          "  %%arg_zero%u = icmp eq i64 %%v%u, 0\n"
          "  %%arg_nulok%u = and i1 %%arg_null%u, %%arg_zero%u\n"
          "  br i1 %%arg_nulok%u, label %%arg_next%u, label %%arg_object_check%u\n"
          "arg_object_check%u:\n"
          "  %%arg_object%u = icmp eq i32 %%k%u, 3\n"
          "  %%arg_nonzero%u = icmp ne i64 %%v%u, 0\n"
          "  %%arg_live%u = and i1 %%arg_object%u, %%arg_nonzero%u\n"
          "  br i1 %%arg_live%u, label %%arg_type_check%u, label %%invalid\n"
          "arg_type_check%u:\n"
          "  %%arg_type%u = call i64 @\"nyash.object.type_id_h\"(i64 %%v%u)\n"
          "  %%arg_class%u = icmp eq i64 %%arg_type%u, %u\n"
          "  br i1 %%arg_class%u, label %%arg_next%u, label %%invalid\narg_next%u:\n",
          v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,runtime_type,v,v,v);
      continue;
    }
    if ((lv4_is(fn, "role", "ordinary_borrowed_nullable_handle") ||
        lv4_is(fn, "role", "ordinary_mixed_handle")) &&
        i == lv4_u(fn, "result_formal_ordinal")) {
      lv4_emit_borrowed_result_formal_guard(f, v);
      continue;
    }
    if (yyjson_is_true(yyjson_obj_get(param, "text_view"))) {
      fprintf(f,
          "  %%arg_text%u = icmp eq i32 %%k%u, 4\n"
          "  %%arg_textptr%u = icmp ne i64 %%v%u, 0\n"
          "  %%arg_textok%u = and i1 %%arg_text%u, %%arg_textptr%u\n"
          "  br i1 %%arg_textok%u, label %%arg_next%u, label %%invalid\narg_next%u:\n",
          v,v, v,v, v,v,v, v,v,v);
      continue;
    }
    /* Nonzero supplements the closed incoming/live proof; it is not that
       proof. The null pair (kind 0, payload 0) is the third admitted
       shape — every other kind/payload mix is ABI drift, never a
       silently nulled carrier. */
    fprintf(f,
        "  %%arg_int%u = icmp eq i32 %%k%u, 1\n"
        "  %%arg_bool%u = icmp eq i32 %%k%u, 2\n"
        "  %%arg_bit%u = icmp ule i64 %%v%u, 1\n"
        "  %%arg_valid_bool%u = and i1 %%arg_bool%u, %%arg_bit%u\n"
        "  %%arg_scalar%u = or i1 %%arg_int%u, %%arg_valid_bool%u\n"
        "  %%arg_object%u = icmp eq i32 %%k%u, 3\n"
        "  %%arg_nonzero%u = icmp ne i64 %%v%u, 0\n"
        "  %%arg_live%u = and i1 %%arg_object%u, %%arg_nonzero%u\n"
        "  %%arg_null%u = icmp eq i32 %%k%u, 0\n"
        "  %%arg_void%u = icmp eq i64 %%v%u, 0\n"
        "  %%arg_nulok%u = and i1 %%arg_null%u, %%arg_void%u\n"
        "  %%arg_pre%u = or i1 %%arg_scalar%u, %%arg_live%u\n"
        "  %%arg_ok%u = or i1 %%arg_pre%u, %%arg_nulok%u\n"
        "  br i1 %%arg_ok%u, label %%arg_next%u, label %%invalid\narg_next%u:\n",
        v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v,v);
  }
  fprintf(f, "  br label %%bb%u\n", lv4_u(fn, "entry"));
  for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    yyjson_val* block = yyjson_arr_get(blocks, bi);
    yyjson_val* rows = yyjson_obj_get(block, "instructions");
    yyjson_val* term = lv4_term(block);
    uint32_t b = lv4_u(block, "id");
    char normal_label[64], fault_label[64];
    yyjson_val* edges = yyjson_obj_get(block, "edges");
    lv4_edge_label(normal_label, in, fi, b, 0, lv4_u(yyjson_arr_get(edges, 0), "target"));
    lv4_edge_label(fault_label, in, fi, b, 1, lv4_u(yyjson_arr_get(edges, 1), "target"));
    fprintf(f, "bb%u:\n", b);
    for (size_t ri = 0; ri < yyjson_arr_size(rows); ri++) {
      yyjson_val* ins = yyjson_obj_get(yyjson_arr_get(rows, ri), "instruction");
      uint32_t dst = lv4_u(ins, "dst");
      if (lv4_is(ins, "op", "fault_frame_enter")) {
        if (!birth && !ordinary) fprintf(f,
            "  %%init = call i32 @\"nyash.fault.frame_init_v1\"(ptr %%frame)\n"
            "  %%init_ok = icmp eq i32 %%init, 0\n"
            "  br i1 %%init_ok, label %%step%u_%zu, label %%init_failed\nstep%u_%zu:\n", b, ri, b, ri);
      } else if (lv4_is(ins, "op", "keep_alive")) {
        /* MIR optimization metadata only: emit no LLVM or runtime retain. */
      } else if (lv4_is(ins, "op", "const_i64")) {
        yyjson_val* value = yyjson_obj_get(ins, "value");
        int64_t n = yyjson_is_sint(value) ? yyjson_get_sint(value) : (int64_t)yyjson_get_uint(value);
        fprintf(f, "  %%v%u = add i64 0, %lld\n", dst, (long long)n);
      } else if (lv4_is(ins, "op", "const_bool")) {
        fprintf(f, "  %%v%u = add i64 0, %u\n", dst,
            yyjson_get_bool(yyjson_obj_get(ins, "value")) ? 1u : 0u);
      } else if (lv4_is(ins, "op", "const_null")) {
        /* Void null sentinel: i64 0 on the wire; `home_release_if_live`
           is the sole consumer that must not release it. */
        fprintf(f, "  %%v%u = add i64 0, 0\n", dst);
      } else if (lv4_is(ins, "op", "const_f64_bits")) {
        fprintf(f, "  %%v%u = bitcast i64 %llu to double\n", dst,
            (unsigned long long)yyjson_get_uint(yyjson_obj_get(ins, "bits")));
      } else if (lv4_is(ins, "op", "string_residence_install")) {
        /* Validated lifecycle handoff preserves the existing handle/token pair. */
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
      } else if (lv4_is(ins, "op", "add") || lv4_is(ins, "op", "mul")) {
        uint32_t alhs = lv4_u(ins, "lhs"), arhs = lv4_u(ins, "rhs");
        /* A dominated arithmetic operand may read the same lent Integer view;
           the kind==1 branch proves the i64 read at the use site and any
           other kind is a semantic Fault, uniform with the compare. */
        if (lv4_input_type(in, fn, alhs) == LV4_TAGGED)
          fprintf(f, "  %%aok%u_%u_%zu = icmp eq i32 %%k%u, 1\n"
              "  br i1 %%aok%u_%u_%zu, label %%as%u_%u_%zu, label %%fault\nas%u_%u_%zu:\n",
              alhs, b, ri, alhs, alhs, b, ri, alhs, b, ri, alhs, b, ri);
        if (arhs != alhs && lv4_input_type(in, fn, arhs) == LV4_TAGGED)
          fprintf(f, "  %%aok%u_%u_%zu = icmp eq i32 %%k%u, 1\n"
              "  br i1 %%aok%u_%u_%zu, label %%as%u_%u_%zu, label %%fault\nas%u_%u_%zu:\n",
              arhs, b, ri, arhs, arhs, b, ri, arhs, b, ri, arhs, b, ri);
        fprintf(f, "  %%v%u = %s i64 %%v%u, %%v%u\n", dst,
            lv4_is(ins, "op", "mul") ? "mul" : "add", alhs, arhs);
      } else if (lv4_is(ins, "op", "sub") || lv4_is(ins, "op", "div")) {
        /* Admission requires two I64 operands and a positive literal divisor.
           Plain instructions preserve current-I64 wrap/truncation policy. */
        fprintf(f, "  %%v%u = %s i64 %%v%u, %%v%u\n", dst,
            lv4_is(ins, "op", "sub") ? "sub" : "sdiv",
            lv4_u(ins, "lhs"), lv4_u(ins, "rhs"));
      } else if (lv4_is(ins, "op", "compare")) {
        uint32_t clhs = lv4_u(ins, "lhs"), crhs = lv4_u(ins, "rhs");
        /* A borrowed tagged operand may lend the checked-compare Integer
           view only while its kind lane is exactly 1 at the use site;
           any other kind is a semantic Fault, never a silent rewrap. */
        if (lv4_input_type(in, fn, clhs) == LV4_TAGGED)
          fprintf(f, "  %%cok%u_%u_%zu = icmp eq i32 %%k%u, 1\n"
              "  br i1 %%cok%u_%u_%zu, label %%cs%u_%u_%zu, label %%fault\ncs%u_%u_%zu:\n",
              clhs, b, ri, clhs, clhs, b, ri, clhs, b, ri, clhs, b, ri);
        if (crhs != clhs && lv4_input_type(in, fn, crhs) == LV4_TAGGED)
          fprintf(f, "  %%cok%u_%u_%zu = icmp eq i32 %%k%u, 1\n"
              "  br i1 %%cok%u_%u_%zu, label %%cs%u_%u_%zu, label %%fault\ncs%u_%u_%zu:\n",
              crhs, b, ri, crhs, crhs, b, ri, crhs, b, ri, crhs, b, ri);
        fprintf(f, "  %%cmp%u = icmp %s i64 %%v%u, %%v%u\n"
            "  %%v%u = zext i1 %%cmp%u to i64\n", dst,
            yyjson_get_str(yyjson_obj_get(ins, "predicate")),
            clhs, crhs, dst, dst);
      } else if (lv4_is(ins, "op", "borrowed_null_compare")) {
        /* The borrowed/null equality reads the carrier's kind or payload
           lane only — never the Integer view — so Null answers true and
           every well-formed non-null kind answers false with no fault
           branch and no payload read of the tagged carrier. */
        uint32_t nlhs = lv4_u(ins, "lhs"), nrhs = lv4_u(ins, "rhs");
        hako_lv4_value* lrow = lv4_index_value(&in->index[fi], nlhs);
        hako_lv4_value* rrow = lv4_index_value(&in->index[fi], nrhs);
        hako_lv4_value* carrier;
        if (!lrow || !rrow) return 0;
        if (lv4_is(lrow->instruction, "op", "const_null") ==
            lv4_is(rrow->instruction, "op", "const_null")) return 0;
        carrier = lv4_is(lrow->instruction, "op", "const_null") ? rrow : lrow;
        if (carrier->kind == LV4_TAGGED)
          fprintf(f, "  %%nul%u = icmp %s i32 %%k%u, 0\n"
              "  %%v%u = zext i1 %%nul%u to i64\n", dst,
              yyjson_get_str(yyjson_obj_get(ins, "predicate")), carrier->id, dst, dst);
        else if (carrier->kind == LV4_HANDLE || carrier->kind == LV4_BORROWED_OBJECT || carrier->kind == LV4_MIXED_OBJECT)
          fprintf(f, "  %%nul%u = icmp %s i64 %%v%u, 0\n"
              "  %%v%u = zext i1 %%nul%u to i64\n", dst,
              yyjson_get_str(yyjson_obj_get(ins, "predicate")), carrier->id, dst, dst);
        else
          fprintf(f, "  %%v%u = add i64 0, %d\n", dst,
              lv4_is(ins, "predicate", "ne") ? 1 : 0);
      } else if (lv4_is(ins, "op", "phi")) {
        yyjson_val* inputs = yyjson_obj_get(ins, "inputs");
        fprintf(f, "  %%v%u = phi i64 ", dst);
        for (size_t pi = 0; pi < yyjson_arr_size(inputs); pi++) {
          yyjson_val* input = yyjson_arr_get(inputs, pi);
          uint32_t pred = lv4_u(input, "block");
          hako_lv4_block* predecessor = lv4_index_block(&in->index[fi], pred);
          yyjson_val* incoming = yyjson_obj_get(predecessor->block, "edges");
          size_t ordinal = 0;
          while (ordinal < yyjson_arr_size(incoming) &&
              lv4_u(yyjson_arr_get(incoming, ordinal), "target") != b) ordinal++;
          if (ordinal == yyjson_arr_size(incoming)) return 0;
          char label[64];
          lv4_edge_label(label, in, fi, pred, ordinal, b);
          fprintf(f, "%s[ %%v%u, %%%s ]", pi ? ", " : "", lv4_u(input, "value"), label);
        }
        fprintf(f, "\n");
      } else if (lv4_is(ins, "op", "invoke_normal_result")) {
        int kind = lv4_input_type(in, fn, dst);
        hako_lv4_block* producer = in->native ? NULL : lv4_index_block(&in->index[fi], lv4_u(ins, "invoke_block"));
        yyjson_val* producer_op = producer ? yyjson_obj_get(lv4_term(producer->block), "operation") :
            in->native ? yyjson_obj_get(lv4_term(hako_physical_block_by_id(fn, lv4_u(ins, "invoke_block"))), "operation") : NULL;
        const char* storage = kind == LV4_MAP ? "map" : kind == LV4_KEY ? "key" :
            kind == LV4_OUTCOME ? "outcome" : kind == LV4_MAP_VIEW ? "view" :
            kind == LV4_TEXT_VIEW ? "text" : NULL;
        if (kind == LV4_OWNED_STRING) {
          uint32_t producer_id = lv4_u(ins, "invoke_block");
          fprintf(f,
              "  %%v%u = load i64, ptr %%strout%u, align 8\n"
              "  %%tokenptr%u = getelementptr inbounds %%nyrt.string.owned, ptr %%strout%u, i32 0, i32 1\n"
              "  %%t%u = load i64, ptr %%tokenptr%u, align 8\n",
              dst, producer_id, dst, producer_id, dst, dst);
        } else if ((kind == LV4_I64 || kind == LV4_BOOL) &&
            hako_physical_string_result(yyjson_get_str(yyjson_obj_get(producer_op, "kind"))) == (kind == LV4_BOOL ? 3 : 2))
          fprintf(f, "  %%v%u = load i64, ptr %%strout%u, align 8\n", dst, lv4_u(ins, "invoke_block"));
        else if (kind == LV4_ARRAY_BORROW)
          fprintf(f, "  %%v%u = load i64, ptr %%owned_get%u, align 8\n", dst, lv4_u(ins, "invoke_block"));
        else if (storage) fprintf(f, "  %%v%u = getelementptr i8, ptr %%%s%u, i64 0\n", dst, storage, lv4_u(ins, "invoke_block"));
        else if (kind == LV4_I64 && (lv4_is(producer_op, "kind", "map_checked_get") ||
            lv4_is(producer_op, "kind", "map_array_length")))
          fprintf(f, "  %%v%u = load i64, ptr %%get_out%u, align 8\n", dst, lv4_u(ins, "invoke_block"));
        else if (producer_op && lv4_is(producer_op, "kind", "field_get"))
          fprintf(f, "  %%v%u = load i64, ptr %%field_read%u, align 8\n", dst, lv4_u(ins, "invoke_block"));
        else if ((kind == LV4_I64 || kind == LV4_BOOL || kind == LV4_HANDLE || kind == LV4_BORROWED_OBJECT || kind == LV4_MIXED_OBJECT) &&
            producer_op && lv4_is(producer_op, "kind", "ordinary_call")) {
          /* A nullable call result lands in the same i64 out-slot as a
             scalar one; the handle-or-sentinel distinction is carried by
             the value, not the storage. */
          if (fi)
            fprintf(f, "  %%v%u = load i64, ptr %%call_out%u, align 8\n", dst, lv4_u(ins, "invoke_block"));
          else
            fprintf(f, "  %%v%u = load i64, ptr %%ordinary_out, align 8\n", dst);
          if (hako_physical_object_result_call(producer_op))
            lv4_emit_object_result_normal(f, producer_op, lv4_u(ins, "invoke_block"), dst);
        }
        else if (kind == LV4_I64 && fi)
          fprintf(f, "  %%v%u = load i64, ptr %%call_out%u, align 8\n", dst, lv4_u(ins, "invoke_block"));
        else if (kind == LV4_I64)
          fprintf(f, "  %%v%u = load i64, ptr %%ordinary_out, align 8\n", dst);
        else fprintf(f, "  %%v%u = load i64, ptr %%new%u, align 8\n", dst, lv4_u(ins, "invoke_block"));
      } else if (lv4_is(ins, "op", "object_field_get")) {
        if (lv4_object_residence(in, lv4_u(ins, "object_id"), lv4_u(ins, "field_ordinal"))) {
          char base[32], out[32];
          snprintf(base, sizeof(base), "%%v%u", lv4_u(ins, "base"));
          snprintf(out, sizeof(out), "read%u", dst);
          if (!lv4_emit_object_residence_read(f, in, lv4_u(ins, "object_id"),
              lv4_u(ins, "field_ordinal"), base, out, "invalid")) return 0;
          fprintf(f, "  %%v%u = load i64, ptr %%read%u, align 8\n", dst, dst);
        } else {
        fprintf(f,
            "  %%get%u = call i32 @\"nyash.object.checked_field_get_i64_v1\"(i32 %u, i64 %%v%u, i64 %lld, i64 %d, ptr %%read%u)\n"
            "  %%get_ok%u = icmp eq i32 %%get%u, 0\n"
            "  br i1 %%get_ok%u, label %%step%u_%zu, label %%invalid\nstep%u_%zu:\n"
            "  %%v%u = load i64, ptr %%read%u, align 8\n",
            dst, in->profile, lv4_u(ins, "base"), (long long)lv4_type_id(in, ins), lv4_slot(in, ins), dst,
            dst, dst, dst, b, ri, b, ri, dst, dst);
        }
      } else if (lv4_is(ins, "op", "field_set")) {
        yyjson_val* check = yyjson_obj_get(ins, "exact_numeric_runtime_check");
        unsigned long long site =
            (unsigned long long)yyjson_get_uint(yyjson_obj_get(ins, "site"));
        uint32_t fbase = lv4_u(ins, "base"), fstored = lv4_u(ins, "value");
        if (lv4_is(check, "kind", "dynamic_integer_range"))
          /* usize write lane: the i64 lane is in range exactly when it is
             non-negative. The emitted branch is the range authority's own
             lowering; a negative value is a semantic Fault at this row. */
          lv4_emit_usize_field_range(f, b, ri, fstored, site, "fault");
        fprintf(f,
            "  %%set%u_%zu = call i32 @\"nyash.object.checked_field_set_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 %%v%u, i64 %lld, i64 %d, i64 %%v%u)\n"
            "  switch i32 %%set%u_%zu, label %%invalid [ i32 0, label %%sd%u_%zu i32 1, label %%fault ]\nsd%u_%zu:\n",
            b, ri, in->profile, site, fbase,
            (long long)lv4_type_id(in, ins), lv4_slot(in, ins), fstored,
            b, ri, b, ri, b, ri);
      } else if (lv4_is(ins, "op", "array_set")) {
        uint32_t sarray = lv4_u(ins, "array"), sindex = lv4_u(ins, "index"),
            svalue = lv4_u(ins, "value");
        unsigned long long site =
            (unsigned long long)yyjson_get_uint(yyjson_obj_get(ins, "site"));
        /* A dominated `.set` element value may read the same lent Integer
           view; the kind==1 branch proves the i64 payload at the use site,
           uniform with the checked compare and the dominated `+`. */
        if (lv4_input_type(in, fn, svalue) == LV4_TAGGED)
          fprintf(f, "  %%vok%u_%u_%zu = icmp eq i32 %%k%u, 1\n"
              "  br i1 %%vok%u_%u_%zu, label %%vs%u_%u_%zu, label %%fault\nvs%u_%u_%zu:\n",
              svalue, b, ri, svalue, svalue, b, ri, svalue, b, ri, svalue, b, ri);
        fprintf(f,
            "  %%aset%u_%zu = call i32 @\"nyash.array.checked_set_i64_v1\"(ptr %%frame, i64 %llu, i64 %%v%u, i64 %%v%u, i64 %%v%u)\n"
            "  switch i32 %%aset%u_%zu, label %%invalid [ i32 0, label %%asd%u_%zu i32 1, label %%fault ]\nasd%u_%zu:\n",
            b, ri, site, sarray, sindex, svalue, b, ri, b, ri, b, ri);
      } else if (lv4_is(ins, "op", "array_length")) {
        fprintf(f, "  %%v%u = call i64 @\"nyash.array.slot_len_h\"(i64 %%v%u)\n",
            lv4_u(ins, "dst"), lv4_u(ins, "array"));
      } else if (lv4_is(ins, "op", "array_get")) {
        /* Same surface as the generic lane's `call_method` get lowering:
           the i64-or-handle carrier, miss → the lane's null sentinel 0.
           No fault edge — the read records nothing at a site. */
        fprintf(f,
            "  %%v%u = call i64 @\"nyash.array.slot_load_hi\"(i64 %%v%u, i64 %%v%u)\n",
            lv4_u(ins, "dst"), lv4_u(ins, "array"), lv4_u(ins, "index"));
      }
      /* const_unit has no lane; unused text is eliminated, never materialized as zero. */
    }
    if (lv4_is(term, "op", "jump")) {
      fprintf(f, "  br label %%%s\n", normal_label);
    } else if (lv4_is(term, "op", "branch")) {
      fprintf(f, "  %%branch%u = icmp ne i64 %%v%u, 0\n"
          "  br i1 %%branch%u, label %%%s, label %%%s\n",
          b, lv4_u(term, "condition"), b, normal_label, fault_label);
    } else if (lv4_is(term, "op", "return_fault")) {
      fprintf(f, "  br label %%fault\n");
    } else if (lv4_is(term, "op", "return")) {
      if (!lv4_emit_return(f, in, fn, term, b, birth, ordinary_map, ordinary)) return 0;
    } else {
      yyjson_val* op = yyjson_obj_get(term, "operation");
      unsigned long long site = yyjson_get_uint(yyjson_obj_get(op, "site"));
      if (lv4_string_operation(op)) {
        lv4_string_emit(f, op, fi, b, normal_label, fault_label);
        continue;
      }
      if (hako_physical_map_kind(yyjson_get_str(yyjson_obj_get(op, "kind")))) {
        lv4_map_emit_operation(f, in, term, b, fi, normal_label, fault_label);
        continue;
      }
      if (lv4_is(op, "kind", "field_set") &&
          lv4_input_type(in, fn, lv4_u(op, "value")) == LV4_TAGGED) {
        uint32_t value = lv4_u(op, "value");
        fprintf(f,
            "  %%store_int%u = icmp eq i32 %%k%u, 1\n"
            "  br i1 %%store_int%u, label %%store%u, label %%type_fault%u\ntype_fault%u:\n"
            "  %%type_status%u = call i32 @\"nyash.fault.record_static_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 1, i64 2)\n"
            "  switch i32 %%type_status%u, label %%invalid [ i32 1, label %%%s ]\nstore%u:\n",
            b,value,b,b,b,b,b,NYRT_FAULT_REASON_FIELD_TYPE_MISMATCH_V1,site,
            b,fault_label,b);
      }
      if (lv4_is(op, "kind", "field_set") &&
          lv4_is(yyjson_obj_get(op, "exact_numeric_runtime_check"), "kind", "dynamic_integer_range"))
        lv4_emit_usize_field_range(f, b, yyjson_get_uint(yyjson_obj_get(yyjson_obj_get(block, "terminator"), "index")),
            lv4_u(op, "value"), site, fault_label);
      if (lv4_is(op, "kind", "array_append") && lv4_is(op, "representation", "bool"))
        fprintf(f, "  %%bool%u = trunc i64 %%v%u to i32\n", b, lv4_u(op, "value"));
      if (lv4_is(op, "kind", "ordinary_call")) {
        yyjson_val* call = yyjson_obj_get(op, "call");
        yyjson_val* args = yyjson_obj_get(call, "args");
        for (size_t i = 0; i < yyjson_arr_size(args); i++) {
          yyjson_val* arg = yyjson_arr_get(args, i);
          if (!lv4_is(arg, "kind", "borrowed_text")) continue;
          lv4_string_operand_emit(f, yyjson_obj_get(arg, "operand"), fi, b, (unsigned)i);
          fprintf(f, "  %%textbits%u_%zu = ptrtoint ptr %%strarg%u_%zu to i64\n", b,i,b,i);
        }
        lv4_emit_nullable_actual_tags(f, args, b);
        if (lv4_is(op, "result", "unit"))
          fprintf(f, "  %%status%u = call i32 @hako_lifecycle_ordinary_%u(ptr %%frame",
              b, lv4_u(call, "target"));
        else if (lv4_is(op, "result", "map"))
          fprintf(f, "  %%status%u = call i32 @hako_lifecycle_ordinary_%u(ptr %%frame, ptr %%map%u",
              b, lv4_u(call, "target"), b);
        else if (lv4_is(op, "result", "text"))
          fprintf(f, "  %%status%u = call i32 @hako_lifecycle_ordinary_%u(ptr %%frame, ptr %%strout%u",
              b, lv4_u(call, "target"), b);
        else if (ordinary || birth)
          fprintf(f, "  %%status%u = call i32 @hako_lifecycle_ordinary_%u(ptr %%frame, ptr %%call_out%u",
              b, lv4_u(call, "target"), b);
        else
          fprintf(f, "  %%status%u = call i32 @hako_lifecycle_ordinary_%u(ptr %%frame, ptr %%ordinary_out",
              b, lv4_u(call, "target"));
        if (hako_physical_object_result_call(op)) fprintf(f, ", ptr %%call_outcome%u", b);
        if (yyjson_is_uint(yyjson_obj_get(call, "receiver")))
          fprintf(f, ", i64 %%v%u", lv4_u(call, "receiver"));
        for (size_t i = 0; i < yyjson_arr_size(args); i++) {
          yyjson_val* arg = yyjson_arr_get(args, i);
          if (lv4_emit_pair_actual(f, arg, b, i)) continue;
          /* A "map" actual is the caller's borrowed storage pointer; the
             callee's "map" formal receives it as `ptr`. */
          fprintf(f, lv4_is(arg, "kind", "map") ? ", ptr %%v%u" : ", i64 %%v%u",
              lv4_u(arg, "value"));
        }
        fprintf(f, ")\n");
      } else {
      /* `home_release_if_live` discharges the lease slot without touching
         the kernel when the wire value is the Void sentinel (i64 0); only a
         live handle reaches the release call, and its status still drives
         the normal/fault landing. */
      if (lv4_is(op, "kind", "object_end_if_owned")) {
        char outcome[64];
        snprintf(outcome, sizeof(outcome), "end_outcome%u", b);
        if (!lv4_emit_object_value_outcome(f, in, fn, lv4_u(op, "value"), outcome)) return 0;
        fprintf(f,
            "  %%end_owned%u = icmp eq i32 %%end_outcome%u, 1\n"
            "  br i1 %%end_owned%u, label %%complete_end%u, label %%%s\ncomplete_end%u:\n"
            "  %%status%u = call i32 @hako_lifecycle_end_%u(ptr %%frame, i64 %%v%u, i64 %llu)\n"
            "  switch i32 %%status%u, label %%invalid [ i32 0, label %%%s i32 1, label %%%s ]\n",
            b,b, b,b,normal_label,b, b,lv4_u(op, "object_id"),lv4_u(op, "value"),site,
            b,normal_label,fault_label);
        continue;
      }
      if (lv4_is(op, "kind", "home_release_if_live")) {
        hako_lv4_function_index* index = &in->index[fi];
        hako_lv4_value* value = lv4_index_value(index, lv4_u(op, "value"));
        if (lv4_birth_array_value(index, value) || lv4_script_arguments_root(index, lv4_u(op, "value"))) {
          /* Existing intrinsic Array leases keep their separate transport;
             this shared release opcode does not turn them into objects. */
          fprintf(f, "  %%live%u = icmp ne i64 %%v%u, 0\n", b, lv4_u(op, "value"));
        } else {
          char outcome[64];
          snprintf(outcome, sizeof(outcome), "release_outcome%u", b);
          if (!lv4_emit_object_value_outcome(f, in, fn, lv4_u(op, "value"), outcome)) return 0;
          fprintf(f, "  %%live%u = icmp eq i32 %%release_outcome%u, 1\n", b, b);
        }
        fprintf(f, "  br i1 %%live%u, label %%checked_rel%u, label %%%s\nchecked_rel%u:\n",
            b, b, normal_label, b);
      }
      if (lv4_is(op, "kind", "field_residence_release")) {
        /* Owned ArrayBox field residence: read the zero-initialized slot,
           release the child only when it carries a live handle, then land
           on the normal edge. A failed read is contract violation, never
           a fault edge. */
        fprintf(f,
            "  %%fr_get%u = call i32 @\"nyash.object.checked_field_get_i64_v1\"(i32 %u, i64 %%v%u, i64 %lld, i64 %d, ptr %%fr_read%u)\n"
            "  %%fr_ok%u = icmp eq i32 %%fr_get%u, 0\n"
            "  br i1 %%fr_ok%u, label %%fr_step%u, label %%invalid\nfr_step%u:\n",
            b, in->profile, lv4_u(op, "base"), (long long)lv4_type_id(in, op),
            lv4_slot(in, op), b, b, b, b, b, b);
        fprintf(f,
            "  %%fr_val%u = load i64, ptr %%fr_read%u, align 8\n"
            "  %%fr_live%u = icmp ne i64 %%fr_val%u, 0\n"
            "  br i1 %%fr_live%u, label %%fr_rel%u, label %%fr_done%u\nfr_rel%u:\n"
            "  call void @nyrt_handle_release_h(i64 %%fr_val%u)\n"
            "  br label %%fr_done%u\nfr_done%u:\n"
            "  br label %%%s\n",
            b, b, b, b, b, b, b, b, b, b, b, normal_label);
        continue;
      }
      if (lv4_is(op, "kind", "object_field_release")) {
        /* The admitted child End owns contents and outer Home together.
           Root home_release remains outer-only after explicit field cleanup. */
        char base[32], out[32];
        snprintf(base, sizeof(base), "%%v%u", lv4_u(op, "base"));
        snprintf(out, sizeof(out), "ofr_read%u", b);
        if (!lv4_emit_object_residence_read(f, in, lv4_u(op, "object_id"),
            lv4_u(op, "field_ordinal"), base, out, "invalid")) return 0;
        fprintf(f,
            "  %%ofr_val%u = load i64, ptr %%ofr_read%u, align 8\n"
            "  %%ofr_live%u = icmp eq i32 %%ofr_read%u_outcome, 1\n"
            "  br i1 %%ofr_live%u, label %%ofr_rel%u, label %%ofr_done%u\nofr_rel%u:\n",
            b, b, b, b, b, b, b, b);
        fprintf(f,
            "  %%ofr_status%u = call i32 @hako_lifecycle_end_%u(ptr %%frame, i64 %%ofr_val%u, i64 %llu)\n"
            "  switch i32 %%ofr_status%u, label %%invalid [ i32 0, label %%ofr_done%u i32 1, label %%%s ]\nofr_done%u:\n"
            "  br label %%%s\n",
            b, lv4_u(op, "child_object_id"), b, site,
            b, b, fault_label, b, normal_label);
        continue;
      }
      if (lv4_is(op, "kind", "birth_call")) {
        yyjson_val* call = yyjson_obj_get(op, "call");
        yyjson_val* args = yyjson_obj_get(call, "args");
        lv4_emit_nullable_actual_tags(f, args, b);
        yyjson_val* callee = yyjson_arr_get(in->functions, lv4_u(call, "target"));
        yyjson_val* formals = yyjson_obj_get(callee, "params");
        for (size_t i = 0; i < yyjson_arr_size(args); i++) {
          if (!lv4_is(yyjson_arr_get(formals, i), "representation", "nullable_kind_payload_v1")) continue;
          char outcome[64];
          snprintf(outcome, sizeof(outcome), "birth_outcome%u_%zu", b, i);
          if (!lv4_emit_object_value_outcome(f, in, fn, lv4_u(yyjson_arr_get(args, i), "value"), outcome)) return 0;
        }
        /* A dominated `new`-argument view spells "tagged": the caller
           proves the lent Integer lane (kind==1) at this exact call edge —
           any other kind is a semantic Fault at the use site, uniform with
           the checked-compare and dominated-`+` lanes — and the callee
           still sees the proven pair through its own prologue. */
        for (size_t i = 0; i < yyjson_arr_size(args); i++) {
          yyjson_val* arg = yyjson_arr_get(args, i);
          uint32_t value = lv4_u(arg, "value");
          if (lv4_is(arg, "kind", "tagged"))
            fprintf(f, "  %%bok%u_%u_%zu = icmp eq i32 %%k%u, 1\n"
                "  br i1 %%bok%u_%u_%zu, label %%bs%u_%u_%zu, label %%fault\nbs%u_%u_%zu:\n",
                value, b, i, value, value, b, i, value, b, i, value, b, i);
        }
      }
      if (lv4_is(op, "kind", "object_field_set") &&
          lv4_object_residence(in, lv4_u(op, "object_id"), lv4_u(op, "field_ordinal")) &&
          !lv4_emit_object_residence_outcome(f, in, fn, op, b)) return 0;
      fprintf(f, "  %%status%u = call i32 ", b);
      if (lv4_owned_array_kind(op)) {
        lv4_owned_array_emit_call(f, op, b, site);
      } else if (lv4_is(op, "kind", "script_arguments_new")) {
        fprintf(f, "@\"nyash.array.checked_script_arguments_new_v1\"(ptr %%frame, i64 %llu, ptr @argv_%zu_%u, i64 %zu, ptr %%new%u)\n",
            site, fi, b, yyjson_arr_size(yyjson_obj_get(op, "values")), b);
      } else if (lv4_is(op, "kind", "array_new")) {
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
          if (!lv4_emit_pair_actual(f, arg, b, i)) return 0;
          yyjson_val* callee = yyjson_arr_get(in->functions, lv4_u(call, "target"));
          if (lv4_is(yyjson_arr_get(yyjson_obj_get(callee, "params"), i),
              "representation", "nullable_kind_payload_v1"))
            fprintf(f, ", i32 %%birth_outcome%u_%zu", b, i);
        }
        fprintf(f, ")\n");
      } else if (lv4_is(op, "kind", "new_box")) {
        fprintf(f, "@\"nyash.object.checked_new_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 %lld, ptr @hako_lifecycle_layout_%u, i64 %u, ptr %%new%u)\n",
            in->profile, site, (long long)lv4_type_id(in, op), lv4_u(op, "object_id"),
            lv4_u(lv4_layout(in, lv4_u(op, "object_id")), "field_count"), b);
      } else if (lv4_is(op, "kind", "field_get")) {
        fprintf(f, "@\"nyash.object.checked_field_get_fallible_i64_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 %%v%u, i64 %lld, i64 %d, ptr %%field_read%u)\n",
            in->profile, site, lv4_u(op, "base"), (long long)lv4_type_id(in, op), lv4_slot(in, op), b);
      } else if (lv4_is(op, "kind", "field_set")) {
        fprintf(f, "@\"nyash.object.checked_field_set_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 %%v%u, i64 %lld, i64 %d, i64 %%v%u)\n",
            in->profile, site, lv4_u(op, "base"), (long long)lv4_type_id(in, op), lv4_slot(in, op), lv4_u(op, "value"));
        /* A handle-valued store consumes the lease on both edges: the
           kernel only validates and writes the slot, so a faulted store
           leaves the in-flight residence owned by this frame — discharge
           it here rather than dropping it silently. */
        if (lv4_input_type(in, fn, lv4_u(op, "value")) == LV4_HANDLE) {
          fprintf(f,
              "  switch i32 %%status%u, label %%invalid [ i32 0, label %%%s i32 1, label %%fs_fault%u ]\n"
              "fs_fault%u:\n  call void @nyrt_handle_release_h(i64 %%v%u)\n  br label %%%s\n",
              b, normal_label, b, b, lv4_u(op, "value"), fault_label);
          continue;
        }
      } else if (lv4_is(op, "kind", "object_field_set")) {
        if (lv4_object_residence(in, lv4_u(op, "object_id"), lv4_u(op, "field_ordinal"))) {
          if (!lv4_emit_object_residence_store(f, in, op, b, site)) return 0;
        } else {
        fprintf(f, "@\"nyash.object.checked_field_set_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 %%v%u, i64 %lld, i64 %d, i64 %%v%u)\n",
            in->profile, site, lv4_u(op, "base"), (long long)lv4_type_id(in, op), lv4_slot(in, op), lv4_u(op, "value"));
        }
        /* The child lease moves into the slot only on Normal; a faulted
           store lands on the op's own fault edge where the MIR discharge
           chain — sealed residence releases newest-first, then the
           child's Home — is the sole physical owner of the in-flight
           cleanup. No inline discharge here. */
      } else {
        fprintf(f, "@\"nyash.object.%s_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 %%v%u, i64 %lld)\n",
            lv4_is(op, "kind", "home_release") || lv4_is(op, "kind", "home_release_if_live")
                ? "home_release_plain_i64" : "reclaim_unpublished",
            in->profile, site, lv4_u(op, "value"), (long long)lv4_type_id(in, op));
      }
      }
      fprintf(f, "  switch i32 %%status%u, label %%invalid [ i32 0, label %%%s i32 1, label %%%s ]\n",
          b, normal_label, fault_label);
    }
  }
  for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    yyjson_val* block = yyjson_arr_get(blocks, bi);
    yyjson_val* edges = yyjson_obj_get(block, "edges");
    for (size_t ei = 0; ei < yyjson_arr_size(edges); ei++) {
      uint32_t target = lv4_u(yyjson_arr_get(edges, ei), "target");
      char label[64];
      if (lv4_edge_label(label, in, fi, lv4_u(block, "id"), ei, target))
        fprintf(f, "%s:\n  br label %%bb%u\n", label, target);
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
  lv4_string_globals(f, in);
  lv4_script_arguments_globals(f, in);
  /* Plain Array operations share one external ABI in both physical profiles. */
  fprintf(f,
      "declare i32 @\"nyash.array.checked_new_v1\"(ptr, i64, ptr)\n"
      "declare i32 @\"nyash.array.checked_claim_v1\"(ptr, i64, i64, i32)\n"
      "declare i32 @\"nyash.array.checked_append_i64_v1\"(ptr, i64, i64, i64)\n"
      "declare i32 @\"nyash.array.checked_append_bool_v1\"(ptr, i64, i64, i32)\n"
      "declare i32 @\"nyash.array.checked_append_f64_v1\"(ptr, i64, i64, double)\n"
      "declare i32 @\"nyash.array.checked_set_i64_v1\"(ptr, i64, i64, i64, i64)\n"
      "declare i64 @\"nyash.array.slot_load_hi\"(i64, i64)\n"
      "declare i64 @\"nyash.array.slot_len_h\"(i64)\n"
      "declare void @nyrt_handle_release_h(i64)\n");
  if (in->native) return lv4_emit_function(f, in, abi, 0);
  fprintf(f,
      "declare i32 @\"nyash.object.checked_new_v1\"(ptr, i32, i64, i64, ptr, i64, ptr)\n"
      "declare i64 @\"nyash.object.type_id_h\"(i64)\n"
      "declare i32 @\"nyash.object.checked_field_set_v1\"(ptr, i32, i64, i64, i64, i64, i64)\n"
      "declare i32 @\"nyash.object.checked_field_get_i64_v1\"(i32, i64, i64, i64, ptr)\n"
      "declare i32 @\"nyash.object.checked_field_get_fallible_i64_v1\"(ptr, i32, i64, i64, i64, i64, ptr)\n"
      "declare i32 @\"nyash.object.checked_residence_set_v1\"(ptr, i32, i64, i64, i64, i64, i64, i64, i32)\n"
      "declare i32 @\"nyash.object.checked_residence_get_v1\"(i32, i64, i64, i64, i64, ptr)\n"
      "declare i32 @\"nyash.object.home_release_plain_i64_v1\"(ptr, i32, i64, i64, i64)\n"
      "declare i32 @\"nyash.object.reclaim_unpublished_v1\"(ptr, i32, i64, i64, i64)\n");
  lv4_map_declarations(f, in);
  lv4_owned_array_globals(f, in);
  for (size_t li = 0; li < in->layout_count; li++) {
    yyjson_val* layout = in->layouts[li].row;
    uint32_t count = lv4_u(layout, "field_count");
    fprintf(f, "@hako_lifecycle_layout_%u = private constant [%u x i32] [", in->layouts[li].id, count);
    for (uint32_t i = 0; i < count; i++) fprintf(f, "%si32 %u", i ? ", " : "", HAKO_LLVMC_LIFECYCLE_STORAGE_I64);
    fprintf(f, "]\n");
  }
  /* Complete End symbols cover plain and marked admitted layouts. Recursive
     child calls resolve in this module; no runtime class inference is used. */
  for (size_t li = 0; li < in->layout_count; li++)
    if (!lv4_emit_end(f, in, in->layouts[li].id, in->layouts[li].row))
      return 0;
  for (size_t fi = 1; fi < in->index_count; fi++)
    if (!lv4_emit_function(f, in, abi, fi)) return 0;
  return lv4_emit_function(f, in, abi, 0);
}
