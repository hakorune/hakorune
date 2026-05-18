# 293x-752 MIMAP-229A Source Lifecycle-Keyed Release Ledger Diagnostics

Status: landed
Date: 2026-05-19

## Decision

Add diagnostics around the source lifecycle-keyed release ledger.

Selected next row:

```text
MIMAP-230A source release-ledger lifecycle-key migration closeout pack
```

## Context

MIMAP-228A introduced a scalar/model source release ledger keyed by
`reuse_lifecycle_token` while preserving the old modeled-reuse-token keyed
release owner as an unmigrated reference.

The next row should keep the same route shape and add narrower diagnostics for
duplicate lifecycle keys, stale/mismatched lifecycle reports, and migrated-key
reject summary before the migration closeout pack.

MIMAP-229A adds a scalar/model diagnostic owner. It observes the MIMAP-228A
lifecycle-keyed source release ledger and publishes reject summary flags without
mutating either release ledger.

## Scope

- Add diagnostic owner:
  `lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_ledger_diagnostic_box.hako`.
- Add proof app:
  `apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-diagnostics-proof`.
- Add L2 guard:
  `tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_ledger_diagnostics_guard.sh`.
- Add accepted design SSOT:
  `docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-diagnostics-ssot.md`.

## Stop Lines

- No mutation of the old modeled-reuse-token keyed release owner unless the next
  row explicitly selects it.
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
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_ledger_diagnostics_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-229A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-230A source release-ledger lifecycle-key migration closeout pack
```
