# 293x-752 MIMAP-229A Source Lifecycle-Keyed Release Ledger Diagnostics

Status: selected current
Date: 2026-05-19

## Decision

Add diagnostics around the source lifecycle-keyed release ledger.

## Context

MIMAP-228A introduced a scalar/model source release ledger keyed by
`reuse_lifecycle_token` while preserving the old modeled-reuse-token keyed
release owner as an unmigrated reference.

The next row should keep the same route shape and add narrower diagnostics for
duplicate lifecycle keys, stale/mismatched lifecycle reports, and migrated-key
reject summary before the migration closeout pack.

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
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
