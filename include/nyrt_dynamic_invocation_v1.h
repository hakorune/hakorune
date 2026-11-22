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
#define NYRT_DYNAMIC_TAG_TYPED_STORE_BORROW_V1 5u

typedef struct NyrtDynamicInvocationOperandV1 {
    uint32_t kind;
    uint32_t tag;
    uint64_t payload;
} NyrtDynamicInvocationOperandV1;

/* Only an address-significant, process-lifetime static image may issue a
 * program descriptor. Typed-store payloads require an allocation stamped
 * with the exact descriptor address. All lengths count elements, not bytes. */
typedef uint32_t (*NyrtDispatchThunkV1)(void *, int64_t,
    const NyrtDynamicInvocationOperandV1 *, size_t, int64_t *);

typedef struct NyrtDispatchTypeV1 {
    int64_t type_id;
    const uint32_t *layout;
    size_t layout_count;
} NyrtDispatchTypeV1;

typedef struct NyrtDispatchProviderV1 {
    int64_t type_id;
    const uint8_t *selector;
    size_t selector_len;
    size_t arity;
    const uint32_t *parameters; /* 0 = Dynamic, 1 = declared I64. */
    NyrtDispatchThunkV1 thunk;
} NyrtDispatchProviderV1;

typedef struct NyrtDispatchProgramV1 {
    uint32_t version;
    uint32_t reserved;
    const NyrtDispatchTypeV1 *types;
    size_t type_count;
    const NyrtDispatchProviderV1 *rows;
    size_t row_count;
} NyrtDispatchProgramV1;

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
/* Program-aware checked call and allocation. Fault preserves the out-slot.
 * Tagged argument 4 carries the address of an existing aligned borrowed Text
 * descriptor. Its storage and underlying Text must remain live throughout
 * the synchronous call. Tag 5 borrows a live same-program SafeMutex object.
 * The thunk runs after releasing the store lock and retains no borrowed input. */
uint32_t nyrt_dynamic_checked_invoke_program_v1(void *, uint64_t,
    const NyrtDispatchProgramV1 *, const NyrtDynamicInvocationOperandV1 *,
    const uint8_t *, size_t, const NyrtDynamicInvocationOperandV1 *, size_t,
    uint64_t *) __asm__("nyash.dynamic.checked_invoke_program_v1");
uint32_t nyrt_object_checked_new_in_program_v1(void *, uint32_t, uint64_t,
    const NyrtDispatchProgramV1 *, int64_t, const uint32_t *, size_t, int64_t *)
    __asm__("nyash.object.checked_new_in_program_v1");
/* ArrayBox Get semantics; Normal publishes one independent opaque carrier. */
uint32_t nyrt_dynamic_checked_array_get_v1(void *, uint64_t, int64_t, int64_t, uint64_t *)
    __asm__("nyash.dynamic.checked_array_get_v1");
/* Borrow an integer payload; a different live carrier kind records Fault. */
uint32_t nyrt_dynamic_checked_integer_v1(void *, uint64_t, uint64_t, int64_t *)
    __asm__("nyash.dynamic.checked_integer_v1");
/* Copy one admitted borrowed scalar or Text and publish a token only on Normal. */
uint32_t nyrt_dynamic_checked_materialize_borrowed_v1(
    void *, uint64_t, uint32_t, uint64_t, uint64_t *)
    __asm__("nyash.dynamic.checked_materialize_borrowed_v1");
/* Read the original borrowed formal without registering or consuming a token. */
uint32_t nyrt_dynamic_checked_borrowed_integer_v1(
    void *, uint64_t, uint32_t, uint64_t, int64_t *)
    __asm__("nyash.dynamic.checked_borrowed_integer_v1");
/* Borrow an integer carrier, produce a fresh carrier on Normal, and leave the
 * input alive. Fault leaves the output untouched and publishes no carrier. */
uint32_t nyrt_dynamic_checked_sub_i64_v1(void *, uint64_t, uint64_t, int64_t, uint64_t *)
    __asm__("nyash.dynamic.checked_sub_i64_v1");
/* Move one owned String pair, or the null sentinel pair (0, 0), into a fresh
 * opaque carrier. Fault leaves the pair untouched and publishes no carrier. */
uint32_t nyrt_dynamic_checked_publish_text_or_null_v1(void *, uint64_t, uint64_t, uint64_t, uint64_t *)
    __asm__("nyash.dynamic.checked_publish_text_or_null_v1");
/* Borrow a carrier and report whether it holds null (1) or not (0). */
uint32_t nyrt_dynamic_checked_is_null_v1(void *, uint64_t, uint64_t, int64_t *)
    __asm__("nyash.dynamic.checked_is_null_v1");
uint32_t nyrt_dynamic_checked_end_v1(void *, uint64_t, uint64_t)
    __asm__("nyash.dynamic.checked_end_v1");

#ifdef __cplusplus
}
#endif

#endif
