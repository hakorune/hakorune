// bench_random_mixed.c — Random mixed small allocations (16–1024B)
// Usage (direct-link builds via Makefile):
//   ./bench_random_mixed_hakmem [cycles] [ws] [seed]
//   ./bench_random_mixed_system [cycles] [ws] [seed]
//
// Default: 10M cycles for steady-state measurement (use 100K for quick smoke test)
// Recommended: Run 10 times and calculate mean/median/stddev for accurate results
//
// Prints: "Throughput = <ops/s> operations per second, relative time: <s>."

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <time.h>
#include <string.h>
#include <strings.h>
#include <stdatomic.h>
#include <sys/resource.h>
#include "core/bench_profile.h"

#ifdef USE_HAKMEM
#include "hakmem.h"
#include "hakmem_build_flags.h"
#include "core/box/c7_meta_used_counter_box.h"
#include "core/box/tiny_class_stats_box.h"
#include "core/box/tiny_class_policy_box.h"
#include "core/box/ss_stats_box.h"
#include "core/box/warm_pool_rel_counters_box.h"
#include "core/box/tiny_mem_stats_box.h"

// Box BenchMeta: Benchmark metadata management (bypass hakmem wrapper)
// Phase 15: Separate BenchMeta (slots array) from CoreAlloc (user workload)
extern void* __libc_calloc(size_t, size_t);
extern void __libc_free(void*);
#define BENCH_META_CALLOC __libc_calloc
#define BENCH_META_FREE __libc_free

// Phase 20-2: BenchFast mode - prealloc pool init
#include "core/box/bench_fast_box.h"
#else
// System malloc build: use standard libc
#define BENCH_META_CALLOC calloc
#define BENCH_META_FREE free
#endif

static inline uint64_t now_ns(void) {
  struct timespec ts; clock_gettime(CLOCK_MONOTONIC, &ts);
  return (uint64_t)ts.tv_sec*1000000000ull + (uint64_t)ts.tv_nsec;
}

static inline uint32_t xorshift32(uint32_t* s){
  uint32_t x=*s; x^=x<<13; x^=x>>17; x^=x<<5; *s=x; return x;
}

// Debug helper: C7 専用ベンチモード (ENV: HAKMEM_BENCH_C7_ONLY=1)
static int bench_mode_c7_only = -1;
static inline int bench_is_c7_only_mode(void) {
  if (bench_mode_c7_only == -1) {
    const char* e = getenv("HAKMEM_BENCH_C7_ONLY");
    bench_mode_c7_only = (e && *e && *e != '0') ? 1 : 0;
  }
  return bench_mode_c7_only;
}

// C5/C6 専用ベンチモード (ENV: HAKMEM_BENCH_C5_ONLY / HAKMEM_BENCH_C6_ONLY)
static int bench_mode_c5_only = -1;
static int bench_mode_c6_only = -1;
static inline int bench_is_c5_only_mode(void) {
  if (bench_mode_c5_only == -1) {
    const char* e = getenv("HAKMEM_BENCH_C5_ONLY");
    bench_mode_c5_only = (e && *e && *e != '0') ? 1 : 0;
  }
  return bench_mode_c5_only;
}
static inline int bench_is_c6_only_mode(void) {
  if (bench_mode_c6_only == -1) {
    const char* e = getenv("HAKMEM_BENCH_C6_ONLY");
    bench_mode_c6_only = (e && *e && *e != '0') ? 1 : 0;
  }
  return bench_mode_c6_only;
}

