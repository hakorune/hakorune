// Pure-route unsupported-shape diagnostic owner.
//
// This file owns only diagnostic state and formatting. It must not change route
// selection, lowering eligibility, or fallback behavior.

struct HakoLlvmcPureUnsupportedShapeDiagnostic {
  char first_op[64];
  char reason[64];
  char target_return_type[64];
  char target_shape_reason[128];
  char blocker_symbol[256];
  char blocker_reason[128];
  char callee_symbol[256];
  char receiver_origin_box_name[128];
  char route_id[128];
  char target_result_box_name[128];
  char next_check_hint[256];
  long long first_block;
  long long first_inst;
};

static void hako_llvmc_pure_unsupported_shape_init(
    struct HakoLlvmcPureUnsupportedShapeDiagnostic* diag) {
  if (!diag) return;
  memset(diag, 0, sizeof(*diag));
  diag->first_block = -1;
  diag->first_inst = -1;
}

static const char* hako_llvmc_pure_unsupported_shape_owner_hint(const char* op) {
  if (!op || !op[0]) return "unknown";
  if (strcmp(op, "boxcall") == 0 ||
      strcmp(op, "call") == 0 ||
      strcmp(op, "externcall") == 0 ||
      strcmp(op, "newbox") == 0 ||
      strcmp(op, "copy") == 0 ||
      strcmp(op, "phi") == 0) {
    return "mir_normalizer";
  }
  if (strcmp(op, "mir_call") == 0 ||
      strcmp(op, "module_function") == 0) {
    return "backend_lowering";
  }
  if (strcmp(op, "const") == 0 ||
      strcmp(op, "compare") == 0 ||
      strcmp(op, "static_data_load") == 0 ||
      strcmp(op, "select") == 0 ||
      strcmp(op, "variant_make") == 0 ||
      strcmp(op, "variant_tag") == 0 ||
      strcmp(op, "variant_project") == 0 ||
      strcmp(op, "branch") == 0 ||
      strcmp(op, "jump") == 0 ||
      strcmp(op, "ret") == 0 ||
      strcmp(op, "binop") == 0) {
    return "backend_lowering";
  }
  return "unknown";
}

static void hako_llvmc_copy_diag_field(
    char* dst,
    size_t dst_len,
    const char* value,
    const char* fallback) {
  if (!dst || dst_len == 0) return;
  snprintf(dst, dst_len, "%s", (value && value[0]) ? value : fallback);
}

static void hako_llvmc_pure_unsupported_shape_note_route_detail(
    struct HakoLlvmcPureUnsupportedShapeDiagnostic* diag,
    long long block_id,
    size_t inst_idx,
    const char* op,
    const char* reason,
    const char* target_return_type,
    const char* target_shape_reason,
    const char* target_shape_blocker_symbol,
    const char* target_shape_blocker_reason,
    const char* callee_symbol,
    const char* receiver_origin_box_name,
    const char* route_id,
    const char* target_result_box_name,
    const char* next_check_hint) {
  if (!diag || diag->first_op[0]) return;
  hako_llvmc_copy_diag_field(diag->first_op, sizeof(diag->first_op), op, "unknown");
  hako_llvmc_copy_diag_field(
      diag->reason,
      sizeof(diag->reason),
      reason,
      "no_lowering_variant");
  hako_llvmc_copy_diag_field(
      diag->target_return_type,
      sizeof(diag->target_return_type),
      target_return_type,
      "-");
  hako_llvmc_copy_diag_field(
      diag->target_shape_reason,
      sizeof(diag->target_shape_reason),
      target_shape_reason,
      "-");
  hako_llvmc_copy_diag_field(
      diag->blocker_symbol,
      sizeof(diag->blocker_symbol),
      target_shape_blocker_symbol,
      "-");
  hako_llvmc_copy_diag_field(
      diag->blocker_reason,
      sizeof(diag->blocker_reason),
      target_shape_blocker_reason,
      "-");
  hako_llvmc_copy_diag_field(
      diag->callee_symbol,
      sizeof(diag->callee_symbol),
      callee_symbol ? callee_symbol : target_shape_blocker_symbol,
      "-");
  hako_llvmc_copy_diag_field(
      diag->receiver_origin_box_name,
      sizeof(diag->receiver_origin_box_name),
      receiver_origin_box_name,
      "-");
  hako_llvmc_copy_diag_field(diag->route_id, sizeof(diag->route_id), route_id, "-");
  hako_llvmc_copy_diag_field(
      diag->target_result_box_name,
      sizeof(diag->target_result_box_name),
      target_result_box_name,
      "-");
  hako_llvmc_copy_diag_field(
      diag->next_check_hint,
      sizeof(diag->next_check_hint),
      next_check_hint ? next_check_hint : target_shape_blocker_reason,
      "-");
  diag->first_block = block_id;
  diag->first_inst = (long long)inst_idx;
}

