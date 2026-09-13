/* C ABI fixture that reopens the caller-owned input before returning. */
#if defined(_WIN32)
#define _CRT_SECURE_NO_WARNINGS
#endif
#include <stdio.h>
#include <stdlib.h>

int hako_llvmc_compile_json_with_options_v1(
    const char* input, const char* output, const void* options, char** error) {
  const char* record = getenv("HAKO_CAPI_RECORD_PATH");
  FILE* in;
  FILE* copy;
  FILE* out;
  char path[4096];
  int n;
  (void)options;
  (void)error;
  if (!input || !output || !record) return -2;
  n = snprintf(path, sizeof(path), "%s.path", record);
  if (n <= 0 || (size_t)n >= sizeof(path)) return -3;
  copy = fopen(path, "wb");
  if (!copy) return -4;
  fputs(input, copy);
  if (fclose(copy) != 0) return -5;
  n = snprintf(path, sizeof(path), "%s.input", record);
  if (n <= 0 || (size_t)n >= sizeof(path)) return -6;
  in = fopen(input, "rb");
  copy = fopen(path, "wb");
  if (!in || !copy) return -7;
  {
    int ch;
    while ((ch = fgetc(in)) != EOF) fputc(ch, copy);
  }
  fclose(in);
  fclose(copy);
  out = fopen(output, "wb");
  if (!out) return -8;
  fputs("capi-observer-object", out);
  return fclose(out) == 0 ? 0 : -9;
}
