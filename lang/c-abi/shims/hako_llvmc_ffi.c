// hako_llvmc_ffi.c — Minimal FFI bridge that forwards to hako_aot.c
// Exports functions that hako_aot.c dlopens when HAKO_AOT_USE_FFI=1.
// Phase 21.2 introduced a guarded "pure C-API" toggle (HAKO_CAPI_PURE=1).
// Phase 29ck now names the current compile policy through
// HAKO_BACKEND_COMPILE_RECIPE / HAKO_BACKEND_COMPAT_REPLAY, while keeping
// HAKO_CAPI_PURE as a legacy compat alias for historical pure packs.
// Supported seeds still try the pure-first boundary subset here, and
// unsupported shapes in that lane replay the explicit `--driver harness`
// keep lane directly from this shim.
// The default export surface still presents as a thin hako_aot forwarder,
// while recipe-aware callers can use an explicit pure-first export so route
// selection no longer depends on this shim's generic symbol.

#include <stddef.h>
#include <stdlib.h>
#include <string.h>
#if !defined(_WIN32)
#include <unistd.h>
#endif

// hako_aot.h provides hako_aot_compile_json / hako_aot_link_obj
#include "../include/hako_aot.h"
#include "hako_json_v1.h"
#include "yyjson.h"
#if !defined(_WIN32)
#include <dlfcn.h>
#endif

#include "hako_llvmc_ffi_common.inc"
#include "hako_llvmc_ffi_string_loop_seed.inc"
#include "hako_llvmc_ffi_string_search_seed.inc"
#include "hako_llvmc_ffi_route.inc"
#include "hako_llvmc_ffi_pure_compile.inc"
