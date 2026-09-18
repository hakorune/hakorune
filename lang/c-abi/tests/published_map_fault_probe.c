/* Generated-code fault/disposal observations against the actual runtime ABI. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/resource.h>
#include "../../../include/nyrt_fault_v1.h"

static const char* mode;
static unsigned map_init, map_dispose, key_init, key_dispose, outcome_init, outcome_dispose;
static int is_mode(const char* expected) { return mode && !strcmp(mode, expected); }
static uint32_t fault(void* frame, uint64_t site) {
  return nyrt_fault_record_static_v1(frame, 100, site, 0, 0);
}

#define BOOKKEEPING(label, symbol) \
  extern uint32_t real_##label(void*) __asm__("__real_nyash.map." symbol "_v1"); \
  uint32_t wrap_##label(void*) __asm__("__wrap_nyash.map." symbol "_v1"); \
  uint32_t wrap_##label(void* ptr) { label++; return real_##label(ptr); }
BOOKKEEPING(map_init, "storage_init")
BOOKKEEPING(map_dispose, "storage_dispose")
BOOKKEEPING(key_init, "key_init")
BOOKKEEPING(key_dispose, "key_dispose")
BOOKKEEPING(outcome_init, "outcome_init")
BOOKKEEPING(outcome_dispose, "outcome_dispose")

extern uint32_t real_new(void*, uint32_t, uint64_t, void*) __asm__("__real_nyash.map.checked_new_v1");
uint32_t wrap_new(void*, uint32_t, uint64_t, void*) __asm__("__wrap_nyash.map.checked_new_v1");
uint32_t wrap_new(void* frame, uint32_t profile, uint64_t site, void* map) {
  if (is_mode("unknown")) return 99;
  if (is_mode("invalid")) return 2;
  if (is_mode("new-fault")) return fault(frame, site);
  return real_new(frame, profile, site, map);
}

#ifdef HAKO_MAP_CONSUMER_PROBE
static void consumer_record_key(void*, const uint8_t*, size_t);
static void consumer_record_end(void*, uint32_t);
#endif
extern uint32_t real_prepare(void*, uint64_t, void*, const uint8_t*, size_t)
    __asm__("__real_nyash.map.key_prepare_utf8_v1");
uint32_t wrap_prepare(void*, uint64_t, void*, const uint8_t*, size_t)
    __asm__("__wrap_nyash.map.key_prepare_utf8_v1");
uint32_t wrap_prepare(void* frame, uint64_t site, void* key, const uint8_t* bytes, size_t len) {
  if (is_mode("prepare-fault")) return fault(frame, site); /* Empty key, no attempted move. */
#ifdef HAKO_MAP_CONSUMER_PROBE
  consumer_record_key(key, bytes, len);
#endif
  return real_prepare(frame, site, key, bytes, len);
}

extern uint32_t real_install(void*, uint32_t, uint64_t, void*, void*, int64_t, int64_t, void*)
    __asm__("__real_nyash.map.checked_install_indexed_v1");
uint32_t wrap_install(void*, uint32_t, uint64_t, void*, void*, int64_t, int64_t, void*)
    __asm__("__wrap_nyash.map.checked_install_indexed_v1");
uint32_t wrap_install(void* frame, uint32_t profile, uint64_t site, void* map,
    void* key, int64_t value, int64_t type, void* out) {
  /* Real attempt consumes key but rejects identity before candidate transfer. */
  if (is_mode("install-fault")) value = INT64_MAX;
  return real_install(frame, profile, site, map, key, value, type, out);
}

#ifdef HAKO_MAP_VALUE_PROBE
static unsigned value_installs;
extern uint32_t real_value(void*, uint32_t, uint64_t, void*, void*, uint32_t, int64_t, void*)
    __asm__("__real_nyash.map.checked_install_value_v1");
uint32_t wrap_value(void*, uint32_t, uint64_t, void*, void*, uint32_t, int64_t, void*)
    __asm__("__wrap_nyash.map.checked_install_value_v1");
uint32_t wrap_value(void* frame, uint32_t profile, uint64_t site, void* map,
    void* key, uint32_t kind, int64_t value, void* out) {
  /* The Value proof supplies true followed by integer30, each through Copies. */

  if ((!value_installs && (kind != NYRT_MAP_VALUE_BOOL || value != 1)) ||
      (value_installs && (kind != NYRT_MAP_VALUE_I64 || value != 30))) abort();
  value_installs++;
  /* Exercise the shared real attempt-Fault transition; no fake opaque mutation. */
  if (is_mode("install-fault") || is_mode("value-install-fault"))
    return real_install(frame, profile, site, map, key, INT64_MAX, 900, out);
  return real_value(frame, profile, site, map, key, kind, value, out);
}
#endif

