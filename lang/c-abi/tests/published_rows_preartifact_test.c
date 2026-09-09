#define _POSIX_C_SOURCE 200809L
/* Exercise shared typed admission and the real ABI. Synthetic MIR cases are
 * physical contract witnesses, not source/publication acceptance. */
#include "../include/hako_llvmc_ffi.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <assert.h>
#include "yyjson.h"

#include "static_v2_file_test_helper.h"

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

static void test_selected_call_activity(void) {
  char *error = NULL;
  assert(!hako_llvmc_published_call_rows_active());
  assert(hako_llvmc_published_static_method_rows_begin(NULL, 0, &error) != 0);
  assert(error && !hako_llvmc_published_call_rows_active());
  free(error); error = NULL;
  hako_llvmc_published_static_method_call_v1 row = {0};
  row.function_name = "owner";
  row.target_symbol = "target";
  row.kind = HAKO_LLVMC_PUBLISHED_CALL_KIND_FREE_FUNCTION;
  for (int malformed = 0; malformed < 2; malformed++) {
    assert(hako_llvmc_published_call_rows_begin_v2(
        malformed ? &row : NULL, malformed ? 0 : 1, &error) != 0);
    assert(error && !hako_llvmc_published_call_rows_active());
    free(error); error = NULL;
  }
  assert(hako_llvmc_published_call_rows_begin_v2(NULL, 0, &error) == 0);
  assert(hako_llvmc_published_call_rows_active());
  assert(!hako_llvmc_published_static_method_peek_row_for_site("owner", 0, 0));
  const char *types[] = {"Global", "Extern"};
  for (size_t i = 0; i < 2; i++) {
    assert(hako_llvmc_published_static_method_take_i64_global_row_v1(
        "owner", 0, 0, types[i], -1, NULL, NULL) == -1);
  }
  assert(hako_llvmc_published_static_method_rows_finish(&error) == 0);
  hako_llvmc_published_static_method_rows_end();
  assert(!hako_llvmc_published_call_rows_active());
  assert(hako_llvmc_published_call_rows_begin_v2(&row, 1, &error) == 0);
  assert(hako_llvmc_published_static_method_take_row_v1(&row) == 1);
  /* Failed nested activation preserves both the binding and consumed ledger. */
  assert(hako_llvmc_published_static_method_rows_begin(&row, 1, &error) != 0);
  assert(error && strstr(error, "rows already active"));
  free(error); error = NULL;
  assert(hako_llvmc_published_call_rows.rows == &row);
  assert(hako_llvmc_published_static_method_take_row_v1(&row) == -1);
  assert(hako_llvmc_published_static_method_rows_finish(&error) == 0);
  hako_llvmc_published_static_method_rows_end();
  assert(!hako_llvmc_published_call_rows_active());
}

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
  assert(hako_llvmc_published_static_method_peek_i64_global_row_v1(
      "missing", 7, 3, "Global", 2, args, &found) == -1);
  assert(found == NULL);
  assert(hako_llvmc_published_static_method_take_i64_global_row_v1(
      "missing", 7, 3, "Global", 2, args, &found) == -1);
  assert(hako_llvmc_published_static_method_peek_i64_global_row_v1(
      "missing", 7, 3, "Method", 2, args, &found) == 0);
  assert(hako_llvmc_published_static_method_peek_i64_global_row_v1(
      "missing", 7, 3, "Extern", 2, args, &found) == -1);
  assert(hako_llvmc_published_static_method_take_i64_global_row_v1(
      "missing", 7, 3, "Extern", 2, args, &found) == -1);
  assert(hako_llvmc_published_static_method_take_i64_global_row_v1(
      row.function_name, 7, 3, "Extern", 2, args, &found) == -1);
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
  assert(hako_llvmc_published_static_method_peek_i64_global_row_v1(
      "missing", 7, 3, "Global", 2, args, &found) == 0);
  assert(found == NULL);
  assert(hako_llvmc_published_static_method_peek_i64_global_row_v1(
      "missing", 7, 3, "Extern", 2, args, &found) == 0);
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
  int rc = compile_static_test_file_v2(
      input, rows, 2, output, &error);
  assert(rc != 0 && error && access(output, F_OK) != 0);
  free(error);
  error = NULL;
  rows[1].arity = 1;
  rc = compile_static_test_file_v2(
      input, rows, 2, output, &error);
  if (rc != 0) fprintf(stderr, "nested prepass rc=%d: %s\n", rc, error ? error : "none");
  assert(rc == 0 && access(output, F_OK) == 0);
  free(error);
  error = NULL;
  assert(unlink(output) == 0);
  rows[0].kind = rows[1].kind = HAKO_LLVMC_PUBLISHED_CALL_KIND_STATIC_METHOD;
  assert(compile_static_test_file_v2(input, rows, 2, output, &error) == 0);
  assert(error == NULL && access(output, F_OK) == 0);
  assert(unlink(input) == 0 && unlink(output) == 0);
}

/* The explicit generic C entry has no published-row session. */
extern int hako_llvmc_compile_json_pure_first(const char*, const char*, char**);

