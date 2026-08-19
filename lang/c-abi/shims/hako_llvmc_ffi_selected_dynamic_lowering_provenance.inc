/* Default-off event sink for issuer-emitted selected Dynamic origins. */
#ifndef HAKO_LLVM_C_SELECTED_DYNAMIC_LOWERING_PROVENANCE_INC
#define HAKO_LLVM_C_SELECTED_DYNAMIC_LOWERING_PROVENANCE_INC

#ifndef HAKO_LLVMC_C1_PROVENANCE_ENABLED_V1
#define HAKO_LLVMC_C1_PROVENANCE_ENABLED_V1 0
#endif

#ifndef HAKO_LLVMC_C1_PROVENANCE_EVENT_V1
#define HAKO_LLVMC_C1_PROVENANCE_EVENT_V1(                           \
    entity, mir_block, mir_instruction, mir_arm, mir_target,          \
    llvm_from, llvm_to, disposition, reason) (0)
#endif

#ifndef HAKO_LLVMC_C1_LOWERED_LLVM_EVIDENCE_V1
#define HAKO_LLVMC_C1_LOWERED_LLVM_EVIDENCE_V1(path, err_out) (0)
#endif

static int hako_llvmc_c1_provenance_event_v1(
    const char* entity, long long mir_block, long long mir_instruction,
    const char* mir_arm, long long mir_target, const char* llvm_from,
    const char* llvm_to, const char* disposition, const char* reason) {
  if (!HAKO_LLVMC_C1_PROVENANCE_ENABLED_V1) return 0;
  return HAKO_LLVMC_C1_PROVENANCE_EVENT_V1(
      entity, mir_block, mir_instruction, mir_arm, mir_target,
      llvm_from, llvm_to, disposition, reason);
}

static int hako_llvmc_c1_provenance_block_v1(
    long long bid, long long ii, const char* llvm_block,
    const char* disposition, const char* reason) {
  return hako_llvmc_c1_provenance_event_v1(
      "block", bid, ii, "none", -1, llvm_block, "", disposition, reason);
}

static int hako_llvmc_c1_provenance_internal_edge_v1(
    long long bid, long long ii, const char* llvm_from,
    const char* llvm_to, const char* disposition, const char* reason) {
  return hako_llvmc_c1_provenance_event_v1(
      "edge", bid, ii, "none", -1, llvm_from, llvm_to,
      disposition, reason);
}

static int hako_llvmc_c1_provenance_mir_edge_v1(
    long long bid, long long ii, const char* arm, long long target,
    const char* llvm_from, const char* llvm_to,
    const char* disposition, const char* reason) {
  return hako_llvmc_c1_provenance_event_v1(
      "edge", bid, ii, arm, target, llvm_from, llvm_to,
      disposition, reason);
}

static int hako_llvmc_c1_provenance_emitted_branch_v1(
    long long bid, long long ii, const char* source,
    long long then_block, long long else_block) {
  char then_label[48];
  char else_label[48];
  if (snprintf(then_label, sizeof(then_label), "bb%lld", then_block) <= 0 ||
      snprintf(else_label, sizeof(else_label), "bb%lld", else_block) <= 0) return 0;
  return hako_llvmc_c1_provenance_mir_edge_v1(
             bid, ii, "then", then_block, source, then_label,
             "preserved", "branch") == 0 &&
         hako_llvmc_c1_provenance_mir_edge_v1(
             bid, ii, "else", else_block, source, else_label,
             "preserved", "branch") == 0;
}

static int hako_llvmc_c1_provenance_emitted_jump_v1(
    long long bid, long long ii, const char* source, long long target) {
  char target_label[48];
  if (snprintf(target_label, sizeof(target_label), "bb%lld", target) <= 0) return 0;
  return hako_llvmc_c1_provenance_mir_edge_v1(
             bid, ii, "target", target, source, target_label,
             "preserved", "jump") == 0;
}

#endif
