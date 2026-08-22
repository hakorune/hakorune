// NyRT DynamicV2 one-shot lease End ABI (revision 1).
//
// This is a neutral C calling-convention projection.  The lease table,
// generation check, and handle release remain owned by Rust
// `runtime::dynamic_v2_lease`; this header owns only fixed-width ABI/status
// vocabulary for a statically linked Boundary caller.

#ifndef NYRT_DYNAMIC_V2_LEASE_V1_H
#define NYRT_DYNAMIC_V2_LEASE_V1_H

#include <stdint.h>

#define NYRT_DYNAMIC_V2_LEASE_ABI_REVISION_V1 UINT32_C(1)

#define NYRT_DYNAMIC_V2_LEASE_CONSUME_OK UINT32_C(0)
#define NYRT_DYNAMIC_V2_LEASE_CONSUME_INVALID_TOKEN UINT32_C(1)
#define NYRT_DYNAMIC_V2_LEASE_CONSUME_UNKNOWN_OR_ALREADY_CONSUMED UINT32_C(2)
#define NYRT_DYNAMIC_V2_LEASE_CONSUME_STALE_HANDLE_IDENTITY UINT32_C(3)

#ifdef __cplusplus
extern "C" {
#endif

uint32_t nyrt_dynamic_v2_lease_consume_end_authorized_v1(
    uint64_t lease_token);

#ifdef __cplusplus
}
#endif

#if defined(__cplusplus)
static_assert(NYRT_DYNAMIC_V2_LEASE_ABI_REVISION_V1 == 1,
              "DynamicV2 lease ABI revision");
static_assert(NYRT_DYNAMIC_V2_LEASE_CONSUME_OK == 0,
              "DynamicV2 lease OK status");
static_assert(NYRT_DYNAMIC_V2_LEASE_CONSUME_INVALID_TOKEN == 1,
              "DynamicV2 lease invalid-token status");
static_assert(NYRT_DYNAMIC_V2_LEASE_CONSUME_UNKNOWN_OR_ALREADY_CONSUMED == 2,
              "DynamicV2 lease unknown-token status");
static_assert(NYRT_DYNAMIC_V2_LEASE_CONSUME_STALE_HANDLE_IDENTITY == 3,
              "DynamicV2 lease stale-identity status");
#else
_Static_assert(NYRT_DYNAMIC_V2_LEASE_ABI_REVISION_V1 == 1,
               "DynamicV2 lease ABI revision");
_Static_assert(NYRT_DYNAMIC_V2_LEASE_CONSUME_OK == 0,
               "DynamicV2 lease OK status");
_Static_assert(NYRT_DYNAMIC_V2_LEASE_CONSUME_INVALID_TOKEN == 1,
               "DynamicV2 lease invalid-token status");
_Static_assert(NYRT_DYNAMIC_V2_LEASE_CONSUME_UNKNOWN_OR_ALREADY_CONSUMED == 2,
               "DynamicV2 lease unknown-token status");
_Static_assert(NYRT_DYNAMIC_V2_LEASE_CONSUME_STALE_HANDLE_IDENTITY == 3,
               "DynamicV2 lease stale-identity status");
#endif

#endif