static void test_missing_global_rows_cannot_use_legacy_names(void) {
  const char *body =
      "{\"functions\":[{\"name\":\"main\",\"params\":[],\"metadata\":{"
      "\"same_module_function_definitions\":[{\"target_symbol\":\"nested\","
      "\"definition_kind\":\"same_module_function\"}]},\"blocks\":[{\"id\":0,"
      "\"instructions\":[{\"op\":\"const\",\"dst\":1,\"value\":{\"type\":\"i64\",\"value\":6}},"
      "{\"op\":\"mir_call\",\"mir_call\":{\"callee\":{\"type\":\"Global\",\"name\":\"print\"},\"args\":[1]}},"
      "{\"op\":\"ret\",\"value\":1}]}]},"
      "{\"name\":\"nested\",\"params\":[1],\"metadata\":{},\"blocks\":[{\"id\":1,"
      "\"instructions\":[{\"op\":\"mir_call\",\"mir_call\":{\"callee\":{\"type\":\"Global\",\"name\":\"print\"},\"args\":[1]}},"
      "{\"op\":\"ret\",\"value\":1}]}]}]}";
  char input[] = "/tmp/hakorune-published-missing-XXXXXX";
  int fd = mkstemp(input);
  assert(fd >= 0);
  FILE *file = fdopen(fd, "w");
  assert(file && fputs(body, file) >= 0 && fclose(file) == 0);
  char output[sizeof(input) + 2];
  snprintf(output, sizeof(output), "%s.o", input);
  hako_llvmc_published_static_method_call_v1 rows[2] = {0};
  rows[0].function_name = "main";
  rows[0].instruction_index = 1;
  rows[1].function_name = "nested";
  rows[1].block_id = 1;
  for (int i = 0; i < 2; i++) {
    rows[i].kind = HAKO_LLVMC_PUBLISHED_CALL_KIND_BUILTIN_PRINT;
    rows[i].arity = 1;
  }
  char *error = NULL;
  assert(compile_static_test_file_v2(input, rows, 2, output, &error) == 0);
  assert(error == NULL && access(output, F_OK) == 0 && unlink(output) == 0);
  for (int retained = 0; retained < 2; retained++) {
    int rc = compile_static_test_file_v2(input, &rows[retained], 1, output, &error);
    assert(rc != 0 && error && access(output, F_OK) != 0);
    /* Not a late residual error: the supplied other row is valid. */
    assert(!strstr(error, "typed row was not consumed"));
    free(error);
    error = NULL;
  }
  int rc = hako_llvmc_compile_json_pure_first(input, output, &error);
  if (rc != 0) fprintf(stderr, "generic print rc=%d: %s\n", rc, error ? error : "none");
  assert(rc == 0 && access(output, F_OK) == 0);
  free(error);
  error = NULL;
  assert(unlink(output) == 0);
  const char *global = "{\"type\":\"Global\",\"name\":\"print\"}";
  const char *external = "{\"type\":\"Extern\",\"name\":\"nyash.console.log\"}";
  for (int site = 0; site < 2; site++) {
    const char *at = strstr(body, global);
    if (site == 1) at = strstr(at + strlen(global), global);
    assert(at);
    file = fopen(input, "w");
    assert(file);
    assert(fwrite(body, 1, (size_t)(at - body), file) == (size_t)(at - body));
    assert(fputs(external, file) >= 0 && fputs(at + strlen(global), file) >= 0);
    assert(fclose(file) == 0);
    rc = compile_static_test_file_v2(
        input, &rows[1 - site], 1, output, &error);
    assert(rc != 0 && error && access(output, F_OK) != 0);
    assert(!strstr(error, "typed row was not consumed"));
    assert(!strstr(error, "extern_call_missing_plan"));
    if (site == 0) assert(strstr(error, "published_extern_not_allowed"));
    free(error);
    error = NULL;
    rc = hako_llvmc_compile_json_pure_first(input, output, &error);
    /* Generic admission still reaches its existing required-plan terminal. */
    assert(rc != 0 && error && strstr(error, "extern_call_missing_plan"));
    assert(access(output, F_OK) != 0);
    free(error);
    error = NULL;
  }
  assert(unlink(input) == 0);
}