static void hako_llvmc_pure_unsupported_shape_note_detail(
    struct HakoLlvmcPureUnsupportedShapeDiagnostic* diag,
    long long block_id,
    size_t inst_idx,
    const char* op,
    const char* reason,
    const char* target_return_type,
    const char* target_shape_reason,
    const char* target_shape_blocker_symbol,
    const char* target_shape_blocker_reason) {
  hako_llvmc_pure_unsupported_shape_note_route_detail(
      diag,
      block_id,
      inst_idx,
      op,
      reason,
      target_return_type,
      target_shape_reason,
      target_shape_blocker_symbol,
      target_shape_blocker_reason,
      target_shape_blocker_symbol,
      NULL,
      NULL,
      NULL,
      target_shape_blocker_reason);
}

static void hako_llvmc_pure_unsupported_shape_note(
    struct HakoLlvmcPureUnsupportedShapeDiagnostic* diag,
    long long block_id,
    size_t inst_idx,
    const char* op,
    const char* reason,
    const char* target_shape_reason) {
  hako_llvmc_pure_unsupported_shape_note_detail(
      diag,
      block_id,
      inst_idx,
      op,
      reason,
      NULL,
      target_shape_reason,
      NULL,
      NULL);
}

static void hako_llvmc_pure_unsupported_shape_emit(
    const struct HakoLlvmcPureUnsupportedShapeDiagnostic* diag) {
  char block_buf[32];
  char inst_buf[32];
  const char* op = diag && diag->first_op[0] ? diag->first_op : "unknown";
  const char* reason = diag && diag->reason[0] ? diag->reason : "no_lowering_variant";
  if (!hako_llvmc_route_trace_enabled()) return;
  if (diag && diag->first_block >= 0) {
    snprintf(block_buf, sizeof(block_buf), "%lld", diag->first_block);
  } else {
    snprintf(block_buf, sizeof(block_buf), "-");
  }
  if (diag && diag->first_inst >= 0) {
    snprintf(inst_buf, sizeof(inst_buf), "%lld", diag->first_inst);
  } else {
    snprintf(inst_buf, sizeof(inst_buf), "-");
  }
  fprintf(
      stderr,
      "[llvm-pure/unsupported-shape] recipe=pure-first first_block=%s first_inst=%s first_op=%s owner_hint=%s reason=%s target_return_type=%s target_shape_reason=%s target_shape_blocker_symbol=%s target_shape_blocker_reason=%s callee_symbol=%s receiver_origin_box_name=%s route_id=%s target_result_box_name=%s next_check_hint=%s\n",
      block_buf,
      inst_buf,
      op,
      hako_llvmc_pure_unsupported_shape_owner_hint(op),
      reason,
      diag && diag->target_return_type[0] ? diag->target_return_type : "-",
      diag && diag->target_shape_reason[0] ? diag->target_shape_reason : "-",
      diag && diag->blocker_symbol[0] ? diag->blocker_symbol : "-",
      diag && diag->blocker_reason[0] ? diag->blocker_reason : "-",
      diag && diag->callee_symbol[0] ? diag->callee_symbol : "-",
      diag && diag->receiver_origin_box_name[0] ? diag->receiver_origin_box_name : "-",
      diag && diag->route_id[0] ? diag->route_id : "-",
      diag && diag->target_result_box_name[0] ? diag->target_result_box_name : "-",
      diag && diag->next_check_hint[0] ? diag->next_check_hint : "-");
}

static int hako_llvmc_pure_unsupported_shape_set_error(
    char** err_out,
    const struct HakoLlvmcPureUnsupportedShapeDiagnostic* diag) {
  char msg[1024];
  char block_buf[32];
  char inst_buf[32];
  const char* op = diag && diag->first_op[0] ? diag->first_op : "unknown";
  const char* reason = diag && diag->reason[0] ? diag->reason : "no_lowering_variant";
  const char* callee = diag && diag->callee_symbol[0] ? diag->callee_symbol : "-";
  const char* hint = diag && diag->next_check_hint[0]
      ? diag->next_check_hint
      : "rerun_with_NYASH_LLVM_ROUTE_TRACE=1";
  if (diag && diag->first_block >= 0) {
    snprintf(block_buf, sizeof(block_buf), "%lld", diag->first_block);
  } else {
    snprintf(block_buf, sizeof(block_buf), "-");
  }
  if (diag && diag->first_inst >= 0) {
    snprintf(inst_buf, sizeof(inst_buf), "%lld", diag->first_inst);
  } else {
    snprintf(inst_buf, sizeof(inst_buf), "-");
  }
  snprintf(
      msg,
      sizeof(msg),
      "unsupported pure shape for current backend recipe; first_block=%s first_inst=%s first_op=%s owner_hint=%s reason=%s callee_symbol=%s next_check_hint=%s",
      block_buf,
      inst_buf,
      op,
      hako_llvmc_pure_unsupported_shape_owner_hint(op),
      reason,
      callee,
      hint);
  return set_err_owned(err_out, msg);
}
