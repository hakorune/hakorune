/* Exact immutable ingress payload, no runtime environment or Array factory. */
static void lv4_script_arguments_globals(FILE* f, const hako_lv4_input* in) {
  fprintf(f, "%%nyrt.script.argument = type { ptr, i64 }\n"
      "declare i32 @\"nyash.array.checked_script_arguments_new_v1\"(ptr, i64, ptr, i64, ptr)\n");
  for (size_t fi = 0; fi < yyjson_arr_size(in->functions); fi++) {
    yyjson_val* blocks = hako_physical_blocks(yyjson_arr_get(in->functions, fi));
    for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
      yyjson_val* block = yyjson_arr_get(blocks, bi);
      yyjson_val* op = yyjson_obj_get(lv4_term(block), "operation");
      if (!lv4_is(op, "kind", "script_arguments_new")) continue;
      yyjson_val* values = yyjson_obj_get(op, "values");
      uint32_t b = lv4_u(block, "id");
      size_t count = yyjson_arr_size(values);
      for (size_t i = 0; i < count; i++) {
        yyjson_val* text = yyjson_arr_get(values, i);
        size_t len = yyjson_get_len(text);
        const unsigned char* bytes = (const unsigned char*)yyjson_get_str(text);
        fprintf(f, "@argv_text_%zu_%u_%zu = private unnamed_addr constant [%zu x i8] c\"", fi, b, i, len);
        for (size_t j = 0; j < len; j++) fprintf(f, "\\%02X", bytes[j]);
        fprintf(f, "\", align 1\n");
      }
      fprintf(f, "@argv_%zu_%u = private constant [%zu x %%nyrt.script.argument] [", fi, b, count);
      for (size_t i = 0; i < count; i++)
        fprintf(f, "%s%%nyrt.script.argument { ptr @argv_text_%zu_%u_%zu, i64 %zu }",
            i ? ", " : "", fi, b, i, yyjson_get_len(yyjson_arr_get(values, i)));
      fprintf(f, "], align 8\n");
    }
  }
}
