/* Invocation-private state representation. Source meaning and wire/runtime ABI
 * are unchanged. Indexed origins fit int; tags cannot overlap those payloads. */
#include <limits.h>
typedef uint64_t hako_lv4_state;
#define LV4_TRANSFER_BASE (UINT64_C(1) << 32)
#define LV4_STRING_RANK_BASE (UINT64_C(2) << 32)
/* A carrier lease uses its exact SSA origin slot. The kind table distinguishes
 * it from owned object/string/map state and Return requires it to be ended. */
#define LV4_DYNAMIC_CARRIER_LIVE (UINT64_C(5) << 32)

static hako_lv4_state lv4_map_transfer(int origin) {
  return LV4_TRANSFER_BASE + (uint32_t)origin;
}

/* Temporary and installed-Home ranks are disjoint ordered domains. */
#define LV4_STRING_HOME_BASE (UINT64_C(3) << 32)
#define LV4_STRING_LIMIT (UINT64_C(4) << 32)
/* A single pending source replacement is part of the existing per-origin state.
 * No origin is encoded: the old/new markers travel with those exact live ranks. */
#define LV4_STRING_OLD (UINT64_C(1) << 63)
#define LV4_STRING_NEW (UINT64_C(1) << 62)
#define LV4_STRING_RETURN (UINT64_C(1) << 61)
#define LV4_STRING_CALL_HELD (UINT64_C(1) << 60)
/* Carrier descriptors can be live Static, Owned, or either after a join.
 * A virtual Home rank also orders Static residences; zero means spent. */
