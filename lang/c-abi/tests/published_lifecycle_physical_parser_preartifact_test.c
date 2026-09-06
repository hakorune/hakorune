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

int main(void) {
  const char* valid =
      "{\"schema\":\"hako.published-lifecycle-physical-program.v1\","
      "\"fault_abi_version\":1,\"functions\":["
      "{\"role\":\"root_i64\",\"params\":[],\"blocks\":[{}]},"
      "{\"role\":\"birth_unit\",\"params\":[0],\"blocks\":[{}]}],"
      "\"layouts\":[{\"field_count\":2,\"fields\":[{},{}]}]}";
  accepts(valid);
  rejects("{}", "schema");
  rejects("{\"schema\":\"hako.published-lifecycle-physical-program.v1\",\"fault_abi_version\":1,\"functions\":[{\"role\":\"birth_unit\",\"params\":[],\"blocks\":[{}]},{\"role\":\"birth_unit\",\"params\":[],\"blocks\":[{}]}],\"layouts\":[{\"field_count\":0,\"fields\":[]}]}", "function-role");
  rejects("{\"schema\":\"hako.published-lifecycle-physical-program.v1\",\"fault_abi_version\":1,\"functions\":[{\"role\":\"root_i64\",\"params\":[],\"blocks\":[{}]},{\"role\":\"birth_unit\",\"params\":[],\"blocks\":[{}]}],\"layouts\":[{\"field_count\":2,\"fields\":[{}]}]}", "layout");
  rejects("{\"schema\":\"hako.published-lifecycle-physical-program.v1\",\"fault_abi_version\":9,\"functions\":[{\"role\":\"root_i64\",\"params\":[],\"blocks\":[{}]},{\"role\":\"birth_unit\",\"params\":[],\"blocks\":[{}]}],\"layouts\":[{\"field_count\":0,\"fields\":[]}]}", "abi");
  return 0;
}
