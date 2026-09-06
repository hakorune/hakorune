#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../include/hako_llvmc_ffi.h"

static void rejects(hako_llvmc_published_lifecycle_frame_v2* frame, const char* reason) {
  char* error = NULL;
  assert(hako_llvmc_compile_published_lifecycle_v2(frame, "/tmp/hako-v2-no-object.o", &error) != 0);
  assert(error && strstr(error, reason));
  assert(fopen("/tmp/hako-v2-no-object.o", "rb") == NULL);
  free(error);
}

static void rejects_body(
    hako_llvmc_published_lifecycle_frame_v2* frame,
    hako_llvmc_published_lifecycle_body_site_v1* site,
    const char* reason) {
  char* error = NULL;
  assert(hako_llvmc_compile_published_lifecycle_body_v2(
      "published-lifecycle-body.json", frame, site, 1,
      "/tmp/hako-v2-body-no-object.o", &error) != 0);
  assert(error && strstr(error, reason));
  assert(fopen("/tmp/hako-v2-body-no-object.o", "rb") == NULL);
  free(error);
}

int main(void) {
  hako_llvmc_published_lifecycle_definition_v2 definition = {0};
  hako_llvmc_published_lifecycle_formal_v2 formal = {0};
  hako_llvmc_published_lifecycle_operation_v2 operation = {0};
  hako_llvmc_published_lifecycle_operand_v2 operand = {0};
  hako_llvmc_published_lifecycle_control_v2 control = {0};
  hako_llvmc_published_lifecycle_layout_v2 layout = {0};
  hako_llvmc_published_lifecycle_field_v2 field = {0};
  hako_llvmc_published_lifecycle_frame_v2 frame = {
    .abi_revision = HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABI_REVISION_V2,
    .storage_profile = HAKO_LLVMC_OBJECT_STORAGE_SAFE_MUTEX_V1,
    .definitions = &definition, .definition_count = 1,
    .formals = &formal, .formal_count = 1,
    .operations = &operation, .operation_count = 1,
    .operands = &operand, .operand_count = 1,
    .controls = &control, .control_count = 1,
    .layouts = &layout, .layout_count = 1,
    .fields = &field, .field_count = 1,
  };
  rejects(&frame, "consumer-pending");
  frame.storage_profile = 99;
  rejects(&frame, "storage-profile");
  frame.storage_profile = HAKO_LLVMC_OBJECT_STORAGE_SINGLE_THREAD_EXACT_V1;
  frame.operations = NULL;
  rejects(&frame, "operation-rows");
  frame.operations = &operation;
  operation.function_name = "main";
  operation.block_id = 7;
  operation.instruction_index = 3;
  operation.kind = 2;
  operation.fault_frame = 4;
  operation.normal_landing = 8;
  operation.fault_landing = 9;
  operation.object_id = 11;
  hako_llvmc_published_lifecycle_body_site_v1 site = {
    .function_name = "main", .block_id = 7, .instruction_index = 3,
    .normal_result = 12, .fault_frame = 4, .normal_landing = 8,
    .fault_landing = 9, .object_id = 11,
  };
  rejects_body(&frame, &site, "body-consumer-pending");
  site.normal_result = UINT32_MAX;
  rejects_body(&frame, &site, "body-site-invalid");
  site.normal_result = 12;
  site.block_id = 99;
  rejects_body(&frame, &site, "body-site-mismatch");
  puts("published lifecycle V2 preartifact checks passed");
  return 0;
}
