/* Physical V4 test driver: header geometry, not Rust archive admission proof. */
#include <stdio.h>
#include <stdlib.h>
#include "../include/hako_llvmc_ffi.h"
#include "../../../include/nyrt_fault_v1.h"
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
  char* error = NULL;
  if (argc != 3 && argc != 4) return 2;
  if (argc == 4) session.frame_size++; /* Deliberately malformed session. */
  int rc = hako_llvmc_compile_published_lifecycle_physical_v4(
      argv[1], &session, argv[2], &error);
  if (error) { fprintf(stderr, "%s\n", error); free(error); }
  return rc ? 1 : 0;
}