#if defined(HAKO_MAP_THREE_OWNER_PROBE) || defined(HAKO_MAP_FOUR_OWNER_PROBE) || defined(HAKO_MAP_FIVE_OWNER_PROBE) || defined(HAKO_MAP_CONSUMER_PROBE)
static unsigned ordinal_mode(const char* prefix) {
  size_t length = strlen(prefix);
  char* end = NULL;
  unsigned long value;
  if (!mode || strncmp(mode, prefix, length) != 0) return 0;
  value = strtoul(mode + length, &end, 10);
  return *end == '\0' && value <= 8 ? (unsigned)value : 0;
}
#endif

#ifdef HAKO_MAP_THREE_OWNER_PROBE
static unsigned three_value_installs;
static int64_t three_value_sequence[8];
static unsigned three_outcome_fault_fired;

extern uint32_t real_three_value(void*, uint32_t, uint64_t, void*, void*, uint32_t, int64_t, void*)
    __asm__("__real_nyash.map.checked_install_value_v1");
uint32_t wrap_three_value(void*, uint32_t, uint64_t, void*, void*, uint32_t, int64_t, void*)
    __asm__("__wrap_nyash.map.checked_install_value_v1");
uint32_t wrap_three_value(void* frame, uint32_t profile, uint64_t site, void* map,
    void* key, uint32_t kind, int64_t value, void* out) {
  unsigned ordinal = ++three_value_installs;
  if (ordinal > 8 || kind != NYRT_MAP_VALUE_I64) abort();
  three_value_sequence[ordinal - 1] = value;
  if (ordinal_mode("value-install-fault-") == ordinal)
    return real_install(frame, profile, site, map, key, INT64_MAX, 900, out);
  return real_three_value(frame, profile, site, map, key, kind, value, out);
}
#endif

#ifdef HAKO_MAP_FOUR_OWNER_PROBE
static unsigned four_value_installs;
static int64_t four_value_sequence[8];
static unsigned four_outcome_fault_fired;

extern uint32_t real_four_value(void*, uint32_t, uint64_t, void*, void*, uint32_t, int64_t, void*)
    __asm__("__real_nyash.map.checked_install_value_v1");
uint32_t wrap_four_value(void*, uint32_t, uint64_t, void*, void*, uint32_t, int64_t, void*)
    __asm__("__wrap_nyash.map.checked_install_value_v1");
uint32_t wrap_four_value(void* frame, uint32_t profile, uint64_t site, void* map,
    void* key, uint32_t kind, int64_t value, void* out) {
  unsigned ordinal = ++four_value_installs;
  if (ordinal > 8 || kind != NYRT_MAP_VALUE_I64) abort();
  four_value_sequence[ordinal - 1] = value;
  if (ordinal_mode("value-install-fault-") == ordinal)
    return real_install(frame, profile, site, map, key, INT64_MAX, 900, out);
  return real_four_value(frame, profile, site, map, key, kind, value, out);
}
#endif

#ifdef HAKO_MAP_FIVE_OWNER_PROBE
static unsigned five_value_installs;
static int64_t five_value_sequence[8];
static unsigned five_outcome_fault_fired;

extern uint32_t real_five_value(void*, uint32_t, uint64_t, void*, void*, uint32_t, int64_t, void*)
    __asm__("__real_nyash.map.checked_install_value_v1");
uint32_t wrap_five_value(void*, uint32_t, uint64_t, void*, void*, uint32_t, int64_t, void*)
    __asm__("__wrap_nyash.map.checked_install_value_v1");
uint32_t wrap_five_value(void* frame, uint32_t profile, uint64_t site, void* map,
    void* key, uint32_t kind, int64_t value, void* out) {
  unsigned ordinal = ++five_value_installs;
  if (ordinal > 8 || kind != NYRT_MAP_VALUE_I64) abort();
  five_value_sequence[ordinal - 1] = value;
  if (ordinal_mode("value-install-fault-") == ordinal)
    return real_install(frame, profile, site, map, key, INT64_MAX, 900, out);
  return real_five_value(frame, profile, site, map, key, kind, value, out);
}
#endif

