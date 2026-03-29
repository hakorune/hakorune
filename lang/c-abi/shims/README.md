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
- `hako_llvmc_ffi_array_micro_seed.inc`
  - pure-first seed emit/match helpers for array get/set micro paths
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
- `hako_llvmc_ffi_emit_seam.inc`
  - shared emit primitives used by pure compile (`emit_branch` / `emit_ret` / `emit_call_*`)
- `hako_llvmc_ffi_compiler_state.inc`
  - shared origin / type / const / alias helper tables used by pure compile and generic method lowering
- `hako_llvmc_ffi_string_concat_lowering.inc`
  - thin wrapper for string concat lowering that now delegates emit details
- `hako_llvmc_ffi_string_concat_emit.inc`
  - string concat chain state plus `concat_hh` / `concat3_hhh` emit helpers and route-adjacent trace hooks
- `hako_llvmc_ffi_string_chain_policy.inc`
  - compiler-side mirror of `.hako` string-chain route vocabulary used by pure-first concat lowering
- `hako_llvmc_ffi_mir_call_prepass.inc`
  - `mir_call` prepass need-flag scan helpers used before generic pure lowering emits LLVM IR
- `hako_llvmc_ffi_generic_method_match.inc`
  - generic method match/plan helpers used by pure-first `mir_call`
- `hako_llvmc_ffi_generic_method_policy.inc`
  - compiler-side mirror of `.hako` generic collection/runtime method vocabulary
- `hako_llvmc_ffi_generic_method_len_policy.inc`
  - compiler-side mirror of `.hako` generic method `len` action route
- `hako_llvmc_ffi_generic_method_push_policy.inc`
  - compiler-side mirror of `.hako` generic method `push` action route
- `hako_llvmc_ffi_generic_method_has_policy.inc`
  - compiler-side mirror of `.hako` generic method `has` action route
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
- Current stage2 direction is moving owner decisions into `.hako`; treat these partitions as transitional thin shim surfaces, not permanent semantic owners.
- First code slices now extracted emit primitives into `hako_llvmc_ffi_emit_seam.inc`, and generic-method routing/classification is being split toward `hako_llvmc_ffi_generic_method_match.inc`.
- The shared compiler-state helper table is now landing in `hako_llvmc_ffi_compiler_state.inc`; keep route/placement decisions separate from raw state access.
- String concat emit helpers now live in `hako_llvmc_ffi_string_concat_emit.inc`; keep concat routing thin and avoid re-growing the wrapper body.
- `hako_llvmc_ffi_string_chain_policy.inc` is the compiler-side bridge to `lang/src/runtime/kernel/string/chain_policy.hako`; keep route names aligned and avoid reopening the `pure_compile` ladder.
- `hako_llvmc_ffi_generic_method_policy.inc` is the compiler-side bridge to `lang/src/runtime/collections/method_policy_box.hako`; keep emit-kind names aligned and avoid re-growing `generic_method_match.inc`.
- `hako_llvmc_ffi_generic_method_len_policy.inc` is the first generic-method action seam; keep `len` route ownership out of `generic_method_lowering.inc`.
- `hako_llvmc_ffi_generic_method_push_policy.inc` is the second generic-method action seam; keep `push` route ownership out of `generic_method_lowering.inc`.
- `hako_llvmc_ffi_generic_method_has_policy.inc` is the third generic-method action seam; keep `has` route ownership out of `generic_method_lowering.inc`.
