# Shims

This directory keeps C-side ABI shims thin and responsibility-partitioned.

## Responsibility Boundary

- `.inc` files consume MIR-owned metadata and emit backend calls.
- `.inc` files may perform backend-local operand normalization and variant selection only after MIR has already decided legality.
- `.inc` files must not become semantic planners for publication defer, provenance, StableView legality, or read-side alias continuation.
- Temporary exact matchers must be listed in the phase-137x Legacy Retirement Ledger or moved to explicit legacy fixtures before they can be kept.
- Active seam closeout SSOT: `../../../docs/development/current/main/phases/phase-137x/137x-95-mir-backend-seam-closeout-before-textlane.md`.

## `hako_llvmc_ffi.c`

- Top-level owner translation unit for the `ny-llvmc` boundary bridge.
- It is the only file compiled directly by
  [`tools/build_hako_llvmc_ffi.sh`](../../../tools/build_hako_llvmc_ffi.sh).
- Its body is split into `*.inc` partitions so the route owner stays thin
  without widening the exported ABI surface.

Current partitions:

- `hako_llvmc_ffi_common.inc`
  - env, route-trace, file, and generic shared helpers
- `hako_llvmc_ffi_string_metadata_fn_readers.inc`
  - function-level string relation/plan readers used by exact seed bridges
  - keeps string metadata JSON access out of the generic `common` owner
- `hako_llvmc_ffi_string_candidate_plan_readers.inc`
  - register-rooted string candidate/plan readers used by legacy bridge and concat-policy consumers
  - keeps candidate-plan JSON access out of `string_chain_terms.inc`
- `hako_llvmc_ffi_string_loop_seed.inc`
  - thin facade include for loop/string hot-path seed families
- `hako_llvmc_ffi_string_loop_seed_emitters.inc`
  - exact LLVM IR emitters shared by the loop/string pure-first seed families
- `hako_llvmc_ffi_string_loop_seed_copy_graph.inc`
  - copy-chain/copy-graph helper layer shared by the exact string seed matchers
- `hako_llvmc_ffi_string_loop_seed_views_only.inc`
  - views-only exact matcher family
- `hako_llvmc_ffi_string_loop_seed_length_hot_loop.inc`
  - string-length hot-loop exact matcher family
- `hako_llvmc_ffi_string_loop_seed_substring_concat.inc`
  - substring-concat exact matcher family, including the remaining metadata-first + shape-fallback bridge
- `hako_llvmc_ffi_array_string_store_seed.inc`
  - pure-first seed emit/match helpers for the exact array/string-store micro path
  - temporary bridge surface only; removal/shrink gate is tracked in the phase-137x Legacy Retirement Ledger
- `hako_llvmc_ffi_string_search_seed.inc`
  - pure-first seed emit/match helpers for search/index-of paths
- `hako_llvmc_ffi_array_micro_seed.inc`
  - pure-first seed emit/match helpers for array get/set micro paths
- `hako_llvmc_ffi_user_box_micro_seed.inc`
  - pure-first seed emit/match helpers for the narrow typed user-box point-add / flag-toggle micro paths
  - now partitioned further into `hako_llvmc_ffi_user_box_micro_seed_helpers.inc` plus typed family slices
- `hako_llvmc_ffi_sum_local_seed.inc`
  - thin facade include for local variant/sum pure-first seeds
- `hako_llvmc_ffi_sum_local_seed_metadata_helpers.inc`
  - shared metadata readers for thin-entry and sum-placement local aggregate checks
- `hako_llvmc_ffi_sum_local_seed_emitters.inc`
  - exact local variant tag/project pure IR emitters
- `hako_llvmc_ffi_sum_local_seed_matchers_tag.inc`
  - local/tag matcher family
- `hako_llvmc_ffi_sum_local_seed_matchers_project_copy.inc`
  - project-through-copy matcher family
- `hako_llvmc_ffi_sum_local_seed_matchers_project_local.inc`
  - direct local project matcher family
- `hako_llvmc_ffi_concat_const_suffix_seed.inc`
  - dedicated exact-micro pure-first seed for `kilo_micro_concat_const_suffix`
