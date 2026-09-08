/* Instrument document ownership without production hooks or public exports. */
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "yyjson.h"
static unsigned read_attempts, parsed, freed;
static yyjson_doc* active_document;
/* This driver's static/pure routes only realloc the Named outcome table. */
static void* active_outcomes;
static unsigned realloc_attempts, fail_realloc_at;
static void* counted_realloc(void* ptr, size_t size) {
  assert(active_document && ptr == active_outcomes);
  if (++realloc_attempts == fail_realloc_at) return NULL;
  void* result = realloc(ptr, size);
  if (result) active_outcomes = result;
  return result;
}
static void counted_free(void* ptr) {
  if (ptr && ptr == active_outcomes) {
    assert(active_document);
    active_outcomes = NULL;
  }
  free(ptr);
}
static yyjson_doc* counted_read_file(const char* path, yyjson_read_flag flags,
                                    const yyjson_alc* alc, yyjson_read_err* err) {
  read_attempts++;
  yyjson_doc* doc = yyjson_read_file(path, flags, alc, err);
  if (doc) { assert(!active_document); active_document = doc; parsed++; }
  return doc;
}
static void counted_doc_free(yyjson_doc* doc) {
  assert(doc && doc == active_document && !active_outcomes);
  active_document = NULL;
  freed++;
  yyjson_doc_free(doc);
}
#define realloc counted_realloc
#define free counted_free
#define yyjson_read_file counted_read_file
#define yyjson_doc_free counted_doc_free
#include "../shims/hako_llvmc_ffi.c"
#undef yyjson_read_file
#undef yyjson_doc_free
#undef realloc
#undef free

int main(int argc, char** argv) {
  assert(argc == 4);
  const char* failure = getenv("TEST_NAMED_REALLOC_FAIL_AT");
  if (failure) fail_realloc_at = (unsigned)atoi(failure);
  char* error = NULL;
  int rc;
  if (!strcmp(argv[3], "generic")) {
    rc = compile_json_compat_pure(argv[1], argv[2], &error);
  } else {
    hako_llvmc_published_static_method_call_v1 row = {0};
    row.function_name = "main";
    row.instruction_index = 1;
    row.target_symbol = "anchor";
    row.kind = HAKO_LLVMC_PUBLISHED_CALL_KIND_FREE_FUNCTION;
    rc = hako_llvmc_compile_published_static_method_v1(argv[1], &row, 1, argv[2], &error);
  }
  if (fail_realloc_at) assert(realloc_attempts == fail_realloc_at);
  assert(read_attempts == 1 && parsed == freed && !active_document);
  if (error) fprintf(stderr, "%s\n", error);
  free(error);
  printf("rc=%d reads=%u parsed=%u freed=%u\n", rc, read_attempts, parsed, freed);
  return 0;
}
