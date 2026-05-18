# 293x-763 MIMAP-240A Segment Arena Backing Scalar Requirement Matrix Inventory

Status: selected current
Date: 2026-05-19

## Decision

Inventory the scalar requirement matrix for future segment arena backing before
opening real arena backing allocation, no-escape raw pointer residence, real
segment-map execution, or atomic bitmap execution.

## Context

MIMAP-236A inventoried arena backing readiness, MIMAP-237A added diagnostics,
and MIMAP-238A closed out the family with representative L3 evidence. The next
bridge should keep everything scalar/model-only and name the arena backing
requirements that must be satisfied before real backing is allowed.

## Scope

- Arena id and segment id requirement facts.
- Slice count / committed / free geometry facts.
- Required alignment and page-size facts.
- Blocked substrate reason categories for arena backing, raw pointer,
  segment-map, atomic bitmap, OSVM/page-source, worker/provider activation, and
  backend matcher leaks.

## Stop Lines

- No real arena backing allocation.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No real segment allocation/free execution.
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
