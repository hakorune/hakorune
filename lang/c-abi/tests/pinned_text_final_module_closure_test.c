#include "../shims/hako_llvmc_ffi.c"

static const char* const VALID_MODULE =
    "declare i32 @hako_text_formal_residence_enter_v1(ptr, i32, ptr, i32) nounwind\n"
    "declare void @hako_text_formal_residence_finish_or_abort_v1(ptr) nounwind\n"
    "declare void @llvm.trap() nounwind\n"
    "declare void @extra() nounwind\n"
    "define i64 @ny_main(i64 %r0, i64 %r1, i64 %r2, i64 %r3) {\n"
    "entry:\n"
    "  %pairs = alloca [2 x { i64, i64 }], align 8\n"
    "  %frame = alloca i8, i64 64, align 8\n"
    "  %status = call i32 @hako_text_formal_residence_enter_v1(ptr %pairs, i32 2, ptr %frame, i32 64)\n"
    "  %ok = icmp eq i32 %status, 0\n"
    "  br i1 %ok, label %normal, label %trap\n"
    "normal:\n"
    "  %marker = add i64 0, 0\n"
    "  %pick = icmp eq i64 %r0, %marker\n"
    "  br i1 %pick, label %ret0, label %ret1\n"
    "ret0:\n"
    "  call void @hako_text_formal_residence_finish_or_abort_v1(ptr %frame)\n"
    "  ret i64 0\n"
    "ret1:\n"
    "  call void @hako_text_formal_residence_finish_or_abort_v1(ptr %frame)\n"
    "  ret i64 1\n"
    "trap:\n"
    "  call void @llvm.trap()\n"
    "  unreachable\n"
    "}\n";

static char* replace_once(
    const char* input,
    const char* needle,
    const char* replacement) {
  const char* found = strstr(input, needle);
  size_t prefix;
  size_t suffix;
  size_t replacement_length;
  char* output;
  if (!found) return NULL;
  prefix = (size_t)(found - input);
  suffix = strlen(found + strlen(needle));
  replacement_length = strlen(replacement);
  output = (char*)malloc(prefix + replacement_length + suffix + 1);
  if (!output) return NULL;
  memcpy(output, input, prefix);
  memcpy(output + prefix, replacement, replacement_length);
  memcpy(output + prefix + replacement_length, found + strlen(needle), suffix + 1);
  return output;
}

static char* build_eh_module(void) {
  char* step1 = replace_once(
      VALID_MODULE,
      "declare void @extra() nounwind\n",
      "declare void @extra()\ndeclare i32 @__gxx_personality_v0(...)\n");
  char* step2;
  char* step3;
  char* result;
  if (!step1) return NULL;
  step2 = replace_once(
      step1,
      "define i64 @ny_main(i64 %r0, i64 %r1, i64 %r2, i64 %r3) {\n",
      "define i64 @ny_main(i64 %r0, i64 %r1, i64 %r2, i64 %r3) personality ptr @__gxx_personality_v0 {\n");
  free(step1);
  if (!step2) return NULL;
  step3 = replace_once(
      step2,
      "  %marker = add i64 0, 0\n",
      "  invoke void @extra() to label %after_eh unwind label %lpad\n"
      "after_eh:\n  %marker = add i64 0, 0\n");
  free(step2);
  if (!step3) return NULL;
  result = replace_once(
      step3,
      "}\n",
      "lpad:\n  %lp = landingpad { ptr, i32 } cleanup\n"
      "  resume { ptr, i32 } %lp\n}\n");
  free(step3);
  return result;
}

static int invoke_case(
    struct HakoPtfbTargetMachineSession* session,
    struct HakoPtfSelectedCandidateView* candidate,
    const char* label,
    const char* module,
    int expect_success,
    const char* expected_error) {
  char object_path[256];
  char temporary_path[320];
  char* error = NULL;
  int rc;
  snprintf(
      object_path,
      sizeof(object_path),
      "/tmp/hako-ptfc-final-%ld-%s.o",
      (long)getpid(),
      label);
  snprintf(
      temporary_path,
      sizeof(temporary_path),
      "%s.ptfb-tm-%ld.tmp",
      object_path,
      (long)getpid());
  remove(object_path);
  remove(temporary_path);
  rc = hako_llvmc_ptfb_session_emit_object_from_bytes(
      session,
      module,
      strlen(module),
      candidate,
      "ny_main",
      object_path,
      &error);
  if (expect_success) {
    if (rc != 0 || error || !hako_llvmc_file_exists(object_path)) {
      free(error);
      remove(object_path);
      remove(temporary_path);
      return 0;
    }
  } else if (rc == 0 || !error ||
             (expected_error && !strstr(error, expected_error)) ||
             hako_llvmc_file_exists(object_path) ||
             hako_llvmc_file_exists(temporary_path)) {
    free(error);
    remove(object_path);
    remove(temporary_path);
    return 0;
  }
  free(error);
  remove(object_path);
  remove(temporary_path);
  return 1;
}

int main(void) {
  struct HakoPtfbModuleCensus census;
  struct HakoPtfbTargetMachineSession session;
  struct HakoPtfSelectedCandidateView candidate;
  char* missing_nounwind;
  char* missing_finish;
  char* extra_call;
  char* eh_module;
  char* error = NULL;
  int ok = 0;
  memset(&census, 0, sizeof(census));
  memset(&candidate, 0, sizeof(candidate));
  census.contract_count = 1;
  candidate.carrier.normal_exit_count = 2;
  if (hako_llvmc_ptfb_session_open(&session, &census, &error) != 0) {
    free(error);
    return 1;
  }
  missing_nounwind = replace_once(
      VALID_MODULE,
      "residence_enter_v1(ptr, i32, ptr, i32) nounwind",
      "residence_enter_v1(ptr, i32, ptr, i32)");
  missing_finish = replace_once(
      VALID_MODULE,
      "  call void @hako_text_formal_residence_finish_or_abort_v1(ptr %frame)\n",
      "  ; Finish intentionally removed\n");
  extra_call = replace_once(
      VALID_MODULE,
      "  %marker = add i64 0, 0\n",
      "  call void @extra()\n  %marker = add i64 0, 0\n");
  eh_module = build_eh_module();
  if (!missing_nounwind || !missing_finish || !extra_call || !eh_module) {
    goto cleanup;
  }
  if (!invoke_case(&session, &candidate, "valid", VALID_MODULE, 1, NULL)) {
    goto cleanup;
  }
  if (!invoke_case(
          &session,
          &candidate,
          "attribute",
          missing_nounwind,
          0,
          "function/ABI mismatch")) {
    goto cleanup;
  }
  if (!invoke_case(
          &session,
          &candidate,
          "finish",
          missing_finish,
          0,
          "Return bypassed Finish")) {
    goto cleanup;
  }
  if (!invoke_case(
          &session,
          &candidate,
          "call",
          extra_call,
          0,
          "call set drifted")) {
    goto cleanup;
  }
  if (!invoke_case(
          &session,
          &candidate,
          "eh",
          eh_module,
          0,
          "EH construct is forbidden")) {
    goto cleanup;
  }
  ok = 1;
cleanup:
  free(missing_nounwind);
  free(missing_finish);
  free(extra_call);
  free(eh_module);
  hako_llvmc_ptfb_session_close(&session);
  return ok ? 0 : 2;
}
