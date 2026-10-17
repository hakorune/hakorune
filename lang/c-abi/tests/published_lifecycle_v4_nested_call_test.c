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


  /* Normal results follow a leading PHI prefix, but no ordinary operation.
   * Both PHIs borrow the exact Invoke predecessor's available I64 value. */
  char* prefix_input = replace_once(nested,
      "\"mode\":\"root_owned\"}}],\"terminator\":{\"index\":1",
      "\"mode\":\"root_owned\"}},{\"index\":1,\"instruction\":{\"op\":\"const_i64\",\"dst\":3,\"value\":7}}],\"terminator\":{\"index\":2");
  const char* normal_row =
      "{\"index\":0,\"instruction\":{\"op\":\"invoke_normal_result\",\"invoke_block\":0,\"dst\":2}}],\"terminator\":{\"index\":1";
  const char* prefix_rows =
      "{\"index\":0,\"instruction\":{\"op\":\"phi\",\"dst\":4,\"inputs\":[{\"block\":0,\"value\":3}]}},"
      "{\"index\":1,\"instruction\":{\"op\":\"phi\",\"dst\":5,\"inputs\":[{\"block\":0,\"value\":3}]}},"
      "{\"index\":2,\"instruction\":{\"op\":\"invoke_normal_result\",\"invoke_block\":0,\"dst\":2}}],\"terminator\":{\"index\":3";
  char* prefix = replace_once(prefix_input, normal_row, prefix_rows);
  free(prefix_input);
  error = NULL;
  assert(compile(&session, prefix, &error) == 0);
  assert(error == NULL);
  output = fopen(output_path, "rb");
  assert(output && fgetc(output) != EOF);
  assert(fclose(output) == 0);
  const char* prefix_bad_needles[] = {
      "\"op\":\"phi\",\"dst\":5,\"inputs\":[{\"block\":0,\"value\":3}]",
      "\"op\":\"invoke_normal_result\",\"invoke_block\":0,\"dst\":2",
      "\"normal\":1,\"fault\":2"
  };
  const char* prefix_bad_rows[] = {
      "\"op\":\"copy\",\"dst\":5,\"src\":3",
      "\"op\":\"invoke_normal_result\",\"invoke_block\":1,\"dst\":2",
      "\"normal\":2,\"fault\":1"
  };
  for (size_t i = 0; i < 3; i++) {
    char* changed = replace_once(prefix, prefix_bad_needles[i], prefix_bad_rows[i]);
    error = NULL;
    assert(compile(&session, changed, &error) != 0);
    assert(error);
    free(error);
    free(changed);
    assert(fopen(output_path, "rb") == NULL);
  }
  free(prefix);

  /* Physical arithmetic is I64-only. The checked source projection is tested
   * in Rust; these cases exercise V4 admission and emitted LLVM independently. */
  const char* scalar_rows =
      "{\"index\":1,\"instruction\":{\"op\":\"const_i64\",\"dst\":2,\"value\":-9}},"
      "{\"index\":2,\"instruction\":{\"op\":\"const_i64\",\"dst\":3,\"value\":4}},"
      "{\"index\":3,\"instruction\":{\"op\":\"sub\",\"dst\":4,\"lhs\":2,\"rhs\":3}},"
      "{\"index\":4,\"instruction\":{\"op\":\"div\",\"dst\":5,\"lhs\":4,\"rhs\":3}}";
  char* scalar = replace_once(nested,
      "{\"index\":1,\"instruction\":{\"op\":\"const_i64\",\"dst\":2,\"value\":7}}",
      scalar_rows);
  char* scalar_complete = replace_once(scalar,
      "\"index\":2,\"instruction\":{\"op\":\"return\",\"value\":2}",
      "\"index\":5,\"instruction\":{\"op\":\"return\",\"value\":5}");
  free(scalar);
  error = NULL;
  assert(compile(&session, scalar_complete, &error) == 0);
  assert(error == NULL);
  const char* bad_scalar[] = {
      "\"op\":\"const_bool\",\"dst\":2,\"value\":true",
      "\"op\":\"const_null\",\"dst\":2"
  };
  for (size_t i = 0; i < sizeof(bad_scalar) / sizeof(bad_scalar[0]); i++) {
    char* changed = replace_once(scalar_complete,
        "\"op\":\"const_i64\",\"dst\":2,\"value\":-9", bad_scalar[i]);
    error = NULL;
    assert(compile(&session, changed, &error) != 0);
    assert(error);
    free(error);
    free(changed);
    assert(fopen(output_path, "rb") == NULL);
  }
  /* Add keeps its checked tagged behavior; Sub/Div do not inherit it. */
  char* tagged = replace_once(nested,
      "\"mode\":\"borrowed\"}}],\"terminator\":{\"index\":1",
      "\"mode\":\"borrowed\"}},{\"index\":1,\"instruction\":{\"op\":\"const_i64\",\"dst\":3,\"value\":9}}],\"terminator\":{\"index\":2");
  char* tagged_call = replace_once(tagged, "\"target\":2,\"args\":[]",
      "\"target\":2,\"args\":[{\"kind\":1,\"value\":3}]");
  free(tagged);
  tagged = replace_once(tagged_call,
      "\"name\":\"inner\",\"role\":\"ordinary_i64\",\"receiver\":null,\"receiver_object\":null,\"params\":[]",
      "\"name\":\"inner\",\"role\":\"ordinary_i64\",\"receiver\":null,\"receiver_object\":null,\"params\":[{\"value\":9,\"representation\":\"borrowed_kind_payload_v1\",\"object_view\":null}]");
  free(tagged_call);
  tagged_call = replace_once(tagged, "\"dst\":2,\"value\":7}}",
      "\"dst\":2,\"value\":7}},{\"index\":2,\"instruction\":{\"op\":\"add\",\"dst\":3,\"lhs\":9,\"rhs\":2}}");
  free(tagged);
  tagged = replace_once(tagged_call,
      "\"index\":2,\"instruction\":{\"op\":\"return\",\"value\":2}",
      "\"index\":3,\"instruction\":{\"op\":\"return\",\"value\":3}");
  free(tagged_call);
  error = NULL;
  assert(compile(&session, tagged, &error) == 0);
  assert(error == NULL);
  for (size_t i = 0; i < 2; i++) {
    char* changed = replace_once(tagged, "\"op\":\"add\"",
        i ? "\"op\":\"div\"" : "\"op\":\"sub\"");
    error = NULL;
    assert(compile(&session, changed, &error) != 0);
    assert(error && strstr(error, "unsupported-cohort"));
    free(error);
    free(changed);
    assert(fopen(output_path, "rb") == NULL);
  }
  free(tagged);
  free(scalar_complete);

  /* Installed-object teardown: object 7 stores an owned-array child 8
   * whose layout row marks its sole ArrayBox residence. The
   * object_field_release consumer must emit the nested walk — read the
   * child's marked slot, release the live residence, then release the
   * child storage — inside one checked step. */
  const char* nested_release =
      "{\"schema\":\"hako.published-lifecycle-physical-program.v2\",\"fault_abi_version\":1,\"storage_profile\":1,\"process_result_site\":99,\"functions\":["
      "{\"name\":\"main\",\"role\":\"root_i64\",\"receiver\":null,\"receiver_object\":null,\"params\":[],\"entry\":0,\"blocks\":["
      "{\"id\":0,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"fault_frame_enter\",\"dst\":1,\"mode\":\"root_owned\"}}],\"terminator\":{\"index\":1,\"instruction\":{\"op\":\"invoke\",\"operation\":{\"kind\":\"new_box\",\"object_id\":7,\"site\":0},\"fault_frame\":1,\"normal\":1,\"fault\":9}},\"edges\":[{\"target\":1,\"args\":null},{\"target\":9,\"args\":null}]},"
      "{\"id\":1,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"invoke_normal_result\",\"invoke_block\":0,\"dst\":2}}],\"terminator\":{\"index\":1,\"instruction\":{\"op\":\"invoke\",\"operation\":{\"kind\":\"object_field_release\",\"object_id\":7,\"field_ordinal\":0,\"base\":2,\"child_object_id\":8,\"site\":1},\"fault_frame\":1,\"normal\":2,\"fault\":10}},\"edges\":[{\"target\":2,\"args\":null},{\"target\":10,\"args\":null}]},"
      "{\"id\":2,\"instructions\":[],\"terminator\":{\"index\":0,\"instruction\":{\"op\":\"invoke\",\"operation\":{\"kind\":\"home_release\",\"object_id\":7,\"value\":2,\"site\":2},\"fault_frame\":1,\"normal\":5,\"fault\":9}},\"edges\":[{\"target\":5,\"args\":null},{\"target\":9,\"args\":null}]},"
      "{\"id\":5,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"const_i64\",\"dst\":4,\"value\":0}}],\"terminator\":{\"index\":1,\"instruction\":{\"op\":\"return\",\"value\":4}},\"edges\":[]},"
      "{\"id\":9,\"instructions\":[],\"terminator\":{\"index\":0,\"instruction\":{\"op\":\"return_fault\",\"fault_frame\":1}},\"edges\":[]},"
      "{\"id\":10,\"instructions\":[],\"terminator\":{\"index\":0,\"instruction\":{\"op\":\"invoke\",\"operation\":{\"kind\":\"home_release\",\"object_id\":7,\"value\":2,\"site\":3},\"fault_frame\":1,\"normal\":9,\"fault\":9}},\"edges\":[{\"target\":9,\"args\":null},{\"target\":9,\"args\":null}]}]}"
      "],\"layouts\":[{\"object_id\":7,\"runtime_type_id\":7,\"field_count\":1,\"fields\":[{\"declaration_ordinal\":0,\"runtime_slot\":0,\"storage_kind\":1}],\"owned_residences\":[]},{\"object_id\":8,\"runtime_type_id\":8,\"field_count\":1,\"fields\":[{\"declaration_ordinal\":0,\"runtime_slot\":0,\"storage_kind\":1}],\"owned_residences\":[0]}]}";
  error = NULL;
  rc = compile(&session, nested_release, &error);
  assert(rc == 0);
  assert(error == NULL);
  output = fopen(output_path, "rb");
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

  /* A birth caller owns the ExactI64 provider-argument lane: its sealed
   * qualified-static call invokes the ordinary callee through the same
   * caller-local out-slot, lands faults on the shared reclaim head, and
   * the projection feeds the birth_call actuals. Root's own fault edges
   * converge through the discharge block so merged lease states agree. */
  const char* birth_caller =
      "{\"schema\":\"hako.published-lifecycle-physical-program.v2\",\"fault_abi_version\":1,\"storage_profile\":1,\"process_result_site\":99,\"functions\":["
      "{\"name\":\"main\",\"role\":\"root_i64\",\"receiver\":null,\"receiver_object\":null,\"params\":[],\"entry\":0,\"blocks\":["
      "{\"id\":0,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"fault_frame_enter\",\"dst\":1,\"mode\":\"root_owned\"}}],\"terminator\":{\"index\":1,\"instruction\":{\"op\":\"invoke\",\"operation\":{\"kind\":\"new_box\",\"object_id\":7,\"site\":0},\"fault_frame\":1,\"normal\":1,\"fault\":4}},\"edges\":[{\"target\":1,\"args\":null},{\"target\":4,\"args\":null}]},"
      "{\"id\":1,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"invoke_normal_result\",\"invoke_block\":0,\"dst\":2}}],\"terminator\":{\"index\":1,\"instruction\":{\"op\":\"invoke\",\"operation\":{\"kind\":\"birth_call\",\"call\":{\"target\":1,\"receiver\":2,\"args\":[],\"dst\":null}},\"fault_frame\":1,\"normal\":2,\"fault\":5}},\"edges\":[{\"target\":2,\"args\":null},{\"target\":5,\"args\":null}]},"
      "{\"id\":2,\"instructions\":[],\"terminator\":{\"index\":0,\"instruction\":{\"op\":\"invoke\",\"operation\":{\"kind\":\"home_release\",\"object_id\":7,\"value\":2,\"site\":1},\"fault_frame\":1,\"normal\":3,\"fault\":4}},\"edges\":[{\"target\":3,\"args\":null},{\"target\":4,\"args\":null}]},"
      "{\"id\":3,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"const_i64\",\"dst\":3,\"value\":0}}],\"terminator\":{\"index\":1,\"instruction\":{\"op\":\"return\",\"value\":3}},\"edges\":[]},"
      "{\"id\":4,\"instructions\":[],\"terminator\":{\"index\":0,\"instruction\":{\"op\":\"return_fault\",\"fault_frame\":1}},\"edges\":[]},"
      "{\"id\":5,\"instructions\":[],\"terminator\":{\"index\":0,\"instruction\":{\"op\":\"invoke\",\"operation\":{\"kind\":\"home_release\",\"object_id\":7,\"value\":2,\"site\":2},\"fault_frame\":1,\"normal\":4,\"fault\":4}},\"edges\":[{\"target\":4,\"args\":null},{\"target\":4,\"args\":null}]}]},"
      "{\"name\":\"Pair.birth\",\"role\":\"birth_unit\",\"receiver\":0,\"receiver_object\":null,\"params\":[],\"entry\":0,\"blocks\":["
      "{\"id\":0,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"fault_frame_enter\",\"dst\":1,\"mode\":\"borrowed\"}}],\"terminator\":{\"index\":1,\"instruction\":{\"op\":\"invoke\",\"operation\":{\"kind\":\"ordinary_call\",\"call\":{\"target\":2,\"args\":[],\"dst\":null},\"result\":\"i64\"},\"fault_frame\":1,\"normal\":1,\"fault\":2}},\"edges\":[{\"target\":1,\"args\":null},{\"target\":2,\"args\":null}]},"
      "{\"id\":1,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"invoke_normal_result\",\"invoke_block\":0,\"dst\":2}},{\"index\":1,\"instruction\":{\"op\":\"const_unit\",\"dst\":3}}],\"terminator\":{\"index\":2,\"instruction\":{\"op\":\"return\",\"value\":3}},\"edges\":[]},"
      "{\"id\":2,\"instructions\":[],\"terminator\":{\"index\":0,\"instruction\":{\"op\":\"return_fault\",\"fault_frame\":1}},\"edges\":[]}]},"
      "{\"name\":\"inner\",\"role\":\"ordinary_i64\",\"receiver\":null,\"receiver_object\":null,\"params\":[],\"entry\":0,\"blocks\":["
      "{\"id\":0,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"fault_frame_enter\",\"dst\":1,\"mode\":\"borrowed\"}},{\"index\":1,\"instruction\":{\"op\":\"const_i64\",\"dst\":2,\"value\":7}}],\"terminator\":{\"index\":2,\"instruction\":{\"op\":\"return\",\"value\":2}},\"edges\":[]}]}],"
      "\"layouts\":[{\"object_id\":7,\"runtime_type_id\":7,\"field_count\":0,\"fields\":[],\"owned_residences\":[]}]}";
  error = NULL;
  rc = compile(&session, birth_caller, &error);
  assert(rc == 0);
  assert(error == NULL);
  output = fopen(output_path, "rb");
  assert(output);
  assert(fgetc(output) != EOF);
  assert(fclose(output) == 0);

  /* The same birth caller spelling any other result kind is outside the
   * sealed ExactI64 provider lane: a map claim fails the parser's
   * function-body contract before admission even runs. */
  char* birth_map = replace_once(birth_caller,
      "\"kind\":\"ordinary_call\",\"call\":{\"target\":2,\"args\":[],\"dst\":null},\"result\":\"i64\"",
      "\"kind\":\"ordinary_call\",\"call\":{\"target\":2,\"args\":[],\"dst\":null},\"result\":\"map\"");
  error = NULL;
  rc = compile(&session, birth_map, &error);
  assert(rc != 0);
  assert(error != NULL);
  assert(strstr(error,
      "[freeze:contract][published-lifecycle-physical-parser/function-body]") != NULL);
  free(error);
  assert(fopen(output_path, "rb") == NULL);
  free(birth_map);


  /* Independent ABI fixture: Bool keeps its role through nested call/return,
     including an unused root result. This is not source admission evidence. */
  char* bool_nested = replace_once(nested,
      "\"op\":\"invoke_normal_result\",\"invoke_block\":0,\"dst\":2}}],\"terminator\":{\"index\":1,\"instruction\":{\"op\":\"return\",\"value\":2}",
      "\"op\":\"invoke_normal_result\",\"invoke_block\":0,\"dst\":2}},{\"index\":1,\"instruction\":{\"op\":\"const_i64\",\"dst\":3,\"value\":7}}],\"terminator\":{\"index\":2,\"instruction\":{\"op\":\"return\",\"value\":3}");
  for (int call = 0; call < 2; call++) {
    char* next = replace_once(bool_nested, "ordinary_i64", "ordinary_bool");
    free(bool_nested); bool_nested = next;
    next = replace_once(bool_nested, "\"result\":\"i64\"", "\"result\":\"bool\"");
    free(bool_nested); bool_nested = next;
  }
  char* bool_leaf = replace_once(bool_nested,
      "\"op\":\"const_i64\",\"dst\":2,\"value\":7",
      "\"op\":\"const_bool\",\"dst\":2,\"value\":true");
  free(bool_nested); bool_nested = bool_leaf;
  for (int boolean = 0; boolean < 2; boolean++) {
    char* input = boolean ? replace_once(bool_nested, "\"value\":true", "\"value\":true")
        : replace_once(bool_nested, "\"value\":true", "\"value\":false");
    error = NULL;
    int bool_rc = compile(&session, input, &error);
    if (bool_rc != 0) fprintf(stderr, "Bool fixture: %s\n", error ? error : "missing diagnostic");
    assert(bool_rc == 0);
    assert(error == NULL);
    free(input);
  }
  char* wrong_bool = replace_once(bool_nested,
      "\"op\":\"const_bool\",\"dst\":2,\"value\":true",
      "\"op\":\"const_i64\",\"dst\":2,\"value\":1");
  error = NULL;
  assert(compile(&session, wrong_bool, &error) != 0);
  assert(error != NULL);
  free(error);
  free(wrong_bool);
  assert(fopen(output_path, "rb") == NULL);
  char* wrong_role = replace_once(bool_nested, "ordinary_bool", "ordinary_i64");
  error = NULL;
  assert(compile(&session, wrong_role, &error) != 0);
  assert(error != NULL);
  free(error);
  free(wrong_role);
  assert(fopen(output_path, "rb") == NULL);
  free(bool_nested);

  remove(input_path);
  return 0;
}