- `hako_llvmc_ffi_indexof_observer_state.inc`
  - shared `indexOf` observer match/state helpers used by pure-first and probe lanes
- `hako_llvmc_ffi_indexof_observer_direct_match.inc`
  - direct `indexOf` observer match helpers for `select` / `branch`
- `hako_llvmc_ffi_indexof_observer_block_match.inc`
  - cross-block and interleaved `indexOf` observer match helpers
  - now partitioned further into `hako_llvmc_ffi_indexof_observer_block_match_cross.inc`, `hako_llvmc_ffi_indexof_observer_block_match_branch.inc`, and `hako_llvmc_ffi_indexof_observer_block_match_select.inc`
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
  - now partitioned further into `hako_llvmc_ffi_string_concat_emit_helpers.inc` and `hako_llvmc_ffi_string_concat_emit_routes.inc`
- `hako_llvmc_ffi_concat_hh_len_seed.inc`
  - dedicated exact-micro pure-first seed for `kilo_micro_concat_hh_len`
- `hako_llvmc_ffi_string_chain_terms.inc`
  - shared string-chain enum/name terms used by policy and producer-window seams
- `hako_llvmc_ffi_string_chain_policy.inc`
  - compiler-side mirror of `.hako` string-chain route / retained-form / post-store observer vocabulary used by pure-first concat lowering
- `hako_llvmc_ffi_mir_call_route_policy.inc`
  - compiler-side mirror of `.hako` generic `mir_call` receiver-family route vocabulary
- `hako_llvmc_ffi_mir_call_need_policy.inc`
  - compiler-side mirror of `.hako` `mir_call` prepass need-vocabulary
- `hako_llvmc_ffi_mir_call_surface_policy.inc`
  - compiler-side mirror of `.hako` constructor/global/string-extern accept surfaces
- `hako_llvmc_ffi_mir_call_prepass.inc`
  - `mir_call` prepass scan/mutation helpers that consume the `.hako` need-vocabulary before generic pure lowering emits LLVM IR
- `hako_llvmc_ffi_mir_call_dispatch.inc`
  - native `mir_call` dispatcher seam that consumes route/surface policy mirrors plus generic method lowering
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
- `hako_llvmc_ffi_generic_method_substring_policy.inc`
  - compiler-side mirror of `.hako` generic method `substring` action route
- `hako_llvmc_ffi_generic_method_get_policy.inc`
  - compiler-side mirror of `.hako` generic method `get` fallback route
- `hako_llvmc_ffi_generic_method_get_window.inc`
  - compiler-state-heavy `GET` window helper bundle for producer-side probe logic
- `hako_llvmc_ffi_generic_method_get_lowering.inc`
  - thin `GET` dispatcher seam that consumes `get_policy` and `get_window`
- `hako_llvmc_ffi_string_concat_window.inc`
  - producer-window helper seam for `string_concat_match.inc`
- `hako_llvmc_ffi_generic_method_lowering.inc`
  - non-`indexOf` generic method lowering helpers used by pure-first `mir_call`
- `hako_llvmc_ffi_mir_call_shell.inc`
  - shared `mir_call` emit-shell helpers for constructor/global/string-extern lowering
- `hako_llvmc_ffi_indexof_observer_trace.inc`
  - route-trace wrappers for `indexOf` observer families
- `hako_llvmc_ffi_route.inc`
  - harness keep replay, selected-route entry points, forwarders
- `hako_llvmc_ffi_pure_compile.inc`
  - `compile_json_compat_pure(...)`, generic walk orchestration, and the remaining exported link surface
  - now partitioned further into `hako_llvmc_ffi_pure_compile_generic_lowering.inc` and `hako_llvmc_ffi_pure_compile_minimal_paths.inc`

Rules:

- Do not compile `*.inc` directly.
- Keep `hako_llvmc_ffi.c` as the single compiled owner unless there is a
  deliberate ABI/linkage split.
- Do not treat `*.inc` as long-term semantic-owner nouns.
- Future boundary truth is owned by `hako.abi` / `hako.value_repr` / ABI or ownership-layout manifests; shims are thin emitted or partitioned forms.
- Prefer adding new pure-first matcher families to a dedicated partition before
  widening the top-level owner shell.
