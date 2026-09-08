/* One invocation-owned binding of typed V2 rows to the retained body.
 * No operand graph or source classification is issued here. Lookup observes;
 * only the actual instruction emitter consumes the frame-wide ledger. */
struct HakoStaticV2MapSite {
  yyjson_val *function, *instruction;
  unsigned char used;
};
struct HakoStaticV2ValueSite {
  yyjson_val *function, *instruction;
  unsigned char used;
};
struct HakoStaticV2Index {
  const hako_llvmc_published_static_frame_v2* frame;
  struct HakoStaticV2MapSite* maps;
  struct HakoStaticV2ValueSite* values;
};

static void hako_static_v2_clear(struct HakoStaticV2Index* index) {
  free(index->maps);
  free(index->values);
  *index = (struct HakoStaticV2Index){0};
}

static int hako_static_v2_u32(yyjson_val* value, uint32_t* out) {
  if (!yyjson_is_int(value) || (yyjson_is_sint(value) && yyjson_get_sint(value) < 0) ||
      yyjson_get_uint(value) > UINT32_MAX) return 0;
  *out = (uint32_t)yyjson_get_uint(value);
  return 1;
}

static int hako_static_v2_text(yyjson_val* value, const char* expected) {
  return expected && yyjson_is_str(value) &&
      yyjson_get_len(value) == strlen(expected) &&
      !memcmp(yyjson_get_str(value), expected, strlen(expected));
}

/* A second match is malformed, not first-match resolution. */
static yyjson_val* hako_static_v2_function(yyjson_val* functions, const char* name) {
  yyjson_val* found = NULL;
  if (!name || !*name) return NULL;
  for (size_t i = 0; i < yyjson_arr_size(functions); i++) {
    yyjson_val* function = yyjson_arr_get(functions, i);
    if (hako_static_v2_text(yyjson_obj_get(function, "name"), name)) {
      if (found) return NULL;
      found = function;
    }
  }
  return found;
}

static yyjson_val* hako_static_v2_instruction(
    yyjson_val* function, uint32_t bid, uint32_t ordinal) {
  yyjson_val *blocks = yyjson_obj_get(function, "blocks"), *found = NULL;
  for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    yyjson_val* block = yyjson_arr_get(blocks, bi);
    uint32_t id;
    if (!hako_static_v2_u32(yyjson_obj_get(block, "id"), &id)) return NULL;
    if (id == bid) {
      if (found) return NULL;
      found = block;
    }
  }
  yyjson_val* instructions = yyjson_obj_get(found, "instructions");
  return yyjson_arr_get(instructions, ordinal);
}

static yyjson_val* hako_static_v2_definition(yyjson_val* function, uint32_t value) {
  yyjson_val* params = yyjson_obj_get(function, "params");
  for (size_t pi = 0; pi < yyjson_arr_size(params); pi++) {
    uint32_t id;
    if (!hako_static_v2_u32(yyjson_arr_get(params, pi), &id) || id == value) return NULL;
  }
  yyjson_val *blocks = yyjson_obj_get(function, "blocks"), *found = NULL;
  for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    yyjson_val* instructions = yyjson_obj_get(yyjson_arr_get(blocks, bi), "instructions");
    for (size_t ii = 0; ii < yyjson_arr_size(instructions); ii++) {
      yyjson_val* instruction = yyjson_arr_get(instructions, ii);
      yyjson_val* dst = yyjson_obj_get(instruction, "dst");
      uint32_t id;
      if (dst && !yyjson_is_null(dst) && !hako_static_v2_u32(dst, &id)) return NULL;
      if (hako_static_v2_u32(dst, &id) && id == value) {
        if (found) return NULL;
        found = instruction;
      }
    }
  }
  return found;
}

static int hako_static_v2_map_find(
    const struct HakoStaticV2Index* index, yyjson_val* function, yyjson_val* instruction) {
  if (!index->frame) return -1;
  for (size_t i = 0; i < index->frame->map_operation_count; i++)
    if (index->maps[i].function == function && index->maps[i].instruction == instruction)
      return (int)i;
  return -1;
}

static int hako_static_v2_value_find(
    const struct HakoStaticV2Index* index, yyjson_val* function, uint32_t value) {
  if (!index->frame) return -1;
  for (size_t i = 0; i < index->frame->value_count; i++)
    if (index->values[i].function == function && index->frame->values[i].value_id == value)
      return (int)i;
  return -1;
}

