/* Private options-owner proof; no child or artifact is created. */
#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "../include/hako_llvmc_ffi.h"

static int fail_allocation;
static size_t allocations, releases;
static void* option_alloc(size_t size) {
  if (fail_allocation) return NULL;
  void* result = malloc(size);
  if (result) allocations++;
  return result;
}
static void option_free(void* pointer) {
  if (pointer) releases++;
  free(pointer);
}
static int set_err_owned(char** error, const char* message) {
  *error = strdup(message);
  return -1;
}
static int hako_llvmc_tool_exists(const char* path) { return path && *path; }
static const char* hako_llvmc_backend_compile_recipe(void) { return "pure-first"; }
static const char* hako_llvmc_opt_level(void) { return "0"; }
static const char* hako_llvmc_llc_flags(void) {
  const char* value = getenv("NYASH_NY_LLVM_LLC_FLAGS");
  return value && *value ? value : "-O3 -mcpu=native";
}
#define malloc option_alloc
#define free option_free
#include "../shims/hako_llvmc_ffi_physical_options.inc"
#undef malloc
#undef free

int main(void) {
  char path[] = "compiler";
  char* error = NULL;
  struct HakoLlvmcPhysicalOptions options = {0};
  hako_llvmc_physical_contract_v1 contract = {0};
  contract.revision = HAKO_LLVMC_PHYSICAL_CONTRACT_REVISION;
  contract.byte_size = sizeof(contract);
  contract.ingress_profile = HAKO_LLVMC_PHYSICAL_PROFILE_EXPLICIT_HARNESS;
  contract.llvmc_path = path;
  assert(hako_llvmc_physical_options_copy_named_harness(&options, &contract, &error) == 0);
  path[0] = 'X';
  assert(!strcmp(options.llvmc_path, "compiler"));
  assert(!options.compile_recipe && !options.opt_level);
  hako_llvmc_physical_options_destroy(&options);
  assert(allocations == 1 && releases == 1 && !options.llvmc_path);
  fail_allocation = 1;
  assert(hako_llvmc_physical_options_copy_named_harness(&options, &contract, &error) != 0);
  assert(strstr(error, "compile-options/oom") && !options.llvmc_path);
  free(error); error = NULL;
  fail_allocation = 0;
  contract.opt_level = "0";
  assert(hako_llvmc_physical_options_copy_named_harness(&options, &contract, &error) != 0);
  assert(strstr(error, "harness-fields"));
  free(error); error = NULL;
  contract.opt_level = NULL;
  assert(hako_llvmc_physical_options_copy(&options, &contract, &error) != 0);
  assert(strstr(error, "profile-flags"));
  free(error); error = NULL;
  contract.ingress_profile = HAKO_LLVMC_PHYSICAL_PROFILE_BOUNDARY_PURE_FIRST;
  contract.llvmc_path = NULL;
  contract.compile_recipe = "pure-first";
  contract.opt_level = "0";
  contract.compat_replay = "harness";
  assert(hako_llvmc_physical_options_copy(&options, &contract, &error) != 0);
  assert(strstr(error, "recipe-replay") && allocations == 1);
  free(error); error = NULL;
  contract.compat_replay = "none";
  setenv("NYASH_NY_LLVM_LLC_FLAGS", "-captured-before", 1);
  assert(hako_llvmc_physical_options_copy(&options, &contract, &error) == 0);
  setenv("NYASH_NY_LLVM_LLC_FLAGS", "-changed-after", 1);
  assert(options.llc_flags && !strcmp(options.llc_flags, "-captured-before"));
  assert(allocations == 4); /* recipe, level and the captured Boundary default */
  hako_llvmc_physical_options_destroy(&options);
  assert(allocations == releases);
  contract.llc_flags = "";
  assert(hako_llvmc_physical_options_copy(&options, &contract, &error) == 0);
  assert(options.llc_flags && !options.llc_flags[0]);
  hako_llvmc_physical_options_destroy(&options);
  assert(allocations == releases);
  unsetenv("NYASH_NY_LLVM_LLC_FLAGS");
  return 0;
}
