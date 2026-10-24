static void lv4_object_layout_globals(FILE* f, const hako_lv4_input* in) {
  fprintf(f,
      "@hako_lifecycle_text_end_descriptor = private constant { i32, i32, i64, ptr } { i32 %u, i32 4, i64 0, ptr @hako_lifecycle_text_end_adapter }\n"
      "define internal i32 @hako_lifecycle_text_end_adapter(ptr %%frame, i64 %%site, ptr %%value) {\nstart:\n"
      "  %%payload = getelementptr { i32, i32, i64, i64 }, ptr %%value, i32 0, i32 2\n"
      "  %%handle = load i64, ptr %%payload, align 8\n"
      "  %%tokenptr = getelementptr { i32, i32, i64, i64 }, ptr %%value, i32 0, i32 3\n"
      "  %%token = load i64, ptr %%tokenptr, align 8\n"
      "  %%status = call i32 @\"nyash.string.checked_end_v1\"(ptr %%frame, i64 %%site, i64 %%handle, i64 %%token)\n"
      "  ret i32 %%status\n}\n", in->profile);
  fprintf(f,
      "@hako_lifecycle_array_end_descriptor = private constant { i32, i32, i64, ptr } { i32 %u, i32 6, i64 0, ptr @hako_lifecycle_array_end_adapter }\n"
      "define internal i32 @hako_lifecycle_array_end_adapter(ptr %%frame, i64 %%site, ptr %%value) {\nstart:\n"
      "  %%payload = getelementptr { i32, i32, i64, i64 }, ptr %%value, i32 0, i32 2\n"
      "  %%handle = load i64, ptr %%payload, align 8\n"
      "  call void @nyrt_handle_release_h(i64 %%handle)\n"
      "  ret i32 0\n}\n", in->profile);
  for (size_t li = 0; li < in->layout_count; li++) {
    yyjson_val* layout = in->layouts[li].row;
    uint32_t count = lv4_u(layout, "field_count");
    fprintf(f, "@hako_lifecycle_layout_%u = private constant [%u x i32] [", in->layouts[li].id, count);
    for (uint32_t i = 0; i < count; i++) fprintf(f, "%si32 %u", i ? ", " : "", lv4_u(yyjson_arr_get(yyjson_obj_get(layout, "fields"), i), "storage_kind"));
    fprintf(f, "]\n");
  }
}

static void lv4_dynamic_declarations(FILE* f) {
  fprintf(f,
      "declare i32 @\"nyash.object.home_release_completed_fields_v1\"(ptr, i32, i64, i64, i64)\n"
      "declare i32 @\"nyash.object.field_outcome_init_v1\"(ptr)\n"
      "declare i32 @\"nyash.object.checked_dynamic_replace_v1\"(ptr, i32, i64, i64, i64, i64, ptr, ptr, ptr)\n"
      "declare i32 @\"nyash.object.checked_dynamic_initialize_v1\"(ptr, i32, i64, i64, i64, i64, ptr, ptr, ptr)\n"
      "declare i32 @\"nyash.object.checked_map_field_install_v1\"(ptr, i32, i64, i64, i64, i64, ptr, ptr)\n"
      "declare i32 @\"nyash.object.checked_dynamic_take_v1\"(ptr, i32, i64, i64, i64, i64, ptr)\n"
      "declare i32 @\"nyash.object.checked_dynamic_get_v1\"(ptr, i32, i64, i64, i64, i64, i32, ptr)\n"
      "declare i32 @\"nyash.object.field_outcome_end_v1\"(ptr, i64, ptr)\n"
      "declare i32 @\"nyash.object.field_outcome_dispose_v1\"(ptr)\n");
}

/* Dynamic field storage is a physical requirement, never an owning value fact. */
static int lv4_layout_dynamic(yyjson_val* layout) {
  yyjson_val* fields = yyjson_obj_get(layout, "fields");
  for (size_t i = 0; i < yyjson_arr_size(fields); i++)
    if (lv4_u(yyjson_arr_get(fields, i), "storage_kind") == HAKO_LLVMC_LIFECYCLE_STORAGE_DYNAMIC) return 1;
  return 0;
}

