#include "../shims/hako_llvmc_ffi.c"

int main(void) {
  struct HakoPtfSelectedLlvmDraft draft;
  char* error = NULL;

  if (hako_llvmc_ptfc_open_selected_draft(&draft, &error) != 0) return 1;
  draft.enter_count = 1;
  draft.trap_count = 1;
  draft.finish_count = 2;
  draft.leaf_count = 3;
  fputs(
      "store i64 %r0\nstore i64 %r1\nstore i64 %r2\nstore i64 %r3\n"
      "%ptfc_frame = alloca i8\n"
      "call i32 @hako_text_formal_residence_enter_v1\n"
      "br i1 %ptfc_enter_ok\n"
      "call void @llvm.trap()\n  unreachable\n"
      "; ptfc leaf a\n; ptfc leaf b\n; ptfc leaf c\n"
      "ret i64 0\n"
      "call void @hako_text_formal_residence_finish_or_abort_v1(ptr %ptfc_frame)\n"
      "call void @hako_text_formal_residence_finish_or_abort_v1(ptr %ptfc_frame)\n"
      "ret i64 1\n",
      draft.stream);
  if (hako_llvmc_ptfc_verify_and_discard_selected_llvm(&draft, &error) == 0) {
    free(error);
    return 2;
  }
  if (draft.stream != NULL || !error ||
      !strstr(error, "private pinned Text LLVM verification failed")) {
    free(error);
    return 3;
  }
  free(error);
  return 0;
}
