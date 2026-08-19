#include <stdio.h>

static const char* hako_meso_ir_path_v1 = NULL;
static const char* hako_meso_object_path_v1 = NULL;
static unsigned int hako_meso_emit_count_v1 = 0;

static int hako_emit_meso_object_v1(void* raw_session, void* module, char** err_out);

#define HAKO_LLVMC_PTFC_FINAL_MODULE_EVIDENCE_V1(session, module, err_out) \
  hako_emit_meso_object_v1((session), (module), (err_out))
#include "../shims/hako_llvmc_ffi.c"

static int hako_read_meso_ir_v1(char** bytes_out, size_t* size_out) {
  FILE* input;
  long size;
  char* bytes;
  if (!bytes_out || !size_out || !(input = fopen(hako_meso_ir_path_v1, "rb")) ||
      fseek(input, 0, SEEK_END) != 0 || (size = ftell(input)) <= 0 ||
      fseek(input, 0, SEEK_SET) != 0) {
    if (input) fclose(input);
    return 0;
  }
  bytes = (char*)malloc((size_t)size + 1);
  if (!bytes) {
    fclose(input);
    return 0;
  }
  if (fread(bytes, 1, (size_t)size, input) != (size_t)size || fclose(input) != 0) {
    free(bytes);
    return 0;
  }
  bytes[size] = '\0';
  *bytes_out = bytes;
  *size_out = (size_t)size;
  return 1;
}

static int hako_emit_meso_object_v1(void* raw_session, void* module, char** err_out) {
  struct HakoPtfbTargetMachineSession* session =
      (struct HakoPtfbTargetMachineSession*)raw_session;
  char* bytes = NULL;
  size_t size = 0;
  void* buffer = NULL;
  int result = -1;
  if (!session || !module || !hako_meso_ir_path_v1 || !hako_meso_object_path_v1 ||
      hako_meso_emit_count_v1 != 0 || !hako_read_meso_ir_v1(&bytes, &size)) {
    return set_err_owned(err_out, "invalid meso object evidence state");
  }
  buffer = session->create_buffer_copy(bytes, size, "hako-s6c-meso-outline.ll");
  free(bytes);
  if (!buffer) return set_err_owned(err_out, "cannot create meso LLVM buffer");
  result = hako_llvmc_ptfb_session_emit_owned_buffer(
      session, buffer, NULL, NULL, hako_meso_object_path_v1, err_out);
  buffer = NULL;
  if (result != 0) {
    remove(hako_meso_object_path_v1);
    return result;
  }
  hako_meso_emit_count_v1++;
  return 0;
}

int main(int argc, char** argv) {
  char* error = NULL;
  int result;
  if (argc != 5) return 2;
  hako_meso_ir_path_v1 = argv[3];
  hako_meso_object_path_v1 = argv[4];
  remove(argv[2]);
  remove(argv[4]);
  result = hako_llvmc_compile_json_pure_first(argv[1], argv[2], &error);
  if (result != 0 || error || hako_meso_emit_count_v1 != 1) {
    fprintf(stderr, "%s\n", error ? error : "meso object evidence failed");
    free(error);
    remove(argv[2]);
    remove(argv[4]);
    return 1;
  }
  return 0;
}
