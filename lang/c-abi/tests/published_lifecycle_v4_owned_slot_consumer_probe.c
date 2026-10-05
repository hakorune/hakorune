/* Inject each acquisition/store Fault into the unchanged source-issued graph. */
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../../../include/nyrt_fault_v1.h"
static const char* lane;
static unsigned fail_at, attempts, acquired, freed, objects, reports, disposed;
static unsigned object_attempts, array_attempts, store_attempts;
static int64_t arrays[5], handles[3];
static unsigned array_live[5], object_live[3];
static uint64_t injected_site;
static int fail(const char* kind, unsigned ordinal) {
  if (strcmp(lane,kind) || ordinal != fail_at) return 0;
  attempts++;
  return 1;
}
static void finish(void) {
  assert(freed == acquired && disposed == 1 && reports == (fail_at ? 1u : 0u));
  assert(attempts == (fail_at ? 1u : 0u));
  for (unsigned i=0;i<objects;i++) assert(!object_live[i]);
  printf("OWNED_SLOT_OK %s %u %u\n",lane,fail_at,acquired);
}
uint32_t real_init(void*) __asm__("__real_nyash.fault.frame_init_v1");
uint32_t wrap_init(void*) __asm__("__wrap_nyash.fault.frame_init_v1");
uint32_t wrap_init(void* f) {
  lane=getenv("OWNED_SLOT_FAULT"); if (!lane) lane="normal";
  const char* n=getenv("OWNED_SLOT_AT"); fail_at=n ? (unsigned)atoi(n) : 0;
  atexit(finish); return real_init(f);
}
static uint32_t inject(void* f,uint64_t site) {
  injected_site=site; return nyrt_fault_record_static_v1(f,101,site,29,0);
}
uint32_t real_array(void*,uint64_t,int64_t*) __asm__("__real_nyash.array.checked_new_v1");
uint32_t wrap_array(void*,uint64_t,int64_t*) __asm__("__wrap_nyash.array.checked_new_v1");
uint32_t wrap_array(void* f,uint64_t site,int64_t* out) {
  if(fail("array",++array_attempts)) return inject(f,site);
  uint32_t rc=real_array(f,site,out); assert(!rc && acquired<5);
  arrays[acquired]=*out; array_live[acquired++]=1; return rc;
}
void real_release(int64_t) __asm__("__real_nyrt_handle_release_h");
void wrap_release(int64_t) __asm__("__wrap_nyrt_handle_release_h");
void wrap_release(int64_t h) {
  unsigned i=acquired;
  while(i && (!array_live[i-1] || arrays[i-1]!=h)) --i;
  assert(i && i==acquired-freed); array_live[i-1]=0; ++freed; real_release(h);
}
uint32_t real_new(void*,uint32_t,uint64_t,int64_t,const uint32_t*,size_t,int64_t*)
 __asm__("__real_nyash.object.checked_new_v1");
uint32_t wrap_new(void*,uint32_t,uint64_t,int64_t,const uint32_t*,size_t,int64_t*)
 __asm__("__wrap_nyash.object.checked_new_v1");
uint32_t wrap_new(void* f,uint32_t p,uint64_t s,int64_t t,const uint32_t* l,size_t n,int64_t* out) {
  if(fail("object",++object_attempts)) return inject(f,s);
  uint32_t rc=real_new(f,p,s,t,l,n,out); assert(!rc && objects<3);
  handles[objects]=*out; object_live[objects++]=1; return rc;
}
uint32_t real_store(void*,uint32_t,uint64_t,int64_t,int64_t,size_t,int64_t)
 __asm__("__real_nyash.object.checked_field_set_v1");
uint32_t wrap_store(void*,uint32_t,uint64_t,int64_t,int64_t,size_t,int64_t)
 __asm__("__wrap_nyash.object.checked_field_set_v1");
uint32_t wrap_store(void* f,uint32_t p,uint64_t s,int64_t h,int64_t t,size_t n,int64_t v) {
  if(fail("store",++store_attempts)) return inject(f,s);
  return real_store(f,p,s,h,t,n,v);
}
static void drop_object(int64_t h) {
  unsigned i=0; while(i<objects && (!object_live[i] || handles[i]!=h)) ++i;
  assert(i<objects); object_live[i]=0;
}
uint32_t real_home(void*,uint32_t,uint64_t,int64_t,int64_t)
 __asm__("__real_nyash.object.home_release_plain_i64_v1");
uint32_t wrap_home(void*,uint32_t,uint64_t,int64_t,int64_t)
 __asm__("__wrap_nyash.object.home_release_plain_i64_v1");
uint32_t wrap_home(void* f,uint32_t p,uint64_t s,int64_t h,int64_t t) {
  drop_object(h); return real_home(f,p,s,h,t);
}
uint32_t real_reclaim(void*,uint32_t,uint64_t,int64_t,int64_t)
 __asm__("__real_nyash.object.reclaim_unpublished_v1");
uint32_t wrap_reclaim(void*,uint32_t,uint64_t,int64_t,int64_t)
 __asm__("__wrap_nyash.object.reclaim_unpublished_v1");
uint32_t wrap_reclaim(void* f,uint32_t p,uint64_t s,int64_t h,int64_t t) {
  drop_object(h); return real_reclaim(f,p,s,h,t);
}
int32_t real_report(const void*) __asm__("__real_nyash.fault.report_final_v1");
int32_t wrap_report(const void*) __asm__("__wrap_nyash.fault.report_final_v1");
int32_t wrap_report(const void* f) {
  const NyrtFaultFrameV1* fault=f; assert(fail_at && ++reports==1 && freed==acquired);
  assert(fault->primary.reason==101 && fault->primary.details[0]==29);
  assert(fault->primary.site==injected_site); return real_report(f);
}
uint32_t real_dispose(void*) __asm__("__real_nyash.fault.frame_dispose_v1");
uint32_t wrap_dispose(void*) __asm__("__wrap_nyash.fault.frame_dispose_v1");
uint32_t wrap_dispose(void* f) { ++disposed; return real_dispose(f); }