static const hako_llvmc_object_field_abi_v1* lv4_field_abi(
    const hako_llvmc_lifecycle_target_session_v2* target) {
  if (!target || target->revision != 3u) return NULL;
  const hako_llvmc_object_field_abi_v1* field =
      &((const hako_llvmc_lifecycle_target_session_v3*)target)->field;
  return field->revision == 1u ? field : NULL;
}

static int lv4_field_outcome_alloca(FILE* f, const char* name,
    const hako_llvmc_object_field_abi_v1* abi, int candidate) {
  if (!abi || abi->revision != 1u) return 0;
  fprintf(f, "  %%%s_out = alloca [%u x i8], align %u\n",
      name, abi->outcome_size, abi->outcome_align);
  if (candidate) fprintf(f, "  %%%s_candidate = alloca { i32, i32, i64, i64 }, align %u\n",
      name, abi->value_align);
  return !ferror(f);
}

/* One prepare/commit/detached-End owner for replacement and complete slot drain.
 * A Map initializer moves its candidate only on Normal; scalar candidates have no Home.
 * Unissued disposal preserves the caller's old slot;
 * committed End Fault never rolls back. Both exits dispose before continuation. */
static int lv4_emit_dynamic_transaction(FILE* f, const char* name, uint32_t profile,
    const char* site, const char* base, long long type, int slot, const char* candidate, const char* candidate_token,
    int initialize, uint32_t residence_object, const char* candidate_kind,
    const char* normal, const char* fault) {
  if (slot < 0) return 0;
  if (candidate && initialize != 2) fprintf(f,
      "  store { i32, i32, i64, i64 } zeroinitializer, ptr %%%s_candidate, align 8\n"
      "  store i32 2, ptr %%%s_candidate, align 4\n"
      "  %%%s_payload = getelementptr { i32, i32, i64, i64 }, ptr %%%s_candidate, i32 0, i32 2\n"
      "  store i64 %s, ptr %%%s_payload, align 8\n",
      name, name, name, name, candidate, name);
  if (initialize == 5) fprintf(f, "  store i32 6, ptr %%%s_candidate, align 4\n", name);
  if (initialize == 6) {
    if (!candidate_kind || !residence_object) return 0;
    fprintf(f,
        "  %%%s_is_null = icmp eq i32 %s, 0\n"
        "  %%%s_kind = select i1 %%%s_is_null, i32 1, i32 5\n"
        "  store i32 %%%s_kind, ptr %%%s_candidate, align 4\n",
        name, candidate_kind, name, name, name, name);
  }
  if (initialize == 3) {
    if (!candidate || !candidate_token) return 0;
    fprintf(f, "  store i32 4, ptr %%%s_candidate, align 4\n"
        "  %%%s_token = getelementptr { i32, i32, i64, i64 }, ptr %%%s_candidate, i32 0, i32 3\n"
        "  store i64 %s, ptr %%%s_token, align 8\n",
        name, name, name, candidate_token, name);
  }
  fprintf(f,
      "  %%%s_init = call i32 @\"nyash.object.field_outcome_init_v1\"(ptr %%%s_out)\n"
      "  switch i32 %%%s_init, label %%invalid [ i32 0, label %%%s_prepare ]\n%s_prepare:\n",
      name, name, name, name, name);
  if (initialize == 2) fprintf(f,
      "  %%%s_commit = call i32 @\"nyash.object.checked_map_field_install_v1\"(ptr %%frame, i32 %u, i64 %s, i64 %s, i64 %lld, i64 %d, ptr %s, ptr %%%s_out)\n",
      name, profile, site, base, type, slot, candidate, name);
  else if (initialize == 3) fprintf(f,
      "  %%%s_commit = call i32 @\"nyash.object.checked_dynamic_initialize_v1\"(ptr %%frame, i32 %u, i64 %s, i64 %s, i64 %lld, i64 %d, ptr %%%s_candidate, ptr @hako_lifecycle_text_end_descriptor, ptr %%%s_out)\n",
      name, profile, site, base, type, slot, name, name);
  else if (initialize == 5) fprintf(f,
      "  %%%s_commit = call i32 @\"nyash.object.checked_dynamic_initialize_v1\"(ptr %%frame, i32 %u, i64 %s, i64 %s, i64 %lld, i64 %d, ptr %%%s_candidate, ptr @hako_lifecycle_array_end_descriptor, ptr %%%s_out)\n",
      name, profile, site, base, type, slot, name, name);
  else if (initialize == 6) fprintf(f,
      "  %%%s_commit = call i32 @\"nyash.object.checked_dynamic_initialize_v1\"(ptr %%frame, i32 %u, i64 %s, i64 %s, i64 %lld, i64 %d, ptr %%%s_candidate, ptr @hako_lifecycle_object_end_descriptor_%u, ptr %%%s_out)\n",
      name, profile, site, base, type, slot, name, residence_object, name);
  else if (candidate) fprintf(f,
      "  %%%s_commit = call i32 @\"nyash.object.%s_v1\"(ptr %%frame, i32 %u, i64 %s, i64 %s, i64 %lld, i64 %d, ptr %%%s_candidate, ptr null, ptr %%%s_out)\n",
      name, initialize ? "checked_dynamic_initialize" : "checked_dynamic_replace",
      profile, site, base, type, slot, name, name);
  else fprintf(f,
      "  %%%s_commit = call i32 @\"nyash.object.checked_dynamic_take_v1\"(ptr %%frame, i32 %u, i64 %s, i64 %s, i64 %lld, i64 %d, ptr %%%s_out)\n",
      name, profile, site, base, type, slot, name);
  fprintf(f,
      "  switch i32 %%%s_commit, label %%invalid [ i32 0, label %%%s_%s i32 1, label %%%s_unissued ]\n",
      name, name, initialize == 2 ? "moved" : "end", name);
  if (initialize == 2) fprintf(f,
      "%s_moved:\n  %%%s_source_dispose = call i32 @\"nyash.map.storage_dispose_v1\"(ptr %s)\n"
      "  switch i32 %%%s_source_dispose, label %%invalid [ i32 0, label %%%s_end ]\n",
      name, name, candidate, name, name);
  fprintf(f, "%s_unissued:\n"
      "  %%%s_cancel = call i32 @\"nyash.object.field_outcome_dispose_v1\"(ptr %%%s_out)\n"
      "  switch i32 %%%s_cancel, label %%invalid [ i32 0, label %%%s ]\n%s_end:\n"
      "  %%%s_ended = call i32 @\"nyash.object.field_outcome_end_v1\"(ptr %%frame, i64 %s, ptr %%%s_out)\n"
      "  switch i32 %%%s_ended, label %%invalid [ i32 0, label %%%s_dispose i32 1, label %%%s_dispose ]\n%s_dispose:\n"
      "  %%%s_disposed = call i32 @\"nyash.object.field_outcome_dispose_v1\"(ptr %%%s_out)\n"
      "  switch i32 %%%s_disposed, label %%invalid [ i32 0, label %%%s_finish ]\n%s_finish:\n"
      "  switch i32 %%%s_ended, label %%invalid [ i32 0, label %%%s i32 1, label %%%s ]\n",
      name, name, name, name, fault, name,
      name, site, name, name, name, name, name, name, name,
      name, name, name, name, normal,
      (initialize == 5 || initialize == 6) ? fault : (initialize ? "invalid" : fault));
  return !ferror(f);
}

