# 293x-763 MIMAP-240A Segment Arena Backing Scalar Requirement Matrix Inventory

Status: landed
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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_requirement_matrix_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-240A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Scope

- Added the requirement matrix owner and typed report.
- Added a proof app that consumes MIMAP-236A readiness and MIMAP-237A
  diagnostics, then records accepted, readiness-rejected, diagnostic-rejected,
  invalid-geometry, and closed-substrate requirement rows.
- Added the MIMAP-240A L2 guard, proof manifest entry, check index entry, and
  requirement matrix SSOT.

## Selected Next Row

MIMAP-240A selects:

```text
MIMAP-241A segment arena backing requirement matrix diagnostics
```

Reason:

```text
the requirement matrix inventory now names the closed substrates. Add an
observer-only diagnostic row before closing out the family with representative
L3 evidence.
```
