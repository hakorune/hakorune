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

/* Static v2 frame consumed by the retained selected host.
 * The original body remains the only operand/CFG graph.
 * All pointers borrow caller-owned buffers for one synchronous invocation. */
#define HAKO_LLVMC_STATIC_FRAME_REVISION 2u
#define HAKO_LLVMC_MAP_OP_ALLOCATE 1u
#define HAKO_LLVMC_MAP_OP_WRITE 2u
#define HAKO_LLVMC_MAP_VALUE_I64 1u
#define HAKO_LLVMC_MAP_VALUE_BOOL 2u
#define HAKO_LLVMC_MAP_VALUE_F64 3u
#define HAKO_LLVMC_MAP_VALUE_VOID 4u
#define HAKO_LLVMC_MAP_VALUE_HANDLE 5u
#define HAKO_LLVMC_MAP_ACTION_EXACT_BITS 1u
#define HAKO_LLVMC_MAP_ACTION_ORIGINAL_VALUE 2u
#define HAKO_LLVMC_MAP_ACTION_COPY 3u
#define HAKO_LLVMC_MAP_ACTION_PHI 4u
#define HAKO_LLVMC_MAP_ACTION_SELECT 5u
#define HAKO_LLVMC_MAP_ACTION_FORMAL 6u
#define HAKO_LLVMC_MAP_ACTION_OPERATION 7u
/* Named NewBox alias of original operand0; not the Copy opcode. */
#define HAKO_LLVMC_MAP_ACTION_NAMED_ALIAS_OPERAND_ZERO 8u
/* Validated physical consumer; body still owns opcode and operands. */
#define HAKO_LLVMC_MAP_PHYSICAL_I64_BINARY 1u
#define HAKO_LLVMC_MAP_PHYSICAL_I64_COMPARE 2u
#define HAKO_LLVMC_MAP_PHYSICAL_BOOL_COMPARE 3u
#define HAKO_LLVMC_MAP_PHYSICAL_STRING_COMPARE 4u
#define HAKO_LLVMC_MAP_PHYSICAL_STRING_CONCAT 5u
#define HAKO_LLVMC_MAP_PHYSICAL_I64_NOT 6u
#define HAKO_LLVMC_MAP_PHYSICAL_BOOL_NOT 7u
#define HAKO_LLVMC_MAP_ENCODING_I64_BITS 1u
#define HAKO_LLVMC_MAP_ENCODING_BOOL_I1_ZEXT 2u
#define HAKO_LLVMC_MAP_ORIGINAL_REQUIRED 1u

typedef struct hako_llvmc_map_operation_v2 {
  const char* function_name;
  uint32_t block_id, instruction_index, kind, reserved;
} hako_llvmc_map_operation_v2;

typedef struct hako_llvmc_value_projection_v2 {
  const char* function_name;
  uint32_t value_id, action, value_kind, encoding, flags, source_ordinal, operation;
  uint64_t payload;
} hako_llvmc_value_projection_v2;

typedef struct hako_llvmc_expanded_function_v2 {
  const char* function_name;
  const char* internal_target;
} hako_llvmc_expanded_function_v2;

typedef struct hako_llvmc_published_static_frame_v2 {
  uint32_t revision, byte_size;
  const hako_llvmc_published_static_method_call_v1* calls;
  uint64_t call_count;
  const hako_llvmc_map_operation_v2* map_operations;
  uint64_t map_operation_count;
  const hako_llvmc_value_projection_v2* values;
  uint64_t value_count;
  const hako_llvmc_expanded_function_v2* expanded_functions;
  uint64_t expanded_function_count;
} hako_llvmc_published_static_frame_v2;

// Physical transport discriminators.  These values are not semantic target
// authority; they select the already-published row consumer only.
#define HAKO_LLVMC_PUBLISHED_CALL_KIND_STATIC_METHOD 1u
#define HAKO_LLVMC_PUBLISHED_CALL_KIND_BUILTIN_PRINT 2u
#define HAKO_LLVMC_PUBLISHED_CALL_KIND_FREE_FUNCTION 3u
#define HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_LITERAL_APPEND 4u
#define HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_PUSH 5u
#define HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_SET 6u
#define HAKO_LLVMC_PUBLISHED_CALL_KIND_ARRAY_INSERT 7u
#define HAKO_LLVMC_PUBLISHED_CALL_KIND_INTRINSIC_ARRAY_NEW 8u

#define HAKO_LLVMC_PUBLISHED_ROW_FLAG_DST_PRESENT 1u
#define HAKO_LLVMC_PUBLISHED_ROW_FLAG_INDEX_PRESENT 2u

/* Versioned physical compile options.  These values select tools and
 * compatibility routing only; MIR/source meaning remains in the published
 * rows and the existing C lowering owner. */
