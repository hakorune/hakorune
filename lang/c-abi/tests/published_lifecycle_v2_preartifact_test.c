#define _POSIX_C_SOURCE 200809L

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../include/hako_llvmc_ffi.h"
#include "../../../include/nyrt_fault_v1.h"

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
  FILE* json = fopen("/tmp/published-lifecycle-body.json", "wb");
  assert(json);
  fputs("not-yet-consumed-body", json);
  fclose(json);
  assert(hako_llvmc_compile_published_lifecycle_body_v2(
      "/tmp/published-lifecycle-body.json", frame, site, 1,
      "/tmp/hako-v2-body-no-object.o", &error) != 0);
  assert(error && strstr(error, reason));
  assert(fopen("/tmp/hako-v2-body-no-object.o", "rb") == NULL);
  free(error);
}

static void rejects_body_v3(
    hako_llvmc_published_lifecycle_frame_v2* frame,
    hako_llvmc_published_lifecycle_body_site_v1* site,
    hako_llvmc_lifecycle_target_session_v1* session,
    const char* reason) {
  char* error = NULL;
  FILE* json = fopen("/tmp/published-lifecycle-body-v3.json", "wb");
  assert(json);
  fputs("not-yet-consumed-body", json);
  fclose(json);
  assert(hako_llvmc_compile_published_lifecycle_body_v3(
      "/tmp/published-lifecycle-body-v3.json", frame, site, 1, session,
      "/tmp/hako-v3-body-no-object.o", &error) != 0);
  assert(error && strstr(error, reason));
  assert(fopen("/tmp/hako-v3-body-no-object.o", "rb") == NULL);
  free(error);
}

