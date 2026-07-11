---
Status: Active
Decision: accepted
Date: 2026-04-12
Scope: `phase179x` で入れた string plan seam を owner / export / shim consumer の向きで整理し、exact bridge を benchmark-shaped matcher から段階的に縮める。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/phases/phase-163x/README.md
  - src/mir/string_kernel_plan.rs
  - src/mir/string_corridor_placement.rs
  - src/mir/string_corridor_relation.rs
  - src/runner/mir_json_emit/mod.rs
  - lang/c-abi/shims/hako_llvmc_ffi_common.inc
  - lang/c-abi/shims/hako_llvmc_ffi_string_chain_terms.inc
  - lang/c-abi/shims/hako_llvmc_ffi_string_loop_seed.inc
---

# 180x String Seam Cleanup SSOT

## Problem

`phase179x` proved the first metadata-first `substring_concat` len route, but the structure is still mixed:

- relation reads downstream candidate-plan details
- exporter assembles backend-facing legality locally
- shim readers are still mixed into generic owner files
- `string_loop_seed` still contains benchmark-shaped exact fallback families

That means the current fast path wins are real, but the seam is still fragile.

## Structural Rule

Use this direction only:

`canonical MIR facts -> relations -> candidates -> string kernel plan -> JSON export -> shim consumer`

Do not allow:

- relation to read candidate-owned plan internals
- exporter to invent backend policy
- shim readers to depend on both candidate-plan JSON and kernel-plan JSON long term

## Owner Split

### MIR

- `string_corridor.rs`
  - fact ownership only
- `string_corridor_relation.rs`
  - relation ownership only
- `string_corridor_placement.rs`
  - candidate ownership only
- `string_kernel_plan.rs`
  - backend-consumable derived plan ownership

### Export

- `src/runner/mir_json_emit/mod.rs`
  - serialization only
  - no string legality invention

### Shim

- `hako_llvmc_ffi_common.inc`
  - generic runtime/env/file helpers only
- `hako_llvmc_ffi_string_chain_terms.inc`
  - term/classifier vocabulary only
- dedicated string metadata reader includes
  - plan/relation reader ownership

## Cleanup Order

1. extract `StringKernelPlan` owner from placement
2. stop `relation -> candidate` reverse dependency
3. split string metadata readers from shim generic owner files
4. split `string_loop_seed` by family
5. retire exact matcher families only after plan-first keepers prove parity

## Retirement Rule

Do not delete a shape fallback only because a new plan exists.

Delete only when:

- the same fixture/smoke is green on the plan-first lane
- exact asm/perf keeper remains green
- shim no longer needs candidate-plan JSON to recover that family
