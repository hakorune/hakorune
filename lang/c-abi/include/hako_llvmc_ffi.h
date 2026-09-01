// hako_llvmc_ffi.h — versioned typed MIR backend ingress.
//
// This boundary is deliberately smaller than the JSON compatibility entry:
// the caller supplies an already-published StaticBoxMethod call site and its
// one-way physical symbol projection.  The C consumer may use the rows only
// for that exact site; it must not resolve names or repair receiver operands.

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
} hako_llvmc_published_static_method_call_v1;

// Compile a module whose selected StaticBoxMethod call sites are described by
// the typed rows.  json_in remains a physical body transport for this bounded
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
