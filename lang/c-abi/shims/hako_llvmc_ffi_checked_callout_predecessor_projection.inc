/* Exact MIR-edge to physical-predecessor projection for selected C1 lowering. */
#ifndef HAKO_LLVM_C_CHECKED_CALLOUT_PREDECESSOR_PROJECTION_INC
#define HAKO_LLVM_C_CHECKED_CALLOUT_PREDECESSOR_PROJECTION_INC

enum { HAKO_LLVMC_C1_PREDECESSOR_ROW_CAP = 16 };

struct HakoLlvmcC1PredecessorRowV1 {
  long long mir_predecessor;
  long long mir_successor;
  long long instruction_index;
  char physical_label[96];
  unsigned char emitted;
};

struct HakoLlvmcC1PredecessorProjectionV1 {
  struct HakoLlvmcC1PredecessorRowV1 rows[HAKO_LLVMC_C1_PREDECESSOR_ROW_CAP];
  size_t row_count;
  int enabled;
};

static int hako_llvmc_c1_projection_block_exists_v1(
    yyjson_val* blocks, long long target) {
  if (!(blocks && yyjson_is_arr(blocks))) return 0;
  for (size_t i = 0; i < yyjson_arr_size(blocks); i++) {
    int ok = 0;
    yyjson_val* block = yyjson_arr_get(blocks, i);
    long long block_id = hako_llvmc_c1_block_id(block, &ok);
    if (ok && block_id == target) return 1;
  }
  return 0;
}

static int hako_llvmc_c1_predecessor_add_v1(
    struct HakoLlvmcC1PredecessorProjectionV1* projection,
    long long predecessor, long long successor, long long instruction_index,
    const char* physical_label) {
  if (!projection || predecessor < 0 || successor < 0 ||
      instruction_index < 0 || !physical_label || !physical_label[0] ||
      projection->row_count >= HAKO_LLVMC_C1_PREDECESSOR_ROW_CAP) return 0;
  for (size_t i = 0; i < projection->row_count; i++) {
    if (projection->rows[i].mir_predecessor == predecessor &&
        projection->rows[i].mir_successor == successor) {
      return 0;
    }
  }
  struct HakoLlvmcC1PredecessorRowV1* row =
      &projection->rows[projection->row_count++];
  row->mir_predecessor = predecessor;
  row->mir_successor = successor;
  row->instruction_index = instruction_index;
  snprintf(row->physical_label, sizeof(row->physical_label), "%s", physical_label);
  return 1;
}