int main(int argc, char** argv){
  bench_apply_profile();

  int cycles = (argc>1)? atoi(argv[1]) : 10000000; // total ops (10M for steady-state measurement)
  int ws     = (argc>2)? atoi(argv[2]) : 8192;     // working-set slots
  uint32_t seed = (argc>3)? (uint32_t)strtoul(argv[3],NULL,10) : 1234567u;
  struct rusage ru0 = {0}, ru1 = {0};
  getrusage(RUSAGE_SELF, &ru0);

  // サイズレンジ（Tiny-only / Non-Tiny-only の比較用）
  // 既定: 16..1040 bytes（元の挙動と同等）
  size_t min_size = 16u;
  size_t max_size = 16u + 0x3FFu; // 16..1040 ≒ 16..1024

  // 優先順位: argv[4]/argv[5] → ENV → 既定
  if (argc > 4) {
    long v = atol(argv[4]);
    if (v > 0) min_size = (size_t)v;
  } else {
    const char* e = getenv("HAKMEM_BENCH_MIN_SIZE");
    if (e && *e) {
      long v = atol(e);
      if (v > 0) min_size = (size_t)v;
    }
  }

  if (argc > 5) {
    long v = atol(argv[5]);
    if (v > 0) max_size = (size_t)v;
  } else {
    const char* e = getenv("HAKMEM_BENCH_MAX_SIZE");
    if (e && *e) {
      long v = atol(e);
      if (v > 0) max_size = (size_t)v;
    }
  }

  if (min_size < 1) min_size = 1;
  if (max_size < min_size) max_size = min_size;

  // C5/C6/C7 専用モード: サイズを各クラス帯に固定
  if (bench_is_c5_only_mode()) {
    min_size = 256;
    max_size = 256;
  } else if (bench_is_c6_only_mode()) {
    min_size = 512;
    max_size = 512;
  } else if (bench_is_c7_only_mode()) {
    min_size = 1024;
    max_size = 1024;
  }

  if (cycles <= 0) cycles = 1;
  if (ws <= 0) ws = 1024;

#ifdef USE_HAKMEM
  // Phase 20-2: BenchFast prealloc pool initialization
  // Must be called BEFORE main benchmark loop to avoid recursion
  int prealloc_count = bench_fast_init();
  if (prealloc_count > 0) {
    fprintf(stderr, "[BENCH] BenchFast mode: %d blocks preallocated\n", prealloc_count);
  }
#else
  // System malloc also needs warmup for fair comparison
  (void)malloc(1); // Force libc initialization
#endif

  // Box BenchMeta: Use __libc_calloc to bypass hakmem wrapper
  void** slots = (void**)BENCH_META_CALLOC((size_t)ws, sizeof(void*));
  if (!slots) { fprintf(stderr, "alloc failed (slots)\n"); return 1; }

  // Warmup run (exclude from timing) - HAKMEM_BENCH_WARMUP=N
  const char* warmup_env = getenv("HAKMEM_BENCH_WARMUP");
  int warmup_cycles = warmup_env ? atoi(warmup_env) : 0;
  if (warmup_cycles > 0) {
    fprintf(stderr, "[BENCH_WARMUP] Running %d warmup cycles (not timed)...\n", warmup_cycles);
    uint32_t warmup_seed = seed;
    for (int i=0; i<warmup_cycles; i++){
      uint32_t r = xorshift32(&warmup_seed);
      int idx = (int)(r % (uint32_t)ws);
      if (slots[idx]){
        free(slots[idx]);
        slots[idx] = NULL;
      } else {
        size_t sz = 16u + (r & 0x3FFu);
        if (sz < min_size) sz = min_size;
        if (sz > max_size) sz = max_size;
        void* p = malloc(sz);
        if (p) {
          ((unsigned char*)p)[0] = (unsigned char)r;
          slots[idx] = p;
        }
      }
    }
    // Drain warmup allocations
    for (int i=0;i<ws;i++){ if (slots[i]) { free(slots[i]); slots[i]=NULL; } }
    fprintf(stderr, "[BENCH_WARMUP] Warmup completed. Starting timed run...\n");
  }

  // SuperSlab Prefault Phase: Pre-allocate SuperSlabs BEFORE timing starts
  // Purpose: Trigger ALL page faults during warmup (cold path) instead of during timed loop (hot path)
  // Strategy: Run warmup iterations matching the actual benchmark workload
  // Expected: This eliminates ~132K page faults from timed section -> 2-4x throughput improvement
  //
  // Key insight: Page faults occur when allocating from NEW SuperSlabs. A single pass through
  // the working set is insufficient - we need enough iterations to exhaust TLS caches and
  // force allocation of all SuperSlabs that will be used during the timed loop.
  const char* prefault_env = getenv("HAKMEM_BENCH_PREFAULT");
  int prefault_iters = prefault_env ? atoi(prefault_env) : (cycles / 10); // Default: 10% of main loop
  if (prefault_iters > 0) {
    fprintf(stderr, "[WARMUP] SuperSlab prefault: %d warmup iterations (not timed)...\n", prefault_iters);
    uint32_t warmup_seed = seed + 0xDEADBEEF; // Use DIFFERENT seed to avoid RNG sequence interference
    int warmup_allocs = 0, warmup_frees = 0;

    // Run same workload as main loop, but don't time it
    for (int i = 0; i < prefault_iters; i++) {
      uint32_t r = xorshift32(&warmup_seed);
      int idx = (int)(r % (uint32_t)ws);
      if (slots[idx]) {
        free(slots[idx]);
        slots[idx] = NULL;
        warmup_frees++;
      } else {
        size_t sz = 16u + (r & 0x3FFu); // 16..1040 bytes（後段でクランプ）
        if (sz < min_size) sz = min_size;
        if (sz > max_size) sz = max_size;
        void* p = malloc(sz);
        if (p) {
          ((unsigned char*)p)[0] = (unsigned char)r;
          slots[idx] = p;
          warmup_allocs++;
        }
      }
    }

    fprintf(stderr, "[WARMUP] Complete. Allocated=%d Freed=%d SuperSlabs populated.\n\n",
            warmup_allocs, warmup_frees);

    // Main loop will use original 'seed' variable, ensuring reproducible sequence
  }

  uint64_t start = now_ns();
  int frees = 0, allocs = 0;
  for (int i=0; i<cycles; i++){
    if (0 && (i >= 66000 || (i > 28000 && i % 1000 == 0))) {  // DISABLED for perf
      fprintf(stderr, "[TEST] Iteration %d (allocs=%d frees=%d)\n", i, allocs, frees);
    }
    uint32_t r = xorshift32(&seed);
    int idx = (int)(r % (uint32_t)ws);
    if (slots[idx]){
      if (0 && i > 28300) {  // DISABLED (Phase 2 perf)
        fprintf(stderr, "[FREE] i=%d ptr=%p idx=%d\n", i, slots[idx], idx);
        fflush(stderr);
      }
      free(slots[idx]);
      if (0 && i > 28300) {  // DISABLED (Phase 2 perf)
        fprintf(stderr, "[FREE_DONE] i=%d\n", i);
        fflush(stderr);
      }
      slots[idx] = NULL;
      frees++;
      } else {
        // 16..1024 bytes (power-of-two-ish skew, thenクランプ)
        size_t sz = 16u + (r & 0x3FFu); // 16..1040 (approx 16..1024)
        if (sz < min_size) sz = min_size;
        if (sz > max_size) sz = max_size;
      if (0 && i > 28300) {  // DISABLED (Phase 2 perf)
        fprintf(stderr, "[MALLOC] i=%d sz=%zu idx=%d\n", i, sz, idx);
        fflush(stderr);
      }
      void* p = malloc(sz);
      if (0 && i > 28300) {  // DISABLED (Phase 2 perf)
        fprintf(stderr, "[MALLOC_DONE] i=%d p=%p\n", i, p);
        fflush(stderr);
      }
      if (!p) continue;
      // touch first byte to avoid optimizer artifacts
      ((unsigned char*)p)[0] = (unsigned char)r;
      slots[idx] = p;
      allocs++;
    }
  }
  // drain
  fprintf(stderr, "[TEST] Main loop completed. Starting drain phase...\n");
  for (int i=0;i<ws;i++){ if (slots[i]) { free(slots[i]); slots[i]=NULL; } }
  fprintf(stderr, "[TEST] Drain phase completed.\n");
  uint64_t end = now_ns();
  getrusage(RUSAGE_SELF, &ru1);
  double sec = (double)(end-start)/1e9;
  double tput = (double)cycles / (sec>0.0?sec:1e-9);
  // Include params in output to avoid confusion about test conditions
  printf("Throughput = %9.0f ops/s [iter=%d ws=%d] time=%.3fs\n", tput, cycles, ws, sec);
  long rss_kb = ru1.ru_maxrss;
  fprintf(stderr, "[RSS] max_kb=%ld\n", rss_kb);
  (void)allocs; (void)frees;

  // Box BenchMeta: Use __libc_free to bypass hakmem wrapper
  BENCH_META_FREE(slots);

#ifdef USE_HAKMEM
  // Phase 20-2: Print BenchFast stats (verify pool wasn't exhausted)
  bench_fast_stats();

  // Production Performance Measurements (ENV-gated: HAKMEM_MEASURE_UNIFIED_CACHE=1)
  extern void unified_cache_print_measurements(void);
  extern void tls_sll_print_measurements(void);
  extern void shared_pool_print_measurements(void);
  unified_cache_print_measurements();
  tls_sll_print_measurements();
  shared_pool_print_measurements();

  // OBSERVE: per-class class stats (thread/global) for policy tuning
  const char* stats_dump_env = getenv("HAKMEM_TINY_STATS_DUMP");
  const char* policy_profile_env = getenv("HAKMEM_TINY_POLICY_PROFILE");
  int policy_is_auto = (policy_profile_env &&
                        strcasecmp(policy_profile_env, "auto") == 0);
  int dump_stats = (stats_dump_env && *stats_dump_env && *stats_dump_env != '0') || policy_is_auto;
  if (dump_stats) {
    tiny_class_stats_dump_thread(stderr, "[CLASS_STATS_THREAD]");
    tiny_class_stats_dump_global(stderr, "[CLASS_STATS_GLOBAL]");
  }

  const char* tiny_mem_dump_env = getenv("HAKMEM_TINY_MEM_DUMP");
  if (tiny_mem_dump_env && *tiny_mem_dump_env && *tiny_mem_dump_env != '0') {
    tiny_mem_stats_dump();
  }

  // Superslab/slab counters (ENV: HAKMEM_SS_STATS_DUMP=1)
  ss_stats_dump_if_requested();

  // Warm Pool Stats (ENV-gated: HAKMEM_WARM_POOL_STATS=1)
  extern void tiny_warm_pool_print_stats_public(void);
  tiny_warm_pool_print_stats_public();

  if (policy_is_auto) {
    tiny_class_policy_refresh_auto();
    tiny_class_policy_dump("[POLICY_AUTO]");
  }

  #if HAKMEM_BUILD_RELEASE
  // Minimal Release-side telemetry to verify Warm path usage (C7-only)
  extern _Atomic uint64_t g_rel_c7_warm_pop;
  extern _Atomic uint64_t g_rel_c7_warm_push;
  fprintf(stderr,
          "[REL_C7_CARVE] attempts=%llu success=%llu zero=%llu\n",
          (unsigned long long)warm_pool_rel_c7_carve_attempts(),
          (unsigned long long)warm_pool_rel_c7_carve_successes(),
          (unsigned long long)warm_pool_rel_c7_carve_zeroes());
  fprintf(stderr,
          "[REL_C7_WARM] pop=%llu push=%llu\n",
          (unsigned long long)atomic_load_explicit(&g_rel_c7_warm_pop, memory_order_relaxed),
          (unsigned long long)atomic_load_explicit(&g_rel_c7_warm_push, memory_order_relaxed));
  fprintf(stderr,
          "[REL_C7_WARM_PREFILL] calls=%llu slabs=%llu\n",
          (unsigned long long)warm_pool_rel_c7_prefill_calls(),
          (unsigned long long)warm_pool_rel_c7_prefill_slabs());
  fprintf(stderr,
          "[REL_C7_META_USED_INC] total=%llu backend=%llu tls=%llu front=%llu\n",
          (unsigned long long)c7_meta_used_total(),
          (unsigned long long)c7_meta_used_backend(),
          (unsigned long long)c7_meta_used_tls(),
          (unsigned long long)c7_meta_used_front());
  #else
  fprintf(stderr,
          "[DBG_C7_META_USED_INC] total=%llu backend=%llu tls=%llu front=%llu\n",
          (unsigned long long)c7_meta_used_total(),
          (unsigned long long)c7_meta_used_backend(),
          (unsigned long long)c7_meta_used_tls(),
          (unsigned long long)c7_meta_used_front());
  #endif

  // Phase 21-1: Ring cache - DELETED (A/B test: OFF is faster)
  // extern void ring_cache_print_stats(void);
  // ring_cache_print_stats();

  // Phase 27: UltraHeap front statistics (experimental, UltraHeap ビルドのみ)
  // ENV: HAKMEM_TINY_ULTRA_HEAP_DUMP=1 で出力有効化
  #if HAKMEM_TINY_ULTRA_HEAP
  {
    const char* dump = getenv("HAKMEM_TINY_ULTRA_HEAP_DUMP");
    if (dump && *dump && *dump != '0') {
      extern void tiny_ultra_heap_stats_snapshot(uint64_t hit[8],
                                                 uint64_t refill[8],
                                                 uint64_t fallback[8],
                                                 int reset);
      uint64_t hit[8] = {0}, refill[8] = {0}, fallback[8] = {0};
      tiny_ultra_heap_stats_snapshot(hit, refill, fallback, 0);
      fprintf(stderr, "[ULTRA_HEAP_STATS] class hit refill fallback\n");
      for (int c = 0; c < 8; c++) {
        if (hit[c] || refill[c] || fallback[c]) {
          fprintf(stderr, "  C%d: %llu %llu %llu\n",
                  c,
                  (unsigned long long)hit[c],
                  (unsigned long long)refill[c],
                  (unsigned long long)fallback[c]);
        }
      }
    }
  }
  #endif
#endif

  return 0;
}
