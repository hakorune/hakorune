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
MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-PACK-RUN-295X-001:
  selected current after 295x-50. Run the selected comparison workload pack
  with the explicit empty runtime config profile and keep winner claims closed.
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
| 18 | `295x-18` | Landed | Ran C mimalloc and `.hako` realloc/aligned same-workload evidence through the normalizer. |
| 19 | `295x-19` | Landed | Closed the realloc/aligned workload family. |
| 20 | `295x-20` | Landed | Selected representative-mixed-small-v0 as the next mixed-size workload family. |
| 21 | `295x-21` | Landed | Added C runner and `.hako` evidence contract support for representative-mixed-small-v0. |
| 22 | `295x-22` | Landed | Ran mixed-size same-workload evidence through the normalizer. |
| 23 | `295x-23` | Landed | Closed the mixed-size workload family. |
| 24 | `295x-24` | Landed | Selected representative-huge-ish-v0 as the next huge-ish workload family. |
| 25 | `295x-25` | Landed | Added C runner and `.hako` evidence contract support for representative-huge-ish-v0. |
| 26 | `295x-26` | Landed | Ran huge-ish same-workload evidence through the normalizer. |
| 27 | `295x-27` | Landed | Closed the huge-ish workload family. |
| 28 | `295x-28` | Landed | Defined repeated measurement policy before winner claims. |
| 29 | `295x-29` | Landed | Implemented repeated evidence runner without winner claims. |
| 30 | `295x-30` | Landed | Ran selected workload pack without winner claims. |
| 31 | `295x-31` | Landed | Closed repeated measurement pack. |
| 32 | `295x-32` | Landed | Added presentation-only repeated measurement report. |
| 33 | `295x-33` | Landed | Selected RSS gap attribution plan. |
| 34 | `295x-34` | Landed | Added empty/baseline repeated evidence. |
| 35 | `295x-35` | Landed | Computed baseline-subtracted RSS gap evidence. |
| 36 | `295x-36` | Landed | Closed RSS gap attribution pack. |
| 37 | `295x-37` | Landed | Selected empty exact-EXE footprint diagnostic. |
| 38 | `295x-38` | Landed | Observed empty exact-EXE RSS and static/loadable footprint. |
| 39 | `295x-39` | Landed | Closed empty exact-EXE footprint diagnostic. |
| 40 | `295x-40` | Landed | Added env-gated NyRT self-RSS checkpoints. |
| 41 | `295x-41` | Landed | Ran empty no-output exact-EXE checkpoint diagnostic. |
| 42 | `295x-42` | Landed | Selected plugin-host substage RSS diagnostics. |
| 43 | `295x-43` | Landed | Added and ran plugin-host substage RSS checkpoints. |
| 44 | `295x-44` | Landed | Selected generated-config plugin load-set footprint diagnostic. |
| 45 | `295x-45` | Landed | Ran generated-config plugin load-set RSS diagnostic. |
| 46 | `295x-46` | Landed | Closed plugin load-set diagnostic and selected exact-EXE minimal config pilot. |
| 47 | `295x-47` | Landed | Added comparison-runner-only exact-EXE minimal runtime config pilot. |
| 48 | `295x-48` | Landed | Compared root versus generated-empty runtime config exact-EXE evidence. |
| 49 | `295x-49` | Landed | Closed `MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-CLOSEOUT-295X-001` and selected runtime config profile contract. |
| 50 | `295x-50` | Landed | Documented `MIMALLOC-COMPARISON-RUNTIME-CONFIG-PROFILE-CONTRACT-295X-001` for comparison-runner runtime config profiles. |
| 51 | `MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-PACK-RUN-295X-001` | Current | Run the selected comparison workload pack with explicit empty runtime config. |

## Parked

- provider package / DLL generation;
- provider activation and provider API execution;
- process allocator replacement, hooks, backend matchers, and
  `#[global_allocator]`;
- worker/TLS, true threads, atomics, remote-free stress, abandoned heap stress,
  and native allocator replacement claims;
- broad production `usize` field migration outside the comparison workload.
