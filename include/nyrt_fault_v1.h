#ifndef NYRT_FAULT_V1_H
#define NYRT_FAULT_V1_H

#include <stddef.h>
#include <stdint.h>

/* Selected I64 process projection, after normal Home cleanup.
 * details = { actual signed i64, 0 }; site is issued by the physical input. */
#define NYRT_FAULT_REASON_EXIT_CODE_OUT_OF_RANGE_V1 102u
/* Exact FieldSet type failure; details = { expected kind, actual kind }. */
#define NYRT_FAULT_REASON_FIELD_TYPE_MISMATCH_V1 103u

/* Checked Array element tags; source enum discriminants are not this ABI. */
#define NYRT_ARRAY_ELEMENT_I8_V1 1u
#define NYRT_ARRAY_ELEMENT_I16_V1 2u
#define NYRT_ARRAY_ELEMENT_I32_V1 3u
#define NYRT_ARRAY_ELEMENT_I64_V1 4u
#define NYRT_ARRAY_ELEMENT_U8_V1 5u
#define NYRT_ARRAY_ELEMENT_U16_V1 6u
#define NYRT_ARRAY_ELEMENT_U32_V1 7u
/* details: conflict={requested tag,0}; adoption={index,subtype};
 * append={subtype,0}. */
#define NYRT_FAULT_REASON_ARRAY_CLAIM_CONFLICT_V1 200u
#define NYRT_FAULT_REASON_ARRAY_EXISTING_ELEMENT_MISMATCH_V1 201u
#define NYRT_FAULT_REASON_ARRAY_APPEND_ELEMENT_MISMATCH_V1 202u
#define NYRT_ARRAY_TYPE_MISMATCH_V1 1
#define NYRT_ARRAY_NEGATIVE_TO_UNSIGNED_V1 2
#define NYRT_ARRAY_OUT_OF_RANGE_V1 3

/* Internal synchronous ABI. Fields are runtime-owned, never copied/mutated by
 * generated code. Storage must be fresh, aligned and uniquely owned at init;
 * after init it remains live until dispose, including through borrowed calls.
 * Null checks do not validate dangling, misaligned or undersized pointers.
 * Diagnostic message pointers have Rust-private allocation provenance.
 * No caller may install borrowed/malloc/registry bytes in these fields. */
typedef struct NyrtFaultDiagnosticV1 {
    uint32_t reason;
    uint32_t reserved;
    uint64_t site;
    int64_t details[2];
    uint8_t *runtime_private_message;
    size_t runtime_private_message_len;
} NyrtFaultDiagnosticV1;

typedef struct NyrtFaultFrameV1 {
    uint32_t abi_version;
    uint32_t primary_present;
    uint32_t suppressed_len;
    uint32_t omitted;
    NyrtFaultDiagnosticV1 primary;
    NyrtFaultDiagnosticV1 suppressed[8];
} NyrtFaultFrameV1;

#if defined(__cplusplus)
#define NYRT_FAULT_ASSERT static_assert
#else
#define NYRT_FAULT_ASSERT _Static_assert
#endif
NYRT_FAULT_ASSERT(offsetof(NyrtFaultDiagnosticV1, site) == 8, "fault site offset");
NYRT_FAULT_ASSERT(offsetof(NyrtFaultDiagnosticV1, details) == 16, "fault detail offset");
NYRT_FAULT_ASSERT(offsetof(NyrtFaultDiagnosticV1, runtime_private_message) == 32,
                  "fault residence offset");
NYRT_FAULT_ASSERT(sizeof(NyrtFaultDiagnosticV1) == 32 + sizeof(void *) + sizeof(size_t),
                  "fault diagnostic size");
NYRT_FAULT_ASSERT(offsetof(NyrtFaultFrameV1, primary) == 16, "fault primary offset");
NYRT_FAULT_ASSERT(offsetof(NyrtFaultFrameV1, suppressed) == 16 + sizeof(NyrtFaultDiagnosticV1),
                  "fault suppressed offset");
NYRT_FAULT_ASSERT(sizeof(NyrtFaultFrameV1) == 16 + 9 * sizeof(NyrtFaultDiagnosticV1),
                  "fault frame size");
#undef NYRT_FAULT_ASSERT

enum {
    NYRT_FAULT_ABI_VERSION_V1 = 1,
    NYRT_FAULT_NORMAL_V1 = 0,
    NYRT_FAULT_FAULT_V1 = 1,
    /* Broken physical contract, NOT a source Fault or a cleanup successor. */
    NYRT_FAULT_INVALID_CONTRACT_V1 = 2
};

