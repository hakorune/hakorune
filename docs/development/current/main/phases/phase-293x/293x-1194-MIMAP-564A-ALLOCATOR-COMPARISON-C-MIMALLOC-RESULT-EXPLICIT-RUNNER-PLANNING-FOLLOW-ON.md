# 293x-1194 MIMAP-564A Allocator Comparison C Mimalloc Result Explicit Runner Planning Follow-On

Status: completed
Date: 2026-05-22

## Purpose

Define the deeper explicit C mimalloc runner planning boundary after the first
presentation-only extension pack is closed.

This row remains planning-only. It must keep benchmark reruns, allocator/provider
ladders, and explicit C runner execution closed.

## Scope

- Define the future explicit runner pilot boundary as an external evidence source
  (not allocator replacement, not hook, not provider activation).
- Keep output/report schema continuity anchored to landed MIMAP-552A fields:
  `allocator_id`, `runner_kind`, `workload_id`, `allocation_count`,
  `free_count`, `requested_bytes`, `peak_rss_bytes`, `steady_rss_bytes`,
  `exit_code`, and `evidence_complete`.
- Define pilot preconditions only:
  no runtime winner claim, no benchmark rerun, and no execution seam reopening.

## Stop Lines

- No repeated or heavy benchmark pack.
- No process allocator replacement.
- No hook installation.
- No backend matcher additions.
- No `#[global_allocator]`.
- No provider package / DLL generation.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No worker/thread execution.
- No explicit C mimalloc runner execution.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Validation profile: `planning follow-on L2 pack`.

Validated:

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_explicit_runner_planning_follow_on_guard.sh
```

## Task Order

1. Re-run the MIMAP-562A presentation-only extension closeout guard.
2. Confirm explicit runner boundary is planning-only and external-evidence-only.
3. Keep all execution seams closed.

## Completed

- Re-ran the MIMAP-562A closeout guard as the required evidence anchor.
- Fixed the deeper explicit C mimalloc runner boundary as a planning-only seam.
- Selected MIMAP-565A as the next row-selection card.

## Next

MIMAP-565A should choose whether the next row is an explicit runner planning
pilot row, a presentation-only extension follow-on row, or another closeout
extension.