int main(void) {
  hako_llvmc_published_lifecycle_definition_v2 definitions[3] = {
    { .function_name = "Pair.birth", .target_symbol = "Pair.birth/2",
      .role = HAKO_LLVMC_LIFECYCLE_DEFINITION_ROLE_BIRTH_UNIT_V2,
      .result_kind = HAKO_LLVMC_LIFECYCLE_RESULT_KIND_UNIT_V2 },
    { .function_name = "main", .target_symbol = "main",
      .role = HAKO_LLVMC_LIFECYCLE_DEFINITION_ROLE_ROOT_UNIT_V2,
      .source_arity = 0, .receiver_formal = HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
      .object_id = HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
      .result_kind = HAKO_LLVMC_LIFECYCLE_RESULT_KIND_UNIT_V2, .flags = 1 },
    { .function_name = "other", .target_symbol = "other",
      .role = HAKO_LLVMC_LIFECYCLE_DEFINITION_ROLE_ROOT_UNIT_V2,
      .source_arity = 0, .receiver_formal = HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
      .object_id = HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
      .result_kind = HAKO_LLVMC_LIFECYCLE_RESULT_KIND_UNIT_V2, .flags = 1 },
  };
  hako_llvmc_published_lifecycle_formal_v2 formal = {
    .definition_index = 0, .source_ordinal = HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
    .physical_ordinal = 0, .value_id = 1, .wire_revision = 2, .input_kind = 1,
  };
  hako_llvmc_published_lifecycle_operation_v2 operation = {
    .function_name = "main", .block_id = 7, .instruction_index = 3, .kind = 2,
    .definition_index = HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
    .fault_frame = 4, .normal_landing = 8, .fault_landing = 9, .object_id = 11,
    .field_ordinal = HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
    .base = HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
    .value = HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
    .receiver = HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
  };
  hako_llvmc_published_lifecycle_operand_v2 operand = {0};
  hako_llvmc_published_lifecycle_control_v2 control = {
    .function_name = "main", .kind = HAKO_LLVMC_LIFECYCLE_CONTROL_KIND_RETURN_V2,
    .operand = HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
    .origin_block = HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2, .mode = 0,
  };
  hako_llvmc_published_lifecycle_layout_v2 layout = {0};
  hako_llvmc_published_lifecycle_field_v2 field = {0};
  hako_llvmc_published_lifecycle_frame_v2 frame = {
    .abi_revision = HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABI_REVISION_V2,
    .storage_profile = HAKO_LLVMC_OBJECT_STORAGE_SAFE_MUTEX_V1,
    .definitions = definitions, .definition_count = 2,
    .formals = &formal, .formal_count = 1,
    .operations = &operation, .operation_count = 1,
    .operands = &operand, .operand_count = 1,
    .controls = &control, .control_count = 1,
    .layouts = &layout, .layout_count = 1,
    .fields = &field, .field_count = 1,
  };
  rejects(&frame, "consumer-pending");
  formal.source_ordinal = 0;
  rejects(&frame, "formal-receiver");
  formal.source_ordinal = HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2;
  operation.object_id = HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2;
  rejects(&frame, "operation-newbox");
  operation.object_id = 11;
  control.origin_block = 0;
  rejects(&frame, "control-return");
  control.origin_block = HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2;
  definitions[1].role = HAKO_LLVMC_LIFECYCLE_DEFINITION_ROLE_ROOT_I64_V2;
  rejects(&frame, "root-definition");
  definitions[1].result_kind = HAKO_LLVMC_LIFECYCLE_RESULT_KIND_I64_V2;
  control.mode = 1;
  control.operand = 12;
  rejects(&frame, "consumer-pending");
  definitions[1].role = HAKO_LLVMC_LIFECYCLE_DEFINITION_ROLE_ROOT_UNIT_V2;
  definitions[1].result_kind = HAKO_LLVMC_LIFECYCLE_RESULT_KIND_UNIT_V2;
  control.mode = 1;
  rejects(&frame, "root-return");
  control.mode = 0;
  control.operand = 12;
  rejects(&frame, "control-return");
  control.operand = HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2;
  frame.definition_count = 3;
  rejects(&frame, "root-definition");
  frame.definition_count = 2;
  definitions[0].role = 99;
  rejects(&frame, "formal-definition");
  definitions[0].role = HAKO_LLVMC_LIFECYCLE_DEFINITION_ROLE_BIRTH_UNIT_V2;
  frame.storage_profile = 99;
  rejects(&frame, "storage-profile");
  frame.storage_profile = HAKO_LLVMC_OBJECT_STORAGE_SINGLE_THREAD_EXACT_V1;
  frame.operations = NULL;
  rejects(&frame, "operation-rows");
  frame.operations = &operation;
  hako_llvmc_published_lifecycle_body_site_v1 site = {
    .function_name = "main", .block_id = 7, .instruction_index = 3,
    .normal_result = 12, .fault_frame = 4, .normal_landing = 8,
    .fault_landing = 9, .object_id = 11,
  };
  rejects_body(&frame, &site, "body-consumer-pending");
  hako_llvmc_lifecycle_target_session_v1 session = {
    .revision = 1, .target_triple = "x86_64-unknown-linux-gnu",
    .endian = 1, .pointer_width = sizeof(void *), .fault_abi_version = 1,
    .status_abi_version = 1, .diagnostic_size = sizeof(NyrtFaultDiagnosticV1),
    .diagnostic_align = _Alignof(NyrtFaultDiagnosticV1),
    .diagnostic_site_offset = offsetof(NyrtFaultDiagnosticV1, site),
    .diagnostic_details_offset = offsetof(NyrtFaultDiagnosticV1, details),
    .diagnostic_message_offset = offsetof(NyrtFaultDiagnosticV1, runtime_private_message),
    .frame_size = sizeof(NyrtFaultFrameV1), .frame_align = _Alignof(NyrtFaultFrameV1),
    .frame_primary_offset = offsetof(NyrtFaultFrameV1, primary),
    .frame_suppressed_offset = offsetof(NyrtFaultFrameV1, suppressed),
  };
  rejects_body_v3(&frame, &site, &session, "body-consumer-pending");
  assert(setenv("NYASH_NY_LLVM_LLC_FLAGS", "-mtriple=i386-unknown-linux-gnu", 1) == 0);
  rejects_body_v3(&frame, &site, &session, "body-consumer-pending");
  assert(unsetenv("NYASH_NY_LLVM_LLC_FLAGS") == 0);
  session.pointer_width = 4;
  rejects_body_v3(&frame, &site, &session, "lifecycle-session/runtime-abi");
  session.pointer_width = sizeof(void *);
  session.target_triple = "i386-unknown-linux-gnu";
  rejects_body_v3(&frame, &site, &session, "lifecycle-session/llvm-layout");
  session.target_triple = "x86_64-unknown-linux-gnu";
  site.normal_result = HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2;
  rejects_body(&frame, &site, "body-site-invalid");
  site.normal_result = 12;
  site.block_id = 99;
  rejects_body(&frame, &site, "body-site-mismatch");
  puts("published lifecycle V2 preartifact checks passed");
  return 0;
}
