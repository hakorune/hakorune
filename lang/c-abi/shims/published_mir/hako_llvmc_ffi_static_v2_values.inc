/* Value-row wire validation and reachability over the sole retained body graph.
 * Operation rows select physical consumers; no origin/name-based selection. */
static const char* hako_static_v2_operation(yyjson_val* ins) {
  const char* op = yyjson_get_str(yyjson_obj_get(ins, "op_kind"));
  if (!op) op = yyjson_get_str(yyjson_obj_get(ins, "cmp"));
  return op ? op : yyjson_get_str(yyjson_obj_get(ins, "operation"));
}

static const char* hako_static_v2_binary(const char* op) {
  if (!op) return NULL;
  if (!strcmp(op, "Add") || !strcmp(op, "+")) return "add";
  if (!strcmp(op, "Sub") || !strcmp(op, "-")) return "sub";
  if (!strcmp(op, "Mul") || !strcmp(op, "*")) return "mul";
  if (!strcmp(op, "Div") || !strcmp(op, "/")) return "sdiv";
  if (!strcmp(op, "Mod") || !strcmp(op, "%")) return "srem";
  return NULL;
}

static const char* hako_static_v2_predicate(const char* op) {
  if (!op) return NULL;
  if (!strcmp(op, "Lt") || !strcmp(op, "lt") || !strcmp(op, "<")) return "slt";
  if (!strcmp(op, "Le") || !strcmp(op, "le") || !strcmp(op, "<=")) return "sle";
  if (!strcmp(op, "Eq") || !strcmp(op, "eq") || !strcmp(op, "==")) return "eq";
  if (!strcmp(op, "Ne") || !strcmp(op, "ne") || !strcmp(op, "!=")) return "ne";
  if (!strcmp(op, "Ge") || !strcmp(op, "ge") || !strcmp(op, ">=")) return "sge";
  if (!strcmp(op, "Gt") || !strcmp(op, "gt") || !strcmp(op, ">")) return "sgt";
  return NULL;
}

static int hako_static_v2_operand(yyjson_val* ins, const char* field,
                                 const char* alias, uint32_t* value) {
  yyjson_val* input = yyjson_obj_get(ins, field);
  return hako_static_v2_u32(input ? input : yyjson_obj_get(ins, alias), value);
}

/* -1 is a known producer whose actual emission has not joined the ledger yet. */
static int hako_static_v2_nonexact_valid(
    const hako_llvmc_value_projection_v2* row, yyjson_val* ins) {
  if (row->flags != HAKO_LLVMC_MAP_ORIGINAL_REQUIRED || row->payload || row->source_ordinal) return 0;
  if (row->action == HAKO_LLVMC_MAP_ACTION_ORIGINAL_VALUE) {
    if (row->operation || row->encoding != HAKO_LLVMC_MAP_ENCODING_I64_BITS ||
        (row->value_kind != HAKO_LLVMC_MAP_VALUE_I64 && row->value_kind != HAKO_LLVMC_MAP_VALUE_BOOL &&
         row->value_kind != HAKO_LLVMC_MAP_VALUE_HANDLE)) return 0;
    if (hako_static_v2_intrinsic_map(ins)) return row->value_kind == HAKO_LLVMC_MAP_VALUE_HANDLE;
    if (hako_static_v2_text(yyjson_obj_get(ins, "op"), "const")) {
      yyjson_val* value = yyjson_obj_get(ins, "value");
      yyjson_val* type = yyjson_obj_get(value, "type");
      return row->value_kind == HAKO_LLVMC_MAP_VALUE_HANDLE &&
          hako_static_v2_text(yyjson_obj_get(type, "kind"), "handle") &&
          hako_static_v2_text(yyjson_obj_get(type, "box_type"), "StringBox") &&
          yyjson_is_str(yyjson_obj_get(value, "value"));
    }
    return -1;
  }
  if (row->action != HAKO_LLVMC_MAP_ACTION_OPERATION) return 0;
  const char* op = hako_static_v2_operation(ins);
  uint32_t kind = HAKO_LLVMC_MAP_VALUE_BOOL, encoding = HAKO_LLVMC_MAP_ENCODING_BOOL_I1_ZEXT;
  int valid = 0;
  switch (row->operation) {
    case HAKO_LLVMC_MAP_PHYSICAL_I64_BINARY:
      kind = HAKO_LLVMC_MAP_VALUE_I64; encoding = HAKO_LLVMC_MAP_ENCODING_I64_BITS;
      valid = hako_static_v2_text(yyjson_obj_get(ins, "op"), "binop") && hako_static_v2_binary(op);
      break;
    case HAKO_LLVMC_MAP_PHYSICAL_STRING_CONCAT:
      kind = HAKO_LLVMC_MAP_VALUE_HANDLE; encoding = HAKO_LLVMC_MAP_ENCODING_I64_BITS;
      valid = hako_static_v2_text(yyjson_obj_get(ins, "op"), "binop") &&
          hako_static_v2_binary(op) && !strcmp(hako_static_v2_binary(op), "add");
      break;
    case HAKO_LLVMC_MAP_PHYSICAL_I64_COMPARE:
    case HAKO_LLVMC_MAP_PHYSICAL_BOOL_COMPARE:
    case HAKO_LLVMC_MAP_PHYSICAL_STRING_COMPARE:
      valid = hako_static_v2_text(yyjson_obj_get(ins, "op"), "compare") && hako_static_v2_predicate(op);
      break;
    case HAKO_LLVMC_MAP_PHYSICAL_I64_NOT:
    case HAKO_LLVMC_MAP_PHYSICAL_BOOL_NOT:
      valid = hako_static_v2_text(yyjson_obj_get(ins, "op"), "unop") && op &&
          (!strcmp(op, "not") || !strcmp(op, "Not"));
      break;
  }
  return valid && row->value_kind == kind && row->encoding == encoding;
}

