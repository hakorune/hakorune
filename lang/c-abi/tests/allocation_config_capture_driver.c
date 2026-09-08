/* Exercise the real borrowed core after changing ambient configuration. */
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
static const char* captured_llvm;
static int capture_remove(const char* path) {
  if (strstr(path, "hako_pure_gen_") && strstr(path, ".ll")) {
    FILE* input = fopen(path, "rb");
    if (input) {
      FILE* output = fopen(captured_llvm, "wb");
      assert(output);
      char buffer[8192];
      size_t count;
      while ((count = fread(buffer, 1, sizeof(buffer), input)))
        assert(fwrite(buffer, 1, count, output) == count);
      assert(!ferror(input));
      assert(fclose(input) == 0 && fclose(output) == 0);
    }
  }
  return remove(path);
}
#define remove capture_remove
#include "../shims/hako_llvmc_ffi.c"
#undef remove
int main(int argc, char** argv) {
  assert(argc == 4);
  captured_llvm = argv[3];
  struct HakoLlvmcAllocationConfig config = hako_llvmc_capture_allocation_config();
  /* Reverse every observed predicate after capture. */
  setenv("HAKO_ARRAY_SLOT_STORE", (config.runtime_flags & 2) ? "" : "direct_array_i64_exact", 1);
  setenv("HAKO_TYPED_OBJECT_STORE", (config.runtime_flags & 1) ? "single_thread_exact" : "direct_slot_exact", 1);
  setenv("HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER", config.exact_slot_helper ? "0" : "1", 1);
  char* error = NULL;
  yyjson_doc* doc = hako_json_v1_read_owned_file(argv[1], &error);
  assert(doc);
  int rc = compile_doc_compat_pure(doc, argv[1], argv[2], &error, config);
  yyjson_doc_free(doc);
  if (error) fprintf(stderr, "%s\n", error);
  free(error);
  printf("%d %d %d\n", rc, config.runtime_flags, config.exact_slot_helper);
  return rc != 0;
}
