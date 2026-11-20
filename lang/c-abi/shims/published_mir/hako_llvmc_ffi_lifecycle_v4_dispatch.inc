/* The descriptor is immutable image data. Resolve function ordinals only
 * from published metadata that was independently checked by admission. */
static void lv4_dispatch_globals(FILE* f, const hako_lv4_input* in) {
  if (!in->dispatch) return;
  yyjson_val* layouts = yyjson_obj_get(in->root, "layouts");
  size_t type_count = yyjson_arr_size(layouts);
  size_t row_count = yyjson_arr_size(in->dispatch);
  fprintf(f, "%%nyrt.dispatch.type = type { i64, ptr, i64 }\n"
      "%%nyrt.dispatch.row = type { i64, ptr, i64, i64, ptr, ptr }\n"
      "%%nyrt.dispatch.program = type { i32, i32, ptr, i64, ptr, i64 }\n");
  fprintf(f, "@dispatch_types = private constant [%zu x %%nyrt.dispatch.type] [", type_count);
  for (size_t i = 0; i < type_count; i++) {
    yyjson_val* layout = yyjson_arr_get(layouts, i);
    fprintf(f, "%s%%nyrt.dispatch.type { i64 %u, ptr @hako_lifecycle_layout_%u, i64 %u }",
        i ? ", " : "", lv4_u(layout, "runtime_type_id"), lv4_u(layout, "object_id"), lv4_u(layout, "field_count"));
  }
  fprintf(f, "]\n");
  for (size_t i = 0; i < row_count; i++) {
    yyjson_val* row = yyjson_arr_get(in->dispatch, i);
    yyjson_val* selector = yyjson_obj_get(row, "selector");
    size_t len = yyjson_get_len(selector);
    const unsigned char* bytes = (const unsigned char*)yyjson_get_str(selector);
    fprintf(f, "@dispatch_selector_%zu = private constant [%zu x i8] c\"", i, len);
    for (size_t j = 0; j < len; j++) fprintf(f, "\\%02X", bytes[j]);
    fprintf(f, "\"\n");
    yyjson_val* params = yyjson_obj_get(row, "parameters");
    size_t count = yyjson_arr_size(params);
    fprintf(f, "@dispatch_parameters_%zu = private constant [%zu x i32] [", i, count);
    for (size_t j = 0; j < count; j++)
      fprintf(f, "%si32 %u", j ? ", " : "", (unsigned)yyjson_get_uint(yyjson_arr_get(params, j)));
    fprintf(f, "]\n");
    fprintf(f, "define internal i32 @dispatch_thunk_%zu(ptr %%frame, i64 %%receiver, ptr %%args, i64 %%count, ptr %%out) {\nentry:\n", i);
    for (size_t j = 0; j < count; j++) {
      fprintf(f, "  %%ap%zu = getelementptr %%nyrt.dynamic.operand, ptr %%args, i64 %zu, i32 2\n"
          "  %%arg%zu = load i64, ptr %%ap%zu, align 8\n", j, j, j, j);
      if (yyjson_get_uint(yyjson_arr_get(params, j)) == 0) {
        fprintf(f, "  %%kp%zu = getelementptr %%nyrt.dynamic.operand, ptr %%args, i64 %zu, i32 0\n"
            "  %%kind%zu = load i32, ptr %%kp%zu, align 8\n"
            "  %%tp%zu = getelementptr %%nyrt.dynamic.operand, ptr %%args, i64 %zu, i32 1\n"
            "  %%tag%zu = load i32, ptr %%tp%zu, align 4\n"
            "  %%tagged%zu = icmp eq i32 %%kind%zu, 1\n"
            "  %%raw_integer%zu = icmp eq i32 %%kind%zu, 2\n"
            "  %%scalar_tag%zu = select i1 %%raw_integer%zu, i32 1, i32 2\n"
            "  %%formal_tag%zu = select i1 %%tagged%zu, i32 %%tag%zu, i32 %%scalar_tag%zu\n",
            j,j,j,j, j,j,j,j, j,j,j,j, j,j, j,j,j,j);
      }
    }
    fprintf(f, "  %%result = call i32 @hako_lifecycle_ordinary_%u(ptr %%frame, ptr %%out, i64 %%receiver",
        lv4_u(row, "target"));
    for (size_t j = 0; j < count; j++) {
      if (yyjson_get_uint(yyjson_arr_get(params, j)) == 0) fprintf(f, ", i32 %%formal_tag%zu", j);
      fprintf(f, ", i64 %%arg%zu", j);
    }
    fprintf(f, ")\n  ret i32 %%result\n}\n");
  }
  fprintf(f, "@dispatch_rows = private constant [%zu x %%nyrt.dispatch.row] [", row_count);
  for (size_t i = 0; i < row_count; i++) {
    yyjson_val* row = yyjson_arr_get(in->dispatch, i);
    yyjson_val* layout = lv4_layout(in, lv4_u(row, "object_id"));
    fprintf(f, "%s%%nyrt.dispatch.row { i64 %u, ptr @dispatch_selector_%zu, i64 %zu, i64 %u, ptr @dispatch_parameters_%zu, ptr @dispatch_thunk_%zu }",
        i ? ", " : "", lv4_u(layout, "runtime_type_id"), i, yyjson_get_len(yyjson_obj_get(row, "selector")),
        lv4_u(row, "arity"), i, i);
  }
  /* The address itself is the allocation stamp. Do not use unnamed_addr. */
  fprintf(f, "]\n@dispatch_program = private constant %%nyrt.dispatch.program { i32 1, i32 0, ptr @dispatch_types, i64 %zu, ptr @dispatch_rows, i64 %zu }\n",
      type_count, row_count);
  fprintf(f, "declare i32 @\"nyash.dynamic.checked_invoke_program_v1\"(ptr, i64, ptr, ptr, ptr, i64, ptr, i64, ptr)\n"
      "declare i32 @\"nyash.object.checked_new_in_program_v1\"(ptr, i32, i64, ptr, i64, ptr, i64, ptr)\n");
}
