---
Status: Landed
Date: 2026-05-28
Scope: measure exact-EXE after direct cached-page known-live release keeper.
Blocker: POST-KNOWN-LIVE-RELEASE-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-148-RELEASE-DIRECT-CACHED-PAGE-KNOWN-LIVE-RELEASE-IMPLEMENTATION.md
---

# 296x-149 Post Known-Live Release Measurement

## Purpose

Run the full repeated exact-EXE measurement after the lightweight row148
semantic proof. Keep this out of the implementation guard.

## Required Output

```text
output_contract=post-known-live-release-measurement-v0
input_contract=release-direct-cached-page-known-live-release-implementation-v0
elapsed_median_ms
previous_checkpoint_median_ms
keeper_effect
winner_claim=0
replacement_active=0
summary=ok
```

## Evidence

```text
output_contract=post-known-live-release-measurement-v0
input_contract=release-direct-cached-page-known-live-release-implementation-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_known_live_release
workload_id=representative-object-lifecycle-small-block-v0
operation_repeat=8192
sample_count=1
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
sample_0_hako_external_elapsed_ms=600
elapsed_median_ms=600
elapsed_min_ms=600
elapsed_max_ms=600
previous_checkpoint_median_ms=610
keeper_effect=accepted
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
selected_next=post_known_live_release_source_mir_refresh
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_post_known_live_release_measurement_guard.sh
```
