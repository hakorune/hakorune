/* Prove the private FAST selector is sampled once by an invocation. */
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#include "../shims/hako_llvmc_ffi.c"

static void expect_value(const char* value, int expected) {
  struct HakoLlvmcInvocation invocation = {0};
  if (value) {
    setenv("NYASH_LLVM_FAST", value, 1);
  } else {
    unsetenv("NYASH_LLVM_FAST");
  }
  hako_llvmc_invocation_capture_fast(&invocation);
  assert(invocation.fast_enabled == expected);
}

int main(void) {
#if defined(_WIN32)
  return 0;
#else
  const char* enabled[] = {"1", "on", "true", "yes"};
  const char* disabled[] = {NULL, "", "0", "ON", "1suffix", "trueish"};
  struct HakoLlvmcInvocation invocation = {0};

  for (size_t i = 0; i < sizeof(enabled) / sizeof(enabled[0]); i++)
    expect_value(enabled[i], 1);
  for (size_t i = 0; i < sizeof(disabled) / sizeof(disabled[0]); i++)
    expect_value(disabled[i], 0);

  setenv("NYASH_LLVM_FAST", "yes", 1);
  hako_llvmc_invocation_capture_fast(&invocation);
  setenv("NYASH_LLVM_FAST", "0", 1);
  assert(invocation.fast_enabled == 1);

  hako_llvmc_invocation_capture_fast(&invocation);
  assert(invocation.fast_enabled == 0);
  setenv("NYASH_LLVM_FAST", "true", 1);
  hako_llvmc_invocation_capture_fast(&invocation);
  assert(invocation.fast_enabled == 1);

  unsetenv("NYASH_LLVM_FAST");
  puts("fast invocation capture: PASS");
  return 0;
#endif
}
