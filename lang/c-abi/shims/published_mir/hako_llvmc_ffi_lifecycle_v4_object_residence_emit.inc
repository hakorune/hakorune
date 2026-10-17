/* Storage for the same admitted owning-field tuple. This projection issues
 * no source capability: Birth/flow already checked the original field edge.
 * Null/Fresh is the existing cohort; Forward remains unadmitted here. */
static yyjson_val* lv4_object_residence(const hako_lv4_input* in,
    uint32_t object, uint32_t ordinal) {
  yyjson_val* rows = yyjson_obj_get(lv4_layout(in, object), "owned_object_residences");
  for (size_t i = 0; i < yyjson_arr_size(rows); i++) {
    yyjson_val* row = yyjson_arr_get(rows, i);
    if (lv4_u(row, "field_ordinal") == ordinal) return row;
  }
  return NULL;
}

/* ObjectResidenceV1 is {i64 payload, i32 outcome, i32 reserved}. Every selected
 * reader uses this owner; an uninitialized legacy slot never retries a read. */
static int lv4_emit_object_residence_read(FILE* f, const hako_lv4_input* in,
    uint32_t object, uint32_t ordinal, const char* base, const char* out,
    const char* invalid) {
  yyjson_val* layout = lv4_layout(in, object);
  yyjson_val* row = lv4_object_residence(in, object, ordinal);
  yyjson_val* child = row ? lv4_layout(in, lv4_u(row, "child_object_id")) : NULL;
  int slot = lv4_layout_slot(layout, ordinal);
  if (!row || !child || slot < 0) return 0;
  fprintf(f,
      "  %%%s_status = call i32 @\"nyash.object.checked_residence_get_v1\"(i32 %u, i64 %s, i64 %lld, i64 %d, i64 %lld, ptr %%%s)\n"
      "  %%%s_ok = icmp eq i32 %%%s_status, 0\n"
      "  br i1 %%%s_ok, label %%%s_pair, label %%%s\n%s_pair:\n"
      "  %%%s_tagptr = getelementptr { i64, i32, i32 }, ptr %%%s, i32 0, i32 1\n"
      "  %%%s_outcome = load i32, ptr %%%s_tagptr, align 4\n"
      "  %%%s_admitted = icmp ule i32 %%%s_outcome, 2\n"
      "  br i1 %%%s_admitted, label %%%s_done, label %%%s\n%s_done:\n",
      out, in->profile, base, (long long)lv4_u(layout, "runtime_type_id"), slot,
      (long long)lv4_u(child, "runtime_type_id"), out,
      out, out, out, out, invalid, out,
      out, out, out, out, out, out, out, out, invalid, out);
  return !ferror(f);
}

/* Keep the type tag and the residence outcome separate. Current incoming
 * admission still supplies only Null/Fresh; Forward remains closed. */
static void lv4_emit_object_residence_parameter_guard(FILE* f, uint32_t v) {
  fprintf(f,
      "  %%arg_null%u = icmp eq i32 %%k%u, 0\n"
      "  %%arg_zero%u = icmp eq i64 %%v%u, 0\n"
      "  %%arg_nulok%u = and i1 %%arg_null%u, %%arg_zero%u\n"
      "  %%arg_obj%u = icmp eq i32 %%k%u, 3\n"
      "  %%arg_live%u = icmp ne i64 %%v%u, 0\n"
      "  %%arg_objok%u = and i1 %%arg_obj%u, %%arg_live%u\n"
      "  %%arg_outzero%u = icmp eq i32 %%o%u, 0\n"
      "  %%arg_nullout%u = and i1 %%arg_nulok%u, %%arg_outzero%u\n"
      "  %%arg_outnonzero%u = icmp ne i32 %%o%u, 0\n"
      "  %%arg_outdomain%u = icmp ule i32 %%o%u, 2\n"
      "  %%arg_objout%u = and i1 %%arg_outnonzero%u, %%arg_outdomain%u\n"
      "  %%arg_fullobj%u = and i1 %%arg_objok%u, %%arg_objout%u\n"
      "  %%arg_ok%u = or i1 %%arg_nullout%u, %%arg_fullobj%u\n"
      "  br i1 %%arg_ok%u, label %%arg_next%u, label %%invalid\narg_next%u:\n",
      v,v, v,v, v,v,v, v,v, v,v, v,v,v, v,v, v,v,v, v,v, v,v, v,v,v, v,v,v, v,v,v, v,v,v);
}

/* The caller emits this before its status-call prefix. Provider Fresh keeps
 * its existing law; a Provided field uses the same formal's supplied outcome. */
static int lv4_emit_object_residence_outcome(FILE* f, const hako_lv4_input* in,
    yyjson_val* fn, yyjson_val* op, uint32_t b) {
  char outcome[64];
  snprintf(outcome, sizeof(outcome), "residence_outcome%u", b);
  return lv4_emit_object_value_outcome(f, in, fn, lv4_u(op, "value"), outcome);
}

static int lv4_emit_object_residence_store(FILE* f, const hako_lv4_input* in,
    yyjson_val* op, uint32_t b, unsigned long long site) {
  yyjson_val* row = lv4_object_residence(in, lv4_u(op, "object_id"), lv4_u(op, "field_ordinal"));
  if (!row || lv4_u(row, "child_object_id") != lv4_u(op, "child_object_id")) return 0;
  yyjson_val* child = lv4_layout(in, lv4_u(row, "child_object_id"));
  if (!child) return 0;
  fprintf(f,
      "@\"nyash.object.checked_residence_set_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 %%v%u, i64 %lld, i64 %d, i64 %lld, i64 %%v%u, i32 %%residence_outcome%u)\n",
      in->profile, site, lv4_u(op, "base"), (long long)lv4_type_id(in, op),
      lv4_slot(in, op), (long long)lv4_u(child, "runtime_type_id"), lv4_u(op, "value"), b);
  return !ferror(f);
}
