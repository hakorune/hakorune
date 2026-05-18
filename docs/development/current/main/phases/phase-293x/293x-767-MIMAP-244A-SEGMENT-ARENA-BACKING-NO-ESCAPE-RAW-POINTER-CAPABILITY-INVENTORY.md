# 293x-767 MIMAP-244A Segment Arena Backing No-Escape Raw Pointer Capability Inventory

Status: landed
Date: 2026-05-19

## Decision

Inventory the no-escape raw pointer capability boundary before opening any real
raw pointer residence or arena backing allocation.

## Context

MIMAP-240A inventoried arena backing requirements, MIMAP-241A added
diagnostics, and MIMAP-242A closed out that family. The next bridge should name
the raw pointer capability preconditions without creating pointer residence or
allowing pointer-derived lookup.

## Scope

- Scalar owner / lifetime / generation facts for a future no-escape raw pointer
  carrier.
- Address-like scalar carrier facts, not pointer residence.
- Escape-blocker categories for return, storage, alias, backend matcher,
  provider, OSVM, worker, and real segment-map execution.
- L2 validation only; L3 evidence is deferred to a future closeout pack unless
  a new backend route shape is introduced.

## Stop Lines

- No real raw pointer residence.
- No pointer-derived lookup.
- No real arena backing allocation.
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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_no_escape_address_capability_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-244A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Scope

- Added the no-escape address capability owner and typed report.
- Added a proof app that consumes a requirement matrix report and records
  accepted, matrix-rejected, lifetime-rejected, address-rejected, escape, and
  closed-substrate blocker rows.
- Added the MIMAP-244A L2 guard, proof manifest entry, check index entry, and
  capability SSOT.

## Selected Next Row

MIMAP-244A selects:

```text
MIMAP-245A segment arena backing no-escape address capability diagnostics
```

Reason:

```text
the no-escape address capability inventory now names the scalar carrier and
escape blockers. Add an observer-only diagnostic row before closeout.
```
