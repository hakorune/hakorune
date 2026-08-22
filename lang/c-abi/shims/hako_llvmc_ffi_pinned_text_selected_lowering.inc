/* Private textual projection for one preflighted pinned-Text candidate. */
#ifndef HAKO_LLVM_C_PINNED_TEXT_SELECTED_LOWERING_INC
#define HAKO_LLVM_C_PINNED_TEXT_SELECTED_LOWERING_INC

#include "../../../include/nyrt_text_formal_residence_v1.h"

#define HAKO_PTFC_ENTER_DECL_V1 \
  "declare i32 @hako_text_formal_residence_enter_v1(ptr, i32, ptr, i32) nounwind\n"
#define HAKO_PTFC_FINISH_DECL_V1 \
  "declare void @hako_text_formal_residence_finish_or_abort_v1(ptr) nounwind\n"

#include "hako_llvmc_ffi_pinned_text_lowering_provenance.inc"

/* Compile-time benchmark instrumentation only.  Production captures no plan. */
#ifndef HAKO_LLVMC_PTFC_EXACT_LEAF_EVIDENCE_PLAN_V1
#define HAKO_LLVMC_PTFC_EXACT_LEAF_EVIDENCE_PLAN_V1(plan) (0)
#endif

_Static_assert(NYRT_TEXT_RESIDENCE_ENTER_NOUNWIND_V1 == 1,
               "selected Enter projection requires the runtime no-unwind ABI");
_Static_assert(NYRT_TEXT_RESIDENCE_FINISH_OR_ABORT_NOUNWIND_V1 == 1,
               "selected Finish projection requires the runtime no-unwind ABI");
_Static_assert(NYRT_TEXT_RESIDENCE_FINISH_OR_ABORT_NORETURN_V1 == 0,
               "Finish returns after successful residence release");

struct HakoPtfSelectedLlvmDraft {
  FILE* stream;
  unsigned int enter_count;
  unsigned int trap_count;
  unsigned int finish_count;
  unsigned int leaf_count;
  unsigned int scalar_branch_count;
};

struct HakoPtfSelectedLlvmBytes {
  char* data;
  size_t size;
};

static void hako_llvmc_ptfc_release_selected_llvm_bytes(
    struct HakoPtfSelectedLlvmBytes* bytes) {
  if (!bytes) return;
  free(bytes->data);
  bytes->data = NULL;
  bytes->size = 0;
}

static int hako_llvmc_ptfc_open_selected_draft(
    struct HakoPtfSelectedLlvmDraft* draft,
    char** err_out) {
  if (!draft) return set_err_owned(err_out, "missing pinned Text selected draft");
  memset(draft, 0, sizeof(*draft));
  draft->stream = tmpfile();
  if (!draft->stream) {
    return set_err_owned(err_out, "cannot open private pinned Text LLVM draft");
  }
  return 0;
}

static void hako_llvmc_ptfc_discard_selected_draft(
    struct HakoPtfSelectedLlvmDraft* draft) {
  if (!draft || !draft->stream) return;
  fclose(draft->stream);
  draft->stream = NULL;
}

static size_t hako_llvmc_ptfc_count_text(const char* text, const char* needle) {
  size_t count = 0;
  size_t length = needle ? strlen(needle) : 0;
  if (!text || !length) return 0;
  while ((text = strstr(text, needle)) != NULL) {
    count++;
    text += length;
  }
  return count;
}

