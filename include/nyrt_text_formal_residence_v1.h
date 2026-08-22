#ifndef NYRT_TEXT_FORMAL_RESIDENCE_V1_H
#define NYRT_TEXT_FORMAL_RESIDENCE_V1_H

// Private caller-zero runtime frame.  This is not the callable Text ABI and
// must not be serialized through MIR JSON or retained by the runtime.

#include "nyrt_text_formal_v1.h"
#include <stdint.h>
#include <stddef.h>

#if defined(__GNUC__) || defined(__clang__)
#define NYRT_TEXT_FORMAL_RESIDENCE_NOUNWIND_V1 __attribute__((nothrow))
#else
#define NYRT_TEXT_FORMAL_RESIDENCE_NOUNWIND_V1
#endif

#define NYRT_TEXT_FORMAL_RESIDENCE_FRAME_REVISION_V1 UINT32_C(1)
#define NYRT_TEXT_FORMAL_RESIDENCE_FRAME_HEADER_SIZE_V1 UINT32_C(32)
#define NYRT_TEXT_FORMAL_RESIDENCE_ROOT_ROW_SIZE_V1 UINT32_C(16)

typedef struct NyrtTextFormalResidenceFrameV1 {
    uint32_t revision;
    uint32_t header_size;
    uint32_t total_size;
    uint32_t root_count;
    uint64_t lease_token;
    uint64_t reserved;
} NyrtTextFormalResidenceFrameV1;

typedef struct NyrtTextFormalResidenceRootRowV1 {
    const uint8_t *ptr;
    int64_t byte_len;
} NyrtTextFormalResidenceRootRowV1;

enum {
    NYRT_TEXT_RESIDENCE_ENTER_NOUNWIND_V1 = 1,
    NYRT_TEXT_RESIDENCE_FINISH_OR_ABORT_NOUNWIND_V1 = 1,
    NYRT_TEXT_RESIDENCE_FINISH_OR_ABORT_NORETURN_V1 = 0,
    NYRT_TEXT_RESIDENCE_VALID_V1 = 0,
    NYRT_TEXT_RESIDENCE_NULL_ARGUMENT_V1 = 1,
    NYRT_TEXT_RESIDENCE_EMPTY_INPUT_V1 = 2,
    NYRT_TEXT_RESIDENCE_UNSUPPORTED_TARGET_V1 = 3,
    NYRT_TEXT_RESIDENCE_MISALIGNED_ARGUMENT_V1 = 4,
    NYRT_TEXT_RESIDENCE_PAIR_FRAME_OVERLAP_V1 = 5,
    NYRT_TEXT_RESIDENCE_FRAME_TOO_SMALL_V1 = 6,
    NYRT_TEXT_RESIDENCE_FRAME_SIZE_OVERFLOW_V1 = 7,
    NYRT_TEXT_RESIDENCE_LEASE_ZERO_OR_OUT_OF_RANGE_V1 = 16,
    NYRT_TEXT_RESIDENCE_LEASE_MISSING_SLOT_V1 = 17,
    NYRT_TEXT_RESIDENCE_LEASE_GENERATION_MISMATCH_V1 = 18,
    NYRT_TEXT_RESIDENCE_LEASE_NON_TEXT_PAYLOAD_V1 = 19,
    NYRT_TEXT_RESIDENCE_LEASE_RETIREMENT_PENDING_V1 = 20,
    NYRT_TEXT_RESIDENCE_LEASE_PIN_COUNT_OVERFLOW_V1 = 21,
    NYRT_TEXT_RESIDENCE_LEASE_BYTE_LENGTH_OUT_OF_RANGE_V1 = 22,
    NYRT_TEXT_RESIDENCE_LEASE_TOKEN_EXHAUSTED_V1 = 23,
    NYRT_TEXT_RESIDENCE_ROLLBACK_FAILED_V1 = 24,
    NYRT_TEXT_RESIDENCE_INVALID_FRAME_V1 = 32,
    NYRT_TEXT_RESIDENCE_UNKNOWN_OR_ALREADY_FINISHED_V1 = 33,
    NYRT_TEXT_RESIDENCE_FINISH_MISSING_PINNED_SLOT_V1 = 34,
    NYRT_TEXT_RESIDENCE_FINISH_GENERATION_MISMATCH_V1 = 35,
    NYRT_TEXT_RESIDENCE_FINISH_PIN_COUNT_UNDERFLOW_V1 = 36,
    NYRT_TEXT_RESIDENCE_FINISH_STATE_MISMATCH_V1 = 37,
};

uint32_t hako_text_formal_residence_enter_v1(
    const NyrtTextFormalBorrowV1 *pairs,
    uint32_t pair_count,
    NyrtTextFormalResidenceFrameV1 *frame,
    uint32_t frame_bytes) NYRT_TEXT_FORMAL_RESIDENCE_NOUNWIND_V1;

void hako_text_formal_residence_finish_or_abort_v1(
    NyrtTextFormalResidenceFrameV1 *frame)
    NYRT_TEXT_FORMAL_RESIDENCE_NOUNWIND_V1;

_Static_assert(sizeof(NyrtTextFormalResidenceFrameV1) == 32,
               "TextFormalResidence frame header must remain 32 bytes");
_Static_assert(_Alignof(NyrtTextFormalResidenceFrameV1) == 8,
               "TextFormalResidence frame header alignment");
_Static_assert(offsetof(NyrtTextFormalResidenceFrameV1, lease_token) == 16,
               "TextFormalResidence lease token offset");
_Static_assert(sizeof(NyrtTextFormalResidenceRootRowV1) == 16,
               "TextFormalResidence root row must remain 16 bytes");
_Static_assert(_Alignof(NyrtTextFormalResidenceRootRowV1) == 8,
               "TextFormalResidence root row alignment");
_Static_assert(offsetof(NyrtTextFormalResidenceRootRowV1, ptr) == 0,
               "TextFormalResidence root pointer offset");
_Static_assert(offsetof(NyrtTextFormalResidenceRootRowV1, byte_len) == 8,
               "TextFormalResidence root length offset");

#endif // NYRT_TEXT_FORMAL_RESIDENCE_V1_H
