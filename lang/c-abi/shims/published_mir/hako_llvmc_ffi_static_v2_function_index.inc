/* Physical definition/call correspondence over retained body identities.
 * Ordered parameters and actuals remain in the body; no second signature graph. */
static int hako_static_v2_expanded_find(const struct HakoStaticV2Index* index, yyjson_val* fn) {
  if (!index->frame || !fn) return -1;
  for (size_t i = 0; i < index->frame->expanded_function_count; i++)
    if (index->expanded[i].function == fn) return (int)i;
  return -1;
}

static int hako_static_v2_internal_symbol_valid(const char* name) {
  if (!name || !*name) return 0;
  for (const unsigned char* p = (const unsigned char*)name; *p; p++)
    if (!((*p >= 'a' && *p <= 'z') || (*p >= 'A' && *p <= 'Z') ||
          (*p >= '0' && *p <= '9') || *p == '_')) return 0;
  return 1;
}

static int hako_static_v2_bind_functions(struct HakoStaticV2Index* index, yyjson_val* functions) {
  const hako_llvmc_published_static_frame_v2* frame = index->frame;
  index->expanded = frame->expanded_function_count ? calloc(frame->expanded_function_count, sizeof(*index->expanded)) : NULL;
  index->calls = frame->call_count ? calloc(frame->call_count, sizeof(*index->calls)) : NULL;
  if ((frame->expanded_function_count && !index->expanded) || (frame->call_count && !index->calls)) return 0;
  for (size_t i = 0; i < frame->expanded_function_count; i++) {
    const hako_llvmc_expanded_function_v2* row = &frame->expanded_functions[i];
    yyjson_val* fn = hako_static_v2_function(functions, row->function_name);
    if (!fn || !hako_static_v2_internal_symbol_valid(row->internal_target)) return 0;
    for (size_t fi = 0; fi < yyjson_arr_size(functions); fi++)
      if (hako_static_v2_text(yyjson_obj_get(yyjson_arr_get(functions, fi), "name"), row->internal_target)) return 0;
    for (size_t j = 0; j < i; j++)
      if (index->expanded[j].function == fn ||
          !strcmp(frame->expanded_functions[j].internal_target, row->internal_target)) return 0;
    index->expanded[i].function = fn;
  }
  for (size_t i = 0; i < frame->call_count; i++) {
    const hako_llvmc_published_static_method_call_v1* row = &frame->calls[i];
    if (row->kind != HAKO_LLVMC_PUBLISHED_CALL_KIND_STATIC_METHOD &&
        row->kind != HAKO_LLVMC_PUBLISHED_CALL_KIND_FREE_FUNCTION) continue;
    yyjson_val* fn = hako_static_v2_function(functions, row->function_name);
    yyjson_val* target = hako_static_v2_function(functions, row->target_symbol);
    yyjson_val* ins = hako_static_v2_instruction(fn, row->block_id, row->instruction_index);
    yyjson_val* call = yyjson_obj_get(ins, "mir_call");
    yyjson_val* args = yyjson_obj_get(call, "args");
    yyjson_val* params = yyjson_obj_get(target, "params");
    if (!fn || !target || !hako_static_v2_text(yyjson_obj_get(ins, "op"), "mir_call") ||
        !hako_static_v2_text(yyjson_obj_get(yyjson_obj_get(call, "callee"), "type"), "Global") ||
        !yyjson_is_arr(args) || !yyjson_is_arr(params) || yyjson_arr_size(args) != row->arity ||
        yyjson_arr_size(params) != row->arity) return 0;
    for (size_t ai = 0; ai < row->arity; ai++) {
      uint32_t actual, formal;
      if (!hako_static_v2_u32(yyjson_arr_get(args, ai), &actual) ||
          !hako_static_v2_u32(yyjson_arr_get(params, ai), &formal)) return 0;
    }
    for (size_t j = 0; j < i; j++)
      if (index->calls[j].function == fn && index->calls[j].instruction == ins) return 0;
    index->calls[i] = (struct HakoStaticV2CallSite){fn, ins, target};
  }
  return 1;
}

static int hako_static_v2_formal_valid(const struct HakoStaticV2Index* index,
    const hako_llvmc_value_projection_v2* row, yyjson_val* fn) {
  if (hako_static_v2_expanded_find(index, fn) < 0 || row->value_kind || row->encoding ||
      row->payload || row->operation || (row->flags & ~HAKO_LLVMC_MAP_ORIGINAL_REQUIRED)) return 0;
  yyjson_val* params = yyjson_obj_get(fn, "params");
  uint32_t id;
  if (!hako_static_v2_u32(yyjson_arr_get(params, row->source_ordinal), &id) || id != row->value_id) return 0;
  for (size_t pi = 0; pi < yyjson_arr_size(params); pi++) {
    uint32_t other;
    if (!hako_static_v2_u32(yyjson_arr_get(params, pi), &other) ||
        (pi != row->source_ordinal && other == id)) return 0;
  }
  yyjson_val* blocks = yyjson_obj_get(fn, "blocks");
  for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    yyjson_val* insts = yyjson_obj_get(yyjson_arr_get(blocks, bi), "instructions");
    for (size_t ii = 0; ii < yyjson_arr_size(insts); ii++) {
      uint32_t dst;
      if (hako_static_v2_u32(yyjson_obj_get(yyjson_arr_get(insts, ii), "dst"), &dst) && dst == id) return 0;
    }
  }
  return 1;
}

/* -1 delegates nonexpanded sites to the existing original-operand owner. */
static int hako_static_v2_expanded_original_use(const struct HakoStaticV2Index* index,
    yyjson_val* fn, yyjson_val* ins, uint32_t value) {
  for (size_t ci = 0; ci < index->frame->call_count; ci++) {
    const struct HakoStaticV2CallSite* call = &index->calls[ci];
    if (call->function != fn || call->instruction != ins || hako_static_v2_expanded_find(index, call->target) < 0) continue;
    yyjson_val* args = yyjson_obj_get(yyjson_obj_get(ins, "mir_call"), "args");
    yyjson_val* params = yyjson_obj_get(call->target, "params");
    for (size_t ai = 0; ai < yyjson_arr_size(args); ai++) {
      uint32_t actual, formal;
      if (!hako_static_v2_u32(yyjson_arr_get(args, ai), &actual) ||
          !hako_static_v2_u32(yyjson_arr_get(params, ai), &formal)) return 1;
      if (actual != value) continue;
      int vi = hako_static_v2_value_find(index, call->target, formal);
      if (vi < 0 || index->frame->values[vi].action != HAKO_LLVMC_MAP_ACTION_FORMAL ||
          index->frame->values[vi].flags) return 1;
    }
    return 0;
  }
  return -1;
}

static int hako_static_v2_expanded_target(const struct HakoStaticV2Index* index, const char* logical) {
  if (!index->frame || !logical) return -1;
  for (size_t i = 0; i < index->frame->expanded_function_count; i++)
    if (!strcmp(index->frame->expanded_functions[i].function_name, logical)) return (int)i;
  return -1;
}