#define HAKO_LLVMC_PHYSICAL_CONTRACT_REVISION 1u
#define HAKO_LLVMC_PHYSICAL_PROFILE_GENERIC_COMPAT 0u
#define HAKO_LLVMC_PHYSICAL_PROFILE_BOUNDARY_PURE_FIRST 1u
#define HAKO_LLVMC_PHYSICAL_PROFILE_STATIC_V2 2u
#define HAKO_LLVMC_PHYSICAL_PROFILE_EXPLICIT_HARNESS 3u

typedef struct hako_llvmc_physical_contract_v1 {
  uint32_t revision;
  uint32_t byte_size;
  uint32_t ingress_profile;
  uint32_t flags;
  const char* compile_recipe;
  const char* compat_replay;
  const char* opt_level;
  const char* opt_tool_path;
  const char* llc_tool_path;
  const char* llc_flags;
  const char* llvmc_path;
} hako_llvmc_physical_contract_v1;

/* Physical-program.v2 field storage tag, independent of source value kind. */
#define HAKO_LLVMC_LIFECYCLE_STORAGE_I64 1u

/* Invocation-owned physical target facts decoded from the selected runtime
 * archive. This is not MIR/source authority and is consumed by V4. */
typedef struct hako_llvmc_lifecycle_target_session_v2 {
  uint32_t revision;
  const char* target_triple;
  uint32_t endian, pointer_width, fault_abi_version, status_abi_version;
  uint32_t diagnostic_size, diagnostic_align, diagnostic_site_offset;
  uint32_t diagnostic_details_offset, diagnostic_message_offset;
  uint32_t frame_size, frame_align, frame_primary_offset, frame_suppressed_offset;
  uint32_t map_size, map_align, map_revision;
  uint32_t key_size, key_align, key_revision;
  uint32_t outcome_size, outcome_align, outcome_revision;
} hako_llvmc_lifecycle_target_session_v2;

/* Stable physical query status/consumer vocabulary; no source admission. */
enum HakoLlvmcNamedQueryStatus {
  NAMED_QUERY_BOUND = 0,
  NAMED_QUERY_PROGRAM_UNAVAILABLE = 1,
  NAMED_QUERY_UNADDRESSABLE = 2,
  NAMED_QUERY_STORAGE_FAILED = 3,
  NAMED_QUERY_NOT_OBSERVED = 4,
};
enum NamedAllocationConsumer {
  NAMED_ALLOCATION_ARRAY = 0,
  NAMED_ALLOCATION_DIRECT_ARRAY = 1,
  NAMED_ALLOCATION_MAP = 2,
  NAMED_ALLOCATION_FILE = 3,
  NAMED_ALLOCATION_ALIAS_OPERAND_ZERO = 4,
  NAMED_ALLOCATION_TYPED_OBJECT = 5,
  NAMED_ALLOCATION_INVALID_PLAN = 6,
  NAMED_ALLOCATION_UNSUPPORTED = 7,
};

/* Retained static V2: open owns a parsed copy; query never activates rows.
 * The opaque invocation stays at its final address until close. */
typedef struct HakoLlvmcInvocation hako_llvmc_static_invocation_v2;
int hako_llvmc_compile_json_with_options_v1(
    const char* json_in, const char* obj_out,
    const hako_llvmc_physical_contract_v1* contract, char** err_out);
int hako_llvmc_static_open_v2(const char* bytes, size_t length,
    hako_llvmc_static_invocation_v2** out, char** error);
int hako_llvmc_static_open_v2_with_options(const char* bytes, size_t length,
    const hako_llvmc_physical_contract_v1* contract,
    hako_llvmc_static_invocation_v2** out, char** error);
int hako_llvmc_static_query_v2(hako_llvmc_static_invocation_v2* invocation,
    const char* function, size_t length, uint32_t block, uint32_t instruction,
    uint32_t* consumer);
int hako_llvmc_static_compile_v2(hako_llvmc_static_invocation_v2* invocation,
    const hako_llvmc_published_static_frame_v2* frame, const char* output, char** error);
void hako_llvmc_static_close_v2(hako_llvmc_static_invocation_v2* invocation);

/* Selected physical Pair consumer; accepts only physical-program.v2.
 * Borrows input/session for one synchronous invocation; publishes obj_out only after llc succeeds. No compatibility retry. */
int hako_llvmc_compile_published_lifecycle_physical_v4(
    const char* json_in, const hako_llvmc_lifecycle_target_session_v2* session,
    const char* obj_out, char** err_out);

/* Structural/SSA validation only; V4 separately checks types and cohort. */
int hako_llvmc_validate_published_lifecycle_physical_v2(
    const char* json_in, char** err_out);

#ifdef __cplusplus
}
#endif