#define LV4_STRING_STATIC (UINT64_C(1) << 59)
#define LV4_STRING_OWNED (UINT64_C(1) << 58)
#define LV4_STRING_POSSIBLE (LV4_STRING_STATIC | LV4_STRING_OWNED)
#define LV4_STRING_REPLACEMENT (LV4_STRING_OLD | LV4_STRING_NEW)
static hako_lv4_state lv4_string_rank(hako_lv4_state state) {
  return state & ~(LV4_STRING_REPLACEMENT | LV4_STRING_RETURN | LV4_STRING_CALL_HELD | LV4_STRING_POSSIBLE);
}
static int lv4_string_pending(const hako_lv4_state* state, size_t count) {
  for (size_t i = 0; i < count; i++) if (state[i] & LV4_STRING_REPLACEMENT) return 1;
  return 0;
}
static int lv4_string_call_pending(const hako_lv4_state* state, size_t count) {
  for (size_t i = 0; i < count; i++) if (state[i] & LV4_STRING_CALL_HELD) return 1;
  return 0;
}
static int lv4_string_held(const hako_lv4_state* state, size_t count) {
  for (size_t i = 0; i < count; i++) if (state[i] & LV4_STRING_RETURN) return 1;
  return 0;
}
/* Holding an existing residence never changes its acquisition rank. */
static int lv4_string_hold(hako_lv4_state* state, size_t count, int origin) {
  if (origin < 0 || (size_t)origin >= count || lv4_string_pending(state, count) || lv4_string_call_pending(state, count) ||
      lv4_string_held(state, count)) return 0;
  hako_lv4_state rank = lv4_string_rank(state[origin]);
  if (rank <= LV4_STRING_HOME_BASE || rank >= LV4_STRING_LIMIT) return 0;
  for (size_t i = 0; i < count; i++) {
    hako_lv4_state other = lv4_string_rank(state[i]);
    if (other > LV4_STRING_RANK_BASE && other < LV4_STRING_HOME_BASE) return 0;
  }
  state[origin] |= LV4_STRING_RETURN;
  return 1;
}
static hako_lv4_state lv4_string_domain_top(const hako_lv4_state* state, size_t count,
    hako_lv4_state base, hako_lv4_state limit) {
  hako_lv4_state top = base;
  for (size_t i = 0; i < count; i++) {
    hako_lv4_state rank = lv4_string_rank(state[i]);
    if (rank > top && rank < limit) top = rank;
  }
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
static int lv4_string_install(hako_lv4_state* state, size_t count, int origin, int replaces) {
  if (lv4_string_pending(state, count) || lv4_string_call_pending(state, count) || lv4_string_held(state, count) || replaces < -1 ||
      (replaces >= 0 && ((size_t)replaces >= count || replaces == origin ||
          lv4_string_rank(state[replaces]) <= LV4_STRING_HOME_BASE || lv4_string_rank(state[replaces]) >= LV4_STRING_LIMIT))) return 0;
  if (origin < 0 || (size_t)origin >= count || state[origin] <= LV4_STRING_RANK_BASE ||
      state[origin] >= LV4_STRING_HOME_BASE || state[origin] !=
      lv4_string_domain_top(state, count, LV4_STRING_RANK_BASE, LV4_STRING_HOME_BASE)) return 0;
  hako_lv4_state top = lv4_string_domain_top(state, count, LV4_STRING_HOME_BASE, LV4_STRING_LIMIT);
  if (top + 1 >= LV4_STRING_LIMIT) return 0;
  state[origin] = top + 1;
  if (replaces >= 0) { state[origin] |= LV4_STRING_NEW; state[replaces] |= LV4_STRING_OLD; }
  return 1;
}

/* End attempts consume their target before either Normal or Fault. A failed
 * displaced-old End keeps new live; a Fault new End restores ordinary unwind. */
static int lv4_string_end(hako_lv4_state* state, size_t count, int origin, int faulted) {
  if (origin < 0 || (size_t)origin >= count) return 0;
  int old = -1, newest = -1;
  for (size_t i = 0; i < count; i++) {
    if (state[i] & LV4_STRING_OLD) { if (old >= 0) return 0; old = (int)i; }
    if (state[i] & LV4_STRING_NEW) { if (newest >= 0) return 0; newest = (int)i; }
  }
  if ((old >= 0) != (newest >= 0)) return 0;
  hako_lv4_state temporary = lv4_string_domain_top(state, count, LV4_STRING_RANK_BASE, LV4_STRING_HOME_BASE);
  hako_lv4_state rank = lv4_string_rank(state[origin]);
  if (temporary > LV4_STRING_RANK_BASE) {
    if (rank != temporary) return 0;
  } else if (!faulted && old >= 0) {
    if (origin != old || lv4_string_rank(state[newest]) != lv4_string_top(state, count)) return 0;
    state[newest] &= ~LV4_STRING_NEW;
  } else {
    hako_lv4_state top = lv4_string_top(state, count);
    if (!faulted) {
      top = LV4_STRING_HOME_BASE;
      for (size_t i = 0; i < count; i++) {
        hako_lv4_state candidate = lv4_string_rank(state[i]);
        if (!(state[i] & LV4_STRING_RETURN) && candidate > top && candidate < LV4_STRING_LIMIT)
          top = candidate;
      }
      if (state[origin] & LV4_STRING_RETURN) return 0;
    }
    if (rank <= LV4_STRING_HOME_BASE || rank != top) return 0;
    if (origin == newest) state[old] &= ~LV4_STRING_OLD;
  }
  state[origin] = 0;
  /* The held result becomes usable only when its exact argument suffix is gone. */
  hako_lv4_state top = lv4_string_domain_top(state, count, LV4_STRING_RANK_BASE, LV4_STRING_HOME_BASE);
  for (size_t i = 0; i < count; i++)
    if ((state[i] & LV4_STRING_CALL_HELD) && lv4_string_rank(state[i]) == top)
      state[i] &= ~LV4_STRING_CALL_HELD;
  return 1;
}

/* The End attempt consumes on both outcomes. Only successful replacement
 * occupies the displaced Home position; Fault keeps acquisition-order unwind. */
static int lv4_string_end_paths(hako_lv4_state* failure, hako_lv4_state* normal,
    size_t count, int origin, int faulted) {
  int replacement = -1;
  hako_lv4_state displaced = origin >= 0 && (size_t)origin < count ? failure[origin] : 0;
  if (!faulted && (displaced & LV4_STRING_OLD))
    for (size_t i = 0; i < count; i++) if (failure[i] & LV4_STRING_NEW) replacement = (int)i;
  if (!lv4_string_end(failure, count, origin, faulted)) return 0;
  memcpy(normal, failure, count * sizeof(*normal));
  if (replacement >= 0) normal[replacement] =
      (normal[replacement] & ~lv4_string_rank(normal[replacement])) | lv4_string_rank(displaced);
  return 1;
}

static void* lv4_checked_calloc(size_t count, size_t size) {
  if (!size || count > SIZE_MAX / size) return NULL;
  return calloc(count, size);
}
