/* Real C ABI and runtime archive proof; no source/MIR Map acceptance claim. */
#include <assert.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "../../../include/nyrt_fault_v1.h"
extern const unsigned char nyash_runtime_abi_descriptor_v2[236];
static uint32_t word(size_t at) {
  const unsigned char *p = nyash_runtime_abi_descriptor_v2 + at;
  return (uint32_t)p[0] | (uint32_t)p[1]<<8 | (uint32_t)p[2]<<16 | (uint32_t)p[3]<<24;
}
static void *region(size_t at) {
  size_t size=word(at), align=word(at+4); assert(size && align && size%align==0);
  void *ptr=aligned_alloc(align,size); assert(ptr); return ptr;
}
int main(void) {
  assert(!memcmp(nyash_runtime_abi_descriptor_v2,"NYRTABI2",8));
  void *frame=region(56), *map=region(200), *key=region(212), *out=region(224);
  assert(nyrt_fault_frame_init_v1(frame)==0);
  assert(nyrt_map_storage_init_v1(map)==0);
  assert(nyrt_map_checked_new_v1(frame,1,1,map)==0);
  assert(nyrt_map_key_init_v1(key)==0);
  const uint8_t text[]={'a',0,'b'};
  assert(nyrt_map_key_prepare_utf8_v1(frame,2,key,text,sizeof(text))==0);
  const uint32_t layout[]={1}; int64_t child=0, value=0;
  assert(nyrt_object_checked_new_v1(frame,1,3,931,layout,1,&child)==0);
  assert(nyrt_object_checked_field_set_v1(frame,1,4,child,931,0,42)==0);
  assert(nyrt_map_outcome_init_v1(out)==0);
  assert(nyrt_map_checked_install_indexed_v1(frame,1,5,map,key,child,931,out)==0);
  assert(nyrt_map_outcome_dispose_v1(out)==2);
  assert(nyrt_map_storage_dispose_v1(map)==2);
  assert(nyrt_object_checked_field_get_i64_v1(1,child,931,0,&value)==0 && value==42);
  assert(nyrt_map_outcome_end_v1(frame,6,out)==0);
  assert(nyrt_map_outcome_dispose_v1(out)==0);
  assert(nyrt_map_key_dispose_v1(key)==0);
  assert(nyrt_map_checked_end_v1(frame,7,map)==0);
  value=91;
  assert(nyrt_object_checked_field_get_i64_v1(1,child,931,0,&value)==2 && value==91);
  assert(nyrt_map_storage_dispose_v1(map)==0);
  assert(nyrt_fault_frame_dispose_v1(frame)==0);
  free(frame); free(map); free(key); free(out);
  puts("checked Map C ABI: native key, indexed install, end and disposal passed");
  return 0;
}