static int hako_llvmc_c1_prepare_predecessor_projection_v1(
    yyjson_val* fn, yyjson_val* metadata, int enabled,
    struct HakoLlvmcC1PredecessorProjectionV1* projection) {
  yyjson_val* blocks = fn ? yyjson_obj_get(fn, "blocks") : NULL;
  if (!projection) return 0;
  memset(projection, 0, sizeof(*projection));
  projection->enabled = enabled ? 1 : 0;
  if (!projection->enabled) return 1;
  if (!(metadata && blocks && yyjson_is_arr(blocks))) return 0;
  for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    yyjson_val* block = yyjson_arr_get(blocks, bi);
    yyjson_val* instructions = block ? yyjson_obj_get(block, "instructions") : NULL;
    int block_ok = 0;
    long long block_id = hako_llvmc_c1_block_id(block, &block_ok);
    if (!block_ok || !(instructions && yyjson_is_arr(instructions))) return 0;
    for (size_t ii = 0; ii < yyjson_arr_size(instructions); ii++) {
      yyjson_val* instruction = yyjson_arr_get(instructions, ii);
      const char* op = hako_llvmc_c1_text(instruction, "op");
      if (!op) return 0;
      if (!strcmp(op, "checked_callout")) {
        int ok = 0;
        long long site = hako_llvmc_c1_int(instruction, "site_id", &ok);
        long long normal = hako_llvmc_c1_int(instruction, "normal", &ok);
        long long fault = hako_llvmc_c1_int(instruction, "fault", &ok);
        char normal_label[96];
        char fault_label[96];
        if (!ok || !hako_llvmc_c1_call_for_site(metadata, site) ||
            !hako_llvmc_c1_projection_block_exists_v1(blocks, normal) ||
            !hako_llvmc_c1_projection_block_exists_v1(blocks, fault) ||
            !hako_llvmc_c1_format_predecessor_label_v1(
                normal_label, sizeof(normal_label), "c1_validate_normal",
                site, block_id, ii) ||
            !hako_llvmc_c1_format_predecessor_label_v1(
                fault_label, sizeof(fault_label), "c1_validate_fault",
                site, block_id, ii) ||
            !hako_llvmc_c1_predecessor_add_v1(
                projection, block_id, normal, (long long)ii, normal_label) ||
            !hako_llvmc_c1_predecessor_add_v1(
                projection, block_id, fault, (long long)ii, fault_label)) return 0;
      } else if (!strcmp(op, "checked_callout_end")) {
        yyjson_val* terminator = yyjson_arr_get(
            instructions, yyjson_arr_size(instructions) - 1);
        const char* terminator_op = hako_llvmc_c1_text(terminator, "op");
        char label[96];
        if (!terminator_op || !hako_llvmc_c1_format_end_predecessor_label_v1(
                label, sizeof(label), block_id, ii)) return 0;
        if (!strcmp(terminator_op, "jump")) {
          int ok = 0;
          long long target = hako_llvmc_c1_int(terminator, "target", &ok);
          if (!ok || !hako_llvmc_c1_projection_block_exists_v1(blocks, target) ||
              !hako_llvmc_c1_predecessor_add_v1(
                  projection, block_id, target, (long long)ii, label)) return 0;
        } else if (!strcmp(terminator_op, "branch")) {
          int ok = 0;
          long long then_target = hako_llvmc_c1_int(terminator, "then", &ok);
          long long else_target = hako_llvmc_c1_int(terminator, "else", &ok);
          if (!ok ||
              !hako_llvmc_c1_projection_block_exists_v1(blocks, then_target) ||
              !hako_llvmc_c1_projection_block_exists_v1(blocks, else_target) ||
              !hako_llvmc_c1_predecessor_add_v1(
                  projection, block_id, then_target, (long long)ii, label) ||
              !hako_llvmc_c1_predecessor_add_v1(
                  projection, block_id, else_target, (long long)ii, label)) return 0;
        } else if (strcmp(terminator_op, "ret") != 0 &&
                   strcmp(terminator_op, "return") != 0 &&
                   strcmp(terminator_op, "checked_callout_fault") != 0) {
          return 0;
        }
      }
    }
  }
  return projection->row_count >= 4;
}

static const char* hako_llvmc_c1_predecessor_lookup_v1(
    const struct HakoLlvmcC1PredecessorProjectionV1* projection,
    long long predecessor, long long successor) {
  if (!projection || !projection->enabled) return NULL;
  for (size_t i = 0; i < projection->row_count; i++) {
    if (projection->rows[i].mir_predecessor == predecessor &&
        projection->rows[i].mir_successor == successor) {
      return projection->rows[i].physical_label;
    }
  }
  return NULL;
}

static int hako_llvmc_c1_predecessor_note_emission_v1(
    struct HakoLlvmcC1PredecessorProjectionV1* projection,
    long long predecessor, long long instruction_index, size_t expected_rows) {
  size_t matched = 0;
  if (!projection || !projection->enabled) return 1;
  for (size_t i = 0; i < projection->row_count; i++) {
    struct HakoLlvmcC1PredecessorRowV1* row = &projection->rows[i];
    if (row->mir_predecessor == predecessor &&
        row->instruction_index == instruction_index) {
      if (row->emitted) return 0;
      row->emitted = 1;
      matched++;
    }
  }
  return matched == expected_rows;
}

static size_t hako_llvmc_c1_predecessor_site_row_count_v1(
    const struct HakoLlvmcC1PredecessorProjectionV1* projection,
    long long predecessor, long long instruction_index) {
  size_t count = 0;
  if (!projection || !projection->enabled) return 0;
  for (size_t i = 0; i < projection->row_count; i++) {
    if (projection->rows[i].mir_predecessor == predecessor &&
        projection->rows[i].instruction_index == instruction_index) count++;
  }
  return count;
}

static int hako_llvmc_c1_predecessor_projection_complete_v1(
    const struct HakoLlvmcC1PredecessorProjectionV1* projection) {
  if (!projection || !projection->enabled) return 1;
  if (projection->row_count < 4) return 0;
  for (size_t i = 0; i < projection->row_count; i++) {
    if (!projection->rows[i].emitted) return 0;
  }
  return 1;
}

#endif
