/* Physical selection only: malformed shadowed plans must never be consulted.
 * Actual emitter/object evidence is in named_allocation_emission_test.py. */
#include <assert.h>
#include <stdio.h>
#include <string.h>

typedef struct yyjson_val yyjson_val;
static int present = 0, lookups = 0, reads = 0;
static long long type_id = 11, field_count = 2;
static int find_typed_object_plan_index(yyjson_val* root, const char* name, int* index) {
  assert(!root);
  assert(name);
  lookups++;
  if (!present) return 0;
  *index = 7;
  return 1;
}
static long long typed_object_plan_type_id(yyjson_val* root, int index) {
  assert(!root);
  assert(index == 7); reads++; return type_id;
}
static long long typed_object_plan_field_count(yyjson_val* root, int index) {
  assert(!root);
  assert(index == 7); reads++; return field_count;
}
#include "../shims/hako_llvmc_ffi_named_allocation_select.inc"

int main(void) {
  struct GenericPureTypedObjectNewPlanView plan;
  auto void expect(enum NamedAllocationWalker walker, const char* name,
      long long dst, long long arg0, int exact,
      enum NamedAllocationConsumer expected, int consult) {
    lookups = reads = 0;
    plan = (struct GenericPureTypedObjectNewPlanView){-9, -9, -9};
    assert(select_named_allocation_consumer(NULL, walker, name, dst, arg0,
                                            exact, &plan) == expected);
    assert(lookups == consult);
    assert(reads == (consult && present ? 2 : 0));
    if (expected == NAMED_ALLOCATION_TYPED_OBJECT) {
      assert(plan.typed_plan_index == 7 && plan.type_id == type_id);
      assert(plan.field_count == field_count);
    } else {
      assert(plan.typed_plan_index == -9 && plan.type_id == -9);
      assert(plan.field_count == -9);
    }
  }
  for (int walker = NAMED_ALLOCATION_GENERIC;
       walker <= NAMED_ALLOCATION_SAME_MODULE; walker++) {
    for (int bad = 0; bad < 2; bad++) {
      present = bad != 0;
      type_id = 0; field_count = -1;
      for (int dst = 0; dst <= 1; dst++) {
        expect(walker, "DirectArrayI64", dst, 0, 0, NAMED_ALLOCATION_DIRECT_ARRAY, 0);
        expect(walker, "ArrayBox", dst, 0, 0, NAMED_ALLOCATION_ARRAY, 0);
        expect(walker, "ArrayBox", dst, 0, 1, NAMED_ALLOCATION_DIRECT_ARRAY, 0);
        expect(walker, "MapBox", dst, 0, 0, NAMED_ALLOCATION_MAP, 0);
      }
      if (walker == NAMED_ALLOCATION_GENERIC) {
        expect(walker, "FileBox", 0, 0, 0, NAMED_ALLOCATION_FILE, 0);
        expect(walker, "StringBox", 1, 3, 0, NAMED_ALLOCATION_ALIAS_OPERAND_ZERO, 0);
        expect(walker, "StringBox", -1, -3, 0, NAMED_ALLOCATION_ALIAS_OPERAND_ZERO, 0);
      }
    }
    const char* fallback[] = {"User", "StringBox", "FileBox"};
    for (size_t i = 0; i < sizeof(fallback) / sizeof(*fallback); i++) {
      const char* name = fallback[i];
      if (walker == NAMED_ALLOCATION_GENERIC && !strcmp(name, "FileBox")) continue;
      present = 0;
      expect(walker, name, 1, 0, 0, NAMED_ALLOCATION_UNSUPPORTED, 1);
      present = 1; type_id = 11; field_count = 0;
      expect(walker, name, 1, 0, 0, NAMED_ALLOCATION_TYPED_OBJECT, 1);
      expect(walker, name, 0, 0, 0, NAMED_ALLOCATION_INVALID_PLAN, 1);
      type_id = 0;
      expect(walker, name, 1, 0, 0, NAMED_ALLOCATION_INVALID_PLAN, 1);
      type_id = 11; field_count = -1;
      expect(walker, name, 1, 0, 0, NAMED_ALLOCATION_INVALID_PLAN, 1);
    }
    expect(walker, NULL, 1, 1, 0, NAMED_ALLOCATION_UNSUPPORTED, 0);
  }
  present = 1; type_id = 11; field_count = 2;
  expect(NAMED_ALLOCATION_SAME_MODULE, "StringBox", 1, 3, 0,
         NAMED_ALLOCATION_TYPED_OBJECT, 1);
  puts("Named allocation selector: precedence, lazy plan reads and alias boundaries passed");
}
