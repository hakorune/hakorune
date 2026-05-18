# Hako Alloc Segment Arena Backing Requirement Matrix Diagnostics SSOT

Status: active
Date: 2026-05-19
Decision: accepted

## Purpose

Publish observer-only diagnostics for the MIMAP-240A segment arena backing
scalar requirement matrix.

MIMAP-241A consumes the requirement matrix inventory counters plus the last
matrix report, then records scalar summary facts for missing inventory,
readiness/diagnostics/geometry rejects, and each closed substrate requirement
category.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_requirement_matrix_diagnostic_box.hako
```

The owner may:

- observe MIMAP-240A inventory counters;
- summarize readiness, diagnostics, geometry, arena backing, raw pointer, real
  segment-map, atomic bitmap, OSVM, worker, provider, and backend matcher
  requirement reject categories;
- publish last matrix report facts and inactive execution flags.

The owner must not:

- record requirement matrix rows itself;
- allocate arena backing;
- use raw pointer residence;
- mutate a real segment-map;
- execute real segment allocation/free;
- execute atomic bitmap claims;
- call page-source or OSVM seams;
- infer anything from owner names or backend matchers.

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | diagnostic summary accepted |
| `1` | inventory or matrix report missing / not applicable |
| `2-11` | reserved for the MIMAP-240A requirement matrix reason surface |

The accepted diagnostic report mirrors the last MIMAP-240A matrix reason in
`reason`; observer owner counters still use `0`/`1` for its own accept/reject
state.

## Validation

MIMAP-241A uses daily L2 validation:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_requirement_matrix_diagnostics_guard.sh --level L2
```

The guard must:

- prove diagnostic observation after the MIMAP-240A inventory;
- prove category flags for readiness, diagnostics, geometry, and every blocked
  requirement;
- prove empty inventory is rejected;
- prove inactive flags remain zero.

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
