/* Invocation-private state representation. Source meaning and wire/runtime ABI
 * are unchanged. Indexed origins fit int; tags cannot overlap those payloads. */
#include <limits.h>
typedef uint64_t hako_lv4_state;
#define LV4_TRANSFER_BASE (UINT64_C(1) << 32)
#define LV4_STRING_RANK_BASE (UINT64_C(2) << 32)

static hako_lv4_state lv4_map_transfer(int origin) {
  return LV4_TRANSFER_BASE + (uint32_t)origin;
}

/* Temporary and installed-Home ranks are disjoint ordered domains. */
#define LV4_STRING_HOME_BASE (UINT64_C(3) << 32)
#define LV4_STRING_LIMIT (UINT64_C(4) << 32)
static hako_lv4_state lv4_string_domain_top(const hako_lv4_state* state, size_t count,
    hako_lv4_state base, hako_lv4_state limit) {
  hako_lv4_state top = base;
  for (size_t i = 0; i < count; i++) if (state[i] > top && state[i] < limit) top = state[i];
  return top;
}
static hako_lv4_state lv4_string_top(const hako_lv4_state* state, size_t count) {
  hako_lv4_state top = lv4_string_domain_top(state, count, LV4_STRING_RANK_BASE, LV4_STRING_HOME_BASE);
  return top > LV4_STRING_RANK_BASE ? top :
      lv4_string_domain_top(state, count, LV4_STRING_HOME_BASE, LV4_STRING_LIMIT);
}
static int lv4_string_push(const hako_lv4_state* state, size_t count,
    hako_lv4_state* result) {
  hako_lv4_state top = lv4_string_domain_top(state, count, LV4_STRING_RANK_BASE, LV4_STRING_HOME_BASE);
  if (top + 1 >= LV4_STRING_HOME_BASE) return 0;
  *result = top + 1;
  return 1;
}
static int lv4_string_install(hako_lv4_state* state, size_t count, int origin) {
  if (origin < 0 || (size_t)origin >= count || state[origin] <= LV4_STRING_RANK_BASE ||
      state[origin] >= LV4_STRING_HOME_BASE || state[origin] !=
      lv4_string_domain_top(state, count, LV4_STRING_RANK_BASE, LV4_STRING_HOME_BASE)) return 0;
  hako_lv4_state top = lv4_string_domain_top(state, count, LV4_STRING_HOME_BASE, LV4_STRING_LIMIT);
  if (top + 1 >= LV4_STRING_LIMIT) return 0;
  state[origin] = top + 1;
  return 1;
}

static void* lv4_checked_calloc(size_t count, size_t size) {
  if (!size || count > SIZE_MAX / size) return NULL;
  return calloc(count, size);
}
