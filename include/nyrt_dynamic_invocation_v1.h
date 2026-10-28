#ifndef NYRT_DYNAMIC_INVOCATION_V1_H
#define NYRT_DYNAMIC_INVOCATION_V1_H

#include <stddef.h>
#include <stdint.h>

/* Borrowed operands for one ordinary source-bound Dynamic invocation.
 * The caller provides live aligned storage, exclusively owns the FaultFrame,
 * and keeps selector/operand bytes valid for the synchronous call. Frame,
 * descriptors, selector, and output storage must not overlap. The selector
 * and argument count come from the exact source target. */
#define NYRT_DYNAMIC_INVOCATION_TAGGED_V1 1u
#define NYRT_DYNAMIC_INVOCATION_I64_V1 2u
#define NYRT_DYNAMIC_INVOCATION_BOOL_V1 3u
#define NYRT_DYNAMIC_INVOCATION_CARRIER_V1 4u
#define NYRT_FAULT_REASON_DYNAMIC_INVOCATION_V1 400u

typedef struct NyrtDynamicInvocationOperandV1 {
    uint32_t kind;
    uint32_t tag;
    uint64_t payload;
} NyrtDynamicInvocationOperandV1;

#if defined(__cplusplus)
static_assert(sizeof(NyrtDynamicInvocationOperandV1) == 16, "Dynamic invocation operand size");
static_assert(alignof(NyrtDynamicInvocationOperandV1) == 8, "Dynamic invocation operand align");
#else
_Static_assert(sizeof(NyrtDynamicInvocationOperandV1) == 16, "Dynamic invocation operand size");
_Static_assert(_Alignof(NyrtDynamicInvocationOperandV1) == 8, "Dynamic invocation operand align");
#endif

#ifdef __cplusplus
extern "C" {
#endif

/* Inputs are borrowed and cannot escape the synchronous invocation. The
 * Normal output is one opaque carrier token; Fault leaves it untouched and
 * publishes no carrier. End consumes the exact token once. */
uint32_t nyrt_dynamic_checked_invoke_v1(void *, uint64_t,
    const NyrtDynamicInvocationOperandV1 *, const uint8_t *, size_t,
    const NyrtDynamicInvocationOperandV1 *, size_t, uint64_t *)
    __asm__("nyash.dynamic.checked_invoke_v1");
/* ArrayBox Get semantics; Normal publishes one independent opaque carrier. */
uint32_t nyrt_dynamic_checked_array_get_v1(void *, uint64_t, int64_t, int64_t, uint64_t *)
    __asm__("nyash.dynamic.checked_array_get_v1");
/* Borrow an integer payload; a different live carrier kind records Fault. */
uint32_t nyrt_dynamic_checked_integer_v1(void *, uint64_t, uint64_t, int64_t *)
    __asm__("nyash.dynamic.checked_integer_v1");
/* Borrow an integer carrier, produce a fresh carrier on Normal, and leave the
 * input alive. Fault leaves the output untouched and publishes no carrier. */
uint32_t nyrt_dynamic_checked_sub_i64_v1(void *, uint64_t, uint64_t, int64_t, uint64_t *)
    __asm__("nyash.dynamic.checked_sub_i64_v1");
/* Move one owned String pair, or the null sentinel pair (0, 0), into a fresh
 * opaque carrier. Fault leaves the pair untouched and publishes no carrier. */
uint32_t nyrt_dynamic_checked_publish_text_or_null_v1(void *, uint64_t, uint64_t, uint64_t, uint64_t *)
    __asm__("nyash.dynamic.checked_publish_text_or_null_v1");
uint32_t nyrt_dynamic_checked_end_v1(void *, uint64_t, uint64_t)
    __asm__("nyash.dynamic.checked_end_v1");

#ifdef __cplusplus
}
#endif

#endif