static int hako_llvmc_ptfc_verify_and_take_selected_llvm(
    struct HakoPtfSelectedLlvmDraft* draft,
    struct HakoPtfSelectedLlvmBytes* bytes,
    char** err_out) {
  char* text = NULL;
  long size = 0;
  int valid = 0;
  if (!bytes) return set_err_owned(err_out, "missing pinned Text LLVM byte owner");
  memset(bytes, 0, sizeof(*bytes));
  if (!draft || !draft->stream || fflush(draft->stream) != 0 ||
      fseek(draft->stream, 0, SEEK_END) != 0 ||
      (size = ftell(draft->stream)) < 0 ||
      fseek(draft->stream, 0, SEEK_SET) != 0) {
    hako_llvmc_ptfc_discard_selected_draft(draft);
    return set_err_owned(err_out, "cannot inspect private pinned Text LLVM draft");
  }
  text = (char*)malloc((size_t)size + 1);
  if (!text || fread(text, 1, (size_t)size, draft->stream) != (size_t)size) {
    free(text);
    hako_llvmc_ptfc_discard_selected_draft(draft);
    return set_err_owned(err_out, "cannot read private pinned Text LLVM draft");
  }
  text[size] = '\0';
  valid = draft->enter_count == 1 && draft->trap_count == 1 &&
      draft->finish_count == 2 && draft->leaf_count == 3 &&
      draft->scalar_branch_count == 1 &&
      hako_llvmc_ptfc_count_text(text, HAKO_PTFC_ENTER_DECL_V1) == 1 &&
      hako_llvmc_ptfc_count_text(text, HAKO_PTFC_FINISH_DECL_V1) == 1 &&
      hako_llvmc_ptfc_count_text(
          text, "declare i32 @hako_text_formal_residence_enter_v1(") == 1 &&
      hako_llvmc_ptfc_count_text(
          text, "declare void @hako_text_formal_residence_finish_or_abort_v1(") == 1 &&
      hako_llvmc_ptfc_count_text(
          text, "call i32 @hako_text_formal_residence_enter_v1") == 1 &&
      hako_llvmc_ptfc_count_text(
          text, "call void @hako_text_formal_residence_finish_or_abort_v1") == 2 &&
      hako_llvmc_ptfc_count_text(text, "call void @llvm.trap()") == 1 &&
      hako_llvmc_ptfc_count_text(text, "ret i64") == 2 &&
      hako_llvmc_ptfc_count_text(text, "; ptfc leaf ") == 3 &&
      hako_llvmc_ptfc_count_text(text, "store i64 %r") == 4 &&
      hako_llvmc_ptfc_count_text(text, "%ptfc_frame = alloca i8") == 1 &&
      hako_llvmc_ptfc_count_text(text, "br i1 %ptfc_enter_ok") == 1 &&
      hako_llvmc_ptfc_count_text(
          text, "call void @llvm.trap()\n  unreachable") == 1 &&
      hako_llvmc_ptfc_count_text(
          text,
          "call void @hako_text_formal_residence_finish_or_abort_v1(ptr %ptfc_frame)\n  ret i64") == 2 &&
      !strstr(text, "memcmp") && !strstr(text, " invoke ") &&
      !strstr(text, "landingpad") && !strstr(text, "callbr");
  hako_llvmc_ptfc_discard_selected_draft(draft);
  if (!valid) {
    free(text);
    return set_err_owned(err_out, "private pinned Text LLVM verification failed");
  }
  bytes->data = text;
  bytes->size = (size_t)size;
  return 0;
}

static int hako_llvmc_ptfc_emit_selected_entry_header(
    FILE* out,
    const hako_llvmc_entry_rune_selection* selection,
    const struct HakoPtfSelectedCandidateView* candidate) {
  const char* symbol = hako_llvmc_effective_entry_symbol(selection);
  if (!out || !candidate || candidate->carrier.physical_callable_lane_count != 4 ||
      !symbol || !symbol[0] || strchr(symbol, '"') || strchr(symbol, '\\')) {
    return 0;
  }
  fprintf(out, "define i64 @\"%s\"(", symbol);
  for (long long i = 0; i < candidate->carrier.physical_callable_lane_count; i++) {
    fprintf(out, "%si64 %%r%lld", i ? ", " : "", candidate->parameter_value_ids[i]);
  }
  fputs(") {\n", out);
  return 1;
}

static void hako_llvmc_ptfc_emit_selected_declarations(FILE* out) {
  fputs(HAKO_PTFC_ENTER_DECL_V1, out);
  fputs(HAKO_PTFC_FINISH_DECL_V1, out);
}

