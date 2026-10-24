/* One physical Array projection: scalar/Text copies and owned-slot moves.
 * Child End uses the complete source-layout thunk, never a registry. */
static void lv4_owned_array_globals(FILE* f, const hako_lv4_input* in) {
  fprintf(f,
      "declare i32 @\"nyash.array.checked_owned_claim_v1\"(ptr, i64, i64, ptr)\n"
      "declare i32 @\"nyash.array.checked_owned_append_v1\"(ptr, i64, i64, ptr, i64)\n"
      "declare i32 @\"nyash.array.checked_owned_get_v1\"(ptr, i64, i64, ptr, i64, ptr)\n"
      "declare i32 @\"nyash.array.checked_owned_end_v1\"(ptr, i64, i64, ptr)\n"
      "declare i32 @\"nyash.array.checked_carrier_claim_v1\"(ptr, i64, i64, ptr)\n"
      "declare i32 @\"nyash.array.checked_carrier_append_v1\"(ptr, i64, i64, ptr, i64)\n"
      "declare i32 @\"nyash.array.checked_carrier_end_v1\"(ptr, i64, i64, ptr)\n");
  for (size_t li = 0; li < in->layout_count; li++) {
    uint32_t object = in->layouts[li].id;
    for (unsigned nullable = 0; nullable < 2; nullable++) {
      int needed = 0;
      for (size_t fi = 0; fi < in->index_count && !needed; fi++) {
        hako_lv4_function_index* fn = &in->index[fi];
        for (size_t bi = 0; bi < fn->block_count; bi++) {
          yyjson_val* op = yyjson_obj_get(lv4_term(fn->blocks[bi].block), "operation");
          if (lv4_owned_array_kind(op) && lv4_u(op, "object_id") == object &&
              (unsigned)yyjson_get_bool(yyjson_obj_get(op, "nullable")) == nullable) {
            needed = 1; break;
          }
        }
      }
      if (needed) fprintf(f,
          "@hako_array_owned_%u_%u = private constant { i32, i32, i64, ptr } "
          "{ i32 %u, i32 %u, i64 %llu, ptr @hako_lifecycle_end_%u }, align 8\n",
          object, nullable, in->profile, nullable,
          (unsigned long long)yyjson_get_uint(yyjson_obj_get(in->layouts[li].row, "runtime_type_id")), object);
    }
  }
}

static void lv4_carrier_array_globals(FILE* f, const hako_lv4_input* in) {
  int needed = 0;
  for (size_t fi = 0; fi < in->index_count && !needed; fi++) {
    hako_lv4_function_index* fn = &in->index[fi];
    for (size_t bi = 0; bi < fn->block_count; bi++) {
      yyjson_val* op = yyjson_obj_get(lv4_term(fn->blocks[bi].block), "operation");
      const char* kind = yyjson_get_str(yyjson_obj_get(op, "kind"));
      if (kind && (!strcmp(kind, "array_carrier_claim") ||
          !strcmp(kind, "array_carrier_append") || !strcmp(kind, "array_carrier_end"))) {
        needed = 1;
        break;
      }
    }
  }
  if (needed) fprintf(f,
      "@hako_array_carrier_slots_v1 = private constant { i32, i32 } { i32 1, i32 0 }, align 4\n");
}

/* The caller already emitted the common status/result prefix and owns the
 * sole Normal/Fault switch, including InvalidContract's unrecoverable edge. */
static void lv4_owned_array_emit_call(FILE* f, yyjson_val* op,
    uint32_t block, unsigned long long site) {
  const char* kind = lv4_is(op, "kind", "array_owned_claim") ? "claim" :
      lv4_is(op, "kind", "array_owned_append") ? "append" :
      lv4_is(op, "kind", "array_owned_get") ? "get" : "end";
  fprintf(f, "@\"nyash.array.checked_owned_%s_v1\"(ptr %%frame, i64 %llu, i64 %%v%u, ptr @hako_array_owned_%u_%u",
      kind, site, lv4_u(op, "array"), lv4_u(op, "object_id"),
      yyjson_get_bool(yyjson_obj_get(op, "nullable")) ? 1u : 0u);
  if (lv4_is(op, "kind", "array_owned_append"))
    fprintf(f, ", i64 %%v%u", lv4_u(op, "value"));
  if (lv4_is(op, "kind", "array_owned_get"))
    fprintf(f, ", i64 %%v%u, ptr %%owned_get%u", lv4_u(op, "index"), block);
  fprintf(f, ")\n");
}

static void lv4_carrier_array_emit_call(FILE* f, yyjson_val* op,
    uint32_t block, unsigned long long site) {
  const char* kind = lv4_is(op, "kind", "array_carrier_claim") ? "claim" :
      lv4_is(op, "kind", "array_carrier_append") ? "append" : "end";
  fprintf(f, "@\"nyash.array.checked_carrier_%s_v1\"(ptr %%frame, i64 %llu, i64 %%v%u, ptr @hako_array_carrier_slots_v1",
      kind, site, lv4_u(op, "array"));
  if (lv4_is(op, "kind", "array_carrier_append"))
    fprintf(f, ", i64 %%v%u", lv4_u(op, "value"));
  fprintf(f, ")\n");
  (void)block;
}

/* The instruction schema already fixes numeric vs Text operands. */
static void lv4_array_primitive_emit_call(FILE* f, yyjson_val* op, uint32_t b, unsigned long long site) {
  if (lv4_is(op, "kind", "array_new")) {
    fprintf(f, "@\"nyash.array.checked_new_v1\"(ptr %%frame, i64 %llu, ptr %%new%u)\n", site, b);
  } else if (lv4_is(op, "kind", "array_claim")) {
    fprintf(f, "@\"nyash.array.checked_claim_v1\"(ptr %%frame, i64 %llu, i64 %%v%u, i32 %u)\n",
        site, lv4_u(op, "array"), lv4_u(op, "element_tag"));
  } else if (hako_physical_array_text_append(op)) {
    fprintf(f, "@\"nyash.array.checked_append_text_v1\"(ptr %%frame, i64 %llu, i64 %%v%u, ptr %%strarg%u_0)\n",
        site, lv4_u(op, "array"), b);
  } else {
    const char* repr = yyjson_get_str(yyjson_obj_get(op, "representation"));
    const char* lane = lv4_is(op, "representation", "f64") ? "double" :
        lv4_is(op, "representation", "bool") ? "i32" : "i64";
    int boolean = lv4_is(op, "representation", "bool");
    fprintf(f, "@\"nyash.array.checked_append_%s_v1\"(ptr %%frame, i64 %llu, i64 %%v%u, %s %%%s%u)\n",
        repr, site, lv4_u(op, "array"), lane, boolean ? "bool" : "v",
        boolean ? b : lv4_u(op, "value"));
  }
}
