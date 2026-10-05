/* Test-only runtime class drift, without rewriting source-issued layouts. */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

int64_t real_type_id(int64_t) __asm__("__real_nyash.object.type_id_h");
int64_t wrap_type_id(int64_t) __asm__("__wrap_nyash.object.type_id_h");

int64_t wrap_type_id(int64_t handle) {
  int64_t actual = real_type_id(handle);
  if (getenv("V4_PROBE_CLASS_DRIFT")) {
    fprintf(stderr, "CLASS_DRIFT %lld\n", (long long)actual);
    return actual ^ 1;
  }
  return actual;
}
