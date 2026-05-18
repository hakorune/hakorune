# Hako Alloc Segment Map Local Free Reuse Ledger Lifecycle-Keyed Release Ledger Diagnostics SSOT

Status: active
Date: 2026-05-19
Decision: accepted

## Purpose

Model diagnostics for the source lifecycle-keyed release ledger introduced by
MIMAP-228A.

The row observes the lifecycle-keyed source release ledger and publishes a
scalar reject summary. It does not mutate either the old modeled-reuse-token
keyed release owner or the new lifecycle-keyed source release ledger.

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_ledger_diagnostic_box.hako
```

The owner may:

- observe the MIMAP-228A lifecycle-keyed source release ledger;
- publish attempt, accepted, reject, and ledger row counts;
- publish duplicate lifecycle-key, precondition, lifecycle-report,
  modeled/lifecycle token mismatch, and unsupported-requirement reject summary
  flags;
- reject missing ledger evidence and unsupported diagnostic requirements.

The owner must not:

- mutate any release ledger;
- create new release rows;
- define real lifecycle/generation semantics;
- execute real allocator behavior.

## Validation

MIMAP-229A uses L2 scalar/MIR validation:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_ledger_diagnostics_guard.sh --level L2
```

The guard must:

- prove one observed diagnostic summary for a populated lifecycle-keyed source
  release ledger;
- prove missing-ledger and unsupported-requirement diagnostic rejects;
- prove the diagnostic owner does not mutate release ledgers;
- prove real segment allocation/free, raw pointer, segment-map execution,
  arena, atomics, OSVM, worker, provider, and backend matcher seams remain
  inactive.

## Stop Lines

- No mutation of the old modeled-reuse-token keyed release owner.
- No mutation of the lifecycle-keyed source release ledger.
- No generation/lifecycle semantics for real allocator cycles.
- No real segment allocation/free execution.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No real allocator free-list mutation.
- No direct page-array mutation outside explicit modeled page owners.
- No arena backing allocation.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.