/* Allocate transaction storage once in the function entry, including loop uses. */
static int lv4_dynamic_alloca(FILE* f, const hako_lv4_input* in, yyjson_val* op,
    uint32_t block, const hako_llvmc_lifecycle_target_session_v2* target) {
  if (!op || !(lv4_is(op, "kind", "field_get") || lv4_is(op, "kind", "field_set") ||
      lv4_is(op, "kind", "field_residence_release"))) return 1;
  yyjson_val* layout = lv4_layout(in, lv4_u(op, "object_id"));
  yyjson_val* field = yyjson_arr_get(yyjson_obj_get(layout, "fields"), lv4_u(op, "field_ordinal"));
  if (lv4_u(field, "storage_kind") != HAKO_LLVMC_LIFECYCLE_STORAGE_DYNAMIC) return 1;
  const hako_llvmc_object_field_abi_v1* abi = lv4_field_abi(target);
  if (!abi) return 0;
  if (lv4_is(op, "kind", "field_get")) {
    fprintf(f, "  %%dyn_read%u = alloca { i32, i32, i64, i64 }, align %u\n", block, abi->value_align);
    return !ferror(f);
  }
  char name[32]; snprintf(name, sizeof(name), "dyn%u", block);
  return lv4_field_outcome_alloca(f, name, abi, lv4_is(op, "kind", "field_set") &&
      !lv4_is(op, "transaction", "dynamic_map_initialize"));
}

