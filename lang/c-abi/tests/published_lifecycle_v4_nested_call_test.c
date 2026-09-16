#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../include/hako_llvmc_ffi.h"
#include "../../../include/nyrt_fault_v1.h"

static const char* input_path = "/tmp/hako-lifecycle-v4-nested-call.json";
static const char* output_path = "/tmp/hako-lifecycle-v4-nested-call.o";

static void write_input(const char* text) {
  FILE* file = fopen(input_path, "wb");
  assert(file);
  assert(fputs(text, file) >= 0);
  assert(fclose(file) == 0);
}

static char* replace_once(const char* source, const char* needle,
    const char* replacement) {
  const char* at = strstr(source, needle);
  size_t before, total;
  char* changed;
  assert(at);
  before = (size_t)(at - source);
  total = before + strlen(replacement) + strlen(at + strlen(needle)) + 1;
  changed = malloc(total);
  assert(changed);
  memcpy(changed, source, before);
  strcpy(changed + before, replacement);
  strcat(changed, at + strlen(needle));
  return changed;
}

static int compile(const hako_llvmc_lifecycle_target_session_v2* session,
    const char* json, char** error) {
  write_input(json);
  remove(output_path);
  return hako_llvmc_compile_published_lifecycle_physical_v4(
      input_path, session, output_path, error);
}

