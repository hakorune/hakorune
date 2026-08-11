// NyRT Dynamic CallSlot v2 transport schema (wire revision 2).
//
// This header declares fixed-width wire vocabulary only.  It intentionally
// declares no runtime function and no provider/selector registry entry.

#pragma once
#include <stddef.h>
#include <stdint.h>

#define HAKO_DYNAMIC_V2_WIRE_REVISION_V2 UINT32_C(2)
#define HAKO_DYNAMIC_V2_FORWARDED_NONE_V1 UINT32_MAX

#define HAKO_DYNAMIC_V2_TAG_INVALID UINT32_C(0)
#define HAKO_DYNAMIC_V2_TAG_HOST_HANDLE UINT32_C(1)
#define HAKO_DYNAMIC_V2_TAG_IMMEDIATE_I64 UINT32_C(2)

#define HAKO_DYNAMIC_V2_STATUS_NORMAL UINT32_C(0)
#define HAKO_DYNAMIC_V2_STATUS_FAULT UINT32_C(1)
#define HAKO_DYNAMIC_V2_STATUS_SUSPENDED UINT32_C(2)

#define HAKO_DYNAMIC_V2_DISPOSITION_NONE UINT32_C(0)
#define HAKO_DYNAMIC_V2_DISPOSITION_FORWARDED UINT32_C(1)
#define HAKO_DYNAMIC_V2_DISPOSITION_END_AUTHORIZED UINT32_C(2)

#define HAKO_DYNAMIC_V2_FAULT_NONE UINT32_C(0)
#define HAKO_DYNAMIC_V2_FAULT_INVALID_RECEIVER UINT32_C(1)
#define HAKO_DYNAMIC_V2_FAULT_INVALID_HANDLE UINT32_C(2)
#define HAKO_DYNAMIC_V2_FAULT_ARITY UINT32_C(3)
#define HAKO_DYNAMIC_V2_FAULT_UNSUPPORTED_SLOT UINT32_C(4)
#define HAKO_DYNAMIC_V2_FAULT_TYPE_MISMATCH UINT32_C(5)
#define HAKO_DYNAMIC_V2_FAULT_RANGE UINT32_C(6)
#define HAKO_DYNAMIC_V2_FAULT_RUNTIME UINT32_C(7)
#define HAKO_DYNAMIC_V2_FAULT_INVALID_RESULT UINT32_C(8)

typedef struct HakoDynamicV2WireValueV1 {
    uint32_t tag;
    uint32_t reserved;
    uint64_t payload;
} HakoDynamicV2WireValueV1;

typedef struct HakoDynamicV2CallOutV1 {
    uint32_t status;
    uint32_t fault_code;
    uint32_t result_tag;
    uint32_t disposition;
    uint32_t forwarded_input;
    uint32_t reserved;
    uint64_t value_payload;
    uint64_t lease_token;
    uint64_t continuation_token;
} HakoDynamicV2CallOutV1;

#if defined(__cplusplus)
#define HAKO_DYNAMIC_V2_STATIC_ASSERT static_assert
static_assert(sizeof(HakoDynamicV2WireValueV1) == 16, "DynamicV2WireValue size");
static_assert(alignof(HakoDynamicV2WireValueV1) == 8, "DynamicV2WireValue align");
static_assert(sizeof(HakoDynamicV2CallOutV1) == 48, "DynamicV2CallOut size");
static_assert(alignof(HakoDynamicV2CallOutV1) == 8, "DynamicV2CallOut align");
#else
#define HAKO_DYNAMIC_V2_STATIC_ASSERT _Static_assert
_Static_assert(sizeof(HakoDynamicV2WireValueV1) == 16, "DynamicV2WireValue size");
_Static_assert(_Alignof(HakoDynamicV2WireValueV1) == 8, "DynamicV2WireValue align");
_Static_assert(sizeof(HakoDynamicV2CallOutV1) == 48, "DynamicV2CallOut size");
_Static_assert(_Alignof(HakoDynamicV2CallOutV1) == 8, "DynamicV2CallOut align");
#endif

