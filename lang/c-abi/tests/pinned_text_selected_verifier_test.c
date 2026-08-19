#include "../shims/hako_llvmc_ffi.c"

static int write_contract_fixture(
    struct HakoPtfSelectedLlvmDraft* draft,
    int ordered_returns,
    const char* enter_declaration,
    char** error) {
  if (hako_llvmc_ptfc_open_selected_draft(draft, error) != 0) return -1;
  draft->enter_count = 1;
  draft->trap_count = 1;
  draft->finish_count = 2;
  draft->leaf_count = 3;
  if (enter_declaration) {
    fputs(enter_declaration, draft->stream);
    fputs(HAKO_PTFC_FINISH_DECL_V1, draft->stream);
  } else {
    hako_llvmc_ptfc_emit_selected_declarations(draft->stream);
  }
  fputs(
      "store i64 %r0\nstore i64 %r1\nstore i64 %r2\nstore i64 %r3\n"
      "%ptfc_frame = alloca i8\n"
      "call i32 @hako_text_formal_residence_enter_v1\n"
      "br i1 %ptfc_enter_ok\n"
      "call void @llvm.trap()\n  unreachable\n"
      "; ptfc leaf a\n; ptfc leaf b\n; ptfc leaf c\n",
      draft->stream);
  if (ordered_returns) {
    fputs(
        "call void @hako_text_formal_residence_finish_or_abort_v1(ptr %ptfc_frame)\n"
        "  ret i64 0\n"
        "call void @hako_text_formal_residence_finish_or_abort_v1(ptr %ptfc_frame)\n"
        "  ret i64 1\n",
        draft->stream);
  } else {
    fputs(
        "ret i64 0\n"
        "call void @hako_text_formal_residence_finish_or_abort_v1(ptr %ptfc_frame)\n"
        "call void @hako_text_formal_residence_finish_or_abort_v1(ptr %ptfc_frame)\n"
        "ret i64 1\n",
        draft->stream);
  }
  return 0;
}

int main(void) {
  struct HakoPtfSelectedLlvmDraft draft;
  struct HakoPtfSelectedLlvmBytes bytes;
  struct HakoPtfbModuleCensus census;
  struct HakoPtfbTargetMachineSession session;
  char object_path[256];
  char temporary_path[320];
  char* error = NULL;

  if (write_contract_fixture(&draft, 1, NULL, &error) != 0) return 1;
  if (hako_llvmc_ptfc_verify_and_take_selected_llvm(
          &draft, &bytes, &error) != 0) {
    free(error);
    return 2;
  }
  if (!strstr(bytes.data, HAKO_PTFC_ENTER_DECL_V1) ||
      !strstr(bytes.data, HAKO_PTFC_FINISH_DECL_V1) ||
      strstr(bytes.data, " readonly") || strstr(bytes.data, " readnone") ||
      strstr(bytes.data, " nofree") || strstr(bytes.data, " speculatable")) {
    hako_llvmc_ptfc_release_selected_llvm_bytes(&bytes);
    return 3;
  }
  hako_llvmc_ptfc_release_selected_llvm_bytes(&bytes);

  if (write_contract_fixture(&draft, 0, NULL, &error) != 0) return 4;
  if (hako_llvmc_ptfc_verify_and_take_selected_llvm(
          &draft, &bytes, &error) == 0) {
    hako_llvmc_ptfc_release_selected_llvm_bytes(&bytes);
    free(error);
    return 5;
  }
  if (draft.stream != NULL || bytes.data != NULL || bytes.size != 0 || !error ||
      !strstr(error, "private pinned Text LLVM verification failed")) {
    free(error);
    return 6;
  }
  free(error);
  error = NULL;

  if (write_contract_fixture(
          &draft,
          1,
          "declare i32 @hako_text_formal_residence_enter_v1(ptr, i32, ptr, i32) nounwind readonly\n",
          &error) != 0) {
    return 7;
  }
  if (hako_llvmc_ptfc_verify_and_take_selected_llvm(
          &draft, &bytes, &error) == 0) {
    hako_llvmc_ptfc_release_selected_llvm_bytes(&bytes);
    free(error);
    return 8;
  }
  if (!error || !strstr(error, "private pinned Text LLVM verification failed")) {
    free(error);
    return 9;
  }
  free(error);
  error = NULL;

  memset(&census, 0, sizeof(census));
  census.contract_count = 1;
  if (hako_llvmc_ptfb_session_open(&session, &census, &error) != 0) {
    free(error);
    return 10;
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
    return 11;
  }
  hako_llvmc_ptfb_session_close(&session);
  if (!error || hako_llvmc_file_exists(object_path) ||
      hako_llvmc_file_exists(temporary_path)) {
    free(error);
    remove(object_path);
    remove(temporary_path);
    return 12;
  }
  free(error);
  remove(object_path);
  remove(temporary_path);
  return 0;
}
