---
Status: Current
Date: 2026-05-28
Scope: measure exact-EXE after the small-alloc acquire_usize fast path keeper.
Blocker: POST-PAGE-ACQUIRE-USIZE-FAST-PATH-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-152-SMALL-ALLOC-PAGE-ACQUIRE-USIZE-FAST-PATH-IMPLEMENTATION.md
---

# 296x-153 Post Page Acquire Usize Fast Path Measurement

## Purpose

Run the exact-EXE scout measurement after the small-alloc `acquire_usize`
keeper, then classify whether the keeper is accepted, neutral, or regressed.

## Required Output

```text
output_contract=post-page-acquire-usize-fast-path-measurement-v0
input_contract=small-alloc-page-acquire-usize-fast-path-implementation-v0
elapsed_median_ms
previous_checkpoint_ms=600
keeper_effect
winner_claim=0
replacement_active=0
summary=ok
```
