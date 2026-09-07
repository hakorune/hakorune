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

/* Invocation-owned physical target facts decoded from the selected runtime
 * archive. This is not MIR/source authority and is consumed by V4. */
typedef struct hako_llvmc_lifecycle_target_session_v1 {
  uint32_t revision;
  const char* target_triple;
  uint32_t endian, pointer_width, fault_abi_version, status_abi_version;
  uint32_t diagnostic_size, diagnostic_align, diagnostic_site_offset;
  uint32_t diagnostic_details_offset, diagnostic_message_offset;
  uint32_t frame_size, frame_align, frame_primary_offset, frame_suppressed_offset;
} hako_llvmc_lifecycle_target_session_v1;

// Compile a module whose selected published call sites are described by the
// typed rows.  json_in remains a physical body transport for this bounded
// cohort; target identity for selected calls comes only from `calls`.
// Every Global Call in this published session requires its exact row. Missing
// Global rows reject before legacy plan/name dispatch; generic ingress is separate.
int hako_llvmc_compile_published_static_method_v1(
    const char* json_in,
    const hako_llvmc_published_static_method_call_v1* calls,
    size_t call_count,
    const char* obj_out,
    char** err_out);

/* Selected physical Pair consumer; accepts only physical-program.v2.
 * Borrows input/session for one synchronous invocation; publishes obj_out only after llc succeeds. No compatibility retry. */
int hako_llvmc_compile_published_lifecycle_physical_v4(
    const char* json_in, const hako_llvmc_lifecycle_target_session_v1* session,
    const char* obj_out, char** err_out);

/* Structural/SSA validation only; V4 separately checks types and cohort. */
int hako_llvmc_validate_published_lifecycle_physical_v2(
    const char* json_in, char** err_out);

#ifdef __cplusplus
}
#endif
