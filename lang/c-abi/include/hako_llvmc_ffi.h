// hako_llvmc_ffi.h — versioned typed MIR backend ingress.
//
// This boundary is deliberately smaller than the JSON compatibility entry:
// the caller supplies already-published call sites and one-way physical
// projections.  The C consumer may use each row only for that exact site; it
// must not resolve names or repair receiver operands.

#pragma once

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct hako_llvmc_published_static_method_call_v1 {
  const char* function_name;
  uint32_t block_id;
  uint32_t instruction_index;
  const char* target_symbol;
  uint32_t arity;
  uint32_t kind;
  /* ArrayElementWrite payload; zero for call rows. */
  uint32_t site_id;
  uint32_t receiver;
  uint32_t index;
  uint32_t value;
  uint32_t dst;
  uint32_t flags;
} hako_llvmc_published_static_method_call_v1;

// Physical transport discriminators.  These values are not semantic target
// authority; they select the already-published row consumer only.
#define HAKO_LLVMC_PUBLISHED_CALL_KIND_STATIC_METHOD 1u
#define HAKO_LLVMC_PUBLISHED_CALL_KIND_BUILTIN_PRINT 2u
#define HAKO_LLVMC_PUBLISHED_CALL_KIND_FREE_FUNCTION 3u
#define HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_LITERAL_APPEND 4u
#define HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_PUSH 5u
#define HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_SET 6u
#define HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_INSERT 7u

#define HAKO_LLVMC_PUBLISHED_ROW_FLAG_DST_PRESENT 1u
#define HAKO_LLVMC_PUBLISHED_ROW_FLAG_INDEX_PRESENT 2u

// V2 lifecycle transport is a separate ABI. V1 remains byte-for-byte stable.
#define HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABI_REVISION_V2 2u
#define HAKO_LLVMC_OBJECT_STORAGE_SAFE_MUTEX_V1 1u
#define HAKO_LLVMC_OBJECT_STORAGE_SINGLE_THREAD_EXACT_V1 2u

typedef struct hako_llvmc_published_lifecycle_definition_v2 {
  const char* function_name;
  const char* target_symbol;
  uint32_t role, source_arity, receiver_formal, object_id;
  uint32_t result_kind, frame_mode, flags;
} hako_llvmc_published_lifecycle_definition_v2;

typedef struct hako_llvmc_published_lifecycle_formal_v2 {
  uint32_t definition_index, source_ordinal, physical_ordinal;
  uint32_t value_id, wire_revision, input_kind;
} hako_llvmc_published_lifecycle_formal_v2;

typedef struct hako_llvmc_published_lifecycle_operation_v2 {
  const char* function_name;
  uint32_t block_id, instruction_index, kind, definition_index;
  uint32_t fault_frame, normal_landing, fault_landing;
  uint32_t object_id, field_ordinal, base, value, receiver;
  uint32_t operand_count, flags;
} hako_llvmc_published_lifecycle_operation_v2;

typedef struct hako_llvmc_published_lifecycle_operand_v2 {
  uint32_t operation_index, ordinal, value_id, kind;
} hako_llvmc_published_lifecycle_operand_v2;

typedef struct hako_llvmc_published_lifecycle_control_v2 {
  const char* function_name;
  uint32_t block_id, instruction_index, kind, operand, origin_block, mode, flags;
} hako_llvmc_published_lifecycle_control_v2;

typedef struct hako_llvmc_published_lifecycle_layout_v2 {
  uint32_t object_id, runtime_type_id, field_count, destruction_kind;
} hako_llvmc_published_lifecycle_layout_v2;

typedef struct hako_llvmc_published_lifecycle_field_v2 {
  uint32_t object_id, declaration_ordinal, runtime_slot, storage_kind;
} hako_llvmc_published_lifecycle_field_v2;

typedef struct hako_llvmc_published_lifecycle_frame_v2 {
  uint32_t abi_revision, storage_profile;
  const hako_llvmc_published_static_method_call_v1* call_rows;
  size_t call_row_count;
  const hako_llvmc_published_lifecycle_definition_v2* definitions;
  size_t definition_count;
  const hako_llvmc_published_lifecycle_formal_v2* formals;
  size_t formal_count;
  const hako_llvmc_published_lifecycle_operation_v2* operations;
  size_t operation_count;
  const hako_llvmc_published_lifecycle_operand_v2* operands;
  size_t operand_count;
  const hako_llvmc_published_lifecycle_control_v2* controls;
  size_t control_count;
  const hako_llvmc_published_lifecycle_layout_v2* layouts;
  size_t layout_count;
  const hako_llvmc_published_lifecycle_field_v2* fields;
  size_t field_count;
} hako_llvmc_published_lifecycle_frame_v2;

// Compile a module whose selected published call sites are described by the
// typed rows.  json_in remains a physical body transport for this bounded
// cohort; target identity for selected calls comes only from `calls`.
int hako_llvmc_compile_published_static_method_v1(
    const char* json_in,
    const hako_llvmc_published_static_method_call_v1* calls,
    size_t call_count,
    const char* obj_out,
    char** err_out);

int hako_llvmc_compile_published_lifecycle_v2(
    const hako_llvmc_published_lifecycle_frame_v2* frame,
    const char* obj_out,
    char** err_out);

#ifdef __cplusplus
}
#endif