/* -1 is another operation family, 0 is invalid, 1 emitted both existing edges. */
static int lv4_dynamic_emit(FILE* f, const hako_lv4_input* in, yyjson_val* op,
    uint32_t block, const char* normal, const char* fault) {
  if (!(lv4_is(op, "kind", "field_get") || lv4_is(op, "kind", "field_set") ||
      lv4_is(op, "kind", "field_residence_release"))) return -1;
  yyjson_val* layout = lv4_layout(in, lv4_u(op, "object_id"));
  yyjson_val* field = yyjson_arr_get(yyjson_obj_get(layout, "fields"), lv4_u(op, "field_ordinal"));
  if (lv4_u(field, "storage_kind") != HAKO_LLVMC_LIFECYCLE_STORAGE_DYNAMIC) return -1;
  if (lv4_slot(in, op) < 0) return 0;
  if (lv4_is(op, "kind", "field_get")) {
    fprintf(f,
        "  %%dyn_status%u = call i32 @\"nyash.object.checked_dynamic_get_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 %%v%u, i64 %lld, i64 %d, i32 2, ptr %%dyn_read%u)\n"
        "  switch i32 %%dyn_status%u, label %%invalid [ i32 0, label %%dyn_normal%u i32 1, label %%%s ]\ndyn_normal%u:\n"
        "  %%dyn_payload%u = getelementptr { i32, i32, i64, i64 }, ptr %%dyn_read%u, i32 0, i32 2\n"
        "  %%dyn_value%u = load i64, ptr %%dyn_payload%u, align 8\n"
        "  store i64 %%dyn_value%u, ptr %%field_read%u, align 8\n  br label %%%s\n",
        block, in->profile, (unsigned long long)yyjson_get_uint(yyjson_obj_get(op, "site")),
        lv4_u(op, "base"), (long long)lv4_type_id(in, op), lv4_slot(in, op), block,
        block, block, fault, block, block, block, block, block, block, block, normal);
    return !ferror(f);
  }
  int replace = lv4_is(op, "kind", "field_set");
  int initialize = lv4_is(op, "transaction", "dynamic_text_initialize") ? 3 :
      lv4_is(op, "transaction", "dynamic_map_initialize") ? 2 :
      lv4_is(op, "transaction", "dynamic_i64_initialize") ? 1 :
      (lv4_is(op, "transaction", "dynamic_value_initialize") &&
       lv4_is(op, "value_tag", "array") && lv4_is(op, "residence", "empty_array_home") &&
       yyjson_is_null(yyjson_obj_get(op, "residence_object_id"))) ? 5 :
      (lv4_is(op, "transaction", "dynamic_value_initialize") &&
       lv4_is(op, "value_tag", "nullable_object") && lv4_is(op, "residence", "object_home") &&
       yyjson_is_uint(yyjson_obj_get(op, "residence_object_id"))) ? 6 : 0;
  if (!(replace ? (initialize || lv4_is(op, "transaction", "dynamic_i64_replace")) :
      lv4_is(op, "transaction", "take_dynamic"))) return 0;
  char name[32], site[32], base[32], candidate[32], token[32];
  snprintf(name, sizeof(name), "dyn%u", block);
  snprintf(site, sizeof(site), "%llu", (unsigned long long)yyjson_get_uint(yyjson_obj_get(op, "site")));
  snprintf(base, sizeof(base), "%%v%u", lv4_u(op, "base"));
  snprintf(candidate, sizeof(candidate), "%%v%u", lv4_u(op, "value"));
  snprintf(token, sizeof(token), "%%t%u", lv4_u(op, "value"));
  uint32_t residence_object = 0;
  char candidate_kind[32];
  const char* candidate_kind_operand = NULL;
  if (initialize == 6) {
    if (!hako_physical_u32(yyjson_obj_get(op, "residence_object_id"), &residence_object) ||
        !lv4_layout(in, residence_object)) return 0;
    snprintf(candidate_kind, sizeof(candidate_kind), "%%k%u", lv4_u(op, "value"));
    candidate_kind_operand = candidate_kind;
  }
  return lv4_emit_dynamic_transaction(f, name, in->profile, site, base,
      (long long)lv4_type_id(in, op), lv4_slot(in, op), replace ? candidate : NULL,
      initialize == 3 ? token : NULL, initialize, residence_object,
      candidate_kind_operand, normal, fault);
}

