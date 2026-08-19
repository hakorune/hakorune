#include <stdio.h>

static const char* hako_c1_lowered_path_v1 = NULL;
static FILE* hako_c1_origin_stream_v1 = NULL;
static unsigned int hako_c1_lowered_count_v1 = 0;
static unsigned int hako_c1_origin_count_v1 = 0;
static unsigned int hako_c1_fail_after_v1 = 0;

static int hako_c1_capture_origin_v1(
    const char* entity, long long mir_block, long long mir_instruction,
    const char* mir_arm, long long mir_target, const char* llvm_from,
    const char* llvm_to, const char* disposition, const char* reason);
static int hako_c1_capture_lowered_v1(const char* path, char** err_out);

#define HAKO_LLVMC_C1_PROVENANCE_ENABLED_V1 1
#define HAKO_LLVMC_C1_PROVENANCE_EVENT_V1(                            \
    entity, mir_block, mir_instruction, mir_arm, mir_target, llvm_from,\
    llvm_to, disposition, reason)                                     \
  hako_c1_capture_origin_v1(                                           \
      (entity), (mir_block), (mir_instruction), (mir_arm), (mir_target),\
      (llvm_from), (llvm_to), (disposition), (reason))
#define HAKO_LLVMC_C1_LOWERED_LLVM_EVIDENCE_V1(path, err_out) \
  hako_c1_capture_lowered_v1((path), (err_out))
#include "../shims/hako_llvmc_ffi.c"

static int hako_c1_capture_origin_v1(
    const char* entity, long long mir_block, long long mir_instruction,
    const char* mir_arm, long long mir_target, const char* llvm_from,
    const char* llvm_to, const char* disposition, const char* reason) {
  if (!hako_c1_origin_stream_v1) return -1;
  if (hako_c1_fail_after_v1 &&
      hako_c1_origin_count_v1 >= hako_c1_fail_after_v1) return -1;
  if (fprintf(
      hako_c1_origin_stream_v1,
      "%s\t%lld\t%lld\t%s\t%lld\t%s\t%s\t%s\t%s\n",
      entity, mir_block, mir_instruction, mir_arm, mir_target,
      llvm_from, llvm_to, disposition, reason) < 0) return -1;
  hako_c1_origin_count_v1++;
  return 0;
}

static int hako_c1_capture_lowered_v1(const char* path, char** err_out) {
  FILE* input;
  FILE* output;
  unsigned char buffer[8192];
  size_t count;
  int ok = 1;
  if (!path || !hako_c1_lowered_path_v1 || hako_c1_lowered_count_v1 != 0) {
    return set_err_owned(err_out, "invalid selected Dynamic lowered capture state");
  }
  input = fopen(path, "rb");
  output = fopen(hako_c1_lowered_path_v1, "wb");
  if (!input || !output) ok = 0;
  while (ok && (count = fread(buffer, 1, sizeof(buffer), input)) != 0) {
    if (fwrite(buffer, 1, count, output) != count) ok = 0;
  }
  if (input && ferror(input)) ok = 0;
  if (input && fclose(input) != 0) ok = 0;
  if (output && fclose(output) != 0) ok = 0;
  if (!ok) {
    remove(hako_c1_lowered_path_v1);
    return set_err_owned(err_out, "cannot capture selected Dynamic lowered LLVM");
  }
  hako_c1_lowered_count_v1++;
  return 0;
}

int main(int argc, char** argv) {
  char* error = NULL;
  int rc;
  if (argc != 5 && argc != 6) return 2;
  if (argc == 6) hako_c1_fail_after_v1 = (unsigned int)strtoul(argv[5], NULL, 10);
  hako_c1_lowered_path_v1 = argv[3];
  remove(argv[2]);
  remove(argv[3]);
  remove(argv[4]);
  hako_c1_origin_stream_v1 = fopen(argv[4], "wb");
  if (!hako_c1_origin_stream_v1) return 2;
  rc = hako_llvmc_compile_json_pure_first(argv[1], argv[2], &error);
  if (fclose(hako_c1_origin_stream_v1) != 0) rc = -1;
  hako_c1_origin_stream_v1 = NULL;
  if (rc != 0 || error || hako_c1_lowered_count_v1 != 1) {
    fprintf(stderr, "%s\n", error ? error : "selected Dynamic provenance failed");
    free(error);
    remove(argv[2]);
    remove(argv[3]);
    remove(argv[4]);
    return 1;
  }
  return 0;
}
