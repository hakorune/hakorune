/* Link wrappers observe the generated body against the actual Rust runtime.
 * Failure injection changes runtime status only, never the issued input graph. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../../../include/nyrt_fault_v1.h"
#if defined(HAKO_TEST_OWNED_CALL_RESULT) || defined(HAKO_TEST_SCRIPT_ARGUMENTS)
#include <assert.h>
#endif
#ifdef HAKO_TEST_OWNED_CALL_RESULT
static unsigned new_attempts, acquired, freed;
static int64_t handles[256], types[256];
static unsigned char live[256];
static unsigned owned_live_index(int64_t handle, int64_t type) {
  unsigned i = acquired;
  while (i && (!live[i-1] || handles[i-1] != handle)) --i;
  assert(i && types[i-1] == type);
  return i - 1;
}
#endif

static unsigned init, stores, home, reclaim, report, dispose;
static const char* mode;
#ifdef HAKO_TEST_SCRIPT_ARGUMENTS
static unsigned argv_attempts, argv_acquired, argv_released;
static int64_t argv_handle;
#endif
static void counts(void) {
  printf("COUNTS %u %u %u %u %u %u\n", init, stores, home, reclaim, report, dispose);
#ifdef HAKO_TEST_SCRIPT_ARGUMENTS
  assert(argv_acquired == argv_released);
  printf("ARGV %u %u %u\n", argv_attempts, argv_acquired, argv_released);
#endif
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
/* Both storage writers observe one original store and its original site. */
static uint32_t probe_store_status(void* f,uint64_t s,int64_t v) {
  stores++;
  if (!strcmp(mode, "store-invalid")) return 2;
  if ((!strcmp(mode, "fault-first") && stores == 1) ||
      (!strcmp(mode, "fault-second") && stores == 2) || !strcmp(mode, "report-failure") ||
      (getenv("V4_PROBE_FAULT_AT") && stores == strtoul(getenv("V4_PROBE_FAULT_AT"), NULL, 10)))
    return nyrt_fault_record_static_v1(f, 101, s, v, 0);
  return 0;
}
uint32_t real_store(void*,uint32_t,uint64_t,int64_t,int64_t,size_t,int64_t)
    __asm__("__real_nyash.object.checked_field_set_v1");
uint32_t wrap_store(void*,uint32_t,uint64_t,int64_t,int64_t,size_t,int64_t)
    __asm__("__wrap_nyash.object.checked_field_set_v1");
uint32_t wrap_store(void* f,uint32_t p,uint64_t s,int64_t h,int64_t t,size_t slot,int64_t v) {
  uint32_t status = probe_store_status(f,s,v);
  return status ? status : real_store(f,p,s,h,t,slot,v);
}
uint32_t real_residence_store(void*,uint32_t,uint64_t,int64_t,int64_t,size_t,int64_t,int64_t,uint32_t)
    __asm__("__real_nyash.object.checked_residence_set_v1");
uint32_t wrap_residence_store(void*,uint32_t,uint64_t,int64_t,int64_t,size_t,int64_t,int64_t,uint32_t)
    __asm__("__wrap_nyash.object.checked_residence_set_v1");
uint32_t wrap_residence_store(void* f,uint32_t p,uint64_t s,int64_t h,int64_t t,size_t slot,
    int64_t child_type,int64_t v,uint32_t outcome) {
  uint32_t status = probe_store_status(f,s,v);
  return status ? status : real_residence_store(f,p,s,h,t,slot,child_type,v,outcome);
}
uint32_t real_home(void*,uint32_t,uint64_t,int64_t,int64_t)
    __asm__("__real_nyash.object.home_release_plain_i64_v1");
uint32_t wrap_home(void*,uint32_t,uint64_t,int64_t,int64_t)
    __asm__("__wrap_nyash.object.home_release_plain_i64_v1");
uint32_t wrap_home(void* f,uint32_t p,uint64_t s,int64_t h,int64_t t) {
  home++;
#ifdef HAKO_TEST_OWNED_CALL_RESULT
  unsigned i = owned_live_index(h, t);
  uint32_t rc = real_home(f,p,s,h,t);
  assert(!rc);
  live[i] = 0; ++freed;
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
  reclaim++;
#ifdef HAKO_TEST_OWNED_CALL_RESULT
  unsigned i = owned_live_index(h, t);
  uint32_t rc = real_reclaim(f,p,s,h,t);
  assert(!rc);
  live[i] = 0; ++freed;
  return rc;
#else
  return real_reclaim(f,p,s,h,t);
#endif
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
#ifdef HAKO_TEST_UNIT_CALL
/* Observe the original source Print without changing its published graph. */
static unsigned unit_print_attempts;
uint32_t real_unit_print(void*, uint64_t, const NyrtStringOperandV1*)
    __asm__("__real_nyash.string.checked_print_v1");
uint32_t wrap_unit_print(void*, uint64_t, const NyrtStringOperandV1*)
    __asm__("__wrap_nyash.string.checked_print_v1");
uint32_t wrap_unit_print(void* frame, uint64_t site, const NyrtStringOperandV1* value) {
  ++unit_print_attempts;
  printf("UNIT_PRINT %u\n", unit_print_attempts);
  if (mode && !strcmp(mode, "unit-print-fault"))
    return nyrt_fault_record_static_v1(frame, 103, site, 0, 0);
  return real_unit_print(frame, site, value);
}
#endif

#ifdef HAKO_TEST_SCRIPT_ARGUMENTS
uint32_t real_argv(void*,uint64_t,const nyrt_script_argument_utf8_v1*,size_t,int64_t*)
    __asm__("__real_nyash.array.checked_script_arguments_new_v1");
uint32_t wrap_argv(void*,uint64_t,const nyrt_script_argument_utf8_v1*,size_t,int64_t*)
    __asm__("__wrap_nyash.array.checked_script_arguments_new_v1");
uint32_t wrap_argv(void* frame,uint64_t site,const nyrt_script_argument_utf8_v1* values,size_t n,int64_t* out) {
  ++argv_attempts;
  unsigned expected = (unsigned)strtoul(getenv("V4_PROBE_ARGV_COUNT"), NULL, 10);
  assert(n == expected);
  if (n) {
    static const unsigned char alpha[] = "alpha", cat[] = {0xe7,0x8c,0xab}, nul[] = {'x',0,'y'};
    const unsigned char* bytes[] = {alpha,cat,nul,(const unsigned char*)""};
    const size_t lengths[] = {5,3,3,0};
    assert(n == 4);
    for (size_t i = 0; i < n; ++i) {
      assert(values[i].length == lengths[i]);
      assert(!memcmp(values[i].data, bytes[i], lengths[i]));
    }
  }
  if (!strcmp(mode,"argv-fault")) return nyrt_fault_record_static_v1(frame, 102, site, 0, 0);
  uint32_t rc = real_argv(frame,site,values,n,out);
  assert(!rc && *out > 0); argv_handle = *out; ++argv_acquired;
  return rc;
}
void real_argv_release(int64_t) __asm__("__real_nyrt_handle_release_h");
void wrap_argv_release(int64_t) __asm__("__wrap_nyrt_handle_release_h");
void wrap_argv_release(int64_t value) {
  if (value == argv_handle) {
    assert(argv_acquired == 1 && argv_released == 0);
    if (getenv("V4_PROBE_ARGV_BODY") && strcmp(mode,"argv-fault")) assert(home == 1);
    ++argv_released;
  }
  real_argv_release(value);
}
#endif
