/* Observe actual runtime mutations. Optional test-only returned allocation Fault
 * exercises generated control; it does not simulate recovery from fatal OOM. */
#include <assert.h>
#include <stdio.h>
#include <stdint.h>
#include "../../../include/nyrt_fault_v1.h"

static int64_t handles[16];
static unsigned created, released, appends, reports;
#ifdef TEST_FAIL_NEW_AT
static unsigned new_attempts;
#define TEST_INJECTED_NEW_FAULT 9001u
#endif
int64_t nyash_array_length_h(int64_t);
int64_t nyash_array_get_h(int64_t, int64_t);
static unsigned identity(int64_t handle) {
  for (unsigned i = 0; i < created; i++) if (handles[i] == handle) return i + 1;
  assert(!"foreign handle"); return 0;
}
uint32_t real_new(void*, uint64_t, int64_t*) __asm__("__real_nyash.array.checked_new_v1");
uint32_t wrap_new(void*, uint64_t, int64_t*) __asm__("__wrap_nyash.array.checked_new_v1");
uint32_t wrap_new(void* frame, uint64_t site, int64_t* out) {
#ifdef TEST_FAIL_NEW_AT
  if (++new_attempts == TEST_FAIL_NEW_AT) {
    assert(out != NULL);
    /* Do not write the Normal-only out-slot or create a handle. */
    uint32_t status = nyrt_fault_record_static_v1(
        frame, TEST_INJECTED_NEW_FAULT, site, new_attempts, 0);
    assert(status == NYRT_FAULT_FAULT_V1);
    printf("NEW_FAULT %u\n", new_attempts);
    return status;
  }
#endif
  uint32_t status = real_new(frame, site, out);
  if (!status) { assert(created < 16); handles[created++] = *out; }
  printf("NEW %u %u\n", created, status);
  return status;
}
uint32_t real_claim(void*, uint64_t, int64_t, uint32_t) __asm__("__real_nyash.array.checked_claim_v1");
uint32_t wrap_claim(void*, uint64_t, int64_t, uint32_t) __asm__("__wrap_nyash.array.checked_claim_v1");
uint32_t wrap_claim(void* frame, uint64_t site, int64_t handle, uint32_t tag) {
  uint32_t status = real_claim(frame, site, handle, tag);
  printf("CLAIM %u %u %u\n", identity(handle), tag, status);
  return status;
}
static void append_result(int64_t handle, const char* kind, int64_t before, uint32_t status) {
  int64_t after = nyash_array_length_h(handle);
  appends++;
  assert(after == before + (status == 0));
  printf("APPEND %u %s %u %lld %lld\n", identity(handle), kind, status,
      (long long)before, (long long)after);
}
uint32_t real_i64(void*, uint64_t, int64_t, int64_t) __asm__("__real_nyash.array.checked_append_i64_v1");
uint32_t wrap_i64(void*, uint64_t, int64_t, int64_t) __asm__("__wrap_nyash.array.checked_append_i64_v1");
uint32_t wrap_i64(void* f, uint64_t s, int64_t h, int64_t v) {
  int64_t before = nyash_array_length_h(h);
  uint32_t status = real_i64(f, s, h, v);
  append_result(h, "i64", before, status); return status;
}
uint32_t real_bool(void*, uint64_t, int64_t, uint32_t) __asm__("__real_nyash.array.checked_append_bool_v1");
uint32_t wrap_bool(void*, uint64_t, int64_t, uint32_t) __asm__("__wrap_nyash.array.checked_append_bool_v1");
uint32_t wrap_bool(void* f, uint64_t s, int64_t h, uint32_t v) {
  int64_t before = nyash_array_length_h(h);
  uint32_t status = real_bool(f, s, h, v);
  append_result(h, "bool", before, status); return status;
}
uint32_t real_f64(void*, uint64_t, int64_t, double) __asm__("__real_nyash.array.checked_append_f64_v1");
uint32_t wrap_f64(void*, uint64_t, int64_t, double) __asm__("__wrap_nyash.array.checked_append_f64_v1");
uint32_t wrap_f64(void* f, uint64_t s, int64_t h, double v) {
  int64_t before = nyash_array_length_h(h);
  uint32_t status = real_f64(f, s, h, v);
  append_result(h, "f64", before, status); return status;
}
void __real_nyrt_handle_release_h(int64_t);
void __wrap_nyrt_handle_release_h(int64_t handle) {
  unsigned id = identity(handle);
  int64_t length = nyash_array_length_h(handle);
  printf("RELEASE %u %lld %lld\n", id, (long long)length,
      (long long)(length ? nyash_array_get_h(handle, 0) : 0));
  assert(released < created && id == created - released);
  released++;
  __real_nyrt_handle_release_h(handle);
}
int32_t real_report(const void*) __asm__("__real_nyash.fault.report_final_v1");
int32_t wrap_report(const void*) __asm__("__wrap_nyash.fault.report_final_v1");
int32_t wrap_report(const void* frame) {
  const NyrtFaultFrameV1* f = frame;
  assert(released == created && reports++ == 0);
  printf("FAULT %u %lld %lld\n", f->primary.reason,
      (long long)f->primary.details[0], (long long)f->primary.details[1]);
  return real_report(frame);
}
uint32_t real_dispose(void*) __asm__("__real_nyash.fault.frame_dispose_v1");
uint32_t wrap_dispose(void*) __asm__("__wrap_nyash.fault.frame_dispose_v1");
uint32_t wrap_dispose(void* frame) {
  assert(released == created);
  printf("DISPOSE %u %u %u %u\n", created, appends, released, reports);
  return real_dispose(frame);
}
