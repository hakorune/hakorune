# Hako Alloc Segment Map Local Free Reuse Ledger Lifecycle-Token Release-Key Precondition SSOT

Status: active
Date: 2026-05-19
Decision: accepted

## Purpose

Add a scalar precondition observer before any release-ledger key migration row.

The owner reads lifecycle-token observer diagnostics and classifies whether a
future row has enough modeled evidence to consider release-key migration:

```text
observer accepted
release duplicate seen
lifecycle_count >= 2
```

It reports a `migration_candidate` flag but always keeps
`would_migrate_release_ledger_key = 0`.

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_token_release_key_precondition_box.hako
```

The owner may:

- classify observer reports as ready or blocked;
- reject missing observer acceptance, missing duplicate-release evidence,
  insufficient lifecycle count, and unsupported requirements;
- publish a scalar migration-candidate diagnostic.

The owner must not:

- migrate release-ledger keys;
- define real generation/lifecycle semantics;
- mutate source reuse ledger or release owner state;
- execute real allocator behavior.

## Validation

MIMAP-220A uses daily L2 validation:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_release_key_precondition_guard.sh --level L2
```

The guard must:

- prove ready classification from accepted lifecycle observer diagnostics;
- prove blocked classifications for missing observer, missing duplicate,
  insufficient lifecycle count, and unsupported requirements;
- prove migration execution remains inactive;
- prove real segment allocation/free, raw pointer, segment-map execution,
  arena, atomics, OSVM, worker, provider, and backend matcher seams remain
  inactive;
- keep L3 EXE deferred to a future closeout pack.

## Stop Lines

- No release ledger key migration.
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
