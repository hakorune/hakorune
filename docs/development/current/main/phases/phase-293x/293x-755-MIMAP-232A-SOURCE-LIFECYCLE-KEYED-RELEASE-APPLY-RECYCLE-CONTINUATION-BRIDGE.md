# 293x-755 MIMAP-232A Source Lifecycle-Keyed Release Apply/Recycle Continuation Bridge

Status: landed
Date: 2026-05-19

## Decision

Add a scalar/model continuation bridge from lifecycle-keyed source release rows
to release-apply and recycled local-free reuse.

Selected next row:

```text
MIMAP-233A source lifecycle-keyed release apply/recycle continuation diagnostics
```

## Context

MIMAP-228A introduced a source release ledger keyed by `reuse_lifecycle_token`
while keeping `modeled_reuse_token` as a backref. MIMAP-230A closed that
migration pack. The next bridge should prove that a lifecycle-keyed release row
can still continue into the modeled reuse ledger release-apply/recycle path
without reopening raw pointer, real segment-map, arena, or atomic behavior.

MIMAP-232A adds a lifecycle-keyed release apply entry on the modeled reuse
ledger. It uses the modeled reuse token backref to apply the current live row
and then records the next recycled local-free reuse row.

## Scope

- Extend modeled reuse ledger owner:
  `lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako`.
- Add proof app:
  `apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-proof`.
- Add first-pattern L3 guard:
  `tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_apply_recycle_continuation_guard.sh`.
- Add accepted design SSOT:
  `docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-ssot.md`.

## Stop Lines

- No use of the old modeled-reuse-token keyed release owner as the continuation
  owner; isolated fixture setup/precondition reports are allowed.
- No generation/lifecycle semantics for real allocator cycles.
- No real segment allocation/free execution.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No real allocator free-list mutation.
- No arena backing allocation.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_apply_recycle_continuation_guard.sh --level L3
bash tools/checks/run_proof_app.sh --only MIMAP-232A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-233A source lifecycle-keyed release apply/recycle continuation diagnostics
```