static int hako_static_v2_intrinsic_map(yyjson_val* instruction) {
  return hako_static_v2_text(yyjson_obj_get(instruction, "op"), "newbox") &&
      hako_static_v2_text(yyjson_obj_get(yyjson_obj_get(instruction, "target"), "kind"),
                          "intrinsic_map");
}

/* ExactBits is pre-JSON authority for Float bits and the Integer/Bool split.
 * JSON checks the matched physical producer; it cannot recover lost distinctions. */
static int hako_static_v2_exact_valid(
    const hako_llvmc_value_projection_v2* row, yyjson_val* instruction) {
  if (row->action != HAKO_LLVMC_MAP_ACTION_EXACT_BITS || row->encoding ||
      row->operation || row->source_ordinal || (row->flags & ~HAKO_LLVMC_MAP_ORIGINAL_REQUIRED) ||
      !hako_static_v2_text(yyjson_obj_get(instruction, "op"), "const")) return 0;
  yyjson_val* value = yyjson_obj_get(instruction, "value");
  yyjson_val* type = yyjson_obj_get(value, "type");
  yyjson_val* literal = yyjson_obj_get(value, "value");
  switch (row->value_kind) {
    case HAKO_LLVMC_MAP_VALUE_I64:
    case HAKO_LLVMC_MAP_VALUE_BOOL:
      return hako_static_v2_text(type, "i64") && yyjson_is_int(literal) &&
          (uint64_t)yyjson_get_sint(literal) == row->payload &&
          (row->value_kind != HAKO_LLVMC_MAP_VALUE_BOOL || row->payload <= 1);
    case HAKO_LLVMC_MAP_VALUE_F64:
      /* Non-finite JSON numbers can be null; the retained bits are authoritative. */
      return hako_static_v2_text(type, "f64") && !row->flags &&
          (yyjson_is_num(literal) || yyjson_is_null(literal));
    case HAKO_LLVMC_MAP_VALUE_VOID:
      return (hako_static_v2_text(type, "void") || hako_static_v2_text(type, "null")) &&
          row->payload == 0;
    default: return 0;
  }
}

#include "hako_llvmc_ffi_static_v2_values.inc"

/* Physical use check for Map-only exact constants. Unknown instruction forms
 * remain unsupported until their operand consumer joins V2; no broad numeric
 * scan of metadata or recursive JSON interpretation is used. */
static int hako_static_v2_original_use(yyjson_val* ins, uint32_t value) {
  const char* op = yyjson_get_str(yyjson_obj_get(ins, "op"));
  if (!op) return -1;
  if (!strcmp(op, "const") || !strcmp(op, "jump")) return 0;
  const char* fields[8] = {0};
  yyjson_val* args = NULL;
  if (!strcmp(op, "map_literal_entry_write")) { fields[0] = "receiver"; fields[1] = "key"; }
  else if (!strcmp(op, "copy") || !strcmp(op, "copy_owned") || !strcmp(op, "unop")) fields[0] = "src";
  else if (!strcmp(op, "ret") || !strcmp(op, "variant_tag") || !strcmp(op, "variant_project")) fields[0] = "value";
  else if (!strcmp(op, "branch")) fields[0] = "cond";
  else if (!strcmp(op, "select")) { fields[0] = "cond"; fields[1] = "then_val"; fields[2] = "else_val"; }
  else if (!strcmp(op, "binop") || !strcmp(op, "compare")) {
    fields[0] = "lhs"; fields[1] = "rhs"; fields[2] = "left"; fields[3] = "right";
  } else if (!strcmp(op, "field_get") || !strcmp(op, "field_set")) {
    fields[0] = "box"; fields[1] = "value";
  } else if (!strcmp(op, "newbox")) args = yyjson_obj_get(ins, "args");
  else if (!strcmp(op, "mir_call")) {
    yyjson_val* call = yyjson_obj_get(ins, "mir_call");
    uint32_t receiver;
    if (hako_static_v2_u32(yyjson_obj_get(yyjson_obj_get(call, "callee"), "receiver"), &receiver) && receiver == value) return 1;
    args = yyjson_obj_get(call, "args");
  } else if (!strcmp(op, "phi")) {
    args = yyjson_obj_get(ins, "incoming");
    if (!args) args = yyjson_obj_get(ins, "values");
    if (!yyjson_is_arr(args)) return -1;
    for (size_t i = 0; i < yyjson_arr_size(args); i++) {
      yyjson_val* pair = yyjson_arr_get(args, i);
      yyjson_val* incoming = yyjson_is_arr(pair) ? yyjson_arr_get(pair, 0) : yyjson_obj_get(pair, "value");
      uint32_t id;
      if (!hako_static_v2_u32(incoming, &id)) return -1;
      if (id == value) return 1;
    }
    return 0;
  } else return -1;
  for (size_t i = 0; fields[i]; i++) {
    uint32_t id;
    if (hako_static_v2_u32(yyjson_obj_get(ins, fields[i]), &id) && id == value) return 1;
  }
  for (size_t i = 0; i < yyjson_arr_size(args); i++) {
    uint32_t id;
    if (!hako_static_v2_u32(yyjson_arr_get(args, i), &id)) return -1;
    if (id == value) return 1;
  }
  return 0;
}

