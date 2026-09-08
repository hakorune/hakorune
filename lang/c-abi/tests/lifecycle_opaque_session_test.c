/* Validate an actual extracted descriptor against the production session owner.
 * No Map source/emission claim: this test stops at target admission. */
#include <assert.h>
#include <dlfcn.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../include/hako_llvmc_ffi.h"
static int set_err_owned(char **out, const char *text) { *out = strdup(text); return -1; }
#include "../shims/published_mir/hako_llvmc_ffi_lifecycle_target_session_v1.inc"
static uint32_t word(const unsigned char *p) {
  return (uint32_t)p[0] | (uint32_t)p[1]<<8 | (uint32_t)p[2]<<16 | (uint32_t)p[3]<<24;
}
static void rejects(hako_llvmc_lifecycle_target_session_v2 in, const char *reason) {
  hako_lts_session out = {0}; char *error = NULL;
  assert(hako_lts_open(&in, &out, &error) != 0);
  assert(error && strstr(error, reason)); free(error); hako_lts_close(&out);
}
int main(int argc, char **argv) {
  unsigned char bytes[236]; char triple[128] = {0};
  assert(argc == 2); FILE *file = fopen(argv[1], "rb"); assert(file);
  assert(fread(bytes, 1, sizeof(bytes), file) == sizeof(bytes));
  assert(fgetc(file) == EOF); fclose(file);
  assert(!memcmp(bytes, "NYRTABI2", 8) && word(bytes+12) == 2);
  uint32_t n = word(bytes+16); assert(n && n < sizeof(triple)); memcpy(triple, bytes+72, n);
  hako_llvmc_lifecycle_target_session_v2 in = {
    .revision=2, .target_triple=triple,
    .endian=word(bytes+20), .pointer_width=word(bytes+24),
    .fault_abi_version=word(bytes+28), .status_abi_version=word(bytes+32),
    .diagnostic_size=word(bytes+36), .diagnostic_align=word(bytes+40),
    .diagnostic_site_offset=word(bytes+44), .diagnostic_details_offset=word(bytes+48),
    .diagnostic_message_offset=word(bytes+52), .frame_size=word(bytes+56),
    .frame_align=word(bytes+60), .frame_primary_offset=word(bytes+64),
    .frame_suppressed_offset=word(bytes+68),
    .map_size=word(bytes+200), .map_align=word(bytes+204), .map_revision=word(bytes+208),
    .key_size=word(bytes+212), .key_align=word(bytes+216), .key_revision=word(bytes+220),
    .outcome_size=word(bytes+224), .outcome_align=word(bytes+228), .outcome_revision=word(bytes+232)
  };
  hako_lts_session session = {0}; char *error = NULL;
  assert(hako_lts_open(&in,&session,&error) == 0); assert(!error); hako_lts_close(&session);
  hako_llvmc_lifecycle_target_session_v2 bad = in; bad.revision=1; rejects(bad,"session/input");
#define CHECK(name) \
  bad=in; bad.name##_size=0; rejects(bad,"opaque-" #name); \
  bad=in; bad.name##_align=3; rejects(bad,"opaque-" #name); \
  bad=in; bad.name##_revision=0; rejects(bad,"opaque-" #name)
  CHECK(map); CHECK(key); CHECK(outcome);
  puts("opaque session: actual descriptor accepted; 10 invalid sessions rejected");
  return 0;
}
