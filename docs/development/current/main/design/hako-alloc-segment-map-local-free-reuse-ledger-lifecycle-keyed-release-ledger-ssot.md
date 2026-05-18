# Hako Alloc Segment Map Local Free Reuse Ledger Lifecycle-Keyed Release Ledger SSOT

Status: active
Date: 2026-05-19
Decision: accepted

## Purpose

Model the controlled source release-ledger lifecycle-key migration pilot.

The row introduces a new lifecycle-keyed source release ledger owner instead of
mutating the old modeled-reuse-token keyed release owner in place. It uses
`reuse_lifecycle_token` as the release key and keeps `modeled_reuse_token` as a
backref field.

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_ledger_box.hako
```

The owner may:

- accept an already-ready MIMAP-220A precondition report and an accepted
  lifecycle-token report;
- record one migrated source release row keyed by `reuse_lifecycle_token`;
- publish `source_release_ledger_key_kind = 1` and `release_key_migrated = 1`;
- reject missing precondition readiness, invalid lifecycle reports,
  modeled-reuse-token mismatch, duplicate lifecycle-keyed release rows, and
  unsupported requirements.

The owner must not:

- mutate the old modeled-reuse-token keyed release owner;
- define real generation/lifecycle semantics;
- execute real allocator behavior.

## Validation

MIMAP-228A is a first-pattern row and uses L3 validation:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_ledger_guard.sh --level L3
```

The guard must:

- prove one accepted lifecycle-keyed source release row;
- prove duplicate, precondition, lifecycle-report, token-mismatch, and
  unsupported-requirement rejects;
- prove the old modeled-reuse-token keyed release owner stays unmigrated;
- prove real segment allocation/free, raw pointer, segment-map execution,
  arena, atomics, OSVM, worker, provider, and backend matcher seams remain
  inactive.

## Stop Lines

- No mutation of the old modeled-reuse-token keyed release owner.
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
