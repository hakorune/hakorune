# 293x-775 MIMAP-252A Segment Arena Backing Modeled Residence Arena-Binding Inventory

Status: selected current
Date: 2026-05-19

## Decision

Add a scalar/model inventory row that binds an accepted modeled no-escape
address residence report to an accepted scalar requirement matrix for the same
segment and arena.

## Context

MIMAP-248A records a modeled no-escape address residence. MIMAP-250A closes out
that family with representative exact-MIR evidence. The next safe bridge toward
arena backing is to prove that the modeled residence can be associated with the
arena requirement matrix in scalar/model space before any real pointer
residence, pointer-derived lookup, or real arena backing opens.

## Scope

- Add an owner for modeled residence arena-binding inventory.
- Preserve segment id, arena id, residence token, lifetime generation, and
  requirement-matrix geometry facts.
- Reject missing/rejected residence or matrix reports, segment/arena mismatch,
  invalid residence token, invalid geometry, and closed-substrate requirement
  flags.

## Stop Lines

- No real raw pointer residence.
- No pointer-derived lookup or dereference.
- No real arena backing allocation.
- No real segment-map mutation.
- No real segment allocation/free execution.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_residence_arena_binding_guard.sh --level L0
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_residence_arena_binding_guard.sh --level L1
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_residence_arena_binding_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-252A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

MIMAP-252A uses daily L2 validation. Representative L3 evidence is deferred to
the future modeled residence arena-binding closeout pack unless this row opens
a new backend route shape.
