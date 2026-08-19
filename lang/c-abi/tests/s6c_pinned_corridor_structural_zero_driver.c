#include <stdio.h>

static const char* hako_ptfc_evidence_path_v1 = NULL;
static unsigned int hako_ptfc_evidence_count_v1 = 0;
static FILE* hako_ptfc_provenance_stream_v1 = NULL;

static int hako_ptfc_capture_final_module_v1(
    void* raw_session,
    void* module,
    char** err_out);
static int hako_ptfc_capture_provenance_v1(
    const char* entity, long long mir_block, long long mir_instruction,
    const char* mir_arm, long long mir_target, const char* llvm_from,
    const char* llvm_to, const char* disposition, const char* reason);

#define HAKO_LLVMC_PTFC_FINAL_MODULE_EVIDENCE_V1(session, module, err_out) \
  hako_ptfc_capture_final_module_v1((session), (module), (err_out))
#define HAKO_LLVMC_PTFC_PROVENANCE_EVENT_V1(                            \
    entity, mir_block, mir_instruction, mir_arm, mir_target, llvm_from,  \
    llvm_to, disposition, reason)                                       \
  hako_ptfc_capture_provenance_v1(                                      \
      (entity), (mir_block), (mir_instruction), (mir_arm), (mir_target),\
      (llvm_from), (llvm_to), (disposition), (reason))
#include "../shims/hako_llvmc_ffi.c"

typedef char* (*hako_ptfc_print_module_to_string_fn)(void*);

static int hako_ptfc_capture_provenance_v1(
    const char* entity, long long mir_block, long long mir_instruction,
    const char* mir_arm, long long mir_target, const char* llvm_from,
    const char* llvm_to, const char* disposition, const char* reason) {
  if (!hako_ptfc_provenance_stream_v1) return 0;
  return fprintf(
      hako_ptfc_provenance_stream_v1,
      "%s\t%lld\t%lld\t%s\t%lld\t%s\t%s\t%s\t%s\n",
      entity, mir_block, mir_instruction, mir_arm, mir_target,
      llvm_from, llvm_to, disposition, reason) < 0 ? -1 : 0;
}

static int hako_ptfc_capture_final_module_v1(
    void* raw_session,
    void* module,
    char** err_out) {
  struct HakoPtfbTargetMachineSession* session =
      (struct HakoPtfbTargetMachineSession*)raw_session;
  hako_ptfc_print_module_to_string_fn print_module;
  char* text;
  FILE* output;
  size_t length;
  int ok;
  if (!session || !session->handle || !module || !hako_ptfc_evidence_path_v1 ||
      hako_ptfc_evidence_count_v1 != 0) {
    return set_err_owned(err_out, "invalid structural evidence capture state");
  }
  print_module = (hako_ptfc_print_module_to_string_fn)
      hako_llvmc_ptfb_session_symbol(session->handle, "LLVMPrintModuleToString");
  if (!print_module) {
    return set_err_owned(err_out, "LLVM18 final-module printer is unavailable");
  }
  text = print_module(module);
  if (!text) return set_err_owned(err_out, "LLVM18 final-module print failed");
  output = fopen(hako_ptfc_evidence_path_v1, "wb");
  if (!output) {
    session->dispose_message(text);
    return set_err_owned(err_out, "cannot open final-module evidence output");
  }
  length = strlen(text);
  ok = fwrite(text, 1, length, output) == length;
  if (fclose(output) != 0) ok = 0;
  session->dispose_message(text);
  if (!ok) {
    remove(hako_ptfc_evidence_path_v1);
    return set_err_owned(err_out, "cannot publish final-module evidence");
  }
  hako_ptfc_evidence_count_v1++;
  return 0;
}

int main(int argc, char** argv) {
  char* error = NULL;
  int rc;
  if (argc != 4 && argc != 5) return 2;
  hako_ptfc_evidence_path_v1 = argv[3];
  if (argc == 5) {
    remove(argv[4]);
    hako_ptfc_provenance_stream_v1 = fopen(argv[4], "wb");
    if (!hako_ptfc_provenance_stream_v1) return 2;
  }
  remove(argv[2]);
  remove(argv[3]);
  rc = hako_llvmc_compile_json_pure_first(argv[1], argv[2], &error);
  if (hako_ptfc_provenance_stream_v1 &&
      fclose(hako_ptfc_provenance_stream_v1) != 0) rc = 1;
  hako_ptfc_provenance_stream_v1 = NULL;
  if (rc != 0 || error || hako_ptfc_evidence_count_v1 != 1) {
    fprintf(stderr, "%s\n", error ? error : "structural evidence compile failed");
    free(error);
    remove(argv[2]);
    remove(argv[3]);
    if (argc == 5) remove(argv[4]);
    return 1;
  }
  return 0;
}