#ifdef HAKO_MAP_CONSUMER_PROBE
/* C8 map-consumer lane. The v4 map storage is opaque to read ABI, so the
 * returned map's contents are evidenced transitively: key bytes are captured
 * at prepare, installs are recorded against their map storage, storage_move
 * links the callee slot to the caller slot, and checked_end resolves which
 * installed entries each ended storage carried. */
#define CONSUMER_MAX 8
static unsigned consumer_attempts;
static unsigned consumer_installs;
static void* consumer_install_map[CONSUMER_MAX];
static char consumer_install_key[CONSUMER_MAX][32];
static int64_t consumer_install_value[CONSUMER_MAX];
static void* consumer_key_ptr[CONSUMER_MAX];
static char consumer_key_text[CONSUMER_MAX][32];
static unsigned consumer_key_prepares;
static unsigned consumer_moves;
static void* consumer_move_dst[CONSUMER_MAX];
static void* consumer_move_src[CONSUMER_MAX];
static unsigned consumer_ends;
static int consumer_end_moved[CONSUMER_MAX];
static int64_t consumer_end_value[CONSUMER_MAX];
static char consumer_end_key[CONSUMER_MAX][32];

static const char* consumer_key_for(void* key) {
  for (unsigned i = 0; i < consumer_key_prepares && i < CONSUMER_MAX; i++)
    if (consumer_key_ptr[i] == key) return consumer_key_text[i];
  return "?";
}
static void consumer_record_key(void* key, const uint8_t* bytes, size_t len) {
  unsigned slot = consumer_key_prepares;
  if (slot >= CONSUMER_MAX) abort();
  consumer_key_ptr[slot] = key;
  size_t copy = len < 31 ? len : 31;
  memcpy(consumer_key_text[slot], bytes, copy);
  consumer_key_text[slot][copy] = '\0';
  consumer_key_prepares++;
}
/* Follow the move chain from an ended caller slot back to the storage that
 * received the installs, then report the installed entry it carried. */
static void consumer_record_end(void* map, uint32_t result) {
  unsigned slot = consumer_ends;
  void* origin = map;
  int moved = 0;
  if (slot >= CONSUMER_MAX) abort();
  for (unsigned hop = 0; hop < CONSUMER_MAX; hop++) {
    int advanced = 0;
    for (unsigned i = 0; i < consumer_moves && i < CONSUMER_MAX; i++) {
      if (consumer_move_dst[i] == origin) {
        origin = consumer_move_src[i];
        moved = 1;
        advanced = 1;
        break;
      }
    }
    if (!advanced) break;
  }
  consumer_end_moved[slot] = moved;
  consumer_end_value[slot] = -1;
  consumer_end_key[slot][0] = '\0';
  if (result == 0) {
    for (unsigned i = consumer_installs; i > 0; i--) {
      if (consumer_install_map[i - 1] == origin) {
        snprintf(consumer_end_key[slot], 32, "%s", consumer_install_key[i - 1]);
        consumer_end_value[slot] = consumer_install_value[i - 1];
        break;
      }
    }
  }
  consumer_ends++;
}

extern uint32_t real_consumer_value(void*, uint32_t, uint64_t, void*, void*, uint32_t, int64_t, void*)
    __asm__("__real_nyash.map.checked_install_value_v1");
uint32_t wrap_consumer_value(void*, uint32_t, uint64_t, void*, void*, uint32_t, int64_t, void*)
    __asm__("__wrap_nyash.map.checked_install_value_v1");
uint32_t wrap_consumer_value(void* frame, uint32_t profile, uint64_t site, void* map,
    void* key, uint32_t kind, int64_t value, void* out) {
  unsigned ordinal = ++consumer_attempts;
  uint32_t result;
  if (ordinal > CONSUMER_MAX || kind != NYRT_MAP_VALUE_I64) abort();
  if (ordinal_mode("value-install-fault-") == ordinal)
    return real_install(frame, profile, site, map, key, INT64_MAX, 900, out);
  result = real_consumer_value(frame, profile, site, map, key, kind, value, out);
  /* Only a completed install carried the entry; a faulted attempt consumed
   * the key into the outcome but never reached the map. */
  if (result == 0) {
    unsigned slot = consumer_installs;
    if (slot >= CONSUMER_MAX) abort();
    consumer_install_map[slot] = map;
    consumer_install_value[slot] = value;
    snprintf(consumer_install_key[slot], 32, "%s", consumer_key_for(key));
    consumer_installs = slot + 1;
  }
  return result;
}

