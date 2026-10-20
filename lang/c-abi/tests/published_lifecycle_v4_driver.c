/* Physical V4 test driver: header geometry, not Rust archive admission proof. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../include/hako_llvmc_ffi.h"
#include "../../../include/nyrt_fault_v1.h"
#ifdef HAKO_TEST_RUNTIME_DESCRIPTOR
extern const unsigned char nyash_runtime_abi_descriptor_v2[236];
static uint32_t descriptor_word(size_t offset) {
  const unsigned char* p = nyash_runtime_abi_descriptor_v2 + offset;
  return (uint32_t)p[0] | (uint32_t)p[1] << 8 | (uint32_t)p[2] << 16 | (uint32_t)p[3] << 24;
}
#endif
int main(int argc, char** argv) {
  hako_llvmc_lifecycle_target_session_v2 session = {
    .revision = 2, .target_triple = "x86_64-unknown-linux-gnu",
    .endian = 1, .pointer_width = sizeof(void *), .fault_abi_version = 1,
    .status_abi_version = 1, .diagnostic_size = sizeof(NyrtFaultDiagnosticV1),
    .diagnostic_align = _Alignof(NyrtFaultDiagnosticV1),
    .diagnostic_site_offset = offsetof(NyrtFaultDiagnosticV1, site),
    .diagnostic_details_offset = offsetof(NyrtFaultDiagnosticV1, details),
    .diagnostic_message_offset = offsetof(NyrtFaultDiagnosticV1, runtime_private_message),
    .frame_size = sizeof(NyrtFaultFrameV1), .frame_align = _Alignof(NyrtFaultFrameV1),
    .frame_primary_offset = offsetof(NyrtFaultFrameV1, primary),
    .frame_suppressed_offset = offsetof(NyrtFaultFrameV1, suppressed),
    /* Synthetic opaque geometry: this driver never proves runtime Map layout. */
    .map_size = 32, .map_align = 8, .map_revision = 1,
    .key_size = 32, .key_align = 8, .key_revision = 1,
    .outcome_size = 32, .outcome_align = 8, .outcome_revision = 1,
  };
#ifdef HAKO_TEST_RUNTIME_DESCRIPTOR
  if (memcmp(nyash_runtime_abi_descriptor_v2, "NYRTABI2", 8) ||
      descriptor_word(12) != 2 || descriptor_word(56) != session.frame_size ||
      descriptor_word(60) != session.frame_align) return 2;
#define OPAQUE_LAYOUT(name, offset) \
  session.name##_size = descriptor_word(offset); \
  session.name##_align = descriptor_word(offset + 4); \
  session.name##_revision = descriptor_word(offset + 8)
  OPAQUE_LAYOUT(map, 200);
  OPAQUE_LAYOUT(key, 212);
  OPAQUE_LAYOUT(outcome, 224);
#undef OPAQUE_LAYOUT
#endif
  hako_llvmc_lifecycle_target_session_v3 extended = { .base = session };
#ifdef HAKO_TEST_RUNTIME_DESCRIPTOR
  extended.base.revision = 3;
  const unsigned char* field = nyash_object_field_abi_descriptor_v1;
#define FIELD_WORD(index) ((uint32_t)field[4*(index)] | (uint32_t)field[4*(index)+1]<<8 | \
    (uint32_t)field[4*(index)+2]<<16 | (uint32_t)field[4*(index)+3]<<24)
  if (FIELD_WORD(0) != 32 || FIELD_WORD(1) != 1) return 2;
  extended.field = (hako_llvmc_object_field_abi_v1){
      FIELD_WORD(1), FIELD_WORD(2), FIELD_WORD(3), FIELD_WORD(4), FIELD_WORD(5), FIELD_WORD(6), FIELD_WORD(7)};
#undef FIELD_WORD
#endif
  char* error = NULL;
  if (argc != 3 && argc != 4) return 2;
  if (argc == 4) extended.base.frame_size++; /* Deliberately malformed session. */
  int rc = hako_llvmc_compile_published_lifecycle_physical_v4(
      argv[1], &extended.base, argv[2], &error);
  if (error) { fprintf(stderr, "%s\n", error); free(error); }
  return rc ? 1 : 0;
}
