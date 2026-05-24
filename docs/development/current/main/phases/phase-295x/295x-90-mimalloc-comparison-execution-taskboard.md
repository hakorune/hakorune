---
Status: Active
Date: 2026-05-24
Scope: taskboard for phase-295x mimalloc comparison execution.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/README.md
---

# 295x-90 Mimalloc Comparison Execution Taskboard

## Rule

One row should open one comparison seam. Do not mix comparison evidence work
with provider activation, host allocator replacement, DLL packaging, atomics,
worker/TLS, or broad exact `usize` field migration.

## Current Truth

- Phase-294x closed the exact `usize` comparison-quality slice at `294x-270`.
- The `.hako` / `hako_alloc` V5 vertical-slice evidence is stable after the
  page-heap non-id exact `usize` closeout.
- The next work should resume mimalloc-facing development from explicit
  comparison evidence, not from broad allocator field drainage.
- Current comparison rows are contract/evidence rows. They do not make
  performance or memory winner claims until a later apples-to-apples repeated
  benchmark row defines workload equivalence, warmup, repetitions, and summary
  statistics.

## Current Blocker

```text
MIMALLOC-COMPARISON-REALLOC-ALIGNED-EVIDENCE-295X-RUN-001:
  selected current after 295x-17. Run C mimalloc and `.hako` through the
  normalizer for representative-realloc-aligned-v0, requiring structural
  count/requested/realloc/aligned parity while keeping moved/copy/RSS as
  evidence-only fields.
```

## Queue

| Order | Row | Status | Boundary |
| --- | --- | --- | --- |
| 0 | `295x-00` | Landed | Lock phase-295x and switch current pointers from phase-294x closeout to mimalloc comparison execution. |
| 1 | `295x-01` | Landed | Selected explicit C mimalloc evidence contract refresh as the first execution/evidence row. |
| 2 | `295x-02` | Landed | Validated stable output / memory-use evidence contract without benchmark repetition expansion. |
| 3 | `295x-03` | Landed | Selected the `.hako` vs C comparison ledger refresh. |
| 4 | `295x-04` | Landed | Consumed existing `.hako` vertical-slice and C runner evidence through the result ledger and diagnostics. |
| 5 | `295x-05` | Landed | Consolidated the comparison method and selected result ledger closeout. |
| 6 | `295x-06` | Landed | Closed the refreshed result ledger pack without winner claims. |
| 7 | `295x-07` | Landed | Executed the same-workload memory report path for representative-small-block-v0. |
| 8 | `295x-08` | Landed | Closed the same-workload execution refresh without benchmark/winner expansion. |
| 9 | `295x-09` | Landed | Selected repeated-run evidence refresh before adding a wider `.hako` port seam. |
| 10 | `295x-10` | Landed | Refreshed repeated same-workload RSS evidence without winner claims. |
| 11 | `295x-11` | Landed | Closed repeated-run evidence and selected count-evidence seam selection. |
| 12 | `295x-12` | Landed | Selected `.hako` allocation/free count evidence refresh. |
| 13 | `295x-13` | Landed | Surfaced `.hako` allocation/free counts in hako memory evidence. |
| 14 | `295x-14` | Landed | Closed matching `.hako`/C allocation-free count evidence. |
| 15 | `295x-15` | Landed | Selected representative-realloc-aligned-v0 as the next same-workload family. |
| 16 | `295x-16` | Landed | Added realloc/aligned workload contract and optional evidence fields. |
| 17 | `295x-17` | Landed | Added a narrow exact-EXE `.hako` realloc/aligned evidence app. |
| 18 | `MIMALLOC-COMPARISON-REALLOC-ALIGNED-EVIDENCE-295X-RUN-001` | Current | Run C mimalloc and `.hako` same-workload evidence through the normalizer. |

## Parked

- provider package / DLL generation;
- provider activation and provider API execution;
- process allocator replacement, hooks, backend matchers, and
  `#[global_allocator]`;
- worker/TLS, true threads, atomics, remote-free stress, abandoned heap stress,
  and native allocator replacement claims;
- broad production `usize` field migration outside the comparison workload.
