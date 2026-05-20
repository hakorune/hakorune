# 293x-1048 MIMAP-426A Post Host Replacement Optional Ladder Row Selection

Status: selected current
Date: 2026-05-21

## Purpose

Select the next allocator row after the optional host replacement ladder has
been recorded as a parked proposal. The default direction should return to
allocator implementation and comparison evidence rather than process allocator
replacement.

## Candidate Rows

- allocator comparison baseline inventory
- memory-usage benchmark target inventory
- next concrete hako_alloc implementation seam
- optional replacement execution remains parked

## Stop Lines

- No hook installation.
- No backend matcher additions.
- No process allocator replacement.
- No `#[global_allocator]`.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Planning validation is L0:

```text
current state pointer guard
git diff --check
```
