# 291x-749 JoinValueSpace Helper Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/join_value_space.rs`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

291x-747 worker inventory marked the extra `JoinValueSpace` methods as a
test-surface candidate. Production callers use the core allocator path:

- `JoinValueSpace::new`
- `alloc_param`
- `alloc_local`
- constants / region boundaries

The helper methods for PHI reservation, region introspection, counters, overlap
verification, and allocator closures were only exercised by self-tests. Keeping
them public made the allocator look broader than its active responsibility.

## Decision

Prune only the unused helper/debug surface.

Keep `alloc_join_param` / `alloc_join_local` for a later wrapper API reconcile
card because docs still name those wrappers as a desired explicit JoinIR
allocation API.

## Landed

- Removed PHI reservation helpers:
  - `reserve_phi`
  - `is_phi_reserved`
- Removed region/debug helpers:
  - `Region`
  - `region_of`
  - `verify_region`
  - `param_count`
  - `local_count`
  - `phi_reserved_count`
  - `verify_no_overlap`
- Removed allocator closure helpers:
  - `local_allocator`
  - `param_allocator`
- Removed tests that existed only to cover those helper methods.
- Kept the core allocation tests and the phase-201 no-collision scenario.

## Remaining Queue Impact

The `JoinValueSpace` extra helper shelf is removed. The wrapper API decision
remains separate:

- either migrate active callers from `alloc_param` / `alloc_local` to
  `alloc_join_param` / `alloc_join_local`
- or delete the wrappers and update the docs that recommend them

## Proof

- `rg -n "reserve_phi|is_phi_reserved|region_of|param_count\\(|local_count\\(|phi_reserved_count|verify_no_overlap|local_allocator|param_allocator|pub enum Region" src/mir/join_ir/lowering/join_value_space.rs`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
- `cargo test --lib join_value_space`
