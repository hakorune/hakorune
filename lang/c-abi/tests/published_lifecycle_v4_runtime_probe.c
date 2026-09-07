/* Link wrappers observe the generated body against the actual Rust runtime.
 * Failure injection changes runtime status only, never the issued input graph. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../../../include/nyrt_fault_v1.h"

static unsigned init, stores, home, reclaim, report, dispose;
static const char* mode;
static void counts(void) {
  printf("COUNTS %u %u %u %u %u %u\n", init, stores, home, reclaim, report, dispose);
}
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
      (!strcmp(mode, "fault-second") && stores == 2) || !strcmp(mode, "report-failure"))
    return nyrt_fault_record_static_v1(f, 101, s, v, 0);
  return real_store(f,p,s,h,t,slot,v);
}
uint32_t real_home(void*,uint32_t,uint64_t,int64_t,int64_t)
    __asm__("__real_nyash.object.home_release_plain_i64_v1");
uint32_t wrap_home(void*,uint32_t,uint64_t,int64_t,int64_t)
    __asm__("__wrap_nyash.object.home_release_plain_i64_v1");
uint32_t wrap_home(void* f,uint32_t p,uint64_t s,int64_t h,int64_t t) {
  home++; return real_home(f,p,s,h,t);
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
  dispose++; return real_dispose(f);
}
