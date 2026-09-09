/* File-backed proof fixtures enter the real bytes/session ABI. No V1 export. */
#include "../include/hako_llvmc_ffi.h"
#include <stdio.h>
#include <stdlib.h>
static int compile_static_test_file_v2(const char* input,
    const hako_llvmc_published_static_method_call_v1* calls, size_t count,
    const char* output, char** error) {
  FILE* file = fopen(input, "rb");
  if (!file) return -1;
  if (fseek(file, 0, SEEK_END)) { fclose(file); return -1; }
  long length = ftell(file);
  if (length < 0 || fseek(file, 0, SEEK_SET)) { fclose(file); return -1; }
  char* bytes = malloc((size_t)length + 1);
  if (!bytes) { fclose(file); return -1; }
  size_t read_count = fread(bytes, 1, (size_t)length, file);
  fclose(file);
  if (read_count != (size_t)length) { free(bytes); return -1; }
  hako_llvmc_static_invocation_v2* invocation = NULL;
  int rc = hako_llvmc_static_open_v2(bytes, read_count, &invocation, error);
  free(bytes);
  if (rc != 0) return rc;
  hako_llvmc_published_static_frame_v2 frame = {0};
  frame.revision = HAKO_LLVMC_STATIC_FRAME_REVISION;
  frame.byte_size = sizeof(frame);
  frame.calls = calls;
  frame.call_count = count;
  rc = hako_llvmc_static_compile_v2(invocation, &frame, output, error);
  hako_llvmc_static_close_v2(invocation);
  return rc;
}
