# 293x-695 MIMAP-173A Post Segment Map Released Span Local Free Candidate Bridge Row Selection

Status: selected current
Date: 2026-05-18

## Decision

Choose the next narrow row after MIMAP-172A proves the segment-map
released-span to local-free candidate bridge.

## Context

The current scalar/model chain now proves:

```text
explicit-ID readiness
  -> modeled consume ledger live token
  -> modeled ledger release report
  -> released-span ledger can observe the segment-map release report
  -> local-free candidate ledger can consume that released-span row
```

The next row should choose between local-free apply-plan composition, modeled
free-list observation, or a cleanup sidecar. It should not jump directly to
raw pointer residence, arena backing, real segment-map execution, real
free-list mutation, or atomic bitmap behavior.

## Stop Lines

- No real segment allocation/free execution.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No real free-list mutation unless a future row explicitly selects a modeled
  bridge and keeps execution closed.
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