/* Source-admitted complete End helpers in the lifecycle-v4 emitter. */
/* Complete End for an admitted source layout: reverse marked contents, then
 * outer Home. Children call this same owner. Fault (1) retains the frame's
 * primary and continues cleanup; InvalidContract (2) is unrecoverable. Layout
 * metadata does not itself grant a caller destruction permission. */
static int lv4_emit_end(FILE* f, const hako_lv4_input* in, uint32_t id,
    yyjson_val* layout, const hako_llvmc_object_field_abi_v1* abi) {
  yyjson_val* arrays = yyjson_obj_get(layout, "owned_residences");
  yyjson_val* objects = yyjson_obj_get(layout, "owned_object_residences");
  size_t na = yyjson_arr_size(arrays), no = yyjson_arr_size(objects);
  long long type = (long long)lv4_u(layout, "runtime_type_id");
  fprintf(f,
      "define internal i32 @hako_lifecycle_end_%u(ptr %%frame, i64 %%handle, i64 %%site) {\nstart:\n",
      id);
  fprintf(f, "  %%td_status = alloca i32, align 4\n  store i32 0, ptr %%td_status, align 4\n");
  for (size_t i = 0; i < na + no; i++)
    fprintf(f, "  %%td_read%zu = alloca { i64, i32, i32 }, align 8\n", i);
  yyjson_val* fields = yyjson_obj_get(layout, "fields");
  for (size_t ordinal = 0; ordinal < yyjson_arr_size(fields); ordinal++)
    if (lv4_u(yyjson_arr_get(fields, ordinal), "storage_kind") == HAKO_LLVMC_LIFECYCLE_STORAGE_DYNAMIC) {
      char name[40]; snprintf(name, sizeof(name), "td_dyn%zu", ordinal);
      if (!lv4_field_outcome_alloca(f, name, abi, 0)) return 0;
    }
  /* One reverse declaration walk across static marks and every Dynamic slot. */
  for (size_t at = yyjson_arr_size(fields), ai = na, oi = no, mi = 0; at > 0;) {
    uint32_t ordinal = (uint32_t)--at;
    int array_mark = ai > 0 && yyjson_get_uint(yyjson_arr_get(arrays, ai - 1)) == ordinal;
    int object_mark = oi > 0 && lv4_u(yyjson_arr_get(objects, oi - 1), "field_ordinal") == ordinal;
    if (lv4_u(yyjson_arr_get(fields, ordinal), "storage_kind") == HAKO_LLVMC_LIFECYCLE_STORAGE_DYNAMIC) {
      char name[40], done[48], fault[48];
      snprintf(name, sizeof(name), "td_dyn%u", ordinal);
      snprintf(done, sizeof(done), "%s_done", name); snprintf(fault, sizeof(fault), "%s_fault", name);
      if (array_mark || object_mark || !lv4_emit_dynamic_transaction(f, name, in->profile,
          "%site", "%handle", type, lv4_layout_slot(layout, ordinal), NULL, NULL, 0, 0, NULL, done, fault)) return 0;
      fprintf(f, "%s:\n  store i32 1, ptr %%td_status, align 4\n  br label %%%s\n%s:\n", fault, done, done);
      continue;
    }
    if (!array_mark && !object_mark) continue;
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
      mi++; continue;
    }
    oi--;
    yyjson_val* row = yyjson_arr_get(objects, oi);
    uint32_t child_id = lv4_u(row, "child_object_id");
    yyjson_val* child = lv4_layout(in, child_id);
    int slot = lv4_layout_slot(layout, ordinal);
    if (!child || slot < 0) return 0;
    char out[32];
    snprintf(out, sizeof(out), "td_read%zu", mi);
    if (!lv4_emit_object_residence_read(f, in, id, ordinal, "%handle", out, "bad")) return 0;
    fprintf(f,
        "  %%td_val%zu = load i64, ptr %%td_read%zu, align 8\n"
        "  %%td_live%zu = icmp eq i32 %%td_read%zu_outcome, 1\n"
        "  br i1 %%td_live%zu, label %%td_chain%zu, label %%td_done%zu\ntd_chain%zu:\n",
        mi, mi, mi, mi, mi, mi, mi, mi);
    fprintf(f,
        "  %%td_call%zu = call i32 @hako_lifecycle_end_%u(ptr %%frame, i64 %%td_val%zu, i64 %%site)\n"
        "  switch i32 %%td_call%zu, label %%bad [ i32 0, label %%td_done%zu i32 1, label %%td_fault%zu ]\ntd_fault%zu:\n"
        "  store i32 1, ptr %%td_status, align 4\n"
        "  br label %%td_done%zu\ntd_done%zu:\n",
        mi, child_id, mi, mi, mi, mi, mi, mi, mi);
    mi++;
  }
  fprintf(f,
      "  %%td_outer = call i32 @\"nyash.object.%s_v1\"(ptr %%frame, i32 %u, i64 %%site, i64 %%handle, i64 %lld)\n"
      "  switch i32 %%td_outer, label %%bad [ i32 0, label %%finish i32 1, label %%finish ]\nfinish:\n"
      "  %%td_prior = load i32, ptr %%td_status, align 4\n"
      "  %%td_result = or i32 %%td_prior, %%td_outer\n"
      "  ret i32 %%td_result\nbad:\n  br label %%invalid\ninvalid:\n  ret i32 2\n}\n",
      lv4_layout_dynamic(layout) ? "home_release_completed_fields" : "home_release_plain_i64", in->profile, type);
  /* Immutable complete-End capability shared by every residence consumer.
   * The value is a borrowed projection; the callback creates no second Home. */
  fprintf(f,
      "@hako_lifecycle_object_end_descriptor_%u = private constant { i32, i32, i64, ptr } { i32 %u, i32 5, i64 %lld, ptr @hako_lifecycle_object_end_adapter_%u }\n"
      "define internal i32 @hako_lifecycle_object_end_adapter_%u(ptr %%frame, i64 %%site, ptr %%value) {\nstart:\n"
      "  %%tagptr = getelementptr { i32, i32, i64, i64 }, ptr %%value, i32 0, i32 0\n"
      "  %%tag = load i32, ptr %%tagptr, align 4\n"
      "  switch i32 %%tag, label %%invalid [ i32 1, label %%empty i32 5, label %%object ]\n"
      "empty:\n  ret i32 0\nobject:\n"
      "  %%payload = getelementptr { i32, i32, i64, i64 }, ptr %%value, i32 0, i32 2\n"
      "  %%handle = load i64, ptr %%payload, align 8\n"
      "  %%status = call i32 @hako_lifecycle_end_%u(ptr %%frame, i64 %%handle, i64 %%site)\n"
      "  ret i32 %%status\ninvalid:\n  ret i32 2\n}\n",
      id, in->profile, type, id, id, id);
  return !ferror(f);
}
