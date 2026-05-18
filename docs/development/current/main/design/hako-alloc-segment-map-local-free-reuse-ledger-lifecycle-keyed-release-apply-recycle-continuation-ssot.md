# Hako Alloc Segment Map Local Free Reuse Ledger Lifecycle-Keyed Release Apply/Recycle Continuation SSOT

Status: active
Date: 2026-05-19
Decision: accepted

## Purpose

Model the scalar continuation from lifecycle-keyed source release rows to the
existing reuse ledger release-apply/recycle path.

MIMAP-228A changed the source release key to `reuse_lifecycle_token` while
preserving `modeled_reuse_token` as a backref. MIMAP-232A uses that backref to
apply the release to the current live reuse-ledger row and then records a new
recycled local-free reuse row.

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako
```

The owner may:

- accept a lifecycle-keyed source release report that has
  `release_key_migrated = 1` and `lifecycle_keyed_release_ledger_present = 1`;
- apply the release to the current live reuse-ledger row using
  `modeled_reuse_token` as an explicit backref;
- keep older dead rows with the same modeled token from blocking the live-row
  apply path;
- allow the regular local-free reuse ledger path to record the next recycled
  row after the lifecycle-keyed apply.

The owner must not:

- use the old modeled-reuse-token keyed release owner as the continuation
  owner; isolated fixture setup/precondition reports are allowed;
- infer anything from owner names or backend matchers;
- define real generation/lifecycle semantics;
- execute real allocator behavior.

## Validation

MIMAP-232A is a first-pattern bridge and uses L3 validation:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_apply_recycle_continuation_guard.sh --level L3
```

The guard must:

- prove a lifecycle-keyed source release row applies to the live recycled row;
- prove the next local-free reuse row can be recorded after lifecycle-keyed
  release apply;
- prove duplicate live reuse and unsupported lifecycle-keyed apply rejects;
- prove raw pointer, segment-map execution, arena, atomics, OSVM, worker,
  provider, and backend matcher seams remain inactive.

## Stop Lines

- No use of the old modeled-reuse-token keyed release owner as the continuation
  owner; isolated fixture setup/precondition reports are allowed.
- No generation/lifecycle semantics for real allocator cycles.
- No real segment allocation/free execution.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No real allocator free-list mutation outside the existing modeled reuse ledger
  scalar live flag owner.
- No direct page-array mutation outside explicit modeled page owners.
- No arena backing allocation.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.
