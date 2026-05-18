# 293x-756 MIMAP-233A Source Lifecycle-Keyed Release Apply/Recycle Continuation Diagnostics

Status: landed
Date: 2026-05-19

## Decision

Add diagnostics around lifecycle-keyed release apply/recycle continuation.

Selected next row:

```text
MIMAP-234A source lifecycle-keyed release apply/recycle continuation closeout pack
```

## Context

MIMAP-232A connects lifecycle-keyed source release rows to modeled reuse-ledger
release-apply and recycled local-free reuse. The next row should keep the same
route shape and add narrower diagnostics for missing live row, unsupported
lifecycle-keyed apply, and post-continuation duplicate reuse before a closeout
pack.

MIMAP-233A adds an observer-only diagnostic owner for those facts and keeps L3
EXE evidence deferred to the closeout pack.

## Scope

- Add diagnostic owner:
  `lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_apply_recycle_diagnostic_box.hako`.
- Add proof app:
  `apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-diagnostics-proof`.
- Add L2 guard:
  `tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_apply_recycle_continuation_diagnostics_guard.sh`.
- Add accepted design SSOT:
  `docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-diagnostics-ssot.md`.

## Stop Lines

- No use of the old modeled-reuse-token keyed release owner as the continuation
  owner; isolated fixture setup/precondition reports are allowed.
- No generation/lifecycle semantics for real allocator cycles.
- No real segment allocation/free execution.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No real allocator free-list mutation beyond the modeled reuse ledger owner.
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
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_apply_recycle_continuation_diagnostics_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-233A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-234A source lifecycle-keyed release apply/recycle continuation closeout pack
```
