/* Native child for the direct named C-harness Windows acceptance. */
#if defined(_WIN32)
#define _CRT_SECURE_NO_WARNINGS
#endif
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static const char* argument_value(int argc, char** argv, const char* name) {
  int i;
  for (i = 1; i + 1 < argc; i++) {
    if (strcmp(argv[i], name) == 0) return argv[i + 1];
  }
  return NULL;
}

int main(int argc, char** argv) {
  const char* input = argument_value(argc, argv, "--in");
  const char* output = argument_value(argc, argv, "--out");
  const char* mode = getenv("HAKO_HARNESS_TEST_MODE");
  FILE* file;
  char buffer[128];
  size_t size;

  if (!input || !output) return 2;
  file = fopen(input, "rb");
  if (!file) {
    fputs("input reopen failed\n", stderr);
    return 3;
  }
  size = fread(buffer, 1, sizeof(buffer), file);
  if (fclose(file) != 0 || size == 0) {
    fputs("input read failed\n", stderr);
    return 4;
  }
  if (mode && strcmp(mode, "fail") == 0) {
    fputs("direct child failure\nsecond line\n", stderr);
    return 7;
  }
  if (mode && strcmp(mode, "missing") == 0) return 0;
  file = fopen(output, "wb");
  if (!file) {
    fputs("output reopen failed\n", stderr);
    return 5;
  }
  fputs("native-child-object", file);
  return fclose(file) == 0 ? 0 : 6;
}
