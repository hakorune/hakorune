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

static int hako_static_v2_is_copy(uint32_t action) {
  return action == HAKO_LLVMC_MAP_ACTION_COPY || action == HAKO_LLVMC_MAP_ACTION_NAMED_ALIAS_OPERAND_ZERO;
}

static int hako_static_v2_copy_source(
    const hako_llvmc_value_projection_v2* row, yyjson_val* ins, uint32_t* source) {
  yyjson_val* value = row->action == HAKO_LLVMC_MAP_ACTION_COPY
      ? yyjson_obj_get(ins, "src") : yyjson_arr_get(yyjson_obj_get(ins, "args"), 0);
  return hako_static_v2_u32(value, source);
}

static int hako_static_v2_is_transfer(uint32_t action) {
  return hako_static_v2_is_copy(action) || action == HAKO_LLVMC_MAP_ACTION_PHI || action == HAKO_LLVMC_MAP_ACTION_SELECT;
}

static yyjson_val* hako_static_v2_phi_inputs(yyjson_val* ins) {
  yyjson_val* incoming = yyjson_obj_get(ins, "incoming");
  yyjson_val* values = yyjson_obj_get(ins, "values");
  if (incoming && values) return NULL;
  return incoming ? incoming : values;
}

static int hako_static_v2_phi_input(yyjson_val* pair, uint32_t* value, uint32_t* pred) {
  if (yyjson_is_arr(pair)) return yyjson_arr_size(pair) == 2 &&
      hako_static_v2_u32(yyjson_arr_get(pair, 0), value) && hako_static_v2_u32(yyjson_arr_get(pair, 1), pred);
  yyjson_val* block = yyjson_obj_get(pair, "pred");
  if (!block) block = yyjson_obj_get(pair, "block");
  return hako_static_v2_u32(yyjson_obj_get(pair, "value"), value) && hako_static_v2_u32(block, pred);
}

