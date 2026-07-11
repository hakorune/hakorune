# Phase 180x: string seam cleanup before broader DCE

- Status: Landed
- Purpose: `phase179x` で入れた `StringKernelPlan` seam を本当の owner に近づけて、`string` bridge を benchmark-shaped matcher 群から段階的に切り離す。
- Scope:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/phases/README.md`
  - `docs/development/current/main/phases/phase-137x/README.md`
  - `docs/development/current/main/phases/phase-163x/README.md`
  - `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
  - `src/mir/string_kernel_plan.rs`
  - `src/mir/string_corridor_placement.rs`
  - `src/mir/string_corridor_relation.rs`
  - `src/runner/mir_json_emit/mod.rs`
  - `lang/c-abi/shims/hako_llvmc_ffi_common.inc`
  - `lang/c-abi/shims/hako_llvmc_ffi_string_chain_terms.inc`
  - `lang/c-abi/shims/hako_llvmc_ffi_string_loop_seed.inc`
- Non-goals:
  - no new string IR dialect
  - no broader DCE widening in this phase
  - no new benchmark-specific fast path
  - no full-loop exact fallback retirement before plan lane proves the same keeper

## Decision Now

- `phase179x` is landed, and `phase180x` has now finished the seam cleanup:
  - landed: `StringKernelPlan` owner is extracted out of placement/export sidecars
  - landed: `stable_length_scalar` relation no longer reads downstream candidate-plan windows
  - landed: function-level string metadata readers are out of generic `common`
  - landed: register-rooted candidate/plan readers are out of `string_chain_terms`
  - landed: `string_loop_seed` is split into emitters / copy-graph / views-only / length-hot-loop / substring-concat families behind a thin facade
  - landed: the old dedicated `substring_concat_len_ascii_seed` ladder is now only a thin wrapper to the loop matcher, so the len-only exact route comes from the metadata-first seam alone
  - landed: `substring_concat_loop_ascii_seed` now splits a narrow metadata-first len preamble from the remaining legacy full-loop fallback helper
  - landed: `StringKernelPlan` now exports the remaining exact-loop scalar payload (`seed_literal`, `seed_length`, `loop_bound`, `split_length`) and the substring-concat full-loop route reads that metadata before touching the legacy helper
  - landed: the remaining raw full-loop fallback inside `substring_concat_loop_ascii_seed` is retired, so the substring-concat loop route is now plan-first only
- this phase was a BoxShape cleanup before more optimization, and the remaining raw fallback has now been retired:
  1. extract `StringKernelPlan` owner into its own MIR module
  2. stop `relation -> candidate` reverse dependency
  3. split string metadata readers out of heavy shim owner files
  4. only then shrink the remaining exact matcher bridge
- current state:
  - `180xA` landed: phase/docs/pointer lock is in place
  - `180xB` landed: `StringKernelPlan` owner now lives in `src/mir/string_kernel_plan.rs`
  - `180xC` landed: `stable_length_scalar` no longer reads downstream candidate-plan windows; relation now derives its narrow witness from the carried base definition itself
  - `180xD` first cut landed: function-level string metadata readers moved out of `hako_llvmc_ffi_common.inc`
  - `180xD` second cut landed: register-rooted candidate/plan readers now live in `hako_llvmc_ffi_string_candidate_plan_readers.inc`, and `string_chain_terms.inc` is back to term/classifier ownership
  - `180xE` landed: `string_loop_seed` is now physically split into emitters / copy-graph / views-only / length-hot-loop / substring-concat families behind a thin facade
  - `180xF` first cut landed: the dedicated `substring_concat_len_ascii_seed` ladder is retired into a thin wrapper, so the len-only exact route now comes only from the same metadata-first `stable_length_scalar + StringKernelPlan` seam used by the loop matcher
  - `180xF` second cut landed: `substring_concat_loop_ascii_seed` now separates a narrow metadata-first len preamble from the remaining legacy full-loop fallback helper
  - `180xF` third cut landed: the legacy full-loop fallback no longer depends on duplicated preheader/header exact checks beyond the values it still truly needs (`seed_len`, `loop_bound`)
  - `180xF` fourth cut landed: `StringKernelPlan` now exports the remaining exact-loop scalar payload (`seed_literal`, `seed_length`, `loop_bound`, `split_length`), and the substring-concat full-loop route reads that metadata before touching the legacy helper
  - `180xF` fifth cut landed: the remaining raw full-loop fallback inside `substring_concat_loop_ascii_seed` is retired, so the substring-concat loop route is now plan-first only

## Acceptance

- MIR owns `StringKernelPlan` as a dedicated derived view, not as a placement sidecar
- relation no longer depends on candidate plan internals
- MIR JSON exporter serializes plan data without inventing legality/policy locally
- string metadata readers in C shims are responsibility-partitioned away from generic `common` and `terms`
- `string_loop_seed` is responsibility-partitioned by family without changing the matcher surface seen by `pure_compile`
- exact perf/asm keepers for `kilo_micro_substring_concat` stay green while structure is cleaned and the loop route stays plan-first only

## Exit

- `relation -> candidate -> plan -> export -> shim` direction is explicit and one-way
- `string_loop_seed` keeps only the remaining bridge families that still lack a plan-complete route
- broader DCE can resume without raw shape matcher fragility being the limiting factor
