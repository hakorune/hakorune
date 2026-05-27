---
Status: Current
Date: 2026-05-27
Scope: measure the object-lifecycle facade exact-EXE after the active field fast path keeper.
Blocker: HAKO-MIMALLOC-POST-ACTIVE-FIELD-FAST-PATH-KEEPER-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-101-HAKO-MIMALLOC-SELECT-SINGLE-PAGE-ACTIVE-FIELD-FAST-PATH-KEEPER.md
---

# 296x-102 Hako Mimalloc Post Active Field Fast Path Keeper Measurement

## Purpose

Rerun the repeated exact-EXE measurement after row101 so the active field fast
path is judged by measured behavior.

## Required Output

```text
output_contract=hako-mimalloc-post-active-field-fast-path-keeper-measurement-v0
input_contract=hako-mimalloc-select-single-page-active-field-fast-path-keeper-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_active_field_fast_path_keeper
keeper=select_single_page_active_field_fast_path
sample_count
after_hako_elapsed_median_ms
select_page_single_fast_path_count
select_page_single_fallback_count=0
release_known_page_fast_path_count
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

Do not implement another keeper, open provider activation, replacement, hooks,
globals, or winner claims in this row.