#ifdef __cplusplus
extern "C" {
#endif
uint32_t nyrt_fault_frame_init_v1(void *) __asm__("nyash.fault.frame_init_v1");
uint32_t nyrt_fault_record_static_v1(void *, uint32_t, uint64_t, int64_t, int64_t)
    __asm__("nyash.fault.record_static_v1");
/* Final entry reports before disposal. Disposal invalidates the frame and
 * releases each retained message once; it does not free caller storage. */
uint32_t nyrt_fault_frame_dispose_v1(void *) __asm__("nyash.fault.frame_dispose_v1");
/* Final entry only: reporting status, NOT Normal/Fault. 0=reported,
 * -1=invalid frame, -2=sink failure. Dispose remains mandatory after failure. */
int32_t nyrt_fault_report_final_v1(const void *) __asm__("nyash.fault.report_final_v1");
/* profile 1=SafeMutex, 2=SingleThreadExact. Layout tags must all be exact i64
 * (1) for this profile; count zero permits NULL. Pointers must not overlap.
 * InvalidContract never records a source Fault. Result is written only Normal.
 * No operation infers source permissions; generated callers supply published
 * definition/type/layout and destruction admission. */
uint32_t nyrt_object_checked_new_v1(void *, uint32_t, uint64_t, int64_t,
    const uint32_t *, size_t, int64_t *) __asm__("nyash.object.checked_new_v1");
uint32_t nyrt_object_checked_field_set_v1(void *, uint32_t, uint64_t, int64_t,
    int64_t, size_t, int64_t) __asm__("nyash.object.checked_field_set_v1");
/* Exact source read: Normal writes out; InvalidContract leaves it unchanged.
 * Never returns source Fault, records a diagnostic or substitutes zero. */
uint32_t nyrt_object_checked_field_get_i64_v1(uint32_t, int64_t, int64_t,
    size_t, int64_t *) __asm__("nyash.object.checked_field_get_i64_v1");
uint32_t nyrt_object_reclaim_unpublished_v1(void *, uint32_t, uint64_t, int64_t,
    int64_t) __asm__("nyash.object.reclaim_unpublished_v1");
uint32_t nyrt_object_home_release_plain_i64_v1(void *, uint32_t, uint64_t, int64_t,
    int64_t) __asm__("nyash.object.home_release_plain_i64_v1");
/* Native Array only: same live aligned exclusive frame/nonoverlapping pointer
 * contract as above. New writes its out-slot only on Normal. Bool is exactly
 * uint32 0/1; F64 is double, I64 is never a handle-or-integer carrier.
 * Returned contract failures record Fault and preserve Array state. Malformed
 * frame/tag/Bool/handle yields InvalidContract without mutation or recording.
 * Success does not clear a previous Fault. Fatal allocator failure/OS kill
 * has no returned-result or cleanup guarantee. Release uses nyrt_handle_release_h. */
uint32_t nyrt_array_checked_new_v1(void *, uint64_t, int64_t *)
    __asm__("nyash.array.checked_new_v1");
uint32_t nyrt_array_checked_claim_v1(void *, uint64_t, int64_t, uint32_t)
    __asm__("nyash.array.checked_claim_v1");
uint32_t nyrt_array_checked_append_i64_v1(void *, uint64_t, int64_t, int64_t)
    __asm__("nyash.array.checked_append_i64_v1");
uint32_t nyrt_array_checked_append_bool_v1(void *, uint64_t, int64_t, uint32_t)
    __asm__("nyash.array.checked_append_bool_v1");
uint32_t nyrt_array_checked_append_f64_v1(void *, uint64_t, int64_t, double)
    __asm__("nyash.array.checked_append_f64_v1");
/* Checked Map ABI: opaque region sizes/alignment come only from descriptor V2.
 * All regions are initialized, aligned, nonoverlapping and synchronous. Init
 * requires fresh unique bytes, not a live value. Key UTF-8 prepare precedes child
 * evaluation; a null byte pointer is permitted only at length zero. Ready key
 * may be cancelled by dispose. Live Map/Ready outcome disposal rejects.
 * End callbacks retain no mutable ABI borrows; outcomes consume before end.
 * Native key cancellation never issues source Home/finalization semantics. */
uint32_t nyrt_map_storage_init_v1(void *) __asm__("nyash.map.storage_init_v1");
uint32_t nyrt_map_checked_new_v1(void *, uint32_t, uint64_t, void *) __asm__("nyash.map.checked_new_v1");
uint32_t nyrt_map_key_init_v1(void *) __asm__("nyash.map.key_init_v1");
uint32_t nyrt_map_key_prepare_utf8_v1(void *, uint64_t, void *, const uint8_t *, size_t) __asm__("nyash.map.key_prepare_utf8_v1");
uint32_t nyrt_map_key_dispose_v1(void *) __asm__("nyash.map.key_dispose_v1");
uint32_t nyrt_map_outcome_init_v1(void *) __asm__("nyash.map.outcome_init_v1");
uint32_t nyrt_map_checked_install_indexed_v1(void *, uint32_t, uint64_t, void *, void *, int64_t, int64_t, void *) __asm__("nyash.map.checked_install_indexed_v1");
/* Checked Map value kinds. Bool payload is exactly 0 or 1; neither kind is a handle. */
#define NYRT_MAP_VALUE_I64 1u
#define NYRT_MAP_VALUE_BOOL 2u
uint32_t nyrt_map_checked_install_value_v1(void *, uint32_t, uint64_t, void *, void *, uint32_t, int64_t, void *) __asm__("nyash.map.checked_install_value_v1");
uint32_t nyrt_map_outcome_end_v1(void *, uint64_t, void *) __asm__("nyash.map.outcome_end_v1");
uint32_t nyrt_map_outcome_dispose_v1(void *) __asm__("nyash.map.outcome_dispose_v1");
uint32_t nyrt_map_checked_end_v1(void *, uint64_t, void *) __asm__("nyash.map.checked_end_v1");
uint32_t nyrt_map_storage_dispose_v1(void *) __asm__("nyash.map.storage_dispose_v1");
#ifdef __cplusplus
}
#endif

#endif