static void test_intrinsic_array_allocation_rows(void) {
  const char *valid = "\"target\":{\"kind\":\"intrinsic_array\"},\"args\":[]";
  const char *bad[] = {
      "\"type\":\"ArrayBox\",\"args\":[]",
      "\"target\":{\"kind\":\"intrinsic_array\"},\"type\":\"ArrayBox\",\"args\":[]",
      "\"target\":{\"kind\":\"other\"},\"args\":[]",
      "\"target\":{\"kind\":\"intrinsic_array\"},\"args\":[1]",
      "\"target\":null,\"args\":[]"
  };
  const char *format =
      "{\"functions\":[{\"name\":\"main\",\"params\":[],\"metadata\":{"
      "\"same_module_function_definitions\":[{\"target_symbol\":\"nested\","
      "\"definition_kind\":\"same_module_function\"}]},\"blocks\":[{\"id\":0,"
      "\"instructions\":[{\"op\":\"newbox\",\"dst\":0,%s},"
      "{\"op\":\"const\",\"dst\":2,\"value\":{\"type\":\"i64\",\"value\":30}},"
      "{\"op\":\"ret\",\"value\":2}]}]},"
      "{\"name\":\"nested\",\"params\":[],\"metadata\":{},\"blocks\":[{\"id\":1,"
      "\"instructions\":[{\"op\":\"newbox\",\"dst\":1,%s},"
      "{\"op\":\"const\",\"dst\":2,\"value\":{\"type\":\"i64\",\"value\":4}},"
      "{\"op\":\"ret\",\"value\":2}]}]}]}";
  hako_llvmc_published_static_method_call_v1 rows[2] = {0};
  rows[0].function_name = "main";
  rows[1].function_name = "nested";
  rows[1].block_id = rows[1].dst = 1;
  for (int i = 0; i < 2; i++) {
    rows[i].kind = HAKO_LLVMC_PUBLISHED_CALL_KIND_INTRINSIC_ARRAY_NEW;
    rows[i].flags = HAKO_LLVMC_PUBLISHED_ROW_FLAG_DST_PRESENT;
  }
  char input[] = "/tmp/hakorune-intrinsic-array-XXXXXX";
  int fd = mkstemp(input);
  assert(fd >= 0 && close(fd) == 0);
  char output[sizeof(input) + 2], body[4096];
  snprintf(output, sizeof(output), "%s.o", input);
  char *error = NULL;
  for (int test = -1; test < (int)(sizeof(bad) / sizeof(bad[0])); test++) {
    for (int site = 0; site < 2; site++) {
      const char *first = test >= 0 && site == 0 ? bad[test] : valid;
      const char *second = test >= 0 && site == 1 ? bad[test] : valid;
      int length = snprintf(body, sizeof(body), format, first, second);
      assert(length > 0 && length < (int)sizeof(body));
      FILE *file = fopen(input, "w");
      assert(file && fputs(body, file) >= 0 && fclose(file) == 0);
      int rc = compile_static_test_file_v2(input, rows, 2, output, &error);
      if (test < 0) {
        if (rc) fprintf(stderr, "intrinsic allocation rc=%d: %s\n", rc, error ? error : "none");
        assert(rc == 0 && access(output, F_OK) == 0 && unlink(output) == 0);
        assert(compile_static_test_file_v2(
            input, &rows[site], 1, output, &error) != 0);
        assert(error && access(output, F_OK) != 0);
        free(error); error = NULL;
        assert(hako_llvmc_compile_json_pure_first(input, output, &error) != 0);
        assert(error && access(output, F_OK) != 0);
      } else {
        assert(rc != 0 && error && access(output, F_OK) != 0);
      }
      free(error); error = NULL;
    }
  }
  yyjson_doc *doc = yyjson_read(
      "{\"op\":\"newbox\",\"target\":{\"kind\":\"intrinsic_array\"},\"args\":[],\"dst\":0}",
      strlen("{\"op\":\"newbox\",\"target\":{\"kind\":\"intrinsic_array\"},\"args\":[],\"dst\":0}"), 0);
  assert(doc);
  assert(hako_llvmc_published_static_method_rows_begin(rows, 2, &error) == 0);
  assert(hako_llvmc_published_intrinsic_array_peek_v1("main", 0, 0, yyjson_doc_get_root(doc), NULL) == 1);
  assert(hako_llvmc_published_intrinsic_array_take_v1("main", 0, 0, yyjson_doc_get_root(doc)) == 1);
  assert(hako_llvmc_published_intrinsic_array_take_v1("main", 0, 0, yyjson_doc_get_root(doc)) == -1);
  assert(hako_llvmc_published_static_method_rows_finish(&error) != 0);
  free(error); error = NULL;
  hako_llvmc_published_static_method_rows_end();
  for (int field = 0; field < 5; field++) {
    hako_llvmc_published_static_method_call_v1 malformed = rows[0];
    if (field == 0) malformed.flags = 0;
    if (field == 1) malformed.receiver = 1;
    if (field == 2) malformed.arity = 1;
    if (field == 3) malformed.target_symbol = "ArrayBox";
    if (field == 4) malformed.dst = UINT32_MAX;
    assert(hako_llvmc_published_static_method_rows_begin(&malformed, 1, &error) != 0);
    assert(error); free(error); error = NULL;
  }
  yyjson_doc_free(doc);
  assert(unlink(input) == 0);
  puts("intrinsic array allocation entry/nested and rejection: PASS");
}

int main(int argc, char **argv) {
  test_selected_call_activity();
  test_intrinsic_array_allocation_rows();
  test_prepass_peek_and_emitter_take();
  test_array_row_rejects_second_take();
  test_same_module_prepass_uses_published_row();
  test_missing_global_rows_cannot_use_legacy_names();
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
  int rc = compile_static_test_file_v2(
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
