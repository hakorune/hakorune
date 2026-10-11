/* Marked-field teardown helpers within the existing lifecycle-v4 emitter. */
/* A layout carries a teardown inventory when either mark list is
 * nonempty: `owned_residences` array slots or `owned_object_residences`
 * (field,child) rows — the sealed census rows the physical ABI pins. */
static int lv4_layout_marked(yyjson_val* layout) {
  return layout &&
      (yyjson_arr_size(yyjson_obj_get(layout, "owned_residences")) > 0 ||
       yyjson_arr_size(yyjson_obj_get(layout, "owned_object_residences")) > 0);
}

/* The generated per-class teardown helper — the sole physical owner of
 * an object's marked-field contents. `object_field_release` calls it on
 * the live child, and each object mark inside it calls the child's own
 * helper before that child's plain Home release, so nested depth and
 * self-reference recurse through the emitted call graph on live
 * instances — never through a census-time unroll. `%site` carries the
 * calling op's diagnostic site so a nested fault attributes to the
 * outermost release. Status contract: 0 contents released, nonzero is
 * the kernel status (1 fault edge, 2 contract violation). */
static int lv4_emit_teardown(FILE* f, const hako_lv4_input* in, uint32_t id,
    yyjson_val* layout) {
  yyjson_val* arrays = yyjson_obj_get(layout, "owned_residences");
  yyjson_val* objects = yyjson_obj_get(layout, "owned_object_residences");
  size_t na = yyjson_arr_size(arrays), no = yyjson_arr_size(objects);
  long long type = (long long)lv4_u(layout, "runtime_type_id");
  fprintf(f,
      "define internal i32 @hako_lifecycle_teardown_%u(ptr %%frame, i64 %%handle, i64 %%site) {\nstart:\n",
      id);
  for (size_t i = 0; i < na + no; i++)
    fprintf(f, "  %%td_read%zu = alloca i64, align 8\n", i);
  /* Newest-first across both mark lists: each walks declaration order,
     so the merge picks the larger pending ordinal — the same reverse
     declaration order `end_plan` emits. */
  for (size_t ai = na, oi = no, mi = 0; ai > 0 || oi > 0; mi++) {
    int array_mark = ai > 0 &&
        (oi == 0 || (uint32_t)yyjson_get_uint(yyjson_arr_get(arrays, ai - 1)) >
            lv4_u(yyjson_arr_get(objects, oi - 1), "field_ordinal"));
    if (array_mark) {
      ai--;
      int slot = lv4_layout_slot(layout,
          (uint32_t)yyjson_get_uint(yyjson_arr_get(arrays, ai)));
      if (slot < 0) return 0;
      fprintf(f,
          "  %%td_get%zu = call i32 @\"nyash.object.checked_field_get_i64_v1\"(i32 %u, i64 %%handle, i64 %lld, i64 %d, ptr %%td_read%zu)\n"
          "  %%td_ok%zu = icmp eq i32 %%td_get%zu, 0\n"
          "  br i1 %%td_ok%zu, label %%td_step%zu, label %%bad\ntd_step%zu:\n",
          mi, in->profile, type, slot, mi, mi, mi, mi, mi, mi);
      fprintf(f,
          "  %%td_val%zu = load i64, ptr %%td_read%zu, align 8\n"
          "  %%td_live%zu = icmp ne i64 %%td_val%zu, 0\n"
          "  br i1 %%td_live%zu, label %%td_rel%zu, label %%td_done%zu\ntd_rel%zu:\n"
          "  call void @nyrt_handle_release_h(i64 %%td_val%zu)\n"
          "  br label %%td_done%zu\ntd_done%zu:\n",
          mi, mi, mi, mi, mi, mi, mi, mi, mi, mi, mi);
      continue;
    }
    oi--;
    yyjson_val* row = yyjson_arr_get(objects, oi);
    uint32_t ordinal = lv4_u(row, "field_ordinal");
    uint32_t child_id = lv4_u(row, "child_object_id");
    yyjson_val* child = lv4_layout(in, child_id);
    int slot = lv4_layout_slot(layout, ordinal);
    if (!child || slot < 0) return 0;
    fprintf(f,
        "  %%td_get%zu = call i32 @\"nyash.object.checked_field_get_i64_v1\"(i32 %u, i64 %%handle, i64 %lld, i64 %d, ptr %%td_read%zu)\n"
        "  %%td_ok%zu = icmp eq i32 %%td_get%zu, 0\n"
        "  br i1 %%td_ok%zu, label %%td_step%zu, label %%bad\ntd_step%zu:\n",
        mi, in->profile, type, slot, mi, mi, mi, mi, mi, mi);
    fprintf(f,
        "  %%td_val%zu = load i64, ptr %%td_read%zu, align 8\n"
        "  %%td_live%zu = icmp ne i64 %%td_val%zu, 0\n"
        "  br i1 %%td_live%zu, label %%td_chain%zu, label %%td_done%zu\ntd_chain%zu:\n",
        mi, mi, mi, mi, mi, mi, mi, mi);
    if (lv4_layout_marked(child))
      fprintf(f,
          "  %%td_call%zu = call i32 @hako_lifecycle_teardown_%u(ptr %%frame, i64 %%td_val%zu, i64 %%site)\n"
          "  %%td_cok%zu = icmp eq i32 %%td_call%zu, 0\n"
          "  br i1 %%td_cok%zu, label %%td_home%zu, label %%td_prop%zu\ntd_prop%zu:\n"
          "  ret i32 %%td_call%zu\ntd_home%zu:\n",
          mi, child_id, mi, mi, mi, mi, mi, mi, mi, mi, mi);
    else
      fprintf(f, "  br label %%td_home%zu\ntd_home%zu:\n", mi, mi);
    fprintf(f,
        "  %%td_rel%zu = call i32 @\"nyash.object.home_release_plain_i64_v1\"(ptr %%frame, i32 %u, i64 %%site, i64 %%td_val%zu, i64 %lld)\n"
        "  %%td_rok%zu = icmp eq i32 %%td_rel%zu, 0\n"
        "  br i1 %%td_rok%zu, label %%td_done%zu, label %%td_prop2%zu\ntd_prop2%zu:\n"
        "  ret i32 %%td_rel%zu\ntd_done%zu:\n",
        mi, in->profile, mi,
        (long long)lv4_u(child, "runtime_type_id"),
        mi, mi, mi, mi, mi, mi, mi, mi);
  }
  fprintf(f, "  ret i32 0\nbad:\n  ret i32 2\n}\n");
  return !ferror(f);
}
