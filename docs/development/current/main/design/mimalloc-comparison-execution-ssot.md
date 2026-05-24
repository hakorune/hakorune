---
Status: Active
Date: 2026-05-24
Scope: SSOT for the post-294x mimalloc comparison execution lane.
Related:
  - docs/development/current/main/phases/phase-295x/README.md
  - docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md
  - docs/development/current/main/phases/phase-294x/294x-270-PHASE-294X-USIZE-COMPARISON-CLOSEOUT.md
---

# Mimalloc Comparison Execution SSOT

## Decision

Phase-295x resumes mimalloc-facing work from a comparison execution seam.

The goal is not provider activation, DLL packaging, host allocator replacement,
or native allocator replacement. The goal is a narrow, reproducible comparison
between:

- the existing `.hako` / `hako_alloc` vertical-slice evidence; and
- the explicit C mimalloc runner evidence surface.

## Boundary

Open:

- explicit runner output/evidence shape selection;
- stable workload and metric contract checks;
- comparison ledger/presentation rows that consume existing evidence;
- `.hako` port rows that directly improve the comparison workload.

Keep closed:

- provider package / DLL generation;
- provider activation and provider API execution;
- process allocator replacement, hooks, backend matchers, and
  `#[global_allocator]`;
- worker/TLS, true threads, atomics, remote-free stress, abandoned heap stress,
  and native allocator replacement claims;
- broad production `usize` field migration not required by the comparison
  workload.

## Validation Cadence

- Planning/inventory rows: docs/static guard.
- `.hako` model/evidence rows: VM + MIR JSON + route preflight.
- First execution or closeout rows: representative L3 where the row explicitly
  requires it.
- Heavy allocator-wide packs: explicit only.

## Stop Line

Do not use the comparison lane as a back door for allocator-provider
activation or full native mimalloc reimplementation. If a row needs those
surfaces, split it into a later phase/lane with a new SSOT.
