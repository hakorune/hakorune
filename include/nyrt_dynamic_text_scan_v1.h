// NyRT TextScan symbolic AOT export facts (revision 1).
//
// This header owns only neutral, pre-link export vocabulary.  It declares no
// provider, registry, function pointer, runtime address, image, or selector
// lookup.  CoreMethod result/effect rows remain the callable semantic owner.

#pragma once
#include <stdint.h>

#define HAKO_TEXT_SCAN_CONTRACT_ID "hako.text.scan@1"
#define HAKO_TEXT_SCAN_ABI_REVISION UINT32_C(1)
#define HAKO_TEXT_SCAN_PROFILE_CODEPOINT_CLAMPED UINT32_C(1)
#define HAKO_TEXT_SCAN_SUSPENSION_NON_SUSPENDING UINT32_C(0)

#define HAKO_TEXT_SCAN_ENTRY_SUBSTRING UINT32_C(1)
#define HAKO_TEXT_SCAN_ENTRY_INDEX_OF UINT32_C(2)
#define HAKO_TEXT_SCAN_ENTRY_COUNT UINT32_C(2)

#define HAKO_TEXT_SCAN_SUBSTRING_ARITY UINT32_C(2)
#define HAKO_TEXT_SCAN_INDEX_OF_ARITY UINT32_C(1)

// Physical lanes are intentionally symbolic facts; they are not a call ABI
// implementation and do not imply a resolved executable address.
#define HAKO_TEXT_SCAN_LANE_RECEIVER UINT32_C(0)
#define HAKO_TEXT_SCAN_LANE_ARG0 UINT32_C(1)
#define HAKO_TEXT_SCAN_LANE_ARG1 UINT32_C(2)

#define HAKO_TEXT_SCAN_VALUE_HOST_HANDLE UINT32_C(1)
#define HAKO_TEXT_SCAN_VALUE_IMMEDIATE_I64 UINT32_C(2)

#define HAKO_TEXT_SCAN_SUBSTRING_RECEIVER_LANE UINT32_C(1)
#define HAKO_TEXT_SCAN_INDEX_OF_RECEIVER_LANE UINT32_C(1)

#define HAKO_TEXT_SCAN_LEASE_NONE UINT32_C(0)
#define HAKO_TEXT_SCAN_LEASE_END_AUTHORIZED UINT32_C(1)

#define HAKO_TEXT_SCAN_SYMBOL_SUBSTRING "hako.text.scan.substring.v1"
#define HAKO_TEXT_SCAN_SYMBOL_INDEX_OF "hako.text.scan.index_of.v1"

#if defined(__cplusplus)
#define HAKO_TEXT_SCAN_STATIC_ASSERT static_assert
#else
#define HAKO_TEXT_SCAN_STATIC_ASSERT _Static_assert
#endif

HAKO_TEXT_SCAN_STATIC_ASSERT(HAKO_TEXT_SCAN_ENTRY_SUBSTRING == 1, "TextScan substring entry");
HAKO_TEXT_SCAN_STATIC_ASSERT(HAKO_TEXT_SCAN_ENTRY_INDEX_OF == 2, "TextScan indexOf entry");
HAKO_TEXT_SCAN_STATIC_ASSERT(HAKO_TEXT_SCAN_ENTRY_COUNT == 2, "TextScan entry count");
HAKO_TEXT_SCAN_STATIC_ASSERT(HAKO_TEXT_SCAN_LANE_RECEIVER == 0, "TextScan receiver lane");
HAKO_TEXT_SCAN_STATIC_ASSERT(HAKO_TEXT_SCAN_LANE_ARG0 == 1, "TextScan arg0 lane");
HAKO_TEXT_SCAN_STATIC_ASSERT(HAKO_TEXT_SCAN_LANE_ARG1 == 2, "TextScan arg1 lane");
HAKO_TEXT_SCAN_STATIC_ASSERT(HAKO_TEXT_SCAN_VALUE_HOST_HANDLE == 1, "TextScan host handle tag");
HAKO_TEXT_SCAN_STATIC_ASSERT(HAKO_TEXT_SCAN_VALUE_IMMEDIATE_I64 == 2, "TextScan i64 tag");
HAKO_TEXT_SCAN_STATIC_ASSERT(
    HAKO_TEXT_SCAN_SUBSTRING_RECEIVER_LANE == HAKO_TEXT_SCAN_VALUE_HOST_HANDLE,
    "TextScan substring receiver lane"
);
HAKO_TEXT_SCAN_STATIC_ASSERT(
    HAKO_TEXT_SCAN_INDEX_OF_RECEIVER_LANE == HAKO_TEXT_SCAN_VALUE_HOST_HANDLE,
    "TextScan indexOf receiver lane"
);

#undef HAKO_TEXT_SCAN_STATIC_ASSERT
