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

## Current Blocker

```text
MIMALLOC-COMPARISON-C-RUNNER-EVIDENCE-CONTRACT-REFRESH-001:
  selected current after 295x-01. Revalidate the explicit C mimalloc runner
  evidence contract against the current `.hako` vertical slice before choosing
  the next `.hako` port seam. Do not open provider/DLL/replacement seams.
```

## Queue

| Order | Row | Status | Boundary |
| --- | --- | --- | --- |
| 0 | `295x-00` | Landed | Lock phase-295x and switch current pointers from phase-294x closeout to mimalloc comparison execution. |
| 1 | `295x-01` | Landed | Selected explicit C mimalloc evidence contract refresh as the first execution/evidence row. |
| 2 | `MIMALLOC-COMPARISON-C-RUNNER-EVIDENCE-CONTRACT-REFRESH-001` | Current | Validate stable output / memory-use evidence contract; no benchmark repetition expansion unless selected. |
| 3 | `.hako` vs C comparison ledger refresh | Planned | Consume existing `.hako` vertical-slice and C runner evidence. |
| 4 | next `.hako` port seam selection | Planned | Select only a seam that improves the comparison workload. |

## Parked

- provider package / DLL generation;
- provider activation and provider API execution;
- process allocator replacement, hooks, backend matchers, and
  `#[global_allocator]`;
- worker/TLS, true threads, atomics, remote-free stress, abandoned heap stress,
  and native allocator replacement claims;
- broad production `usize` field migration outside the comparison workload.