int main(void) {
  /* root -> helper -> inner: an ordinary_i64 callee performs its own
   * ordinary i64 call. The nested result lands in a caller-local out-slot;
   * the callee's own %out_i64 stays reserved for its return handoff. */
  const char* nested =
      "{\"schema\":\"hako.published-lifecycle-physical-program.v2\",\"fault_abi_version\":1,\"storage_profile\":1,\"process_result_site\":99,\"functions\":["
      "{\"name\":\"main\",\"role\":\"root_i64\",\"receiver\":null,\"receiver_object\":null,\"params\":[],\"entry\":0,\"blocks\":["
      "{\"id\":0,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"fault_frame_enter\",\"dst\":1,\"mode\":\"root_owned\"}}],\"terminator\":{\"index\":1,\"instruction\":{\"op\":\"invoke\",\"operation\":{\"kind\":\"ordinary_call\",\"call\":{\"target\":1,\"args\":[],\"dst\":null},\"result\":\"i64\"},\"fault_frame\":1,\"normal\":1,\"fault\":2}},\"edges\":[{\"target\":1,\"args\":null},{\"target\":2,\"args\":null}]},"
      "{\"id\":1,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"invoke_normal_result\",\"invoke_block\":0,\"dst\":2}}],\"terminator\":{\"index\":1,\"instruction\":{\"op\":\"return\",\"value\":2}},\"edges\":[]},"
      "{\"id\":2,\"instructions\":[],\"terminator\":{\"index\":0,\"instruction\":{\"op\":\"return_fault\",\"fault_frame\":1}},\"edges\":[]}]},"
      "{\"name\":\"helper\",\"role\":\"ordinary_i64\",\"receiver\":null,\"receiver_object\":null,\"params\":[],\"entry\":0,\"blocks\":["
      "{\"id\":0,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"fault_frame_enter\",\"dst\":1,\"mode\":\"borrowed\"}}],\"terminator\":{\"index\":1,\"instruction\":{\"op\":\"invoke\",\"operation\":{\"kind\":\"ordinary_call\",\"call\":{\"target\":2,\"args\":[],\"dst\":null},\"result\":\"i64\"},\"fault_frame\":1,\"normal\":1,\"fault\":2}},\"edges\":[{\"target\":1,\"args\":null},{\"target\":2,\"args\":null}]},"
      "{\"id\":1,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"invoke_normal_result\",\"invoke_block\":0,\"dst\":2}}],\"terminator\":{\"index\":1,\"instruction\":{\"op\":\"return\",\"value\":2}},\"edges\":[]},"
      "{\"id\":2,\"instructions\":[],\"terminator\":{\"index\":0,\"instruction\":{\"op\":\"return_fault\",\"fault_frame\":1}},\"edges\":[]}]},"
      "{\"name\":\"inner\",\"role\":\"ordinary_i64\",\"receiver\":null,\"receiver_object\":null,\"params\":[],\"entry\":0,\"blocks\":["
      "{\"id\":0,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"fault_frame_enter\",\"dst\":1,\"mode\":\"borrowed\"}},{\"index\":1,\"instruction\":{\"op\":\"const_i64\",\"dst\":2,\"value\":7}}],\"terminator\":{\"index\":2,\"instruction\":{\"op\":\"return\",\"value\":2}},\"edges\":[]}]}],"
      "\"layouts\":[]}";

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
  int rc = compile(&session, nested, &error);
  assert(rc == 0);
  assert(error == NULL);
  FILE* output = fopen(output_path, "rb");
  assert(output);
  assert(fgetc(output) != EOF);
  assert(fclose(output) == 0);

  /* The same nested shape with the helper's invoke_block pointer moved to a
   * non-invoke block is a parser-level malformed relation. */
  char* malformed = replace_once(nested,
      "\"invoke_normal_result\",\"invoke_block\":0,\"dst\":2}}],\"terminator\":{\"index\":1,\"instruction\":{\"op\":\"return\",\"value\":2}},\"edges\":[]},{\"id\":2,\"instructions\":[],\"terminator\":{\"index\":0,\"instruction\":{\"op\":\"return_fault\",\"fault_frame\":1}},\"edges\":[]}]},"
      "{\"name\":\"inner\"",
      "\"invoke_normal_result\",\"invoke_block\":1,\"dst\":2}}],\"terminator\":{\"index\":1,\"instruction\":{\"op\":\"return\",\"value\":2}},\"edges\":[]},{\"id\":2,\"instructions\":[],\"terminator\":{\"index\":0,\"instruction\":{\"op\":\"return_fault\",\"fault_frame\":1}},\"edges\":[]}]},"
      "{\"name\":\"inner\"");
  error = NULL;
  rc = compile(&session, malformed, &error);
  assert(rc != 0);
  assert(error != NULL);
  assert(strstr(error,
      "[freeze:contract][published-lifecycle-physical-parser/function-body]") != NULL);
  free(error);
  assert(fopen(output_path, "rb") == NULL);
  free(malformed);

  /* An ordinary-call target that drifts to a non-ordinary role is a
   * parser-level malformed relation. */
  char* drifted = replace_once(nested,
      "{\"name\":\"helper\",\"role\":\"ordinary_i64\"",
      "{\"name\":\"helper\",\"role\":\"birth_unit\"");
  error = NULL;
  rc = compile(&session, drifted, &error);
  assert(rc != 0);
  assert(error != NULL);
  assert(strstr(error,
      "[freeze:contract][published-lifecycle-physical-parser/function-body]") != NULL);
  free(error);
  assert(fopen(output_path, "rb") == NULL);
  free(drifted);

  /* A birth caller may not issue an ordinary call even when the call
   * itself is structurally valid: only root and ordinary callers own that
   * lane. The parser admits the shape; V4 admission rejects the cohort. */
  const char* birth_caller =
      "{\"schema\":\"hako.published-lifecycle-physical-program.v2\",\"fault_abi_version\":1,\"storage_profile\":1,\"process_result_site\":99,\"functions\":["
      "{\"name\":\"main\",\"role\":\"root_i64\",\"receiver\":null,\"receiver_object\":null,\"params\":[],\"entry\":0,\"blocks\":["
      "{\"id\":0,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"fault_frame_enter\",\"dst\":1,\"mode\":\"root_owned\"}}],\"terminator\":{\"index\":1,\"instruction\":{\"op\":\"invoke\",\"operation\":{\"kind\":\"new_box\",\"object_id\":7,\"site\":0},\"fault_frame\":1,\"normal\":1,\"fault\":4}},\"edges\":[{\"target\":1,\"args\":null},{\"target\":4,\"args\":null}]},"
      "{\"id\":1,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"invoke_normal_result\",\"invoke_block\":0,\"dst\":2}}],\"terminator\":{\"index\":1,\"instruction\":{\"op\":\"invoke\",\"operation\":{\"kind\":\"birth_call\",\"call\":{\"target\":1,\"receiver\":2,\"args\":[],\"dst\":null}},\"fault_frame\":1,\"normal\":2,\"fault\":4}},\"edges\":[{\"target\":2,\"args\":null},{\"target\":4,\"args\":null}]},"
      "{\"id\":2,\"instructions\":[],\"terminator\":{\"index\":0,\"instruction\":{\"op\":\"invoke\",\"operation\":{\"kind\":\"home_release\",\"object_id\":7,\"value\":2,\"site\":1},\"fault_frame\":1,\"normal\":3,\"fault\":4}},\"edges\":[{\"target\":3,\"args\":null},{\"target\":4,\"args\":null}]},"
      "{\"id\":3,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"const_i64\",\"dst\":3,\"value\":0}}],\"terminator\":{\"index\":1,\"instruction\":{\"op\":\"return\",\"value\":3}},\"edges\":[]},"
      "{\"id\":4,\"instructions\":[],\"terminator\":{\"index\":0,\"instruction\":{\"op\":\"return_fault\",\"fault_frame\":1}},\"edges\":[]}]},"
      "{\"name\":\"Pair.birth\",\"role\":\"birth_unit\",\"receiver\":0,\"receiver_object\":null,\"params\":[],\"entry\":0,\"blocks\":["
      "{\"id\":0,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"fault_frame_enter\",\"dst\":1,\"mode\":\"borrowed\"}}],\"terminator\":{\"index\":1,\"instruction\":{\"op\":\"invoke\",\"operation\":{\"kind\":\"ordinary_call\",\"call\":{\"target\":2,\"args\":[],\"dst\":null},\"result\":\"i64\"},\"fault_frame\":1,\"normal\":1,\"fault\":2}},\"edges\":[{\"target\":1,\"args\":null},{\"target\":2,\"args\":null}]},"
      "{\"id\":1,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"invoke_normal_result\",\"invoke_block\":0,\"dst\":2}},{\"index\":1,\"instruction\":{\"op\":\"const_unit\",\"dst\":3}}],\"terminator\":{\"index\":2,\"instruction\":{\"op\":\"return\",\"value\":3}},\"edges\":[]},"
      "{\"id\":2,\"instructions\":[],\"terminator\":{\"index\":0,\"instruction\":{\"op\":\"return_fault\",\"fault_frame\":1}},\"edges\":[]}]},"
      "{\"name\":\"inner\",\"role\":\"ordinary_i64\",\"receiver\":null,\"receiver_object\":null,\"params\":[],\"entry\":0,\"blocks\":["
      "{\"id\":0,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"fault_frame_enter\",\"dst\":1,\"mode\":\"borrowed\"}},{\"index\":1,\"instruction\":{\"op\":\"const_i64\",\"dst\":2,\"value\":7}}],\"terminator\":{\"index\":2,\"instruction\":{\"op\":\"return\",\"value\":2}},\"edges\":[]}]}],"
      "\"layouts\":[{\"object_id\":7,\"runtime_type_id\":7,\"field_count\":0,\"fields\":[]}]}";
  error = NULL;
  rc = compile(&session, birth_caller, &error);
  assert(rc != 0);
  assert(error != NULL);
  assert(strstr(error,
      "[freeze:contract][published-lifecycle-v4/unsupported-cohort]") != NULL);
  free(error);
  assert(fopen(output_path, "rb") == NULL);

  remove(input_path);
  return 0;
}
