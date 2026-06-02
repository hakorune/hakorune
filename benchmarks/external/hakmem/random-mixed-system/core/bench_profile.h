#pragma once
#include <dlfcn.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <unistd.h>

#ifdef USE_HAKMEM
#include "box/wrapper_env_box.h"  // wrapper_env_refresh_from_env (Phase 2 B4)
#include "box/tiny_static_route_box.h"  // tiny_static_route_refresh_from_env (Phase 3 C3)
#include "box/hakmem_env_snapshot_box.h"  // hakmem_env_snapshot_refresh_from_env (Phase 4 E1)
#include "box/tiny_free_route_cache_env_box.h"  // tiny_free_static_route_refresh_from_env (Phase 8)
#include "box/tiny_c7_preserve_header_env_box.h"  // tiny_c7_preserve_header_env_refresh_from_env (Phase 13 v1)
#include "box/tiny_tcache_env_box.h"  // tiny_tcache_env_refresh_from_env (Phase 14 v1)
#include "box/tiny_unified_lifo_env_box.h"  // tiny_unified_lifo_env_refresh_from_env (Phase 15 v1)
#include "box/front_fastlane_alloc_legacy_direct_env_box.h"  // front_fastlane_alloc_legacy_direct_env_refresh_from_env (Phase 16 v1)
#include "box/fastlane_direct_env_box.h"  // fastlane_direct_env_refresh_from_env (Phase 19-1)
#include "box/tiny_header_hotfull_env_box.h"  // tiny_header_hotfull_env_refresh_from_env (Phase 21)
#include "box/tiny_inline_slots_fixed_mode_box.h"  // tiny_inline_slots_fixed_mode_refresh_from_env (Phase 78-1)
#include "box/tiny_inline_slots_switch_dispatch_fixed_box.h"  // tiny_inline_slots_switch_dispatch_fixed_refresh_from_env (Phase 80-*)
#include "box/free_path_commit_once_fixed_box.h"  // free_path_commit_once_refresh_from_env (Phase 85)
#include "box/free_path_legacy_mask_box.h"  // free_path_legacy_mask_refresh_from_env (Phase 86)
#include "box/tiny_c6_inline_slots_ifl_env_box.h"  // tiny_c6_inline_slots_ifl_refresh_from_env (Phase 91)
#include "box/bench_env_mutation_box.h"  // Bench guard: avoid getenv reentrancy during putenv
#include "box/tiny_env_box.h"  // tiny_env_refresh (Phase 96-ROBUST-4)
#endif

// env が未設定のときだけ既定値を入れる
static inline void bench_setenv_default(const char* key, const char* val) {
  if (getenv(key) != NULL) return;
  static void* (*real_malloc)(size_t) = NULL;
  static int (*real_putenv)(char*) = NULL;
  if (!real_malloc) {
    real_malloc = (void* (*)(size_t))dlsym(RTLD_NEXT, "malloc");
    if (!real_malloc) real_malloc = malloc;
  }
  if (!real_putenv) {
    real_putenv = (int (*)(char*))dlsym(RTLD_NEXT, "putenv");
    if (!real_putenv) real_putenv = putenv;
  }
  size_t klen = strlen(key);
  size_t vlen = strlen(val);
  char* buf = (char*)real_malloc(klen + vlen + 2);
  if (!buf) return;
  memcpy(buf, key, klen);
  buf[klen] = '=';
  memcpy(buf + klen + 1, val, vlen);
  buf[klen + 1 + vlen] = '\0';
  {
    char msg[256];
    int n = snprintf(msg, sizeof(msg), "[bench_profile] set %s=%s\n", key, val);
    if (n > 0) {
      if (n > (int)sizeof(msg)) n = (int)sizeof(msg);
      ssize_t w = write(2, msg, (size_t)n);
      (void)w;
    }
  }
  // Guard: during putenv(), glibc may call malloc/free while environ is mid-update.
  // If hakmem wrappers call getenv() in that window, getenv can crash.
  // We mark the window and force wrappers to bypass to libc malloc/free.
#ifdef USE_HAKMEM
  bench_env_mutation_enter();
#endif
  real_putenv(buf);  // takes ownership; do not free
#ifdef USE_HAKMEM
  bench_env_mutation_exit();
#endif
}