static int hako_static_v2_operation_inputs(
    const hako_llvmc_value_projection_v2* row, yyjson_val* ins, uint32_t inputs[2]) {
  if (row->action != HAKO_LLVMC_MAP_ACTION_OPERATION) return 0;
  if (row->operation == HAKO_LLVMC_MAP_PHYSICAL_I64_NOT || row->operation == HAKO_LLVMC_MAP_PHYSICAL_BOOL_NOT)
    return hako_static_v2_operand(ins, "src", "operand", &inputs[0]) ? 1 : -1;
  return hako_static_v2_operand(ins, "lhs", "left", &inputs[0]) &&
      hako_static_v2_operand(ins, "rhs", "right", &inputs[1]) ? 2 : -1;
}

/* Mark reachable rows using temporary bits, never the actual-emission ledger. */
static int hako_static_v2_value_closure(const struct HakoStaticV2Index* index) {
  size_t n = index->frame->value_count;
  unsigned char* reached = n ? calloc(n, 1) : NULL;
  if (n && !reached) return 0;
  int valid = 0;
  for (size_t mi = 0; mi < index->frame->map_operation_count; mi++) {
    if (index->frame->map_operations[mi].kind != HAKO_LLVMC_MAP_OP_WRITE) continue;
    uint32_t value;
    if (!hako_static_v2_u32(yyjson_obj_get(index->maps[mi].instruction, "value"), &value)) goto done;
    int vi = hako_static_v2_value_find(index, index->maps[mi].function, value);
    if (vi < 0) goto done;
    reached[vi] = 1;
  }
  int changed;
  do {
    changed = 0;
    for (size_t vi = 0; vi < n; vi++) {
      if (!reached[vi]) continue;
      uint32_t inputs[2];
      int count = hako_static_v2_operation_inputs(&index->frame->values[vi], index->values[vi].instruction, inputs);
      if (count < 0) goto done;
      for (int i = 0; i < count; i++) {
        int child = hako_static_v2_value_find(index, index->values[vi].function, inputs[i]);
        if (child < 0 || !(index->frame->values[child].flags & HAKO_LLVMC_MAP_ORIGINAL_REQUIRED)) goto done;
        const hako_llvmc_value_projection_v2* parent = &index->frame->values[vi];
        const hako_llvmc_value_projection_v2* input = &index->frame->values[child];
        uint32_t expected = HAKO_LLVMC_MAP_VALUE_I64;
        if (parent->operation == HAKO_LLVMC_MAP_PHYSICAL_BOOL_COMPARE || parent->operation == HAKO_LLVMC_MAP_PHYSICAL_BOOL_NOT)
          expected = HAKO_LLVMC_MAP_VALUE_BOOL;
        if (parent->operation == HAKO_LLVMC_MAP_PHYSICAL_STRING_COMPARE || parent->operation == HAKO_LLVMC_MAP_PHYSICAL_STRING_CONCAT) {
          expected = HAKO_LLVMC_MAP_VALUE_HANDLE;
          /* Current String inputs are exact String constants or selected concat;
           * another Handle producer cannot stand in for a String consumer. */
          if (!(input->action == HAKO_LLVMC_MAP_ACTION_OPERATION && input->operation == HAKO_LLVMC_MAP_PHYSICAL_STRING_CONCAT) &&
              !(input->action == HAKO_LLVMC_MAP_ACTION_ORIGINAL_VALUE &&
                hako_static_v2_text(yyjson_obj_get(index->values[child].instruction, "op"), "const"))) goto done;
        }
        if (input->value_kind != expected) goto done;
        if (!reached[child]) { reached[child] = 1; changed = 1; }
      }
    }
  } while (changed);
  for (size_t i = 0; i < n; i++) if (!reached[i]) goto done;
  valid = 1;
done:
  free(reached);
  return valid;
}
