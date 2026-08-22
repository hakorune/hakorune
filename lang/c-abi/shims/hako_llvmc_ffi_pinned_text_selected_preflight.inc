/*
 * Strict, effect-free consumer for one Rust-issued pinned-Text lifecycle
 * candidate.  This validates transport coherence only.  It emits no LLVM,
 * opens no backend session, and never reconstructs source meaning.
 */
#ifndef HAKO_LLVM_C_PINNED_TEXT_SELECTED_PREFLIGHT_INC
#define HAKO_LLVM_C_PINNED_TEXT_SELECTED_PREFLIGHT_INC

enum HakoPtfSelectedAccessKind {
  HAKO_PTFC_ACCESS_NONE = 0,
  HAKO_PTFC_ACCESS_BYTE_LEN = 1,
  HAKO_PTFC_ACCESS_UTF8_WIDTH_AT = 2,
  HAKO_PTFC_ACCESS_SCALAR_EQ_TEXT = 3,
};

struct HakoPtfSelectedPlanView {
  long long dst;
  enum HakoPtfSelectedAccessKind kind;
  long long lhs_root;
  long long rhs_root;
  long long byte_offset;
  long long width;
};

struct HakoPtfSelectedCandidateView {
  struct HakoPtfCarrierView carrier;
  long long parameter_value_ids[1024];
  struct HakoPtfSelectedPlanView plans[128];
  long long scalar_branch_block;
  long long scalar_branch_then;
  long long scalar_branch_else;
};

static long long hako_llvmc_ptfc_count_integer_occurrences(
    yyjson_val* value,
    long long needle) {
  long long count = 0;
  if (!value) return 0;
  if (yyjson_is_int(value)) return yyjson_get_sint(value) == needle ? 1 : 0;
  if (yyjson_is_arr(value)) {
    size_t index;
    size_t maximum;
    yyjson_val* child;
    yyjson_arr_foreach(value, index, maximum, child) {
      count += hako_llvmc_ptfc_count_integer_occurrences(child, needle);
    }
  } else if (yyjson_is_obj(value)) {
    size_t index;
    size_t maximum;
    yyjson_val* key;
    yyjson_val* child;
    yyjson_obj_foreach(value, index, maximum, key, child) {
      (void)key;
      count += hako_llvmc_ptfc_count_integer_occurrences(child, needle);
    }
  }
  return count;
}

static int hako_llvmc_ptfc_block_exists(
    const hako_llvmc_generic_pure_program_view* program,
    long long wanted) {
  for (size_t bi = 0; bi < program->entry.block_count; bi++) {
    long long block_id = -1;
    if (hako_llvmc_ptfc_nonnegative(
            yyjson_arr_get(program->entry.blocks, bi), "id", &block_id) &&
        block_id == wanted) {
      return 1;
    }
  }
  return 0;
}

static int hako_llvmc_ptfc_owner_payload_matches(
    yyjson_val* instruction,
    const struct HakoPtfCarrierView* carrier) {
  yyjson_val* owner = instruction ? yyjson_obj_get(instruction, "owner") : NULL;
  long long stamp = 0;
  return carrier &&
      hako_llvmc_ptfc_owner_matches(
          owner, carrier->compilation_brand, carrier->owner_slot) &&
      hako_llvmc_ptfc_nonnegative(instruction, "plan_stamp", &stamp) &&
      stamp == carrier->plan_stamp;
}

static int hako_llvmc_ptfc_block_is_finish_site(
    const struct HakoPtfCarrierView* carrier,
    long long block_id) {
  if (!carrier) return 0;
  for (long long i = 0; i < carrier->normal_exit_count; i++) {
    if (carrier->finish_sites[i] == block_id) return 1;
  }
  return 0;
}

