#ifndef NYRT_TEXT_FORMAL_V1_H
#define NYRT_TEXT_FORMAL_V1_H

// Caller-zero physical Text formal wire.  The slot is never sufficient on
// its own: the generation is published by the same callable-entry cohort and
// is checked before the callee body observes the formal.

#include <stdint.h>
#include <stddef.h>

#define NYRT_TEXT_FORMAL_WIRE_REVISION_V1 UINT32_C(1)

typedef struct NyrtTextFormalBorrowV1 {
    uint64_t slot;
    uint64_t generation;
} NyrtTextFormalBorrowV1;

enum {
    NYRT_TEXT_FORMAL_VALID_V1 = 0,
    NYRT_TEXT_FORMAL_ZERO_OR_OUT_OF_RANGE_SLOT_V1 = 1,
    NYRT_TEXT_FORMAL_MISSING_SLOT_V1 = 2,
    NYRT_TEXT_FORMAL_GENERATION_MISMATCH_V1 = 3,
    NYRT_TEXT_FORMAL_NON_TEXT_PAYLOAD_V1 = 4,
};

uint32_t hako_text_formal_validate_v1(uint64_t slot, uint64_t generation);

_Static_assert(sizeof(NyrtTextFormalBorrowV1) == 16,
               "TextFormalBorrowV1 wire must remain 16 bytes");
_Static_assert(_Alignof(NyrtTextFormalBorrowV1) == 8,
               "TextFormalBorrowV1 wire alignment");
_Static_assert(offsetof(NyrtTextFormalBorrowV1, slot) == 0,
               "TextFormalBorrowV1 slot offset");
_Static_assert(offsetof(NyrtTextFormalBorrowV1, generation) == 8,
               "TextFormalBorrowV1 generation offset");

#endif // NYRT_TEXT_FORMAL_V1_H