static unsigned consumer_outcome_fault_fired;
#endif

#ifdef HAKO_MAP_READ_PROBE
/* Readable-map lane: one checked scalar get must leave the borrowed/owned
 * storage live — no key/outcome bookkeeping, no disposal — and the owning
 * `checked_end` must still run afterwards on both Normal and Fault paths. */
static unsigned read_seq, read_get_seq, read_end_seq, read_ends;
static char read_key[32];
static size_t read_key_len;
static int64_t read_out;
static uint32_t read_status;

extern uint32_t real_read_get(void*, uint64_t, void*, const uint8_t*, size_t, int64_t*)
    __asm__("__real_nyash.map.checked_get_i64_v1");
uint32_t wrap_read_get(void*, uint64_t, void*, const uint8_t*, size_t, int64_t*)
    __asm__("__wrap_nyash.map.checked_get_i64_v1");
uint32_t wrap_read_get(void* frame, uint64_t site, void* map,
    const uint8_t* bytes, size_t len, int64_t* out) {
  size_t copy;
  uint32_t status;
  read_get_seq = ++read_seq;
  status = real_read_get(frame, site, map, bytes, len, out);
  copy = len < 31 ? len : 31;
  memcpy(read_key, bytes, copy);
  read_key[copy] = '\0';
  read_key_len = len;
  read_out = *out;
  read_status = status;
  return status;
}
#endif

#if defined(HAKO_MAP_CALL_PROBE) || defined(HAKO_MAP_CONSUMER_PROBE)
/* The map-result Call lane's return handoff: the callee's storage_move into
 * caller-owned out storage is counted separately from storage_dispose. */
static unsigned moves;
extern uint32_t real_move(void*, void*) __asm__("__real_nyash.map.storage_move_v1");
uint32_t wrap_move(void*, void*) __asm__("__wrap_nyash.map.storage_move_v1");
uint32_t wrap_move(void* dst, void* src) {
  moves++;
#ifdef HAKO_MAP_CONSUMER_PROBE
  if (consumer_moves >= CONSUMER_MAX) abort();
  consumer_move_dst[consumer_moves] = dst;
  consumer_move_src[consumer_moves] = src;
  consumer_moves++;
#endif
  return real_move(dst, src);
}
#endif

extern uint32_t real_outcome_end(void*, uint64_t, void*) __asm__("__real_nyash.map.outcome_end_v1");
uint32_t wrap_outcome_end(void*, uint64_t, void*) __asm__("__wrap_nyash.map.outcome_end_v1");
uint32_t wrap_outcome_end(void* frame, uint64_t site, void* out) {
  uint32_t result = real_outcome_end(frame, site, out);
#ifdef HAKO_MAP_VALUE_PROBE
  if (!result && is_mode("value-outcome-fault") && value_installs == 1) return fault(frame, site);
#endif
#ifdef HAKO_MAP_THREE_OWNER_PROBE
  if (!result && !three_outcome_fault_fired &&
      ordinal_mode("value-outcome-fault-") == three_value_installs) {
    three_outcome_fault_fired = 1;
    return fault(frame, site);
  }
#endif
#ifdef HAKO_MAP_FOUR_OWNER_PROBE
  if (!result && !four_outcome_fault_fired &&
      ordinal_mode("value-outcome-fault-") == four_value_installs) {
    four_outcome_fault_fired = 1;
    return fault(frame, site);
  }
#endif
#ifdef HAKO_MAP_FIVE_OWNER_PROBE
  if (!result && !five_outcome_fault_fired &&
      ordinal_mode("value-outcome-fault-") == five_value_installs) {
    five_outcome_fault_fired = 1;
    return fault(frame, site);
  }
#endif
#ifdef HAKO_MAP_CONSUMER_PROBE
  if (!result && !consumer_outcome_fault_fired &&
      ordinal_mode("value-outcome-fault-") == consumer_attempts) {
    consumer_outcome_fault_fired = 1;
    return fault(frame, site);
  }
#endif
  return !result && is_mode("outcome-fault") ? fault(frame, site) : result;
}

