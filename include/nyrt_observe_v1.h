#ifndef NYRT_OBSERVE_V1_H
#define NYRT_OBSERVE_V1_H
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif
/* Observation only; enabled by perf-observe + NYASH_PERF_COUNTERS.
 * Storage release counts payload drop, not registry capacity/RSS reclamation. */
enum { NYRT_OBSERVE_STORE = 0, NYRT_OBSERVE_DIRECT = 1 };
enum { NYRT_OBSERVE_ALLOC = 0, NYRT_OBSERVE_END = 1,
       NYRT_OBSERVE_FREE = 2, NYRT_OBSERVE_RECLAIM = 3 };
void nyash_observe_object_storage_event_v1(uint32_t representation, uint32_t event)
    __asm__("nyash.observe.object_storage_event_v1");
#ifdef __cplusplus
}
#endif
#endif