/* -1 is a known producer whose actual emission has not joined the ledger yet. */
static int hako_static_v2_nonexact_valid(
    const hako_llvmc_value_projection_v2* row, yyjson_val* ins) {
  if (row->action == HAKO_LLVMC_MAP_ACTION_PHI || row->action == HAKO_LLVMC_MAP_ACTION_SELECT) {
    if (row->value_kind || row->encoding || row->payload || row->source_ordinal || row->operation ||
        (row->flags & ~HAKO_LLVMC_MAP_ORIGINAL_REQUIRED)) return 0;
    if (row->action == HAKO_LLVMC_MAP_ACTION_SELECT) {
      uint32_t cond, left, right;
      return hako_static_v2_text(yyjson_obj_get(ins, "op"), "select") &&
          hako_static_v2_u32(yyjson_obj_get(ins, "cond"), &cond) &&
          hako_static_v2_u32(yyjson_obj_get(ins, "then_val"), &left) &&
          hako_static_v2_u32(yyjson_obj_get(ins, "else_val"), &right);
    }
    yyjson_val* inputs = hako_static_v2_phi_inputs(ins);
    if (!hako_static_v2_text(yyjson_obj_get(ins, "op"), "phi") ||
        !yyjson_is_arr(inputs) || !yyjson_arr_size(inputs) || yyjson_arr_size(inputs) > 16) return 0;
    uint32_t preds[16];
    for (size_t i = 0; i < yyjson_arr_size(inputs); i++) {
      uint32_t value;
      if (!hako_static_v2_phi_input(yyjson_arr_get(inputs, i), &value, &preds[i])) return 0;
      for (size_t j = 0; j < i; j++) if (preds[j] == preds[i]) return 0;
    }
    return 1;
  }
  if (hako_static_v2_is_copy(row->action)) {
    uint32_t source;
    if (row->value_kind || row->encoding || row->payload || row->source_ordinal || row->operation ||
        (row->flags & ~HAKO_LLVMC_MAP_ORIGINAL_REQUIRED) || !hako_static_v2_copy_source(row, ins, &source)) return 0;
    if (row->action == HAKO_LLVMC_MAP_ACTION_COPY) {
      if (hako_static_v2_text(yyjson_obj_get(ins, "op"), "copy_owned")) return -1;
      return hako_static_v2_text(yyjson_obj_get(ins, "op"), "copy");
    }
    yyjson_val* type = yyjson_obj_get(ins, "type");
    return hako_static_v2_text(yyjson_obj_get(ins, "op"), "newbox") &&
        yyjson_is_str(type) && yyjson_get_len(type) && !yyjson_obj_get(ins, "target");
  }
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
    if (hako_static_v2_text(yyjson_obj_get(ins, "op"), "newbox")) {
      yyjson_val* type = yyjson_obj_get(ins, "type");
      return row->value_kind == HAKO_LLVMC_MAP_VALUE_HANDLE &&
          ((yyjson_is_str(type) && yyjson_get_len(type)) ||
           hako_static_v2_text(yyjson_obj_get(yyjson_obj_get(ins, "target"), "kind"), "intrinsic_array"));
    }
    if (hako_static_v2_text(yyjson_obj_get(ins, "op"), "mir_call"))
      return row->value_kind == HAKO_LLVMC_MAP_VALUE_I64;
    uint32_t plan;
    if (hako_static_v2_text(yyjson_obj_get(ins, "op"), "variant_make") ||
        hako_static_v2_text(yyjson_obj_get(ins, "op"), "variant_tag") ||
        hako_static_v2_text(yyjson_obj_get(ins, "op"), "variant_project")) {
      if (!hako_static_v2_u32(yyjson_obj_get(ins, "boxed_sum_abi_plan_id"), &plan)) return 0;
      if (hako_static_v2_text(yyjson_obj_get(ins, "op"), "variant_make"))
        return row->value_kind == HAKO_LLVMC_MAP_VALUE_HANDLE ? -1 : 0;
      if (hako_static_v2_text(yyjson_obj_get(ins, "op"), "variant_tag"))
        return row->value_kind == HAKO_LLVMC_MAP_VALUE_I64;
      yyjson_val* storage = yyjson_obj_get(ins, "boxed_sum_payload_storage");
      yyjson_val* type = yyjson_obj_get(ins, "payload_type");
      if (row->value_kind == HAKO_LLVMC_MAP_VALUE_HANDLE) {
        if (!hako_static_v2_text(storage, "handle")) return 0;
        return hako_static_v2_text(type, "String") ? 1 : -1;
      }
      return hako_static_v2_text(storage, "i64") &&
          hako_static_v2_text(type, row->value_kind == HAKO_LLVMC_MAP_VALUE_BOOL ? "Bool" : "Integer");
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

static int hako_static_v2_value_inputs(
    const hako_llvmc_value_projection_v2* row, yyjson_val* ins, uint32_t inputs[16]) {
  if (row->action == HAKO_LLVMC_MAP_ACTION_PHI) {
    yyjson_val* values = hako_static_v2_phi_inputs(ins);
    size_t n = yyjson_arr_size(values);
    if (!n || n > 16) return -1;
    for (size_t i = 0; i < n; i++) {
      uint32_t pred;
      if (!hako_static_v2_phi_input(yyjson_arr_get(values, i), &inputs[i], &pred)) return -1;
    }
    return (int)n;
  }
  if (row->action == HAKO_LLVMC_MAP_ACTION_SELECT)
    return hako_static_v2_u32(yyjson_obj_get(ins, "then_val"), &inputs[0]) &&
        hako_static_v2_u32(yyjson_obj_get(ins, "else_val"), &inputs[1]) ? 2 : -1;
  if (hako_static_v2_is_copy(row->action))
    return hako_static_v2_copy_source(row, ins, &inputs[0]) ? 1 : -1;
  if (row->action != HAKO_LLVMC_MAP_ACTION_OPERATION) return 0;
  if (row->operation == HAKO_LLVMC_MAP_PHYSICAL_I64_NOT || row->operation == HAKO_LLVMC_MAP_PHYSICAL_BOOL_NOT)
    return hako_static_v2_operand(ins, "src", "operand", &inputs[0]) ? 1 : -1;
  return hako_static_v2_operand(ins, "lhs", "left", &inputs[0]) &&
      hako_static_v2_operand(ins, "rhs", "right", &inputs[1]) ? 2 : -1;
}

#include "hako_llvmc_ffi_static_v2_domains.inc"
