// bench_tiny_hot.c — Tiny hot-path microbench
// Usage: ./bench_tiny_hot_system [size] [batch] [cycles]
//        ./bench_tiny_hot_hakmem [size] [batch] [cycles]
// Prints: "Throughput = <ops/s> ops/s ..."

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#include "core/bench_profile.h"

#ifdef USE_HAKMEM
#include "hakmem.h"
#endif

#ifdef USE_MIMALLOC
#include "mimalloc-bench/extern/mi/include/mimalloc.h"
#define malloc  mi_malloc
#define free    mi_free
#define calloc  mi_calloc
#define realloc mi_realloc
#endif

static inline uint64_t now_ns(void) {
  struct timespec ts;
  clock_gettime(CLOCK_MONOTONIC, &ts);
  return (uint64_t)ts.tv_sec * 1000000000ull + (uint64_t)ts.tv_nsec;
}

int main(int argc, char** argv) {
  bench_apply_profile();

  size_t size = 64;
  int batch = 100;
  int cycles = 100000;

  if (argc >= 2) size = (size_t)strtoull(argv[1], NULL, 10);
  if (argc >= 3) batch = atoi(argv[2]);
  if (argc >= 4) cycles = atoi(argv[3]);

  if (batch <= 0) batch = 100;
  if (cycles <= 0) cycles = 100000;

  void** ptrs = (void**)malloc(sizeof(void*) * (size_t)batch);
  if (!ptrs) {
    fprintf(stderr, "alloc ptrs failed\n");
    return 1;
  }

  uint64_t start = now_ns();
  for (int c = 0; c < cycles; c++) {
    for (int i = 0; i < batch; i++) {
      ptrs[i] = malloc(size);
    }
    for (int i = batch - 1; i >= 0; i--) {
      free(ptrs[i]);
    }
  }
  uint64_t end = now_ns();

  double sec = (double)(end - start) / 1e9;
  double ops_total = (double)cycles * (double)batch * 2.0;
  double tput = ops_total / (sec > 0.0 ? sec : 1e-9);

  printf("Throughput = %9.0f ops/s [size=%zu batch=%d cycles=%d] time=%.3fs\n", tput, size, batch, cycles, sec);

  free(ptrs);
  return 0;
}

