#ifndef NYRT_FAULT_V1_H
#define NYRT_FAULT_V1_H

#include <stddef.h>
#include <stdint.h>

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
uint32_t nyrt_object_reclaim_unpublished_v1(void *, uint32_t, uint64_t, int64_t,
    int64_t) __asm__("nyash.object.reclaim_unpublished_v1");
uint32_t nyrt_object_home_release_plain_i64_v1(void *, uint32_t, uint64_t, int64_t,
    int64_t) __asm__("nyash.object.home_release_plain_i64_v1");
#ifdef __cplusplus
}
#endif

#endif