HAKO_DYNAMIC_V2_STATIC_ASSERT(offsetof(HakoDynamicV2WireValueV1, tag) == 0, "wire tag offset");
HAKO_DYNAMIC_V2_STATIC_ASSERT(offsetof(HakoDynamicV2WireValueV1, payload) == 8, "wire payload offset");
HAKO_DYNAMIC_V2_STATIC_ASSERT(offsetof(HakoDynamicV2CallOutV1, status) == 0, "out status offset");
HAKO_DYNAMIC_V2_STATIC_ASSERT(offsetof(HakoDynamicV2CallOutV1, fault_code) == 4, "out fault offset");
HAKO_DYNAMIC_V2_STATIC_ASSERT(offsetof(HakoDynamicV2CallOutV1, result_tag) == 8, "out result tag offset");
HAKO_DYNAMIC_V2_STATIC_ASSERT(offsetof(HakoDynamicV2CallOutV1, disposition) == 12, "out disposition offset");
HAKO_DYNAMIC_V2_STATIC_ASSERT(offsetof(HakoDynamicV2CallOutV1, forwarded_input) == 16, "out forwarded lane offset");
HAKO_DYNAMIC_V2_STATIC_ASSERT(offsetof(HakoDynamicV2CallOutV1, reserved) == 20, "out reserved offset");
HAKO_DYNAMIC_V2_STATIC_ASSERT(offsetof(HakoDynamicV2CallOutV1, value_payload) == 24, "out value offset");
HAKO_DYNAMIC_V2_STATIC_ASSERT(offsetof(HakoDynamicV2CallOutV1, lease_token) == 32, "out lease offset");
HAKO_DYNAMIC_V2_STATIC_ASSERT(offsetof(HakoDynamicV2CallOutV1, continuation_token) == 40, "out continuation offset");
HAKO_DYNAMIC_V2_STATIC_ASSERT(HAKO_DYNAMIC_V2_WIRE_REVISION_V2 == 2, "wire revision");
HAKO_DYNAMIC_V2_STATIC_ASSERT(HAKO_DYNAMIC_V2_TAG_INVALID == 0, "invalid tag");
HAKO_DYNAMIC_V2_STATIC_ASSERT(HAKO_DYNAMIC_V2_TAG_HOST_HANDLE == 1, "host handle tag");
HAKO_DYNAMIC_V2_STATIC_ASSERT(HAKO_DYNAMIC_V2_TAG_IMMEDIATE_I64 == 2, "i64 tag");
HAKO_DYNAMIC_V2_STATIC_ASSERT(HAKO_DYNAMIC_V2_STATUS_NORMAL == 0, "normal status");
HAKO_DYNAMIC_V2_STATIC_ASSERT(HAKO_DYNAMIC_V2_STATUS_FAULT == 1, "fault status");
HAKO_DYNAMIC_V2_STATIC_ASSERT(HAKO_DYNAMIC_V2_STATUS_SUSPENDED == 2, "suspended status");
HAKO_DYNAMIC_V2_STATIC_ASSERT(HAKO_DYNAMIC_V2_DISPOSITION_NONE == 0, "none disposition");
HAKO_DYNAMIC_V2_STATIC_ASSERT(HAKO_DYNAMIC_V2_DISPOSITION_FORWARDED == 1, "forwarded disposition");
HAKO_DYNAMIC_V2_STATIC_ASSERT(HAKO_DYNAMIC_V2_DISPOSITION_END_AUTHORIZED == 2, "end disposition");
HAKO_DYNAMIC_V2_STATIC_ASSERT(HAKO_DYNAMIC_V2_FAULT_NONE == 0, "none fault");
HAKO_DYNAMIC_V2_STATIC_ASSERT(HAKO_DYNAMIC_V2_FAULT_INVALID_RECEIVER == 1, "invalid receiver fault");
HAKO_DYNAMIC_V2_STATIC_ASSERT(HAKO_DYNAMIC_V2_FAULT_INVALID_HANDLE == 2, "invalid handle fault");
HAKO_DYNAMIC_V2_STATIC_ASSERT(HAKO_DYNAMIC_V2_FAULT_ARITY == 3, "arity fault");
HAKO_DYNAMIC_V2_STATIC_ASSERT(HAKO_DYNAMIC_V2_FAULT_UNSUPPORTED_SLOT == 4, "unsupported slot fault");
HAKO_DYNAMIC_V2_STATIC_ASSERT(HAKO_DYNAMIC_V2_FAULT_TYPE_MISMATCH == 5, "type mismatch fault");
HAKO_DYNAMIC_V2_STATIC_ASSERT(HAKO_DYNAMIC_V2_FAULT_RANGE == 6, "range fault");
HAKO_DYNAMIC_V2_STATIC_ASSERT(HAKO_DYNAMIC_V2_FAULT_RUNTIME == 7, "runtime fault");
HAKO_DYNAMIC_V2_STATIC_ASSERT(HAKO_DYNAMIC_V2_FAULT_INVALID_RESULT == 8, "invalid result fault");

#undef HAKO_DYNAMIC_V2_STATIC_ASSERT
