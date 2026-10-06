/* Minimal runtime entry for static-v2 execution fixtures whose generated
 * `ny_main` returns a raw i64 result.  The suite kernel archive supplies
 * no `main` (lifecycle-core build); the trap-observation
 * `static_v2_runtime_probe.c` entry is specialized to its own asserts, so
 * plain value-returning fixtures use this entry instead. */
#include <stdint.h>

extern int64_t ny_main(void);

int main(void) { return (int)ny_main(); }