static int hako_llvmc_ptfc_parse_selected_plan(
    yyjson_val* instruction,
    const struct HakoPtfCarrierView* carrier,
    struct HakoPtfSelectedCandidateView* out,
    unsigned char* seen,
    char** err_out) {
  static const char* const instruction_keys[] = {
      "op", "dst", "plan", "plan_stamp", "access"};
  static const char* const byte_len_keys[] = {"kind", "root"};
  static const char* const width_keys[] = {"kind", "root", "byte_offset"};
  static const char* const equality_keys[] = {
      "kind", "lhs_root", "lhs_byte_offset", "lhs_width", "rhs_root"};
  yyjson_val* access = yyjson_obj_get(instruction, "access");
  struct HakoPtfSelectedPlanView row;
  long long plan = 0;
  long long stamp = 0;
  memset(&row, 0, sizeof(row));
  row.lhs_root = -1;
  row.rhs_root = -1;
  row.byte_offset = -1;
  row.width = -1;
  if (!hako_llvmc_ptfc_keys_only(instruction, instruction_keys, 5) ||
      !hako_llvmc_ptfc_nonnegative(instruction, "dst", &row.dst) ||
      !hako_llvmc_ptfc_nonnegative(instruction, "plan", &plan) ||
      !hako_llvmc_ptfc_nonnegative(instruction, "plan_stamp", &stamp) ||
      stamp != carrier->plan_stamp || plan < 0 || plan >= carrier->plan_count ||
      plan >= 128 || seen[plan]) {
    return set_err_owned(err_out, "pinned Text selected plan header mismatch");
  }
  if (plan == 0) {
    if (!hako_llvmc_ptfc_keys_only(access, byte_len_keys, 2) ||
        !hako_llvmc_ptfc_text(access, "kind", "byte_len") ||
        !hako_llvmc_ptfc_nonnegative(access, "root", &row.lhs_root) ||
        row.lhs_root != 0) {
      return set_err_owned(err_out, "pinned Text selected ByteLen plan mismatch");
    }
    row.kind = HAKO_PTFC_ACCESS_BYTE_LEN;
  } else if (plan == 1) {
    if (!hako_llvmc_ptfc_keys_only(access, width_keys, 3) ||
        !hako_llvmc_ptfc_text(access, "kind", "utf8_width_at") ||
        !hako_llvmc_ptfc_nonnegative(access, "root", &row.lhs_root) ||
        !hako_llvmc_ptfc_nonnegative(access, "byte_offset", &row.byte_offset) ||
        row.lhs_root != 0) {
      return set_err_owned(err_out, "pinned Text selected WidthAt plan mismatch");
    }
    row.kind = HAKO_PTFC_ACCESS_UTF8_WIDTH_AT;
  } else if (plan == 2) {
    if (!hako_llvmc_ptfc_keys_only(access, equality_keys, 5) ||
        !hako_llvmc_ptfc_text(
            access, "kind", "utf8_scalar_slice_eq_whole_text") ||
        !hako_llvmc_ptfc_nonnegative(access, "lhs_root", &row.lhs_root) ||
        !hako_llvmc_ptfc_nonnegative(
            access, "lhs_byte_offset", &row.byte_offset) ||
        !hako_llvmc_ptfc_nonnegative(access, "lhs_width", &row.width) ||
        !hako_llvmc_ptfc_nonnegative(access, "rhs_root", &row.rhs_root) ||
        row.lhs_root != 0 || row.rhs_root != 1) {
      return set_err_owned(err_out, "pinned Text selected scalar equality plan mismatch");
    }
    row.kind = HAKO_PTFC_ACCESS_SCALAR_EQ_TEXT;
  } else {
    return set_err_owned(err_out, "unknown pinned Text selected plan index");
  }
  seen[plan] = 1;
  out->plans[plan] = row;
  return 0;
}

