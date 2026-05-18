# Hako Alloc Segment Map Local Free Reuse Ledger Lifecycle-Token Pilot SSOT

Status: active
Date: 2026-05-19
Decision: accepted

## Purpose

Introduce a narrow scalar lifecycle-token owner after the release-applied
recycle second-release diagnostic closeout.

MIMAP-212A does not change the source reuse ledger or the release owner. It
adds a dedicated owner that derives one scalar reuse-lifecycle token from:

```text
modeled_reuse_token
explicit lifecycle_id
```

The token proves that future rows can distinguish modeled lifecycle attempts
without retrofitting generation semantics into the existing release ledger in
the same row.

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_token_box.hako
```

The owner may:

- derive `reuse_lifecycle_token = modeled_reuse_token * 1000 + lifecycle_id`;
- record accepted lifecycle-token rows;
- reject invalid shape, duplicate lifecycle token, and unsupported requirement
  branches;
- use local brand constructors for the modeled reuse token and lifecycle id
  boundary.

The owner must not:

- migrate release-ledger keys;
- define generation/lifecycle semantics for real allocator cycles;
- mutate source reuse ledger or release owner state;
- execute real allocator behavior.

## Validation

MIMAP-212A uses daily L2 validation:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_pilot_guard.sh --level L2
```

The guard must:

- prove lifecycle token acceptance for two lifecycle ids of the same modeled
  reuse token;
- prove duplicate lifecycle-token rejection;
- prove invalid-shape and unsupported-requirement rejection;
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
