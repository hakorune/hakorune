/* Return output from the existing admitted function. This child issues no
 * identity, lease, result role, or source permission; ferror stays with owner. */
static void lv4_emit_return(FILE* f, const hako_lv4_input* in,
    yyjson_val* fn, yyjson_val* term, uint32_t b,
    int birth, int ordinary_map, int ordinary) {
  if (birth) fprintf(f, "  ret i32 0\n");
  else if (ordinary_map) {
    /* Move the live lease into caller-owned storage, then dispose the
       moved-from slot. The caller owns the resulting live map. */
    uint32_t value = lv4_u(term, "value");
    fprintf(f,
        "  %%mv%u = call i32 @\"nyash.map.storage_move_v1\"(ptr %%out_map, ptr %%v%u)\n"
        "  %%mv_ok%u = icmp eq i32 %%mv%u, 0\n"
        "  br i1 %%mv_ok%u, label %%mv_step%u, label %%invalid\nmv_step%u:\n"
        "  %%md%u = call i32 @\"nyash.map.storage_dispose_v1\"(ptr %%v%u)\n"
        "  %%md_ok%u = icmp eq i32 %%md%u, 0\n"
        "  br i1 %%md_ok%u, label %%mv_done%u, label %%invalid\nmv_done%u:\n"
        "  ret i32 0\n",
        b, value, b, b, b, b, b, b, value, b, b, b, b, b);
  }
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
}
