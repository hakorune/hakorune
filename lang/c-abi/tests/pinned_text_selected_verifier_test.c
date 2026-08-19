#include "../shims/hako_llvmc_ffi.c"

int main(void) {
  struct HakoPtfSelectedLlvmDraft draft;
  struct HakoPtfSelectedLlvmBytes bytes;
  struct HakoPtfbModuleCensus census;
  struct HakoPtfbTargetMachineSession session;
  char object_path[256];
  char temporary_path[320];
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
  if (hako_llvmc_ptfc_verify_and_take_selected_llvm(
          &draft, &bytes, &error) == 0) {
    hako_llvmc_ptfc_release_selected_llvm_bytes(&bytes);
    free(error);
    return 2;
  }
  if (draft.stream != NULL || bytes.data != NULL || bytes.size != 0 || !error ||
      !strstr(error, "private pinned Text LLVM verification failed")) {
    free(error);
    return 3;
  }
  free(error);
  error = NULL;

  memset(&census, 0, sizeof(census));
  census.contract_count = 1;
  if (hako_llvmc_ptfb_session_open(&session, &census, &error) != 0) {
    free(error);
    return 4;
  }
  snprintf(
      object_path,
      sizeof(object_path),
      "/tmp/hako-ptfc-invalid-%ld.o",
      (long)getpid());
  snprintf(
      temporary_path,
      sizeof(temporary_path),
      "%s.ptfb-tm-%ld.tmp",
      object_path,
      (long)getpid());
  remove(object_path);
  remove(temporary_path);
  if (hako_llvmc_ptfb_session_emit_object_from_bytes(
          &session,
          "not llvm ir",
          strlen("not llvm ir"),
          object_path,
          &error) == 0) {
    hako_llvmc_ptfb_session_close(&session);
    return 5;
  }
  hako_llvmc_ptfb_session_close(&session);
  if (!error || hako_llvmc_file_exists(object_path) ||
      hako_llvmc_file_exists(temporary_path)) {
    free(error);
    remove(object_path);
    remove(temporary_path);
    return 6;
  }
  free(error);
  remove(object_path);
  remove(temporary_path);
  return 0;
}
