# 293x-739 MIMAP-216A Segment Map Local Free Reuse Ledger Lifecycle-Token Observer Diagnostic

Status: landed
Date: 2026-05-19

## Decision

Add a narrow observer/diagnostic sidecar for the lifecycle-token pilot.

## Context

MIMAP-212A proved that a dedicated owner can derive explicit lifecycle tokens
for the same modeled reuse token. MIMAP-214A closed that pack with
representative exact-MIR L3 EXE evidence.

MIMAP-216A observes that lifecycle-token state alongside the existing release
owner duplicate diagnostic:

```text
two lifecycle tokens exist for one modeled reuse token
  -> second release still rejects by modeled reuse token
  -> observer reports release key is still modeled reuse token
```

This is a diagnostic row only. It does not migrate release-ledger keys or define
real allocator generation semantics.

## Scope

- Add lifecycle-token observer owner:
  `lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_token_observer_box.hako`.
- Add proof app:
  `apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-proof`.
- Add L2 guard:
  `tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_observer_diagnostic_guard.sh`.
- Add accepted design SSOT:
  `docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-ssot.md`.

## Stop Lines

- No release ledger key migration.
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
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_observer_diagnostic_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-216A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-217A post-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic row selection
```