- During backend-owner cutover, flipped boundary shapes move out of daily ownership first; delete/archive timing is tracked in the phase-29x legacy ledger.
- Current stage2 direction is moving owner decisions into `.hako`; treat these partitions as transitional thin shim surfaces, not permanent semantic owners.
- First code slices now extracted emit primitives into `hako_llvmc_ffi_emit_seam.inc`, and generic-method routing/classification is being split toward `hako_llvmc_ffi_generic_method_match.inc`.
- The shared compiler-state helper table is now landing in `hako_llvmc_ffi_compiler_state.inc`; keep route/placement decisions separate from raw state access.
- String concat emit helpers now live in `hako_llvmc_ffi_string_concat_emit.inc`; keep concat routing thin and avoid re-growing the wrapper body.
- `hako_llvmc_ffi_string_chain_terms.inc` is the shared term layer for string-chain policy/window seams; keep route-term definitions out of the heavy producer-window helpers.
- `hako_llvmc_ffi_string_candidate_plan_readers.inc` owns register-rooted candidate/plan JSON readers; keep metadata access out of `string_chain_terms.inc`.
- `hako_llvmc_ffi_string_loop_seed.inc` is now only a family facade; keep matcher/copy/emit details inside the dedicated seed-family includes and preserve dispatch order there.
- `hako_llvmc_ffi_string_chain_policy.inc` is the compiler-side bridge to `lang/src/runtime/kernel/string/chain_policy.hako`; keep route / retained-form / post-store observer names aligned and avoid reopening the `pure_compile` ladder.
- `hako_llvmc_ffi_mir_call_route_policy.inc`, `hako_llvmc_ffi_mir_call_need_policy.inc`, and `hako_llvmc_ffi_mir_call_surface_policy.inc` bridge to `lang/src/runtime/meta/`; keep compiler semantic tables out of `pure_compile.inc`.
- `hako_llvmc_ffi_mir_call_dispatch.inc` is the only `mir_call` dispatcher seam that `pure_compile.inc` should call directly.
- `hako_llvmc_ffi_generic_method_policy.inc` is the compiler-side bridge to `lang/src/runtime/collections/method_policy_box.hako`; keep emit-kind names aligned and avoid re-growing `generic_method_match.inc`.
- `hako_llvmc_ffi_generic_method_len_policy.inc` is the first generic-method action seam; keep `len` route ownership out of `generic_method_lowering.inc`.
- `hako_llvmc_ffi_generic_method_push_policy.inc` is the second generic-method action seam; keep `push` route ownership out of `generic_method_lowering.inc`.
- `hako_llvmc_ffi_generic_method_has_policy.inc` is the third generic-method action seam; keep `has` route ownership out of `generic_method_lowering.inc`.
- `hako_llvmc_ffi_generic_method_substring_policy.inc` is the fourth generic-method action seam; keep `substring` route ownership out of `generic_method_lowering.inc`.
- `hako_llvmc_ffi_generic_method_get_policy.inc` is the eleventh-stage fallback seam; keep only final `get` route ownership there and leave window/RMW/indexOf defer logic in compiler-state-heavy lowering for now.
- `hako_llvmc_ffi_generic_method_get_window.inc` is the `GET` producer-window helper bundle; keep its producer-side probe logic thin and avoid growing it back into `pure_compile.inc`.
- `hako_llvmc_ffi_generic_method_get_lowering.inc` is the dispatcher seam for `GET`; keep the case body out of `generic_method_lowering.inc`.
- `hako_llvmc_ffi_string_concat_window.inc` is the producer-window helper seam for string concat; keep helper logic out of `string_concat_match.inc` once the migration settles.
- Keep `hako_llvmc_ffi_generic_method_get_window.inc`, `hako_llvmc_ffi_string_concat_window.inc`, and the `indexOf` observer family native in this wave; they are compiler-state-heavy analyzers, not the next `.hako` owner tables.
- For pure-first lanes, the formal boundary contract is `llpath canonical emit`: promotable scalar stack slots are canonicalized before object emission, and the current implementation does that with `opt -passes=mem2reg` before `llc`.
- `HAKO_CAPI_TM` is an explicit bypass / compat-probe keep lane; do not treat it as the canonical mainline contract.
