/* Exercise the private plan storage and reader. Names in this test are safe;
 * symbol syntax validation is an unchanged dependency, not tested here. */
#include <assert.h>
#include <stdio.h>
#include <string.h>
#include "yyjson.h"
#define ARR_LEN(a) (sizeof(a) / sizeof((a)[0]))

int main(void) {
  auto const char* read_str(yyjson_val* value, const char* key) {
    return yyjson_get_str(yyjson_obj_get(value, key));
  }
  auto int llvm_quoted_symbol_name_is_safe(const char* name) {
    assert(name && !strchr(name, '"') && !strchr(name, '\n'));
    return 1;
  }
#include "../shims/hako_llvmc_ffi_physical_definition_plan.inc"
  struct PhysicalDefinitionPlan plan = {0};
  auto int read(const char* json) {
    yyjson_doc* doc = yyjson_read(json, strlen(json), 0);
    assert(doc);
    /* Keep the borrowed symbol strings alive for the complete plan test. */
    int result = read_physical_definition_plan(&plan, yyjson_doc_get_root(doc));
    assert(plan.read_result == result);
    memset(&plan, 0, sizeof(plan));
    yyjson_doc_free(doc);
    return result;
  }
  assert(read("{}") == 0);
  assert(read("{\"metadata\":{\"same_module_function_definitions\":null}}") == 0);
  assert(read("{\"metadata\":{\"same_module_function_definitions\":42}}") == -1);
  assert(read("{\"metadata\":{\"same_module_function_definitions\":[{}]}}") == -1);
  const char* json = "{\"metadata\":{\"same_module_function_definitions\":["
      "{\"target_symbol\":\"a\",\"definition_kind\":\"leaf_i64_function\"},"
      "{\"target_symbol\":\"a\",\"definition_kind\":\"same_module_function\"},"
      "{\"target_symbol\":\"a\",\"definition_kind\":\"same_module_function\"},"
      "{\"target_symbol\":\"b\",\"definition_kind\":\"same_module_function\"}]}}";
  yyjson_doc* doc = yyjson_read(json, strlen(json), 0);
  assert(doc && read_physical_definition_plan(&plan, yyjson_doc_get_root(doc)) == 4);
  assert(plan.read_result == 4 && !plan.same_module_overflow);
  assert(plan.leaf_count == 1 && plan.same_module_count == 2);
  assert(!strcmp(plan.same_module[0], "a") && !strcmp(plan.same_module[1], "b"));
  assert(physical_definition_plan_contains(&plan, 0, "a"));
  assert(physical_definition_plan_contains(&plan, 1, "a"));
  yyjson_doc_free(doc);
  memset(&plan, 0, sizeof(plan));
  const char* partial = "{\"metadata\":{\"same_module_function_definitions\":["
      "{\"target_symbol\":\"kept\",\"definition_kind\":\"same_module_function\"},"
      "{\"target_symbol\":\"bad\",\"definition_kind\":\"unknown\"}]}}";
  doc = yyjson_read(partial, strlen(partial), 0);
  assert(doc && read_physical_definition_plan(&plan, yyjson_doc_get_root(doc)) == -1);
  assert(plan.read_result == -1 && !plan.same_module_overflow);
  assert(plan.same_module_count == 1 && !strcmp(plan.same_module[0], "kept"));
  yyjson_doc_free(doc);
  memset(&plan, 0, sizeof(plan));
  char names[1025][24];
  for (size_t i = 0; i < ARR_LEN(names); i++) snprintf(names[i], sizeof(names[i]), "f%zu", i);
  for (int leaf = 0; leaf <= 1; leaf++) {
    memset(&plan, 0, sizeof(plan));
    size_t capacity = leaf ? 256 : 1024;
    for (size_t i = 0; i < capacity; i++)
      assert(physical_definition_plan_add(&plan, leaf, names[i]) == 1);
    assert(physical_definition_plan_add(&plan, leaf, names[0]) == 1);
    assert(physical_definition_plan_add(&plan, leaf, names[capacity]) == 0);
    assert(physical_definition_plan_contains(&plan, leaf, names[capacity-1]));
    assert(!physical_definition_plan_contains(&plan, leaf, names[capacity]));
    assert(leaf ? !plan.same_module_overflow : plan.same_module_overflow == names[capacity]);
  }
  /* No diagnostic callback is in scope: observation only retains detail. */
  puts("physical definition plan: metadata, duplicates, partial failure, capacities PASS");
}
