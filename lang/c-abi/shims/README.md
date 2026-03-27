# Shims

This directory keeps C-side ABI shims thin and responsibility-partitioned.

## `hako_llvmc_ffi.c`

- Top-level owner translation unit for the `ny-llvmc` boundary bridge.
- It is the only file compiled directly by
  [`tools/build_hako_llvmc_ffi.sh`](../../../tools/build_hako_llvmc_ffi.sh).
- Its body is split into `*.inc` partitions so the route owner stays thin
  without widening the exported ABI surface.

Current partitions:

- `hako_llvmc_ffi_common.inc`
  - env, route-trace, file, and string helpers
- `hako_llvmc_ffi_string_loop_seed.inc`
  - pure-first seed emit/match helpers for loop/string hot paths
- `hako_llvmc_ffi_string_search_seed.inc`
  - pure-first seed emit/match helpers for search/index-of paths
- `hako_llvmc_ffi_indexof_observer_state.inc`
  - shared `indexOf` observer match/state helpers used by pure-first and probe lanes
- `hako_llvmc_ffi_indexof_observer_direct_match.inc`
  - direct `indexOf` observer match helpers for `select` / `branch`
- `hako_llvmc_ffi_indexof_observer_block_match.inc`
  - cross-block and interleaved `indexOf` observer match helpers
- `hako_llvmc_ffi_indexof_observer_lowering.inc`
  - `indexOf` observer defer/argument/emit helpers used by pure-first lowering
- `hako_llvmc_ffi_const_string_hoist.inc`
  - FAST-lane entry hoist for generic pure string constants so loop-local boxer churn stays out of hot blocks
- `hako_llvmc_ffi_string_concat_lowering.inc`
  - string concat chain state plus `concat_hh` / `concat3_hhh` emit helpers for generic pure lowering
- `hako_llvmc_ffi_mir_call_prepass.inc`
  - `mir_call` prepass need-flag scan helpers used before generic pure lowering emits LLVM IR
- `hako_llvmc_ffi_generic_method_lowering.inc`
  - non-`indexOf` generic method lowering helpers used by pure-first `mir_call`
- `hako_llvmc_ffi_mir_call_shell.inc`
  - shared `mir_call` emit-shell helpers for constructor/global lowering and runtime route classification
- `hako_llvmc_ffi_indexof_observer_trace.inc`
  - route-trace wrappers for `indexOf` observer families
- `hako_llvmc_ffi_route.inc`
  - harness keep replay, selected-route entry points, forwarders
- `hako_llvmc_ffi_pure_compile.inc`
  - `compile_json_compat_pure(...)`, generic walk orchestration, and the remaining exported link surface

Rules:

- Do not compile `*.inc` directly.
- Keep `hako_llvmc_ffi.c` as the single compiled owner unless there is a
  deliberate ABI/linkage split.
- Prefer adding new pure-first matcher families to a dedicated partition before
  widening the top-level owner shell.
- During backend-owner cutover, flipped boundary shapes move out of daily ownership first; delete/archive timing is tracked in the phase-29x legacy ledger.