static int hako_llvmc_ptfc_emit_selected_leaf(
    FILE* out,
    const struct HakoPtfSelectedPlanView* plan,
    long long direct_then,
    long long direct_else) {
  if (!out || !plan) return 0;
  if (plan->kind == HAKO_PTFC_ACCESS_BYTE_LEN) {
    fputs("  ; ptfc leaf byte_len\n", out);
    fputs("  %ptfc_subject_ptr_addr = getelementptr i8, ptr %ptfc_frame, i64 32\n", out);
    fputs("  %ptfc_subject_ptr = load ptr, ptr %ptfc_subject_ptr_addr, align 8\n", out);
    fputs("  %ptfc_subject_len_addr = getelementptr i8, ptr %ptfc_frame, i64 40\n", out);
    fputs("  %ptfc_subject_len = load i64, ptr %ptfc_subject_len_addr, align 8\n", out);
    fputs("  %ptfc_needle_ptr_addr = getelementptr i8, ptr %ptfc_frame, i64 48\n", out);
    fputs("  %ptfc_needle_ptr = load ptr, ptr %ptfc_needle_ptr_addr, align 8\n", out);
    fputs("  %ptfc_needle_len_addr = getelementptr i8, ptr %ptfc_frame, i64 56\n", out);
    fputs("  %ptfc_needle_len = load i64, ptr %ptfc_needle_len_addr, align 8\n", out);
    fprintf(out, "  %%r%lld = add i64 %%ptfc_subject_len, 0\n", plan->dst);
    return 1;
  }
  if (plan->kind == HAKO_PTFC_ACCESS_UTF8_WIDTH_AT) {
    fputs("  ; ptfc leaf utf8_width_at\n", out);
    fprintf(out, "  %%ptfc_byte_ptr_%lld = getelementptr i8, ptr %%ptfc_subject_ptr, i64 %%r%lld\n",
            plan->dst, plan->byte_offset);
    fprintf(out, "  %%ptfc_byte_%lld = load i8, ptr %%ptfc_byte_ptr_%lld, align 1\n",
            plan->dst, plan->dst);
    fprintf(out, "  %%ptfc_ascii_%lld = icmp ult i8 %%ptfc_byte_%lld, -128\n",
            plan->dst, plan->dst);
    fprintf(out, "  br i1 %%ptfc_ascii_%lld, label %%ptfc_width_ascii_%lld, label %%ptfc_width_non_ascii_%lld\n",
            plan->dst, plan->dst, plan->dst);
    fprintf(out, "ptfc_width_ascii_%lld:\n  br label %%ptfc_width_merge_%lld\n",
            plan->dst, plan->dst);
    fprintf(out, "ptfc_width_non_ascii_%lld:\n", plan->dst);
    fprintf(out, "  %%ptfc_two_%lld = icmp ult i8 %%ptfc_byte_%lld, -32\n",
            plan->dst, plan->dst);
    fprintf(out, "  br i1 %%ptfc_two_%lld, label %%ptfc_width_two_%lld, label %%ptfc_width_wide_%lld\n",
            plan->dst, plan->dst, plan->dst);
    fprintf(out, "ptfc_width_two_%lld:\n  br label %%ptfc_width_merge_%lld\n",
            plan->dst, plan->dst);
    fprintf(out, "ptfc_width_wide_%lld:\n", plan->dst);
    fprintf(out, "  %%ptfc_three_%lld = icmp ult i8 %%ptfc_byte_%lld, -16\n",
            plan->dst, plan->dst);
    fprintf(out, "  %%ptfc_wide_width_%lld = select i1 %%ptfc_three_%lld, i64 3, i64 4\n",
            plan->dst, plan->dst);
    fprintf(out, "  br label %%ptfc_width_merge_%lld\n", plan->dst);
    fprintf(out, "ptfc_width_merge_%lld:\n  %%r%lld = phi i64 [ 1, %%ptfc_width_ascii_%lld ], [ 2, %%ptfc_width_two_%lld ], [ %%ptfc_wide_width_%lld, %%ptfc_width_wide_%lld ]\n",
            plan->dst, plan->dst, plan->dst, plan->dst, plan->dst, plan->dst);
    return 1;
  }
  if (plan->kind == HAKO_PTFC_ACCESS_SCALAR_EQ_TEXT) {
    long long dst = plan->dst;
    fputs("  ; ptfc leaf utf8_scalar_slice_eq_whole_text\n", out);
    fprintf(out, "  %%ptfc_len_eq_%lld = icmp eq i64 %%r%lld, %%ptfc_needle_len\n",
            dst, plan->width);
    fprintf(out, "  br i1 %%ptfc_len_eq_%lld, label %%ptfc_eq_lead_%lld, label %%ptfc_eq_false_%lld\n",
            dst, dst, dst);
    fprintf(out, "ptfc_eq_lead_%lld:\n", dst);
    fprintf(out, "  %%ptfc_rptr_0_%lld = getelementptr i8, ptr %%ptfc_needle_ptr, i64 0\n", dst);
    fprintf(out, "  %%ptfc_r0_%lld = load i8, ptr %%ptfc_rptr_0_%lld, align 1\n", dst, dst);
    fprintf(out, "  %%ptfc_c0_%lld = icmp eq i8 %%ptfc_byte_%lld, %%ptfc_r0_%lld\n",
            dst, plan->width, dst);
    fprintf(out, "  br i1 %%ptfc_c0_%lld, label %%ptfc_eq_after_b0_%lld, label %%ptfc_eq_false_%lld\n",
            dst, dst, dst);
    for (int byte = 1; byte < 4; byte++) {
      fprintf(out, "ptfc_eq_after_b%d_%lld:\n", byte - 1, dst);
      fprintf(out, "  %%ptfc_done_%d_%lld = icmp eq i64 %%r%lld, %d\n",
              byte, dst, plan->width, byte);
      fprintf(out, "  br i1 %%ptfc_done_%d_%lld, label %%ptfc_eq_true_%lld, label %%ptfc_eq_b%d_%lld\n",
              byte, dst, dst, byte, dst);
      fprintf(out, "ptfc_eq_b%d_%lld:\n", byte, dst);
      fprintf(out, "  %%ptfc_lptr_%d_%lld = getelementptr i8, ptr %%ptfc_byte_ptr_%lld, i64 %d\n",
              byte, dst, plan->width, byte);
      fprintf(out, "  %%ptfc_rptr_%d_%lld = getelementptr i8, ptr %%ptfc_needle_ptr, i64 %d\n",
              byte, dst, byte);
      fprintf(out, "  %%ptfc_l%d_%lld = load i8, ptr %%ptfc_lptr_%d_%lld, align 1\n",
              byte, dst, byte, dst);
      fprintf(out, "  %%ptfc_r%d_%lld = load i8, ptr %%ptfc_rptr_%d_%lld, align 1\n",
              byte, dst, byte, dst);
      fprintf(out, "  %%ptfc_c%d_%lld = icmp eq i8 %%ptfc_l%d_%lld, %%ptfc_r%d_%lld\n",
              byte, dst, byte, dst, byte, dst);
      if (byte < 3) {
        fprintf(out, "  br i1 %%ptfc_c%d_%lld, label %%ptfc_eq_after_b%d_%lld, label %%ptfc_eq_false_%lld\n",
                byte, dst, byte, dst, dst);
      } else {
        fprintf(out, "  br i1 %%ptfc_c%d_%lld, label %%ptfc_eq_true_%lld, label %%ptfc_eq_false_%lld\n",
                byte, dst, dst, dst);
      }
    }
    if (direct_then >= 0 && direct_else >= 0) {
      fprintf(out, "ptfc_eq_false_%lld:\n  br label %%bb%lld\n", dst, direct_else);
      fprintf(out, "ptfc_eq_true_%lld:\n  br label %%bb%lld\n", dst, direct_then);
    } else {
      fprintf(out, "ptfc_eq_false_%lld:\n  br label %%ptfc_eq_merge_%lld\n", dst, dst);
      fprintf(out, "ptfc_eq_true_%lld:\n  br label %%ptfc_eq_merge_%lld\n", dst, dst);
      fprintf(out, "ptfc_eq_merge_%lld:\n  %%r%lld = phi i1 [ false, %%ptfc_eq_false_%lld ], [ true, %%ptfc_eq_true_%lld ]\n",
              dst, dst, dst, dst);
    }
    return 1;
  }
  return 0;
}

