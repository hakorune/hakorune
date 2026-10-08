/* Link wrappers observe the generated body against the actual Rust runtime.
 * Failure injection changes runtime status only, never the issued input graph. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../../../include/nyrt_fault_v1.h"
#ifdef HAKO_TEST_OWNED_CALL_RESULT
#include <assert.h>
static unsigned new_attempts, acquired, freed;
static int64_t handles[256], types[256];
static unsigned char live[256];
#endif

static unsigned init, stores, home, reclaim, report, dispose;
static const char* mode;
static void counts(void) {
  printf("COUNTS %u %u %u %u %u %u\n", init, stores, home, reclaim, report, dispose);
#ifdef HAKO_TEST_OWNED_CALL_RESULT
  assert(acquired == freed);
  printf("OWNED %u %u %u\n", new_attempts, acquired, freed);
#endif
}
#ifdef HAKO_TEST_OWNED_CALL_RESULT
uint32_t real_new(void*,uint32_t,uint64_t,int64_t,const uint32_t*,size_t,int64_t*)
    __asm__("__real_nyash.object.checked_new_v1");
uint32_t wrap_new(void*,uint32_t,uint64_t,int64_t,const uint32_t*,size_t,int64_t*)
    __asm__("__wrap_nyash.object.checked_new_v1");
uint32_t wrap_new(void* f,uint32_t p,uint64_t s,int64_t t,const uint32_t* l,size_t n,int64_t* out) {
  ++new_attempts;
  const char* fault = getenv("V4_PROBE_NEW_FAULT_AT");
  if (fault && new_attempts == strtoul(fault, NULL, 10))
    return nyrt_fault_record_static_v1(f, 101, s, t, 0);
  uint32_t rc = real_new(f,p,s,t,l,n,out);
  assert(!rc && acquired < 256);
  handles[acquired] = *out; types[acquired] = t; live[acquired++] = 1;
  return rc;
}
#endif
uint32_t real_init(void*) __asm__("__real_nyash.fault.frame_init_v1");
uint32_t wrap_init(void*) __asm__("__wrap_nyash.fault.frame_init_v1");
uint32_t wrap_init(void* frame) {
  mode = getenv("V4_PROBE_MODE");
  if (!mode) mode = "normal";
  if (init++ == 0) atexit(counts);
  if (!strcmp(mode, "init-invalid")) return 2;
  return real_init(frame);
}
uint32_t real_store(void*,uint32_t,uint64_t,int64_t,int64_t,size_t,int64_t)
    __asm__("__real_nyash.object.checked_field_set_v1");
uint32_t wrap_store(void*,uint32_t,uint64_t,int64_t,int64_t,size_t,int64_t)
    __asm__("__wrap_nyash.object.checked_field_set_v1");
uint32_t wrap_store(void* f,uint32_t p,uint64_t s,int64_t h,int64_t t,size_t slot,int64_t v) {
  stores++;
  if (!strcmp(mode, "store-invalid")) return 2;
  if ((!strcmp(mode, "fault-first") && stores == 1) ||
      (!strcmp(mode, "fault-second") && stores == 2) || !strcmp(mode, "report-failure") ||
      (getenv("V4_PROBE_FAULT_AT") && stores == strtoul(getenv("V4_PROBE_FAULT_AT"), NULL, 10)))
    return nyrt_fault_record_static_v1(f, 101, s, v, 0);
  return real_store(f,p,s,h,t,slot,v);
}
uint32_t real_home(void*,uint32_t,uint64_t,int64_t,int64_t)
    __asm__("__real_nyash.object.home_release_plain_i64_v1");
uint32_t wrap_home(void*,uint32_t,uint64_t,int64_t,int64_t)
    __asm__("__wrap_nyash.object.home_release_plain_i64_v1");
uint32_t wrap_home(void* f,uint32_t p,uint64_t s,int64_t h,int64_t t) {
  home++;
#ifdef HAKO_TEST_OWNED_CALL_RESULT
  unsigned i = acquired;
  while (i && (!live[i-1] || handles[i-1] != h)) --i;
  assert(i && types[i-1] == t);
  uint32_t rc = real_home(f,p,s,h,t);
  assert(!rc);
  live[i-1] = 0; ++freed;
  /* This cleanup operation consumes the lease on both status edges. */
  const char* fault = getenv("V4_PROBE_HOME_FAULT_AT");
  if (fault && home == strtoul(fault, NULL, 10))
    return nyrt_fault_record_static_v1(f, 102, s, h, 0);
  return rc;
#else
  return real_home(f,p,s,h,t);
#endif
}
uint32_t real_reclaim(void*,uint32_t,uint64_t,int64_t,int64_t)
    __asm__("__real_nyash.object.reclaim_unpublished_v1");
uint32_t wrap_reclaim(void*,uint32_t,uint64_t,int64_t,int64_t)
    __asm__("__wrap_nyash.object.reclaim_unpublished_v1");
uint32_t wrap_reclaim(void* f,uint32_t p,uint64_t s,int64_t h,int64_t t) {
  reclaim++; return real_reclaim(f,p,s,h,t);
}
int32_t real_report(const void*) __asm__("__real_nyash.fault.report_final_v1");
int32_t wrap_report(const void*) __asm__("__wrap_nyash.fault.report_final_v1");
int32_t wrap_report(const void* f) {
  const NyrtFaultFrameV1* frame = f;
  report++;
  printf("FAULT %u %llu %lld %lld HOME %u RECLAIM %u\n", frame->primary.reason,
      (unsigned long long)frame->primary.site, (long long)frame->primary.details[0],
      (long long)frame->primary.details[1], home, reclaim);
  return !strcmp(mode,"report-failure") ? -2 : real_report(f);
}
uint32_t real_dispose(void*) __asm__("__real_nyash.fault.frame_dispose_v1");
uint32_t wrap_dispose(void*) __asm__("__wrap_nyash.fault.frame_dispose_v1");
uint32_t wrap_dispose(void* f) {
  printf("DISPOSE HOME %u RECLAIM %u REPORT %u\n", home, reclaim, report);
  dispose++; return real_dispose(f);
}
