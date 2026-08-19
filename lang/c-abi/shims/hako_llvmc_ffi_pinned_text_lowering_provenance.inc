/* Compile-time-only origin journal for the selected pinned-Text lowering. */
#ifndef HAKO_LLVM_C_PINNED_TEXT_LOWERING_PROVENANCE_INC
#define HAKO_LLVM_C_PINNED_TEXT_LOWERING_PROVENANCE_INC

#ifndef HAKO_LLVMC_PTFC_PROVENANCE_EVENT_V1
#define HAKO_LLVMC_PTFC_PROVENANCE_EVENT_V1(                         \
    entity, mir_block, mir_instruction, mir_arm, mir_target,          \
    llvm_from, llvm_to, disposition, reason) (0)
#endif

static int hako_llvmc_ptfc_provenance_event_v1(
    const char* entity,
    long long mir_block,
    long long mir_instruction,
    const char* mir_arm,
    long long mir_target,
    const char* llvm_from,
    const char* llvm_to,
    const char* disposition,
    const char* reason) {
  return HAKO_LLVMC_PTFC_PROVENANCE_EVENT_V1(
      entity, mir_block, mir_instruction, mir_arm, mir_target,
      llvm_from, llvm_to, disposition, reason);
}

static int hako_llvmc_ptfc_provenance_label_v1(
    char* output, size_t output_size, const char* stem, long long suffix) {
  int written;
  if (!output || output_size == 0 || !stem) return 0;
  written = snprintf(output, output_size, "%s%lld", stem, suffix);
  return written > 0 && (size_t)written < output_size;
}

static int hako_llvmc_ptfc_provenance_base_block_v1(
    long long bid, long long ii) {
  char label[48];
  if (ii != 0 ||
      !hako_llvmc_ptfc_provenance_label_v1(label, sizeof(label), "bb", bid)) {
    return ii == 0 ? -1 : 0;
  }
  return hako_llvmc_ptfc_provenance_event_v1(
      "block", bid, -1, "none", -1, label, "", "preserved", "base_block");
}

static int hako_llvmc_ptfc_provenance_base_edge_v1(
    long long bid,
    long long ii,
    const char* arm,
    long long target,
    const char* disposition,
    const char* reason) {
  char from[48];
  char to[48];
  if (!hako_llvmc_ptfc_provenance_label_v1(from, sizeof(from), "bb", bid) ||
      !hako_llvmc_ptfc_provenance_label_v1(to, sizeof(to), "bb", target)) {
    return -1;
  }
  return hako_llvmc_ptfc_provenance_event_v1(
      "edge", bid, ii, arm, target, from, to, disposition, reason);
}

static int hako_llvmc_ptfc_provenance_selected_site_v1(
    long long bid,
    long long ii,
    const char* op,
    yyjson_val* instruction,
    const struct HakoPtfSelectedCandidateView* candidate) {
  long long target;
  long long cond = -1;
  if (!strcmp(op, "jump")) {
    return !hako_llvmc_ptfc_nonnegative(instruction, "target", &target) ? -1 :
        hako_llvmc_ptfc_provenance_base_edge_v1(
        bid, ii, "target", target, "preserved", "jump");
  }
  if (!strcmp(op, "branch") &&
      (!hako_llvmc_ptfc_nonnegative(instruction, "cond", &cond) ||
       cond != candidate->plans[2].dst)) {
    long long then_block = -1;
    long long else_block = -1;
    if (!hako_llvmc_ptfc_nonnegative(instruction, "then", &then_block) ||
        !hako_llvmc_ptfc_nonnegative(instruction, "else", &else_block) ||
        hako_llvmc_ptfc_provenance_base_edge_v1(
            bid, ii, "then", then_block, "preserved", "branch") != 0) {
      return -1;
    }
    return hako_llvmc_ptfc_provenance_base_edge_v1(
        bid, ii, "else", else_block, "preserved", "branch");
  }
  return 0;
}

static int hako_llvmc_ptfc_provenance_named_v1(
    const char* entity, long long bid, long long ii,
    const char* llvm_from, const char* llvm_to,
    const char* disposition, const char* reason) {
  return hako_llvmc_ptfc_provenance_event_v1(
      entity, bid, ii, "none", -1, llvm_from, llvm_to, disposition, reason);
}

static int hako_llvmc_ptfc_provenance_width_v1(
    long long bid, long long ii, long long dst) {
  static const char* stems[] = {
      "ptfc_width_ascii_", "ptfc_width_non_ascii_", "ptfc_width_two_",
      "ptfc_width_wide_", "ptfc_width_merge_"};
  char labels[5][64];
  char base[48];
  int i;
  if (!hako_llvmc_ptfc_provenance_label_v1(base, sizeof(base), "bb", bid)) return -1;
  for (i = 0; i < 5; i++) {
    if (!hako_llvmc_ptfc_provenance_label_v1(
            labels[i], sizeof(labels[i]), stems[i], dst) ||
        hako_llvmc_ptfc_provenance_named_v1(
            "block", bid, ii, labels[i], "", "split", "utf8_width_at") != 0) {
      return -1;
    }
  }
#define HAKO_PTFC_WIDTH_EDGE(from, to) \
  if (hako_llvmc_ptfc_provenance_named_v1( \
          "edge", bid, ii, (from), (to), "split", "utf8_width_at") != 0) return -1
  HAKO_PTFC_WIDTH_EDGE(base, labels[0]);
  HAKO_PTFC_WIDTH_EDGE(base, labels[1]);
  HAKO_PTFC_WIDTH_EDGE(labels[0], labels[4]);
  HAKO_PTFC_WIDTH_EDGE(labels[1], labels[2]);
  HAKO_PTFC_WIDTH_EDGE(labels[1], labels[3]);
  HAKO_PTFC_WIDTH_EDGE(labels[2], labels[4]);
  HAKO_PTFC_WIDTH_EDGE(labels[3], labels[4]);
#undef HAKO_PTFC_WIDTH_EDGE
  return 0;
}

