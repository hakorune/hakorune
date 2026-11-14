#include "../../../../include/nyrt_observe_v1.h"

/* The live count shares word zero with the postorder parent only after zero.
 * Volatile, non-atomic accesses preserve the physical release boundary; no
 * thread sharing, runtime helper or measurement mode is introduced. */
static void lv4_emit_direct_count_drop(FILE* f, const char* pointer,
    const char* stem, const char* zero, const char* retained) {
  fprintf(f,
      "  %%%s_rc_count = load volatile i64, ptr %s, align 8\n"
      "  %%%s_rc_next = sub i64 %%%s_rc_count, 1\n"
      "  store volatile i64 %%%s_rc_next, ptr %s, align 8\n"
      "  %%%s_rc_is_zero = icmp eq i64 %%%s_rc_next, 0\n"
      "  br i1 %%%s_rc_is_zero, label %%%s, label %%%s\n",
      stem, pointer, stem, stem, stem, pointer, stem, stem, stem, zero, retained);
}

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
  if (lv4_direct_layout(in, object)) {
    fprintf(f, "  %%%s_value = call i64 @hako_direct_get(i64 %s, i64 %llu)\n"
        "  store i64 %%%s_value, ptr %%%s, align 8\n"
        "  %%%s_live = icmp ne i64 %%%s_value, 0\n"
        "  %%%s_outcome = zext i1 %%%s_live to i32\n",
        out, base, (unsigned long long)lv4_direct_offset(in, object, ordinal), out, out, out, out, out, out);
    return !ferror(f);
  }
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
static void lv4_emit_object_residence_parameter_guard(FILE* f, uint32_t v, int direct) {
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
      "  %%arg_outdomain%u = icmp ule i32 %%o%u, %u\n"
      "  %%arg_objout%u = and i1 %%arg_outnonzero%u, %%arg_outdomain%u\n"
      "  %%arg_fullobj%u = and i1 %%arg_objok%u, %%arg_objout%u\n"
      "  %%arg_ok%u = or i1 %%arg_nullout%u, %%arg_fullobj%u\n"
      "  br i1 %%arg_ok%u, label %%arg_next%u, label %%invalid\narg_next%u:\n",
      v,v, v,v, v,v,v, v,v, v,v, v,v,v, v,v, v,v,v, v,v, v,v,(direct ? 1u : 2u), v,v,v, v,v,v, v,v,v, v,v,v);
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
  if (lv4_direct_layout(in, lv4_u(op, "object_id"))) {
    fprintf(f, "@hako_direct_set(i64 %%v%u, i64 %%v%u, i64 %llu)\n",
        lv4_u(op, "base"), lv4_u(op, "value"),
        (unsigned long long)lv4_direct_offset(in, lv4_u(op, "object_id"), lv4_u(op, "field_ordinal")));
    return !ferror(f);
  }
  fprintf(f,
      "@\"nyash.object.checked_residence_set_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 %%v%u, i64 %lld, i64 %d, i64 %lld, i64 %%v%u, i32 %%residence_outcome%u)\n",
      in->profile, site, lv4_u(op, "base"), (long long)lv4_type_id(in, op),
      lv4_slot(in, op), (long long)lv4_u(child, "runtime_type_id"), lv4_u(op, "value"), b);
  return !ferror(f);
}

/* Emission-only observation, frozen once with this invocation. Disabled
 * instrumentation contributes no call or parameter to the generated program. */
static const char* lv4_direct_event_ir(const hako_lv4_input* in, unsigned event) {
  static const char* const events[] = {
    "  call void @\"nyash.observe.object_storage_event_v1\"(i32 1, i32 0)\n",
    "  call void @\"nyash.observe.object_storage_event_v1\"(i32 1, i32 1)\n",
    "  call void @\"nyash.observe.object_storage_event_v1\"(i32 1, i32 2)\n",
    "  call void @\"nyash.observe.object_storage_event_v1\"(i32 1, i32 3)\n",
  };
  return in->observe_objects && event < 4 ? events[event] : "";
}
static void lv4_emit_direct_release_call(FILE* f, const hako_lv4_input* in, yyjson_val* op) {
  fprintf(f, "@hako_direct_release_%u(i64 %%v%u", lv4_u(op, "object_id"), lv4_u(op, "value"));
  if (in->observe_objects) fprintf(f, ", i32 %u", lv4_is(op, "kind", "reclaim_unpublished")
      ? NYRT_OBSERVE_RECLAIM : NYRT_OBSERVE_END);
  fprintf(f, ")\n");
}

