# Phase 180x: string seam cleanup before broader DCE

- Status: Active
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

- `phase179x` is landed, but `StringKernelPlan` is still not the sole seam:
  - relation currently reads downstream candidate-plan details
  - exporter still assembles part/legality payloads itself
  - shim readers are still split across `common` and `string_chain_terms`
- this phase is a BoxShape cleanup before more optimization:
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
  - still open:
    - `string_loop_seed` family split

## Acceptance

- MIR owns `StringKernelPlan` as a dedicated derived view, not as a placement sidecar
- relation no longer depends on candidate plan internals
- MIR JSON exporter serializes plan data without inventing legality/policy locally
- string metadata readers in C shims are responsibility-partitioned away from generic `common` and `terms`
- exact perf/asm keepers for `kilo_micro_substring_concat` stay green while structure is cleaned

## Exit

- `relation -> candidate -> plan -> export -> shim` direction is explicit and one-way
- `string_loop_seed` keeps only the remaining bridge families that still lack a plan-complete route
- broader DCE can resume without raw shape matcher fragility being the limiting factor
