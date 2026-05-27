---
Status: Current
Date: 2026-05-27
Scope: rollback the regressed small-alloc inline success result keeper.
Blocker: HAKO-MIMALLOC-ROLLBACK-INLINE-SUCCESS-RESULT-KEEPER-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-110-HAKO-MIMALLOC-POST-INLINE-SUCCESS-RESULT-KEEPER-MEASUREMENT.md
---

# 296x-111 Hako Mimalloc Rollback Inline Success Result Keeper

## Purpose

Row110 measured the row109 inline success result keeper as a regression:

```text
previous_checkpoint_median_ms=620
after_hako_elapsed_median_ms=630
median_delta_vs_previous_ms=10
keeper_effect=regressed
```

Rollback that single keeper while preserving the accepted first-page cache and
small-alloc direct select keepers.

## Required Output

```text
output_contract=hako-mimalloc-rollback-inline-success-result-keeper-v0
input_contract=hako-mimalloc-post-inline-success-result-keeper-measurement-v0
rolled_back_keeper=small_alloc_inline_success_result_fast_path
inline_success_result_present=0
small_alloc_direct_select_preserved=1
proof_summary=ok
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

Do not add another optimization in the rollback row. Keep accepted prior
keepers intact.