static int hako_static_v2_bind(
    struct HakoStaticV2Index* index, yyjson_doc* document,
    const hako_llvmc_published_static_frame_v2* frame, char** error) {
  if (index->frame) return set_err_owned(error, "[freeze:contract][static-v2/already-bound]");
  if (!frame || frame->revision != HAKO_LLVMC_STATIC_FRAME_REVISION ||
      frame->byte_size != sizeof(*frame))
    return set_err_owned(error, "[freeze:contract][static-v2/header]");
  #define V2_PAIR(p, n, element) ((!(p) == !(n)) && (n) <= INT_MAX && (n) <= SIZE_MAX / sizeof(element))
  if (!V2_PAIR(frame->calls, frame->call_count, *frame->calls) ||
      !V2_PAIR(frame->map_operations, frame->map_operation_count, *index->maps) ||
      !V2_PAIR(frame->values, frame->value_count, *index->values) ||
      !V2_PAIR(frame->expanded_functions, frame->expanded_function_count, *frame->expanded_functions))
    return set_err_owned(error, "[freeze:contract][static-v2/table-pair]");
  #undef V2_PAIR
  if (frame->expanded_function_count)
    return set_err_owned(error, "[freeze:contract][static-v2/expanded-consumer-unsupported]");
  index->maps = frame->map_operation_count ? calloc(frame->map_operation_count, sizeof(*index->maps)) : NULL;
  index->values = frame->value_count ? calloc(frame->value_count, sizeof(*index->values)) : NULL;
  const char* reason = "[freeze:contract][static-v2/storage]";
  if ((frame->map_operation_count && !index->maps) || (frame->value_count && !index->values)) goto invalid;
  index->frame = frame;
  yyjson_val* functions = yyjson_obj_get(yyjson_doc_get_root(document), "functions");
  reason = "[freeze:contract][static-v2/map-site]";
  for (size_t i = 0; i < frame->map_operation_count; i++) {
    const hako_llvmc_map_operation_v2* row = &frame->map_operations[i];
    yyjson_val* fn = hako_static_v2_function(functions, row->function_name);
    yyjson_val* ins = hako_static_v2_instruction(fn, row->block_id, row->instruction_index);
    if (!fn || !ins || row->reserved) goto invalid;
    for (size_t j = 0; j < i; j++) if (index->maps[j].function == fn && index->maps[j].instruction == ins) goto invalid;
    if (row->kind == HAKO_LLVMC_MAP_OP_ALLOCATE) {
      uint32_t dst;
      yyjson_val* args = yyjson_obj_get(ins, "args");
      if (!hako_static_v2_intrinsic_map(ins) || yyjson_obj_get(ins, "type") ||
          !yyjson_is_arr(args) || yyjson_arr_size(args) ||
          !hako_static_v2_u32(yyjson_obj_get(ins, "dst"), &dst) ||
          hako_static_v2_definition(fn, dst) != ins) goto invalid;
    } else if (row->kind != HAKO_LLVMC_MAP_OP_WRITE ||
        !hako_static_v2_text(yyjson_obj_get(ins, "op"), "map_literal_entry_write")) goto invalid;
    index->maps[i] = (struct HakoStaticV2MapSite){fn, ins, 0};
  }
  reason = "[freeze:contract][static-v2/value-action]";
  for (size_t i = 0; i < frame->value_count; i++) {
    const hako_llvmc_value_projection_v2* row = &frame->values[i];
    yyjson_val* fn = hako_static_v2_function(functions, row->function_name);
    yyjson_val* ins = hako_static_v2_definition(fn, row->value_id);
    if ((row->action >= HAKO_LLVMC_MAP_ACTION_COPY && row->action <= HAKO_LLVMC_MAP_ACTION_FORMAL) ||
        row->action == HAKO_LLVMC_MAP_ACTION_NAMED_ALIAS_OPERAND_ZERO) {
      reason = "[freeze:contract][static-v2/value-consumer-unsupported]";
      goto invalid;
    }
    if (!fn || !ins) goto invalid;
    int valid = row->action == HAKO_LLVMC_MAP_ACTION_EXACT_BITS
        ? hako_static_v2_exact_valid(row, ins) : hako_static_v2_nonexact_valid(row, ins);
    if (valid < 0) { reason = "[freeze:contract][static-v2/original-consumer-unsupported]"; goto invalid; }
    if (!valid) goto invalid;
    if (row->action == HAKO_LLVMC_MAP_ACTION_ORIGINAL_VALUE &&
        hako_static_v2_text(yyjson_obj_get(ins, "op"), "mir_call")) {
      yyjson_val* call = yyjson_obj_get(ins, "mir_call");
      if (!hako_static_v2_text(yyjson_obj_get(yyjson_obj_get(call, "callee"), "type"), "Global")) goto invalid;
      size_t matches = 0;
      for (size_t ci = 0; ci < frame->call_count; ci++) {
        const hako_llvmc_published_static_method_call_v1* selected = &frame->calls[ci];
        if (!hako_static_v2_text(yyjson_obj_get(fn, "name"), selected->function_name) ||
            hako_static_v2_instruction(fn, selected->block_id, selected->instruction_index) != ins) continue;
        yyjson_val* target = hako_static_v2_function(functions, selected->target_symbol);
        yyjson_val* args = yyjson_obj_get(call, "args");
        yyjson_val* params = yyjson_obj_get(target, "params");
        if ((selected->kind != HAKO_LLVMC_PUBLISHED_CALL_KIND_STATIC_METHOD &&
             selected->kind != HAKO_LLVMC_PUBLISHED_CALL_KIND_FREE_FUNCTION) ||
            !target || !yyjson_is_arr(args) || !yyjson_is_arr(params) ||
            yyjson_arr_size(args) != selected->arity || yyjson_arr_size(params) != selected->arity) goto invalid;
        matches++;
      }
      if (matches != 1) goto invalid;
    }
    for (size_t j = 0; j < i; j++) if (index->values[j].function == fn && frame->values[j].value_id == row->value_id) goto invalid;
    index->values[i] = (struct HakoStaticV2ValueSite){fn, ins, 0};
  }
  reason = "[freeze:contract][static-v2/body-coverage]";
  for (size_t fi = 0; fi < yyjson_arr_size(functions); fi++) {
    yyjson_val* fn = yyjson_arr_get(functions, fi);
    yyjson_val* blocks = yyjson_obj_get(fn, "blocks");
    for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
      yyjson_val* instructions = yyjson_obj_get(yyjson_arr_get(blocks, bi), "instructions");
      for (size_t ii = 0; ii < yyjson_arr_size(instructions); ii++) {
        yyjson_val* ins = yyjson_arr_get(instructions, ii);
        int write = hako_static_v2_text(yyjson_obj_get(ins, "op"), "map_literal_entry_write");
        if ((write || hako_static_v2_intrinsic_map(ins)) && hako_static_v2_map_find(index, fn, ins) < 0) goto invalid;
        if (write) {
          uint32_t receiver, key, value;
          if (!hako_static_v2_u32(yyjson_obj_get(ins, "receiver"), &receiver) ||
              !hako_static_v2_u32(yyjson_obj_get(ins, "key"), &key) ||
              !hako_static_v2_u32(yyjson_obj_get(ins, "value"), &value) ||
              !hako_static_v2_definition(fn, receiver) || !hako_static_v2_definition(fn, key) ||
              hako_static_v2_value_find(index, fn, value) < 0) goto invalid;
        }
        for (size_t vi = 0; vi < frame->value_count; vi++) {
          const hako_llvmc_value_projection_v2* row = &frame->values[vi];
          if (index->values[vi].function != fn || row->flags) continue;
          if (hako_static_v2_original_use(ins, row->value_id) != 0) goto invalid;
        }
      }
    }
  }
  if (!hako_static_v2_value_closure(index)) {
    reason = "[freeze:contract][static-v2/value-reference-closure]";
    goto invalid;
  }
  return 0;
invalid:
  hako_static_v2_clear(index);
  return set_err_owned(error, reason);
}

static int hako_static_v2_finish(const struct HakoStaticV2Index* index, char** error) {
  if (!index->frame) return 0;
  for (size_t i = 0; i < index->frame->map_operation_count; i++)
    if (!index->maps[i].used) return set_err_owned(error, "[freeze:contract][static-v2/map-row-not-consumed]");
  for (size_t i = 0; i < index->frame->value_count; i++)
    if (!index->values[i].used) return set_err_owned(error, "[freeze:contract][static-v2/value-row-not-consumed]");
  return 0;
}