static int hako_llvmc_ptfc_preflight_selected_candidate(
    const hako_llvmc_generic_pure_program_view* program,
    struct HakoPtfSelectedCandidateView* out,
    char** err_out) {
  static const char* const block_keys[] = {"id", "instructions"};
  static const char* const enter_keys[] = {
      "op", "owner", "plan_stamp", "normal", "trap"};
  static const char* const finish_keys[] = {"op", "owner", "plan_stamp"};
  static const char* const trap_keys[] = {"op", "owner", "plan_stamp"};
  static const char* const return_keys[] = {"op", "value"};
  static const char* const branch_keys[] = {"op", "cond", "then", "else"};
  yyjson_val* metadata;
  yyjson_val* frame;
  yyjson_val* carrier;
  yyjson_val* params;
  unsigned char seen_plans[128];
  long long enter_count = 0;
  long long trap_count = 0;
  long long finish_count = 0;
  long long return_count = 0;
  long long scalar_branch_count = 0;
  int source_block_found = 0;
  int normal_block_found = 0;
  int trap_block_found = 0;

  if (!program || !out || program->function_count != 1 ||
      program->entry_index != 0 || program->selected_dynamic) {
    return set_err_owned(err_out, "pinned Text selected preflight requires one function");
  }
  memset(out, 0, sizeof(*out));
  out->scalar_branch_block = -1;
  out->scalar_branch_then = -1;
  out->scalar_branch_else = -1;
  memset(seen_plans, 0, sizeof(seen_plans));
  metadata = program->entry.metadata;
  frame = metadata ? yyjson_obj_get(metadata, "pinned_text_backend_frame_v1") : NULL;
  carrier = metadata ? yyjson_obj_get(metadata, "pinned_text_residence_carrier_v1") : NULL;
  if (!frame || !carrier) {
    return set_err_owned(err_out, "pinned Text selected frame/carrier metadata is missing");
  }
  if (hako_llvmc_ptfc_parse_carrier(frame, carrier, &out->carrier, err_out) != 0) {
    return -1;
  }
  if (out->carrier.root_count != 2 ||
      out->carrier.physical_callable_lane_count != 4 ||
      out->carrier.plan_count != 3 ||
      out->carrier.slot_lanes[0] != 0 ||
      out->carrier.generation_lanes[0] != 1 ||
      out->carrier.slot_lanes[1] != 2 ||
      out->carrier.generation_lanes[1] != 3) {
    return set_err_owned(err_out, "pinned Text selected physical shape mismatch");
  }
  params = yyjson_obj_get(program->entry.fn, "params");
  if (!(params && yyjson_is_arr(params)) ||
      yyjson_arr_size(params) !=
          (size_t)out->carrier.physical_callable_lane_count) {
    return set_err_owned(err_out, "pinned Text selected parameter census mismatch");
  }
  for (size_t i = 0; i < yyjson_arr_size(params); i++) {
    if (!hako_llvmc_ptfc_int_value(
            yyjson_arr_get(params, i), &out->parameter_value_ids[i])) {
      return set_err_owned(err_out, "invalid pinned Text selected parameter ValueId");
    }
    for (size_t j = 0; j < i; j++) {
      if (out->parameter_value_ids[j] == out->parameter_value_ids[i]) {
        return set_err_owned(err_out, "duplicate pinned Text selected parameter ValueId");
      }
    }
  }

  for (size_t bi = 0; bi < program->entry.block_count; bi++) {
    yyjson_val* block = yyjson_arr_get(program->entry.blocks, bi);
    yyjson_val* instructions;
    long long block_id = 0;
    if (!hako_llvmc_ptfc_keys_only(block, block_keys, 2) ||
        !hako_llvmc_ptfc_nonnegative(block, "id", &block_id)) {
      return set_err_owned(err_out, "invalid pinned Text selected block");
    }
    for (size_t prior = 0; prior < bi; prior++) {
      long long prior_id = 0;
      hako_llvmc_ptfc_nonnegative(
          yyjson_arr_get(program->entry.blocks, prior), "id", &prior_id);
      if (prior_id == block_id) {
        return set_err_owned(err_out, "duplicate pinned Text selected block id");
      }
    }
    if (block_id == out->carrier.enter_source) source_block_found = 1;
    if (block_id == out->carrier.normal_landing) normal_block_found = 1;
    if (block_id == out->carrier.trap_landing) trap_block_found = 1;
    instructions = yyjson_obj_get(block, "instructions");
    if (!(instructions && yyjson_is_arr(instructions))) {
      return set_err_owned(err_out, "pinned Text selected block has no instructions");
    }
    for (size_t ii = 0; ii < yyjson_arr_size(instructions); ii++) {
      yyjson_val* instruction = yyjson_arr_get(instructions, ii);
      yyjson_val* op_value = instruction ? yyjson_obj_get(instruction, "op") : NULL;
      const char* op = op_value && yyjson_is_str(op_value)
          ? yyjson_get_str(op_value) : NULL;
      if (!op) return set_err_owned(err_out, "pinned Text selected instruction has no op");
      if (!strcmp(op, "pinned_text_op")) {
        if (block_id == out->carrier.trap_landing ||
            hako_llvmc_ptfc_block_is_finish_site(&out->carrier, block_id)) {
          return set_err_owned(err_out, "pinned Text selected plan is in a lifecycle block");
        }
        if (hako_llvmc_ptfc_parse_selected_plan(
                instruction, &out->carrier, out, seen_plans, err_out) != 0) {
          return -1;
        }
      } else if (!strcmp(op, "pinned_text_residence_enter")) {
        long long normal = 0;
        long long trap = 0;
        if (!hako_llvmc_ptfc_keys_only(instruction, enter_keys, 5) ||
            !hako_llvmc_ptfc_owner_payload_matches(instruction, &out->carrier) ||
            !hako_llvmc_ptfc_nonnegative(instruction, "normal", &normal) ||
            !hako_llvmc_ptfc_nonnegative(instruction, "trap", &trap) ||
            block_id != out->carrier.enter_source ||
            normal != out->carrier.normal_landing ||
            trap != out->carrier.trap_landing ||
            yyjson_arr_size(instructions) != 1 || ii != 0) {
          return set_err_owned(err_out, "pinned Text selected Enter site mismatch");
        }
        enter_count++;
      } else if (!strcmp(op, "pinned_text_residence_trap")) {
        if (!hako_llvmc_ptfc_keys_only(instruction, trap_keys, 3) ||
            !hako_llvmc_ptfc_owner_payload_matches(instruction, &out->carrier) ||
            block_id != out->carrier.trap_landing ||
            yyjson_arr_size(instructions) != 1 || ii != 0) {
          return set_err_owned(err_out, "pinned Text selected Trap site mismatch");
        }
        trap_count++;
      } else if (!strcmp(op, "pinned_text_residence_finish")) {
        yyjson_val* next = ii + 1 < yyjson_arr_size(instructions)
            ? yyjson_arr_get(instructions, ii + 1) : NULL;
        yyjson_val* next_op_value = next ? yyjson_obj_get(next, "op") : NULL;
        const char* next_op = next_op_value && yyjson_is_str(next_op_value)
            ? yyjson_get_str(next_op_value) : NULL;
        if (!hako_llvmc_ptfc_keys_only(instruction, finish_keys, 3) ||
            !hako_llvmc_ptfc_owner_payload_matches(instruction, &out->carrier) ||
            !hako_llvmc_ptfc_block_is_finish_site(&out->carrier, block_id) ||
            !next_op || strcmp(next_op, "ret") != 0 ||
            ii + 2 != yyjson_arr_size(instructions)) {
          return set_err_owned(err_out, "pinned Text selected Finish site mismatch");
        }
        finish_count++;
      } else if (!strcmp(op, "ret")) {
        yyjson_val* prior = ii > 0 ? yyjson_arr_get(instructions, ii - 1) : NULL;
        yyjson_val* prior_op_value = prior ? yyjson_obj_get(prior, "op") : NULL;
        const char* prior_op = prior_op_value && yyjson_is_str(prior_op_value)
            ? yyjson_get_str(prior_op_value) : NULL;
        if (!hako_llvmc_ptfc_keys_only(instruction, return_keys, 2) ||
            !hako_llvmc_ptfc_nonnegative(instruction, "value", NULL) ||
            !hako_llvmc_ptfc_block_is_finish_site(&out->carrier, block_id) ||
            !prior_op || strcmp(prior_op, "pinned_text_residence_finish") != 0 ||
            ii + 1 != yyjson_arr_size(instructions)) {
          return set_err_owned(err_out, "pinned Text selected Return coverage mismatch");
        }
        return_count++;
      } else if (!strcmp(op, "branch")) {
        long long cond = -1;
        long long then_block = -1;
        long long else_block = -1;
        if (seen_plans[2] &&
            hako_llvmc_ptfc_nonnegative(instruction, "cond", &cond) &&
            cond == out->plans[2].dst) {
          yyjson_val* prior = ii > 0
              ? yyjson_arr_get(instructions, ii - 1) : NULL;
          long long prior_plan = -1;
          if (!hako_llvmc_ptfc_keys_only(instruction, branch_keys, 4) ||
              !hako_llvmc_ptfc_nonnegative(
                  instruction, "then", &then_block) ||
              !hako_llvmc_ptfc_nonnegative(
                  instruction, "else", &else_block) ||
              !prior ||
              !hako_llvmc_ptfc_text(prior, "op", "pinned_text_op") ||
              !hako_llvmc_ptfc_nonnegative(prior, "plan", &prior_plan) ||
              prior_plan != 2 || ii + 1 != yyjson_arr_size(instructions) ||
              then_block == else_block) {
            return set_err_owned(
                err_out, "pinned Text selected scalar Branch relation mismatch");
          }
          out->scalar_branch_block = block_id;
          out->scalar_branch_then = then_block;
          out->scalar_branch_else = else_block;
          scalar_branch_count++;
        }
      } else if (!strncmp(op, "pinned_text_", strlen("pinned_text_"))) {
        return set_err_owned(err_out, "unknown pinned Text selected operation");
      }
    }
  }

  if (!source_block_found || !normal_block_found || !trap_block_found ||
      enter_count != 1 || trap_count != 1 ||
      finish_count != out->carrier.normal_exit_count ||
      return_count != out->carrier.normal_exit_count) {
    return set_err_owned(err_out, "pinned Text selected lifecycle census mismatch");
  }
  for (long long i = 0; i < out->carrier.plan_count; i++) {
    if (!seen_plans[i]) {
      return set_err_owned(err_out, "missing pinned Text selected plan");
    }
  }
  if (out->plans[2].byte_offset != out->plans[1].byte_offset ||
      out->plans[2].width != out->plans[1].dst) {
    return set_err_owned(err_out, "pinned Text selected plan relation mismatch");
  }
  if (scalar_branch_count != 1 ||
      out->scalar_branch_block < 0 ||
      !hako_llvmc_ptfc_block_exists(program, out->scalar_branch_then) ||
      !hako_llvmc_ptfc_block_exists(program, out->scalar_branch_else) ||
      hako_llvmc_ptfc_count_integer_occurrences(
          program->entry.blocks, out->plans[2].dst) != 2) {
    return set_err_owned(
        err_out, "pinned Text selected scalar Branch sole-use mismatch");
  }
  return 0;
}

#endif
