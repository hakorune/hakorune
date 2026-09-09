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

extern uint32_t real_prepare(void*, uint64_t, void*, const uint8_t*, size_t)
    __asm__("__real_nyash.map.key_prepare_utf8_v1");
uint32_t wrap_prepare(void*, uint64_t, void*, const uint8_t*, size_t)
    __asm__("__wrap_nyash.map.key_prepare_utf8_v1");
uint32_t wrap_prepare(void* frame, uint64_t site, void* key, const uint8_t* bytes, size_t len) {
  if (is_mode("prepare-fault")) return fault(frame, site); /* Empty key, no attempted move. */
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
extern uint32_t real_value(void*, uint32_t, uint64_t, void*, void*, uint32_t, int64_t, void*)
    __asm__("__real_nyash.map.checked_install_value_v1");
uint32_t wrap_value(void*, uint32_t, uint64_t, void*, void*, uint32_t, int64_t, void*)
    __asm__("__wrap_nyash.map.checked_install_value_v1");
uint32_t wrap_value(void* frame, uint32_t profile, uint64_t site, void* map,
    void* key, uint32_t kind, int64_t value, void* out) {
  /* The Value proof supplies true followed by integer30, each through Copies. */
  static unsigned installs;
  if ((!installs && (kind != NYRT_MAP_VALUE_BOOL || value != 1)) ||
      (installs && (kind != NYRT_MAP_VALUE_I64 || value != 30))) abort();
  installs++;
  /* Exercise the shared real attempt-Fault transition; no fake opaque mutation. */
  if (is_mode("install-fault"))
    return real_install(frame, profile, site, map, key, INT64_MAX, 900, out);
  return real_value(frame, profile, site, map, key, kind, value, out);
}
#endif

extern uint32_t real_outcome_end(void*, uint64_t, void*) __asm__("__real_nyash.map.outcome_end_v1");
uint32_t wrap_outcome_end(void*, uint64_t, void*) __asm__("__wrap_nyash.map.outcome_end_v1");
uint32_t wrap_outcome_end(void* frame, uint64_t site, void* out) {
  uint32_t result = real_outcome_end(frame, site, out);
  return !result && is_mode("outcome-fault") ? fault(frame, site) : result;
}

extern uint32_t real_end(void*, uint64_t, void*) __asm__("__real_nyash.map.checked_end_v1");
uint32_t wrap_end(void*, uint64_t, void*) __asm__("__wrap_nyash.map.checked_end_v1");
uint32_t wrap_end(void* frame, uint64_t site, void* map) {
  uint32_t result = real_end(frame, site, map);
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
  printf("%lld %u %u %u %u %u %u\n", (long long)result,
      map_init, map_dispose, key_init, key_dispose, outcome_init, outcome_dispose);
  return (int)result;
}
