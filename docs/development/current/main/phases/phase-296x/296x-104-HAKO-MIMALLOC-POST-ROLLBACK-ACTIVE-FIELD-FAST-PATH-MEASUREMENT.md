---
Status: Current
Date: 2026-05-27
Scope: measure the object-lifecycle facade exact-EXE after rolling back the active field fast path.
Blocker: HAKO-MIMALLOC-POST-ROLLBACK-ACTIVE-FIELD-FAST-PATH-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-103-HAKO-MIMALLOC-ROLLBACK-ACTIVE-FIELD-FAST-PATH-KEEPER.md
---

# 296x-104 Hako Mimalloc Post Rollback Active Field Fast Path Measurement

## Purpose

Rerun the repeated exact-EXE measurement after row103 to confirm the source has
returned to the row99 first-page cache baseline before selecting another
optimization.

## Required Output

```text
output_contract=hako-mimalloc-post-rollback-active-field-fast-path-measurement-v0
input_contract=hako-mimalloc-rollback-active-field-fast-path-keeper-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_active_field_fast_path_rollback
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
