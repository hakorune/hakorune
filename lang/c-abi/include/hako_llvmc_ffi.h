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

// Compile a module whose selected published call sites are described by the
// typed rows.  json_in remains a physical body transport for this bounded
// cohort; target identity for selected calls comes only from `calls`.
int hako_llvmc_compile_published_static_method_v1(
    const char* json_in,
    const hako_llvmc_published_static_method_call_v1* calls,
    size_t call_count,
    const char* obj_out,
    char** err_out);

#ifdef __cplusplus
}
#endif
