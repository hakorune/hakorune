# 293x-751 MIMAP-228A Source Release-Ledger Lifecycle-Key Migration Pilot

Status: landed
Date: 2026-05-19

## Decision

Add a controlled source release-ledger lifecycle-key migration pilot.

## Context

MIMAP-224A proved a shadow release ledger keyed by `reuse_lifecycle_token`, and
MIMAP-226A closed that shadow pack with representative exact-MIR L3 EXE
evidence.

MIMAP-228A introduces a new source release ledger owner keyed by lifecycle token:

```text
precondition ready
  -> lifecycle report accepted
  -> modeled reuse token matches
  -> source release row keyed by reuse_lifecycle_token
```

This row does not mutate the old modeled-reuse-token keyed release owner in
place. It keeps modeled reuse token as a backref field and remains scalar/model
only.

## Scope

- Add migrated source release ledger owner:
  `lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_ledger_box.hako`.
- Add proof app:
  `apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-proof`.
- Add first-pattern L3 guard:
  `tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_ledger_guard.sh`.
- Add accepted design SSOT:
  `docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-ssot.md`.

## Stop Lines

- No mutation of the old modeled-reuse-token keyed release owner.
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
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_ledger_guard.sh --level L3
bash tools/checks/run_proof_app.sh --only MIMAP-228A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-229A source lifecycle-keyed release ledger diagnostics
```