extern uint32_t real_end(void*, uint64_t, void*) __asm__("__real_nyash.map.checked_end_v1");
uint32_t wrap_end(void*, uint64_t, void*) __asm__("__wrap_nyash.map.checked_end_v1");
uint32_t wrap_end(void* frame, uint64_t site, void* map) {
  uint32_t result;
#ifdef HAKO_MAP_READ_PROBE
  read_end_seq = ++read_seq;
#endif
  result = real_end(frame, site, map);
#ifdef HAKO_MAP_READ_PROBE
  read_ends++;
#endif
#ifdef HAKO_MAP_CONSUMER_PROBE
  consumer_record_end(map, result);
#endif
  return !result && is_mode("end-fault") ? fault(frame, site) : result;
}

#ifdef HAKO_MAP_SOURCE_PROBE
static unsigned outer_ends, reports;
extern uint32_t real_home(void*, uint32_t, uint64_t, int64_t, int64_t)
    __asm__("__real_nyash.object.home_release_plain_i64_v1");
uint32_t wrap_home(void*, uint32_t, uint64_t, int64_t, int64_t)
    __asm__("__wrap_nyash.object.home_release_plain_i64_v1");
uint32_t wrap_home(void* frame, uint32_t profile, uint64_t site, int64_t value, int64_t type) {
  outer_ends++;
  return real_home(frame, profile, site, value, type);
}
extern int32_t real_report(const void*) __asm__("__real_nyash.fault.report_final_v1");
int32_t wrap_report(const void*) __asm__("__wrap_nyash.fault.report_final_v1");
int32_t wrap_report(const void* frame) {
  reports++;
  printf("REPORT %u OUTER %u MAP %u KEY %u OUTCOME %u\n",
      ((const NyrtFaultFrameV1*)frame)->primary.reason, outer_ends,
      map_dispose, key_dispose, outcome_dispose);
  return real_report(frame);
}
extern uint32_t real_frame_dispose(void*) __asm__("__real_nyash.fault.frame_dispose_v1");
uint32_t wrap_frame_dispose(void*) __asm__("__wrap_nyash.fault.frame_dispose_v1");
uint32_t wrap_frame_dispose(void* frame) {
  printf("FRAME OUTER %u REPORTS %u MAP %u KEY %u OUTCOME %u\n",
      outer_ends, reports, map_dispose, key_dispose, outcome_dispose);
#if defined(HAKO_MAP_THREE_OWNER_PROBE) || defined(HAKO_MAP_FOUR_OWNER_PROBE) || defined(HAKO_MAP_FIVE_OWNER_PROBE)
  printf("VALUES");
#ifdef HAKO_MAP_THREE_OWNER_PROBE
  for (unsigned i = 0; i < three_value_installs; i++) printf(" %lld", (long long)three_value_sequence[i]);
#elif defined(HAKO_MAP_FOUR_OWNER_PROBE)
  for (unsigned i = 0; i < four_value_installs; i++) printf(" %lld", (long long)four_value_sequence[i]);
#else
  for (unsigned i = 0; i < five_value_installs; i++) printf(" %lld", (long long)five_value_sequence[i]);
#endif
  printf("\n");
#endif
#ifdef HAKO_MAP_CONSUMER_PROBE
  for (unsigned i = 0; i < consumer_ends; i++) {
    if (consumer_end_value[i] >= 0)
      printf("END %s=%lld %s\n", consumer_end_key[i],
          (long long)consumer_end_value[i],
          consumer_end_moved[i] ? "moved" : "local");
    else
      printf("END empty %s\n", consumer_end_moved[i] ? "moved" : "local");
  }
#endif
  return real_frame_dispose(frame);
}
#endif

extern int64_t ny_main(void);
#ifdef HAKO_MAP_SOURCE_PROBE
int probe_main(int argc, char** argv) __asm__("__wrap_main");
int probe_main(int argc, char** argv) {
#else
int main(int argc, char** argv) {
#endif
  struct rlimit limit = {0, 0}; setrlimit(RLIMIT_CORE, &limit);
  mode = argc == 2 ? argv[1] : "normal";
  int64_t result = ny_main();
  printf("%lld %u %u %u %u %u %u", (long long)result,
      map_init, map_dispose, key_init, key_dispose, outcome_init, outcome_dispose);
#if defined(HAKO_MAP_CALL_PROBE) || defined(HAKO_MAP_CONSUMER_PROBE)
  printf(" %u", moves);
#endif
#ifdef HAKO_MAP_READ_PROBE
  printf(" READ %s %zu %lld %u %u %u %u", read_key, read_key_len,
      (long long)read_out, read_status, read_get_seq, read_end_seq, read_ends);
#endif
  printf("\n");
  return (int)result;
}
