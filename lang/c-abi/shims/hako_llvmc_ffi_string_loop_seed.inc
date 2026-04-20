#ifndef HAKO_LLVMC_STRING_LOOP_SEED_INC
#define HAKO_LLVMC_STRING_LOOP_SEED_INC

/*
 * Thin facade for string pure-first exact seed families.
 *
 * Dispatch order is semantic: shared emit helpers first, then local graph
 * helpers, then independent matcher families, and finally the substring-concat
 * bridge family that still carries the remaining legacy fallback.
 */

#include "hako_llvmc_ffi_string_loop_seed_emitters.inc"
#include "hako_llvmc_ffi_string_loop_seed_copy_graph.inc"
#include "hako_llvmc_ffi_string_loop_seed_views_only.inc"
#include "hako_llvmc_ffi_string_loop_seed_substring_concat.inc"

#endif
