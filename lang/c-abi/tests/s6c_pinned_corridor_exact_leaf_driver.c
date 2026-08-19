#include <stdio.h>

static struct HakoPtfSelectedPlanView hako_exact_leaf_plan_v1;
static unsigned int hako_exact_leaf_plan_count_v1 = 0;
static const char* hako_exact_leaf_object_path_v1 = NULL;
static const char* hako_exact_leaf_ir_path_v1 = NULL;

static int hako_capture_exact_leaf_plan_v1(
    const struct HakoPtfSelectedPlanView* plan);
static int hako_emit_exact_leaf_evidence_v1(
    void* raw_session,
    void* module,
    char** err_out);

#define HAKO_LLVMC_PTFC_EXACT_LEAF_EVIDENCE_PLAN_V1(plan) \
  hako_capture_exact_leaf_plan_v1((plan))
#define HAKO_LLVMC_PTFC_FINAL_MODULE_EVIDENCE_V1(session, module, err_out) \
  hako_emit_exact_leaf_evidence_v1((session), (module), (err_out))
#include "../shims/hako_llvmc_ffi.c"

static int hako_capture_exact_leaf_plan_v1(
    const struct HakoPtfSelectedPlanView* plan) {
  if (!plan || plan->kind != HAKO_PTFC_ACCESS_SCALAR_EQ_TEXT ||
      hako_exact_leaf_plan_count_v1 != 0) {
    return -1;
  }
  hako_exact_leaf_plan_v1 = *plan;
  hako_exact_leaf_plan_count_v1 = 1;
  return 0;
}

static int hako_read_exact_leaf_draft_v1(
    FILE* stream,
    char** bytes_out,
    size_t* size_out) {
  long size;
  char* bytes;
  if (!stream || !bytes_out || !size_out || fflush(stream) != 0 ||
      fseek(stream, 0, SEEK_END) != 0 || (size = ftell(stream)) <= 0 ||
      fseek(stream, 0, SEEK_SET) != 0) {
    return 0;
  }
  bytes = (char*)malloc((size_t)size + 1);
  if (!bytes || fread(bytes, 1, (size_t)size, stream) != (size_t)size) {
    free(bytes);
    return 0;
  }
  bytes[size] = '\0';
  *bytes_out = bytes;
  *size_out = (size_t)size;
  return 1;
}

static int hako_emit_exact_leaf_evidence_v1(
    void* raw_session,
    void* module,
    char** err_out) {
  struct HakoPtfbTargetMachineSession* session =
      (struct HakoPtfbTargetMachineSession*)raw_session;
  const struct HakoPtfSelectedPlanView* plan = &hako_exact_leaf_plan_v1;
  FILE* draft = NULL;
  char* bytes = NULL;
  size_t byte_count = 0;
  void* buffer = NULL;
  int result = -1;
  if (!session || !module || !hako_exact_leaf_object_path_v1 ||
      !hako_exact_leaf_ir_path_v1 ||
      hako_exact_leaf_plan_count_v1 != 1) {
    return set_err_owned(err_out, "missing exact real-plan leaf evidence");
  }
  draft = tmpfile();
  if (!draft) return set_err_owned(err_out, "cannot open exact-leaf draft");
  fprintf(
      draft,
      "define i1 @hako_s6c_exact_leaf(ptr %%ptfc_subject_ptr, i64 %%r%lld, "
      "i64 %%r%lld, ptr %%ptfc_needle_ptr, i64 %%ptfc_needle_len) {\nentry:\n",
      plan->byte_offset,
      plan->width);
  if (!hako_llvmc_ptfc_emit_selected_leaf(draft, plan)) {
    set_err_owned(err_out, "production exact-leaf emitter rejected captured plan");
    goto cleanup;
  }
  fprintf(draft, "  ret i1 %%r%lld\n}\n", plan->dst);
  if (!hako_read_exact_leaf_draft_v1(draft, &bytes, &byte_count)) {
    set_err_owned(err_out, "cannot read exact-leaf draft");
    goto cleanup;
  }
  {
    FILE* ir_out = fopen(hako_exact_leaf_ir_path_v1, "wb");
    int ir_ok = ir_out && fwrite(bytes, 1, byte_count, ir_out) == byte_count;
    if (ir_out && fclose(ir_out) != 0) ir_ok = 0;
    if (!ir_ok) {
      remove(hako_exact_leaf_ir_path_v1);
      set_err_owned(err_out, "cannot publish exact-leaf IR evidence");
      goto cleanup;
    }
  }
  buffer = session->create_buffer_copy(
      bytes, byte_count, "hako-s6c-exact-leaf-evidence.ll");
  if (!buffer) {
    set_err_owned(err_out, "cannot create exact-leaf LLVM buffer");
    goto cleanup;
  }
  result = hako_llvmc_ptfb_session_emit_owned_buffer(
      session,
      buffer,
      NULL,
      NULL,
      hako_exact_leaf_object_path_v1,
      err_out);
  buffer = NULL;

cleanup:
  if (buffer && session->dispose_memory_buffer) session->dispose_memory_buffer(buffer);
  free(bytes);
  fclose(draft);
  if (result != 0) remove(hako_exact_leaf_object_path_v1);
  if (result != 0) remove(hako_exact_leaf_ir_path_v1);
  return result;
}

int main(int argc, char** argv) {
  char* error = NULL;
  int result;
  if (argc != 5) return 2;
  hako_exact_leaf_object_path_v1 = argv[3];
  hako_exact_leaf_ir_path_v1 = argv[4];
  remove(argv[2]);
  remove(argv[3]);
  remove(argv[4]);
  result = hako_llvmc_compile_json_pure_first(argv[1], argv[2], &error);
  if (result != 0 || error || hako_exact_leaf_plan_count_v1 != 1) {
    fprintf(stderr, "%s\n", error ? error : "exact-leaf evidence compile failed");
    free(error);
    remove(argv[2]);
    remove(argv[3]);
    remove(argv[4]);
    return 1;
  }
  return 0;
}
