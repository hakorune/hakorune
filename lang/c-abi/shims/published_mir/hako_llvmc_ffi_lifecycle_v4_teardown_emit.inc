/* Source-admitted complete End helpers in the lifecycle-v4 emitter. */
/* Complete End for an admitted source layout: reverse marked contents, then
 * outer Home. Children call this same owner. Fault (1) retains the frame's
 * primary and continues cleanup; InvalidContract (2) is unrecoverable. Layout
 * metadata does not itself grant a caller destruction permission. */
static int lv4_emit_end(FILE* f, const hako_lv4_input* in, uint32_t id,
    yyjson_val* layout) {
  yyjson_val* arrays = yyjson_obj_get(layout, "owned_residences");
  yyjson_val* objects = yyjson_obj_get(layout, "owned_object_residences");
  size_t na = yyjson_arr_size(arrays), no = yyjson_arr_size(objects);
  long long type = (long long)lv4_u(layout, "runtime_type_id");
  fprintf(f,
      "define internal i32 @hako_lifecycle_end_%u(ptr %%frame, i64 %%handle, i64 %%site) {\nstart:\n",
      id);
  fprintf(f, "  %%td_status = alloca i32, align 4\n  store i32 0, ptr %%td_status, align 4\n");
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
    fprintf(f,
        "  %%td_call%zu = call i32 @hako_lifecycle_end_%u(ptr %%frame, i64 %%td_val%zu, i64 %%site)\n"
        "  switch i32 %%td_call%zu, label %%bad [ i32 0, label %%td_done%zu i32 1, label %%td_fault%zu ]\ntd_fault%zu:\n"
        "  store i32 1, ptr %%td_status, align 4\n"
        "  br label %%td_done%zu\ntd_done%zu:\n",
        mi, child_id, mi, mi, mi, mi, mi, mi, mi);
  }
  fprintf(f,
      "  %%td_outer = call i32 @\"nyash.object.home_release_plain_i64_v1\"(ptr %%frame, i32 %u, i64 %%site, i64 %%handle, i64 %lld)\n"
      "  switch i32 %%td_outer, label %%bad [ i32 0, label %%finish i32 1, label %%finish ]\nfinish:\n"
      "  %%td_prior = load i32, ptr %%td_status, align 4\n"
      "  %%td_result = or i32 %%td_prior, %%td_outer\n"
      "  ret i32 %%td_result\nbad:\n  ret i32 2\n}\n",
      in->profile, type);
  return !ferror(f);
}
