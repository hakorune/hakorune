---
Status: Landed
Date: 2026-05-28
Scope: refresh source/MIR after the known-live release keeper measurement.
Blocker: POST-KNOWN-LIVE-RELEASE-SOURCE-MIR-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-149-POST-KNOWN-LIVE-RELEASE-MEASUREMENT.md
---

# 296x-150 Post Known-Live Release Source/MIR Refresh

## Purpose

Refresh source and MIR shape after the known-live release keeper measurement
before selecting another page-array or compiler helper-copy keeper.

## Required Output

```text
output_contract=post-known-live-release-source-mir-refresh-v0
input_contract=post-known-live-release-measurement-v0
active_owner
selected_next
summary=ok
```

## Evidence

```text
output_contract=post-known-live-release-source-mir-refresh-v0
input_contract=post-known-live-release-measurement-v0
small_alloc_mir_instruction_count=185
small_alloc_call_count=16
small_alloc_copy_count=99
release_block_mir_instruction_count=127
release_block_call_count=12
release_block_copy_count=71
direct_release_mir_instruction_count=86
direct_release_call_count=3
direct_release_known_live_call_count=1
page_acquire_mir_instruction_count=235
page_acquire_call_count=4
page_acquire_copy_count=116
page_acquire_array_get_call_count=2
page_acquire_array_set_call_count=2
page_release_known_live_mir_instruction_count=76
page_release_known_live_call_count=2
page_release_known_live_array_get_call_count=0
page_release_known_live_array_set_call_count=2
active_owner=allocator_page_array_surface
secondary_owner=compiler_helper_copy
selected_next=page_acquire_fast_path_keeper_selection
winner_claim=0
replacement_active=0
summary=ok
```

Interpretation:

```text
The known-live release keeper removed the direct release live-check get from
the hot path, but page_acquire remains the largest page-local ArrayBox/MIR
surface. Compiler helper-copy pressure is still secondary and should not be
mixed into the next page keeper row.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_post_known_live_release_source_mir_refresh_guard.sh
```
