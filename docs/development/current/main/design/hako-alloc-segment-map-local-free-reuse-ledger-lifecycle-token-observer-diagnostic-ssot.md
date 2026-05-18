# Hako Alloc Segment Map Local Free Reuse Ledger Lifecycle-Token Observer Diagnostic SSOT

Status: active
Date: 2026-05-19
Decision: accepted

## Purpose

Add a diagnostic observer after the lifecycle-token pilot closeout.

The observer reads lifecycle-token pilot state and the release-owner duplicate
diagnostic to report the current boundary:

```text
lifecycle tokens can distinguish modeled attempts
release ledger still keys release records by modeled reuse token
```

This makes the next decision explicit before any row migrates release-ledger
keys or defines real generation/lifecycle semantics.

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_token_observer_box.hako
```

The owner may:

- observe lifecycle-token owner counts;
- observe whether the release owner rejected a duplicate release;
- report that release-ledger keys are still modeled reuse tokens;
- reject missing lifecycle-token state and unsupported-requirement branches.

The owner must not:

- migrate release-ledger keys;
- define real generation/lifecycle semantics;
- mutate source reuse ledger or release owner state;
- execute real allocator behavior.

## Validation

MIMAP-216A uses daily L2 validation:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_observer_diagnostic_guard.sh --level L2
```

The guard must:

- prove the observer accepts lifecycle-token state after the duplicate release
  diagnostic;
- prove missing lifecycle-token state and unsupported requirements reject;
- prove release-ledger migration remains inactive;
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
