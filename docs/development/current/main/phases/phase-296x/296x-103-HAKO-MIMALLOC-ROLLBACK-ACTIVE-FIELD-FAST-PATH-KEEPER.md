---
Status: Landed
Date: 2026-05-27
Scope: rollback the regressed select single-page active field fast path keeper.
Blocker: HAKO-MIMALLOC-ROLLBACK-ACTIVE-FIELD-FAST-PATH-KEEPER-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-102-HAKO-MIMALLOC-POST-ACTIVE-FIELD-FAST-PATH-KEEPER-MEASUREMENT.md
---

# 296x-103 Hako Mimalloc Rollback Active Field Fast Path Keeper

## Purpose

Row102 measured the row101 active field fast path as a regression:

```text
previous_median_ms=620
after_hako_elapsed_median_ms=650
median_delta_ms=30
keeper_effect=regressed
```

Rollback that single keeper and return the source to the row99/row100
first-page cache baseline before selecting another optimization.

## Required Output

```text
output_contract=hako-mimalloc-rollback-active-field-fast-path-keeper-v0
input_contract=hako-mimalloc-post-active-field-fast-path-keeper-measurement-v0
rolled_back_keeper=select_single_page_active_field_fast_path
active_field_fast_path_present=0
first_page_cache_preserved=1
generic_lifecycle_fallback_preserved=1
proof_summary=ok
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

Do not add another optimization in the rollback row. Keep the row98 first-page
cache keeper intact.

## Landed Evidence

```text
output_contract=hako-mimalloc-rollback-active-field-fast-path-keeper-v0
input_contract=hako-mimalloc-post-active-field-fast-path-keeper-measurement-v0
rolled_back_keeper=select_single_page_active_field_fast_path
active_field_fast_path_present=0
first_page_cache_preserved=1
generic_lifecycle_fallback_preserved=1
proof_app=apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako
proof_summary=ok
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
winner_claim=0
replacement_active=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_rollback_active_field_fast_path_keeper_guard.sh
```
