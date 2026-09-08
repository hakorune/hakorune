/* Actual root reader + selector: first-match and independent document binding. */
#include <assert.h>
#include <stdio.h>
#include <string.h>
#include "yyjson.h"
#include "../shims/hako_llvmc_ffi_typed_object_root_lookup.inc"
#include "../shims/hako_llvmc_ffi_named_allocation_select.inc"
int main(void) {
  const char* first = "{\"typed_object_plans\":["
      "{\"box_name\":\"User\",\"type_id\":11,\"field_count\":2},"
      "{\"box_name\":\"User\",\"type_id\":22,\"field_count\":3}]}";
  const char* second = "{\"typed_object_plans\":["
      "{\"box_name\":\"User\",\"type_id\":0,\"field_count\":2},"
      "{\"box_name\":\"User\",\"type_id\":11,\"field_count\":2}]}";
  yyjson_doc* a = yyjson_read(first, strlen(first), 0);
  yyjson_doc* b = yyjson_read(second, strlen(second), 0);
  assert(a && b);
  yyjson_val* root = yyjson_doc_get_root(a);
  struct GenericPureTypedObjectNewPlanView plan = {0};
  int index = -1;
  assert(typed_object_plan_count(root) == 2);
  assert(find_typed_object_plan_index(root, "User", &index) && index == 0);
  assert(!typed_object_plan_at_index(root, -1));
  assert(!typed_object_plan_at_index(root, 2));
  for (int walker = NAMED_ALLOCATION_GENERIC; walker <= NAMED_ALLOCATION_SAME_MODULE; walker++) {
    assert(select_named_allocation_consumer(root, walker, "User", 1, 0, 0, &plan)
           == NAMED_ALLOCATION_TYPED_OBJECT);
    assert(plan.typed_plan_index == 0 && plan.type_id == 11 && plan.field_count == 2);
    assert(select_named_allocation_consumer(yyjson_doc_get_root(b), walker,
                                           "User", 1, 0, 0, &plan)
           == NAMED_ALLOCATION_INVALID_PLAN);
    assert(select_named_allocation_consumer(NULL, walker, "User", 1, 0, 0, &plan)
           == NAMED_ALLOCATION_UNSUPPORTED);
  }
  yyjson_doc_free(a);
  yyjson_doc_free(b);
  puts("typed plan root: first-match and document isolation passed");
}
