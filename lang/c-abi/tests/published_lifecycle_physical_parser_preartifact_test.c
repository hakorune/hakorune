#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../include/hako_llvmc_ffi.h"

static const char* path = "/tmp/hako-lifecycle-physical-parser.json";

static void write_json(const char* json) {
  FILE* file = fopen(path, "wb");
  assert(file);
  assert(fputs(json, file) >= 0);
  assert(fclose(file) == 0);
}

static void accepts(const char* json) {
  char* error = NULL;
  write_json(json);
  assert(hako_llvmc_validate_published_lifecycle_physical_v1(path, &error) == 0);
  assert(error == NULL);
}

static void rejects(const char* json, const char* reason) {
  char* error = NULL;
  write_json(json);
  assert(hako_llvmc_validate_published_lifecycle_physical_v1(path, &error) != 0);
  assert(error && strstr(error, reason));
  free(error);
}

static void rejects_replace(const char* source, const char* needle,
    const char* replacement, const char* reason) {
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
  rejects(changed, reason);
  free(changed);
}

int main(void) {
  /* This is a complete ABI-bearing physical input, deliberately covering
   * function membership, values, CFG, invoke, layouts and both frame modes. */
  const char* valid =
      "{\"schema\":\"hako.published-lifecycle-physical-program.v1\",\"fault_abi_version\":1,\"storage_profile\":1,\"functions\":["
      "{\"name\":\"main\",\"role\":\"root_i64\",\"params\":[],\"entry\":0,\"blocks\":["
      "{\"id\":0,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"const_i64\",\"dst\":1,\"value\":30}},{\"index\":1,\"instruction\":{\"op\":\"fault_frame_enter\",\"dst\":2,\"mode\":\"root_owned\"}}],\"terminator\":{\"index\":2,\"instruction\":{\"op\":\"invoke\",\"operation\":{\"kind\":\"new_box\",\"object_id\":7},\"fault_frame\":2,\"normal\":1,\"fault\":2}},\"edges\":[{\"target\":1,\"args\":null},{\"target\":2,\"args\":null}]},"
      "{\"id\":1,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"invoke_normal_result\",\"invoke_block\":0,\"dst\":3}},{\"index\":1,\"instruction\":{\"op\":\"const_i64\",\"dst\":4,\"value\":10}}],\"terminator\":{\"index\":2,\"instruction\":{\"op\":\"invoke\",\"operation\":{\"kind\":\"field_set\",\"object_id\":7,\"field_ordinal\":0,\"base\":3,\"value\":4},\"fault_frame\":2,\"normal\":3,\"fault\":2}},\"edges\":[{\"target\":3,\"args\":null},{\"target\":2,\"args\":null}]},"
      "{\"id\":2,\"instructions\":[],\"terminator\":{\"index\":0,\"instruction\":{\"op\":\"return_fault\",\"fault_frame\":2}},\"edges\":[]},"
      "{\"id\":3,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"object_field_get\",\"dst\":5,\"base\":3,\"object_id\":7,\"field_ordinal\":0}},{\"index\":1,\"instruction\":{\"op\":\"const_i64\",\"dst\":6,\"value\":20}},{\"index\":2,\"instruction\":{\"op\":\"add\",\"dst\":7,\"lhs\":5,\"rhs\":6}},{\"index\":3,\"instruction\":{\"op\":\"copy\",\"dst\":8,\"src\":7}}],\"terminator\":{\"index\":4,\"instruction\":{\"op\":\"jump\",\"target\":4,\"args\":null}},\"edges\":[{\"target\":4,\"args\":null}]},"
      "{\"id\":4,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"phi\",\"dst\":9,\"inputs\":[{\"block\":3,\"value\":8}]}}],\"terminator\":{\"index\":1,\"instruction\":{\"op\":\"invoke\",\"operation\":{\"kind\":\"birth_call\",\"call\":{\"target\":1,\"receiver\":3,\"args\":[4,6],\"dst\":null}},\"fault_frame\":2,\"normal\":5,\"fault\":2}},\"edges\":[{\"target\":5,\"args\":null},{\"target\":2,\"args\":null}]},"
      "{\"id\":5,\"instructions\":[],\"terminator\":{\"index\":0,\"instruction\":{\"op\":\"return\",\"value\":9}},\"edges\":[]}]},"
      "{\"name\":\"Pair.birth\",\"role\":\"birth_unit\",\"params\":[0,1,2],\"entry\":0,\"blocks\":[{\"id\":0,\"instructions\":[{\"index\":0,\"instruction\":{\"op\":\"fault_frame_enter\",\"dst\":3,\"mode\":\"borrowed\"}},{\"index\":1,\"instruction\":{\"op\":\"const_string\",\"dst\":4,\"value\":\"pair\"}},{\"index\":2,\"instruction\":{\"op\":\"const_unit\",\"dst\":5}},{\"index\":3,\"instruction\":{\"op\":\"birth_call\",\"call\":{\"target\":1,\"receiver\":0,\"args\":[1,2],\"dst\":null}}}],\"terminator\":{\"index\":4,\"instruction\":{\"op\":\"return\",\"value\":null}},\"edges\":[]}] }],"
      "\"layouts\":[{\"object_id\":7,\"runtime_type_id\":9,\"field_count\":1,\"fields\":[{\"declaration_ordinal\":0,\"runtime_slot\":0,\"storage_kind\":1}]}]}";
  accepts(valid);
  rejects("{}", "schema");
  rejects_replace(valid, "\"dst\":9", "\"dst\":8", "function-body");
  rejects_replace(valid, "\"op\":\"const_i64\",\"dst\":1,\"value\":30", "\"op\":\"const_i64\",\"dst\":1,\"dst\":1,\"value\":30", "function-body");
  rejects_replace(valid, "\"call\":{\"target\":1", "\"call\":{\"target\":0", "function-body");
  rejects_replace(valid, "\"args\":[4,6]", "\"args\":[4]", "function-body");
  rejects_replace(valid, "\"block\":3,\"value\":8", "\"block\":99,\"value\":8", "function-body");
  rejects_replace(valid, "\"block\":3,\"value\":8", "\"block\":3,\"value\":9", "function-body");
  rejects_replace(valid, "\"inputs\":[{\"block\":3,\"value\":8}]", "\"inputs\":[{\"block\":3,\"value\":8},{\"block\":3,\"value\":8}]", "function-body");
  rejects_replace(valid, "\"src\":7", "\"src\":99", "function-body");
  rejects_replace(valid, "\"operation\":{\"kind\":\"new_box\",\"object_id\":7}", "\"operation\":{\"kind\":\"new_box\",\"object_id\":99}", "function-body");
  rejects_replace(valid, "\"mode\":\"root_owned\"", "\"mode\":\"borrowed\"", "function-body");
  rejects_replace(valid, "\"invoke_block\":0", "\"invoke_block\":1", "function-body");
  rejects_replace(valid, "\"op\":\"return\",\"value\":9", "\"op\":\"return\",\"value\":null", "function-body");
  rejects_replace(valid, "\"op\":\"return\",\"value\":null", "\"op\":\"return\",\"value\":0", "function-body");
  rejects_replace(valid, "\"op\":\"return\",\"value\":9", "\"op\":\"add\",\"value\":9", "function-body");
  rejects_replace(valid, "\"field_count\":1", "\"field_count\":2", "abi-layout");
  rejects_replace(valid, "\"storage_profile\":1,", "", "schema");
  rejects_replace(valid, "\"storage_profile\":1", "\"storage_profile\":99", "abi-layout");
  return 0;
}