/* Direct words contain native addresses only. They never enter runtime handle
 * calls; admission rejects unimplemented payload/opaque boundaries. */
static void lv4_direct_globals(FILE* f, const hako_lv4_input* in) {
  if (!in->direct) return;
  if (in->observe_objects) fprintf(f, "declare void @\"nyash.observe.object_storage_event_v1\"(i32, i32)\n");
  fprintf(f, "declare ptr @malloc(i64)\ndeclare void @free(ptr)\n"
      "define internal i64 @hako_direct_get(i64 %%word, i64 %%offset) alwaysinline {\nentry:\n"
      "  %%base = inttoptr i64 %%word to ptr\n  %%slot = getelementptr i8, ptr %%base, i64 %%offset\n"
      "  %%value = load i64, ptr %%slot, align 8\n  ret i64 %%value\n}\n"
      "define internal i32 @hako_direct_set(i64 %%word, i64 %%value, i64 %%offset) alwaysinline {\nentry:\n"
      "  %%base = inttoptr i64 %%word to ptr\n  %%slot = getelementptr i8, ptr %%base, i64 %%offset\n"
      "  store i64 %%value, ptr %%slot, align 8\n  ret i32 0\n}\n"
      "define internal i32 @hako_direct_get_checked(i64 %%word, i64 %%offset, ptr %%out) alwaysinline {\nentry:\n"
      "  %%value = call i64 @hako_direct_get(i64 %%word, i64 %%offset)\n"
      "  store i64 %%value, ptr %%out, align 8\n  ret i32 0\n}\n"
      "define internal i32 @hako_direct_borrow(ptr %%frame, i64 %%site, i64 %%word, i64 %%offset, i1 %%nullable, ptr %%out) {\nentry:\n"
      "  %%value = call i64 @hako_direct_get(i64 %%word, i64 %%offset)\n"
      "  %%live = icmp ne i64 %%value, 0\n  %%ok = or i1 %%nullable, %%live\n"
      "  br i1 %%ok, label %%normal, label %%fault\nnormal:\n"
      "  store i64 %%value, ptr %%out, align 8\n  ret i32 0\nfault:\n"
      "  %%status = call i32 @\"nyash.fault.record_static_v1\"(ptr %%frame, i32 101, i64 %%site, i64 0, i64 0)\n"
      "  ret i32 %%status\n}\n");
  for (size_t li = 0; li < in->layout_count; li++) {
    uint32_t id = in->layouts[li].id;
    yyjson_val* layout = lv4_direct_layout(in, id);
    if (!layout) continue;
    uint64_t size = yyjson_get_uint(yyjson_obj_get(yyjson_obj_get(layout, "placement"), "size"));
    yyjson_val* children = yyjson_obj_get(layout, "owned_object_residences");
    size_t count = yyjson_arr_size(children);
    fprintf(f, "@hako_direct_children_%u = private constant [%zu x { i64, ptr }] [", id, count);
    for (size_t ci = 0; ci < count; ci++) {
      yyjson_val* child = yyjson_arr_get(children, ci);
      fprintf(f, "%s{ i64, ptr } { i64 %llu, ptr @hako_direct_descriptor_%u }", ci ? ", " : "",
          (unsigned long long)lv4_direct_offset(in, id, lv4_u(child, "field_ordinal")), lv4_u(child, "child_object_id"));
    }
    fprintf(f, "]\n");
    fprintf(f,
        "@hako_direct_descriptor_%u = private constant { i64, i32, i32, i32, ptr, ptr } { i64 %llu, i32 8, i32 %u, i32 %zu, ptr @hako_direct_children_%u, ptr @hako_lifecycle_end_%u }\n"
        "define internal i32 @hako_direct_new_%u(ptr %%frame, i64 %%site, ptr %%out) {\nentry:\n"
        "  %%size = load i64, ptr @hako_direct_descriptor_%u, align 8\n"
        "  %%allocation = call ptr @malloc(i64 %%size)\n  %%failed = icmp eq ptr %%allocation, null\n"
        "  br i1 %%failed, label %%allocation_fault, label %%allocated\nallocation_fault:\n"
        "  %%fault = call i32 @\"nyash.fault.record_static_v1\"(ptr %%frame, i32 100, i64 %%site, i64 0, i64 0)\n"
        "  ret i32 %%fault\nallocated:\n%s"
        "  store i64 1, ptr %%allocation, align 8\n"
        "  %%cursor = getelementptr i8, ptr %%allocation, i64 8\n  store i64 %llu, ptr %%cursor, align 8\n"
        "  %%word = ptrtoint ptr %%allocation to i64\n  store i64 %%word, ptr %%out, align 8\n  ret i32 0\n}\n",
        id, (unsigned long long)size, id, count, id, id, id, id, lv4_direct_event_ir(in, NYRT_OBSERVE_ALLOC),
        (unsigned long long)(((uint64_t)id << 32) | count));
    fprintf(f, "define internal i32 @hako_direct_release_%u(i64 %%word%s) {\nentry:\n"
        "  %%allocation = inttoptr i64 %%word to ptr\n",
        id, in->observe_objects ? ", i32 %event" : "");
    lv4_emit_direct_count_drop(f, "%allocation", "release", "release_zero", "release_retained");
    fprintf(f, "release_zero:\n%s  call void @free(ptr %%allocation)\n%s"
        "  ret i32 0\nrelease_retained:\n  ret i32 0\n}\n",
        in->observe_objects ? "  call void @\"nyash.observe.object_storage_event_v1\"(i32 1, i32 %event)\n" : "",
        lv4_direct_event_ir(in, NYRT_OBSERVE_FREE));
  }
}