static int hako_llvmc_ptfc_provenance_scalar_eq_v1(
    long long bid, long long ii, long long dst, long long width_dst,
    long long then_block, long long else_block) {
  static const char* stems[] = {
      "ptfc_eq_lead_", "ptfc_eq_after_b0_", "ptfc_eq_b1_",
      "ptfc_eq_after_b1_", "ptfc_eq_b2_", "ptfc_eq_after_b2_",
      "ptfc_eq_b3_", "ptfc_eq_false_", "ptfc_eq_true_"};
  static const int edge_pairs[][2] = {
      {0, 1}, {0, 7}, {1, 8}, {1, 2}, {2, 3}, {2, 7},
      {3, 8}, {3, 4}, {4, 5}, {4, 7}, {5, 8}, {5, 6},
      {6, 8}, {6, 7}};
  char labels[9][64];
  char width_merge[64];
  char then_label[48];
  char else_label[48];
  int i;
  if (!hako_llvmc_ptfc_provenance_label_v1(
          width_merge, sizeof(width_merge), "ptfc_width_merge_", width_dst) ||
      !hako_llvmc_ptfc_provenance_label_v1(
          then_label, sizeof(then_label), "bb", then_block) ||
      !hako_llvmc_ptfc_provenance_label_v1(
          else_label, sizeof(else_label), "bb", else_block)) return -1;
  for (i = 0; i < 9; i++) {
    if (!hako_llvmc_ptfc_provenance_label_v1(
            labels[i], sizeof(labels[i]), stems[i], dst) ||
        hako_llvmc_ptfc_provenance_named_v1(
            "block", bid, ii, labels[i], "", "split", "scalar_eq") != 0) {
      return -1;
    }
  }
  if (hako_llvmc_ptfc_provenance_named_v1(
          "edge", bid, ii, width_merge, labels[0], "split", "scalar_eq") != 0 ||
      hako_llvmc_ptfc_provenance_named_v1(
          "edge", bid, ii, width_merge, labels[7], "split", "scalar_eq") != 0) {
    return -1;
  }
  for (i = 0; i < (int)(sizeof(edge_pairs) / sizeof(edge_pairs[0])); i++) {
    if (hako_llvmc_ptfc_provenance_named_v1(
            "edge", bid, ii, labels[edge_pairs[i][0]], labels[edge_pairs[i][1]],
            "split", "scalar_eq") != 0) return -1;
  }
  if (hako_llvmc_ptfc_provenance_event_v1(
          "edge", bid, ii + 1, "else", else_block, labels[7], else_label,
          "split", "direct_continuation") != 0 ||
      hako_llvmc_ptfc_provenance_event_v1(
          "edge", bid, ii + 1, "then", then_block, labels[8], then_label,
          "split", "direct_continuation") != 0) return -1;
  return 0;
}

static int hako_llvmc_ptfc_provenance_emitted_op_v1(
    long long bid, long long ii, const char* op, yyjson_val* instruction,
    const struct HakoPtfSelectedCandidateView* candidate) {
  if (!strcmp(op, "pinned_text_residence_enter")) {
    if (hako_llvmc_ptfc_provenance_base_edge_v1(
            bid, ii, "normal", candidate->carrier.normal_landing,
            "preserved", "residence_enter") != 0) return -1;
    return hako_llvmc_ptfc_provenance_base_edge_v1(
        bid, ii, "trap", candidate->carrier.trap_landing,
        "preserved", "residence_enter");
  }
  if (!strcmp(op, "pinned_text_op")) {
    long long plan_index = -1;
    const struct HakoPtfSelectedPlanView* plan;
    if (!hako_llvmc_ptfc_nonnegative(instruction, "plan", &plan_index) ||
        plan_index >= candidate->carrier.plan_count) return -1;
    plan = &candidate->plans[plan_index];
    if (plan->kind == HAKO_PTFC_ACCESS_UTF8_WIDTH_AT) {
      return hako_llvmc_ptfc_provenance_width_v1(bid, ii, plan->dst);
    }
    if (plan->kind == HAKO_PTFC_ACCESS_SCALAR_EQ_TEXT) {
      return hako_llvmc_ptfc_provenance_scalar_eq_v1(
          bid, ii, plan->dst, plan->width,
          candidate->scalar_branch_then, candidate->scalar_branch_else);
    }
  }
  return 0;
}

#endif
