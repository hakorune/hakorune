# 293x-743 MIMAP-220A Segment Map Local Free Reuse Ledger Lifecycle-Token Release-Key Precondition Observer

Status: landed
Date: 2026-05-19

## Decision

Add a narrow precondition observer for a future release-key migration decision.

## Context

MIMAP-216A observes lifecycle-token facts while confirming the release ledger is
still keyed by modeled reuse token. MIMAP-218A closed that diagnostic pack with
representative exact-MIR L3 EXE evidence.

MIMAP-220A classifies whether those observed facts are ready for a future row to
consider release-key migration:

```text
observer diagnostic accepted
  -> release duplicate seen
  -> lifecycle_count >= 2
  -> migration candidate yes, migration execution no
```

This is a precondition observer only. It does not migrate release-ledger keys or
define real allocator generation semantics.

## Scope

- Add precondition owner:
  `lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_token_release_key_precondition_box.hako`.
- Add proof app:
  `apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-proof`.
- Add L2 guard:
  `tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_release_key_precondition_guard.sh`.
- Add accepted design SSOT:
  `docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-ssot.md`.

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
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_release_key_precondition_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-220A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-221A post-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition row selection
```
