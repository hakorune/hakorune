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
/* Reuse this native archive boundary for the same opaque-placement protocol.
 * These callbacks are explicit published End choices, not runtime tag dispatch. */
static void *field_outcome;
static int64_t field_parent;
static uint32_t field_profile = 1;
static unsigned field_end_count;
static uint32_t field_object_end(void *frame, uint64_t site, const NyrtDynamicFieldValueV1 *value) {
  NyrtDynamicFieldValueV1 seen = {0};
  assert(nyrt_object_field_outcome_end_v1(frame,site,field_outcome)==2);
  assert(nyrt_object_checked_dynamic_get_v1(frame,field_profile,site,field_parent,940,0,
      NYRT_FIELD_I64_V1,&seen)==0 && seen.value==42);
  assert(nyrt_object_home_release_plain_i64_v1(frame,field_profile,site,value->value,941)==0);
  field_end_count++;
  return nyrt_fault_record_static_v1(frame,101,site,0,0);
}
static uint32_t field_text_end(void *frame, uint64_t site, const NyrtDynamicFieldValueV1 *value) {
  return nyrt_string_checked_end_v1(frame,site,(uint64_t)value->value,value->token);
}
extern void nyrt_handle_release_h(int64_t);
static uint32_t field_array_end(void *frame, uint64_t site, const NyrtDynamicFieldValueV1 *value) {
  (void)frame; (void)site;
  nyrt_handle_release_h(value->value);
  return 0;
}
static const NyrtDynamicFieldEndDescriptorV1 field_text[] = {{1,NYRT_FIELD_TEXT_V1,0,field_text_end},{2,NYRT_FIELD_TEXT_V1,0,field_text_end}};
static const NyrtDynamicFieldEndDescriptorV1 field_array[] = {{1,NYRT_FIELD_ARRAY_V1,0,field_array_end},{2,NYRT_FIELD_ARRAY_V1,0,field_array_end}};
static const NyrtDynamicFieldEndDescriptorV1 field_child[] = {{1,NYRT_FIELD_OBJECT_V1,941,field_object_end},{2,NYRT_FIELD_OBJECT_V1,941,field_object_end}};
static uint32_t field_word(size_t at) {
  const unsigned char *p=nyash_object_field_abi_descriptor_v1+at;
  return (uint32_t)p[0] | (uint32_t)p[1]<<8 | (uint32_t)p[2]<<16 | (uint32_t)p[3]<<24;
}
static void field_finish(void *frame, uint64_t site) {
  assert(nyrt_object_field_outcome_end_v1(frame,site,field_outcome)==0);
  assert(nyrt_object_field_outcome_dispose_v1(field_outcome)==0);
}
int main(void) {
  if (getenv("HAKO_TYPED_OBJECT_STORE") && !strcmp(getenv("HAKO_TYPED_OBJECT_STORE"),"single_thread_exact")) field_profile=2;
  assert(!memcmp(nyash_runtime_abi_descriptor_v2,"NYRTABI2",8));
  void *frame=region(56), *map=region(200), *key=region(212), *out=region(224);
  int64_t child=0, value=0;
  if (field_profile==1) {
  assert(nyrt_fault_frame_init_v1(frame)==0);
  assert(nyrt_map_storage_init_v1(map)==0);
  assert(nyrt_map_checked_new_v1(frame,1,1,map)==0);
  assert(nyrt_map_key_init_v1(key)==0);
  const uint8_t text[]={'a',0,'b'};
  assert(nyrt_map_key_prepare_utf8_v1(frame,2,key,text,sizeof(text))==0);
  const uint32_t layout[]={1};
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
  }
  /* Scalar tags, explicit Empty, atomic replacement and exact child End. */
  assert(field_word(0)==32 && field_word(4)==1);
  assert(field_word(8)==sizeof(NyrtDynamicFieldValueV1));
  assert(field_word(12)==_Alignof(NyrtDynamicFieldValueV1));
  assert(field_word(24)==sizeof(NyrtDynamicFieldEndDescriptorV1));
  assert(field_word(28)==_Alignof(NyrtDynamicFieldEndDescriptorV1));
  field_outcome=aligned_alloc(field_word(20),field_word(16)); assert(field_outcome);
  assert(nyrt_fault_frame_init_v1(frame)==0);
  const uint32_t dynamic_layout[]={NYRT_OBJECT_STORAGE_DYNAMIC_V1};
  assert(nyrt_object_checked_new_v1(frame,field_profile,100,940,dynamic_layout,1,&field_parent)==0);
  NyrtDynamicFieldValueV1 scalar={NYRT_FIELD_I64_V1,0,7,0}, seen={99,88,77,66};
  NyrtDynamicFieldValueV1 sentinel=seen;
  assert(nyrt_object_checked_dynamic_get_v1(frame,field_profile,110,field_parent,940,0,NYRT_FIELD_I64_V1,&seen)==1);
  assert(!memcmp(&seen,&sentinel,sizeof(seen)));
  assert(((NyrtFaultFrameV1*)frame)->primary.site==110);
  assert(nyrt_object_checked_dynamic_get_v1(frame,field_profile,111,field_parent,940,0,0,&seen)==1);
  assert(!memcmp(&seen,&sentinel,sizeof(seen)));
  const NyrtDynamicFieldValueV1 scalars[]={scalar,{NYRT_FIELD_BOOL_V1,0,1,0},{NYRT_FIELD_NULL_V1,0,0,0}};
  for (size_t i=0;i<3;i++) {
    assert(nyrt_object_field_outcome_init_v1(field_outcome)==0);
    assert(nyrt_object_checked_dynamic_replace_v1(frame,field_profile,120,field_parent,940,0,&scalars[i],NULL,field_outcome)==0);
    assert(nyrt_object_field_outcome_dispose_v1(field_outcome)==2);
    assert(nyrt_object_checked_dynamic_get_v1(frame,field_profile,121,field_parent,940,0,scalars[i].kind,&seen)==0);
    assert(!memcmp(&seen,&scalars[i],sizeof(seen)));
    assert(nyrt_object_checked_dynamic_get_v1(frame,field_profile,121,field_parent,940,0,0,&seen)==0);
    assert(seen.kind==scalars[i].kind);
    if (scalars[i].kind!=NYRT_FIELD_I64_V1)
      assert(nyrt_object_checked_dynamic_get_v1(frame,field_profile,122,field_parent,940,0,NYRT_FIELD_I64_V1,&seen)==1);
    field_finish(frame,123);
  }
  assert(nyrt_object_field_outcome_init_v1(field_outcome)==0);
  NyrtDynamicFieldValueV1 invalid={NYRT_FIELD_BOOL_V1,0,2,0};
  const NyrtDynamicFieldValueV1 rejected[]={invalid,{NYRT_FIELD_EMPTY_V1,0,0,0},{7,0,1,0}};
  for (size_t i=0;i<3;i++)
    assert(nyrt_object_checked_dynamic_replace_v1(frame,field_profile,130,field_parent,940,0,&rejected[i],NULL,field_outcome)==2);
  assert(nyrt_object_checked_dynamic_replace_v1(frame,field_profile,131,field_parent,999,0,&scalar,NULL,field_outcome)==1);
  assert(nyrt_object_checked_dynamic_get_v1(frame,field_profile,132,field_parent,940,0,NYRT_FIELD_NULL_V1,&seen)==0);
  assert(nyrt_object_field_outcome_dispose_v1(field_outcome)==0);
  assert(nyrt_object_home_release_plain_i64_v1(frame,field_profile,133,field_parent,940)==1);
  const uint32_t child_layout[]={1};
  assert(nyrt_object_checked_new_v1(frame,field_profile,140,941,child_layout,1,&child)==0);
  assert(nyrt_object_checked_field_set_v1(frame,field_profile,140,child,941,0,17)==0);
  assert(nyrt_object_checked_field_get_i64_v1(field_profile,child,941,0,&value)==0 && value==17);
  NyrtDynamicFieldValueV1 object={NYRT_FIELD_OBJECT_V1,0,child,0};
  assert(nyrt_object_field_outcome_init_v1(field_outcome)==0);
  assert(nyrt_object_checked_dynamic_replace_v1(frame,field_profile,141,field_parent,940,0,&object,NULL,field_outcome)==2);
  assert(nyrt_object_checked_dynamic_replace_v1(frame,field_profile,142,field_parent,940,0,&object,&field_child[field_profile-1],field_outcome)==0);
  field_finish(frame,143);
  assert(nyrt_object_checked_dynamic_get_v1(frame,field_profile,143,field_parent,940,0,0,&seen)==0 && seen.kind==NYRT_FIELD_OBJECT_V1);
  assert(nyrt_object_field_outcome_init_v1(field_outcome)==0);
  assert(nyrt_object_checked_dynamic_replace_v1(frame,field_profile,144,field_parent,940,0,&object,&field_child[field_profile-1],field_outcome)==1);
  scalar.value=42;
  assert(nyrt_object_checked_dynamic_replace_v1(frame,field_profile,150,field_parent,940,0,&scalar,NULL,field_outcome)==0);
  assert(nyrt_object_field_outcome_end_v1(frame,160,field_outcome)==1);
  assert(field_end_count==1 && ((NyrtFaultFrameV1*)frame)->primary.site==110);
  assert(nyrt_object_checked_dynamic_get_v1(frame,field_profile,160,field_parent,940,0,NYRT_FIELD_I64_V1,&seen)==0 && seen.value==42);
  assert(nyrt_object_field_outcome_end_v1(frame,161,field_outcome)==2);
  assert(nyrt_object_field_outcome_dispose_v1(field_outcome)==0);
  value=777; assert(nyrt_object_checked_field_get_i64_v1(field_profile,child,941,0,&value)==2 && value==777);
  /* Independent Text preparation transfers an existing generation/token pair. */
  NyrtStringOperandV1 literal={NYRT_STRING_STATIC_TEXT_V1,0,(const uint8_t*)"abc",3,0,0,0};
  NyrtOwnedStringV1 text_pair={0};
  assert(nyrt_string_checked_materialize_v1(frame,170,&literal,&text_pair)==0);
  NyrtDynamicFieldValueV1 text_value={NYRT_FIELD_TEXT_V1,0,(int64_t)text_pair.handle,text_pair.token};
  assert(nyrt_object_field_outcome_init_v1(field_outcome)==0);
  assert(nyrt_object_checked_dynamic_replace_v1(frame,field_profile,171,field_parent,999,0,&text_value,&field_text[field_profile-1],field_outcome)==1);
  NyrtStringOperandV1 borrowed={NYRT_STRING_OWNED_TEXT_V1,0,NULL,0,text_pair.handle,text_pair.token,0};
  assert(nyrt_string_checked_length_v1(frame,172,&borrowed,&value)==0 && value==3);
  assert(nyrt_object_checked_dynamic_replace_v1(frame,field_profile,173,field_parent,940,0,&text_value,&field_text[field_profile-1],field_outcome)==0);
  field_finish(frame,174);
  assert(nyrt_object_checked_dynamic_get_v1(frame,field_profile,175,field_parent,940,0,NYRT_FIELD_TEXT_V1,&seen)==0);
  assert(seen.value==text_value.value && seen.token==text_value.token);
  assert(nyrt_object_field_outcome_init_v1(field_outcome)==0);
  assert(nyrt_object_checked_dynamic_take_v1(frame,field_profile,176,field_parent,940,0,field_outcome)==0);
  field_finish(frame,177);
  assert(nyrt_string_checked_length_v1(frame,178,&borrowed,&value)==2);
  /* Direct Array Home uses the existing source-selected release owner. */
  int64_t array=0; assert(nyrt_array_checked_new_v1(frame,180,&array)==0);
  assert(nyrt_array_checked_claim_v1(frame,181,array,NYRT_ARRAY_ELEMENT_I64_V1)==0);
  NyrtDynamicFieldValueV1 array_value={NYRT_FIELD_ARRAY_V1,0,array,0};
  assert(nyrt_object_field_outcome_init_v1(field_outcome)==0);
  assert(nyrt_object_checked_dynamic_replace_v1(frame,field_profile,182,field_parent,940,0,&array_value,&field_array[field_profile-1],field_outcome)==0);
  field_finish(frame,183);
  assert(nyrt_object_checked_dynamic_get_v1(frame,field_profile,184,field_parent,940,0,NYRT_FIELD_ARRAY_V1,&seen)==0 && seen.value==array);
  assert(nyrt_object_field_outcome_init_v1(field_outcome)==0);
  assert(nyrt_object_checked_dynamic_take_v1(frame,field_profile,185,field_parent,940,0,field_outcome)==0);
  field_finish(frame,186);
  assert(nyrt_array_checked_append_i64_v1(frame,187,array,1)==2);
  assert(nyrt_object_home_release_plain_i64_v1(frame,field_profile,190,field_parent,940)==0);
  assert(nyrt_fault_frame_dispose_v1(frame)==0);
  free(field_outcome);
  free(frame); free(map); free(key); free(out);
  printf("checked field C ABI profile=%u: tagged replacement, detached End and disposal passed%s\n",
      field_profile,field_profile==1 ? " (Map also checked)" : "");
  return 0;
}
