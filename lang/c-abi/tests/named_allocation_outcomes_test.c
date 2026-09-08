/* Real conditional selector/outcome owner; no execution-coverage claims. */
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "yyjson.h"
enum { HAKO_LLVMC_RUNTIME_MODE_REQUIRES_DIRECT_ARRAY_I64_EXACT = 2 };
struct HakoLlvmcAllocationConfig { int runtime_flags; int exact_slot_helper; };
#include "../shims/hako_llvmc_ffi_typed_object_root_lookup.inc"
#include "../shims/hako_llvmc_ffi_named_allocation_select.inc"
static unsigned selections, allocations, releases;
static int fail_allocation;
static enum NamedAllocationConsumer counted_select(yyjson_val* root,
    enum NamedAllocationWalker walker, const char* target, long long dst,
    long long arg0, int direct, struct GenericPureTypedObjectNewPlanView* plan) {
  selections++;
  return select_named_allocation_consumer(root, walker, target, dst, arg0, direct, plan);
}
static void* counted_realloc(void* ptr, size_t size) {
  if (fail_allocation) return NULL;
  if (!ptr) allocations++;
  return realloc(ptr, size);
}
static void counted_free(void* ptr) { if (ptr) releases++; free(ptr); }
#define select_named_allocation_consumer counted_select
#define realloc counted_realloc
#define free counted_free
#include "../shims/hako_llvmc_ffi_named_allocation_outcomes.inc"
#undef select_named_allocation_consumer
#undef realloc
#undef free
int main(void) {
  const char* json = "{\"functions\":["
      "{\"name\":\"same\",\"blocks\":[{\"id\":0,\"instructions\":["
      "{\"op\":\"newbox\",\"type\":\"StringBox\",\"dst\":1,\"args\":[2]},"
      "{\"op\":\"newbox\",\"type\":\"NoPlan\",\"dst\":3},"
      "{\"op\":\"newbox\",\"target\":null},"
      "{\"op\":\"newbox\",\"target\":{\"kind\":\"unknown\"}},"
      "{\"op\":\"newbox\",\"target\":{\"kind\":\"IntrinsicArray\"}}]}]},"
      "{\"name\":\"same\",\"blocks\":[{\"id\":0,\"instructions\":["
      "{\"op\":\"newbox\",\"type\":\"MapBox\",\"dst\":1}]}]}],"
      "\"typed_object_plans\":[{\"box_name\":\"StringBox\",\"type_id\":11,\"field_count\":0}]}";
  yyjson_doc* doc = yyjson_read(json, strlen(json), 0);
  assert(doc);
  yyjson_val* root = yyjson_doc_get_root(doc);
  yyjson_val* functions = yyjson_obj_get(root, "functions");
  yyjson_val* first = yyjson_arr_get(functions, 0);
  yyjson_val* second = yyjson_arr_get(functions, 1);
  yyjson_val* instructions = yyjson_obj_get(yyjson_arr_get(yyjson_obj_get(first, "blocks"), 0), "instructions");
  yyjson_val* other = yyjson_arr_get(yyjson_obj_get(yyjson_arr_get(yyjson_obj_get(second, "blocks"), 0), "instructions"), 0);
  struct HakoLlvmcAllocationConfig config = {0};
  struct NamedAllocationOutcomes owner;
  named_allocation_outcomes_init(&owner, root, &config);
  assert(!owner.storage_failed && owner.count == 3 && selections == 6);
  for (int repeat = 0; repeat < 3; repeat++) {
    const struct NamedAllocationOutcome* generic = named_allocation_outcomes_find(
        &owner, first, yyjson_arr_get(instructions, 0), NAMED_ALLOCATION_GENERIC);
    const struct NamedAllocationOutcome* same = named_allocation_outcomes_find(
        &owner, first, yyjson_arr_get(instructions, 0), NAMED_ALLOCATION_SAME_MODULE);
    assert(generic && generic->consumer == NAMED_ALLOCATION_ALIAS_OPERAND_ZERO);
    assert(same && same->consumer == NAMED_ALLOCATION_TYPED_OBJECT && same->plan.type_id == 11);
    assert(named_allocation_outcomes_find(&owner, second, other, NAMED_ALLOCATION_GENERIC)->consumer == NAMED_ALLOCATION_MAP);
    assert(!named_allocation_outcomes_find(&owner, first, other, NAMED_ALLOCATION_GENERIC));
    assert(named_allocation_outcomes_find(&owner, first, yyjson_arr_get(instructions, 1), NAMED_ALLOCATION_GENERIC)->consumer == NAMED_ALLOCATION_UNSUPPORTED);
  }
  assert(selections == 6);
  for (size_t i = 2; i < 5; i++)
    assert(!named_allocation_outcomes_find(&owner, first, yyjson_arr_get(instructions, i), NAMED_ALLOCATION_GENERIC));
  named_allocation_outcomes_destroy(&owner);
  assert(allocations == releases && !owner.rows);
  fail_allocation = 1;
  named_allocation_outcomes_init(&owner, root, &config);
  assert(owner.storage_failed && !named_allocation_outcomes_find(
      &owner, first, yyjson_arr_get(instructions, 0), NAMED_ALLOCATION_GENERIC));
  named_allocation_outcomes_destroy(&owner);
  assert(allocations == releases);
  yyjson_doc_free(doc);
  puts("Named outcomes: identity, repeated reads, intrinsic exclusion, deferred failure PASS");
}