static int hako_llvmc_ptfc_try_emit_selected_op(
    FILE* out,
    yyjson_val* instruction,
    const char* op,
    long long bid,
    long long ii,
    const struct HakoPtfSelectedCandidateView* candidate,
    struct HakoPtfSelectedLlvmDraft* draft) {
  long long plan_index = 0;
  if (!out || !instruction || !op || !candidate || !draft) return -1;
  if (!strcmp(op, "pinned_text_op")) {
    if (!hako_llvmc_ptfc_nonnegative(instruction, "plan", &plan_index) ||
        plan_index < 0 || plan_index >= candidate->carrier.plan_count ||
        (candidate->plans[plan_index].kind == HAKO_PTFC_ACCESS_SCALAR_EQ_TEXT &&
         HAKO_LLVMC_PTFC_EXACT_LEAF_EVIDENCE_PLAN_V1(
             &candidate->plans[plan_index]) != 0) ||
        !hako_llvmc_ptfc_emit_selected_leaf(
            out,
            &candidate->plans[plan_index],
            candidate->plans[plan_index].kind == HAKO_PTFC_ACCESS_SCALAR_EQ_TEXT
                ? candidate->scalar_branch_then : -1,
            candidate->plans[plan_index].kind == HAKO_PTFC_ACCESS_SCALAR_EQ_TEXT
                ? candidate->scalar_branch_else : -1)) {
      return -1;
    }
    draft->leaf_count++;
    if (hako_llvmc_ptfc_provenance_emitted_op_v1(
            bid, ii, op, instruction, candidate) != 0) return -1;
    return 1;
  }
  if (!strcmp(op, "branch")) {
    long long cond = -1;
    long long then_block = -1;
    long long else_block = -1;
    if (!hako_llvmc_ptfc_nonnegative(instruction, "cond", &cond) ||
        !hako_llvmc_ptfc_nonnegative(instruction, "then", &then_block) ||
        !hako_llvmc_ptfc_nonnegative(instruction, "else", &else_block) ||
        cond != candidate->plans[2].dst ||
        then_block != candidate->scalar_branch_then ||
        else_block != candidate->scalar_branch_else ||
        draft->scalar_branch_count != 0) {
      return 0;
    }
    draft->scalar_branch_count++;
    return 1;
  }
  if (!strcmp(op, "pinned_text_residence_enter")) {
    fputs("  %ptfc_pairs = alloca [2 x { i64, i64 }], align 8\n", out);
    for (long long i = 0; i < candidate->carrier.root_count; i++) {
      fprintf(out, "  %%ptfc_pair_%lld = getelementptr [2 x { i64, i64 }], ptr %%ptfc_pairs, i64 0, i64 %lld\n", i, i);
      fprintf(out, "  store i64 %%r%lld, ptr %%ptfc_pair_%lld, align 8\n",
              candidate->parameter_value_ids[candidate->carrier.slot_lanes[i]], i);
      fprintf(out, "  %%ptfc_pair_%lld_gen = getelementptr { i64, i64 }, ptr %%ptfc_pair_%lld, i32 0, i32 1\n", i, i);
      fprintf(out, "  store i64 %%r%lld, ptr %%ptfc_pair_%lld_gen, align 8\n",
              candidate->parameter_value_ids[candidate->carrier.generation_lanes[i]], i);
    }
    fprintf(out, "  %%ptfc_frame = alloca i8, i64 %lld, align 8\n", candidate->carrier.frame_size);
    fprintf(out, "  %%ptfc_status = call i32 @hako_text_formal_residence_enter_v1(ptr %%ptfc_pairs, i32 %lld, ptr %%ptfc_frame, i32 %lld)\n",
            candidate->carrier.root_count, candidate->carrier.frame_size);
    fputs("  %ptfc_enter_ok = icmp eq i32 %ptfc_status, 0\n", out);
    fprintf(out, "  br i1 %%ptfc_enter_ok, label %%bb%lld, label %%bb%lld\n",
            candidate->carrier.normal_landing, candidate->carrier.trap_landing);
    draft->enter_count++;
    if (hako_llvmc_ptfc_provenance_emitted_op_v1(
            bid, ii, op, instruction, candidate) != 0) return -1;
    return 1;
  }
  if (!strcmp(op, "pinned_text_residence_trap")) {
    fputs("  call void @llvm.trap()\n  unreachable\n", out);
    draft->trap_count++;
    return 1;
  }
  if (!strcmp(op, "pinned_text_residence_finish")) {
    fputs("  call void @hako_text_formal_residence_finish_or_abort_v1(ptr %ptfc_frame)\n", out);
    draft->finish_count++;
    return 1;
  }
  return 0;
}

#endif
