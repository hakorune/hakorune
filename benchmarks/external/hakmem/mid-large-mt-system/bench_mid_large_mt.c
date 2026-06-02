// bench_mid_large_mt.c — Mid/Large (8–32KiB) MT allocator benchmark
// Usage: ./bench_mid_large_mt_system <threads> <cycles> <ws> <seed>
//        ./bench_mid_large_mt_hakmem <threads> <cycles> <ws> <seed>
// Prints: "Throughput = <ops/s> ops/s ..."

#include <pthread.h>
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

typedef struct {
  int ws;
  int cycles;
  uint32_t seed;
  void** slots;
} worker_t;

static inline uint64_t now_ns(void) {
  struct timespec ts;
  clock_gettime(CLOCK_MONOTONIC, &ts);
  return (uint64_t)ts.tv_sec * 1000000000ull + (uint64_t)ts.tv_nsec;
}

static inline uint32_t xorshift32(uint32_t* s) {
  uint32_t x = *s;
  x ^= x << 13;
  x ^= x >> 17;
  x ^= x << 5;
  *s = x;
  return x;
}

static void* worker_run(void* arg) {
  worker_t* w = (worker_t*)arg;
  int ws = w->ws;
  int cycles = w->cycles;
  uint32_t rnd = w->seed;
  void** slots = w->slots;

  for (int i = 0; i < cycles; i++) {
    uint32_t r = xorshift32(&rnd);
    int idx = (int)(r % (uint32_t)ws);
    if (slots[idx]) {
      free(slots[idx]);
      slots[idx] = NULL;
    } else {
      // sizes 8–32 KiB (with small fuzz)
      size_t lg = 13 + (r % 3);      // 13..15 -> 8KiB..32KiB
      size_t base = (size_t)1 << lg; // 8/16/32 KiB
      size_t add = (size_t)(r & 0x7FFu);
      size_t sz = base + add;
      void* p = malloc(sz);
      if (p) {
        ((unsigned char*)p)[0] = (unsigned char)r;
        slots[idx] = p;
      }
    }
  }

  for (int i = 0; i < ws; i++) {
    if (slots[i]) {
      free(slots[i]);
      slots[i] = NULL;
    }
  }
  return NULL;
}

int main(int argc, char** argv) {
  bench_apply_profile();

  int threads = (argc > 1) ? atoi(argv[1]) : 4;
  int cycles = (argc > 2) ? atoi(argv[2]) : 40000; // per-thread ops
  int ws = (argc > 3) ? atoi(argv[3]) : 2048;      // per-thread working set
  uint32_t seed = (argc > 4) ? (uint32_t)strtoul(argv[4], NULL, 10) : 42u;
  if (threads < 1) threads = 1;
  if (cycles < 1) cycles = 1;
  if (ws < 1) ws = 256;

  pthread_t* th = (pthread_t*)malloc(sizeof(pthread_t) * (size_t)threads);
  worker_t* wk = (worker_t*)malloc(sizeof(worker_t) * (size_t)threads);
  if (!th || !wk) {
    fprintf(stderr, "alloc failed (threads)\n");
    return 1;
  }

  uint64_t start = now_ns();
  for (int t = 0; t < threads; t++) {
    wk[t].ws = ws;
    wk[t].cycles = cycles;
    wk[t].seed = seed ^ (uint32_t)(t + 1) * 2654435761u;
    wk[t].slots = (void**)calloc((size_t)ws, sizeof(void*));
    if (!wk[t].slots) {
      fprintf(stderr, "alloc failed (slots)\n");
      return 1;
    }
    pthread_create(&th[t], NULL, worker_run, &wk[t]);
  }
  for (int t = 0; t < threads; t++) {
    pthread_join(th[t], NULL);
    free(wk[t].slots);
  }
  uint64_t end = now_ns();

  double sec = (double)(end - start) / 1e9;
  double ops_total = (double)threads * (double)cycles;
  double tput = ops_total / (sec > 0.0 ? sec : 1e-9);

  printf("Throughput = %9.0f ops/s [t=%d iter=%d ws=%d] time=%.3fs\n", tput, threads, cycles, ws, sec);

  free(th);
  free(wk);
  return 0;
}

