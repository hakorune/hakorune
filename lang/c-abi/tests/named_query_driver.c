/* Temporary private-query integration driver; retire with the public V2 host
 * witness. Includes actual C implementation, never adds a production export. */
#define main document_lifetime_driver_main
#include "pure_document_lifetime_driver.c"
#undef main

static unsigned program_reads, selections;
void __cyg_profile_func_enter(void*, void*) __attribute__((no_instrument_function));
void __cyg_profile_func_exit(void*, void*) __attribute__((no_instrument_function));
void __cyg_profile_func_enter(void* function, void* caller) {
  (void)caller;
  if (function == (void*)hako_llvmc_read_generic_pure_program_view) program_reads++;
  if (function == (void*)select_named_allocation_consumer) selections++;
}
void __cyg_profile_func_exit(void* function, void* caller) { (void)function; (void)caller; }

int main(int argc, char** argv) {
  assert(argc == 4);
  const char* failure = getenv("TEST_NAMED_REALLOC_FAIL_AT");
  if (failure) fail_realloc_at = (unsigned)atoi(failure);
  char* error = NULL;
  yyjson_doc* document = hako_json_v1_read_owned_file(argv[1], &error);
  assert(document && !error);
  struct HakoLlvmcInvocation invocation;
  hako_llvmc_invocation_init(&invocation, document, hako_llvmc_capture_allocation_config());
  unsigned initial_selections = selections;
  yyjson_doc* requests = yyjson_read(argv[3], strlen(argv[3]), 0);
  assert(requests);
  yyjson_val* request_root = yyjson_doc_get_root(requests);
  yyjson_val* call = yyjson_obj_get(request_root, "call");
  yyjson_val* request_list = yyjson_is_arr(request_root) ? request_root
      : yyjson_obj_get(request_root, "queries");
  yyjson_mut_doc* response = yyjson_mut_doc_new(NULL);
  yyjson_mut_val* list = yyjson_mut_arr(response);
  yyjson_mut_doc_set_root(response, list);
  for (size_t i = 0; i < yyjson_arr_size(request_list); i++) {
    yyjson_val* request = yyjson_arr_get(request_list, i);
    yyjson_val* name = yyjson_obj_get(request, "function");
    uint32_t block, ordinal;
    assert(hako_llvmc_named_query_u32(yyjson_obj_get(request, "block"), &block));
    assert(hako_llvmc_named_query_u32(yyjson_obj_get(request, "instruction"), &ordinal));
    const struct NamedAllocationOutcome* outcome = NULL;
    enum HakoLlvmcNamedQueryStatus status = hako_llvmc_invocation_query_named(
        &invocation, yyjson_get_str(name), yyjson_get_len(name), block, ordinal, &outcome);
    yyjson_mut_val* row = yyjson_mut_obj(response);
    yyjson_mut_obj_add_uint(response, row, "status", status);
    if (outcome) yyjson_mut_obj_add_uint(response, row, "consumer", outcome->consumer);
    yyjson_mut_arr_append(list, row);
  }
  assert(!hako_llvmc_published_static_method_rows && !hako_llvmc_published_static_method_row_count);
  assert(selections == initial_selections && program_reads <= 1);
  char* json = yyjson_mut_write(response, 0, NULL);
  assert(json);
  puts(json);
  fflush(stdout);
  free(json);
  yyjson_mut_doc_free(response);
  /* Keep the same invocation alive while Rust plans or rejects the input. */
  int action = getchar();
  int rc = 0;
  if (action == 'c') {
    /* This driver permits V1 compile for non-Map witnesses only. */
    yyjson_val* functions = yyjson_obj_get(yyjson_doc_get_root(document), "functions");
    for (size_t fi = 0; fi < yyjson_arr_size(functions); fi++) {
      yyjson_val* blocks = yyjson_obj_get(yyjson_arr_get(functions, fi), "blocks");
      for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
        yyjson_val* instructions = yyjson_obj_get(yyjson_arr_get(blocks, bi), "instructions");
        for (size_t ii = 0; ii < yyjson_arr_size(instructions); ii++) {
          yyjson_val* ins = yyjson_arr_get(instructions, ii);
          const char* op = yyjson_get_str(yyjson_obj_get(ins, "op"));
          const char* kind = yyjson_get_str(yyjson_obj_get(yyjson_obj_get(ins, "target"), "kind"));
          assert(!op || strcmp(op, "map_literal_entry_write"));
          assert(!kind || strcmp(kind, "intrinsic_map"));
        }
      }
    }
    hako_llvmc_published_static_method_call_v1 row = {0};
    row.function_name = call ? yyjson_get_str(yyjson_obj_get(call, "function")) : "main";
    row.instruction_index = call ? yyjson_get_uint(yyjson_obj_get(call, "instruction")) : 1;
    row.target_symbol = call ? yyjson_get_str(yyjson_obj_get(call, "target")) : "anchor";
    row.kind = call ? yyjson_get_uint(yyjson_obj_get(call, "kind")) : HAKO_LLVMC_PUBLISHED_CALL_KIND_FREE_FUNCTION;
    rc = hako_llvmc_compile_published_static_input_v1(
        &invocation, argv[1], &row, 1, argv[2], &error);
  } else {
    assert(action == 'x' || action == EOF);
  }
  assert(!hako_llvmc_published_static_method_rows && !hako_llvmc_published_static_method_row_count);
  assert(selections == initial_selections && program_reads <= 1);
  yyjson_doc_free(requests);
  hako_llvmc_invocation_destroy(&invocation);
  assert(read_attempts == 1 && parsed == 1 && freed == 1 && !active_document);
  if (error) fprintf(stderr, "%s\n", error);
  free(error);
  printf("rc=%d parsed=%u freed=%u program_reads=%u selections=%u\n",
      rc, parsed, freed, program_reads, selections);
  return 0;
}
