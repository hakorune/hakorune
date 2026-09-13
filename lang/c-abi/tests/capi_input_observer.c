/* C ABI fixture that reopens the caller-owned input before returning. */
#if defined(_WIN32)
#define _CRT_SECURE_NO_WARNINGS
#define HAKO_CAPI_OBSERVER_EXPORT __declspec(dllexport)
#include <windows.h>
#else
#define HAKO_CAPI_OBSERVER_EXPORT
#endif
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Rust updates the process environment through the OS API.  A MinGW DLL
 * loaded into an MSVC test process must read that shared OS environment
 * directly instead of relying on a CRT-local getenv cache. */
static int observer_read_env(const char* name, char* value, size_t capacity) {
#if defined(_WIN32)
  DWORD length = GetEnvironmentVariableA(name, value, (DWORD)capacity);
  if (length == 0 || (size_t)length >= capacity) return 0;
  return 1;
#else
  const char* inherited = getenv(name);
  size_t length;
  if (!inherited) return 0;
  length = strlen(inherited);
  if (length >= capacity) return 0;
  memcpy(value, inherited, length + 1);
  return 1;
#endif
}

HAKO_CAPI_OBSERVER_EXPORT int hako_llvmc_compile_json_with_options_v1(
    const char* input, const char* output, const void* options, char** error) {
  char record[4096];
  char mode[32];
  FILE* in;
  FILE* copy;
  FILE* out;
  char path[4096];
  int n;
  (void)options;
  (void)error;
  if (!input || !output ||
      !observer_read_env("HAKO_CAPI_RECORD_PATH", record, sizeof(record)))
    return -2;
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
  if (observer_read_env("HAKO_CAPI_OBSERVER_MODE", mode, sizeof(mode)) &&
      strcmp(mode, "fail") == 0) {
    return -10;
  }
  out = fopen(output, "wb");
  if (!out) return -8;
  fputs("capi-observer-object", out);
  return fclose(out) == 0 ? 0 : -9;
}
