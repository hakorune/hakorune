#define _POSIX_C_SOURCE 200809L
/* Exercise the real typed ABI with an existing call-free MIR input.
 * No source fixture or semantic target is synthesized by this test. */
#include "../include/hako_llvmc_ffi.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <assert.h>
#include "yyjson.h"

/* Exercise the shared internal transport owner as well as the real ABI below.
 * No test-only export or duplicate row implementation is introduced. */
static int set_err_owned(char **out, const char *message) {
  if (out) {
    size_t length = strlen(message) + 1;
    *out = malloc(length);
    assert(*out);
    memcpy(*out, message, length);
  }
  return -1;
}
#include "../shims/published_mir/hako_llvmc_ffi_published_static_method.inc"

static void test_prepass_peek_and_emitter_take(void) {
  hako_llvmc_published_static_method_call_v1 row = {0};
  row.function_name = "renamed_physical_function";
  row.block_id = 7;
  row.instruction_index = 3;
  row.kind = HAKO_LLVMC_PUBLISHED_CALL_KIND_FREE_FUNCTION;
  row.target_symbol = "opaque_target";
  row.arity = 1;
  yyjson_doc *doc = yyjson_read("[1]", 3, 0);
  assert(doc);
  yyjson_val *args = yyjson_doc_get_root(doc);
  char *error = NULL;
  const hako_llvmc_published_static_method_call_v1 *found = NULL;
  assert(hako_llvmc_published_static_method_rows_begin(&row, 1, &error) == 0);
  for (int i = 0; i < 2; i++) {
    assert(hako_llvmc_published_static_method_peek_i64_global_row_v1(
        row.function_name, 7, 3, "Global", 2, args, &found) == 1);
    assert(found == &row);
  }
  assert(hako_llvmc_published_static_method_rows_finish(&error) != 0);
  assert(error && strstr(error, "typed row was not consumed"));
  free(error);
  error = NULL;
  /* Coordinates must not wrap down to the valid u32 row. */
  assert(!hako_llvmc_published_static_method_peek_row_for_site(
      row.function_name, 4294967303LL, 3));
  if (SIZE_MAX > UINT32_MAX)
    assert(!hako_llvmc_published_static_method_peek_row_for_site(
        row.function_name, 7, (size_t)UINT32_MAX + 4));
  assert(!hako_llvmc_published_static_method_peek_row_for_site("foreign", 7, 3));
  assert(hako_llvmc_published_static_method_take_i64_global_row_v1(
      row.function_name, 7, 3, "Method", 2, args, &found) == -1);
  assert(found == NULL);
  assert(hako_llvmc_published_static_method_take_i64_global_row_v1(
      row.function_name, 7, 3, "Global", 2, args, &found) == 1);
  assert(found == &row);
  assert(hako_llvmc_published_static_method_rows_finish(&error) == 0);
  assert(hako_llvmc_published_static_method_take_i64_global_row_v1(
      row.function_name, 7, 3, "Global", 2, args, &found) == -1);
  assert(found == NULL); /* duplicate is not absence/generic fallback */
  hako_llvmc_published_static_method_rows_end();
  yyjson_doc_free(doc);
}

static void test_array_row_rejects_second_take(void) {
  const char *body = "{\"op\":\"array_element_write\",\"kind\":\"push\","
      "\"site_id\":5,\"receiver\":1,\"value\":2}";
  yyjson_doc *doc = yyjson_read(body, strlen(body), 0);
  assert(doc);
  hako_llvmc_published_static_method_call_v1 row = {0};
  row.function_name = "array_owner";
  row.kind = HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_PUSH;
  row.site_id = 5;
  row.receiver = 1;
  row.value = 2;
  char *error = NULL;
  const hako_llvmc_published_static_method_call_v1 *found = NULL;
  assert(hako_llvmc_published_static_method_rows_begin(&row, 1, &error) == 0);
  assert(hako_llvmc_published_static_method_take_array_write_row_v1(
      row.function_name, 0, 0, yyjson_doc_get_root(doc), &found) == 1);
  assert(found == &row);
  assert(hako_llvmc_published_static_method_take_array_write_row_v1(
      row.function_name, 0, 0, yyjson_doc_get_root(doc), &found) == -1);
  assert(found == NULL);
  assert(hako_llvmc_published_static_method_rows_finish(&error) == 0);
  hako_llvmc_published_static_method_rows_end();
  yyjson_doc_free(doc);
}

