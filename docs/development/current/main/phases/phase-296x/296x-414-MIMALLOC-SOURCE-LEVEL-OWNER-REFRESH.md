---
Status: Landed
Date: 2026-05-30
Scope: refresh the remaining source-level owner after the direct-path closeout and decide whether any new fast path is justified.
Blocker: MIMALLOC-SOURCE-LEVEL-OWNER-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-413-POST-DIRECTARRAY-REMAINING-DIRECT-PATH-SURFACE-CHECK.md
  - docs/development/current/main/phases/phase-296x/296x-415-MIMALLOC-SOURCE-LEVEL-OWNER-SELECTION.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
  - docs/development/current/main/investigations/phase296x-414-msr001-object-lifecycle-facade-scan.md
  - docs/development/current/main/investigations/phase296x-414-msr002-page-model-page-queue-scan.md
  - docs/development/current/main/investigations/phase296x-414-msr003-result-capsule-scan.md
  - docs/development/current/main/investigations/phase296x-414-msr005-array-lane-backlog-freeze.md
  - docs/development/current/main/design/array-lane-extension-roadmap-ssot.md
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
---

# 296x-414 Mimalloc Source-Level Owner Refresh

## Purpose

Return to mimalloc source-level optimization after the DirectArray closeout.

This row does not open another helper or substrate fast path. It refreshes the
remaining source-level owner so the next row can choose a single mimalloc
source-level target, if any, without reopening the direct-path lane.

## Contract

```text
output_contract=mimalloc-source-level-owner-refresh-v0
input_contract=post-directarray-remaining-direct-path-surface-check-v0
workload_id=representative-object-lifecycle-small-block-v0

source_level_owner_surface=object_lifecycle_facade
source_level_owner_file=lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
typed_object_legacy_field_helper_open=0
runtime_databox_consumer_surface_open=0
public_arraybox_runtime_surface_open=0
directarray_optional_member_open=0
result_capsule_value_aggregate_open=0
page_model_page_queue_open=0
array_lane_extension_backlog_documented=1

new_fast_path_open=0
new_fast_path_owner=none
return_to_mimalloc_source_level=1
selected_boundary=mimalloc_source_level_owner_selection
next_diagnostic=mimalloc_source_level_owner_selection
selected_next=mimalloc_source_level_owner_selection

open_new_fast_path_only_if_positive_net_helper_delta=1
open_new_fast_path_only_if_perf_owner_pct_above_threshold=1
open_new_fast_path_only_if_selected_callsite_or_family=1
open_new_fast_path_only_if_no_recent_nonkeeper=1
open_new_fast_path_only_if_no_silent_fallback=1

optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Mini Task Board

Keep each item small enough for a mini worker. This row is a source-level
owner refresh, not a new direct-path substrate.
Do not bundle multiple source families into one worker pass.

### MSR-001: Object Lifecycle Facade Scan

Input:
- `lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako`

Output:
- short note on whether the object lifecycle facade is still the top source-level owner
- [investigation note](../../investigations/phase296x-414-msr001-object-lifecycle-facade-scan.md)

Acceptance:
- no new fast path is proposed
- the source-level surface remains the focus

Result:
- `object_lifecycle_facade` remains the top source-level owner surface for row414
- no helper or substrate lane is reopened from this scan

### MSR-002: Page Model / Page Queue Scan

Input:
- `docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md`

Output:
- short note on whether page-model/page-queue should stay parked or be revisited as source-level work
- [investigation note](../../investigations/phase296x-414-msr002-page-model-page-queue-scan.md)

Acceptance:
- no direct-path reopen is proposed
- the source-level lane stays explicit

Result:
- page-model/page-queue stay parked as source-level work for row414
- no new fast path is reopened from this scan

### MSR-003: Result Capsule Scan

Input:
- `docs/development/current/main/design/capsule-value-result-contract-ssot.md`

Output:
- short note on whether result capsule work is still closed for this lane
- [investigation note](../../investigations/phase296x-414-msr003-result-capsule-scan.md)

Acceptance:
- no ValueAggregate reopen is proposed
- the source-level lane stays explicit

Result:
- result capsule work stays closed for row414
- no ValueAggregate reopen is proposed from this scan

### MSR-004: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row414 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_mimalloc_source_level_owner_refresh_guard.sh` passes
- `bash tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

Result:
- the row414 guard passes
- the compact current-state pointer guard passes
- the worktree diff stays clean

### MSR-005: Array Lane Backlog Freeze

Input:
- `docs/development/current/main/design/array-lane-extension-roadmap-ssot.md`

Output:
- short note confirming Array extension work remains backlog and does not reopen
  the DirectArray fast-path lane
- [investigation note](../../investigations/phase296x-414-msr005-array-lane-backlog-freeze.md)

Acceptance:
- public `ArrayBox` identity remains unchanged
- plugin object values stay Boxed/handle-first
- plugin scalar inline work requires explicit ABI facts
- record/union inline layout stays deferred
- no implementation is proposed from this row

Result:
- Array extension work remains backlog for row414
- the DirectArray fast-path lane stays closed

## Decision

The direct-path lane is closed. The remaining work is source-level owner
refresh, with `object_lifecycle_facade` as the explicit candidate surface for
the next selection row.

This row does not reopen any helper or substrate fast path.

## Forbidden

- no new DirectArray member
- no helper micro-optimization
- no generic typed-field residence retry
- no RuntimeDataBox fallback widening
- no public ArrayBox handle reinterpretation
- no provider activation
- no allocator replacement
- no hook installation
- no `#[global_allocator]`

## Guard

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_source_level_owner_refresh_guard.sh
```
