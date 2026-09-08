/* Independently pin the C consumer's wire constants and callable signatures. */
#include "../../../include/nyrt_fault_v1.h"
_Static_assert(NYRT_ARRAY_ELEMENT_I8_V1 == 1 && NYRT_ARRAY_ELEMENT_I16_V1 == 2 &&
    NYRT_ARRAY_ELEMENT_I32_V1 == 3 && NYRT_ARRAY_ELEMENT_I64_V1 == 4 &&
    NYRT_ARRAY_ELEMENT_U8_V1 == 5 && NYRT_ARRAY_ELEMENT_U16_V1 == 6 &&
    NYRT_ARRAY_ELEMENT_U32_V1 == 7, "Array element tags");
_Static_assert(NYRT_FAULT_REASON_ARRAY_CLAIM_CONFLICT_V1 == 200 &&
    NYRT_FAULT_REASON_ARRAY_EXISTING_ELEMENT_MISMATCH_V1 == 201 &&
    NYRT_FAULT_REASON_ARRAY_APPEND_ELEMENT_MISMATCH_V1 == 202, "Array reasons");
_Static_assert(NYRT_ARRAY_TYPE_MISMATCH_V1 == 1 &&
    NYRT_ARRAY_NEGATIVE_TO_UNSIGNED_V1 == 2 &&
    NYRT_ARRAY_OUT_OF_RANGE_V1 == 3, "Array error subtypes");
_Static_assert(_Generic(&nyrt_array_checked_new_v1,
    uint32_t (*)(void *, uint64_t, int64_t *): 1, default: 0), "new signature");
_Static_assert(_Generic(&nyrt_array_checked_claim_v1,
    uint32_t (*)(void *, uint64_t, int64_t, uint32_t): 1, default: 0), "claim signature");
_Static_assert(_Generic(&nyrt_array_checked_append_i64_v1,
    uint32_t (*)(void *, uint64_t, int64_t, int64_t): 1, default: 0), "i64 signature");
_Static_assert(_Generic(&nyrt_array_checked_append_bool_v1,
    uint32_t (*)(void *, uint64_t, int64_t, uint32_t): 1, default: 0), "Bool signature");
_Static_assert(_Generic(&nyrt_array_checked_append_f64_v1,
    uint32_t (*)(void *, uint64_t, int64_t, double): 1, default: 0), "F64 signature");
