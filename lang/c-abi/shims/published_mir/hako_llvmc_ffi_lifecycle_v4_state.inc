/* Invocation-private state representation. Source meaning and wire/runtime ABI
 * are unchanged. Indexed origins fit int; tags cannot overlap those payloads. */
#include <limits.h>
typedef uint64_t hako_lv4_state;
#define LV4_TRANSFER_BASE (UINT64_C(1) << 32)
#define LV4_STRING_RANK_BASE (UINT64_C(2) << 32)

static hako_lv4_state lv4_map_transfer(int origin) {
  return LV4_TRANSFER_BASE + (uint32_t)origin;
}

static hako_lv4_state lv4_string_top(const hako_lv4_state* state, size_t count) {
  hako_lv4_state top = LV4_STRING_RANK_BASE;
  for (size_t i = 0; i < count; i++) if (state[i] > top) top = state[i];
  return top;
}

static int lv4_string_push(const hako_lv4_state* state, size_t count,
    hako_lv4_state* result) {
  hako_lv4_state top = lv4_string_top(state, count);
  if (top == UINT64_MAX) return 0;
  *result = top + 1;
  return 1;
}

static void* lv4_checked_calloc(size_t count, size_t size) {
  if (!size || count > SIZE_MAX / size) return NULL;
  return calloc(count, size);
}
