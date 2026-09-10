#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../include/hako_llvmc_ffi.h"
#include "../../../include/nyrt_fault_v1.h"

static const char* input_path = "/tmp/hako-lifecycle-v4-receiver-identity.json";
static const char* output_path = "/tmp/hako-lifecycle-v4-receiver-identity.o";

int main(void) {
  /* The root allocates object 8, while the selected ordinary callee declares
   * object 7. V2 accepts both layouts; V4 must reject the mismatched receiver
   * before opening the target session or writing an object. */
  const char* json =
      "{\"schema\":\"hako.published-lifecycle-physical-program.v2\",\"fault_abi_version\":1,\"storage_profile\":1,\"process_result_site\":2,\"functions\":["
      "{\"name\":\"main\",\"role\":\"root_i64\",\"receiver\":null,\"receiver_object\":null,\"params\":[],\"entry\":0,\"blocks\":["
      "{\"id\":0,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"fault_frame_enter\",\"dst\":1,\"mode\":\"root_owned\"}}],\"terminator\":{\"index\":1,\"instruction\":{\"op\":\"invoke\",\"operation\":{\"kind\":\"new_box\",\"object_id\":8,\"site\":0},\"fault_frame\":1,\"normal\":1,\"fault\":2}},\"edges\":[{\"target\":1,\"args\":null},{\"target\":2,\"args\":null}]},"
      "{\"id\":1,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"invoke_normal_result\",\"invoke_block\":0,\"dst\":2}}],\"terminator\":{\"index\":1,\"instruction\":{\"op\":\"invoke\",\"operation\":{\"kind\":\"ordinary_call\",\"call\":{\"target\":1,\"receiver\":2,\"args\":[],\"dst\":null},\"result\":\"i64\"},\"fault_frame\":1,\"normal\":3,\"fault\":2}},\"edges\":[{\"target\":3,\"args\":null},{\"target\":2,\"args\":null}]},"
      "{\"id\":2,\"instructions\":[],\"terminator\":{\"index\":0,\"instruction\":{\"op\":\"return_fault\",\"fault_frame\":1}},\"edges\":[]},"
      "{\"id\":3,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"invoke_normal_result\",\"invoke_block\":1,\"dst\":3}}],\"terminator\":{\"index\":1,\"instruction\":{\"op\":\"invoke\",\"operation\":{\"kind\":\"home_release\",\"object_id\":8,\"value\":2,\"site\":1},\"fault_frame\":1,\"normal\":4,\"fault\":2}},\"edges\":[{\"target\":4,\"args\":null},{\"target\":2,\"args\":null}]},"
      "{\"id\":4,\"instructions\":[],\"terminator\":{\"index\":0,\"instruction\":{\"op\":\"return\",\"value\":3}},\"edges\":[]}]},"
      "{\"name\":\"Pair.sum\",\"role\":\"ordinary_i64\",\"receiver\":0,\"receiver_object\":7,\"params\":[],\"entry\":0,\"blocks\":["
      "{\"id\":0,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"fault_frame_enter\",\"dst\":1,\"mode\":\"borrowed\"}},{\"index\":1,\"instruction\":{\"op\":\"const_i64\",\"dst\":2,\"value\":42}}],\"terminator\":{\"index\":2,\"instruction\":{\"op\":\"return\",\"value\":2}},\"edges\":[]}] }"
      "],\"layouts\":[{\"object_id\":7,\"runtime_type_id\":7,\"field_count\":0,\"fields\":[]},{\"object_id\":8,\"runtime_type_id\":8,\"field_count\":0,\"fields\":[]}]}";
  FILE* file = fopen(input_path, "wb");
  assert(file);
  assert(fputs(json, file) >= 0);
  assert(fclose(file) == 0);
  remove(output_path);

  hako_llvmc_lifecycle_target_session_v2 session = {
    .revision = 2, .target_triple = "x86_64-unknown-linux-gnu",
    .endian = 1, .pointer_width = 8, .fault_abi_version = 1,
    .status_abi_version = 1, .diagnostic_size = sizeof(NyrtFaultDiagnosticV1),
    .diagnostic_align = _Alignof(NyrtFaultDiagnosticV1),
    .diagnostic_site_offset = offsetof(NyrtFaultDiagnosticV1, site),
    .diagnostic_details_offset = offsetof(NyrtFaultDiagnosticV1, details),
    .diagnostic_message_offset = offsetof(NyrtFaultDiagnosticV1, runtime_private_message),
    .frame_size = sizeof(NyrtFaultFrameV1), .frame_align = _Alignof(NyrtFaultFrameV1),
    .frame_primary_offset = offsetof(NyrtFaultFrameV1, primary),
    .frame_suppressed_offset = offsetof(NyrtFaultFrameV1, suppressed),
    .map_size = 32, .map_align = 8, .map_revision = 1,
    .key_size = 32, .key_align = 8, .key_revision = 1,
    .outcome_size = 32, .outcome_align = 8, .outcome_revision = 1,
  };
  char* error = NULL;
  int rc = hako_llvmc_compile_published_lifecycle_physical_v4(
      input_path, &session, output_path, &error);
  assert(rc != 0);
  assert(error == NULL || strstr(error, "published-lifecycle-v4") != NULL);
  free(error);
  assert(fopen(output_path, "rb") == NULL);
  remove(input_path);
  return 0;
}
