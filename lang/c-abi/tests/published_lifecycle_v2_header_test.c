#include <assert.h>
#include <stddef.h>

#include "../include/hako_llvmc_ffi.h"

_Static_assert(sizeof(hako_llvmc_published_lifecycle_definition_v2) == 48,
               "definition row ABI drift");
_Static_assert(sizeof(hako_llvmc_published_lifecycle_formal_v2) == 24,
               "formal row ABI drift");
_Static_assert(sizeof(hako_llvmc_published_lifecycle_operation_v2) == 64,
               "operation row ABI drift");
_Static_assert(sizeof(hako_llvmc_published_lifecycle_operand_v2) == 16,
               "operand row ABI drift");
_Static_assert(sizeof(hako_llvmc_published_lifecycle_control_v2) == 40,
               "control row ABI drift");
_Static_assert(sizeof(hako_llvmc_published_lifecycle_layout_v2) == 16,
               "layout row ABI drift");
_Static_assert(sizeof(hako_llvmc_published_lifecycle_field_v2) == 16,
               "field row ABI drift");
_Static_assert(sizeof(hako_llvmc_published_lifecycle_frame_v2) == 136,
               "frame ABI drift");

int main(void) {
  hako_llvmc_published_lifecycle_frame_v2 frame = {0};
  frame.abi_revision = HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABI_REVISION_V2;
  frame.storage_profile = HAKO_LLVMC_OBJECT_STORAGE_SAFE_MUTEX_V1;
  assert(frame.abi_revision == 2u);
  assert(frame.storage_profile !=
         HAKO_LLVMC_OBJECT_STORAGE_SINGLE_THREAD_EXACT_V1);
  return 0;
}
