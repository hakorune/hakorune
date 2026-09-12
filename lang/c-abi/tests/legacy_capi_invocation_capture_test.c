/* Prove the private legacy CAPI capture without opening a compiler artifact. */
#include <assert.h>
#include <stdlib.h>

#include "../shims/hako_llvmc_ffi.c"

int main(void) {
#if defined(_WIN32)
  return 0;
#else
  struct HakoLlvmcInvocation invocation = {0};

  unsetenv("HAKO_CAPI_TM");
  unsetenv("HAKO_LLVM_OPT_LEVEL");
  unsetenv("NYASH_LLVM_OPT_LEVEL");
  hako_llvmc_invocation_capture_legacy_capi(&invocation, 1);
  assert(invocation.legacy_capi.target_machine_enabled == 0);
  assert(invocation.legacy_capi.opt_level == 0);

  setenv("HAKO_CAPI_TM", "1", 1);
  setenv("HAKO_LLVM_OPT_LEVEL", "2suffix", 1);
  setenv("NYASH_LLVM_OPT_LEVEL", "3", 1);
  hako_llvmc_invocation_capture_legacy_capi(&invocation, 1);
  assert(invocation.legacy_capi.target_machine_enabled == 1);
  assert(invocation.legacy_capi.opt_level == 2);

  /* HAKO presence wins even when its value is empty. */
  setenv("HAKO_LLVM_OPT_LEVEL", "", 1);
  hako_llvmc_invocation_capture_legacy_capi(&invocation, 1);
  assert(invocation.legacy_capi.target_machine_enabled == 1);
  assert(invocation.legacy_capi.opt_level == 0);

  /* mutation-after-capture: an eligible invocation keeps its captured state. */
  setenv("HAKO_LLVM_OPT_LEVEL", "1", 1);
  hako_llvmc_invocation_capture_legacy_capi(&invocation, 1);
  setenv("HAKO_CAPI_TM", "0", 1);
  setenv("HAKO_LLVM_OPT_LEVEL", "3", 1);
  setenv("NYASH_LLVM_OPT_LEVEL", "2", 1);
  assert(invocation.legacy_capi.target_machine_enabled == 1);
  assert(invocation.legacy_capi.opt_level == 1);

  /* Non-legacy lanes reset and do not read ambient selectors. */
  hako_llvmc_invocation_capture_legacy_capi(&invocation, 0);
  assert(invocation.legacy_capi.target_machine_enabled == 0);
  assert(invocation.legacy_capi.opt_level == 0);

  unsetenv("HAKO_CAPI_TM");
  unsetenv("HAKO_LLVM_OPT_LEVEL");
  unsetenv("NYASH_LLVM_OPT_LEVEL");
  return 0;
#endif
}
