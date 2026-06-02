/* Minimal CRT-build stub for bench_mixed_ws.c.
 *
 * The vendored mixed working-set fixture is built with HZ3_BENCH_USE_CRT=1 so
 * the benchmark calls libc malloc/realloc/free and can be driven by LD_PRELOAD.
 * No hz3 allocator symbols are referenced in this mode.
 */
