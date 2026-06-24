---
Status: Landed
Date: 2026-05-28
Scope: select the next page-array keeper from dynamic weight evidence.
Blocker: PAGE-ARRAY-KEEPER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-146-PAGE-ARRAY-DYNAMIC-WEIGHT-PROBE.md
---

# 296x-147 Page Array Keeper Selection

## Purpose

Select exactly one next keeper from page-local ArrayBox dynamic weight evidence.
Do not mix compiler helper-call lowering into the same row; it remains a
secondary owner after page-array keeper selection.

## Required Output

```text
output_contract=page-array-keeper-selection-v0
input_contract=page-array-dynamic-weight-probe-v0
selected_keeper
keeper_owner
expected_dynamic_weight_reduction
fallback_preservation
summary=ok
```

## Evidence

```text
output_contract=page-array-keeper-selection-v0
input_contract=page-array-dynamic-weight-probe-v0
selected_keeper=release_direct_cached_page_known_live_release
keeper_owner=lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
keeper_uses_existing_method=HakoAllocPageModel.releaseLocalKnownLive/1
expected_dynamic_weight_reduction=524288
expected_dynamic_weight_reduction_percent=12
fallback_preservation=generic_releaseLocal_unchanged
safety_precondition=direct_cached_page_same_page_id_and_cached_page_non_null
compiler_helper_copy_secondary=1
winner_claim=0
replacement_active=0
selected_next=release_direct_cached_page_known_live_release_implementation
summary=ok
```

Interpretation:

```text
The keeper targets the proof workload's direct cached release path only. It
uses the existing known-live page method to remove one block_used.get per
release while preserving generic releaseLocal checks and fallback behavior.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_page_array_keeper_selection_guard.sh
```