static void test_same_module_prepass_uses_published_row(void) {
  /* Physical consumer test, not a source/publication proof. The nested call
   * intentionally has no legacy lowering plan and its JSON name is not the
   * published target. The exact row must serve prepass and emission alike. */
  const char *body =
      "{\"functions\":[{\"name\":\"main\",\"params\":[],\"metadata\":{"
      "\"same_module_function_definitions\":["
      "{\"target_symbol\":\"nested\",\"definition_kind\":\"same_module_function\"},"
      "{\"target_symbol\":\"leaf\",\"definition_kind\":\"same_module_function\"}]},"
      "\"blocks\":[{\"id\":0,\"instructions\":["
      "{\"op\":\"const\",\"dst\":1,\"value\":{\"type\":\"i64\",\"value\":6}},"
      "{\"op\":\"mir_call\",\"dst\":2,\"mir_call\":{"
      "\"callee\":{\"type\":\"Global\",\"name\":\"not_target\"},\"args\":[1]}},"
      "{\"op\":\"ret\",\"value\":2}]}]},"
      "{\"name\":\"nested\",\"params\":[1],\"metadata\":{},\"blocks\":[{"
      "\"id\":1,\"instructions\":[{\"op\":\"mir_call\",\"dst\":2,\"mir_call\":{"
      "\"callee\":{\"type\":\"Global\",\"name\":\"not_target\"},\"args\":[1]}},"
      "{\"op\":\"ret\",\"value\":2}]}]},"
      "{\"name\":\"leaf\",\"params\":[1],\"metadata\":{},\"blocks\":[{"
      "\"id\":2,\"instructions\":[{\"op\":\"ret\",\"value\":1}]}]}]}";
  char input[] = "/tmp/hakorune-published-prepass-XXXXXX";
  int fd = mkstemp(input);
  assert(fd >= 0);
  FILE *file = fdopen(fd, "w");
  assert(file && fputs(body, file) >= 0);
  assert(fclose(file) == 0);
  char output[sizeof(input) + 2];
  snprintf(output, sizeof(output), "%s.o", input);
  hako_llvmc_published_static_method_call_v1 rows[2] = {0};
  rows[0].function_name = "main";
  rows[0].instruction_index = 1;
  rows[0].target_symbol = "nested";
  rows[1].function_name = "nested";
  rows[1].block_id = 1;
  rows[1].target_symbol = "leaf";
  for (int i = 0; i < 2; i++) {
    rows[i].kind = HAKO_LLVMC_PUBLISHED_CALL_KIND_FREE_FUNCTION;
    rows[i].arity = 1;
  }
  char *error = NULL;
  rows[1].arity = 2;
  int rc = hako_llvmc_compile_published_static_method_v1(
      input, rows, 2, output, &error);
  assert(rc != 0 && error && access(output, F_OK) != 0);
  free(error);
  error = NULL;
  rows[1].arity = 1;
  rc = hako_llvmc_compile_published_static_method_v1(
      input, rows, 2, output, &error);
  if (rc != 0) fprintf(stderr, "nested prepass rc=%d: %s\n", rc, error ? error : "none");
  assert(rc == 0 && access(output, F_OK) == 0);
  free(error);
  assert(unlink(input) == 0 && unlink(output) == 0);
}

int main(int argc, char **argv) {
  test_prepass_peek_and_emitter_take();
  test_array_row_rejects_second_take();
  test_same_module_prepass_uses_published_row();
  puts("published peek/take and coordinate tests: PASS");
  if (argc == 1) return 0;
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
