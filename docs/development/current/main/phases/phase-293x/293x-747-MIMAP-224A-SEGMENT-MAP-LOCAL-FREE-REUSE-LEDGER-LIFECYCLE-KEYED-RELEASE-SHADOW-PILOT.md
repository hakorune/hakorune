# 293x-747 MIMAP-224A Segment Map Local Free Reuse Ledger Lifecycle-Keyed Release Shadow Pilot

Status: landed
Date: 2026-05-19

## Decision

Add a lifecycle-keyed release shadow ledger pilot.

## Context

MIMAP-220A classifies lifecycle-token facts as a future migration candidate
while keeping the actual release ledger keyed by modeled reuse token. MIMAP-222A
closed that pack with representative exact-MIR L3 EXE evidence.

MIMAP-224A adds a shadow owner that records a release row keyed by
`reuse_lifecycle_token` only when the precondition observer is ready and the
lifecycle-token report is accepted:

```text
precondition ready
  -> lifecycle report accepted
  -> modeled reuse token matches
  -> shadow row keyed by reuse_lifecycle_token
```

This is not source release-ledger key migration. It only proves the next ledger
shape in scalar/model space.

## Scope

- Add shadow owner:
  `lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_shadow_box.hako`.
- Add proof app:
  `apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-proof`.
- Add L2 guard:
  `tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_shadow_guard.sh`.
- Add accepted design SSOT:
  `docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-ssot.md`.

## Stop Lines

- No source release ledger key migration.
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
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_shadow_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-224A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-225A post-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow row selection
```