// ベンチ専用: HAKMEM_PROFILE に応じて ENV をプリセットする
static inline void bench_apply_mixed_tinyv3_c7_common(void) {
  bench_setenv_default("HAKMEM_TINY_HEAP_PROFILE", "C7_SAFE");
  bench_setenv_default("HAKMEM_TINY_C7_HOT", "1");
  bench_setenv_default("HAKMEM_TINY_HOTHEAP_V2", "0");
  bench_setenv_default("HAKMEM_SMALL_HEAP_V3_ENABLED", "1");
  bench_setenv_default("HAKMEM_SMALL_HEAP_V3_CLASSES", "0x80");
  bench_setenv_default("HAKMEM_SMALL_HEAP_V4_ENABLED", "0");
  bench_setenv_default("HAKMEM_SMALL_HEAP_V4_CLASSES", "0x0");
  bench_setenv_default("HAKMEM_TINY_PTR_FAST_CLASSIFY_V4_ENABLED", "0");
  bench_setenv_default("HAKMEM_SMALL_SEGMENT_V4_ENABLED", "0");
  bench_setenv_default("HAKMEM_POOL_V2_ENABLED", "0");
  bench_setenv_default("HAKMEM_TINY_FRONT_V3_ENABLED", "1");
  bench_setenv_default("HAKMEM_TINY_FRONT_V3_LUT_ENABLED", "1");
  bench_setenv_default("HAKMEM_TINY_PTR_FAST_CLASSIFY_ENABLED", "1");
  // Phase FREE-TINY-FAST-DUALHOT-1: C0-C3 direct fast free (skip policy snapshot)
  bench_setenv_default("HAKMEM_FREE_TINY_FAST_HOTCOLD", "1");
  // Phase 2 B4: Wrapper hot/cold split (malloc/free wrapper shape)
  bench_setenv_default("HAKMEM_WRAP_SHAPE", "1");
  // Phase 4 E1: ENV Snapshot Consolidation (+3.92% proven on Mixed)
  bench_setenv_default("HAKMEM_ENV_SNAPSHOT", "1");
  // Phase 5 E4-1: Free wrapper ENV snapshot (+3.51% proven on Mixed, 10-run)
  bench_setenv_default("HAKMEM_FREE_WRAPPER_ENV_SNAPSHOT", "1");
  // Phase 5 E4-2: Malloc wrapper ENV snapshot (+21.83% proven on Mixed, 10-run)
  bench_setenv_default("HAKMEM_MALLOC_WRAPPER_ENV_SNAPSHOT", "1");
  // Phase 5 E5-1: Free Tiny Direct Path (+3.35% proven on Mixed, 10-run)
  bench_setenv_default("HAKMEM_FREE_TINY_DIRECT", "1");
  // Phase 6-1: Front FastLane (Layer Collapse) (+11.13% proven on Mixed, 10-run)
  bench_setenv_default("HAKMEM_FRONT_FASTLANE", "1");
  // Phase 6-2: Front FastLane Free DeDup (+5.18% proven on Mixed, 10-run)
  bench_setenv_default("HAKMEM_FRONT_FASTLANE_FREE_DEDUP", "1");
  // Phase 21: Tiny Header HotFull (alloc header hot/cold split; opt-out with 0)
  bench_setenv_default("HAKMEM_TINY_HEADER_HOTFULL", "1");
  // Phase 19-1b: FastLane Direct (wrapper layer bypass, +5.88% proven on Mixed, 10-run)
  bench_setenv_default("HAKMEM_FASTLANE_DIRECT", "1");
  // Phase 9: FREE-TINY-FAST MONO DUALHOT (+2.72% proven on Mixed, 10-run)
  bench_setenv_default("HAKMEM_FREE_TINY_FAST_MONO_DUALHOT", "1");
  // Phase 10: FREE-TINY-FAST MONO LEGACY DIRECT (+1.89% proven on Mixed, 10-run)
  bench_setenv_default("HAKMEM_FREE_TINY_FAST_MONO_LEGACY_DIRECT", "1");
  // Phase 4-4: C6 ULTRA free+alloc 統合を有効化 (default OFF, manual opt-in)
  bench_setenv_default("HAKMEM_TINY_C6_ULTRA_FREE_ENABLED", "0");
  // Phase MID-V3: Mid/Pool HotBox v3
  // Mixed (16–1024B) では MID_V3(C6) が大きく遅くなるため、デフォルト OFF に固定。
  // C6-heavy プロファイル側でのみ ON を推奨する（C6-heavy のみ最適化対象）。
  bench_setenv_default("HAKMEM_MID_V3_ENABLED", "0");
  bench_setenv_default("HAKMEM_MID_V3_CLASSES", "0x0");
  // Phase 2 B3: Routing branch shape optimization (LIKELY on LEGACY, cold helper for rare routes)
  bench_setenv_default("HAKMEM_TINY_ALLOC_ROUTE_SHAPE", "1");
  // Phase 3 C3: Static routing (policy_snapshot bypass, +2.2% proven)
  bench_setenv_default("HAKMEM_TINY_STATIC_ROUTE", "1");
  // Phase 3 D1: Free route cache (TLS cache for free path routing, +2.19% proven)
  bench_setenv_default("HAKMEM_FREE_STATIC_ROUTE", "1");
  // Phase 69-1: Warm Pool Size=16 (+3.26% Strong GO, ENV-only)
  bench_setenv_default("HAKMEM_WARM_POOL_SIZE", "16");
  // Phase 75-3: C5+C6 Inline Slots (GO +5.41% proven, 4-point matrix A/B)
  bench_setenv_default("HAKMEM_TINY_C5_INLINE_SLOTS", "1");
  bench_setenv_default("HAKMEM_TINY_C6_INLINE_SLOTS", "1");
  // Phase 76-1: C4 Inline Slots (GO +1.73%, 10-run A/B)
  bench_setenv_default("HAKMEM_TINY_C4_INLINE_SLOTS", "1");
  // Phase 78-1: Inline Slots Fixed Mode (GO, removes per-op ENV gate overhead)
  bench_setenv_default("HAKMEM_TINY_INLINE_SLOTS_FIXED", "1");
  // Phase 80-1: Inline Slots Switch Dispatch (GO +1.65%, removes if-chain comparisons)
  bench_setenv_default("HAKMEM_TINY_INLINE_SLOTS_SWITCHDISPATCH", "1");
  // Phase 83-1: Switch Dispatch Fixed Mode (removes per-op ENV gate, bench-only)
  bench_setenv_default("HAKMEM_TINY_INLINE_SLOTS_SWITCHDISPATCH_FIXED", "1");
  // Phase 24: C7 ULTRA Header Light (eliminates per-alloc write_header call)
  bench_setenv_default("HAKMEM_TINY_C7_ULTRA_HEADER_LIGHT", "1");
}

