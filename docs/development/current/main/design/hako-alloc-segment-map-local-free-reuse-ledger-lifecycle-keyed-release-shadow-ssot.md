# Hako Alloc Segment Map Local Free Reuse Ledger Lifecycle-Keyed Release Shadow SSOT

Status: active
Date: 2026-05-19
Decision: accepted

## Purpose

Model a shadow release ledger keyed by reuse lifecycle token after the
release-key precondition observer accepts.

This is not a migration of the existing source release ledger. The source
release owner remains keyed by modeled reuse token. The shadow owner proves the
next ledger shape in scalar/model space while `would_migrate_release_ledger_key`
stays `0`.

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_shadow_box.hako
```

The owner may:

- accept an already-ready MIMAP-220A precondition report and an accepted
  lifecycle-token report;
- record one shadow row keyed by `reuse_lifecycle_token`;
- reject missing precondition readiness, invalid lifecycle reports,
  modeled-reuse-token mismatch, duplicate lifecycle-keyed release rows, and
  unsupported requirements.

The owner must not:

- migrate the source release ledger key;
- define real generation/lifecycle semantics;
- mutate source reuse ledger or release owner state;
- execute real allocator behavior.

## Validation

MIMAP-224A uses daily L2 validation:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_shadow_guard.sh --level L2
```

The guard must:

- prove one accepted shadow row keyed by reuse lifecycle token;
- prove duplicate, precondition, lifecycle-report, token-mismatch, and
  unsupported-requirement rejects;
- prove release-ledger key migration remains inactive;
- prove real segment allocation/free, raw pointer, segment-map execution,
  arena, atomics, OSVM, worker, provider, and backend matcher seams remain
  inactive;
- keep L3 EXE deferred to a future closeout pack.

## Stop Lines

- No source release ledger key migration.
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
