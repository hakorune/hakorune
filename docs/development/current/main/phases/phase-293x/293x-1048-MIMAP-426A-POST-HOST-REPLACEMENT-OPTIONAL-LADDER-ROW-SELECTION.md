# 293x-1048 MIMAP-426A Post Host Replacement Optional Ladder Row Selection

Status: landed
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

## Decision

Select:

```text
MIMAP-427A Allocator Comparison Baseline Inventory
```

Reason:

The optional host replacement ladder is now documented and parked. The active
lane should return to the main goal: building a `.hako` / `hako_alloc`
allocator whose performance and memory usage can be compared against C
mimalloc.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_post_host_replacement_optional_ladder_row_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Evidence

- Selected allocator comparison baseline inventory as the next row.
- Kept optional process allocator replacement parked.
- Kept hook installation, backend matcher additions, process allocator
  replacement, worker/thread execution, and global allocator install closed.