static inline void bench_apply_profile(void) {
  const char* p = getenv("HAKMEM_PROFILE");
  if (!p || !*p) return;

	if (strcmp(p, "MIXED_TINYV3_C7_SAFE") == 0) {
    // Speed-first default (Phase 57): do not set HAKMEM_SS_MEM_LEAN here.
    bench_apply_mixed_tinyv3_c7_common();
  } else if (strcmp(p, "MIXED_TINYV3_C7_BALANCED") == 0) {
    // Balanced mode (Phase 55/56): LEAN+OFF (prewarm suppression only).
    bench_apply_mixed_tinyv3_c7_common();
    bench_setenv_default("HAKMEM_SS_MEM_LEAN", "1");
    bench_setenv_default("HAKMEM_SS_MEM_LEAN_DECOMMIT", "OFF");
    bench_setenv_default("HAKMEM_SS_MEM_LEAN_TARGET_MB", "10");
  } else if (strcmp(p, "C6_HEAVY_LEGACY_POOLV1") == 0) {
    bench_setenv_default("HAKMEM_TINY_HEAP_PROFILE", "C7_SAFE");
    bench_setenv_default("HAKMEM_TINY_C6_HOT", "0");
    bench_setenv_default("HAKMEM_TINY_HOTHEAP_V2", "0");
    bench_setenv_default("HAKMEM_SMALL_HEAP_V3_ENABLED", "1");
    bench_setenv_default("HAKMEM_SMALL_HEAP_V3_CLASSES", "0x80");
    bench_setenv_default("HAKMEM_POOL_V2_ENABLED", "0");
    bench_setenv_default("HAKMEM_POOL_V1_FLATTEN_ENABLED", "0");
    bench_setenv_default("HAKMEM_MID_DESC_CACHE_ENABLED", "1");
    // Phase 4-4: C6 ULTRA free+alloc 統合を有効化 (default OFF, manual opt-in)
    bench_setenv_default("HAKMEM_TINY_C6_ULTRA_FREE_ENABLED", "0");
    // Phase MID-V3: Mid/Pool HotBox v3 (257-768B, C6 only)
    bench_setenv_default("HAKMEM_MID_V3_ENABLED", "1");
    bench_setenv_default("HAKMEM_MID_V3_CLASSES", "0x40");
	    // Phase 6-1: Front FastLane (Layer Collapse) (+11.13% proven on Mixed, 10-run)
	    bench_setenv_default("HAKMEM_FRONT_FASTLANE", "1");
	    // Phase 6-2: Front FastLane Free DeDup (+5.18% proven on Mixed, 10-run)
	    bench_setenv_default("HAKMEM_FRONT_FASTLANE_FREE_DEDUP", "1");
	    // Phase 21: Tiny Header HotFull (alloc header hot/cold split; opt-out with 0)
	    bench_setenv_default("HAKMEM_TINY_HEADER_HOTFULL", "1");
	    // Phase 19-1b: FastLane Direct (wrapper layer bypass)
	    bench_setenv_default("HAKMEM_FASTLANE_DIRECT", "1");
	    // Phase 2 B3: Routing branch shape optimization (LIKELY on LEGACY, cold helper for rare routes)
	    bench_setenv_default("HAKMEM_TINY_ALLOC_ROUTE_SHAPE", "1");
  } else if (strcmp(p, "C6_V7_STUB") == 0) {
    // Phase v7-1: C6-only v7 stub 実験用（MID v3 fallback）
    bench_setenv_default("HAKMEM_TINY_HEAP_PROFILE", "C7_SAFE");
    bench_setenv_default("HAKMEM_TINY_C6_HOT", "0");
    bench_setenv_default("HAKMEM_TINY_HOTHEAP_V2", "0");
    bench_setenv_default("HAKMEM_SMALL_HEAP_V3_ENABLED", "1");
    bench_setenv_default("HAKMEM_SMALL_HEAP_V3_CLASSES", "0x80");
    bench_setenv_default("HAKMEM_POOL_V2_ENABLED", "0");
    bench_setenv_default("HAKMEM_MID_V3_ENABLED", "1");
    bench_setenv_default("HAKMEM_MID_V3_CLASSES", "0x40");
    // v7 stub ON (C6-only)
    bench_setenv_default("HAKMEM_SMALL_HEAP_V7_ENABLED", "1");
    bench_setenv_default("HAKMEM_SMALL_HEAP_V7_CLASSES", "0x40");
  } else if (strcmp(p, "C6_HEAVY_LEGACY_POOLV1_FLATTEN") == 0) {
    // LEGACY mid/smallmid ベンチ専用（C7_SAFE では使用しない）
    bench_setenv_default("HAKMEM_TINY_HEAP_PROFILE", "LEGACY");
    bench_setenv_default("HAKMEM_TINY_C6_HOT", "0");
    bench_setenv_default("HAKMEM_TINY_HOTHEAP_V2", "0");
    bench_setenv_default("HAKMEM_SMALL_HEAP_V3_ENABLED", "1");
    bench_setenv_default("HAKMEM_SMALL_HEAP_V3_CLASSES", "0x80");
    bench_setenv_default("HAKMEM_POOL_V2_ENABLED", "0");
    bench_setenv_default("HAKMEM_POOL_V1_FLATTEN_ENABLED", "1");
    bench_setenv_default("HAKMEM_POOL_V1_FLATTEN_STATS", "1");
    bench_setenv_default("HAKMEM_POOL_ZERO_MODE", "header");
  } else if (strcmp(p, "DEBUG_TINY_FRONT_PERF") == 0) {
    bench_setenv_default("HAKMEM_TINY_HEAP_PROFILE", "C7_SAFE");
    bench_setenv_default("HAKMEM_TINY_C7_HOT", "1");
    bench_setenv_default("HAKMEM_TINY_HOTHEAP_V2", "0");
    bench_setenv_default("HAKMEM_SMALL_HEAP_V3_ENABLED", "1");
    bench_setenv_default("HAKMEM_SMALL_HEAP_V3_CLASSES", "0x80");
    bench_setenv_default("HAKMEM_POOL_V2_ENABLED", "0");
    bench_setenv_default("HAKMEM_TINY_FRONT_V3_ENABLED", "1");
    bench_setenv_default("HAKMEM_TINY_FRONT_V3_LUT_ENABLED", "1");
    bench_setenv_default("HAKMEM_TINY_PTR_FAST_CLASSIFY_ENABLED", "1");
  } else if (strcmp(p, "C6_SMALL_HEAP_V3_EXPERIMENT") == 0) {
    // C6 を SmallObject v3 に載せる研究用（標準では使用しない）
    bench_setenv_default("HAKMEM_TINY_HEAP_PROFILE", "C7_SAFE");
    bench_setenv_default("HAKMEM_TINY_C6_HOT", "1");
    bench_setenv_default("HAKMEM_TINY_HOTHEAP_V2", "0");
    bench_setenv_default("HAKMEM_SMALL_HEAP_V3_ENABLED", "1");
    bench_setenv_default("HAKMEM_SMALL_HEAP_V3_CLASSES", "0x40"); // C6 only
    bench_setenv_default("HAKMEM_SMALL_HEAP_V4_ENABLED", "0");
    bench_setenv_default("HAKMEM_SMALL_HEAP_V4_CLASSES", "0x0");
    bench_setenv_default("HAKMEM_POOL_V2_ENABLED", "0");
	  } else if (strcmp(p, "C6_SMALL_HEAP_V4_EXPERIMENT") == 0) {
    // C6 を SmallObject v4 に載せる研究用（標準では使用しない）
    bench_setenv_default("HAKMEM_TINY_HEAP_PROFILE", "C7_SAFE");
    bench_setenv_default("HAKMEM_TINY_C6_HOT", "1");
    bench_setenv_default("HAKMEM_TINY_HOTHEAP_V2", "0");
    bench_setenv_default("HAKMEM_SMALL_HEAP_V3_ENABLED", "0");
    bench_setenv_default("HAKMEM_SMALL_HEAP_V3_CLASSES", "0x0");
    bench_setenv_default("HAKMEM_SMALL_HEAP_V4_ENABLED", "1");
    bench_setenv_default("HAKMEM_SMALL_HEAP_V4_CLASSES", "0x40"); // C6 only
	    bench_setenv_default("HAKMEM_POOL_V2_ENABLED", "0");
	  }

// Phase 96-ROBUST-5: LD_PRELOAD compatibility - dlsym-based refresh for all ENV caches
  // When benchmarking via LD_PRELOAD (bench_random_mixed_system + libhakmem.so),
  // the benchmark binary is compiled WITHOUT -DUSE_HAKMEM, but we still need to
  // refresh hakmem's ENV caches after bench_setenv_default() sets the variables.
  //
  // Solution: Use dlsym() to conditionally call refresh functions if they exist.
  // This allows bench_apply_profile() to work correctly in BOTH modes:
  // 1. Native hakmem binary (USE_HAKMEM=1): Direct calls via #ifdef block below
  // 2. LD_PRELOAD mode (!USE_HAKMEM): dlsym() finds symbols in libhakmem.so
#ifndef USE_HAKMEM
  {
    // Try to refresh hakmem ENV caches via dlsym (LD_PRELOAD mode)
    void (*refresh_fn)(void);
    #define TRY_REFRESH(name) do { \
      refresh_fn = (void (*)(void))dlsym(RTLD_DEFAULT, #name); \
      if (refresh_fn) refresh_fn(); \
    } while (0)

    TRY_REFRESH(small_policy_v7_bump_version);
    TRY_REFRESH(wrapper_env_refresh_from_env);
    TRY_REFRESH(tiny_static_route_refresh_from_env);
    TRY_REFRESH(hakmem_env_snapshot_refresh_from_env);
    TRY_REFRESH(tiny_free_static_route_refresh_from_env);
    TRY_REFRESH(tiny_c7_preserve_header_env_refresh_from_env);
    TRY_REFRESH(tiny_tcache_env_refresh_from_env);
    TRY_REFRESH(tiny_unified_lifo_env_refresh_from_env);
    TRY_REFRESH(front_fastlane_alloc_legacy_direct_env_refresh_from_env);
    TRY_REFRESH(fastlane_direct_env_refresh_from_env);
    TRY_REFRESH(tiny_header_hotfull_env_refresh_from_env);
    TRY_REFRESH(tiny_inline_slots_fixed_mode_refresh_from_env);
    TRY_REFRESH(tiny_inline_slots_switch_dispatch_fixed_refresh_from_env);
    TRY_REFRESH(free_path_commit_once_refresh_from_env);
    TRY_REFRESH(free_path_legacy_mask_refresh_from_env);
    TRY_REFRESH(tiny_c6_inline_slots_ifl_refresh_from_env);
    TRY_REFRESH(tiny_env_refresh);

    #undef TRY_REFRESH
  }
#else
	  // Phase 3 C3 Step 0: Ensure policy snapshot reflects final ENV after putenv defaults.
	  small_policy_v7_bump_version();
	  // Phase 2 B4: Sync wrapper ENV cache after bench_profile putenv defaults.
	  wrapper_env_refresh_from_env();
	  // Phase 3 C3: Sync static route cache after bench_profile putenv defaults.
	  tiny_static_route_refresh_from_env();
	  // Phase 4 E1: Sync ENV snapshot cache after bench_profile putenv defaults.
	  hakmem_env_snapshot_refresh_from_env();
	  // Phase 8: Sync free static route ENV cache after bench_profile putenv defaults.
	  tiny_free_static_route_refresh_from_env();
	  // Phase 13 v1: Sync C7 preserve header ENV cache after bench_profile putenv defaults.
	  tiny_c7_preserve_header_env_refresh_from_env();
	  // Phase 14 v1: Sync tcache ENV cache after bench_profile putenv defaults.
	  tiny_tcache_env_refresh_from_env();
	  // Phase 15 v1: Sync LIFO ENV cache after bench_profile putenv defaults.
	  tiny_unified_lifo_env_refresh_from_env();
	  // Phase 16 v1: Sync LEGACY direct ENV cache after bench_profile putenv defaults.
	  front_fastlane_alloc_legacy_direct_env_refresh_from_env();
	  // Phase 19-1: Sync FastLane Direct ENV cache after bench_profile putenv defaults.
		  fastlane_direct_env_refresh_from_env();
		  // Phase 21: Sync Tiny Header HotFull ENV cache after bench_profile putenv defaults.
		  tiny_header_hotfull_env_refresh_from_env();
		  // Phase 78-1: Optionally pin C3/C4/C5/C6 inline-slots modes (avoid per-op ENV gates).
		  tiny_inline_slots_fixed_mode_refresh_from_env();
		  // Phase 83-1: Optionally pin switch dispatch mode (avoid per-op ENV gate for dispatch).
		  tiny_inline_slots_switch_dispatch_fixed_refresh_from_env();
		  // Phase 85: Optionally commit-once for C4-C7 LEGACY free path (skip policy/route/mono ceremony).
		  free_path_commit_once_refresh_from_env();
		  // Phase 86: Optionally use legacy mask for early exit (no indirect calls, just bit test).
		  free_path_legacy_mask_refresh_from_env();
		  // Phase 91: C6 intrusive LIFO inline slots (per-class LIFO transformation).
		  tiny_c6_inline_slots_ifl_refresh_from_env();
		  // Phase 96-ROBUST-4: Refresh inline slots ENV snapshot (fix lazy cache race).
		  tiny_env_refresh();
#endif
		}
