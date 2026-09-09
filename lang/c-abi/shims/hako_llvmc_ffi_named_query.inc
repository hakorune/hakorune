#include "../include/hako_llvmc_ffi.h"
/* Private exact-coordinate query. Does not activate rows, emit diagnostics,
 * classify names, reselect consumers, or claim execution/coverage. */


static int hako_llvmc_named_query_u32(yyjson_val* value, uint32_t* out) {
  if (!yyjson_is_int(value) || (yyjson_is_sint(value) && yyjson_get_sint(value) < 0) ||
      yyjson_get_uint(value) > UINT32_MAX) return 0;
  *out = (uint32_t)yyjson_get_uint(value);
  return 1;
}

static enum HakoLlvmcNamedQueryStatus hako_llvmc_invocation_query_named(
    struct HakoLlvmcInvocation* invocation, const char* name, size_t name_len,
    uint32_t block_id, uint32_t instruction_index,
    const struct NamedAllocationOutcome** out) {
  *out = NULL;
  if (invocation->outcomes.storage_failed) return NAMED_QUERY_STORAGE_FAILED;
  if (!name || !name_len || memchr(name, 0, name_len)) return NAMED_QUERY_UNADDRESSABLE;
  if (!hako_llvmc_invocation_observe_program(invocation)) return NAMED_QUERY_PROGRAM_UNAVAILABLE;
  yyjson_val* functions = invocation->program.functions;
  yyjson_val* function = NULL;
  for (size_t i = 0; i < yyjson_arr_size(functions); i++) {
    yyjson_val* candidate = yyjson_arr_get(functions, i);
    yyjson_val* value = yyjson_obj_get(candidate, "name");
    const char* candidate_name = yyjson_get_str(value);
    if (candidate_name && yyjson_get_len(value) == name_len && !memcmp(name, candidate_name, name_len)) {
      if (function) return NAMED_QUERY_UNADDRESSABLE;
      function = candidate;
    }
  }
  if (!function) return NAMED_QUERY_NOT_OBSERVED;
  yyjson_val* blocks = yyjson_obj_get(function, "blocks");
  if (!yyjson_is_arr(blocks)) return NAMED_QUERY_UNADDRESSABLE;
  yyjson_val* block = NULL;
  for (size_t i = 0; i < yyjson_arr_size(blocks); i++) {
    yyjson_val* candidate = yyjson_arr_get(blocks, i);
    uint32_t id;
    if (!hako_llvmc_named_query_u32(yyjson_obj_get(candidate, "id"), &id)) return NAMED_QUERY_UNADDRESSABLE;
    if (id == block_id) {
      if (block) return NAMED_QUERY_UNADDRESSABLE;
      block = candidate;
    }
  }
  if (!block) return NAMED_QUERY_NOT_OBSERVED;
  yyjson_val* instructions = yyjson_obj_get(block, "instructions");
  if (!yyjson_is_arr(instructions) || yyjson_arr_size(instructions) > (uint64_t)UINT32_MAX + 1)
    return NAMED_QUERY_UNADDRESSABLE;
  yyjson_val* instruction = yyjson_arr_get(instructions, instruction_index);
  *out = named_allocation_outcomes_find(&invocation->outcomes, function, instruction,
      function == invocation->program.entry.fn ? NAMED_ALLOCATION_GENERIC : NAMED_ALLOCATION_SAME_MODULE);
  return *out ? NAMED_QUERY_BOUND : NAMED_QUERY_NOT_OBSERVED;
}
