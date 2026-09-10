/* Test-only frame construction; calls the real private retained compiler.
 * This JSON fixture reader is not a production ingress or projection issuer. */
#include <assert.h>
#include "../shims/hako_llvmc_ffi.c"

static const char* fixture_str(yyjson_val* row, const char* key) {
  return yyjson_get_str(yyjson_obj_get(row, key));
}
static uint32_t fixture_u32(yyjson_val* row, const char* key) {
  return (uint32_t)yyjson_get_uint(yyjson_obj_get(row, key));
}

int main(int argc, char** argv) {
  assert(argc == 4);
  yyjson_doc* document = yyjson_read_file(argv[1], 0, NULL, NULL);
  yyjson_doc* fixture = yyjson_read_file(argv[2], 0, NULL, NULL);
  assert(document && fixture);
  yyjson_val* root = yyjson_doc_get_root(fixture);
  yyjson_val* calls_json = yyjson_obj_get(root, "calls");
  yyjson_val* maps_json = yyjson_obj_get(root, "maps");
  yyjson_val* values_json = yyjson_obj_get(root, "values");
  yyjson_val* expanded_json = yyjson_obj_get(root, "expanded");
  size_t ne = yyjson_arr_size(expanded_json);
  hako_llvmc_expanded_function_v2* expanded = ne ? calloc(ne, sizeof(*expanded)) : NULL;
  assert(!ne || expanded);
  size_t nc = yyjson_arr_size(calls_json), nm = yyjson_arr_size(maps_json), nv = yyjson_arr_size(values_json);
  hako_llvmc_published_static_method_call_v1* calls = nc ? calloc(nc, sizeof(*calls)) : NULL;
  hako_llvmc_map_operation_v2* maps = nm ? calloc(nm, sizeof(*maps)) : NULL;
  hako_llvmc_value_projection_v2* values = nv ? calloc(nv, sizeof(*values)) : NULL;
  assert((!nc || calls) && (!nm || maps) && (!nv || values));
  for (size_t i = 0; i < nc; i++) {
    yyjson_val* row = yyjson_arr_get(calls_json, i);
    calls[i].function_name = fixture_str(row, "function");
    calls[i].block_id = fixture_u32(row, "block");
    calls[i].instruction_index = fixture_u32(row, "instruction");
    calls[i].target_symbol = fixture_str(row, "target");
    calls[i].kind = fixture_u32(row, "kind");
    calls[i].arity = fixture_u32(row, "arity");
    calls[i].site_id = fixture_u32(row, "site_id");
    calls[i].receiver = fixture_u32(row, "receiver");
    calls[i].index = fixture_u32(row, "index");
    calls[i].value = fixture_u32(row, "value");
    calls[i].dst = fixture_u32(row, "dst");
    calls[i].flags = fixture_u32(row, "flags");
  }
  for (size_t i = 0; i < nm; i++) {
    yyjson_val* row = yyjson_arr_get(maps_json, i);
    maps[i] = (hako_llvmc_map_operation_v2){fixture_str(row, "function"),
        fixture_u32(row, "block"), fixture_u32(row, "instruction"), fixture_u32(row, "kind"), fixture_u32(row, "reserved")};
  }
  for (size_t i = 0; i < nv; i++) {
    yyjson_val* row = yyjson_arr_get(values_json, i);
    values[i] = (hako_llvmc_value_projection_v2){fixture_str(row, "function"),
        fixture_u32(row, "value"), fixture_u32(row, "action"), fixture_u32(row, "kind"),
        fixture_u32(row, "encoding"), fixture_u32(row, "flags"), fixture_u32(row, "ordinal"),
        fixture_u32(row, "operation"), yyjson_get_uint(yyjson_obj_get(row, "payload"))};
  }
  for (size_t i = 0; i < ne; i++) {
    yyjson_val* row = yyjson_arr_get(expanded_json, i);
    expanded[i] = (hako_llvmc_expanded_function_v2){fixture_str(row, "function"), fixture_str(row, "target")};
  }
  hako_llvmc_published_static_frame_v2 frame = {
      HAKO_LLVMC_STATIC_FRAME_REVISION, sizeof(frame), calls, nc, maps, nm, values, nv, expanded, ne};
  if (yyjson_obj_get(root, "revision")) frame.revision = fixture_u32(root, "revision");
  if (yyjson_obj_get(root, "byte_size")) frame.byte_size = fixture_u32(root, "byte_size");
  if (yyjson_obj_get(root, "value_count")) frame.value_count = yyjson_get_uint(yyjson_obj_get(root, "value_count"));
  struct HakoLlvmcInvocation invocation;
  hako_llvmc_invocation_init(&invocation, document, hako_llvmc_capture_allocation_config(),
      HAKO_LLVMC_INGRESS_STATIC_V2);
  char* error = NULL;
  int rc = hako_llvmc_compile_static_v2_retained(&invocation, &frame, argv[3], &error);
  assert(!invocation.static_v2.frame && !hako_llvmc_published_call_rows_active());
  if (error) fprintf(stderr, "%s\n", error);
  free(error);
  hako_llvmc_invocation_destroy(&invocation);
  yyjson_doc_free(fixture);
  free(calls); free(maps); free(values); free(expanded);
  printf("rc=%d\n", rc);
  return rc == 0 ? 0 : 1;
}