static int lv4_emit_scalar_read(FILE* f, const hako_lv4_input* in, yyjson_val* ins, uint32_t dst, uint32_t b, size_t ri) {
  uint32_t object = lv4_u(ins, "object_id");
  if (lv4_direct_layout(in, object)) {
    fprintf(f, "  %%v%u = call i64 @hako_direct_get(i64 %%v%u, i64 %llu)\n",
        dst, lv4_u(ins, "base"), (unsigned long long)lv4_direct_offset(in, object, lv4_u(ins, "field_ordinal")));
    return !ferror(f);
  }

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
  return !ferror(f);
}

static int lv4_emit_scalar_write(FILE* f, const hako_lv4_input* in, yyjson_val* ins, uint32_t dst, uint32_t b, size_t ri) {
  uint32_t object = lv4_u(ins, "object_id");
  if (lv4_direct_layout(in, object)) {
    fprintf(f, "  %%direct_set%u_%zu = call i32 @hako_direct_set(i64 %%v%u, i64 %%v%u, i64 %llu)\n",
        b, ri, lv4_u(ins, "base"), lv4_u(ins, "value"),
        (unsigned long long)lv4_direct_offset(in, object, lv4_u(ins, "field_ordinal")));
    return !ferror(f);
  }

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
  return !ferror(f);
}

static void lv4_emit_object_allocation_call(FILE* f, const hako_lv4_input* in, yyjson_val* op, unsigned long long site, uint32_t b) {
        if (lv4_direct_layout(in, lv4_u(op, "object_id"))) {
          fprintf(f, "@hako_direct_new_%u(ptr %%frame, i64 %llu, ptr %%new%u)\n", lv4_u(op, "object_id"), site, b);
        } else fprintf(f, "@\"nyash.object.checked_new_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 %lld, ptr @hako_lifecycle_layout_%u, i64 %u, ptr %%new%u)\n",
            in->profile, site, (long long)lv4_type_id(in, op), lv4_u(op, "object_id"),
            lv4_u(lv4_layout(in, lv4_u(op, "object_id")), "field_count"), b);
}

static void lv4_emit_object_borrow_call(FILE* f, const hako_lv4_input* in,
    yyjson_val* op, unsigned long long site, uint32_t b) {
        if (lv4_direct_layout(in, lv4_u(op, "object_id")))
          fprintf(f, "@hako_direct_borrow(ptr %%frame, i64 %llu, i64 %%v%u, i64 %llu, i1 %s, ptr %%field_read%u)\n",
              site, lv4_u(op, "base"), (unsigned long long)lv4_direct_offset(in, lv4_u(op, "object_id"), lv4_u(op, "field_ordinal")),
              yyjson_get_bool(yyjson_obj_get(op, "nullable")) ? "true" : "false", b);
        else fprintf(f, "@\"nyash.object.checked_object_field_borrow_v1\"(ptr %%frame, i32 %u, i64 %llu, i64 %%v%u, i64 %lld, i64 %d, i64 %lld, i32 %u, ptr %%field_read%u)\n",
            in->profile, site, lv4_u(op, "base"), (long long)lv4_type_id(in, op), lv4_slot(in, op),
            (long long)lv4_u(lv4_layout(in, lv4_u(op, "expected_object_id")), "runtime_type_id"),
            yyjson_get_bool(yyjson_obj_get(op, "nullable")) ? 1u : 0u, b);

}
