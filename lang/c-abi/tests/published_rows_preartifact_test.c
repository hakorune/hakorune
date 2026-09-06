/* Exercise the real typed ABI with an existing call-free MIR input.
 * No source fixture or semantic target is synthesized by this test. */
#include "../include/hako_llvmc_ffi.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main(int argc, char **argv) {
  if (argc != 3) return 2;
  if (access(argv[2], F_OK) == 0) return 3;
  hako_llvmc_published_static_method_call_v1 row = {0};
  row.function_name = "main";
  row.block_id = 0;
  row.instruction_index = 999;
  row.kind = HAKO_LLVMC_PUBLISHED_CALL_KIND_BUILTIN_PRINT;
  row.arity = 1;
  char *error = NULL;
  int rc = hako_llvmc_compile_published_static_method_v1(
      argv[1], &row, 1, argv[2], &error);
  int ok = rc != 0 && error &&
      strstr(error, "typed row was not consumed") &&
      access(argv[2], F_OK) != 0;
  if (!ok) fprintf(stderr, "rc=%d error=%s artifact=%d\n", rc,
      error ? error : "none", access(argv[2], F_OK) == 0);
  free(error);
  if (!ok) return 1;
  puts("published residual rejected before object: PASS");
  return 0;
}
